const Color = @import("color.zig").Color;

// Basic colors
pub const WHITE = Color.rgb(1.0, 1.0, 1.0);
pub const BLACK = Color.rgb(0.0, 0.0, 0.0);
pub const RED = Color.rgb(1.0, 0.0, 0.0);
pub const GREEN = Color.rgb(0.0, 1.0, 0.0);
pub const BLUE = Color.rgb(0.0, 0.0, 1.0);
pub const YELLOW = Color.rgb(1.0, 1.0, 0.0);
pub const CYAN = Color.rgb(0.0, 1.0, 1.0);
pub const MAGENTA = Color.rgb(1.0, 0.0, 1.0);

// Grayscale
pub const GRAY = Color.rgb(0.5, 0.5, 0.5);
pub const DARK_GRAY = Color.rgb(0.25, 0.25, 0.25);
pub const LIGHT_GRAY = Color.rgb(0.75, 0.75, 0.75);

// Extended colors
pub const ORANGE = Color.rgb(1.0, 0.5, 0.0);
pub const PURPLE = Color.rgb(0.5, 0.0, 0.5);
pub const PINK = Color.rgb(1.0, 0.75, 0.8);
pub const BROWN = Color.rgb(0.6, 0.4, 0.2);

// Special
pub const TRANSPARENT = Color.rgba(0.0, 0.0, 0.0, 0.0);

// FFI exports for standard colors
export fn color_white() Color {
    return WHITE;
}

export fn color_black() Color {
    return BLACK;
}

export fn color_red() Color {
    return RED;
}

export fn color_green() Color {
    return GREEN;
}

export fn color_blue() Color {
    return BLUE;
}

export fn color_yellow() Color {
    return YELLOW;
}

export fn color_cyan() Color {
    return CYAN;
}

export fn color_magenta() Color {
    return MAGENTA;
}

export fn color_gray() Color {
    return GRAY;
}

export fn color_orange() Color {
    return ORANGE;
}

export fn color_purple() Color {
    return PURPLE;
}

export fn color_pink() Color {
    return PINK;
}

export fn color_transparent() Color {
    return TRANSPARENT;
}
