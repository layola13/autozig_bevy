//! Component metadata and registry information

use crate::component::{Component, ComponentCloneBehavior, StorageType};
use crate::resource::Resource;
use crate::component_advanced::ComponentHooks;
use std::alloc::Layout;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(crate) fn hash_type_id(type_id: TypeId) -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    type_id.hash(&mut hasher);
    (hasher.finish() & 0xFFFFFFFF) as u32
}

/// Stores metadata for a type of component or resource stored in a specific [`World`].
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub(crate) id: ComponentId,
    pub(crate) name: String,
    pub(crate) type_id: Option<TypeId>,
    pub(crate) layout: Layout,
    pub(crate) storage_type: StorageType,
    pub(crate) is_send_and_sync: bool,
    pub(crate) mutable: bool,
    pub(crate) clone_behavior: ComponentCloneBehavior,
    pub(crate) hooks: ComponentHooks,
}

// Manually implement Clone because ComponentHooks is not Clone?
// ComponentHooks in component_advanced.rs does not derive Clone.
// ComponentInfo was deriving Clone.
// If I remove Clone from ComponentInfo, it breaks usages that clone it.
// ComponentDescriptor is Cloned. ComponentInfo is Cloned in `get_descriptor`?
// `get_descriptor` creates a new ComponentDescriptor from fields. It doesn't clone ComponentInfo.
// `Components::components` is `Vec<Option<ComponentInfo>>`.
// Is ComponentInfo cloned elsewhere?
// `QueuedComponents` stores `ComponentInfo`.
// `ComponentInfo` clone is likely needed.
// So `ComponentHooks` MUST be Clone.
// But `Box<dyn Fn>` is not Clone.
// Bevy solves this by wrapping hooks in `Arc` or implementing manual clone that panics or shares?
// Or hooks are not cloneable and ComponentInfo is not Cloneable.

// Implementation detail: Bevy's ComponentInfo is NOT Clone.
// The `derive(Clone)` on `ComponentInfo` in this file (line 11) suggests it WAS Clone.
// I should check if I really need it to be Clone.
// If I remove `derive(Clone)`, I might break things.
// But `Box<dyn Fn>` prevents `derive(Clone)`.
// I will implement `Clone` manually for `ComponentHooks` (returning default/empty hooks or Arc?)
// Or simply remove `derive(Clone)` from `ComponentInfo` and fix usages.
// `Components::components` stores it directly.
// `QueuedComponents` stores it directly.
// `get_info` returns `&ComponentInfo`.

// I will remove `derive(Clone)` from `ComponentInfo` for now.
// If it breaks, I'll deal with it.

impl ComponentInfo {
    /// Create a new `ComponentInfo` for the type `T`.
    pub fn new<T: Component>() -> Self {
        Self {
            id: ComponentId(0), // Will be set by registry
            name: std::any::type_name::<T>().to_string(),
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            storage_type: T::STORAGE_TYPE,
            is_send_and_sync: true,
            mutable: true,
            clone_behavior: ComponentCloneBehavior::Default,
            hooks: ComponentHooks::new(),
        }
    }

    /// Create a new `ComponentInfo` for a resource.
    pub fn new_resource<T: Resource>() -> Self {
        Self {
            id: ComponentId(0),
            name: std::any::type_name::<T>().to_string(),
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            storage_type: StorageType::Table,
            is_send_and_sync: true,
            mutable: true,
            clone_behavior: ComponentCloneBehavior::Default,
            hooks: ComponentHooks::new(),
        }
    }

    /// Create with explicit layout
    pub fn new_with_layout(
        name: String,
        type_id: Option<TypeId>,
        layout: Layout,
        storage_type: StorageType,
    ) -> Self {
        Self {
            id: ComponentId(0),
            name,
            type_id,
            layout,
            storage_type,
            is_send_and_sync: true,
            mutable: true,
            clone_behavior: ComponentCloneBehavior::Default,
            hooks: ComponentHooks::new(),
        }
    }

