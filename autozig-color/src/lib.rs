use autozig::include_zig;

// Color type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// Hsla type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

// Hsva type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsva {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

// LinearRgba type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// Include the unified Zig module
include_zig!("zig/color_all.zig", {
    fn color_rgb(r: f32, g: f32, b: f32) -> Color;
    fn color_rgba(r: f32, g: f32, b: f32, a: f32) -> Color;
    fn color_with_alpha(color: Color, alpha: f32) -> Color;
    fn color_lerp(a: Color, b: Color, t: f32) -> Color;
    fn color_mix(a: Color, b: Color, weight: f32) -> Color;
    fn hsla_init(h: f32, s: f32, l: f32, a: f32) -> Hsla;
    fn hsla_to_rgba(hsla: Hsla) -> Color;
    fn hsla_from_rgba(color: Color) -> Hsla;
    fn hsva_init(h: f32, s: f32, v: f32, a: f32) -> Hsva;
    fn hsva_to_rgba(hsva: Hsva) -> Color;
    fn hsva_from_rgba(color: Color) -> Hsva;
    fn linear_rgba_init(r: f32, g: f32, b: f32, a: f32) -> LinearRgba;
    fn linear_rgba_from_rgba(color: Color) -> LinearRgba;
    fn linear_rgba_to_rgba(linear: LinearRgba) -> Color;
    fn linear_rgba_lerp(a: LinearRgba, b: LinearRgba, t: f32) -> LinearRgba;
    fn color_lighten(color: Color, amount: f32) -> Color;
    fn color_darken(color: Color, amount: f32) -> Color;
    fn color_saturate(color: Color, amount: f32) -> Color;
    fn color_desaturate(color: Color, amount: f32) -> Color;
    fn color_white() -> Color;
    fn color_black() -> Color;
    fn color_red() -> Color;
    fn color_green() -> Color;
    fn color_blue() -> Color;
    fn color_yellow() -> Color;
    fn color_cyan() -> Color;
    fn color_magenta() -> Color;
    fn color_gray() -> Color;
    fn color_orange() -> Color;
    fn color_purple() -> Color;
    fn color_pink() -> Color;
    fn color_transparent() -> Color;
});

impl Color {
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const GRAY: Self = Self { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        color_rgb(r, g, b)
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        color_rgba(r, g, b, a)
    }

    pub fn hex(hex_str: &str) -> Result<Self, &'static str> {
        if hex_str.len() < 6 {
            return Err("Invalid hex string");
        }

        let start = if hex_str.starts_with('#') { 1 } else { 0 };
        let hex_bytes = &hex_str[start..];

        if hex_bytes.len() < 6 {
            return Err("Invalid hex string");
        }

        let r = u8::from_str_radix(&hex_bytes[0..2], 16).map_err(|_| "Invalid hex")?;
        let g = u8::from_str_radix(&hex_bytes[2..4], 16).map_err(|_| "Invalid hex")?;
        let b = u8::from_str_radix(&hex_bytes[4..6], 16).map_err(|_| "Invalid hex")?;
        
        let a = if hex_bytes.len() >= 8 {
            u8::from_str_radix(&hex_bytes[6..8], 16).map_err(|_| "Invalid hex")?
        } else {
            255
        };

        Ok(Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        })
    }

    pub fn with_alpha(self, alpha: f32) -> Self {
        color_with_alpha(self, alpha)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        color_lerp(self, other, t)
    }

    pub fn mix(self, other: Self, weight: f32) -> Self {
        color_mix(self, other, weight)
    }

    pub fn lighten(self, amount: f32) -> Self {
        color_lighten(self, amount)
    }

    pub fn darken(self, amount: f32) -> Self {
        color_darken(self, amount)
    }

    pub fn saturate(self, amount: f32) -> Self {
        color_saturate(self, amount)
    }

    pub fn desaturate(self, amount: f32) -> Self {
        color_desaturate(self, amount)
    }
}

impl Hsla {
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        hsla_init(h, s, l, a)
    }

    pub fn to_rgba(self) -> Color {
        hsla_to_rgba(self)
    }

    pub fn from_rgba(color: Color) -> Self {
        hsla_from_rgba(color)
    }
}

impl Hsva {
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        hsva_init(h, s, v, a)
    }

    pub fn to_rgba(self) -> Color {
        hsva_to_rgba(self)
    }

    pub fn from_rgba(color: Color) -> Self {
        hsva_from_rgba(color)
    }
}

impl LinearRgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        linear_rgba_init(r, g, b, a)
    }

    pub fn from_rgba(color: Color) -> Self {
        linear_rgba_from_rgba(color)
    }

    pub fn to_rgba(self) -> Color {
        linear_rgba_to_rgba(self)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        linear_rgba_lerp(self, other, t)
    }
}