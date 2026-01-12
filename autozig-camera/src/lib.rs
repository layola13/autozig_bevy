//! AutoZig Camera - Bevy camera system for WebGPU/WASM platforms
//! 
//! This crate provides Camera2d and Camera3d components with high-performance
//! Zig implementations for projection, view matrices, and frustum culling.

use autozig::include_zig;

// ============================================================================
// Core Camera Types
// ============================================================================

/// Perspective projection configuration for Camera3d
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerspectiveProjection {
    pub fov: f32,           // Field of view in radians
    pub aspect_ratio: f32,  // Width / height
    pub near: f32,          // Near clipping plane
    pub far: f32,           // Far clipping plane
}

/// Orthographic projection configuration for Camera2d
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrthographicProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub scale: f32,  // Scaling factor for 2D cameras
}

/// 3D Camera with perspective projection
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera3d {
    pub projection: PerspectiveProjection,
    pub view_matrix: [f32; 16],
    pub projection_matrix: [f32; 16],
    pub view_projection_matrix: [f32; 16],
}

/// 2D Camera with orthographic projection
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera2d {
    pub projection: OrthographicProjection,
    pub view_matrix: [f32; 16],
    pub projection_matrix: [f32; 16],
    pub view_projection_matrix: [f32; 16],
}

/// Frustum for culling
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

/// Plane representation for frustum
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: [f32; 3],
    pub distance: f32,
}

// ============================================================================
// NEW API TYPES - STRUCTS (38个)
// ============================================================================

/// Axis-Aligned Bounding Box
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub center: [f32; 3],
    pub half_extents: [f32; 3],
}

/// Camera 3D depth texture usage configuration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Camera3dDepthTextureUsage {
    pub enabled: bool,
}

/// Camera main texture usages
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CameraMainTextureUsages {
    pub usage_flags: u32,
}

/// Camera plugin configuration
#[derive(Debug, Clone)]
pub struct CameraPlugin;

/// Camera projection plugin
#[derive(Debug, Clone)]
pub struct CameraProjectionPlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

/// Camera update system labels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CameraUpdateSystems;

/// Cascaded shadow frustums
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CascadesFrusta {
    pub frusta: Vec<Frustum>,
}

/// Cascaded shadow visible entities
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CascadesVisibleEntities {
    pub entities: Vec<Vec<u64>>,
}

/// Clear color for camera background
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// Computed camera values (cached calculations)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ComputedCameraValues {
    pub projection_matrix: [f32; 16],
    pub target_info: Option<RenderTargetInfo>,
}

/// Cube map face identifier
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubeMapFace {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
}

/// Cube map frustums for each face
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CubemapFrusta {
    pub frusta: [Frustum; 6],
}

/// Cube map visible entities per face
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CubemapVisibleEntities {
    pub entities: [Vec<u64>; 6],
}

/// Custom projection matrix
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomProjection {
    pub matrix: [f32; 16],
}

/// Camera exposure settings
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Exposure {
    pub ev100: f32,
}

/// Half-space (plane) for frustum culling
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HalfSpace {
    pub normal: [f32; 3],
    pub distance: f32,
}

/// Image render target configuration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageRenderTarget {
    pub image_handle: u64,
}

/// Inherited visibility from parent
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InheritedVisibility {
    pub visible: bool,
}

/// Main pass resolution override
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainPassResolutionOverride {
    pub width: u32,
    pub height: u32,
}

/// Manual texture view handle
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManualTextureViewHandle {
    pub handle: u64,
}

/// Marker: No automatic AABB calculation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoAutoAabb;

/// Marker: No CPU culling
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoCpuCulling;

/// Marker: No frustum culling
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoFrustumCulling;

/// Physical camera parameters
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalCameraParameters {
    pub aperture_f_stops: f32,
    pub shutter_speed_s: f32,
    pub sensitivity_iso: f32,
}

/// Render layers bit mask
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderLayers {
    pub bits: u32,
}

/// Render target information
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderTargetInfo {
    pub physical_size: [u32; 2],
    pub scale_factor: f32,
}

