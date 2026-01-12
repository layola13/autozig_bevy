const std = @import("std");
const world_mod = @import("../../zig/world.zig");
const World = world_mod.World;
const Entity = world_mod.World.EntityMeta; // Use EntityMeta internally? No, need Entity struct
// Actually world.zig defines World, but Entity is in entity.zig which is imported by common.zig
// Let's use the definition in world.zig which likely imports Entity.
// Wait, spawn_batch.zig imports std.
// We need to import world.zig to see the World struct layout.

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

    var i: usize = 0;
    while (i < count) : (i += 1) {
        // Reuse free list if available
        if (world.free_list.items.len > 0) {
            const idx = world.free_list.pop().?;
            world.entities.items[idx].is_alive = true;
            // Generation is already incremented on despawn
            entities[i] = EntityStruct{
                .index = idx,
                .generation = world.entities.items[idx].generation,
            };
        } else {
            // Allocate new
            const idx = world.next_entity_index;
            world.next_entity_index += 1;

            // We suppressed error handling here for brevity but in C ABI we might need to be safer
            // effectively panic on append fail for now as we can't return error
            world.entities.append(world.allocator, .{
                .generation = 0,
                .is_alive = true,
            }) catch @panic("OOM in World append");

            entities[i] = EntityStruct{
                .index = idx,
                .generation = 0,
            };
        }
    }

    return entities.ptr;
}
