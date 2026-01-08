//! Camera System
//! Handles perspective and orthographic cameras with projection matrices

const std = @import("std");

/// Projection type
pub const ProjectionType = enum(u32) {
    Perspective = 0,
    Orthographic = 1,
};

/// Viewport descriptor
pub const Viewport = extern struct {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_depth: f32,
    max_depth: f32,
};

/// Camera component
pub const Camera = extern struct {
    projection_type: u32, // ProjectionType
    viewport: Viewport,
    // Perspective parameters
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    // Orthographic parameters
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    // Cached projection matrix
    projection_matrix: [16]f32,
    matrix_dirty: bool,
};

/// Create default viewport
export fn camera_default_viewport() Viewport {
    return Viewport{
        .x = 0.0,
        .y = 0.0,
        .width = 800.0,
        .height = 600.0,
        .min_depth = 0.0,
        .max_depth = 1.0,
    };
}

/// Create viewport with dimensions
export fn camera_viewport_create(x: f32, y: f32, width: f32, height: f32) Viewport {
    return Viewport{
        .x = x,
        .y = y,
        .width = width,
        .height = height,
        .min_depth = 0.0,
        .max_depth = 1.0,
    };
}

/// Create perspective camera
export fn camera_perspective(fov: f32, aspect: f32, near: f32, far: f32) Camera {
    var camera = Camera{
        .projection_type = @intFromEnum(ProjectionType.Perspective),
        .viewport = camera_default_viewport(),
        .fov = fov,
        .aspect_ratio = aspect,
        .near = near,
        .far = far,
        .left = 0.0,
        .right = 0.0,
        .bottom = 0.0,
        .top = 0.0,
        .projection_matrix = [_]f32{0.0} ** 16,
        .matrix_dirty = true,
    };
    camera_update_projection_matrix(&camera);
    return camera;
}

/// Create orthographic camera
export fn camera_orthographic(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) Camera {
    var camera = Camera{
        .projection_type = @intFromEnum(ProjectionType.Orthographic),
        .viewport = camera_default_viewport(),
        .fov = 0.0,
        .aspect_ratio = 1.0,
        .near = near,
        .far = far,
        .left = left,
        .right = right,
        .bottom = bottom,
        .top = top,
        .projection_matrix = [_]f32{0.0} ** 16,
        .matrix_dirty = true,
    };
    camera_update_projection_matrix(&camera);
    return camera;
}

/// Create default perspective camera
export fn camera_default_perspective() Camera {
    return camera_perspective(
        std.math.pi / 4.0, // 45 degrees FOV
        800.0 / 600.0, // aspect ratio
        0.1, // near
        1000.0, // far
    );
}

/// Create default orthographic camera
export fn camera_default_orthographic() Camera {
    return camera_orthographic(
        -10.0, // left
        10.0, // right
        -10.0, // bottom
        10.0, // top
        0.1, // near
        1000.0, // far
    );
}

/// Update projection matrix
export fn camera_update_projection_matrix(camera: *Camera) void {
    if (camera.projection_type == @intFromEnum(ProjectionType.Perspective)) {
        // Perspective projection matrix (column-major)
        const tan_half_fov = @tan(camera.fov / 2.0);
        const f = 1.0 / tan_half_fov;
        const nf = 1.0 / (camera.near - camera.far);

        camera.projection_matrix[0] = f / camera.aspect_ratio;
        camera.projection_matrix[1] = 0.0;
        camera.projection_matrix[2] = 0.0;
        camera.projection_matrix[3] = 0.0;

        camera.projection_matrix[4] = 0.0;
        camera.projection_matrix[5] = f;
        camera.projection_matrix[6] = 0.0;
        camera.projection_matrix[7] = 0.0;

        camera.projection_matrix[8] = 0.0;
        camera.projection_matrix[9] = 0.0;
        camera.projection_matrix[10] = (camera.far + camera.near) * nf;
        camera.projection_matrix[11] = -1.0;

        camera.projection_matrix[12] = 0.0;
        camera.projection_matrix[13] = 0.0;
        camera.projection_matrix[14] = 2.0 * camera.far * camera.near * nf;
        camera.projection_matrix[15] = 0.0;
    } else {
        // Orthographic projection matrix (column-major)
        const rl = 1.0 / (camera.right - camera.left);
        const tb = 1.0 / (camera.top - camera.bottom);
        const fn_ = 1.0 / (camera.far - camera.near);

        camera.projection_matrix[0] = 2.0 * rl;
        camera.projection_matrix[1] = 0.0;
        camera.projection_matrix[2] = 0.0;
        camera.projection_matrix[3] = 0.0;

        camera.projection_matrix[4] = 0.0;
        camera.projection_matrix[5] = 2.0 * tb;
        camera.projection_matrix[6] = 0.0;
        camera.projection_matrix[7] = 0.0;

        camera.projection_matrix[8] = 0.0;
        camera.projection_matrix[9] = 0.0;
        camera.projection_matrix[10] = -2.0 * fn_;
        camera.projection_matrix[11] = 0.0;

        camera.projection_matrix[12] = -(camera.right + camera.left) * rl;
        camera.projection_matrix[13] = -(camera.top + camera.bottom) * tb;
        camera.projection_matrix[14] = -(camera.far + camera.near) * fn_;
        camera.projection_matrix[15] = 1.0;
    }

    camera.matrix_dirty = false;
}

/// Get projection matrix
export fn camera_get_projection_matrix(camera: *const Camera, out_matrix: *[16]f32) void {
    @memcpy(out_matrix, &camera.projection_matrix);
}

/// Set viewport
export fn camera_set_viewport(camera: *Camera, viewport: Viewport) void {
    camera.viewport = viewport;
    if (camera.projection_type == @intFromEnum(ProjectionType.Perspective)) {
        camera.aspect_ratio = viewport.width / viewport.height;
        camera.matrix_dirty = true;
    }
}

/// Set perspective parameters
export fn camera_set_perspective(camera: *Camera, fov: f32, aspect: f32, near: f32, far: f32) void {
    camera.projection_type = @intFromEnum(ProjectionType.Perspective);
    camera.fov = fov;
    camera.aspect_ratio = aspect;
    camera.near = near;
    camera.far = far;
    camera.matrix_dirty = true;
}

/// Set orthographic parameters
export fn camera_set_orthographic(
    camera: *Camera,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) void {
    camera.projection_type = @intFromEnum(ProjectionType.Orthographic);
    camera.left = left;
    camera.right = right;
    camera.bottom = bottom;
    camera.top = top;
    camera.near = near;
    camera.far = far;
    camera.matrix_dirty = true;
}

/// Check if matrix needs update
export fn camera_is_matrix_dirty(camera: *const Camera) bool {
    return camera.matrix_dirty;
}

/// Get aspect ratio
export fn camera_get_aspect_ratio(camera: *const Camera) f32 {
    if (camera.projection_type == @intFromEnum(ProjectionType.Perspective)) {
        return camera.aspect_ratio;
    }
    return camera.viewport.width / camera.viewport.height;
}
