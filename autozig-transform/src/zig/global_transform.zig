const std = @import("std");

pub const GlobalTransform = extern struct {
    matrix: [16]f32,
};

pub export fn global_transform_identity() GlobalTransform {
    return GlobalTransform{
        .matrix = [16]f32{
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        },
    };
}

pub export fn global_transform_from_matrix(matrix: *const [16]f32) GlobalTransform {
    return GlobalTransform{ .matrix = matrix.* };
}

pub export fn global_transform_from_transform(transform: *const Transform) GlobalTransform {
    var result: GlobalTransform = undefined;
    transform_compute_matrix(transform, &result.matrix);
    return result;
}

pub export fn global_transform_mul_transform(
    global: *const GlobalTransform,
    transform: *const Transform,
    out: *GlobalTransform,
) void {
    var local_matrix: [16]f32 = undefined;
    transform_compute_matrix(transform, &local_matrix);

    // Matrix multiplication: global * local
    var result: [16]f32 = undefined;
    var i: usize = 0;
    while (i < 4) : (i += 1) {
        var j: usize = 0;
        while (j < 4) : (j += 1) {
            result[i * 4 + j] = 0.0;
            var k: usize = 0;
            while (k < 4) : (k += 1) {
                result[i * 4 + j] += global.matrix[i * 4 + k] * local_matrix[k * 4 + j];
            }
        }
    }

    out.matrix = result;
}

pub export fn global_transform_transform_point(
    global: *const GlobalTransform,
    point: *const [3]f32,
    out: *[3]f32,
) void {
    const m = global.matrix;
    const p = point.*;

    // Homogeneous coordinate transformation
    const x = m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12];
    const y = m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13];
    const z = m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14];

    out[0] = x;
    out[1] = y;
    out[2] = z;
}

pub export fn global_transform_transform_vector(
    global: *const GlobalTransform,
    vector: *const [3]f32,
    out: *[3]f32,
) void {
    const m = global.matrix;
    const v = vector.*;

    // Vector transformation (no translation)
    const x = m[0] * v[0] + m[4] * v[1] + m[8] * v[2];
    const y = m[1] * v[0] + m[5] * v[1] + m[9] * v[2];
    const z = m[2] * v[0] + m[6] * v[1] + m[10] * v[2];

    out[0] = x;
    out[1] = y;
    out[2] = z;
}

// Forward declarations for Transform functions
const Transform = extern struct {
    translation: [3]f32,
    rotation: [4]f32,
    scale: [3]f32,
};

extern fn transform_compute_matrix(transform: *const Transform, out_matrix: *[16]f32) void;
