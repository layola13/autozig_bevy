// Transform component for 2D/3D transformations
// WebGPU/WASM optimized implementation using 4x4 matrices

const std = @import("std");

/// Transform component with translation, rotation (quaternion), and scale
/// Uses extern struct for FFI compatibility with Rust
pub const Transform = extern struct {
    translation: [3]f32,
    rotation: [4]f32, // Quaternion (x, y, z, w)
    scale: [3]f32,

    /// Create identity transform
    pub fn identity() Transform {
        return Transform{
            .translation = [3]f32{ 0.0, 0.0, 0.0 },
            .rotation = [4]f32{ 0.0, 0.0, 0.0, 1.0 }, // Identity quaternion
            .scale = [3]f32{ 1.0, 1.0, 1.0 },
        };
    }

    /// Create transform from translation
    pub fn from_translation(translation: [3]f32) Transform {
        return Transform{
            .translation = translation,
            .rotation = [4]f32{ 0.0, 0.0, 0.0, 1.0 },
            .scale = [3]f32{ 1.0, 1.0, 1.0 },
        };
    }

    /// Create transform from rotation (quaternion)
    pub fn from_rotation(rotation: [4]f32) Transform {
        return Transform{
            .translation = [3]f32{ 0.0, 0.0, 0.0 },
            .rotation = rotation,
            .scale = [3]f32{ 1.0, 1.0, 1.0 },
        };
    }

    /// Create transform from scale
    pub fn from_scale(scale: [3]f32) Transform {
        return Transform{
            .translation = [3]f32{ 0.0, 0.0, 0.0 },
            .rotation = [4]f32{ 0.0, 0.0, 0.0, 1.0 },
            .scale = scale,
        };
    }

    /// Compute 4x4 transformation matrix (WebGL compatible)
    /// Column-major order for WebGL/WebGPU
    pub fn compute_matrix(self: *const Transform) [16]f32 {
        // Extract quaternion components
        const x = self.rotation[0];
        const y = self.rotation[1];
        const z = self.rotation[2];
        const w = self.rotation[3];

        // Compute rotation matrix from quaternion
        const x2 = x + x;
        const y2 = y + y;
        const z2 = z + z;
        const xx = x * x2;
        const xy = x * y2;
        const xz = x * z2;
        const yy = y * y2;
        const yz = y * z2;
        const zz = z * z2;
        const wx = w * x2;
        const wy = w * y2;
        const wz = w * z2;

        // Build 4x4 matrix with scale and translation (column-major)
        var matrix: [16]f32 = undefined;

        // Column 0
        matrix[0] = (1.0 - (yy + zz)) * self.scale[0];
        matrix[1] = (xy + wz) * self.scale[0];
        matrix[2] = (xz - wy) * self.scale[0];
        matrix[3] = 0.0;

        // Column 1
        matrix[4] = (xy - wz) * self.scale[1];
        matrix[5] = (1.0 - (xx + zz)) * self.scale[1];
        matrix[6] = (yz + wx) * self.scale[1];
        matrix[7] = 0.0;

        // Column 2
        matrix[8] = (xz + wy) * self.scale[2];
        matrix[9] = (yz - wx) * self.scale[2];
        matrix[10] = (1.0 - (xx + yy)) * self.scale[2];
        matrix[11] = 0.0;

        // Column 3 (translation)
        matrix[12] = self.translation[0];
        matrix[13] = self.translation[1];
        matrix[14] = self.translation[2];
        matrix[15] = 1.0;

        return matrix;
    }

    /// Compute local-to-world matrix by multiplying with parent matrix
    pub fn compute_local_to_world(self: *const Transform, parent_matrix: [16]f32) [16]f32 {
        const local_matrix = self.compute_matrix();
        return multiply_matrices(parent_matrix, local_matrix);
    }
};

/// Multiply two 4x4 matrices (column-major order)
fn multiply_matrices(a: [16]f32, b: [16]f32) [16]f32 {
    var result: [16]f32 = undefined;

    // Matrix multiplication: result = a * b
    var col: usize = 0;
    while (col < 4) : (col += 1) {
        var row: usize = 0;
        while (row < 4) : (row += 1) {
            const idx = col * 4 + row;
            result[idx] = a[row] * b[col * 4] +
                a[4 + row] * b[col * 4 + 1] +
                a[8 + row] * b[col * 4 + 2] +
                a[12 + row] * b[col * 4 + 3];
        }
    }

    return result;
}

// Export C-compatible functions for FFI
export fn transform_identity() Transform {
    return Transform.identity();
}

export fn transform_from_translation(translation: *const [3]f32) Transform {
    return Transform.from_translation(translation.*);
}

export fn transform_from_rotation(rotation: *const [4]f32) Transform {
    return Transform.from_rotation(rotation.*);
}

export fn transform_from_scale(scale: *const [3]f32) Transform {
    return Transform.from_scale(scale.*);
}

export fn transform_compute_matrix(transform: *const Transform, out_matrix: *[16]f32) void {
    out_matrix.* = transform.compute_matrix();
}

export fn transform_compute_local_to_world(
    transform: *const Transform,
    parent_matrix: *const [16]f32,
    out_matrix: *[16]f32,
) void {
    out_matrix.* = transform.compute_local_to_world(parent_matrix.*);
}
