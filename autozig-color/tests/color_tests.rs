use autozig_color::{Color, Hsla, Hsva, LinearRgba};

#[test]
fn test_color_creation() {
    let c1 = Color::rgb(1.0, 0.0, 0.0);
    assert_eq!(c1.r, 1.0);
    assert_eq!(c1.g, 0.0);
    assert_eq!(c1.b, 0.0);
    assert_eq!(c1.a, 1.0);

    let c2 = Color::rgba(0.5, 0.5, 0.5, 0.5);
    assert_eq!(c2.r, 0.5);
    assert_eq!(c2.a, 0.5);
}

#[test]
fn test_rgb_to_hsl() {
    let red = Color::RED;
    let hsl = Hsla::from_rgba(red);
    
    assert!((hsl.h - 0.0).abs() < 1e-5);
    assert!((hsl.s - 1.0).abs() < 1e-5);
    assert!((hsl.l - 0.5).abs() < 1e-5);
    assert_eq!(hsl.a, 1.0);
}

#[test]
fn test_hsl_to_rgb() {
    let hsl = Hsla::new(0.0, 1.0, 0.5, 1.0);
    let color = hsl.to_rgba();
    
    assert!((color.r - 1.0).abs() < 1e-5);
    assert!((color.g - 0.0).abs() < 1e-5);
    assert!((color.b - 0.0).abs() < 1e-5);
    assert_eq!(color.a, 1.0);
}

#[test]
fn test_rgb_to_hsv() {
    let red = Color::RED;
    let hsv = Hsva::from_rgba(red);
    
    assert!((hsv.h - 0.0).abs() < 1e-5);
    assert!((hsv.s - 1.0).abs() < 1e-5);
    assert!((hsv.v - 1.0).abs() < 1e-5);
    assert_eq!(hsv.a, 1.0);
}

#[test]
fn test_hsv_to_rgb() {
    let hsv = Hsva::new(120.0, 1.0, 1.0, 1.0);
    let color = hsv.to_rgba();
    
    assert!((color.r - 0.0).abs() < 1e-5);
    assert!((color.g - 1.0).abs() < 1e-5);
    assert!((color.b - 0.0).abs() < 1e-5);
    assert_eq!(color.a, 1.0);
}

#[test]
fn test_color_lerp() {
    let c1 = Color::BLACK;
    let c2 = Color::WHITE;
    let mid = c1.lerp(c2, 0.5);
    
    assert!((mid.r - 0.5).abs() < 1e-5);
    assert!((mid.g - 0.5).abs() < 1e-5);
    assert!((mid.b - 0.5).abs() < 1e-5);
    assert_eq!(mid.a, 1.0);
}

#[test]
fn test_color_with_alpha() {
    let c = Color::RED;
    let transparent = c.with_alpha(0.5);
    
    assert_eq!(transparent.r, 1.0);
    assert_eq!(transparent.g, 0.0);
    assert_eq!(transparent.b, 0.0);
    assert_eq!(transparent.a, 0.5);
}

#[test]
fn test_color_lighten_darken() {
    let gray = Color::GRAY;
    
    let lighter = gray.lighten(0.2);
    let hsl_lighter = Hsla::from_rgba(lighter);
    let hsl_gray = Hsla::from_rgba(gray);
    assert!(hsl_lighter.l > hsl_gray.l);
    
    let darker = gray.darken(0.2);
    let hsl_darker = Hsla::from_rgba(darker);
    assert!(hsl_darker.l < hsl_gray.l);
}

#[test]
fn test_color_hex_parsing() {
    let red_hex = Color::hex("#FF0000").unwrap();
    assert!((red_hex.r - 1.0).abs() < 0.01);
    assert!((red_hex.g - 0.0).abs() < 0.01);
    assert!((red_hex.b - 0.0).abs() < 0.01);
    assert_eq!(red_hex.a, 1.0);

    let green_hex = Color::hex("00FF00").unwrap();
    assert!((green_hex.g - 1.0).abs() < 0.01);

    let semi_blue = Color::hex("#0000FF80").unwrap();
    assert!((semi_blue.b - 1.0).abs() < 0.01);
    assert!((semi_blue.a - 0.5).abs() < 0.01);
}

#[test]
fn test_standard_colors() {
    assert_eq!(Color::WHITE.r, 1.0);
    assert_eq!(Color::WHITE.g, 1.0);
    assert_eq!(Color::WHITE.b, 1.0);
    
    assert_eq!(Color::BLACK.r, 0.0);
    assert_eq!(Color::BLACK.g, 0.0);
    assert_eq!(Color::BLACK.b, 0.0);
    
    assert_eq!(Color::RED.r, 1.0);
    assert_eq!(Color::RED.g, 0.0);
    
    assert_eq!(Color::GREEN.g, 1.0);
    assert_eq!(Color::BLUE.b, 1.0);
    
    assert_eq!(Color::TRANSPARENT.a, 0.0);
}

#[test]
fn test_linear_rgba() {
    let srgb = Color::rgb(0.5, 0.5, 0.5);
    let linear = LinearRgba::from_rgba(srgb);
    
    // sRGB 0.5 should convert to roughly 0.214 in linear space
    assert!(linear.r > 0.2 && linear.r < 0.25);
    
    let back_to_srgb = linear.to_rgba();
    assert!((back_to_srgb.r - srgb.r).abs() < 0.01);
}

#[test]
fn test_linear_rgba_lerp() {
    let l1 = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
    let l2 = LinearRgba::new(1.0, 1.0, 1.0, 1.0);
    let mid = l1.lerp(l2, 0.5);
    
    assert!((mid.r - 0.5).abs() < 1e-5);
    assert!((mid.g - 0.5).abs() < 1e-5);
    assert!((mid.b - 0.5).abs() < 1e-5);
}

#[test]
fn test_color_saturate_desaturate() {
    let color = Color::rgb(0.8, 0.5, 0.5);
    
    let saturated = color.saturate(0.2);
    let hsl_sat = Hsla::from_rgba(saturated);
    let hsl_orig = Hsla::from_rgba(color);
    assert!(hsl_sat.s > hsl_orig.s);
    
    let desaturated = color.desaturate(0.2);
    let hsl_desat = Hsla::from_rgba(desaturated);
    assert!(hsl_desat.s < hsl_orig.s);
}

#[test]
fn test_color_mix() {
    let red = Color::RED;
    let blue = Color::BLUE;
    let purple = red.mix(blue, 0.5);
    
    assert!((purple.r - 0.5).abs() < 1e-5);
    assert!((purple.g - 0.0).abs() < 1e-5);
    assert!((purple.b - 0.5).abs() < 1e-5);
}

#[test]
fn test_hsla_roundtrip() {
    let original = Color::rgb(0.7, 0.3, 0.9);
    let hsl = Hsla::from_rgba(original);
    let back = hsl.to_rgba();
    
    assert!((back.r - original.r).abs() < 1e-5);
    assert!((back.g - original.g).abs() < 1e-5);
    assert!((back.b - original.b).abs() < 1e-5);
}

#[test]
fn test_hsva_roundtrip() {
    let original = Color::rgb(0.2, 0.8, 0.4);
    let hsv = Hsva::from_rgba(original);
    let back = hsv.to_rgba();
    
    assert!((back.r - original.r).abs() < 1e-5);
    assert!((back.g - original.g).abs() < 1e-5);
    assert!((back.b - original.b).abs() < 1e-5);
}