use autozig::include_zig;

// ============================================================================
// Core Color Space Structures
// ============================================================================

/// Standard RGB color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

/// Linear RGB color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

/// HSL color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32,
}

/// HSV color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsva {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub alpha: f32,
}

/// HWB color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hwba {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
    pub alpha: f32,
}

/// CIE Lab color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Laba {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}

/// LCH color space with alpha (cylindrical Lab)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lcha {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}

/// Oklab color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklaba {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}

/// Oklch color space with alpha (cylindrical Oklab)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklcha {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}

/// CIE XYZ color space with alpha
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xyza {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub alpha: f32,
}

// ============================================================================
// Color Enum - Core Type Supporting All Color Spaces
// ============================================================================

/// Main color type supporting all color spaces
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Srgba(Srgba),
    LinearRgba(LinearRgba),
    Hsla(Hsla),
    Hsva(Hsva),
    Hwba(Hwba),
    Laba(Laba),
    Lcha(Lcha),
    Oklaba(Oklaba),
    Oklcha(Oklcha),
    Xyza(Xyza),
}

/// Hex color parsing errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexColorError {
    Length,
    Char(char),
}

// ============================================================================
// Additional Types
// ============================================================================

/// Color curve for animation
#[derive(Debug, Clone)]
pub struct ColorCurve {
    pub start: Color,
    pub end: Color,
}

/// Test color utility
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestColor {
    pub value: Color,
}

// ============================================================================
// Traits
// ============================================================================

/// Alpha channel operations
pub trait Alpha {
    fn with_alpha(&self, alpha: f32) -> Self;
    fn alpha(&self) -> f32;
    fn set_alpha(&mut self, alpha: f32);
}

/// Hue operations
pub trait Hue {
    fn with_hue(&self, hue: f32) -> Self;
    fn hue(&self) -> f32;
    fn set_hue(&mut self, hue: f32);
}

/// Luminance operations
pub trait Luminance {
    fn with_luminance(&self, luminance: f32) -> Self;
    fn luminance(&self) -> f32;
}

/// Saturation operations  
pub trait Saturation {
    fn with_saturation(&self, saturation: f32) -> Self;
    fn saturation(&self) -> f32;
}

/// Grayscale operations
pub trait Gray {
    fn to_gray(&self) -> Self;
}

/// Color mixing
pub trait Mix {
    fn mix(&self, other: &Self, factor: f32) -> Self;
}

/// Euclidean distance in color space
pub trait EuclideanDistance {
    fn distance(&self, other: &Self) -> f32;
}

/// Color to components conversion
pub trait ColorToComponents {
    fn to_f32_array(&self) -> [f32; 4];
    fn to_f32_array_no_alpha(&self) -> [f32; 3];
    fn to_vec4(&self) -> [f32; 4] {
        self.to_f32_array()
    }
}

/// Color to packed format
pub trait ColorToPacked {
    fn to_u32(&self) -> u32;
    fn to_u8_array(&self) -> [u8; 4];
}

/// Color range operations
pub trait ColorRange {
    fn clamp(&self) -> Self;
    fn is_within_bounds(&self) -> bool;
}

// ============================================================================
// Include Zig FFI Bindings
// ============================================================================

