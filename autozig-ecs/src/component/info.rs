//! Component metadata and registry information

use crate::component::{Component, ComponentCloneBehavior, StorageType};
use crate::resource::Resource;
use std::alloc::Layout;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
}

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
    pub(crate) components: Vec<Option<ComponentInfo>>,
    pub(crate) indices: HashMap<TypeId, ComponentId>,
    pub(crate) resource_indices: HashMap<TypeId, ComponentId>,
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
        self.len() == 0
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
        self.components.iter().filter(|c| c.is_some()).count()
    }

    /// Returns `true` if there are any components registered.
    #[inline]
    pub fn any_registered(&self) -> bool {
        self.num_registered() > 0
    }

    /// Gets the metadata associated with the given component, if it is registered.
    #[inline]
    pub fn get_info(&self, id: ComponentId) -> Option<&ComponentInfo> {
        self.components.get(id.0).and_then(|info| info.as_ref())
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
    pub unsafe fn get_info_unchecked(&self, id: ComponentId) -> &ComponentInfo {
        self.components
            .get(id.0)
            .and_then(|info| info.as_ref())
            .expect("Component not registered")
    }

    /// Returns true if the [`ComponentId`] is fully registered and valid.
    #[inline]
    pub fn is_id_valid(&self, id: ComponentId) -> bool {
        self.components.get(id.0).is_some_and(Option::is_some)
    }

    /// Type-erased equivalent of [`Components::valid_component_id()`].
    #[inline]
    pub fn get_valid_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.indices.get(&type_id).copied()
    }

    /// Returns the [`ComponentId`] of the given [`Component`] type `T` if it is fully registered.
    #[inline]
    pub fn valid_component_id<T: Component>(&self) -> Option<ComponentId> {
        self.get_valid_id(TypeId::of::<T>())
    }

    /// Type-erased equivalent of [`Components::valid_resource_id()`].
    #[inline]
    pub fn get_valid_resource_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.resource_indices.get(&type_id).copied()
    }

    /// Returns the [`ComponentId`] of the given [`Resource`] type `T` if it is fully registered.
    #[inline]
    pub fn valid_resource_id<T: Resource>(&self) -> Option<ComponentId> {
        self.get_valid_resource_id(TypeId::of::<T>())
    }

    /// Type-erased equivalent of [`Components::resource_id()`].
    #[inline]
    pub fn get_resource_id(&self, type_id: TypeId) -> Option<ComponentId> {
        self.resource_indices.get(&type_id).copied().or_else(|| {
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

    /// Gets an iterator over all components fully registered with this instance.
    pub fn iter_registered(&self) -> impl Iterator<Item = &ComponentInfo> + '_ {
        self.components.iter().filter_map(Option::as_ref)
    }

    /// Get mutable access to component info (first overload - by ID)
    pub fn get_info_mut(&mut self, id: ComponentId) -> Option<&mut ComponentInfo> {
        self.components.get_mut(id.0).and_then(|info| info.as_mut())
    }

    /// Get mutable access to component info (second overload - direct)
    pub fn mutable(&mut self, id: ComponentId) -> Option<&mut ComponentInfo> {
        self.get_info_mut(id)
    }

    pub fn register<T: Component>(&mut self, _storage_type: StorageType) -> ComponentId {
        let type_id = TypeId::of::<T>();
        if let Some(id) = self.indices.get(&type_id) {
            return *id;
        }
        let info = ComponentInfo::new::<T>();
        let index = self.components.len();
        self.components.push(Some(info));
        let id = ComponentId::new(index);
        self.indices.insert(type_id, id);
        id
    }
}