const std = @import("std");

/// FloatPow - power operations for floating point numbers
pub const FloatPow = struct {
    /// Returns self raised to the power of exp
    pub fn powf_f32(base: f32, exp: f32) f32 {
        return std.math.pow(f32, base, exp);
    }

    pub fn powf_f64(base: f64, exp: f64) f64 {
        return std.math.pow(f64, base, exp);
    }

    /// Returns self raised to an integer power
    pub fn powi_f32(base: f32, exp: i32) f32 {
        if (exp == 0) return 1.0;
        var result: f32 = 1.0;
        var n = if (exp < 0) -exp else exp;
        const b = base;
        while (n > 0) : (n -= 1) {
            result *= b;
        }
        return if (exp < 0) 1.0 / result else result;
    }

    pub fn powi_f64(base: f64, exp: i32) f64 {
        if (exp == 0) return 1.0;
        var result: f64 = 1.0;
        var n = if (exp < 0) -exp else exp;
        const b = base;
        while (n > 0) : (n -= 1) {
            result *= b;
        }
        return if (exp < 0) 1.0 / result else result;
    }

    /// Square root
    pub fn sqrt_f32(x: f32) f32 {
        return @sqrt(x);
    }

    pub fn sqrt_f64(x: f64) f64 {
        return @sqrt(x);
    }

    /// Cube root
    pub fn cbrt_f32(x: f32) f32 {
        if (x < 0) {
            return -std.math.pow(f32, -x, 1.0 / 3.0);
        }
        return std.math.pow(f32, x, 1.0 / 3.0);
    }

    pub fn cbrt_f64(x: f64) f64 {
        if (x < 0) {
            return -std.math.pow(f64, -x, 1.0 / 3.0);
        }
        return std.math.pow(f64, x, 1.0 / 3.0);
    }

    /// Exponential (e^x)
    pub fn exp_f32(x: f32) f32 {
        return @exp(x);
    }

    pub fn exp_f64(x: f64) f64 {
        return @exp(x);
    }

    /// Natural logarithm
    pub fn ln_f32(x: f32) f32 {
        return @log(x);
    }

    pub fn ln_f64(x: f64) f64 {
        return @log(x);
    }

    /// Base-2 logarithm
    pub fn log2_f32(x: f32) f32 {
        return @log2(x);
    }

    pub fn log2_f64(x: f64) f64 {
        return @log2(x);
    }

    /// Base-10 logarithm
    pub fn log10_f32(x: f32) f32 {
        return @log10(x);
    }

    pub fn log10_f64(x: f64) f64 {
        return @log10(x);
    }
};

export fn float_powf_f32(base: f32, exp: f32) f32 {
    return FloatPow.powf_f32(base, exp);
}

export fn float_powf_f64(base: f64, exp: f64) f64 {
    return FloatPow.powf_f64(base, exp);
}

export fn float_powi_f32(base: f32, exp: i32) f32 {
    return FloatPow.powi_f32(base, exp);
}

export fn float_sqrt_f32(x: f32) f32 {
    return FloatPow.sqrt_f32(x);
}

export fn float_cbrt_f32(x: f32) f32 {
    return FloatPow.cbrt_f32(x);
}

export fn float_exp_f32(x: f32) f32 {
    return FloatPow.exp_f32(x);
}

export fn float_ln_f32(x: f32) f32 {
    return FloatPow.ln_f32(x);
}

export fn float_log2_f32(x: f32) f32 {
    return FloatPow.log2_f32(x);
}

export fn float_log10_f32(x: f32) f32 {
    return FloatPow.log10_f32(x);
}
