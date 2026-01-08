// Transform system for updating hierarchies and local-to-world matrices

const std = @import("std");
const Transform = @import("transform.zig").Transform;
const Hierarchy = @import("hierarchy.zig").Hierarchy;
const LocalToWorld = @import("local_to_world.zig").LocalToWorld;

/// Transform system for managing transform updates
pub const TransformSystem = extern struct {
    /// Update hierarchy: compute local-to-world matrix for a node with hierarchy
    pub fn update_hierarchy(
        hierarchy: *Hierarchy,
        transform: *Transform,
        local_to_world: *LocalToWorld,
    ) void {
        // If no parent, local-to-world is just the local transform
        if (!hierarchy.has_parent()) {
            const matrix = transform.compute_matrix();
            local_to_world.set_matrix(matrix);
        }
        // If has parent, the parent's local-to-world should be provided separately
        // This is a simplified version - full hierarchy traversal happens at system level
    }

    /// Update local-to-world matrix from transform and parent's local-to-world
    pub fn update_local_to_world(
        transform: *Transform,
        parent_local_to_world: *LocalToWorld,
        local_to_world: *LocalToWorld,
    ) void {
        const parent_matrix = parent_local_to_world.get_matrix();
        const result_matrix = transform.compute_local_to_world(parent_matrix);
        local_to_world.set_matrix(result_matrix);
    }

    /// Update local-to-world for a root entity (no parent)
    pub fn update_root_local_to_world(
        transform: *Transform,
        local_to_world: *LocalToWorld,
    ) void {
        const matrix = transform.compute_matrix();
        local_to_world.set_matrix(matrix);
    }

    /// Propagate transform changes through hierarchy
    /// This is a simplified version - full traversal with recursion would be done in Rust
    pub fn propagate_transforms(
        transforms: [*]Transform,
        hierarchies: [*]Hierarchy,
        local_to_worlds: [*]LocalToWorld,
        entity_id: u32,
        parent_ltw: *const LocalToWorld,
    ) void {
        // Update this entity's local-to-world
        const transform = &transforms[entity_id];
        const local_to_world = &local_to_worlds[entity_id];

        const parent_matrix = parent_ltw.get_matrix();
        const result_matrix = transform.compute_local_to_world(parent_matrix);
        local_to_world.set_matrix(result_matrix);

        // Propagate to children
        const hierarchy = &hierarchies[entity_id];
        var i: u32 = 0;
        while (i < hierarchy.children_count) : (i += 1) {
            const child_id = hierarchy.children[i];
            if (child_id != 0) {
                propagate_transforms(
                    transforms,
                    hierarchies,
                    local_to_worlds,
                    child_id,
                    local_to_world,
                );
            }
        }
    }
};

// Export C-compatible functions for FFI
export fn transform_system_update_hierarchy(
    hierarchy: *Hierarchy,
    transform: *Transform,
    local_to_world: *LocalToWorld,
) void {
    TransformSystem.update_hierarchy(hierarchy, transform, local_to_world);
}

export fn transform_system_update_local_to_world(
    transform: *Transform,
    parent_local_to_world: *LocalToWorld,
    local_to_world: *LocalToWorld,
) void {
    TransformSystem.update_local_to_world(transform, parent_local_to_world, local_to_world);
}

export fn transform_system_update_root_local_to_world(
    transform: *Transform,
    local_to_world: *LocalToWorld,
) void {
    TransformSystem.update_root_local_to_world(transform, local_to_world);
}

export fn transform_system_propagate_transforms(
    transforms: [*]Transform,
    hierarchies: [*]Hierarchy,
    local_to_worlds: [*]LocalToWorld,
    entity_id: u32,
    parent_ltw: *const LocalToWorld,
) void {
    TransformSystem.propagate_transforms(
        transforms,
        hierarchies,
        local_to_worlds,
        entity_id,
        parent_ltw,
    );
}