/// Bounding sphere
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
}

/// Sub-camera view (for split-screen, etc.)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SubCameraView {
    pub viewport: Viewport,
    pub projection_matrix: [f32; 16],
}

/// View visibility (per-camera visibility)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewVisibility {
    pub visible: bool,
}

/// Viewport configuration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub physical_position: [u32; 2],
    pub physical_size: [u32; 2],
    pub depth: [f32; 2],
}

/// Visibility classification
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityClass {
    Visible = 0,
    Hidden = 1,
    Inherited = 2,
}

/// Visibility plugin
#[derive(Debug, Clone)]
pub struct VisibilityPlugin;

/// Visibility range for LOD
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisibilityRange {
    pub start_margin: f32,
    pub end_margin: f32,
}

/// Visibility range plugin
#[derive(Debug, Clone)]
pub struct VisibilityRangePlugin;

/// List of visible entities
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisibleEntities {
    pub entities: Vec<u64>,
}

/// Visible entity ranges for LOD
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisibleEntityRanges {
    pub ranges: Vec<(u64, f32)>,
}

/// Visible mesh entities
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisibleMeshEntities {
    pub entities: Vec<u64>,
}

// ============================================================================
// NEW API TYPES - ENUMS (13个)
// ============================================================================

/// Camera 3D depth load operation
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Camera3dDepthLoadOp {
    Clear = 0,
    Load = 1,
}

/// Camera output mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraOutputMode {
    Write = 0,
    Skip = 1,
}

/// Clear color configuration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClearColorConfig {
    Default,
    Custom([f32; 4]),
    None,
}

/// Cubemap layout format
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubemapLayout {
    Vertical = 0,
    Horizontal = 1,
    Cross = 2,
}

/// MSAA writeback mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsaaWriteback {
    Enabled = 0,
    Disabled = 1,
}

/// Normalized render target
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NormalizedRenderTarget {
    Window,
    Image(u64),
    TextureView(u64),
}

/// Projection type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
}

/// Render target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderTarget {
    Window,
    Image(u64),
    TextureView(u64),
}

/// Scaling mode for orthographic projections
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScalingMode {
    None = 0,
    WindowSize = 1,
    AutoMin = 2,
    AutoMax = 3,
    FixedVertical(f32) = 4,
    FixedHorizontal(f32) = 5,
}

/// Screen space transmission quality
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenSpaceTransmissionQuality {
    Low = 0,
    Medium = 1,
    High = 2,
    Ultra = 3,
}

/// Viewport conversion error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportConversionError {
    OutOfBounds,
    InvalidSize,
}

/// Entity visibility state
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    Inherited = 0,
    Hidden = 1,
    Visible = 2,
}

/// Visibility system labels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisibilitySystems {
    CalculateBounds,
    UpdateFrusta,
    CheckVisibility,
    VisibilityPropagate,
}

// ============================================================================
// NEW API TYPES - TRAITS (4个)
// ============================================================================

/// Camera projection trait
pub trait CameraProjection {
    fn get_projection_matrix(&self) -> [f32; 16];
    fn update(&mut self, width: f32, height: f32);
    fn far(&self) -> f32;
    fn near(&self) -> f32;
}

/// Dynamic camera projection trait (trait object compatible)
pub trait DynCameraProjection: CameraProjection + Send + Sync {}

/// Mesh AABB trait
pub trait MeshAabb {
    fn compute_aabb(&self) -> Option<Aabb>;
}

/// Set view visibility trait
pub trait SetViewVisibility {
    fn set_visibility(&mut self, visible: bool);
}

// ============================================================================
// Projection Functions
// ============================================================================

