use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AspectRatio {
    pub ratio: f32,
}

include_zig!("zig/aspect_ratio.zig", {
    fn aspect_ratio_new(width: f32, height: f32) -> AspectRatio;
    fn aspect_ratio_width(self_: AspectRatio, height: f32) -> f32;
    fn aspect_ratio_height(self_: AspectRatio, width: f32) -> f32;
});

impl AspectRatio {
    pub fn new(width: f32, height: f32) -> Self { aspect_ratio_new(width, height) }
    pub fn width(&self, height: f32) -> f32 { aspect_ratio_width(*self, height) }
    pub fn height(&self, width: f32) -> f32 { aspect_ratio_height(*self, width) }
}
