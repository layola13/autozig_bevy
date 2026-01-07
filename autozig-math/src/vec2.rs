use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

include_zig!("zig/vec2.zig", {
    fn vec2_new(x: f32, y: f32) -> Vec2;
    fn vec2_splat(value: f32) -> Vec2;
    fn vec2_dot(self_: Vec2, other: Vec2) -> f32;
    fn vec2_add(self_: Vec2, other: Vec2) -> Vec2;
    fn vec2_sub(self_: Vec2, other: Vec2) -> Vec2;
    fn vec2_mul_scalar(self_: Vec2, s: f32) -> Vec2;
    fn vec2_length(self_: Vec2) -> f32;
    fn vec2_length_squared(self_: Vec2) -> f32;
    fn vec2_normalize(self_: Vec2) -> Vec2;
    fn vec2_distance(self_: Vec2, other: Vec2) -> f32;
    fn vec2_lerp(self_: Vec2, other: Vec2, t: f32) -> Vec2;
    fn vec2_min(self_: Vec2, other: Vec2) -> Vec2;
    fn vec2_max(self_: Vec2, other: Vec2) -> Vec2;
    fn vec2_abs(self_: Vec2) -> Vec2;
    fn vec2_perp(self_: Vec2) -> Vec2;
    fn vec2_perp_dot(self_: Vec2, other: Vec2) -> f32;
    fn vec2_rotate(self_: Vec2, angle: f32) -> Vec2;
    fn vec2_reflect(self_: Vec2, normal: Vec2) -> Vec2;
    fn vec2_angle_between(self_: Vec2, other: Vec2) -> f32;
    fn vec2_project_onto(self_: Vec2, other: Vec2) -> Vec2;
});

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0 };
    pub const NEG_X: Self = Self { x: -1.0, y: 0.0 };
    pub const NEG_Y: Self = Self { x: 0.0, y: -1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        vec2_new(x, y)
    }

    pub fn splat(value: f32) -> Self {
        vec2_splat(value)
    }

    pub fn dot(self, other: Self) -> f32 {
        vec2_dot(self, other)
    }

    pub fn length(self) -> f32 {
        vec2_length(self)
    }

    pub fn length_squared(self) -> f32 {
        vec2_length_squared(self)
    }

    pub fn normalize(self) -> Self {
        vec2_normalize(self)
    }

    pub fn distance(self, other: Self) -> f32 {
        vec2_distance(self, other)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        vec2_lerp(self, other, t)
    }

    pub fn min(self, other: Self) -> Self {
        vec2_min(self, other)
    }

    pub fn max(self, other: Self) -> Self {
        vec2_max(self, other)
    }

    pub fn abs(self) -> Self {
        vec2_abs(self)
    }

    pub fn perp(self) -> Self {
        vec2_perp(self)
    }

    pub fn perp_dot(self, other: Self) -> f32 {
        vec2_perp_dot(self, other)
    }

    pub fn rotate(self, angle: f32) -> Self {
        vec2_rotate(self, angle)
    }

    pub fn reflect(self, normal: Self) -> Self {
        vec2_reflect(self, normal)
    }

    pub fn angle_between(self, other: Self) -> f32 {
        vec2_angle_between(self, other)
    }

    pub fn project_onto(self, other: Self) -> Self {
        vec2_project_onto(self, other)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec2_add(self, rhs)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec2_sub(self, rhs)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        vec2_mul_scalar(self, rhs)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        vec2_mul_scalar(rhs, self)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}
