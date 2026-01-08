// LocalToWorld component for world-space transformation
// Stores the final 4x4 transformation matrix

const std = @import("std");

/// LocalToWorld component storing the world-space transformation matrix
/// Uses 4x4 matrix in column-major order (WebGL/WebGPU compatible)
pub const LocalToWorld = extern struct {
    matrix: [16]f32,

    /// Create identity local-to-world matrix
    pub fn identity() LocalToWorld {
        return LocalToWorld{
            .matrix = [16]f32{
                1.0, 0.0, 0.0, 0.0, // Column 0
                0.0, 1.0, 0.0, 0.0, // Column 1
                0.0, 0.0, 1.0, 0.0, // Column 2
                0.0, 0.0, 0.0, 1.0, // Column 3
            },
        };
    }

    /// Create local-to-world from a 4x4 matrix
    pub fn from_matrix(matrix: [16]f32) LocalToWorld {
        return LocalToWorld{
            .matrix = matrix,
        };
    }

    /// Get the transformation matrix
    pub fn get_matrix(self: *const LocalToWorld) [16]f32 {
        return self.matrix;
    }

    /// Set the transformation matrix
    pub fn set_matrix(self: *LocalToWorld, matrix: [16]f32) void {
        self.matrix = matrix;
    }

    /// Extract translation from the matrix
    pub fn get_translation(self: *const LocalToWorld) [3]f32 {
        return [3]f32{
            self.matrix[12],
            self.matrix[13],
            self.matrix[14],
        };
    }

    /// Extract scale from the matrix (approximate, assumes no skew)
    pub fn get_scale(self: *const LocalToWorld) [3]f32 {
        // Calculate magnitude of each basis vector
        const sx = @sqrt(
            self.matrix[0] * self.matrix[0] +
                self.matrix[1] * self.matrix[1] +
                self.matrix[2] * self.matrix[2],
        );
        const sy = @sqrt(
            self.matrix[4] * self.matrix[4] +
                self.matrix[5] * self.matrix[5] +
                self.matrix[6] * self.matrix[6],
        );
        const sz = @sqrt(
            self.matrix[8] * self.matrix[8] +
                self.matrix[9] * self.matrix[9] +
                self.matrix[10] * self.matrix[10],
        );

        return [3]f32{ sx, sy, sz };
    }

    /// Multiply this matrix by another (this = this * other)
    pub fn multiply(self: *LocalToWorld, other: [16]f32) void {
        const a = self.matrix;
        const b = other;
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

        self.matrix = result;
    }

    /// Copy matrix from another LocalToWorld
    pub fn copy_from(self: *LocalToWorld, other: *const LocalToWorld) void {
        self.matrix = other.matrix;
    }

    /// Check if this is an identity matrix (within epsilon)
    pub fn is_identity(self: *const LocalToWorld) bool {
        const epsilon = 0.0001;
        const identity_matrix = LocalToWorld.identity();

        var i: usize = 0;
        while (i < 16) : (i += 1) {
            const diff = @abs(self.matrix[i] - identity_matrix.matrix[i]);
            if (diff > epsilon) {
                return false;
            }
        }
        return true;
    }
};

// Export C-compatible functions for FFI
export fn local_to_world_identity() LocalToWorld {
    return LocalToWorld.identity();
}

export fn local_to_world_from_matrix(matrix: *const [16]f32) LocalToWorld {
    return LocalToWorld.from_matrix(matrix.*);
}

export fn local_to_world_get_matrix(ltw: *const LocalToWorld, out_matrix: *[16]f32) void {
    out_matrix.* = ltw.get_matrix();
}

export fn local_to_world_set_matrix(ltw: *LocalToWorld, matrix: *const [16]f32) void {
    ltw.set_matrix(matrix.*);
}

export fn local_to_world_get_translation(ltw: *const LocalToWorld, out_translation: *[3]f32) void {
    out_translation.* = ltw.get_translation();
}

export fn local_to_world_get_scale(ltw: *const LocalToWorld, out_scale: *[3]f32) void {
    out_scale.* = ltw.get_scale();
}

export fn local_to_world_multiply(ltw: *LocalToWorld, other: *const [16]f32) void {
    ltw.multiply(other.*);
}

export fn local_to_world_copy_from(dest: *LocalToWorld, src: *const LocalToWorld) void {
    dest.copy_from(src);
}

export fn local_to_world_is_identity(ltw: *const LocalToWorld) bool {
    return ltw.is_identity();
}
