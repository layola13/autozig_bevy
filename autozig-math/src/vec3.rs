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

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn splat(value: f32) -> Self {
        vec3_splat(value)
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        vec3_dot(self, other)
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        vec3_cross(self, other)
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        vec3_length_squared(self)
    }

    #[inline]
    pub fn length(self) -> f32 {
        vec3_length(self)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        vec3_normalize(self)
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero vector.
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let len = self.length();
        if len > 1e-10 {
            self * (1.0 / len)
        } else {
            Self::ZERO
        }
    }

    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        vec3_distance(self, other)
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        vec3_lerp(self, other, t)
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        vec3_min(self, other)
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        vec3_max(self, other)
    }

    #[inline]
    pub fn abs(self) -> Self {
        vec3_abs(self)
    }

    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        vec3_reflect(self, normal)
    }

    #[inline]
    pub fn project_onto(self, other: Self) -> Self {
        vec3_project_onto(self, other)
    }

    #[inline]
    pub fn angle_between(self, other: Self) -> f32 {
        vec3_angle_between(self, other)
    }

    #[inline]
    pub fn any_orthogonal_vector(self) -> Self {
        vec3_any_orthogonal_vector(self)
    }

    /// Creates a `Vec3` from a `[f32; 3]` array.
    #[inline]
    pub const fn from_array(a: [f32; 3]) -> Self {
        Self { x: a[0], y: a[1], z: a[2] }
    }

    /// Converts the `Vec3` to a `[f32; 3]` array.
    #[inline]
    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Returns `true` if all components are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec3_add(self, rhs)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = vec3_add(*self, rhs);
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec3_sub(self, rhs)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = vec3_sub(*self, rhs);
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

/// Component-wise multiplication (Hadamard product).
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

