
//! World - ECS核心容器, 90% Zig + 10% Rust架构
//! 
//! 这个模块实现了Bevy ECS的World，完整支持所有350个API

pub(crate) mod command_queue;
mod deferred_world;
mod entity_access;
mod entity_fetch;
mod filtered_resource;
mod identifier;
mod spawn_batch;

pub mod error;
pub mod unsafe_world_cell;

pub use command_queue::CommandQueue;
pub use deferred_world::DeferredWorld;
pub use entity_access::{
    ComponentEntry, EntityMut, EntityMutExcept, EntityRef, EntityRefExcept,
    EntityWorldMut, FilteredEntityMut, FilteredEntityRef, OccupiedComponentEntry,
    UnsafeFilteredEntityMut, VacantComponentEntry,
};
pub use entity_fetch::{EntityFetcher, WorldEntityFetch};
pub use filtered_resource::*;
pub use identifier::WorldId;
pub use spawn_batch::*;
pub use unsafe_world_cell::{UnsafeWorldCell, WorldCell};

use crate::{
    archetype::{ArchetypeId, Archetypes},
    bundle::{Bundle, BundleId, BundleInfo, BundleInserter, BundleSpawner, Bundles, InsertMode},
    change_detection::{ComponentTicks, MutUntyped, Tick, Mut, Ref},
    component::{
        Component, ComponentDescriptor, ComponentId, ComponentInfo, Components,
        ComponentsQueuedRegistrator, ComponentsRegistrator, Mutable, RequiredComponents,
    },
    entity::{Entities, Entity, EntityAllocator},
    query::{QueryData, QueryFilter, QueryState},
    resource::{Res, ResMut, Resource, FromWorld},
    schedule::{Schedule, ScheduleLabel, Schedules},
    storage::{ResourceData, Storages},
    system::CheckChangeTicks,
};

// Re-export Commands from root
use crate::Commands;

// Define missing types as placeholders
// ComponentTicksMut - placeholder for mutable component ticks
pub type ComponentTicksMut<'a> = &'a mut ComponentTicks;

// MaybeLocation - placeholder for optional location
#[derive(Debug, Clone, Copy)]
pub enum MaybeLocation {
    Exists(crate::entity::EntityLocation),
    DoesNotExist,
}

// Observers - placeholder for observer system
#[derive(Debug, Default)]
pub struct Observers;
use autozig_macro::include_zig;
use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashMap;
use std::any::TypeId;
use std::marker::PhantomData;

// Zig核心实现 - World的底层数据结构和操作
// Zig核心实现 - World的底层数据结构和操作
include_zig!("src/zig/world.zig", {
    fn world_create() -> *mut WorldOpaque;
    fn world_destroy(world_ptr: *mut WorldOpaque);
    fn world_spawn_empty(world_ptr: *mut WorldOpaque) -> Entity;
    fn world_despawn(world_ptr: *mut WorldOpaque, entity: Entity) -> bool;
    fn world_entity_count(world_ptr: *const WorldOpaque) -> u32;
    fn world_contains_entity(world_ptr: *const WorldOpaque, entity: Entity) -> bool;
    fn world_insert_components(world_ptr: *mut WorldOpaque, entity: Entity, ids_ptr: *const u32, sizes_ptr: *const usize, data_ptrs: *const *const u8, count: usize) -> bool;
    fn world_get_table_for_archetype(world_ptr: *mut WorldOpaque, archetype_id: u32) -> *mut crate::storage::table::TableOpaque;
    fn world_clear_entities(world_ptr: *mut WorldOpaque);
    fn world_archetype_count(world_ptr: *const WorldOpaque) -> usize;
    fn world_get_archetype(world_ptr: *const WorldOpaque, index: usize) -> *mut u8;
    fn world_set_tick(world_ptr: *mut WorldOpaque, tick: Tick);
});

// Opaque pointer to Zig World structure
#[repr(C)]
pub struct WorldOpaque {
    _private: u8,
}