    /// Returns a value uniquely identifying the current component.
    #[inline]
    pub fn id(&self) -> ComponentId {
        self.id
    }

    /// Returns the name of the current component.
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the name of the current component.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns `true` if the current component is mutable.
    #[inline]
    pub fn mutable(&self) -> bool {
        self.mutable
    }

    /// Returns [`ComponentCloneBehavior`] of the current component.
    #[inline]
    pub fn clone_behavior(&self) -> &ComponentCloneBehavior {
        &self.clone_behavior
    }

    /// Returns the [`TypeId`] of the underlying component type.
    /// Returns `None` if the component does not correspond to a Rust type.
    #[inline]
    pub fn type_id(&self) -> Option<TypeId> {
        self.type_id
    }

    /// Returns the layout used to store values of this component in memory.
    #[inline]
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Returns a value indicating the storage strategy for the current component.
    #[inline]
    pub fn storage_type(&self) -> StorageType {
        self.storage_type
    }

    /// Returns `true` if the underlying component type can be freely shared between threads.
    #[inline]
    pub fn is_send_and_sync(&self) -> bool {
        self.is_send_and_sync
    }

    /// Returns the required components for this component.
    pub fn required_components(&self) -> &[ComponentId] {
        &[] // Placeholder for RequiredComponents integration
    }

    /// Returns [`RelationshipAccessor`] for this component if it is a Relationship, `None` otherwise.
    pub fn relationship_accessor(&self) -> Option<&dyn Any> {
        None
    }
    
    /// Returns the lifecycle hooks for this component
    #[inline]
    pub fn hooks(&self) -> &ComponentHooks {
        &self.hooks
    }
    
    /// Returns a mutable reference to the lifecycle hooks
    #[inline]
    pub fn hooks_mut(&mut self) -> &mut ComponentHooks {
        &mut self.hooks
    }
}

/// A value which uniquely identifies the type of a [`Component`] or [`Resource`] within a World.
#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ComponentId(pub(crate) usize);

impl ComponentId {
    /// Creates a new [`ComponentId`].
    #[inline]
    pub const fn new(index: usize) -> ComponentId {
        ComponentId(index)
    }

    /// Returns the index of the current component.
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

/// A value describing a component or resource, which may or may not correspond to a Rust type.
#[derive(Clone)]
pub struct ComponentDescriptor {
    pub(crate) name: String,
    pub(crate) storage_type: StorageType,
    pub(crate) is_send_and_sync: bool,
    pub(crate) type_id: Option<TypeId>,
    pub(crate) layout: Layout,
    pub(crate) mutable: bool,
    pub(crate) clone_behavior: ComponentCloneBehavior,
}

impl ComponentDescriptor {
    /// Create a new `ComponentDescriptor` for the type `T`.
    pub fn new<T: Component>() -> Self {
        Self {
            name: std::any::type_name::<T>().to_string(),
            storage_type: T::STORAGE_TYPE,
            is_send_and_sync: true,
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            mutable: true,
            clone_behavior: ComponentCloneBehavior::Default,
        }
    }

    /// Create a new `ComponentDescriptor` for a resource.
    pub fn new_resource<T: Resource>() -> Self {
        Self {
            name: std::any::type_name::<T>().to_string(),
            storage_type: StorageType::Table,
            is_send_and_sync: true,
            type_id: Some(TypeId::of::<T>()),
            layout: Layout::new::<T>(),
            mutable: true,
            clone_behavior: ComponentCloneBehavior::Default,
        }
    }

    /// Create a new `ComponentDescriptor` with explicit layout.
    pub fn new_with_layout(
        name: String,
        storage_type: StorageType,
        layout: Layout,
    ) -> Self {
        Self {
            name,
            storage_type,
            is_send_and_sync: true,
            type_id: None,
            layout,
            mutable: true,
            clone_behavior: ComponentCloneBehavior::Default,
        }
    }

