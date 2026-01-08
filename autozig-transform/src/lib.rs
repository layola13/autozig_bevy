//! AutoZig Transform - Bevy transform system for WebGPU/WASM platforms
//! 
//! This crate provides 2D/3D transformation components and systems using
//! Zig for high-performance computations.

use autozig::include_zig;

/// Transform component with translation, rotation (quaternion), and scale
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: [f32; 3],
    pub rotation: [f32; 4], // Quaternion (x, y, z, w)
    pub scale: [f32; 3],
}

/// Hierarchy component for parent-child relationships
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hierarchy {
    pub parent: u32,
    pub children: [u32; 32],
    pub children_count: u32,
}

/// LocalToWorld component storing world-space transformation matrix
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalToWorld {
    pub matrix: [f32; 16], // 4x4 matrix in column-major order
}

// Include Zig implementations
include_zig!("src/zig/transform.zig", {
    fn transform_identity() -> Transform;
    fn transform_from_translation(translation: [f32; 3]) -> Transform;
    fn transform_from_rotation(rotation: [f32; 4]) -> Transform;
    fn transform_from_scale(scale: [f32; 3]) -> Transform;
    fn transform_compute_matrix(transform: *const Transform, out_matrix: *mut [f32; 16]);
    fn transform_compute_local_to_world(
        transform: *const Transform,
        parent_matrix: [f32; 16],
        out_matrix: *mut [f32; 16]
    );
});

include_zig!("src/zig/hierarchy.zig", {
    fn hierarchy_create() -> Hierarchy;
    fn hierarchy_add_child(hierarchy: *mut Hierarchy, child_id: u32) -> bool;
    fn hierarchy_remove_child(hierarchy: *mut Hierarchy, child_id: u32) -> bool;
    fn hierarchy_get_children_count(hierarchy: *const Hierarchy) -> u32;
    fn hierarchy_get_child(hierarchy: *const Hierarchy, index: u32) -> u32;
    fn hierarchy_has_parent(hierarchy: *const Hierarchy) -> bool;
    fn hierarchy_has_children(hierarchy: *const Hierarchy) -> bool;
    fn hierarchy_clear_children(hierarchy: *mut Hierarchy);
    fn hierarchy_set_parent(hierarchy: *mut Hierarchy, parent_id: u32);
    fn hierarchy_clear_parent(hierarchy: *mut Hierarchy);
    fn hierarchy_get_parent(hierarchy: *const Hierarchy) -> u32;
});

include_zig!("src/zig/local_to_world.zig", {
    fn local_to_world_identity() -> LocalToWorld;
    fn local_to_world_from_matrix(matrix: [f32; 16]) -> LocalToWorld;
    fn local_to_world_get_matrix(ltw: *const LocalToWorld, out_matrix: *mut [f32; 16]);
    fn local_to_world_set_matrix(ltw: *mut LocalToWorld, matrix: [f32; 16]);
    fn local_to_world_get_translation(ltw: *const LocalToWorld, out_translation: *mut [f32; 3]);
    fn local_to_world_get_scale(ltw: *const LocalToWorld, out_scale: *mut [f32; 3]);
    fn local_to_world_multiply(ltw: *mut LocalToWorld, other: [f32; 16]);
    fn local_to_world_copy_from(dest: *mut LocalToWorld, src: *const LocalToWorld);
    fn local_to_world_is_identity(ltw: *const LocalToWorld) -> bool;
});

include_zig!("src/zig/system.zig", {
    fn transform_system_update_hierarchy(
        hierarchy: *mut Hierarchy,
        transform: *mut Transform,
        local_to_world: *mut LocalToWorld
    );
    fn transform_system_update_local_to_world(
        transform: *mut Transform,
        parent_local_to_world: *mut LocalToWorld,
        local_to_world: *mut LocalToWorld
    );
    fn transform_system_update_root_local_to_world(
        transform: *mut Transform,
        local_to_world: *mut LocalToWorld
    );
    fn transform_system_propagate_transforms(
        transforms: *mut Transform,
        hierarchies: *mut Hierarchy,
        local_to_worlds: *mut LocalToWorld,
        entity_id: u32,
        parent_ltw: *const LocalToWorld
    );
});

// Transform implementation
impl Transform {
    /// Create identity transform
    pub fn identity() -> Self {
        transform_identity()
    }

    /// Create transform from translation
    pub fn from_translation(translation: [f32; 3]) -> Self {
        transform_from_translation(translation)
    }

    /// Create transform from rotation (quaternion)
    pub fn from_rotation(rotation: [f32; 4]) -> Self {
        transform_from_rotation(rotation)
    }

    /// Create transform from scale
    pub fn from_scale(scale: [f32; 3]) -> Self {
        transform_from_scale(scale)
    }

    /// Compute 4x4 transformation matrix
    pub fn compute_matrix(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        transform_compute_matrix(self, &mut matrix);
        matrix
    }