include_zig!("src/zig/projection.zig", {
    fn projection_perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32, out: *mut [f32; 16]);
    fn projection_perspective_lh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32, out: *mut [f32; 16]);
    fn projection_perspective_infinite_reverse_z(fov_y: f32, aspect: f32, z_near: f32, out: *mut [f32; 16]);
    fn projection_orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32, out: *mut [f32; 16]);
    fn projection_orthographic_lh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32, out: *mut [f32; 16]);
    fn projection_orthographic_2d(window_width: f32, window_height: f32, out: *mut [f32; 16]);
    fn projection_orthographic_scaled(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32, scale: f32, out: *mut [f32; 16]);
    fn projection_from_fov(fov_degrees: f32, aspect: f32, z_near: f32, z_far: f32, out: *mut [f32; 16]);
    fn projection_extract_fov(matrix: *const [f32; 16]) -> f32;
    fn projection_extract_aspect(matrix: *const [f32; 16]) -> f32;
    fn projection_extract_near(matrix: *const [f32; 16]) -> f32;
    fn projection_extract_far(matrix: *const [f32; 16]) -> f32;
    fn projection_is_perspective(matrix: *const [f32; 16]) -> bool;
    fn projection_is_orthographic(matrix: *const [f32; 16]) -> bool;
});

// ============================================================================
// View Matrix Functions
// ============================================================================

include_zig!("src/zig/view.zig", {
    fn view_matrix_from_transform(position: *const [f32; 3], rotation: *const [f32; 4], out: *mut [f32; 16]);
    fn view_look_at_rh(eye: *const [f32; 3], target: *const [f32; 3], up: *const [f32; 3], out: *mut [f32; 16]);
    fn view_look_at_lh(eye: *const [f32; 3], target: *const [f32; 3], up: *const [f32; 3], out: *mut [f32; 16]);
    fn view_extract_position(view_matrix: *const [f32; 16], out_position: *mut [f32; 3]);
    fn view_extract_forward(view_matrix: *const [f32; 16], out_forward: *mut [f32; 3]);
    fn view_extract_right(view_matrix: *const [f32; 16], out_right: *mut [f32; 3]);
    fn view_extract_up(view_matrix: *const [f32; 16], out_up: *mut [f32; 3]);
    fn view_matrix_2d(position: *const [f32; 2], scale: f32, out: *mut [f32; 16]);
    fn view_matrix_2d_rotated(position: *const [f32; 2], rotation: f32, scale: f32, out: *mut [f32; 16]);
    fn view_matrix_inverse(view_matrix: *const [f32; 16], out: *mut [f32; 16]);
});

// ============================================================================
// Frustum Culling Functions
// ============================================================================

include_zig!("src/zig/frustum.zig", {
    fn frustum_from_matrix(view_proj_matrix: *const [f32; 16], out: *mut Frustum);
    fn frustum_test_point(frustum: *const Frustum, point: *const [f32; 3]) -> bool;
    fn frustum_test_aabb(frustum: *const Frustum, min: *const [f32; 3], max: *const [f32; 3]) -> bool;
    fn frustum_test_sphere(frustum: *const Frustum, center: *const [f32; 3], radius: f32) -> bool;
    fn frustum_test_obb(frustum: *const Frustum, center: *const [f32; 3], extents: *const [f32; 3], rotation: *const [f32; 4]) -> bool;
    fn frustum_get_corners(frustum: *const Frustum, out_vertices: *mut [f32; 24]);
    fn frustum_test_sphere_conservative(frustum: *const Frustum, center: *const [f32; 3], radius: f32, margin: f32) -> bool;
    fn frustum_test_aabb_inside(frustum: *const Frustum, min: *const [f32; 3], max: *const [f32; 3]) -> bool;
    fn frustum_distance_to_aabb(frustum: *const Frustum, min: *const [f32; 3], max: *const [f32; 3]) -> f32;
});

// ============================================================================
// Trait Implementations
// ============================================================================

impl CameraProjection for PerspectiveProjection {
    fn get_projection_matrix(&self) -> [f32; 16] {
        self.compute_matrix()
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    fn far(&self) -> f32 {
        self.far
    }

    fn near(&self) -> f32 {
        self.near
    }
}

impl CameraProjection for OrthographicProjection {
    fn get_projection_matrix(&self) -> [f32; 16] {
        self.compute_matrix()
    }

    fn update(&mut self, width: f32, height: f32) {
        self.update_size(width, height);
    }

