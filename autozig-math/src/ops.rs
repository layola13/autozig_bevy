use autozig::include_zig;

include_zig!("zig/ops.zig", {
    fn float_powf_f32(base: f32, exp: f32) -> f32;
    fn float_powf_f64(base: f64, exp: f64) -> f64;
    fn float_powi_f32(base: f32, exp: i32) -> f32;
    fn float_sqrt_f32(x: f32) -> f32;
    fn float_cbrt_f32(x: f32) -> f32;
    fn float_exp_f32(x: f32) -> f32;
    fn float_ln_f32(x: f32) -> f32;
    fn float_log2_f32(x: f32) -> f32;
    fn float_log10_f32(x: f32) -> f32;
});

/// Power and exponential operations trait
pub trait FloatPow {
    fn powf(self, exp: Self) -> Self;
    fn powi(self, exp: i32) -> Self;
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
}

impl FloatPow for f32 {
    fn powf(self, exp: Self) -> Self {
        float_powf_f32(self, exp)
    }
    fn powi(self, exp: i32) -> Self {
        float_powi_f32(self, exp)
    }
    fn sqrt(self) -> Self {
        float_sqrt_f32(self)
    }
    fn cbrt(self) -> Self {
        float_cbrt_f32(self)
    }
    fn exp(self) -> Self {
        float_exp_f32(self)
    }
    fn ln(self) -> Self {
        float_ln_f32(self)
    }
    fn log2(self) -> Self {
        float_log2_f32(self)
    }
    fn log10(self) -> Self {
        float_log10_f32(self)
    }
}

// Note: f64 implementations would need additional exports from Zig
// For now, we use Rust's built-in methods for f64
impl FloatPow for f64 {
    fn powf(self, exp: Self) -> Self {
        self.powf(exp)
    }
    fn powi(self, exp: i32) -> Self {
        self.powi(exp) 
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn cbrt(self) -> Self {
        self.cbrt()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log2(self) -> Self {
        self.log2()
    }
    fn log10(self) -> Self {
        self.log10()
    }
}
