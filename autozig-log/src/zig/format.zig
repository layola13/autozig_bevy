const std = @import("std");

/// Format buffer size
pub const FORMAT_BUFFER_SIZE: usize = 4096;

/// Color codes for terminal output
pub const Color = enum {
    reset,
    black,
    red,
    green,
    yellow,
    blue,
    magenta,
    cyan,
    white,
    bright_black,
    bright_red,
    bright_green,
    bright_yellow,
    bright_blue,
    bright_magenta,
    bright_cyan,
    bright_white,

    pub fn code(self: Color) []const u8 {
        return switch (self) {
            .reset => "\x1b[0m",
            .black => "\x1b[30m",
            .red => "\x1b[31m",
            .green => "\x1b[32m",
            .yellow => "\x1b[33m",
            .blue => "\x1b[34m",
            .magenta => "\x1b[35m",
            .cyan => "\x1b[36m",
            .white => "\x1b[37m",
            .bright_black => "\x1b[90m",
            .bright_red => "\x1b[91m",
            .bright_green => "\x1b[92m",
            .bright_yellow => "\x1b[93m",
            .bright_blue => "\x1b[94m",
            .bright_magenta => "\x1b[95m",
            .bright_cyan => "\x1b[96m",
            .bright_white => "\x1b[97m",
        };
    }
};

/// Format style for log output
pub const FormatStyle = struct {
    use_colors: bool,
    use_timestamp: bool,
    use_module: bool,
    use_level: bool,
};

/// Default format style
pub const default_style = FormatStyle{
    .use_colors = false, // WASM doesn't support terminal colors
    .use_timestamp = true,
    .use_module = true,
    .use_level = true,
};

/// Format a log level with optional color
pub fn formatLevel(level: anytype, use_color: bool, buf: []u8) ![]const u8 {
    const level_str = level.toString();

    if (use_color) {
        const color = switch (level) {
            .trace => Color.bright_black,
            .debug => Color.cyan,
            .info => Color.green,
            .warn => Color.yellow,
            .err => Color.red,
        };

        return std.fmt.bufPrint(
            buf,
            "{s}{s}{s}",
            .{ color.code(), level_str, Color.reset.code() },
        );
    }

    return std.fmt.bufPrint(buf, "{s}", .{level_str});
}

/// Parse log level from string
pub fn parseLevelFromString(str: []const u8) ?@import("logger.zig").LogLevel {
    const LogLevel = @import("logger.zig").LogLevel;

    if (std.mem.eql(u8, str, "trace") or std.mem.eql(u8, str, "TRACE")) {
        return LogLevel.trace;
    } else if (std.mem.eql(u8, str, "debug") or std.mem.eql(u8, str, "DEBUG")) {
        return LogLevel.debug;
    } else if (std.mem.eql(u8, str, "info") or std.mem.eql(u8, str, "INFO")) {
        return LogLevel.info;
    } else if (std.mem.eql(u8, str, "warn") or std.mem.eql(u8, str, "WARN")) {
        return LogLevel.warn;
    } else if (std.mem.eql(u8, str, "error") or std.mem.eql(u8, str, "ERROR") or std.mem.eql(u8, str, "err") or std.mem.eql(u8, str, "ERR")) {
        return LogLevel.err;
    }

    return null;
}

/// Exported function to parse level from string
export fn format_parse_level(str_ptr: [*]const u8, str_len: usize) i32 {
    const str = str_ptr[0..str_len];
    const level = parseLevelFromString(str) orelse return -1;
    return @as(i32, @intCast(@intFromEnum(level)));
}