    fn far(&self) -> f32 {
        self.far
    }

    fn near(&self) -> f32 {
        self.near
    }
}

impl DynCameraProjection for PerspectiveProjection {}
impl DynCameraProjection for OrthographicProjection {}

impl SetViewVisibility for ViewVisibility {
    fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }
}

// ============================================================================
// PerspectiveProjection Implementation
// ============================================================================

impl PerspectiveProjection {
    /// Create new perspective projection with default values
    pub fn new(fov: f32, aspect_ratio: f32) -> Self {
        Self {
            fov,
            aspect_ratio,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Create perspective projection from FOV in degrees
    pub fn from_fov_degrees(fov_degrees: f32, aspect_ratio: f32) -> Self {
        Self::new(fov_degrees.to_radians(), aspect_ratio)
    }

    /// Compute projection matrix (right-handed)
    pub fn compute_matrix(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        projection_perspective_rh(self.fov, self.aspect_ratio, self.near, self.far, &mut matrix);
        matrix
    }

    /// Compute projection matrix (left-handed)
    pub fn compute_matrix_lh(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        projection_perspective_lh(self.fov, self.aspect_ratio, self.near, self.far, &mut matrix);
        matrix
    }

    /// Compute infinite projection with reverse-Z
    pub fn compute_infinite_reverse_z(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        projection_perspective_infinite_reverse_z(self.fov, self.aspect_ratio, self.near, &mut matrix);
        matrix
    }

    /// Update aspect ratio (e.g., on window resize)
    pub fn update_aspect(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    /// Get FOV in degrees
    pub fn fov_degrees(&self) -> f32 {
        self.fov.to_degrees()
    }

    /// Set FOV in degrees
    pub fn set_fov_degrees(&mut self, fov_degrees: f32) {
        self.fov = fov_degrees.to_radians();
    }
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self::new(std::f32::consts::FRAC_PI_4, 16.0 / 9.0)
    }
}

// ============================================================================
// OrthographicProjection Implementation
// ============================================================================

impl OrthographicProjection {
    /// Create new orthographic projection
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near: -1.0,
            far: 1.0,
            scale: 1.0,
        }
    }

    /// Create 2D orthographic projection from window size
    pub fn from_window_size(width: f32, height: f32) -> Self {
        let half_width = width * 0.5;
        let half_height = height * 0.5;
        Self::new(-half_width, half_width, -half_height, half_height)
    }

    /// Compute projection matrix (right-handed)
    pub fn compute_matrix(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        if self.scale != 1.0 {
            projection_orthographic_scaled(
                self.left, self.right, self.bottom, self.top,
                self.near, self.far, self.scale, &mut matrix
            );
        } else {
            projection_orthographic_rh(
                self.left, self.right, self.bottom, self.top,
                self.near, self.far, &mut matrix
            );
        }
        matrix
    }

    /// Compute projection matrix (left-handed)
    pub fn compute_matrix_lh(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        projection_orthographic_lh(
            self.left, self.right, self.bottom, self.top,
            self.near, self.far, &mut matrix
        );
        matrix
    }

    /// Update size (e.g., on window resize)
    pub fn update_size(&mut self, width: f32, height: f32) {
        let half_width = width * 0.5;
        let half_height = height * 0.5;
        self.left = -half_width;
        self.right = half_width;
        self.bottom = -half_height;
        self.top = half_height;
    }

    /// Get width
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Get height
    pub fn height(&self) -> f32 {
        self.top - self.bottom
    }
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        Self::from_window_size(1920.0, 1080.0)
    }
}

// ============================================================================
// Camera3d Implementation
// ============================================================================

