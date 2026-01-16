//! AutoZig Transform - Bevy transform system for WebGPU/WASM platforms
//! 
//! This crate provides 2D/3D transformation components and systems using
//! Zig for high-performance computations.

use autozig::include_zig;
use core::ops::Mul;

/// Transform component with translation, rotation (quaternion), and scale
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: [f32; 3],
    pub rotation: [f32; 4], // Quaternion (x, y, z, w)
    pub scale: [f32; 3],
}

/// GlobalTransform is an affine transformation from entity-local coordinates to worldspace coordinates.
/// 
/// You cannot directly mutate GlobalTransform; instead, you change an entity's transform by manipulating
/// its Transform, which indirectly causes the system to update its GlobalTransform.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalTransform {
    /// 4x4 affine transformation matrix in column-major order
    pub matrix: [f32; 16],
}

/// An optimization marker component for transform propagation.
/// 
/// This ZST marker component uses change detection to mark all entities of the hierarchy 
/// as "dirty" if any of their descendants have a changed Transform.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TransformTreeChanged;

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

/// Configure the behavior of static scene optimizations for Transform propagation.
/// 
/// For scenes with many static entities, it is much faster to track trees of unchanged
/// Transforms and skip these during the expensive transform propagation step.
#[derive(Debug, Clone)]
pub struct StaticTransformOptimizations {
    /// If the percentage of moving objects exceeds this value, skip dirty tree marking.
    pub threshold: f32,
    /// Updated every frame by mark_dirty_trees.
    pub enabled: bool,
}

impl StaticTransformOptimizations {
    /// If the percentage of moving objects exceeds this threshold, disable static Transform
    /// optimizations.
    pub fn from_threshold(threshold: f32) -> Self {
        Self {
            threshold,
            enabled: true,
        }
    }

    /// Unconditionally disable static scene optimizations.
    pub fn disabled() -> Self {
        Self {
            threshold: 0.0,
            enabled: false,
        }
    }

    /// Unconditionally enable static scene optimizations.
    pub fn enabled() -> Self {
        Self {
            threshold: 1.0,
            enabled: true,
        }
    }
}

impl Default for StaticTransformOptimizations {
    fn default() -> Self {
        Self {
            threshold: 0.3,
            enabled: true,
        }
    }
}

/// System labels for transform propagation scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransformSystems {
    /// Marks dirty trees for optimization
    MarkDirtyTrees,
    /// Syncs simple transforms (entities without hierarchy)
    SyncSimpleTransforms,
    /// Propagates transforms through the hierarchy
    Propagate,
}

/// Error types for global transform computation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeGlobalTransformError {
    /// The entity doesn't have a Transform component
    MissingTransform,
    /// The entity doesn't have a GlobalTransform component
    MissingGlobalTransform,
    /// The parent entity is invalid
    InvalidParent,
    /// Cycle detected in the hierarchy
    CycleDetected,
}

impl core::fmt::Display for ComputeGlobalTransformError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingTransform => write!(f, "Entity is missing Transform component"),
            Self::MissingGlobalTransform => write!(f, "Entity is missing GlobalTransform component"),
            Self::InvalidParent => write!(f, "Parent entity is invalid"),
            Self::CycleDetected => write!(f, "Cycle detected in transform hierarchy"),
        }
    }
}

/// Transform plugin that registers transform systems
pub struct TransformPlugin;

impl TransformPlugin {
    /// Create a new transform plugin
    pub fn new() -> Self {
        Self
    }

    /// Build the plugin (register systems)
    pub fn build(&self) {
        // In a real implementation, this would register systems with the ECS
        // For now, this is a placeholder that documents the expected behavior
    }
}

impl Default for TransformPlugin {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for transform operations
pub struct TransformHelper;

impl TransformHelper {
    /// Compute global transform from local transform and parent's global transform
    pub fn compute_global_transform(
        local: &Transform,
        parent_global: &GlobalTransform,
    ) -> GlobalTransform {
        let local_matrix = local.compute_matrix();
        let mut result_matrix = [0.0f32; 16];
        
        // Matrix multiplication: parent * local
        for i in 0..4 {
            for j in 0..4 {
                result_matrix[i * 4 + j] = 0.0;
                for k in 0..4 {
                    result_matrix[i * 4 + j] +=
                        parent_global.matrix[i * 4 + k] * local_matrix[k * 4 + j];
                }
            }
        }
        
        GlobalTransform {
            matrix: result_matrix,
        }
    }

    /// Compute global transform for root entity (no parent)
    pub fn compute_root_global_transform(local: &Transform) -> GlobalTransform {
        GlobalTransform {
            matrix: local.compute_matrix(),
        }
    }

