const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const change_detection = @import("change_detection.zig");
const Tick = change_detection.Tick;
const g_allocator = common.g_allocator;

const Archetype = @import("archetype.zig").Archetype;
const Table = @import("table.zig").Table;

pub const EntityMeta = struct {
    generation: u32,
    is_alive: bool,
    archetype_id: u32,
    row: u32,
};

pub const World = struct {
    archetypes: std.ArrayList(Archetype),
    tables: std.ArrayList(Table),
    entities: std.ArrayList(EntityMeta),
    free_list: std.ArrayList(u32),
    next_entity_index: u32,
    current_tick: Tick,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !*World {
        const self = try allocator.create(World);
        self.* = World{
            .archetypes = std.ArrayList(Archetype){},
            .tables = std.ArrayList(Table){},
            .entities = std.ArrayList(EntityMeta){},
            .free_list = std.ArrayList(u32){},
            .next_entity_index = 0,
            .current_tick = Tick.new(0),
            .allocator = allocator,
        };

        const empty_arch = Archetype.init(allocator, 0);
        try self.archetypes.append(allocator, empty_arch);

        const empty_table = Table.init(allocator);
        try self.tables.append(allocator, empty_table);

        return self;
    }

    pub fn deinit(self: *World) void {
        for (self.archetypes.items) |*arch| arch.deinit();
        self.archetypes.deinit(self.allocator);
        for (self.tables.items) |*table| table.deinit();
        self.tables.deinit(self.allocator);
        self.entities.deinit(self.allocator);
        self.free_list.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn spawnEmpty(self: *World) !Entity {
        var idx: u32 = 0;
        var generation: u32 = 0;

        if (self.free_list.items.len > 0) {
            idx = self.free_list.pop().?;
            const meta = &self.entities.items[idx];
            meta.is_alive = true;
            generation = meta.generation;
        } else {
            idx = self.next_entity_index;
            self.next_entity_index += 1;
            try self.entities.append(self.allocator, .{
                .generation = 0,
                .is_alive = true,
                .archetype_id = 0,
                .row = 0,
            });
            generation = 0;
        }

        return Entity{ .index = idx, .generation = generation };
    }

    pub fn insertComponents(
        self: *World,
        entity: Entity,
        component_ids: [*]const u32,
        component_sizes: [*]const usize,
        component_data_ptrs: [*]const [*]const u8,
        count: usize,
    ) !bool {
        if (entity.index >= self.entities.items.len) return false;
        var meta = &self.entities.items[entity.index];
        if (!meta.is_alive or meta.generation != entity.generation) return false;

        // Find or create target archetype
        var target_arch_id: ?u32 = null;
        for (component_ids[0..count]) |id| {
            std.debug.print("world_insert_components: count={}, id={}\n", .{ count, id });
        }
        for (self.archetypes.items) |*arch| {
            if (arch.table_components.items.len == count) {
                var match = true;
                for (component_ids[0..count]) |id| {
                    if (!arch.hasTableComponent(id)) {
                        match = false;
                        break;
                    }
                }
                if (match) {
                    target_arch_id = arch.id;
                    std.debug.print("world_insert_components: found arch={}\n", .{target_arch_id.?});
                    break;
                }
            }
        }

        if (target_arch_id == null) {
            const new_id = @as(u32, @intCast(self.archetypes.items.len));
            std.debug.print("world_insert_components: creating new arch={} for count={}\n", .{ new_id, count });
            var new_arch = Archetype.init(self.allocator, new_id);
            var new_table = Table.init(self.allocator);

            for (0..count) |i| {
                try new_arch.addTableComponent(component_ids[i]);
                try new_table.addColumn(component_ids[i], component_sizes[i]);
            }

            try self.archetypes.append(self.allocator, new_arch);
            try self.tables.append(self.allocator, new_table);
            target_arch_id = new_id;
        }

        const new_arch_id = target_arch_id.?;
        if (new_arch_id == meta.archetype_id) {
            const table_id = self.archetypes.items[new_arch_id].table_id;
            var table = &self.tables.items[table_id];
            for (0..count) |i| {
                const col = table.getColumn(component_ids[i]).?;
                const ptr = col.getPtr(meta.row).?;
                @memcpy(ptr[0..component_sizes[i]], component_data_ptrs[i][0..component_sizes[i]]);
                col.ticks.items[meta.row] = change_detection.ComponentTicks.new(self.current_tick);
            }
            return true;
        }

        // Move entity
        const old_arch_id = meta.archetype_id;
        var old_arch = &self.archetypes.items[old_arch_id];
        _ = old_arch.removeEntity(entity.index);

        var new_arch = &self.archetypes.items[new_arch_id];
        var new_table = &self.tables.items[new_arch.table_id];
        const new_row = try new_table.pushRow(entity.index, self.current_tick);
        _ = try new_arch.addEntity(entity.index);

        for (0..count) |i| {
            const col = new_table.getColumn(component_ids[i]).?;
            const ptr = col.getPtr(new_row).?;
            @memcpy(ptr[0..component_sizes[i]], component_data_ptrs[i][0..component_sizes[i]]);
        }

        meta.archetype_id = new_arch_id;
        meta.row = @as(u32, @intCast(new_row));

        return true;
    }

    pub fn contains(self: *const World, entity: Entity) bool {
        if (entity.index >= self.entities.items.len) return false;
        const meta = self.entities.items[entity.index];
        return meta.is_alive and meta.generation == entity.generation;
    }
};

export fn world_create() ?*World {
    return World.init(g_allocator) catch null;
}

export fn world_destroy(world: *World) void {
    world.deinit();
}

export fn world_get_table_for_archetype(world: *World, archetype_id: u32) ?*Table {
    if (archetype_id >= world.archetypes.items.len) return null;
    const arch = &world.archetypes.items[archetype_id];
    const table_id = arch.table_id;
    if (table_id >= world.tables.items.len) return null;
    return &world.tables.items[table_id];
}

export fn world_spawn_empty(world: *World) Entity {
    return world.spawnEmpty() catch Entity{ .index = 0xFFFFFFFF, .generation = 0 };
}

export fn world_contains_entity(world: *const World, entity: Entity) bool {
    return world.contains(entity);
}

export fn world_set_tick(world: *World, tick: Tick) void {
    world.current_tick = tick;
}

export fn world_insert_components(
    world: *World,
    entity: Entity,
    component_ids: [*]const u32,
    component_sizes: [*]const usize,
    component_data_ptrs: [*]const [*]const u8,
    count: usize,
) bool {
    return world.insertComponents(entity, component_ids, component_sizes, component_data_ptrs, count) catch false;
}

export fn world_archetype_count(world: *const World) usize {
    return world.archetypes.items.len;
}

export fn world_get_archetype(world: *const World, index: usize) ?*const Archetype {
    if (index >= world.archetypes.items.len) return null;
    return &world.archetypes.items[index];
}
