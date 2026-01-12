const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
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
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !*World {
        const self = try allocator.create(World);
        self.* = World{
            .archetypes = std.ArrayList(Archetype){},
            .tables = std.ArrayList(Table){},
            .entities = std.ArrayList(EntityMeta){},
            .free_list = std.ArrayList(u32){},
            .next_entity_index = 0,
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

export fn world_remove_components(world: *World, entity: Entity, ids_ptr: [*]const u32, count: usize) bool {
    _ = world;
    _ = entity;
    _ = ids_ptr;
    _ = count;
    return true;
}
