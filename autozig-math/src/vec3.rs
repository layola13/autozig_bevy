use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

include_zig!("zig/vec3.zig", {
    fn vec3_new(x: f32, y: f32, z: f32) -> Vec3;
    fn vec3_splat(value: f32) -> Vec3;
    fn vec3_dot(self_: Vec3, other: Vec3) -> f32;
    fn vec3_cross(self_: Vec3, other: Vec3) -> Vec3;
    fn vec3_length_squared(self_: Vec3) -> f32;
    fn vec3_length(self_: Vec3) -> f32;
    fn vec3_add(self_: Vec3, other: Vec3) -> Vec3;
    fn vec3_sub(self_: Vec3, other: Vec3) -> Vec3;
    fn vec3_mul_scalar(self_: Vec3, s: f32) -> Vec3;
    fn vec3_normalize(self_: Vec3) -> Vec3;
    fn vec3_distance(self_: Vec3, other: Vec3) -> f32;
    fn vec3_lerp(self_: Vec3, other: Vec3, t: f32) -> Vec3;
    fn vec3_min(self_: Vec3, other: Vec3) -> Vec3;
    fn vec3_max(self_: Vec3, other: Vec3) -> Vec3;
    fn vec3_abs(self_: Vec3) -> Vec3;
    fn vec3_reflect(self_: Vec3, normal: Vec3) -> Vec3;
    fn vec3_project_onto(self_: Vec3, other: Vec3) -> Vec3;
    fn vec3_angle_between(self_: Vec3, other: Vec3) -> f32;
    fn vec3_any_orthogonal_vector(self_: Vec3) -> Vec3;
});

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };
    pub const NEG_X: Self = Self { x: -1.0, y: 0.0, z: 0.0 };
    pub const NEG_Y: Self = Self { x: 0.0, y: -1.0, z: 0.0 };
    pub const NEG_Z: Self = Self { x: 0.0, y: 0.0, z: -1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3_new(x, y, z)
    }

    pub fn splat(value: f32) -> Self {
        vec3_splat(value)
    }

    pub fn dot(self, other: Self) -> f32 {
        vec3_dot(self, other)
    }

    pub fn cross(self, other: Self) -> Self {
        vec3_cross(self, other)
    }

    pub fn length_squared(self) -> f32 {
        vec3_length_squared(self)
    }

    pub fn length(self) -> f32 {
        vec3_length(self)
    }

    pub fn normalize(self) -> Self {
        vec3_normalize(self)
    }

    pub fn distance(self, other: Self) -> f32 {
        vec3_distance(self, other)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        vec3_lerp(self, other, t)
    }

    pub fn min(self, other: Self) -> Self {
        vec3_min(self, other)
    }

    pub fn max(self, other: Self) -> Self {
        vec3_max(self, other)
    }

    pub fn abs(self) -> Self {
        vec3_abs(self)
    }

    pub fn reflect(self, normal: Self) -> Self {
        vec3_reflect(self, normal)
    }

    pub fn project_onto(self, other: Self) -> Self {
        vec3_project_onto(self, other)
    }

    pub fn angle_between(self, other: Self) -> f32 {
        vec3_angle_between(self, other)
    }

    pub fn any_orthogonal_vector(self) -> Self {
        vec3_any_orthogonal_vector(self)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec3_add(self, rhs)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec3_sub(self, rhs)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        vec3_mul_scalar(self, rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        vec3_mul_scalar(rhs, self)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}