impl Camera3d {
    /// Create new 3D camera
    pub fn new(fov: f32, aspect_ratio: f32) -> Self {
        let projection = PerspectiveProjection::new(fov, aspect_ratio);
        let mut camera = Self {
            projection,
            view_matrix: [0.0; 16],
            projection_matrix: [0.0; 16],
            view_projection_matrix: [0.0; 16],
        };
        camera.update_matrices(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0, 1.0]);
        camera
    }

    /// Update all matrices from position and rotation
    pub fn update_matrices(&mut self, position: &[f32; 3], rotation: &[f32; 4]) {
        // Compute view matrix
        view_matrix_from_transform(position, rotation, &mut self.view_matrix);
        
        // Compute projection matrix
        self.projection_matrix = self.projection.compute_matrix();
        
        // Compute view-projection matrix (projection * view)
        self.view_projection_matrix = multiply_matrices(&self.projection_matrix, &self.view_matrix);
    }

    /// Update from look-at parameters
    pub fn look_at(&mut self, eye: [f32; 3], target: [f32; 3], up: [f32; 3]) {
        view_look_at_rh(&eye, &target, &up, &mut self.view_matrix);
        self.projection_matrix = self.projection.compute_matrix();
        self.view_projection_matrix = multiply_matrices(&self.projection_matrix, &self.view_matrix);
    }

    /// Get camera position
    pub fn position(&self) -> [f32; 3] {
        let mut pos = [0.0f32; 3];
        view_extract_position(&self.view_matrix, &mut pos);
        pos
    }

    /// Get camera forward direction
    pub fn forward(&self) -> [f32; 3] {
        let mut fwd = [0.0f32; 3];
        view_extract_forward(&self.view_matrix, &mut fwd);
        fwd
    }

    /// Get camera right direction
    pub fn right(&self) -> [f32; 3] {
        let mut right = [0.0f32; 3];
        view_extract_right(&self.view_matrix, &mut right);
        right
    }

    /// Get camera up direction
    pub fn up(&self) -> [f32; 3] {
        let mut up = [0.0f32; 3];
        view_extract_up(&self.view_matrix, &mut up);
        up
    }

    /// Create frustum from current matrices
    pub fn frustum(&self) -> Frustum {
        let mut frustum = Frustum {
            planes: [Plane { normal: [0.0; 3], distance: 0.0 }; 6],
        };
        frustum_from_matrix(&self.view_projection_matrix, &mut frustum);
        frustum
    }
}

impl Default for Camera3d {
    fn default() -> Self {
        Self::new(std::f32::consts::FRAC_PI_4, 16.0 / 9.0)
    }
}

// ============================================================================
// Camera2d Implementation
// ============================================================================

impl Camera2d {
    /// Create new 2D camera
    pub fn new(width: f32, height: f32) -> Self {
        let projection = OrthographicProjection::from_window_size(width, height);
        let mut camera = Self {
            projection,
            view_matrix: [0.0; 16],
            projection_matrix: [0.0; 16],
            view_projection_matrix: [0.0; 16],
        };
        camera.update_matrices(&[0.0, 0.0], 0.0);
        camera
    }

    /// Update all matrices from 2D position and rotation
    pub fn update_matrices(&mut self, position: &[f32; 2], rotation: f32) {
        // Compute view matrix
        view_matrix_2d_rotated(position, rotation, self.projection.scale, &mut self.view_matrix);
        
        // Compute projection matrix
        self.projection_matrix = self.projection.compute_matrix();
        
        // Compute view-projection matrix
        self.view_projection_matrix = multiply_matrices(&self.projection_matrix, &self.view_matrix);
    }

    /// Update from position only (no rotation)
    pub fn update_position(&mut self, position: &[f32; 2]) {
        view_matrix_2d(position, self.projection.scale, &mut self.view_matrix);
        self.projection_matrix = self.projection.compute_matrix();
        self.view_projection_matrix = multiply_matrices(&self.projection_matrix, &self.view_matrix);
    }

    /// Set scale (zoom level)
    pub fn set_scale(&mut self, scale: f32) {
        self.projection.scale = scale;
    }

    /// Get scale
    pub fn scale(&self) -> f32 {
        self.projection.scale
    }
}

impl Default for Camera2d {
    fn default() -> Self {
        Self::new(1920.0, 1080.0)
    }
}

// ============================================================================
// Frustum Implementation
// ============================================================================

impl Frustum {
    /// Test if point is in frustum
    pub fn test_point(&self, point: [f32; 3]) -> bool {
        frustum_test_point(self, &point)
    }

