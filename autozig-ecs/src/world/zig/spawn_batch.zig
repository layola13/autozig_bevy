const std = @import("std");
const world_mod = @import("../../zig/world.zig");
const World = world_mod.World;
const common = @import("../../zig/common.zig");
const EntityStruct = common.Entity; // The extern struct

/// Reserves space for a batch of entities in the world
export fn spawn_batch_reserve(world_ptr: *anyopaque, count: usize) void {
    const world: *World = @ptrCast(@alignCast(world_ptr));
    world.entities.ensureTotalCapacity(world.allocator, world.entities.items.len + count) catch return;
}

/// Allocates a batch of entities and returns a pointer to the array
/// Caller is responsible for freeing the returned memory
export fn spawn_batch_alloc_entities(world_ptr: *anyopaque, count: usize) [*]EntityStruct {
    const world: *World = @ptrCast(@alignCast(world_ptr));

    // Allocate return array
    const allocator = std.heap.c_allocator;
    const entities = allocator.alloc(EntityStruct, count) catch {
        @panic("Out of memory");
    };

    // Get Archetype 0 and Table 0
    // Note: World init ensures these exist.
    // Use const for pointers (pointers themselves are const, data is mutable via them)
    // NOTE: items[0] returns *value* (struct copy) if not addressed?
    // &items[0] returns pointer.
    const arch0 = &world.archetypes.items[0];
    const table0 = &world.tables.items[0];

    var i: usize = 0;
    while (i < count) : (i += 1) {
        // Reuse free list if available
        if (world.free_list.items.len > 0) {
            const idx = world.free_list.pop().?; // Safe because len check

            // Add to storage
            const row = arch0.addEntity(idx) catch @panic("Failed to add to archetype 0");
            _ = table0.pushRow(idx) catch @panic("Failed to push to table 0");

            const meta = &world.entities.items[idx];
            meta.is_alive = true;
            meta.archetype_id = 0;
            meta.row = @intCast(row);

            // Generation is already incremented on despawn
            entities[i] = EntityStruct{
                .index = idx,
                .generation = meta.generation,
            };
        } else {
            // Allocate new
            const idx = world.next_entity_index;
            world.next_entity_index += 1;

            // Add to storage
            const row = arch0.addEntity(idx) catch @panic("Failed to add to archetype 0");
            _ = table0.pushRow(idx) catch @panic("Failed to push to table 0");

            world.entities.append(world.allocator, .{
                .generation = 0,
                .is_alive = true,
                .archetype_id = 0,
                .row = @intCast(row),
            }) catch @panic("OOM in World append");

            entities[i] = EntityStruct{
                .index = idx,
                .generation = 0,
            };
        }
    }

    return entities.ptr;
}
