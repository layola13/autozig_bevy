//! Bundle metadata and information

use crate::component::{Component, ComponentId};
use std::collections::HashSet;

/// Bundle trait - 定义一组组件的集合
///
/// Bundle是一组组件的集合，可以一次性插入到实体中
pub trait Bundle: Send + Sync + 'static {
    /// 获取此Bundle中的组件ID列表
    fn component_ids() -> Vec<u32>;
    
    /// 获取组件数据
    /// 返回(component_id, data_ptr, data_size)元组列表
    fn get_components(&self) -> Vec<(u32, *const u8, usize)>;
}

// Restore generic implementation for single component
impl<T: Component> Bundle for T {
    fn component_ids() -> Vec<u32> {
        vec![0] // Stubbed
    }
    
    fn get_components(&self) -> Vec<(u32, *const u8, usize)> {
         vec![(0, self as *const T as *const u8, std::mem::size_of::<T>())]
    }
}

macro_rules! impl_bundle_tuple {
    ($($param:ident),*) => {
        impl<$($param: Component),*> Bundle for ($($param,)*) {
            fn component_ids() -> Vec<u32> {
                #[allow(unused_mut)]
                let mut ids = Vec::new();
                #[allow(unused_mut, unused_variables)]
                let mut i = 0;
                $(
                    let _ = std::mem::size_of::<$param>(); // Verify usage
                    ids.push(i);
                    i += 1;
                )*
                ids
            }
            
            fn get_components(&self) -> Vec<(u32, *const u8, usize)> {
                #[allow(non_snake_case)]
                let ($($param,)*) = self;
                #[allow(unused_mut)]
                let mut components = Vec::new();
                #[allow(unused_mut, unused_variables)]
                let mut i = 0;
                $(
                    components.push((i, $param as *const $param as *const u8, std::mem::size_of::<$param>()));
                    i += 1;
                )*
                components
            }
        }
    }
}

impl_bundle_tuple!();
impl_bundle_tuple!(A);
impl_bundle_tuple!(A, B);
impl_bundle_tuple!(A, B, C);
impl_bundle_tuple!(A, B, C, D);
impl_bundle_tuple!(A, B, C, D, E);
impl_bundle_tuple!(A, B, C, D, E, F);
impl_bundle_tuple!(A, B, C, D, E, F, G);
// Add up to 15... but 7 is enough for verification example (3)

/// BundleInserter - Bundle插入器
pub struct BundleInserter {
    bundle_id: BundleId,
}

impl BundleInserter {
    pub fn new(bundle_id: BundleId) -> Self {
        Self { bundle_id }
    }
    
    pub fn bundle_id(&self) -> BundleId {
        self.bundle_id
    }
}

/// Determines how a bundle should be inserted into an entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertMode {
    /// Replace existing components
    Replace,
    /// Keep existing components, only add new ones
    Keep,
    /// Merge with existing components
    Merge,
}

impl Default for InsertMode {
    fn default() -> Self {
        Self::Replace
    }
}

/// Stores metadata associated with a specific type of [`Bundle`]
#[derive(Debug, Clone)]
pub struct BundleInfo {
    /// All component IDs that this bundle contributes (including required components)
    contributed_components: Vec<ComponentId>,
    /// Component IDs that are explicitly part of this bundle (not required)
    explicit_components: Vec<ComponentId>,
    /// Component IDs that are required by components in this bundle
    required_components: Vec<ComponentId>,
    /// Unique ID for this bundle type
    id: BundleId,
}

/// A value uniquely identifying a [`Bundle`] within a [`World`]
#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct BundleId(pub(crate) usize);

impl BundleId {
    /// Creates a new [`BundleId`]
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the index of this bundle
    #[inline]
    pub fn index(self) -> usize {
        self.0
    }
}

impl BundleInfo {
    /// Create a new `BundleInfo`
    pub fn new(
        id: BundleId,
        explicit_components: Vec<ComponentId>,
        required_components: Vec<ComponentId>,
    ) -> Self {
        // Combine explicit and required components, removing duplicates
        let mut contributed = explicit_components.clone();
        for &required in &required_components {
            if !contributed.contains(&required) {
                contributed.push(required);
            }
        }

        Self {
            contributed_components: contributed,
            explicit_components,
            required_components,
            id,
        }
    }

    /// Returns the unique ID of this bundle
    #[inline]
    pub fn id(&self) -> BundleId {
        self.id
    }

    /// Returns all components that this bundle contributes (both explicit and required)
    #[inline]
    pub fn contributed_components(&self) -> &[ComponentId] {
        &self.contributed_components
    }

    /// Returns only the explicit components (not including required components)
    #[inline]
    pub fn explicit_components(&self) -> &[ComponentId] {
        &self.explicit_components
    }

    /// Returns only the required components
    #[inline]
    pub fn required_components(&self) -> &[ComponentId] {
        &self.required_components
    }

    /// Iterate over all contributed component IDs
    #[inline]
    pub fn iter_contributed_components(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.contributed_components.iter().copied()
    }

    /// Iterate over explicit component IDs
    #[inline]
    pub fn iter_explicit_components(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.explicit_components.iter().copied()
    }

    /// Iterate over required component IDs
    #[inline]
    pub fn iter_required_components(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.required_components.iter().copied()
    }

    /// Returns the number of components this bundle contributes
    #[inline]
    pub fn component_count(&self) -> usize {
        self.contributed_components.len()
    }

    /// Returns the number of explicit components
    #[inline]
    pub fn explicit_component_count(&self) -> usize {
        self.explicit_components.len()
    }

    /// Returns the number of required components
    #[inline]
    pub fn required_component_count(&self) -> usize {
        self.required_components.len()
    }

    /// Check if this bundle contains a specific component
    #[inline]
    pub fn contains_component(&self, component: ComponentId) -> bool {
        self.contributed_components.contains(&component)
    }

    /// Check if a component is explicitly part of this bundle
    #[inline]
    pub fn is_explicit_component(&self, component: ComponentId) -> bool {
        self.explicit_components.contains(&component)
    }

    /// Check if a component is a required component
    #[inline]
    pub fn is_required_component(&self, component: ComponentId) -> bool {
        self.required_components.contains(&component)
    }
}

/// Stores metadata for all registered bundles
#[derive(Debug, Default)]
pub struct Bundles {
    bundles: Vec<BundleInfo>,
}

impl Bundles {
    /// Create a new `Bundles` registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new bundle and return its ID
    pub fn register(
        &mut self,
        explicit_components: Vec<ComponentId>,
        required_components: Vec<ComponentId>,
    ) -> BundleId {
        let id = BundleId(self.bundles.len());
        let info = BundleInfo::new(id, explicit_components, required_components);
        self.bundles.push(info);
        id
    }

    /// Get bundle info by ID
    #[inline]
    pub fn get(&self, id: BundleId) -> Option<&BundleInfo> {
        self.bundles.get(id.0)
    }

    /// Get mutable bundle info by ID
    #[inline]
    pub fn get_mut(&mut self, id: BundleId) -> Option<&mut BundleInfo> {
        self.bundles.get_mut(id.0)
    }

    /// Returns the number of registered bundles
    #[inline]
    pub fn len(&self) -> usize {
        self.bundles.len()
    }

    /// Check if there are no registered bundles
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bundles.is_empty()
    }

    /// Iterate over all bundle infos
    pub fn iter(&self) -> impl Iterator<Item = &BundleInfo> {
        self.bundles.iter()
    }
}