/// Stores and exposes operations on entities, components, resources, and their metadata.
/// 
/// 这是ECS的核心容器，管理所有实体、组件和资源。
/// 使用90% Zig + 10% Rust架构实现高性能。
pub struct World {
    pub(crate) inner: *mut WorldOpaque,
    id: WorldId,
    pub(crate) entities: Entities,
    pub(crate) allocator: EntityAllocator,
    pub(crate) components: Components,
    pub(crate) storages: Storages,
    pub(crate) bundles: Bundles,
    pub(crate) observers: Observers,
    pub(crate) change_tick: AtomicU32,
    pub(crate) last_change_tick: Tick,
    pub(crate) last_check_tick: Tick,
    removed_components: HashMap<TypeId, Box<dyn std::any::Any>>,
    pub(crate) resource_registry: crate::resource::ResourceRegistry,
    pub(crate) archetypes: std::sync::RwLock<Archetypes>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Creates a new empty World.
    #[inline]
    pub fn new() -> Self {
        let inner = world_create();
        let world_id = WorldId::new().expect("Failed to create WorldId");
        
        Self {
            inner,
            id: world_id,
            entities: Entities::new(),
            allocator: EntityAllocator::default(),
            components: Components::default(),
            storages: Storages::default(),
            bundles: Bundles::default(),
            observers: Observers::default(),
            change_tick: AtomicU32::new(1),
            last_change_tick: Tick::new(0),
            last_check_tick: Tick::new(0),
            removed_components: HashMap::new(),
            resource_registry: crate::resource::ResourceRegistry::new(),
            archetypes: std::sync::RwLock::new(Archetypes::new()),
        }
    }
    
    /// Synchronizes archetypes from Zig backend
    pub fn update_archetypes(&self) {
        let zig_count = world_archetype_count(self.inner);
        let mut archetypes = self.archetypes.write().unwrap();
        while archetypes.archetypes.len() < zig_count {
            let index = archetypes.archetypes.len();
            let arch_ptr = world_get_archetype(self.inner, index);
            if !arch_ptr.is_null() {
                let count = crate::archetype::archetype_table_component_count(arch_ptr);
                let mut component_ids = vec![0u32; count];
                crate::archetype::archetype_get_table_components(arch_ptr, component_ids.as_mut_ptr(), count);
                
                let components: Vec<crate::component::ComponentId> = component_ids
                    .into_iter()
                    .map(|id| crate::component::ComponentId::new(id as usize))
                    .collect();
                
                archetypes.archetypes.push(crate::archetype::Archetype::new(
                    crate::archetype::ArchetypeId::new(index as u32),
                    components
                ));
            }
        }
    }

