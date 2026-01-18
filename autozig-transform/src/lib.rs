//! AutoZig Transform - Bevy transform system for WebGPU/WASM platforms
//! 
//! This crate provides 2D/3D transformation components and systems using
//! Zig for high-performance computations. API compatible with Bevy's bevy_transform.

use autozig::include_zig;
use core::ops::Mul;
use autozig_math::{Vec3, Quat, Mat4, Mat3, Dir3, Affine3A, Isometry3d};
use autozig_ecs::component::Component;

/// Transform component with translation, rotation (quaternion), and scale.
/// 
/// Describe the position of an entity. If the entity has a parent, the position is relative
/// to its parent position.
///
/// * To place or move an entity, you should set its [`Transform`].
/// * To get the global transform of an entity, you should get its [`GlobalTransform`].
/// * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    /// Position of the entity. In 2d, the last value of the Vec3 is used for z-ordering.
    pub translation: Vec3,
    /// Rotation of the entity.
    pub rotation: Quat,
    /// Scale of the entity.
    pub scale: Vec3,
}

impl Transform {
    /// An identity [`Transform`] with no translation, rotation, and a scale of 1 on all axes.
    pub const IDENTITY: Self = Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    /// Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
    /// is used for z-ordering elements: higher `z`-value will be in front of lower `z`-value.
    #[inline]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::from_translation(Vec3::new(x, y, z))
    }

    /// Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
    /// transformation matrix.
    #[inline]
    pub fn from_matrix(world_from_local: Mat4) -> Self {
        let (scale, rotation, translation) = world_from_local.to_scale_rotation_translation();
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    /// Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on all axes.
    #[inline]
    pub const fn from_translation(translation: Vec3) -> Self {
        Transform {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on all axes.
    #[inline]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Transform {
            translation: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
        }
    }

    /// Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on all axes.
    #[inline]
    pub const fn from_scale(scale: Vec3) -> Self {
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
        }
    }

    /// Creates a new [`Transform`] that is equivalent to the given [isometry].
    #[inline]
    pub fn from_isometry(iso: Isometry3d) -> Self {
        Transform {
            translation: iso.translation(),
            rotation: iso.rotation(),
            scale: Vec3::ONE,
        }
    }

    /// Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
    /// points towards the `target` position and [`Transform::up`] points towards `up`.
    #[inline]
    #[must_use]
    pub fn looking_at(mut self, target: Vec3, up: Vec3) -> Self {
        self.look_at(target, up);
        self
    }

    /// Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
    /// points in the given `direction` and [`Transform::up`] points towards `up`.
    #[inline]
    #[must_use]
    pub fn looking_to(mut self, direction: Vec3, up: Vec3) -> Self {
        self.look_to(direction, up);
        self
    }

    /// Returns this [`Transform`] with a new translation.
    #[inline]
    #[must_use]
    pub const fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    /// Returns this [`Transform`] with a new rotation.
    #[inline]
    #[must_use]
    pub const fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    /// Returns this [`Transform`] with a new scale.
    #[inline]
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    /// Computes the 3d affine transformation matrix from this transform's translation,
    /// rotation, and scale.
    #[inline]
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Returns the 3d affine transformation matrix from this transforms translation,
    /// rotation, and scale.
    #[inline]
    pub fn compute_affine(&self) -> Affine3A {
        Affine3A::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Compute 4x4 transformation matrix (legacy, for compatibility)
    #[inline]
    pub fn compute_matrix(&self) -> [f32; 16] {
        self.to_matrix().to_cols_array()
    }

    /// Get the unit vector in the local `X` direction.
    #[inline]
    pub fn local_x(&self) -> Dir3 {
        Dir3::new_unchecked(self.rotation * Vec3::X)
    }

    /// Equivalent to [`-local_x()`][Transform::local_x()]
    #[inline]
    pub fn left(&self) -> Dir3 {
        Dir3::new_unchecked(-(self.rotation * Vec3::X))
    }

    /// Equivalent to [`local_x()`][Transform::local_x()]
    #[inline]
    pub fn right(&self) -> Dir3 {
        self.local_x()
    }

    /// Get the unit vector in the local `Y` direction.
    #[inline]
    pub fn local_y(&self) -> Dir3 {
        Dir3::new_unchecked(self.rotation * Vec3::Y)
    }

    /// Equivalent to [`local_y()`][Transform::local_y]
    #[inline]
    pub fn up(&self) -> Dir3 {
        self.local_y()
    }

    /// Equivalent to [`-local_y()`][Transform::local_y]
    #[inline]
    pub fn down(&self) -> Dir3 {
        Dir3::new_unchecked(-(self.rotation * Vec3::Y))
    }

    /// Get the unit vector in the local `Z` direction.
    #[inline]
    pub fn local_z(&self) -> Dir3 {
        Dir3::new_unchecked(self.rotation * Vec3::Z)
    }

    /// Equivalent to [`-local_z()`][Transform::local_z]
    #[inline]
    pub fn forward(&self) -> Dir3 {
        Dir3::new_unchecked(-(self.rotation * Vec3::Z))
    }

    /// Equivalent to [`local_z()`][Transform::local_z]
    #[inline]
    pub fn back(&self) -> Dir3 {
        self.local_z()
    }

    /// Rotates this [`Transform`] by the given rotation.
    #[inline]
    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }

    /// Rotates this [`Transform`] around the given `axis` by `angle` (in radians).
    #[inline]
    pub fn rotate_axis(&mut self, axis: Dir3, angle: f32) {
        self.rotate(Quat::from_axis_angle(axis.into(), angle));
    }

    /// Rotates this [`Transform`] around the `X` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_x(&mut self, angle: f32) {
        self.rotate(Quat::from_rotation_x(angle));
    }

    /// Rotates this [`Transform`] around the `Y` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_y(&mut self, angle: f32) {
        self.rotate(Quat::from_rotation_y(angle));
    }

    /// Rotates this [`Transform`] around the `Z` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_z(&mut self, angle: f32) {
        self.rotate(Quat::from_rotation_z(angle));
    }

    /// Rotates this [`Transform`] by the given `rotation` relative to this [`Transform`]'s current rotation.
    #[inline]
    pub fn rotate_local(&mut self, rotation: Quat) {
        self.rotation = self.rotation * rotation;
    }

    /// Rotates this [`Transform`] around its local `axis` by `angle` (in radians).
    #[inline]
    pub fn rotate_local_axis(&mut self, axis: Dir3, angle: f32) {
        self.rotate_local(Quat::from_axis_angle(axis.into(), angle));
    }

    /// Rotates this [`Transform`] around its local `X` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_local_x(&mut self, angle: f32) {
        self.rotate_local(Quat::from_rotation_x(angle));
    }

    /// Rotates this [`Transform`] around its local `Y` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_local_y(&mut self, angle: f32) {
        self.rotate_local(Quat::from_rotation_y(angle));
    }

    /// Rotates this [`Transform`] around its local `Z` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_local_z(&mut self, angle: f32) {
        self.rotate_local(Quat::from_rotation_z(angle));
    }

    /// Translates this [`Transform`] around a `point` in space.
    #[inline]
    pub fn translate_around(&mut self, point: Vec3, rotation: Quat) {
        self.translation = point + rotation * (self.translation - point);
    }

    /// Rotates this [`Transform`] around a `point` in space.
    #[inline]
    pub fn rotate_around(&mut self, point: Vec3, rotation: Quat) {
        self.translate_around(point, rotation);
        self.rotate(rotation);
    }

    /// Rotates this [`Transform`] so that [`Transform::forward`] points towards the `target` position,
    /// and [`Transform::up`] points towards `up`.
    #[inline]
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        self.look_to(target - self.translation, up);
    }

    /// Rotates this [`Transform`] so that [`Transform::forward`] points in the given `direction`
    /// and [`Transform::up`] points towards `up`.
    #[inline]
    pub fn look_to(&mut self, direction: Vec3, up: Vec3) {
        let back = -direction.normalize_or_zero();
        if back == Vec3::ZERO {
            return;
        }
        let up = up.normalize_or_zero();
        let up = if up == Vec3::ZERO { Vec3::Y } else { up };
        
        let right = up.cross(back).normalize_or_zero();
        let right = if right == Vec3::ZERO {
            // up and back are parallel, use any orthogonal vector
            let any_ortho = if back.x.abs() < 0.9 { Vec3::X } else { Vec3::Y };
            any_ortho.cross(back).normalize()
        } else {
            right
        };
        let up = back.cross(right);
        self.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, back));
    }

    /// Multiplies `self` with `transform` component by component, returning the
    /// resulting [`Transform`]
    #[inline]
    #[must_use]
    pub fn mul_transform(&self, transform: Transform) -> Self {
        let translation = self.transform_point(transform.translation);
        let rotation = self.rotation * transform.rotation;
        let scale = self.scale * transform.scale;
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    /// Transforms the given `point`, applying scale, rotation and translation.
    #[inline]
    pub fn transform_point(&self, mut point: Vec3) -> Vec3 {
        point = self.scale * point;
        point = self.rotation * point;
        point += self.translation;
        point
    }

    /// Returns `true` if, and only if, translation, rotation and scale all are
    /// finite. If any of them contains a `NaN`, positive or negative infinity,
    /// this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.translation.is_finite() && self.rotation.is_finite() && self.scale.is_finite()
    }

    /// Get the [isometry] defined by this transform's rotation and translation, ignoring scale.
    #[inline]
    pub fn to_isometry(&self) -> Isometry3d {
        Isometry3d::from_translation_rotation(self.translation, self.rotation)
    }

    // ====== Legacy/Compatibility Methods ======

    /// Create identity transform (legacy)
    #[inline]
    pub fn identity() -> Self {
        Self::IDENTITY
    }

    /// Compute local-to-world matrix with parent matrix (legacy)
    pub fn compute_local_to_world(&self, parent_matrix: [f32; 16]) -> [f32; 16] {
        let parent = Mat4::from_cols_array(&parent_matrix);
        let local = self.to_matrix();
        (parent * local).to_cols_array()
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<GlobalTransform> for Transform {
    fn from(transform: GlobalTransform) -> Self {
        transform.compute_transform()
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, transform: Transform) -> Self::Output {
        self.mul_transform(transform)
    }
}

impl Mul<GlobalTransform> for Transform {
    type Output = GlobalTransform;

    #[inline]
    fn mul(self, global_transform: GlobalTransform) -> Self::Output {
        GlobalTransform::from(self) * global_transform
    }
}

impl Mul<Vec3> for Transform {
    type Output = Vec3;

    fn mul(self, value: Vec3) -> Self::Output {
        self.transform_point(value)
    }
}

/// GlobalTransform is an affine transformation from entity-local coordinates to worldspace coordinates.
/// 
/// You cannot directly mutate GlobalTransform; instead, you change an entity's transform by manipulating
/// its Transform, which indirectly causes the system to update its GlobalTransform.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalTransform(pub Affine3A);

impl GlobalTransform {
    /// An identity [`GlobalTransform`] that maps all points in space to themselves.
    pub const IDENTITY: Self = Self(Affine3A::IDENTITY);

    /// Creates a new [`GlobalTransform`] at the position `(x, y, z)`.
    #[inline]
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::from_translation(Vec3::new(x, y, z))
    }

    /// Creates a new [`GlobalTransform`] with translation.
    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        GlobalTransform(Affine3A::from_translation(translation))
    }

    /// Creates a new [`GlobalTransform`] with rotation.
    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        GlobalTransform(Affine3A::from_rotation_translation(rotation, Vec3::ZERO))
    }

    /// Creates a new [`GlobalTransform`] with scale.
    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        GlobalTransform(Affine3A::from_scale(scale))
    }

    /// Creates a new [`GlobalTransform`] that is equivalent to the given [isometry].
    #[inline]
    pub fn from_isometry(iso: Isometry3d) -> Self {
        Self::from_rotation(iso.rotation()).with_translation(iso.translation())
    }

    /// Returns this [`GlobalTransform`] with a new translation.
    #[inline]
    #[must_use]
    pub fn with_translation(self, translation: Vec3) -> Self {
        let mut affine = self.0;
        affine.translation = translation.into();
        GlobalTransform(affine)
    }

    /// Returns the 3d affine transformation matrix as a [`Mat4`].
    #[inline]
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from(self.0)
    }

    /// Returns the 3d affine transformation matrix as an [`Affine3A`].
    #[inline]
    pub fn affine(&self) -> Affine3A {
        self.0
    }

    /// Returns the transformation as a [`Transform`].
    #[inline]
    pub fn compute_transform(&self) -> Transform {
        let (scale, rotation, translation) = self.0.to_scale_rotation_translation();
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    /// Get the [isometry] defined by this transform's rotation and translation, ignoring scale.
    #[inline]
    pub fn to_isometry(&self) -> Isometry3d {
        let (_, rotation, translation) = self.0.to_scale_rotation_translation();
        Isometry3d::from_translation_rotation(translation, rotation)
    }

    /// Returns the [`Transform`] `self` would have if it was a child of an entity
    /// with the `parent` [`GlobalTransform`].
    #[inline]
    pub fn reparented_to(&self, parent: &GlobalTransform) -> Transform {
        let relative_affine = parent.affine().inverse() * self.affine();
        let (scale, rotation, translation) = relative_affine.to_scale_rotation_translation();
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`.
    #[inline]
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        self.0.to_scale_rotation_translation()
    }

    /// Get the unit vector in the local `X` direction.
    #[inline]
    pub fn right(&self) -> Dir3 {
        Dir3::new_unchecked((self.0.matrix3 * Vec3::X).normalize())
    }

    /// Equivalent to [`-right()`][GlobalTransform::right]
    #[inline]
    pub fn left(&self) -> Dir3 {
        Dir3::new_unchecked(-self.right().as_vec3())
    }

    /// Get the unit vector in the local `Y` direction.
    #[inline]
    pub fn up(&self) -> Dir3 {
        Dir3::new_unchecked((self.0.matrix3 * Vec3::Y).normalize())
    }

    /// Equivalent to [`-up()`][GlobalTransform::up]
    #[inline]
    pub fn down(&self) -> Dir3 {
        Dir3::new_unchecked(-self.up().as_vec3())
    }

    /// Get the unit vector in the local `Z` direction.
    #[inline]
    pub fn back(&self) -> Dir3 {
        Dir3::new_unchecked((self.0.matrix3 * Vec3::Z).normalize())
    }

    /// Equivalent to [`-back()`][GlobalTransform::back]
    #[inline]
    pub fn forward(&self) -> Dir3 {
        Dir3::new_unchecked(-self.back().as_vec3())
    }

    /// Get the translation as a [`Vec3`].
    #[inline]
    pub fn translation(&self) -> Vec3 {
        self.0.translation.into()
    }

    /// Get the rotation as a [`Quat`].
    #[inline]
    pub fn rotation(&self) -> Quat {
        self.to_scale_rotation_translation().1
    }

    /// Get the scale as a [`Vec3`].
    #[inline]
    pub fn scale(&self) -> Vec3 {
        let det = self.0.matrix3.determinant();
        Vec3::new(
            self.0.matrix3.x_axis().length() * det.signum(),
            self.0.matrix3.y_axis().length(),
            self.0.matrix3.z_axis().length(),
        )
    }

    /// Transforms the given point from local space to global space.
    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.0.transform_point3(point)
    }

    /// Multiplies `self` with `transform` component by component.
    #[inline]
    pub fn mul_transform(&self, transform: Transform) -> Self {
        Self(self.0 * transform.compute_affine())
    }

    // ====== Legacy Methods for Compatibility ======

    /// Create identity GlobalTransform (legacy)
    #[inline]
    pub fn identity() -> Self {
        Self::IDENTITY
    }

    /// Create from matrix (legacy)
    pub fn from_matrix(matrix: [f32; 16]) -> Self {
        let mat4 = Mat4::from_cols_array(&matrix);
        GlobalTransform(Affine3A::from_mat4(mat4))
    }

    /// Create from Transform (legacy)
    pub fn from_transform(transform: &Transform) -> Self {
        GlobalTransform::from(*transform)
    }

    /// Get the transformation matrix (legacy)
    pub fn matrix(&self) -> [f32; 16] {
        self.to_matrix().to_cols_array()
    }
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Transform> for GlobalTransform {
    fn from(transform: Transform) -> Self {
        Self(transform.compute_affine())
    }
}

impl From<Mat4> for GlobalTransform {
    fn from(world_from_local: Mat4) -> Self {
        Self(Affine3A::from_mat4(world_from_local))
    }
}

impl From<Affine3A> for GlobalTransform {
    fn from(affine: Affine3A) -> Self {
        Self(affine)
    }
}

impl Mul<GlobalTransform> for GlobalTransform {
    type Output = GlobalTransform;

    #[inline]
    fn mul(self, global_transform: GlobalTransform) -> Self::Output {
        GlobalTransform(self.0 * global_transform.0)
    }
}

impl Mul<Transform> for GlobalTransform {
    type Output = GlobalTransform;

    #[inline]
    fn mul(self, transform: Transform) -> Self::Output {
        self.mul_transform(transform)
    }
}

impl Mul<Vec3> for GlobalTransform {
    type Output = Vec3;

    #[inline]
    fn mul(self, value: Vec3) -> Self::Output {
        self.transform_point(value)
    }
}

/// An optimization marker component for transform propagation.
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
    pub matrix: [f32; 16],
}

/// Configure the behavior of static scene optimizations for Transform propagation.
#[derive(Debug, Clone)]
pub struct StaticTransformOptimizations {
    pub threshold: f32,
    pub enabled: bool,
}

impl StaticTransformOptimizations {
    pub fn from_threshold(threshold: f32) -> Self {
        Self { threshold, enabled: true }
    }

    pub fn disabled() -> Self {
        Self { threshold: 0.0, enabled: false }
    }

    pub fn enabled() -> Self {
        Self { threshold: 1.0, enabled: true }
    }
}

impl Default for StaticTransformOptimizations {
    fn default() -> Self {
        Self { threshold: 0.3, enabled: true }
    }
}

/// System labels for transform propagation scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransformSystems {
    MarkDirtyTrees,
    SyncSimpleTransforms,
    Propagate,
}

/// Error types for global transform computation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeGlobalTransformError {
    MissingTransform,
    MissingGlobalTransform,
    InvalidParent,
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
    pub fn new() -> Self {
        Self
    }
}

impl Default for TransformPlugin {
    fn default() -> Self {
        Self::new()
    }
}

use autozig_app::{App, Plugin, MainScheduleOrder};

impl Plugin for TransformPlugin {
    fn name(&self) -> &str {
        "TransformPlugin"
    }
    
    fn build(&self, app: &mut App) {
        app.world.register_component::<Transform>();
        app.world.register_component::<GlobalTransform>();
        
        app.add_systems::<autozig_ecs::into_system::ExclusiveSystemMarker>(
            autozig_ecs::schedule::PostUpdate, 
            propagate_transforms_system
        );
    }
}

/// Global world pointer for transform propagation (legacy)
pub static mut WORLD_PTR: *mut u8 = std::ptr::null_mut();

/// Rust system wrapper for transform propagation
pub fn propagate_transforms_system(world: &mut autozig_ecs::world::World) {
    world.update_archetypes();

    let query = world.query::<(&Transform, &mut GlobalTransform)>();
    for (transform, mut global) in query.iter::<(&Transform, &mut GlobalTransform), ()>(world) {
        *global = GlobalTransform::from(*transform);
    }
}

/// System to propagate transforms (legacy extern C)
#[no_mangle]
pub extern "C" fn propagate_transforms() {
    unsafe {
        if WORLD_PTR.is_null() { return; }
        
        let mut world = autozig_ecs::world::World::from_raw(WORLD_PTR as *mut autozig_ecs::world::WorldOpaque);
        propagate_transforms_system(&mut world);
        std::mem::forget(world);
    }
}

/// Helper for transform operations
pub struct TransformHelper;

impl TransformHelper {
    pub fn compute_global_transform(
        local: &Transform,
        parent_global: &GlobalTransform,
    ) -> GlobalTransform {
        parent_global.mul_transform(*local)
    }

    pub fn compute_root_global_transform(local: &Transform) -> GlobalTransform {
        GlobalTransform::from(*local)
    }

    pub fn get_translation(global: &GlobalTransform) -> [f32; 3] {
        global.translation().to_array()
    }

    pub fn get_scale(global: &GlobalTransform) -> [f32; 3] {
        global.scale().to_array()
    }
}

/// Work queue for parallel transform computation
pub struct WorkQueue {
    pub entities: Vec<u32>,
    pub index: usize,
}

impl WorkQueue {
    pub fn new() -> Self {
        Self { entities: Vec::new(), index: 0 }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { entities: Vec::with_capacity(capacity), index: 0 }
    }

    pub fn push(&mut self, entity: u32) {
        self.entities.push(entity);
    }

    pub fn pop(&mut self) -> Option<u32> {
        if self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;
            Some(entity)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.entities.len()
    }

    pub fn len(&self) -> usize {
        self.entities.len() - self.index
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.index = 0;
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl Default for WorkQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for transforming points in space (legacy)
pub trait TransformPoint {
    fn transform_point(&self, point: [f32; 3]) -> [f32; 3];
    fn transform_vector(&self, vector: [f32; 3]) -> [f32; 3];
}

impl TransformPoint for Transform {
    fn transform_point(&self, point: [f32; 3]) -> [f32; 3] {
        let p = Vec3::from_array(point);
        let result = Transform::transform_point(self, p);
        result.to_array()
    }

    fn transform_vector(&self, vector: [f32; 3]) -> [f32; 3] {
        let v = Vec3::from_array(vector);
        let result = self.rotation * (self.scale * v);
        result.to_array()
    }
}

impl TransformPoint for GlobalTransform {
    fn transform_point(&self, point: [f32; 3]) -> [f32; 3] {
        let p = Vec3::from_array(point);
        let result = GlobalTransform::transform_point(self, p);
        result.to_array()
    }

    fn transform_vector(&self, vector: [f32; 3]) -> [f32; 3] {
        let v = Vec3::from_array(vector);
        let result = self.0.transform_vector3(v);
        result.to_array()
    }
}

/// Extension trait for building children with transforms
pub trait BuildChildrenTransformExt {
    fn with_child_transform(&mut self, transform: Transform) -> u32;
    fn with_children_transforms(&mut self, transforms: &[Transform]) -> Vec<u32>;
}

// Include Zig implementations (legacy - keeping for backward compatibility)
include_zig!("src/zig/transform.zig", {
    fn transform_identity() -> LegacyTransform;
    fn transform_from_translation(translation: [f32; 3]) -> LegacyTransform;
    fn transform_from_rotation(rotation: [f32; 4]) -> LegacyTransform;
    fn transform_from_scale(scale: [f32; 3]) -> LegacyTransform;
    fn transform_compute_matrix(transform: *const LegacyTransform, out_matrix: *mut [f32; 16]);
    fn transform_compute_local_to_world(
        transform: *const LegacyTransform,
        parent_matrix: [f32; 16],
        out_matrix: *mut [f32; 16]
    );
});

/// Legacy Transform struct for Zig FFI compatibility
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegacyTransform {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl From<Transform> for LegacyTransform {
    fn from(t: Transform) -> Self {
        LegacyTransform {
            translation: t.translation.to_array(),
            rotation: [t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w],
            scale: t.scale.to_array(),
        }
    }
}

impl From<LegacyTransform> for Transform {
    fn from(t: LegacyTransform) -> Self {
        Transform {
            translation: Vec3::from_array(t.translation),
            rotation: Quat::from_xyzw(t.rotation[0], t.rotation[1], t.rotation[2], t.rotation[3]),
            scale: Vec3::from_array(t.scale),
        }
    }
}

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
        transform: *mut LegacyTransform,
        local_to_world: *mut LocalToWorld
    );
    fn transform_system_update_local_to_world(
        transform: *mut LegacyTransform,
        parent_local_to_world: *mut LocalToWorld,
        local_to_world: *mut LocalToWorld
    );
    fn transform_system_update_root_local_to_world(
        transform: *mut LegacyTransform,
        local_to_world: *mut LocalToWorld
    );
    fn transform_system_propagate_transforms(
        transforms: *mut LegacyTransform,
        hierarchies: *mut Hierarchy,
        local_to_worlds: *mut LocalToWorld,
        entity_id: u32,
        parent_ltw: *const LocalToWorld
    );
});

include_zig!("src/zig/global_transform.zig", {
    fn global_transform_identity() -> LegacyGlobalTransform;
    fn global_transform_from_matrix(matrix: *const [f32; 16]) -> LegacyGlobalTransform;
    fn global_transform_from_transform(transform: *const LegacyTransform) -> LegacyGlobalTransform;
    fn global_transform_mul_transform(
        global: *const LegacyGlobalTransform,
        transform: *const LegacyTransform,
        out: *mut LegacyGlobalTransform
    );
    fn global_transform_transform_point(
        global: *const LegacyGlobalTransform,
        point: *const [f32; 3],
        out: *mut [f32; 3]
    );
    fn global_transform_transform_vector(
        global: *const LegacyGlobalTransform,
        vector: *const [f32; 3],
        out: *mut [f32; 3]
    );
});

/// Legacy GlobalTransform struct for Zig FFI compatibility
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegacyGlobalTransform {
    pub matrix: [f32; 16],
}

impl From<GlobalTransform> for LegacyGlobalTransform {
    fn from(gt: GlobalTransform) -> Self {
        LegacyGlobalTransform {
            matrix: gt.matrix(),
        }
    }
}

impl From<LegacyGlobalTransform> for GlobalTransform {
    fn from(gt: LegacyGlobalTransform) -> Self {
        GlobalTransform::from_matrix(gt.matrix)
    }
}

// Hierarchy implementation 
impl Hierarchy {
    pub fn new() -> Self {
        hierarchy_create()
    }

    pub fn add_child(&mut self, child_id: u32) -> bool {
        hierarchy_add_child(self, child_id)
    }

    pub fn remove_child(&mut self, child_id: u32) -> bool {
        hierarchy_remove_child(self, child_id)
    }

    pub fn children_count(&self) -> u32 {
        hierarchy_get_children_count(self)
    }

    pub fn get_child(&self, index: u32) -> u32 {
        hierarchy_get_child(self, index)
    }

    pub fn has_parent(&self) -> bool {
        hierarchy_has_parent(self)
    }

    pub fn has_children(&self) -> bool {
        hierarchy_has_children(self)
    }

    pub fn clear_children(&mut self) {
        hierarchy_clear_children(self)
    }

    pub fn set_parent(&mut self, parent_id: u32) {
        hierarchy_set_parent(self, parent_id)
    }

    pub fn clear_parent(&mut self) {
        hierarchy_clear_parent(self)
    }

    pub fn parent(&self) -> u32 {
        hierarchy_get_parent(self)
    }

    pub fn children(&self) -> Vec<u32> {
        let count = self.children_count() as usize;
        (0..count).map(|i| self.get_child(i as u32)).collect()
    }
}

impl Default for Hierarchy {
    fn default() -> Self {
        Self::new()
    }
}

// LocalToWorld implementation
impl LocalToWorld {
    pub fn identity() -> Self {
        local_to_world_identity()
    }

    pub fn from_matrix(matrix: [f32; 16]) -> Self {
        local_to_world_from_matrix(matrix)
    }

    pub fn matrix(&self) -> [f32; 16] {
        let mut result = [0.0f32; 16];
        local_to_world_get_matrix(self, &mut result);
        result
    }

    pub fn set_matrix(&mut self, matrix: [f32; 16]) {
        local_to_world_set_matrix(self, matrix)
    }

    pub fn translation(&self) -> [f32; 3] {
        let mut result = [0.0f32; 3];
        local_to_world_get_translation(self, &mut result);
        result
    }

    pub fn scale(&self) -> [f32; 3] {
        let mut result = [0.0f32; 3];
        local_to_world_get_scale(self, &mut result);
        result
    }

    pub fn multiply(&mut self, other: [f32; 16]) {
        local_to_world_multiply(self, other)
    }

    pub fn copy_from(&mut self, src: &LocalToWorld) {
        local_to_world_copy_from(self, src)
    }

    pub fn is_identity(&self) -> bool {
        local_to_world_is_identity(self)
    }
}

impl Default for LocalToWorld {
    fn default() -> Self {
        Self::identity()
    }
}

/// System wrapper for transform operations
pub struct TransformSystem;

impl TransformSystem {
    pub fn update_hierarchy(
        hierarchy: &mut Hierarchy,
        transform: &mut Transform,
        local_to_world: &mut LocalToWorld,
    ) {
        let mut legacy = LegacyTransform::from(*transform);
        transform_system_update_hierarchy(hierarchy, &mut legacy, local_to_world);
        *transform = legacy.into();
    }

    pub fn update_local_to_world(
        transform: &mut Transform,
        parent_local_to_world: &mut LocalToWorld,
        local_to_world: &mut LocalToWorld,
    ) {
        let mut legacy = LegacyTransform::from(*transform);
        transform_system_update_local_to_world(&mut legacy, parent_local_to_world, local_to_world);
        *transform = legacy.into();
    }

    pub fn update_root_local_to_world(
        transform: &mut Transform,
        local_to_world: &mut LocalToWorld,
    ) {
        let mut legacy = LegacyTransform::from(*transform);
        transform_system_update_root_local_to_world(&mut legacy, local_to_world);
        *transform = legacy.into();
    }
}

// Implement Component for Transform types
impl autozig_ecs::component::Component for Transform {}
impl autozig_ecs::component::Component for GlobalTransform {}
impl autozig_ecs::component::Component for Hierarchy {}
impl autozig_ecs::component::Component for LocalToWorld {}