    /// Compute local-to-world matrix with parent matrix
    pub fn compute_local_to_world(&self, parent_matrix: [f32; 16]) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        transform_compute_local_to_world(self, parent_matrix, &mut matrix);
        matrix
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

// Hierarchy implementation
impl Hierarchy {
    /// Create new hierarchy
    pub fn new() -> Self {
        hierarchy_create()
    }

    /// Add a child
    pub fn add_child(&mut self, child_id: u32) -> bool {
        hierarchy_add_child(self, child_id)
    }

    /// Remove a child
    pub fn remove_child(&mut self, child_id: u32) -> bool {
        hierarchy_remove_child(self, child_id)
    }

    /// Get number of children
    pub fn children_count(&self) -> u32 {
        hierarchy_get_children_count(self)
    }

    /// Get child at index
    pub fn get_child(&self, index: u32) -> u32 {
        hierarchy_get_child(self, index)
    }

    /// Get all children as slice
    pub fn children(&self) -> &[u32] {
        &self.children[0..self.children_count as usize]
    }

    /// Check if has parent
    pub fn has_parent(&self) -> bool {
        hierarchy_has_parent(self)
    }

    /// Check if has children
    pub fn has_children(&self) -> bool {
        hierarchy_has_children(self)
    }

    /// Clear all children
    pub fn clear_children(&mut self) {
        hierarchy_clear_children(self)
    }

    /// Set parent
    pub fn set_parent(&mut self, parent_id: u32) {
        hierarchy_set_parent(self, parent_id)
    }

    /// Clear parent
    pub fn clear_parent(&mut self) {
        hierarchy_clear_parent(self)
    }

    /// Get parent ID
    pub fn parent(&self) -> u32 {
        hierarchy_get_parent(self)
    }
}

impl Default for Hierarchy {
    fn default() -> Self {
        Self::new()
    }
}

// LocalToWorld implementation
impl LocalToWorld {
    /// Create identity local-to-world
    pub fn identity() -> Self {
        local_to_world_identity()
    }

    /// Create from matrix
    pub fn from_matrix(matrix: [f32; 16]) -> Self {
        local_to_world_from_matrix(matrix)
    }

    /// Get matrix
    pub fn matrix(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        local_to_world_get_matrix(self, &mut matrix);
        matrix
    }

    /// Set matrix
    pub fn set_matrix(&mut self, matrix: [f32; 16]) {
        local_to_world_set_matrix(self, matrix)
    }

    /// Get translation
    pub fn translation(&self) -> [f32; 3] {
        let mut translation = [0.0f32; 3];
        local_to_world_get_translation(self, &mut translation);
        translation
    }

    /// Get scale
    pub fn scale(&self) -> [f32; 3] {
        let mut scale = [0.0f32; 3];
        local_to_world_get_scale(self, &mut scale);
        scale
    }

    /// Multiply by another matrix
    pub fn multiply(&mut self, other: [f32; 16]) {
        local_to_world_multiply(self, other)
    }

    /// Copy from another LocalToWorld
    pub fn copy_from(&mut self, other: &LocalToWorld) {
        local_to_world_copy_from(self, other)
    }

    /// Check if identity
    pub fn is_identity(&self) -> bool {
        local_to_world_is_identity(self)
    }
}

impl Default for LocalToWorld {
    fn default() -> Self {
        Self::identity()
    }
}

// Transform System
pub struct TransformSystem;

impl TransformSystem {
    /// Update hierarchy
    pub fn update_hierarchy(
        hierarchy: &mut Hierarchy,
        transform: &mut Transform,
        local_to_world: &mut LocalToWorld,
    ) {
        transform_system_update_hierarchy(hierarchy, transform, local_to_world);
    }

    /// Update local-to-world with parent
    pub fn update_local_to_world(
        transform: &mut Transform,
        parent_local_to_world: &mut LocalToWorld,
        local_to_world: &mut LocalToWorld,
    ) {
        transform_system_update_local_to_world(transform, parent_local_to_world, local_to_world);
    }

    /// Update root local-to-world (no parent)
    pub fn update_root_local_to_world(transform: &mut Transform, local_to_world: &mut LocalToWorld) {
        transform_system_update_root_local_to_world(transform, local_to_world);
    }

    /// Propagate transforms through hierarchy
    pub fn propagate_transforms(
        transforms: &mut [Transform],
        hierarchies: &mut [Hierarchy],
        local_to_worlds: &mut [LocalToWorld],
        entity_id: u32,
        parent_ltw: &LocalToWorld,
    ) {
        transform_system_propagate_transforms(
            transforms.as_mut_ptr(),
            hierarchies.as_mut_ptr(),
            local_to_worlds.as_mut_ptr(),
            entity_id,
            parent_ltw,
        );
    }
}