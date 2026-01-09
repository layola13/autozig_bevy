const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

//  Entity structure - 已在entity.zig中定义，合并时会自动可用
// 不需要导入，因为autozig会将所有文件合并到一个generated_autozig.zig中

// World structure - manages entities
pub const World = struct {
    allocator: std.mem.Allocator,
    next_entity_index: u32,
    entities: std.ArrayList(EntityMeta),
    free_list: std.ArrayList(u32),
    
    const EntityMeta = struct {
        generation: u32,
        is_alive: bool,
    };
    
    pub fn init(alloc: std.mem.Allocator) !*World {
        const world = try alloc.create(World);
        world.* = World{
            .allocator = alloc,
            .next_entity_index = 0,
            .entities = std.ArrayList(EntityMeta){},
            .free_list = std.ArrayList(u32){},
        };
        return world;
    }
    
    pub fn deinit(self: *World) void {
        self.entities.deinit(self.allocator);
        self.free_list.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    pub fn spawnEmpty(self: *World) !Entity {
        const index: u32 = if (self.free_list.items.len > 0) blk: {
            const idx = self.free_list.pop().?; // pop() returns ?u32 in Zig 0.12
            const meta = &self.entities.items[idx];
            meta.is_alive = true;
            break :blk idx;
        } else blk: {
            const idx = self.next_entity_index;
            self.next_entity_index += 1;
            try self.entities.append(self.allocator, EntityMeta{
                .generation = 0,
                .is_alive = true,
            });
            break :blk idx;
        };
        
        const generation = self.entities.items[index].generation;
        return Entity{
            .index = index,
            .generation = generation,
        };
    }
    
    pub fn despawn(self: *World, entity: Entity) bool {
        if (entity.index >= self.entities.items.len) return false;
        
        const meta = &self.entities.items[entity.index];
        if (!meta.is_alive or meta.generation != entity.generation) {
            return false;
        }
        
        meta.is_alive = false;
        meta.generation +%= 1;
        self.free_list.append(self.allocator, entity.index) catch return false;
        return true;
    }
    
    pub fn entityCount(self: *const World) u32 {
        var count: u32 = 0;
        for (self.entities.items) |meta| {
            if (meta.is_alive) count += 1;
        }
        return count;
    }
    
    pub fn contains(self: *const World, entity: Entity) bool {
        if (entity.index >= self.entities.items.len) return false;
        const meta = self.entities.items[entity.index];
        return meta.is_alive and meta.generation == entity.generation;
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

export fn world_create() ?*World {
    return World.init(g_allocator) catch null;
}

export fn world_destroy(world_ptr: *World) void {
    world_ptr.deinit();
}

export fn world_spawn_empty(world_ptr: *World) Entity {
    return world_ptr.spawnEmpty() catch Entity{ .index = 0xFFFFFFFF, .generation = 0 };
}

export fn world_despawn(world_ptr: *World, entity: Entity) bool {
    return world_ptr.despawn(entity);
}

export fn world_entity_count(world_ptr: *const World) u32 {
    return world_ptr.entityCount();
}

export fn world_contains_entity(world_ptr: *const World, entity: Entity) bool {
    return world_ptr.contains(entity);
}