include_zig!("zig/color_all.zig", {
    // Srgba functions
    fn srgba_new(red: f32, green: f32, blue: f32, alpha: f32) -> Srgba;
    fn srgba_rgb(red: f32, green: f32, blue: f32) -> Srgba;
    
    // LinearRgba functions
    fn linear_rgba_init(r: f32, g: f32, b: f32, a: f32) -> LinearRgba;
    fn linear_rgba_from_rgba(color: Srgba) -> LinearRgba;
    fn linear_rgba_to_rgba(linear: LinearRgba) -> Srgba;
    fn linear_rgba_lerp(a: LinearRgba, b: LinearRgba, t: f32) -> LinearRgba;
    
    // Hsla functions
    fn hsla_init(h: f32, s: f32, l: f32, a: f32) -> Hsla;
    fn hsla_to_srgba(hsla: Hsla) -> Srgba;
    fn hsla_from_srgba(color: Srgba) -> Hsla;
    
    // Hsva functions
    fn hsva_init(h: f32, s: f32, v: f32, a: f32) -> Hsva;
    fn hsva_to_srgba(hsva: Hsva) -> Srgba;
    fn hsva_from_srgba(color: Srgba) -> Hsva;
    
    // Hwba functions
    fn hwba_new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) -> Hwba;
    
    // Laba functions
    fn laba_new(lightness: f32, a: f32, b: f32, alpha: f32) -> Laba;
    fn laba_delta_e(a: Laba, b: Laba) -> f32;
    
    // Lcha functions
    fn lcha_new(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Lcha;
    
    // Oklaba functions
    fn oklaba_new(lightness: f32, a: f32, b: f32, alpha: f32) -> Oklaba;
    
    // Oklcha functions
    fn oklcha_new(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Oklcha;
    
    // Xyza functions
    fn xyza_new(x: f32, y: f32, z: f32, alpha: f32) -> Xyza;
});

// ============================================================================
// Color Enum Implementation
// ============================================================================

impl Color {
    // Standard color constants
    pub const WHITE: Color = Color::Srgba(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 });
    pub const BLACK: Color = Color::Srgba(Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 });
    pub const RED: Color = Color::Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 });
    pub const GREEN: Color = Color::Srgba(Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 });
    pub const BLUE: Color = Color::Srgba(Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 });
    pub const YELLOW: Color = Color::Srgba(Srgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 });
    pub const CYAN: Color = Color::Srgba(Srgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 });
    pub const MAGENTA: Color = Color::Srgba(Srgba { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 });
    pub const TRANSPARENT: Color = Color::Srgba(Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 });
    pub const ORANGE: Color = Color::Srgba(Srgba { red: 1.0, green: 0.647, blue: 0.0, alpha: 1.0 });
    pub const ORANGE_RED: Color = Color::Srgba(Srgba { red: 1.0, green: 0.27, blue: 0.0, alpha: 1.0 });
    pub const NONE: Color = Color::TRANSPARENT;


    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color::Srgba(srgba_rgb(r, g, b))
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::Srgba(srgba_new(r, g, b, a))
    }
    
    pub fn srgb(r: f32, g: f32, b: f32) -> Self {
        Color::Srgba(srgba_rgb(r, g, b))
    }

    pub fn srgb_u8(r: u8, g: u8, b: u8) -> Self {
         Color::Srgba(Srgba {
            red: r as f32 / 255.0,
            green: g as f32 / 255.0,
            blue: b as f32 / 255.0,
            alpha: 1.0,
        })
    }

    pub fn hex(hex: &str) -> Result<Self, HexColorError> {
        Srgba::hex(hex).map(Color::Srgba)
    }

    pub fn to_srgba(&self) -> Srgba {
        match self {
            Color::Srgba(c) => *c,
            Color::LinearRgba(c) => linear_rgba_to_rgba(*c),
            Color::Hsla(c) => hsla_to_srgba(*c),
            Color::Hsva(c) => hsva_to_srgba(*c),
            _ => Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }, // Placeholder
        }
    }

    pub fn to_linear(&self) -> LinearRgba {
        match self {
            Color::LinearRgba(c) => *c,
            Color::Srgba(c) => linear_rgba_from_rgba(*c),
            _ => linear_rgba_from_rgba(self.to_srgba()),
        }
    }
}

impl From<Srgba> for Color {
    fn from(c: Srgba) -> Self {
        Color::Srgba(c)
    }
}

impl From<LinearRgba> for Color {
    fn from(c: LinearRgba) -> Self {
        Color::LinearRgba(c)
    }
}

// ============================================================================
// Srgba Implementation
// ============================================================================

impl Srgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        srgba_new(red, green, blue, alpha)
    }

    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        srgba_rgb(red, green, blue)
    }

    pub fn hex(hex: &str) -> Result<Self, HexColorError> {
        if hex.len() < 6 {
            return Err(HexColorError::Length);
        }

        let start = if hex.starts_with('#') { 1 } else { 0 };
        let hex_bytes = &hex[start..];

        if hex_bytes.len() < 6 {
            return Err(HexColorError::Length);
        }

        let parse_hex = |s: &str| {
            u8::from_str_radix(s, 16).map_err(|_| HexColorError::Char(s.chars().next().unwrap_or('?')))
        };

        let r = parse_hex(&hex_bytes[0..2])?;
        let g = parse_hex(&hex_bytes[2..4])?;
        let b = parse_hex(&hex_bytes[4..6])?;
        let a = if hex_bytes.len() >= 8 {
            parse_hex(&hex_bytes[6..8])?
        } else {
            255
        };

        Ok(Srgba {
            red: r as f32 / 255.0,
            green: g as f32 / 255.0,
            blue: b as f32 / 255.0,
            alpha: a as f32 / 255.0,
        })
    }

    pub fn to_linear(&self) -> LinearRgba {
        linear_rgba_from_rgba(*self)
    }
}

impl LinearRgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        linear_rgba_init(red, green, blue, alpha)
    }

    pub fn to_srgba(&self) -> Srgba {
        linear_rgba_to_rgba(*self)
    }

    pub fn lerp(&self, other: Self, t: f32) -> Self {
        linear_rgba_lerp(*self, other, t)
    }
}

impl Hsla {
    pub fn new(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        hsla_init(hue, saturation, lightness, alpha)
    }

    pub fn to_srgba(&self) -> Srgba {
        hsla_to_srgba(*self)
    }

    pub fn from_srgba(color: Srgba) -> Self {
        hsla_from_srgba(color)
    }
}

impl Hsva {
    pub fn new(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        hsva_init(hue, saturation, value, alpha)
    }

