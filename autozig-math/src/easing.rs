use autozig::include_zig;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EaseFunction {
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
}

include_zig!("zig/easing.zig", {
    fn ease_sample(ease: EaseFunction, t: f32) -> f32;
});

impl EaseFunction {
    pub fn sample(&self, t: f32) -> f32 { ease_sample(*self, t) }
}
