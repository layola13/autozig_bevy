const std = @import("std");

pub const Mat4 = extern struct {
    cols: [4][4]f32,

    pub const IDENTITY = Mat4{ .cols = .{
        .{ 1.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 1.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 1.0 },
    } };

    pub const ZERO = Mat4{ .cols = .{
        .{ 0.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 0.0 },
    } };

    pub fn identity() Mat4 {
        return IDENTITY;
    }

    pub fn from_translation(translation: Vec3) Mat4 {
        return .{ .cols = .{
            .{ 1.0, 0.0, 0.0, 0.0 },
            .{ 0.0, 1.0, 0.0, 0.0 },
            .{ 0.0, 0.0, 1.0, 0.0 },
            .{ translation.x, translation.y, translation.z, 1.0 },
        } };
    }

    pub fn from_scale(scale: Vec3) Mat4 {
        return .{ .cols = .{
            .{ scale.x, 0.0, 0.0, 0.0 },
            .{ 0.0, scale.y, 0.0, 0.0 },
            .{ 0.0, 0.0, scale.z, 0.0 },
            .{ 0.0, 0.0, 0.0, 1.0 },
        } };
    }

    pub fn from_quat(q: Quat) Mat4 {
        const x2 = q.x + q.x;
        const y2 = q.y + q.y;
        const z2 = q.z + q.z;
        const xx = q.x * x2;
        const xy = q.x * y2;
        const xz = q.x * z2;
        const yy = q.y * y2;
        const yz = q.y * z2;
        const zz = q.z * z2;
        const wx = q.w * x2;
        const wy = q.w * y2;
        const wz = q.w * z2;

        return .{ .cols = .{
            .{ 1.0 - (yy + zz), xy + wz, xz - wy, 0.0 },
            .{ xy - wz, 1.0 - (xx + zz), yz + wx, 0.0 },
            .{ xz + wy, yz - wx, 1.0 - (xx + yy), 0.0 },
            .{ 0.0, 0.0, 0.0, 1.0 },
        } };
    }

    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) Mat4 {
        const rot = Mat4.from_quat(rotation);
        return .{ .cols = .{
            .{ rot.cols[0][0] * scale.x, rot.cols[0][1] * scale.x, rot.cols[0][2] * scale.x, 0.0 },
            .{ rot.cols[1][0] * scale.y, rot.cols[1][1] * scale.y, rot.cols[1][2] * scale.y, 0.0 },
            .{ rot.cols[2][0] * scale.z, rot.cols[2][1] * scale.z, rot.cols[2][2] * scale.z, 0.0 },
            .{ translation.x, translation.y, translation.z, 1.0 },
        } };
    }

    pub fn perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) Mat4 {
        const f = 1.0 / @tan(fov_y * 0.5);
        const range_inv = 1.0 / (z_near - z_far);
        return .{ .cols = .{
            .{ f / aspect, 0.0, 0.0, 0.0 },
            .{ 0.0, f, 0.0, 0.0 },
            .{ 0.0, 0.0, z_far * range_inv, -1.0 },
            .{ 0.0, 0.0, z_near * z_far * range_inv, 0.0 },
        } };
    }

    pub fn perspective_lh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) Mat4 {
        const f = 1.0 / @tan(fov_y * 0.5);
        const range_inv = 1.0 / (z_far - z_near);
        return .{ .cols = .{
            .{ f / aspect, 0.0, 0.0, 0.0 },
            .{ 0.0, f, 0.0, 0.0 },
            .{ 0.0, 0.0, z_far * range_inv, 1.0 },
            .{ 0.0, 0.0, -z_near * z_far * range_inv, 0.0 },
        } };
    }

    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) Mat4 {
        const rml = right - left;
        const tmb = top - bottom;
        const fmn = z_far - z_near;
        return .{ .cols = .{
            .{ 2.0 / rml, 0.0, 0.0, 0.0 },
            .{ 0.0, 2.0 / tmb, 0.0, 0.0 },
            .{ 0.0, 0.0, -1.0 / fmn, 0.0 },
            .{ -(right + left) / rml, -(top + bottom) / tmb, -z_near / fmn, 1.0 },
        } };
    }

    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) Mat4 {
        const f = center.sub(eye).normalize();
        const s = f.cross(up).normalize();
        const u = s.cross(f);

        return .{ .cols = .{
            .{ s.x, u.x, -f.x, 0.0 },
            .{ s.y, u.y, -f.y, 0.0 },
            .{ s.z, u.z, -f.z, 0.0 },
            .{ -s.dot(eye), -u.dot(eye), f.dot(eye), 1.0 },
        } };
    }

    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) Mat4 {
        const f = center.sub(eye).normalize();
        const s = up.cross(f).normalize();
        const u = f.cross(s);

        return .{ .cols = .{
            .{ s.x, u.x, f.x, 0.0 },
            .{ s.y, u.y, f.y, 0.0 },
            .{ s.z, u.z, f.z, 0.0 },
            .{ -s.dot(eye), -u.dot(eye), -f.dot(eye), 1.0 },
        } };
    }

    pub fn mul(self: Mat4, other: Mat4) Mat4 {
        var result: Mat4 = undefined;
        inline for (0..4) |col| {
            inline for (0..4) |row| {
                var sum: f32 = 0.0;
                inline for (0..4) |k| {
                    sum += self.cols[k][row] * other.cols[col][k];
                }
                result.cols[col][row] = sum;
            }
        }
        return result;
    }

    pub fn mul_vec3(self: Mat4, vec: Vec3) Vec3 {
        const x = vec.x;
        const y = vec.y;
        const z = vec.z;

        return .{
            .x = self.cols[0][0] * x + self.cols[1][0] * y + self.cols[2][0] * z + self.cols[3][0],
            .y = self.cols[0][1] * x + self.cols[1][1] * y + self.cols[2][1] * z + self.cols[3][1],
            .z = self.cols[0][2] * x + self.cols[1][2] * y + self.cols[2][2] * z + self.cols[3][2],
        };
    }

    pub fn mul_vec4(self: Mat4, vec: Vec4) Vec4 {
        return .{
            .x = self.cols[0][0] * vec.x + self.cols[1][0] * vec.y + self.cols[2][0] * vec.z + self.cols[3][0] * vec.w,
            .y = self.cols[0][1] * vec.x + self.cols[1][1] * vec.y + self.cols[2][1] * vec.z + self.cols[3][1] * vec.w,
            .z = self.cols[0][2] * vec.x + self.cols[1][2] * vec.y + self.cols[2][2] * vec.z + self.cols[3][2] * vec.w,
            .w = self.cols[0][3] * vec.x + self.cols[1][3] * vec.y + self.cols[2][3] * vec.z + self.cols[3][3] * vec.w,
        };
    }

    pub fn transpose(self: Mat4) Mat4 {
        return .{ .cols = .{
            .{ self.cols[0][0], self.cols[1][0], self.cols[2][0], self.cols[3][0] },
            .{ self.cols[0][1], self.cols[1][1], self.cols[2][1], self.cols[3][1] },
            .{ self.cols[0][2], self.cols[1][2], self.cols[2][2], self.cols[3][2] },
            .{ self.cols[0][3], self.cols[1][3], self.cols[2][3], self.cols[3][3] },
        } };
    }

    pub fn transform_point(self: Mat4, point: Vec3) Vec3 {
        return self.mul_vec3(point);
    }

    pub fn transform_vector(self: Mat4, vec: Vec3) Vec3 {
        return .{
            .x = self.cols[0][0] * vec.x + self.cols[1][0] * vec.y + self.cols[2][0] * vec.z,
            .y = self.cols[0][1] * vec.x + self.cols[1][1] * vec.y + self.cols[2][1] * vec.z,
            .z = self.cols[0][2] * vec.x + self.cols[1][2] * vec.y + self.cols[2][2] * vec.z,
        };
    }

    pub fn determinant(self: Mat4) f32 {
        const m = self.cols;

        const s0 = m[0][0] * m[1][1] - m[1][0] * m[0][1];
        const s1 = m[0][0] * m[2][1] - m[2][0] * m[0][1];
        const s2 = m[0][0] * m[3][1] - m[3][0] * m[0][1];
        const s3 = m[1][0] * m[2][1] - m[2][0] * m[1][1];
        const s4 = m[1][0] * m[3][1] - m[3][0] * m[1][1];
        const s5 = m[2][0] * m[3][1] - m[3][0] * m[2][1];

        const c5 = m[2][2] * m[3][3] - m[3][2] * m[2][3];
        const c4 = m[1][2] * m[3][3] - m[3][2] * m[1][3];
        const c3 = m[1][2] * m[2][3] - m[2][2] * m[1][3];
        const c2 = m[0][2] * m[3][3] - m[3][2] * m[0][3];
        const c1 = m[0][2] * m[2][3] - m[2][2] * m[0][3];
        const c0 = m[0][2] * m[1][3] - m[1][2] * m[0][3];

        return s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;
    }

    pub fn inverse(self: Mat4) Mat4 {
        const m = self.cols;

        const s0 = m[0][0] * m[1][1] - m[1][0] * m[0][1];
        const s1 = m[0][0] * m[2][1] - m[2][0] * m[0][1];
        const s2 = m[0][0] * m[3][1] - m[3][0] * m[0][1];
        const s3 = m[1][0] * m[2][1] - m[2][0] * m[1][1];
        const s4 = m[1][0] * m[3][1] - m[3][0] * m[1][1];
        const s5 = m[2][0] * m[3][1] - m[3][0] * m[2][1];

        const c5 = m[2][2] * m[3][3] - m[3][2] * m[2][3];
        const c4 = m[1][2] * m[3][3] - m[3][2] * m[1][3];
        const c3 = m[1][2] * m[2][3] - m[2][2] * m[1][3];
        const c2 = m[0][2] * m[3][3] - m[3][2] * m[0][3];
        const c1 = m[0][2] * m[2][3] - m[2][2] * m[0][3];
        const c0 = m[0][2] * m[1][3] - m[1][2] * m[0][3];

        const det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;

        if (@abs(det) < std.math.floatEps(f32)) {
            return IDENTITY;
        }

        const inv_det = 1.0 / det;

        return .{ .cols = .{
            .{
                (m[1][1] * c5 - m[2][1] * c4 + m[3][1] * c3) * inv_det,
                (-m[0][1] * c5 + m[2][1] * c2 - m[3][1] * c1) * inv_det,
                (m[0][1] * c4 - m[1][1] * c2 + m[3][1] * c0) * inv_det,
                (-m[0][1] * c3 + m[1][1] * c1 - m[2][1] * c0) * inv_det,
            },
            .{
                (-m[1][0] * c5 + m[2][0] * c4 - m[3][0] * c3) * inv_det,
                (m[0][0] * c5 - m[2][0] * c2 + m[3][0] * c1) * inv_det,
                (-m[0][0] * c4 + m[1][0] * c2 - m[3][0] * c0) * inv_det,
                (m[0][0] * c3 - m[1][0] * c1 + m[2][0] * c0) * inv_det,
            },
            .{
                (m[1][3] * s5 - m[2][3] * s4 + m[3][3] * s3) * inv_det,
                (-m[0][3] * s5 + m[2][3] * s2 - m[3][3] * s1) * inv_det,
                (m[0][3] * s4 - m[1][3] * s2 + m[3][3] * s0) * inv_det,
                (-m[0][3] * s3 + m[1][3] * s1 - m[2][3] * s0) * inv_det,
            },
            .{
                (-m[1][2] * s5 + m[2][2] * s4 - m[3][2] * s3) * inv_det,
                (m[0][2] * s5 - m[2][2] * s2 + m[3][2] * s1) * inv_det,
                (-m[0][2] * s4 + m[1][2] * s2 - m[3][2] * s0) * inv_det,
                (m[0][2] * s3 - m[1][2] * s1 + m[2][2] * s0) * inv_det,
            },
        } };
    }
};

