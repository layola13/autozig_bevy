// Force all modules to be compiled by comptime referencing them
// This ensures all exported functions are included in the final library
comptime {
    // Reference all modules to force compilation
    _ = @import("app.zig");
    _ = @import("plugin.zig");
    _ = @import("plugin_group.zig");
    _ = @import("runner.zig");
    _ = @import("schedule.zig");
    _ = @import("sub_app.zig");
}