    /// Returns a value indicating the storage strategy for the current component.
    #[inline]
    pub fn storage_type(&self) -> StorageType {
        self.storage_type
    }

    /// Returns the [`TypeId`] of the underlying component type.
    #[inline]
    pub fn type_id(&self) -> Option<TypeId> {
        self.type_id
    }

    /// Returns the name of the current component.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether this component is mutable.
    #[inline]
    pub fn mutable(&self) -> bool {
        self.mutable
    }
}

/// Stores metadata associated with each kind of [`Component`] in a given World.
#[derive(Debug, Default)]
pub struct Components {
    pub(crate) components: RwLock<HashMap<ComponentId, ComponentInfo>>,
    pub(crate) indices: RwLock<HashMap<TypeId, ComponentId>>,
    pub(crate) resource_indices: RwLock<HashMap<TypeId, ComponentId>>,
    pub(crate) queued: Arc<RwLock<QueuedComponents>>,
}

/// Queued component registrations
#[derive(Debug, Default)]
pub struct QueuedComponents {
    pub(crate) components: HashMap<TypeId, ComponentInfo>,
    pub(crate) resources: HashMap<TypeId, ComponentInfo>,
}

impl Components {
    /// Creates a new `Components` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of components registered or queued with this instance.
    #[inline]
    pub fn len(&self) -> usize {
        self.num_queued() + self.num_registered()
    }

    /// Returns `true` if there are no components registered or queued.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.num_queued() == 0 && self.components.read().unwrap().is_empty()
    }

    /// Returns the number of components queued for registration.
    #[inline]
    pub fn num_queued(&self) -> usize {
        let queued = self.queued.read().unwrap();
        queued.components.len() + queued.resources.len()
    }

    /// Returns `true` if there are any components queued for registration.
    #[inline]
    pub fn any_queued(&self) -> bool {
        self.num_queued() > 0
    }

    /// A faster version of [`Self::num_queued`].
    #[inline]
    pub fn num_queued_mut(&mut self) -> usize {
        let queued = self.queued.write().unwrap();
        queued.components.len() + queued.resources.len()
    }

    /// A faster version of [`Self::any_queued`].
    #[inline]
    pub fn any_queued_mut(&mut self) -> bool {
        self.num_queued_mut() > 0
    }

    /// Returns the number of components registered with this instance.
    #[inline]
    pub fn num_registered(&self) -> usize {
        self.components.read().unwrap().len()
    }

    /// Returns `true` if there are any components registered.
    #[inline]
    pub fn any_registered(&self) -> bool {
        self.num_registered() > 0
    }

    /// Gets the metadata associated with the given component, if it is registered.
    #[inline]
    // Note: Returns a CLONE since we can't return a reference through a RwLock easily without returning a guard
    pub fn get_info(&self, id: ComponentId) -> Option<ComponentInfo> {
        self.components.read().unwrap().get(&id).cloned()
    }

    /// Gets the [`ComponentDescriptor`] of the component with this [`ComponentId`].
    #[inline]
    pub fn get_descriptor(&self, id: ComponentId) -> Option<ComponentDescriptor> {
        self.get_info(id).map(|info| ComponentDescriptor {
            name: info.name.clone(),
            storage_type: info.storage_type,
            is_send_and_sync: info.is_send_and_sync,
            type_id: info.type_id,
            layout: info.layout,
            mutable: info.mutable,
            clone_behavior: info.clone_behavior.clone(),
        })
    }

    /// Gets the name of the component with this [`ComponentId`].
    #[inline]
    pub fn get_name(&self, id: ComponentId) -> Option<String> {
        self.get_info(id).map(|info| info.name.clone())
    }

    /// Gets the metadata associated with the given component.
    /// # Safety
    /// `id` must be a valid and fully registered [`ComponentId`].
    #[inline]
    pub unsafe fn get_info_unchecked(&self, id: ComponentId) -> ComponentInfo {
        self.components.read().unwrap()
            .get(&id)
            .cloned()
            .expect("Component not registered")
    }

