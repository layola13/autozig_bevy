//! Plugin Group system for organizing and configuring application plugins
//!
//! Provides PluginGroup trait and PluginGroupBuilder for managing collections of plugins
//! with ordering constraints and enable/disable functionality.

#![forbid(unsafe_code)]

use crate::{App, Plugin, ZigPlugin};
use autozig::include_zig;
use core::any::TypeId;
use core::marker::PhantomData;
use core::ptr::NonNull;

/// Opaque Zig PluginGroupBuilder type
#[repr(C)]
pub struct ZigPluginGroupBuilder {
    _private: [u8; 0],
}

// Include Zig FFI functions
include_zig!("src/zig/plugin_group.zig", {
    fn plugin_group_builder_create(name_ptr: *const u8, name_len: usize) -> *mut ZigPluginGroupBuilder;
    fn plugin_group_builder_destroy(builder: *mut ZigPluginGroupBuilder);
    fn plugin_group_builder_contains(builder: *mut ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_is_enabled(builder: *mut ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_add(builder: *mut ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64) -> bool;
    fn plugin_group_builder_add_before(builder: *mut ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64, target_type_id: u64) -> bool;
    fn plugin_group_builder_add_after(builder: *mut ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64, target_type_id: u64) -> bool;
    fn plugin_group_builder_enable(builder: *mut ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_disable(builder: *mut ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_set(builder: *mut ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64) -> bool;
    fn plugin_group_builder_finish(builder: *mut ZigPluginGroupBuilder, app: *mut crate::ZigApp) -> bool;
    fn plugin_group_builder_len(builder: *mut ZigPluginGroupBuilder) -> usize;
    fn plugin_group_builder_enabled_count(builder: *mut ZigPluginGroupBuilder) -> usize;
});

/// Combines multiple [`Plugin`]s into a single unit
///
/// This trait allows organizing related plugins together and provides
/// methods for configuring them before adding to an App.
pub trait PluginGroup: Sized {
    /// Configures the [`Plugin`]s that are to be added
    fn build(self) -> PluginGroupBuilder;
    
    /// Configures a name for the [`PluginGroup`] which is primarily used for debugging
    fn name() -> &'static str {
        core::any::type_name::<Self>()
    }
    
    /// Sets the value of the given [`Plugin`], if it exists
    fn set<T: Plugin>(self, plugin: T) -> PluginGroupBuilder {
        self.build().set(plugin)
    }
}

/// Facilitates the creation and configuration of a [`PluginGroup`]
///
/// Provides build ordering to ensure that [`Plugin`]s which produce/require resources
/// are built before/after dependent/depending [`Plugin`]s. [`Plugin`]s inside the group
/// can be disabled, enabled or reordered.
pub struct PluginGroupBuilder {
    inner: NonNull<ZigPluginGroupBuilder>,
}

impl PluginGroupBuilder {
    /// Start a new builder for the [`PluginGroup`]
    pub fn start<PG: PluginGroup>() -> Self {
        let name = PG::name();
        let ptr = plugin_group_builder_create(name.as_ptr(), name.len());
        Self {
            inner: NonNull::new(ptr).expect("plugin group builder creation failed"),
        }
    }
    
    /// Checks if the [`PluginGroupBuilder`] contains the given [`Plugin`]
    pub fn contains<T: Plugin>(&self) -> bool {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        plugin_group_builder_contains(self.inner.as_ptr(), type_id)
    }
    
    /// Returns `true` if the [`PluginGroupBuilder`] contains the given [`Plugin`] and it's enabled
    pub fn enabled<T: Plugin>(&self) -> bool {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        plugin_group_builder_is_enabled(self.inner.as_ptr(), type_id)
    }
    
    /// Adds the plugin [`Plugin`] at the end of this [`PluginGroupBuilder`]
    ///
    /// If the plugin was already in the group, it is removed from its previous place.
    pub fn add<T: Plugin>(self, plugin: T) -> Self {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        let plugin_ptr = plugin.into_zig_plugin();
        
        plugin_group_builder_add(self.inner.as_ptr(), plugin_ptr, type_id);
        self
    }
    
    /// Adds a [`Plugin`] in this [`PluginGroupBuilder`] before the plugin of type `Target`
    ///
    /// If the plugin was already in the group, it is removed from its previous place.
    ///
    /// # Panics
    ///
    /// Panics if `Target` is not already in this [`PluginGroupBuilder`].
    pub fn add_before<Target: Plugin, T: Plugin>(self, plugin: T) -> Self {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        let target_type_id = type_id_to_u64(TypeId::of::<Target>());
        let plugin_ptr = plugin.into_zig_plugin();
        
        if !plugin_group_builder_add_before(
            self.inner.as_ptr(),
            plugin_ptr,
            type_id,
            target_type_id,
        ) {
            panic!(
                "Plugin does not exist in group: {}",
                core::any::type_name::<Target>()
            );
        }
        
        self
    }
    
    /// Adds a [`Plugin`] in this [`PluginGroupBuilder`] after the plugin of type `Target`
    ///
    /// If the plugin was already in the group, it is removed from its previous place.
    ///
    /// # Panics
    ///
    /// Panics if `Target` is not already in this [`PluginGroupBuilder`].
    pub fn add_after<Target: Plugin, T: Plugin>(self, plugin: T) -> Self {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        let target_type_id = type_id_to_u64(TypeId::of::<Target>());
        let plugin_ptr = plugin.into_zig_plugin();
        
        if !plugin_group_builder_add_after(
            self.inner.as_ptr(),
            plugin_ptr,
            type_id,
            target_type_id,
        ) {
            panic!(
                "Plugin does not exist in group: {}",
                core::any::type_name::<Target>()
            );
        }
        
        self
    }
    
    /// Enables a [`Plugin`]
    ///
    /// [`Plugin`]s within a [`PluginGroup`] are enabled by default. This function is used to
    /// opt back in to a [`Plugin`] after disabling it.
    ///
    /// # Panics
    ///
    /// Panics if there are no plugins of type `T` in this group.
    pub fn enable<T: Plugin>(self) -> Self {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        
        if !plugin_group_builder_enable(self.inner.as_ptr(), type_id) {
            panic!(
                "Cannot enable a plugin that does not exist: {}",
                core::any::type_name::<T>()
            );
        }
        
        self
    }
    
    /// Disables a [`Plugin`], preventing it from being added to the [`App`]
    ///
    /// The disabled [`Plugin`] keeps its place in the [`PluginGroup`], so it can
    /// still be used for ordering with [`add_before`](Self::add_before) or
    /// [`add_after`](Self::add_after), or it can be re-enabled.
    ///
    /// # Panics
    ///
    /// Panics if there are no plugins of type `T` in this group.
    pub fn disable<T: Plugin>(self) -> Self {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        
        if !plugin_group_builder_disable(self.inner.as_ptr(), type_id) {
            panic!(
                "Cannot disable a plugin that does not exist: {}",
                core::any::type_name::<T>()
            );
        }
        
        self
    }
    
    /// Sets the value of the given [`Plugin`], if it exists
    ///
    /// # Panics
    ///
    /// Panics if the [`Plugin`] does not exist.
    pub fn set<T: Plugin>(self, plugin: T) -> Self {
        let type_id = type_id_to_u64(TypeId::of::<T>());
        let plugin_ptr = plugin.into_zig_plugin();
        
        if !plugin_group_builder_set(self.inner.as_ptr(), plugin_ptr, type_id) {
            panic!(
                "{} does not exist in this PluginGroup",
                core::any::type_name::<T>()
            );
        }
        
        self
    }
    
    /// Consumes the [`PluginGroupBuilder`] and builds the contained [`Plugin`]s
    /// in the order specified
    ///
    /// # Panics
    ///
    /// Panics if one of the plugins in the group was already added to the application.
    pub fn finish(self, app: &mut App) {
        if !plugin_group_builder_finish(self.inner.as_ptr(), app.inner.as_ptr()) {
            panic!("Failed to finish plugin group");
        }
        // Builder is consumed, drop will clean up
    }
    
    /// Get number of plugins in the builder
    pub fn len(&self) -> usize {
        plugin_group_builder_len(self.inner.as_ptr())
    }
    
    /// Check if the builder is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Get number of enabled plugins
    pub fn enabled_count(&self) -> usize {
        plugin_group_builder_enabled_count(self.inner.as_ptr())
    }
}

impl Drop for PluginGroupBuilder {
    fn drop(&mut self) {
        plugin_group_builder_destroy(self.inner.as_ptr());
    }
}

// Implement PluginGroup for PluginGroupBuilder to allow chaining
impl PluginGroup for PluginGroupBuilder {
    fn build(self) -> PluginGroupBuilder {
        self
    }
}

/// Helper function to convert TypeId to u64
fn type_id_to_u64(type_id: TypeId) -> u64 {
    use core::hash::Hasher;
    
    let mut hasher = TypeIdHasher::default();
    core::hash::Hash::hash(&type_id, &mut hasher);
    hasher.finish()
}

/// Simple TypeId hasher implementation (completely safe)
#[derive(Default)]
struct TypeIdHasher {
    state: u64,
}

impl core::hash::Hasher for TypeIdHasher {
    fn write(&mut self, bytes: &[u8]) {
        // FNV-1a hash algorithm (completely safe implementation)
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;
        
        if self.state == 0 {
            self.state = FNV_OFFSET_BASIS;
        }
        
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }
    
    fn finish(&self) -> u64 {
        self.state
    }
}

/// Extension trait for App to support plugin groups
pub trait PluginGroupExt {
    /// Add a [`PluginGroup`] to the application
    fn add_plugin_group<G: PluginGroup>(&mut self, group: G) -> &mut Self;
}

impl PluginGroupExt for App {
    fn add_plugin_group<G: PluginGroup>(&mut self, group: G) -> &mut Self {
        let builder = group.build();
        builder.finish(self);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimplePlugin;
    
    #[test]
    fn test_plugin_group_builder_create() {
        let builder = PluginGroupBuilder::start::<TestPluginGroup>();
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
    }
    
    #[test]
    fn test_plugin_group_add() {
        let plugin = SimplePlugin::new("test", |_| {});
        let builder = PluginGroupBuilder::start::<TestPluginGroup>().add(plugin);
        
        assert_eq!(builder.len(), 1);
        assert!(!builder.is_empty());
        assert!(builder.contains::<SimplePlugin>());
    }
    
    struct TestPluginGroup;
    
    impl PluginGroup for TestPluginGroup {
        fn build(self) -> PluginGroupBuilder {
            PluginGroupBuilder::start::<Self>()
        }
    }
}