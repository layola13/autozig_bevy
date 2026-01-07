const std = @import("std");

pub const EaseFunction = enum(u32) {
    Linear = 0,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SineIn,
    SineOut,
    SineInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
    SmoothStepIn,
    SmoothStepOut,
    SmoothStep,
    SmootherStepIn,
    SmootherStepOut,
    SmootherStep,
};

pub fn sample_ease(ease: EaseFunction, t: f32) f32 {
    switch (ease) {
        .Linear => return t,
        .QuadraticIn => return t * t,
        .QuadraticOut => return -(t * (t - 2.0)),
        .QuadraticInOut => {
            if (t < 0.5) return 2.0 * t * t;
            return (-2.0 * t * t) + (4.0 * t) - 1.0;
        },
        .CubicIn => return t * t * t,
        .CubicOut => {
            const f = t - 1.0;
            return f * f * f + 1.0;
        },
        .CubicInOut => {
            if (t < 0.5) return 4.0 * t * t * t;
            const f = (2.0 * t) - 2.0;
            return 0.5 * f * f * f + 1.0;
        },
        .QuarticIn => return t * t * t * t,
        .QuarticOut => {
            const f = t - 1.0;
            return 1.0 - (f * f * f * f);
        },
        .QuarticInOut => {
            if (t < 0.5) return 8.0 * t * t * t * t;
            const f = t - 1.0;
            return 1.0 - 8.0 * f * f * f * f;
        },
        .QuinticIn => return t * t * t * t * t,
        .QuinticOut => {
            const f = t - 1.0;
            return f * f * f * f * f + 1.0;
        },
        .QuinticInOut => {
            if (t < 0.5) return 16.0 * t * t * t * t * t;
            const f = (2.0 * t) - 2.0;
            return 0.5 * f * f * f * f * f + 1.0;
        },
        .SineIn => return 1.0 - @cos((t * std.math.pi) / 2.0),
        .SineOut => return @sin((t * std.math.pi) / 2.0),
        .SineInOut => return -(degrees_cos(std.math.pi * t) - 1.0) / 2.0,
        .CircularIn => return 1.0 - @sqrt(1.0 - t * t),
        .CircularOut => return @sqrt((2.0 - t) * t),
        .CircularInOut => {
            if (t < 0.5) return 0.5 * (1.0 - @sqrt(1.0 - 4.0 * t * t));
            return 0.5 * (@sqrt(-((2.0 * t) - 3.0) * ((2.0 * t) - 1.0)) + 1.0);
        },
        .ExponentialIn => {
            if (t == 0.0) return 0.0;
            return @exp2(10.0 * (t - 1.0));
        },
        .ExponentialOut => {
            if (t == 1.0) return 1.0;
            return 1.0 - @exp2(-10.0 * t);
        },
        .ExponentialInOut => {
            if (t == 0.0) return 0.0;
            if (t == 1.0) return 1.0;
            if (t < 0.5) return @exp2(20.0 * t - 10.0) / 2.0;
            return (2.0 - @exp2(-20.0 * t + 10.0)) / 2.0;
        },
        .ElasticIn => {
            if (t == 0.0) return 0.0;
            if (t == 1.0) return 1.0;
            return -@exp2(10.0 * t - 10.0) * @sin((t * 10.0 - 10.75) * ((2.0 * std.math.pi) / 3.0));
        },
        .ElasticOut => {
            if (t == 0.0) return 0.0;
            if (t == 1.0) return 1.0;
            return @exp2(-10.0 * t) * @sin((t * 10.0 - 0.75) * ((2.0 * std.math.pi) / 3.0)) + 1.0;
        },
        .ElasticInOut => {
            if (t == 0.0) return 0.0;
            if (t == 1.0) return 1.0;
            if (t < 0.5) {
                return -(@exp2(20.0 * t - 10.0) * @sin((20.0 * t - 11.125) * ((2.0 * std.math.pi) / 4.5))) / 2.0;
            }
            return (@exp2(-20.0 * t + 10.0) * @sin((20.0 * t - 11.125) * ((2.0 * std.math.pi) / 4.5))) / 2.0 + 1.0;
        },
        .BackIn => {
            const c1 = 1.70158;
            const c3 = c1 + 1.0;
            return c3 * t * t * t - c1 * t * t;
        },
        .BackOut => {
            const c1 = 1.70158;
            const c3 = c1 + 1.0;
            const f = t - 1.0;
            return 1.0 + c3 * f * f * f + c1 * f * f;
        },
        .BackInOut => {
            const c1 = 1.70158;
            const c2 = c1 * 1.525;
            if (t < 0.5) {
                return (std.math.pow(f32, 2.0 * t, 2.0) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0;
            }
            return (std.math.pow(f32, 2.0 * t - 2.0, 2.0) * ((c2 + 1.0) * (2.0 * t - 2.0) + c2) + 2.0) / 2.0;
        },
        .BounceIn => return 1.0 - bounce_out(1.0 - t),
        .BounceOut => return bounce_out(t),
        .BounceInOut => {
            if (t < 0.5) return (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0;
            return (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0;
        },
        .SmoothStepIn => return 2.0 * t * t - t * t * t * t,
        .SmoothStep => return t * t * (3.0 - 2.0 * t),
        .SmoothStepOut => return t * (2.0 - t),
        .SmootherStep => return t * t * t * (t * (t * 6.0 - 15.0) + 10.0),
        .SmootherStepIn => return t * t * t,
        .SmootherStepOut => return 1.0,
    }
}

fn degrees_cos(radians: f32) f32 {
    return @cos(radians);
}

fn bounce_out(t: f32) f32 {
    const n1 = 7.5625;
    const d1 = 2.75;
    if (t < 1.0 / d1) {
        return n1 * t * t;
    } else if (t < 2.0 / d1) {
        const t2 = t - 1.5 / d1;
        return n1 * t2 * t2 + 0.75;
    } else if (t < 2.5 / d1) {
        const t2 = t - 2.25 / d1;
        return n1 * t2 * t2 + 0.9375;
    } else {
        const t2 = t - 2.625 / d1;
        return n1 * t2 * t2 + 0.984375;
    }
}

export fn ease_sample(ease: EaseFunction, t: f32) f32 {
    return sample_ease(ease, t);
}