    /// Test if AABB intersects frustum
    pub fn test_aabb(&self, min: [f32; 3], max: [f32; 3]) -> bool {
        frustum_test_aabb(self, &min, &max)
    }

    /// Test if sphere intersects frustum
    pub fn test_sphere(&self, center: [f32; 3], radius: f32) -> bool {
        frustum_test_sphere(self, &center, radius)
    }

    /// Test if OBB intersects frustum
    pub fn test_obb(&self, center: [f32; 3], extents: [f32; 3], rotation: [f32; 4]) -> bool {
        frustum_test_obb(self, &center, &extents, &rotation)
    }

    /// Get frustum corner vertices
    pub fn corners(&self) -> [f32; 24] {
        let mut vertices = [0.0f32; 24];
        frustum_get_corners(self, &mut vertices);
        vertices
    }

    /// Test if AABB is completely inside frustum
    pub fn test_aabb_inside(&self, min: [f32; 3], max: [f32; 3]) -> bool {
        frustum_test_aabb_inside(self, &min, &max)
    }

    /// Calculate distance to AABB
    pub fn distance_to_aabb(&self, min: [f32; 3], max: [f32; 3]) -> f32 {
        frustum_distance_to_aabb(self, &min, &max)
    }
}

// ============================================================================
// Aabb Implementation
// ============================================================================

impl Aabb {
    pub fn new(center: [f32; 3], half_extents: [f32; 3]) -> Self {
        Self { center, half_extents }
    }

    pub fn from_min_max(min: [f32; 3], max: [f32; 3]) -> Self {
        let center = [
            (min[0] + max[0]) * 0.5,
            (min[1] + max[1]) * 0.5,
            (min[2] + max[2]) * 0.5,
        ];
        let half_extents = [
            (max[0] - min[0]) * 0.5,
            (max[1] - min[1]) * 0.5,
            (max[2] - min[2]) * 0.5,
        ];
        Self { center, half_extents }
    }

    pub fn min(&self) -> [f32; 3] {
        [
            self.center[0] - self.half_extents[0],
            self.center[1] - self.half_extents[1],
            self.center[2] - self.half_extents[2],
        ]
    }

    pub fn max(&self) -> [f32; 3] {
        [
            self.center[0] + self.half_extents[0],
            self.center[1] + self.half_extents[1],
            self.center[2] + self.half_extents[2],
        ]
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            center: [0.0; 3],
            half_extents: [0.5; 3],
        }
    }
}

// ============================================================================
// Sphere Implementation
// ============================================================================

impl Sphere {
    pub fn new(center: [f32; 3], radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: [0.0; 3],
            radius: 0.5,
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Multiply two 4x4 matrices (column-major order)
fn multiply_matrices(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut result = [0.0f32; 16];
    for col in 0..4 {
        for row in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + row] * b[col * 4 + k];
            }
            result[col * 4 + row] = sum;
        }
    }
    result
}

/// Projection utilities
pub struct ProjectionUtils;

impl ProjectionUtils {
    /// Extract FOV from projection matrix
    pub fn extract_fov(matrix: &[f32; 16]) -> f32 {
        projection_extract_fov(matrix)
    }

    /// Extract aspect ratio from projection matrix
    pub fn extract_aspect(matrix: &[f32; 16]) -> f32 {
        projection_extract_aspect(matrix)
    }

    /// Extract near plane from projection matrix
    pub fn extract_near(matrix: &[f32; 16]) -> f32 {
        projection_extract_near(matrix)
    }

    /// Extract far plane from projection matrix
    pub fn extract_far(matrix: &[f32; 16]) -> f32 {
        projection_extract_far(matrix)
    }

    /// Check if matrix is perspective projection
    pub fn is_perspective(matrix: &[f32; 16]) -> bool {
        projection_is_perspective(matrix)
    }

    /// Check if matrix is orthographic projection
    pub fn is_orthographic(matrix: &[f32; 16]) -> bool {
        projection_is_orthographic(matrix)
    }
}