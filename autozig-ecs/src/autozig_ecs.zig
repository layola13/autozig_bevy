const std = @import("std");

// Force compilation of all modules to emit exported symbols
comptime {
    // Standard modules in src/zig
    _ = @import("zig/world.zig");
    _ = @import("zig/entity.zig");
    _ = @import("zig/component.zig");
    _ = @import("zig/bundle.zig");
    _ = @import("zig/query.zig");
    _ = @import("zig/system.zig");
    _ = @import("zig/event.zig");
    _ = @import("zig/archetype.zig");
    _ = @import("zig/change_detection.zig");
    _ = @import("zig/removed_components.zig");
    _ = @import("zig/resource.zig");
    _ = @import("zig/table.zig");
    _ = @import("zig/entity_sparse_set.zig");
    _ = @import("zig/command.zig");
    _ = @import("zig/plugin.zig");
    _ = @import("zig/system_closure.zig");

    // New query modules
    _ = @import("query/fetch/zig/fetch.zig");
    _ = @import("query/filter/zig/filter.zig");
    _ = @import("query/builder/zig/query_builder.zig");
    _ = @import("world/zig/world_id.zig");
    _ = @import("world/zig/spawn_batch.zig");
}