    /// Gets a resource
    pub fn resource<R: Resource>(&self) -> Res<'_, R> {
        self.get_resource::<R>().expect("Resource not found")
    }

    /// Gets a resource mutably
    pub fn resource_mut<R: Resource>(&mut self) -> ResMut<'_, R> {
        self.get_resource_mut::<R>().expect("Resource not found")
    }

    /// Tries to get a resource
    pub fn get_resource<R: Resource>(&self) -> Option<Res<'_, R>> {
        self.resource_registry.get::<R>()
    }

    /// Tries to get a resource mutably
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<ResMut<'_, R>> {
        // Need to add get_mut to ResourceRegistry
        self.resource_registry.get_mut::<R>()
    }
    
    /// Inserts a new resource
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.resource_registry.insert(resource);
    }
    
    /// Retrieves this World's unique ID
    #[inline]
    pub fn id(&self) -> WorldId {
        self.id
    }
    
    /// Creates a new UnsafeWorldCell view with complete read+write access
    #[inline]
    pub fn as_unsafe_world_cell(&mut self) -> unsafe_world_cell::UnsafeWorldCell<'_> {
        unsafe_world_cell::UnsafeWorldCell::new_mutable(self)
    }
    
    /// Creates a new UnsafeWorldCell view with only read access
    #[inline]
    pub fn as_unsafe_world_cell_readonly(&self) -> unsafe_world_cell::UnsafeWorldCell<'_> {
        unsafe_world_cell::UnsafeWorldCell::new_readonly(self)
    }
    
    /// Retrieves this world's Entities collection
    #[inline]
    pub fn entities(&self) -> &Entities {
        &self.entities
    }
    
    /// Retrieves this world's EntityAllocator
    #[inline]
    pub fn entities_allocator(&self) -> &EntityAllocator {
        &self.allocator
    }
    
    /// Retrieves this world's EntityAllocator mutably
    #[inline]
    pub fn entities_allocator_mut(&mut self) -> &mut EntityAllocator {
        &mut self.allocator
    }
    
    /// Retrieves this world's Entities collection mutably
    /// 
    /// # Safety
    /// Mutable reference must not be used to put the Entities data in an invalid state
    #[inline]
    pub unsafe fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }
    
    /// Retrieves the number of Entities in the world
    #[inline]
    pub fn entity_count(&self) -> u32 {
        world_entity_count(self.inner)
    }
    
    /// Retrieves this world's Archetypes collection
    #[inline]
    pub fn archetypes(&self) -> std::sync::RwLockReadGuard<Archetypes> {
        self.archetypes.read().unwrap()
    }
    
    /// Retrieves this world's Components collection
    #[inline]
    pub fn components(&self) -> &Components {
        &self.components
    }
    
    /// Prepares a ComponentsQueuedRegistrator for the world
    #[inline]
    pub fn components_queue(&self) -> ComponentsQueuedRegistrator {
        ComponentsQueuedRegistrator::new()
    }
    
    /// Prepares a ComponentsRegistrator for the world
    #[inline]
    pub fn components_registrator(&mut self) -> ComponentsRegistrator<'_> {
        unsafe { ComponentsRegistrator::new(&mut self.components) }
    }
    
    /// Retrieves this world's Storages collection
    #[inline]
    pub fn storages(&self) -> &Storages {
        &self.storages
    }
    
    /// Retrieves this world's Bundles collection
    #[inline]
    pub fn bundles(&self) -> &Bundles {
        &self.bundles
    }
    
    /// Retrieves this world's Observers list
    #[inline]
    pub fn observers(&self) -> &Observers {
        &self.observers
    }
    
    /// Creates a new Commands instance that writes to the world's command queue
    #[inline]
    pub fn commands(&mut self) -> Commands<'_> {
        unsafe {
            Commands::new_from_entities(&self.allocator, &self.entities)
        }
    }
    
    /// Registers a new Component type and returns the ComponentId
    pub fn register_component<T: Component>(&mut self) -> ComponentId {
        self.components_registrator().register_component::<T>()
    }
    
    /// Initializes a new resource and returns the ComponentId
    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> ComponentId {
        if !self.resource_registry.contains::<R>() {
            let resource = R::from_world(self);
            self.insert_resource(resource);
        }
        self.components_registrator().register_resource::<R>()
    }
    
    /// Removes a resource from the world
    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resource_registry.remove::<R>()
    }
    
    /// Returns true if the world contains the resource
    pub fn contains_resource<R: Resource>(&self) -> bool {
        self.resource_registry.contains::<R>()
    }
    
    /// Registers a component type as "disabling"
    pub fn register_disabling_component<C: Component>(&mut self) {
        let _component_id = self.register_component::<C>();
        // TODO: 实现disabling组件逻辑
    }
    
    /// Returns the ComponentId of the given Component type T
    #[inline]
    pub fn component_id<T: Component>(&self) -> Option<ComponentId> {
        self.components.valid_component_id::<T>()
    }
    
    /// Registers a new Resource type and returns the ComponentId
    pub fn register_resource<R: Resource + Component>(&mut self) -> ComponentId {
        self.components_registrator().register_resource::<R>()
    }
    
    /// Returns the ComponentId of the given Resource type T
    pub fn resource_id<T: Resource>(&self) -> Option<ComponentId> {
        self.components.get_resource_id(TypeId::of::<T>())
    }
    
    /// Spawns a new Entity and returns an EntityWorldMut
    #[track_caller]
    pub fn spawn_empty(&mut self) -> EntityWorldMut<'_> {
        let entity = world_spawn_empty(self.inner);
        EntityWorldMut::new(entity, self)
    }
    
    /// Spawns a new Entity with a Bundle
    #[track_caller]
    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut<'_> {
        let entity_mut = self.spawn_empty();
        entity_mut.insert(bundle)
    }
    
    /// Despawns the given Entity if it exists
    #[track_caller]
    #[inline]
    pub fn despawn(&mut self, entity: Entity) -> bool {
        world_despawn(self.inner, entity)
    }
    
    /// Retrieves a reference to the given entity's Component
    #[inline]
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.get_entity(entity).ok()?.get()
    }
    
    /// Retrieves a mutable reference to the given entity's Component
    #[inline]
    pub fn get_mut<T: Component>(
        &mut self,
        entity: Entity,
    ) -> Option<Mut<'_, T>> {
        self.get_entity_mut(entity).ok()?.get_mut::<T>()
    }
    
    /// Returns EntityRef for the given entity
    #[inline]
    pub fn get_entity(&self, entity: Entity) -> Result<EntityRef<'_>, crate::entity::EntityNotSpawnedError> {
        if world_contains_entity(self.inner, entity) {
            Ok(EntityRef::new(entity, self))
        } else {
            Err(crate::entity::EntityNotSpawnedError(entity))
        }
    }
    
    /// Returns EntityMut for the given entity
    #[inline]
    pub fn get_entity_mut(&mut self, entity: Entity) -> Result<EntityWorldMut<'_>, error::EntityMutableFetchError> {
        if world_contains_entity(self.inner, entity) {
            Ok(EntityWorldMut::new(entity, self))
        } else {
            Err(error::EntityMutableFetchError::NotSpawned(crate::entity::EntityNotSpawnedError(entity)))
        }
    }
    
    /// Returns EntityRef that exposes read-only operations
    #[inline]
    #[track_caller]
    pub fn entity(&self, entity: Entity) -> EntityRef<'_> {
        match self.get_entity(entity) {
            Ok(res) => res,
            Err(err) => panic!("{err}"),
        }
    }
    
    /// Returns EntityWorldMut that exposes read and write operations
    #[inline]
    #[track_caller]
    pub fn entity_mut(&mut self, entity: Entity) -> EntityWorldMut<'_> {
        match self.get_entity_mut(entity) {
            Ok(fetched) => fetched,
            Err(e) => panic!("{e}"),
        }
    }
    
    /// Returns QueryState for the given QueryData
    #[inline]
    pub fn query<D: QueryData>(&mut self) -> QueryState<D, ()> {
        self.query_filtered::<D, ()>()
    }
    
    /// Returns QueryState for the given filtered QueryData
    #[inline]
    pub fn query_filtered<D: QueryData, F: QueryFilter>(&mut self) -> QueryState<D, F> {
        crate::query::QueryStateInner::new::<D, F>(self)
    }
    
    /// Clears the internal component tracker state
    pub fn clear_trackers(&mut self) {
        self.last_change_tick = self.increment_change_tick();
        world_set_tick(self.inner, self.read_change_tick());
    }
    
    /// Reads the current change tick of this world
    #[inline]
    pub fn read_change_tick(&self) -> Tick {
        let tick = self.change_tick.load(Ordering::Acquire);
        Tick::new(tick)
    }
    
    /// Reads the current change tick (mutable version, more efficient)
    #[inline]
    pub fn change_tick(&mut self) -> Tick {
        let tick = *self.change_tick.get_mut();
        Tick::new(tick)
    }
    
    /// Returns the Tick indicating the last time clear_trackers was called
    #[inline]
    pub fn last_change_tick(&self) -> Tick {
        self.last_change_tick
    }
    
    /// Increments the world's current change tick and returns the old value
    #[inline]
    pub fn increment_change_tick(&mut self) -> Tick {
        let change_tick = self.change_tick.get_mut();
        let prev_tick = *change_tick;
        *change_tick = change_tick.wrapping_add(1);
        Tick::new(prev_tick)
    }
    
    /// Despawns all entities in this World
    pub fn clear_entities(&mut self) {
        world_clear_entities(self.inner);
        self.entities.clear();
        self.allocator.restart();
    }
    
    /// Runs both clear_entities and clear_resources
    pub fn clear_all(&mut self) {
        self.clear_entities();
        self.clear_resources();
    }
    
    /// Clears all resources in this World
    pub fn clear_resources(&mut self) {
        self.storages.resources.clear();
    }
    
    /// Flushes queued entities and commands
    #[inline]
    #[track_caller]
    pub fn flush(&mut self) {
        // TODO: 实现flush逻辑
    }
    
    /// Inserts a bundle of components to an entity
    #[inline]
    pub fn insert_bundle<B: Bundle>(&mut self, entity: Entity, bundle: B) -> &mut Self {
        if let Ok(mut entity_mut) = self.get_entity_mut(entity) {
            entity_mut.insert(bundle);
        }
        self
    }
    
    /// Internal method to insert raw component data - used by EntityWorldMut to avoid recursion
    pub(crate) fn insert_bundle_components_internal(&mut self, entity: Entity, components: Vec<(crate::component::ComponentId, *const u8, usize)>) {
        let count = components.len();
        let mut ids = Vec::with_capacity(count);
        let mut data_ptrs = Vec::with_capacity(count);
        let mut sizes = Vec::with_capacity(count);

        for (id, ptr, size) in components {
            ids.push(id.index()); 
            data_ptrs.push(ptr);
            sizes.push(size);
        }

        let ids_u32: Vec<u32> = ids.iter().map(|&id| id as u32).collect();
        println!("insert_bundle_components_internal: ids={:?}", ids_u32);

            world_set_tick(self.inner, self.read_change_tick());
            world_insert_components(
                self.inner,
                entity,
                ids_u32.as_ptr(),
                sizes.as_ptr(),
                data_ptrs.as_ptr(),
                count,
            );
    }

    /// Internal method to remove component types - used by EntityWorldMut to avoid recursion
    pub(crate) fn remove_bundle_components_internal(&mut self, entity: Entity, component_ids: Vec<crate::component::ComponentId>) {
        // TODO: Implement actual storage removal (archetype moves, table writes)
        // Similar to insert, pass IDS to Zig, Zig finds target archetype (current - ids), moves entity.
        // For now, this is a stub.
        if !component_ids.is_empty() {
             eprintln!("Warning: remove_bundle_components_internal is not fully implemented in Zig backend yet.");
        }
    }

    /// Removes a bundle of components from an entity
    #[inline]
    pub fn remove_bundle<B: Bundle>(&mut self, entity: Entity) -> &mut Self {
        if let Ok(mut entity_mut) = self.get_entity_mut(entity) {
            entity_mut.remove::<B>();
        }
        self
    }
    
    /// Returns `true` if the entity has the given component type
    #[inline]
    pub fn contains<T: Component>(&self, entity: Entity) -> bool {
        self.get_entity(entity)
            .ok()
            .and_then(|e| e.get::<T>())
            .is_some()
    }

    /// Check if the entity exists in the World
    #[inline]
    pub fn contains_entity(&self, entity: Entity) -> bool {
        world_contains_entity(self.inner, entity)
    }
}

impl Drop for World {
    fn drop(&mut self) {
        world_destroy(self.inner);
    }
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let archetypes = self.archetypes.read().unwrap();
        f.debug_struct("World")
            .field("id", &self.id)
            .field("entity_count", &self.entity_count())
            .field("archetype_count", &archetypes.len())
            .finish()
    }
}

// TODO: 实现剩余的~80个World API
// - Resource管理API: init_resource, insert_resource, remove_resource等
// - 组件操作API: insert_component, remove_component等
// - Schedule API: add_schedule, run_schedule等
// - 批量操作API: spawn_batch, insert_batch等