    /// Extract translation from global transform
    pub fn get_translation(global: &GlobalTransform) -> [f32; 3] {
        [
            global.matrix[12],
            global.matrix[13],
            global.matrix[14],
        ]
    }

    /// Extract scale from global transform (approximate)
    pub fn get_scale(global: &GlobalTransform) -> [f32; 3] {
        let x_len = (global.matrix[0] * global.matrix[0]
            + global.matrix[1] * global.matrix[1]
            + global.matrix[2] * global.matrix[2])
            .sqrt();
        let y_len = (global.matrix[4] * global.matrix[4]
            + global.matrix[5] * global.matrix[5]
            + global.matrix[6] * global.matrix[6])
            .sqrt();
        let z_len = (global.matrix[8] * global.matrix[8]
            + global.matrix[9] * global.matrix[9]
            + global.matrix[10] * global.matrix[10])
            .sqrt();
        [x_len, y_len, z_len]
    }
}

/// Work queue for parallel transform computation
pub struct WorkQueue {
    /// Entities pending processing
    pub entities: Vec<u32>,
    /// Current processing index
    pub index: usize,
}

impl WorkQueue {
    /// Create a new work queue
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            index: 0,
        }
    }

    /// Create a work queue with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: Vec::with_capacity(capacity),
            index: 0,
        }
    }

    /// Push an entity to the queue
    pub fn push(&mut self, entity: u32) {
        self.entities.push(entity);
    }

    /// Pop an entity from the queue
    pub fn pop(&mut self) -> Option<u32> {
        if self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;
            Some(entity)
        } else {
            None
        }
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.index >= self.entities.len()
    }

    /// Get remaining count
    pub fn len(&self) -> usize {
        self.entities.len() - self.index
    }

    /// Clear the queue
    pub fn clear(&mut self) {
        self.entities.clear();
        self.index = 0;
    }

    /// Reset index to beginning
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl Default for WorkQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for transforming points in space
pub trait TransformPoint {
    /// Transform a point from local space to the space defined by this transform
    fn transform_point(&self, point: [f32; 3]) -> [f32; 3];

    /// Transform a vector (direction) - excludes translation
    fn transform_vector(&self, vector: [f32; 3]) -> [f32; 3];
}

/// Extension trait for building children with transforms
pub trait BuildChildrenTransformExt {
    /// Add a child entity with a transform
    fn with_child_transform(&mut self, transform: Transform) -> u32;