export fn mat4_identity() Mat4 {
    return Mat4.identity();
}

export fn mat4_from_translation(translation: Vec3) Mat4 {
    return Mat4.from_translation(translation);
}

export fn mat4_from_scale(scale: Vec3) Mat4 {
    return Mat4.from_scale(scale);
}

export fn mat4_from_quat(q: Quat) Mat4 {
    return Mat4.from_quat(q);
}

export fn mat4_from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) Mat4 {
    return Mat4.from_scale_rotation_translation(scale, rotation, translation);
}

export fn mat4_perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) Mat4 {
    return Mat4.perspective_rh(fov_y, aspect, z_near, z_far);
}

export fn mat4_perspective_lh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) Mat4 {
    return Mat4.perspective_lh(fov_y, aspect, z_near, z_far);
}

export fn mat4_orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) Mat4 {
    return Mat4.orthographic_rh(left, right, bottom, top, z_near, z_far);
}

export fn mat4_look_at_rh(eye: Vec3, center: Vec3, up: Vec3) Mat4 {
    return Mat4.look_at_rh(eye, center, up);
}

export fn mat4_look_at_lh(eye: Vec3, center: Vec3, up: Vec3) Mat4 {
    return Mat4.look_at_lh(eye, center, up);
}

export fn mat4_mul(self: Mat4, other: Mat4) Mat4 {
    return self.mul(other);
}

export fn mat4_mul_vec3(self: Mat4, vec: Vec3) Vec3 {
    return self.mul_vec3(vec);
}

export fn mat4_mul_vec4(self: Mat4, vec: Vec4) Vec4 {
    return self.mul_vec4(vec);
}

export fn mat4_transpose(self: Mat4) Mat4 {
    return self.transpose();
}

export fn mat4_transform_point(self: Mat4, point: Vec3) Vec3 {
    return self.transform_point(point);
}

export fn mat4_transform_vector(self: Mat4, vec: Vec3) Vec3 {
    return self.transform_vector(vec);
}

export fn mat4_determinant(self: Mat4) f32 {
    return self.determinant();
}

export fn mat4_inverse(self: Mat4) Mat4 {
    return self.inverse();
}