    pub fn to_srgba(&self) -> Srgba {
        hsva_to_srgba(*self)
    }

    pub fn from_srgba(color: Srgba) -> Self {
        hsva_from_srgba(color)
    }
}

impl Hwba {
    pub fn new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) -> Self {
        hwba_new(hue, whiteness, blackness, alpha)
    }
}

impl Laba {
    pub fn new(lightness: f32, a: f32, b: f32, alpha: f32) -> Self {
        laba_new(lightness, a, b, alpha)
    }

    pub fn distance(&self, other: &Self) -> f32 {
        laba_delta_e(*self, *other)
    }
}

impl Lcha {
    pub fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Self {
        lcha_new(lightness, chroma, hue, alpha)
    }
}

impl Oklaba {
    pub fn new(lightness: f32, a: f32, b: f32, alpha: f32) -> Self {
        oklaba_new(lightness, a, b, alpha)
    }
}

impl Oklcha {
    pub fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Self {
        oklcha_new(lightness, chroma, hue, alpha)
    }
}

impl Xyza {
    pub fn new(x: f32, y: f32, z: f32, alpha: f32) -> Self {
        xyza_new(x, y, z, alpha)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl Alpha for Color {
    fn with_alpha(&self, alpha: f32) -> Self {
        match self {
            Color::Srgba(c) => Color::Srgba(Srgba { alpha, ..*c }),
            Color::LinearRgba(c) => Color::LinearRgba(LinearRgba { alpha, ..*c }),
            Color::Hsla(c) => Color::Hsla(Hsla { alpha, ..*c }),
            Color::Hsva(c) => Color::Hsva(Hsva { alpha, ..*c }),
            Color::Hwba(c) => Color::Hwba(Hwba { alpha, ..*c }),
            Color::Laba(c) => Color::Laba(Laba { alpha, ..*c }),
            Color::Lcha(c) => Color::Lcha(Lcha { alpha, ..*c }),
            Color::Oklaba(c) => Color::Oklaba(Oklaba { alpha, ..*c }),
            Color::Oklcha(c) => Color::Oklcha(Oklcha { alpha, ..*c }),
            Color::Xyza(c) => Color::Xyza(Xyza { alpha, ..*c }),
        }
    }

    fn alpha(&self) -> f32 {
        match self {
            Color::Srgba(c) => c.alpha,
            Color::LinearRgba(c) => c.alpha,
            Color::Hsla(c) => c.alpha,
            Color::Hsva(c) => c.alpha,
            Color::Hwba(c) => c.alpha,
            Color::Laba(c) => c.alpha,
            Color::Lcha(c) => c.alpha,
            Color::Oklaba(c) => c.alpha,
            Color::Oklcha(c) => c.alpha,
            Color::Xyza(c) => c.alpha,
        }
    }

    fn set_alpha(&mut self, alpha: f32) {
        *self = self.with_alpha(alpha);
    }
}

impl Mix for LinearRgba {
    fn mix(&self, other: &Self, factor: f32) -> Self {
        self.lerp(*other, factor)
    }
}

impl ColorToComponents for Srgba {
    fn to_f32_array(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    fn to_f32_array_no_alpha(&self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }
}

impl ColorToComponents for LinearRgba {
    fn to_f32_array(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    fn to_f32_array_no_alpha(&self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }
}

impl ColorToPacked for Srgba {
    fn to_u32(&self) -> u32 {
        let r = (self.red.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.green.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.blue.clamp(0.0, 1.0) * 255.0) as u32;
        let a = (self.alpha.clamp(0.0, 1.0) * 255.0) as u32;
        (a << 24) | (b << 16) | (g << 8) | r
    }

    fn to_u8_array(&self) -> [u8; 4] {
        [
            (self.red.clamp(0.0, 1.0) * 255.0) as u8,
            (self.green.clamp(0.0, 1.0) * 255.0) as u8,
            (self.blue.clamp(0.0, 1.0) * 255.0) as u8,
            (self.alpha.clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }
}

impl EuclideanDistance for LinearRgba {
    fn distance(&self, other: &Self) -> f32 {
        let dr = self.red - other.red;
        let dg = self.green - other.green;
        let db = self.blue - other.blue;
        (dr * dr + dg * dg + db * db).sqrt()
    }
}

impl EuclideanDistance for Laba {
    fn distance(&self, other: &Self) -> f32 {
        self.distance(other)
    }
}

impl ColorCurve {
    pub fn new(start: Color, end: Color) -> Self {
        Self { start, end }
    }

    pub fn sample(&self, t: f32) -> Color {
        // Simple linear interpolation between colors
        let start_linear = self.start.to_linear();
        let end_linear = self.end.to_linear();
        Color::LinearRgba(start_linear.lerp(end_linear, t))
    }
}

impl TestColor {
    pub fn new(value: Color) -> Self {
        Self { value }
    }
}