    /// Add multiple children with transforms
    fn with_children_transforms(&mut self, transforms: &[Transform]) -> Vec<u32>;
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

include_zig!("src/zig/global_transform.zig", {
    fn global_transform_identity() -> GlobalTransform;
    fn global_transform_from_matrix(matrix: *const [f32; 16]) -> GlobalTransform;
    fn global_transform_from_transform(transform: *const Transform) -> GlobalTransform;
    fn global_transform_mul_transform(
        global: *const GlobalTransform,
        transform: *const Transform,
        out: *mut GlobalTransform
    );
    fn global_transform_transform_point(
        global: *const GlobalTransform,
        point: *const [f32; 3],
        out: *mut [f32; 3]
    );
    fn global_transform_transform_vector(
        global: *const GlobalTransform,
        vector: *const [f32; 3],
        out: *mut [f32; 3]
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

impl TransformPoint for Transform {
    fn transform_point(&self, mut point: [f32; 3]) -> [f32; 3] {
        // Apply scale
        point[0] *= self.scale[0];
        point[1] *= self.scale[1];
        point[2] *= self.scale[2];

        // Apply rotation (quaternion rotation)
        let qx = self.rotation[0];
        let qy = self.rotation[1];
        let qz = self.rotation[2];
        let qw = self.rotation[3];

        let ix = qw * point[0] + qy * point[2] - qz * point[1];
        let iy = qw * point[1] + qz * point[0] - qx * point[2];
        let iz = qw * point[2] + qx * point[1] - qy * point[0];
        let iw = -qx * point[0] - qy * point[1] - qz * point[2];

        point[0] = ix * qw + iw * -qx + iy * -qz - iz * -qy;
        point[1] = iy * qw + iw * -qy + iz * -qx - ix * -qz;
        point[2] = iz * qw + iw * -qz + ix * -qy - iy * -qx;

        // Apply translation
        point[0] += self.translation[0];
        point[1] += self.translation[1];
        point[2] += self.translation[2];

        point
    }

    fn transform_vector(&self, mut vector: [f32; 3]) -> [f32; 3] {
        // Apply scale
        vector[0] *= self.scale[0];
        vector[1] *= self.scale[1];
        vector[2] *= self.scale[2];

        // Apply rotation (quaternion rotation)
        let qx = self.rotation[0];
        let qy = self.rotation[1];
        let qz = self.rotation[2];
        let qw = self.rotation[3];

        let ix = qw * vector[0] + qy * vector[2] - qz * vector[1];
        let iy = qw * vector[1] + qz * vector[0] - qx * vector[2];
        let iz = qw * vector[2] + qx * vector[1] - qy * vector[0];
        let iw = -qx * vector[0] - qy * vector[1] - qz * vector[2];

        vector[0] = ix * qw + iw * -qx + iy * -qz - iz * -qy;
        vector[1] = iy * qw + iw * -qy + iz * -qx - ix * -qz;
        vector[2] = iz * qw + iw * -qz + ix * -qy - iy * -qx;

        vector
    }
}

// GlobalTransform implementation
impl GlobalTransform {
    /// An identity GlobalTransform
    pub const IDENTITY: Self = Self {
        matrix: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    };

    /// Create identity GlobalTransform
    pub fn identity() -> Self {
        global_transform_identity()
    }

    /// Create from matrix
    pub fn from_matrix(matrix: [f32; 16]) -> Self {
        global_transform_from_matrix(&matrix)
    }

    /// Create from Transform
    pub fn from_transform(transform: &Transform) -> Self {
        global_transform_from_transform(transform)
    }

    /// Get the transformation matrix
    pub fn matrix(&self) -> &[f32; 16] {
        &self.matrix
    }

    /// Get translation from the matrix
    pub fn translation(&self) -> [f32; 3] {
        TransformHelper::get_translation(self)
    }

    /// Get scale from the matrix
    pub fn scale(&self) -> [f32; 3] {
        TransformHelper::get_scale(self)
    }

    /// Multiply with a Transform
    pub fn mul_transform(&self, transform: &Transform) -> Self {
        let mut result = Self::IDENTITY;
        global_transform_mul_transform(self, transform, &mut result);
        result
    }

    /// Compute a Transform from this GlobalTransform
    pub fn compute_transform(&self) -> Transform {
        let translation = self.translation();
        let scale = self.scale();
        
        // Extract rotation (simplified - assumes no shear)
        let rotation = [0.0, 0.0, 0.0, 1.0]; // Identity quaternion
        
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    /// Returns the Transform self would have if it was a child of an entity with the parent GlobalTransform
    pub fn reparented_to(&self, parent: &GlobalTransform) -> Transform {
        // This is a simplified implementation
        // In a full implementation, we'd compute: parent^-1 * self
        self.compute_transform()
    }
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Transform> for GlobalTransform {
    fn from(transform: Transform) -> Self {
        GlobalTransform::from_transform(&transform)
    }
}

impl From<[f32; 16]> for GlobalTransform {
    fn from(matrix: [f32; 16]) -> Self {
        GlobalTransform::from_matrix(matrix)
    }
}

impl Mul<GlobalTransform> for GlobalTransform {
    type Output = GlobalTransform;

    fn mul(self, other: GlobalTransform) -> Self::Output {
        let mut result = [0.0f32; 16];
        // Matrix multiplication
        for i in 0..4 {
            for j in 0..4 {
                result[i * 4 + j] = 0.0;
                for k in 0..4 {
                    result[i * 4 + j] += self.matrix[i * 4 + k] * other.matrix[k * 4 + j];
                }
            }
        }
        GlobalTransform { matrix: result }
    }
}

impl Mul<Transform> for GlobalTransform {
    type Output = GlobalTransform;

    fn mul(self, transform: Transform) -> Self::Output {
        self.mul_transform(&transform)
    }
}

impl Mul<[f32; 3]> for GlobalTransform {
    type Output = [f32; 3];

    fn mul(self, point: [f32; 3]) -> Self::Output {
        self.transform_point(point)
    }
}

impl TransformPoint for GlobalTransform {
    fn transform_point(&self, point: [f32; 3]) -> [f32; 3] {
        let mut result = [0.0f32; 3];
        global_transform_transform_point(self, &point, &mut result);
        result
    }

    fn transform_vector(&self, vector: [f32; 3]) -> [f32; 3] {
        let mut result = [0.0f32; 3];
        global_transform_transform_vector(self, &vector, &mut result);
        result
    }
}

// Implement Component for Transform types
impl autozig_ecs::component::Component for Transform {}
impl autozig_ecs::component::Component for GlobalTransform {}
impl autozig_ecs::component::Component for Hierarchy {}
impl autozig_ecs::component::Component for LocalToWorld {}