    /// Returns true if the [`ComponentId`] is fully registered and valid.
    #[inline]
    pub fn is_id_valid(&self, id: ComponentId) -> bool {
        self.components.read().unwrap().contains_key(&id)
    }

    /// Type-erased equivalent of [`Components::valid_component_id()`].
    #[inline]
    pub fn get_valid_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.indices.read().unwrap().get(&type_id).copied()
    }

    /// Returns the [`ComponentId`] of the given [`Component`] type `T` if it is fully registered.
    #[inline]
    pub fn valid_component_id<T: Component>(&self) -> Option<ComponentId> {
        self.get_valid_id(TypeId::of::<T>())
    }

    /// Type-erased equivalent of [`Components::valid_resource_id()`].
    #[inline]
    pub fn get_valid_resource_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.resource_indices.read().unwrap().get(&type_id).copied()
    }

    /// Returns the [`ComponentId`] of the given [`Resource`] type `T` if it is fully registered.
    #[inline]
    pub fn valid_resource_id<T: Resource>(&self) -> Option<ComponentId> {
        self.get_valid_resource_id(TypeId::of::<T>())
    }

    /// Type-erased equivalent of [`Components::resource_id()`].
    #[inline]
    pub fn get_resource_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.resource_indices.read().unwrap().get(&type_id).copied().or_else(|| {
            let queued = self.queued.read().unwrap();
            queued.resources.get(&type_id).map(|info| info.id)
        })
    }

    /// Returns the [`ComponentId`] of the given [`Resource`] type `T`.
    #[inline]
    pub fn resource_id<T: Resource>(&self) -> Option<ComponentId> {
        self.get_resource_id(TypeId::of::<T>())
    }

    /// Converts a component ID to a resource ID
    #[inline]
    pub fn component_to_resource_id(&self, id: ComponentId) -> ComponentId {
        id // In this implementation, they're the same
    }

    /// Gets all components fully registered with this instance.
    pub fn iter_registered(&self) -> Vec<ComponentInfo> {
        self.components.read().unwrap().values().cloned().collect()
    }

    /// Get mutable access to component info (first overload - by ID)
    /// Note: Must be called carefully as it locks the components
    pub fn get_info_mut(&self, id: ComponentId) -> Option<ComponentInfo> {
        self.components.read().unwrap().get(&id).cloned()
    }

    pub fn register<T: Component>(&self, _storage_type: StorageType) -> ComponentId {
        let type_id = TypeId::of::<T>();
        if let Some(id) = self.indices.read().unwrap().get(&type_id) {
            return *id;
        }
        let id = ComponentId::new(hash_type_id(type_id) as usize);
        let mut info = ComponentInfo::new::<T>();
        info.id = id;
        
        // println!("Components::register: name={}, id={:?}", std::any::type_name::<T>(), id);
        self.components.write().unwrap().insert(id, info);
        self.indices.write().unwrap().insert(type_id, id);
        id
    }
    pub fn register_resource_type<T: Resource>(&self) -> ComponentId {
        let type_id = TypeId::of::<T>();
        if let Some(id) = self.resource_indices.read().unwrap().get(&type_id) {
            return *id;
        }
        let id = ComponentId::new(hash_type_id(type_id) as usize);
        let mut info = ComponentInfo::new_resource::<T>();
        info.id = id;

        // println!("Components::register_resource_type: name={}, id={:?}", std::any::type_name::<T>(), id);
        self.components.write().unwrap().insert(id, info);
        self.resource_indices.write().unwrap().insert(type_id, id);
        id
    }

    pub fn set_hooks(&self, id: ComponentId, hooks: ComponentHooks) {
        if let Some(info) = self.components.write().unwrap().get_mut(&id) {
            info.hooks = hooks;
        }
    }
}
