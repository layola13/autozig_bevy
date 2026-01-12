//! Access control core implementation in Zig (90% of logic)
//! 访问控制核心实现 - Zig

const std = @import("std");

/// ComponentId type matching Rust side
pub const ComponentId = u32;

/// Access tracking structure - 核心访问追踪结构
pub const AccessCore = extern struct {
    component_read_and_writes: u64, // Bitset for component reads and writes
    component_writes: u64, // Bitset for component writes
    resource_read_and_writes: u64, // Bitset for resource reads and writes
    resource_writes: u64, // Bitset for resource writes
    component_read_and_writes_inverted: bool,
    component_writes_inverted: bool,
    reads_all_resources: bool,
    writes_all_resources: bool,
    archetypal: u64, // Bitset for archetypal components

    pub fn init() AccessCore {
        return .{
            .component_read_and_writes = 0,
            .component_writes = 0,
            .resource_read_and_writes = 0,
            .resource_writes = 0,
            .component_read_and_writes_inverted = false,
            .component_writes_inverted = false,
            .reads_all_resources = false,
            .writes_all_resources = false,
            .archetypal = 0,
        };
    }

    /// Add component read access
    pub fn addComponentRead(self: *AccessCore, index: ComponentId) void {
        if (!self.component_read_and_writes_inverted) {
            const bit: u6 = @intCast(index % 64);
            self.component_read_and_writes |= (@as(u64, 1) << bit);
        }
    }

    /// Add component write access
    pub fn addComponentWrite(self: *AccessCore, index: ComponentId) void {
        self.addComponentRead(index);
        if (!self.component_writes_inverted) {
            const bit: u6 = @intCast(index % 64);
            self.component_writes |= (@as(u64, 1) << bit);
        }
    }

    /// Add resource read access
    pub fn addResourceRead(self: *AccessCore, index: ComponentId) void {
        const bit: u6 = @intCast(index % 64);
        self.resource_read_and_writes |= (@as(u64, 1) << bit);
    }

    /// Add resource write access
    pub fn addResourceWrite(self: *AccessCore, index: ComponentId) void {
        const bit: u6 = @intCast(index % 64);
        self.resource_read_and_writes |= (@as(u64, 1) << bit);
        self.resource_writes |= (@as(u64, 1) << bit);
    }

    /// Remove component read access
    pub fn removeComponentRead(self: *AccessCore, index: ComponentId) void {
        const bit: u6 = @intCast(index % 64);
        const mask = ~(@as(u64, 1) << bit);
        self.component_read_and_writes &= mask;
        self.component_writes &= mask;
    }

    /// Remove component write access
    pub fn removeComponentWrite(self: *AccessCore, index: ComponentId) void {
        const bit: u6 = @intCast(index % 64);
        const mask = ~(@as(u64, 1) << bit);
        self.component_writes &= mask;
    }

    /// Add archetypal access
    pub fn addArchetypal(self: *AccessCore, index: ComponentId) void {
        const bit: u6 = @intCast(index % 64);
        self.archetypal |= (@as(u64, 1) << bit);
    }

    /// Check if has component read access
    pub fn hasComponentRead(self: *const AccessCore, index: ComponentId) bool {
        const bit: u6 = @intCast(index % 64);
        const has_bit = (self.component_read_and_writes & (@as(u64, 1) << bit)) != 0;
        return self.component_read_and_writes_inverted != has_bit;
    }

    /// Check if has any component read
    pub fn hasAnyComponentRead(self: *const AccessCore) bool {
        return self.component_read_and_writes_inverted or self.component_read_and_writes != 0;
    }

    /// Check if has component write access
    pub fn hasComponentWrite(self: *const AccessCore, index: ComponentId) bool {
        const bit: u6 = @intCast(index % 64);
        const has_bit = (self.component_writes & (@as(u64, 1) << bit)) != 0;
        return self.component_writes_inverted != has_bit;
    }

    /// Check if has any component write
    pub fn hasAnyComponentWrite(self: *const AccessCore) bool {
        return self.component_writes_inverted or self.component_writes != 0;
    }

    /// Check if has resource read access
    pub fn hasResourceRead(self: *const AccessCore, index: ComponentId) bool {
        if (self.reads_all_resources) return true;
        const bit: u6 = @intCast(index % 64);
        return (self.resource_read_and_writes & (@as(u64, 1) << bit)) != 0;
    }

    /// Check if has any resource read
    pub fn hasAnyResourceRead(self: *const AccessCore) bool {
        return self.reads_all_resources or self.resource_read_and_writes != 0;
    }

    /// Check if has resource write access
    pub fn hasResourceWrite(self: *const AccessCore, index: ComponentId) bool {
        if (self.writes_all_resources) return true;
        const bit: u6 = @intCast(index % 64);
        return (self.resource_writes & (@as(u64, 1) << bit)) != 0;
    }

    /// Check if has any resource write
    pub fn hasAnyResourceWrite(self: *const AccessCore) bool {
        return self.writes_all_resources or self.resource_writes != 0;
    }

    /// Check if has archetypal access
    pub fn hasArchetypal(self: *const AccessCore, index: ComponentId) bool {
        const bit: u6 = @intCast(index % 64);
        return (self.archetypal & (@as(u64, 1) << bit)) != 0;
    }

    /// Read all components
    pub fn readAllComponents(self: *AccessCore) void {
        self.component_read_and_writes_inverted = true;
        self.component_read_and_writes = 0;
    }

    /// Write all components
    pub fn writeAllComponents(self: *AccessCore) void {
        self.readAllComponents();
        self.component_writes_inverted = true;
        self.component_writes = 0;
    }

    /// Read all resources
    pub fn readAllResources(self: *AccessCore) void {
        self.reads_all_resources = true;
    }

    /// Write all resources
    pub fn writeAllResources(self: *AccessCore) void {
        self.reads_all_resources = true;
        self.writes_all_resources = true;
    }

    /// Read all (components and resources)
    pub fn readAll(self: *AccessCore) void {
        self.readAllComponents();
        self.readAllResources();
    }

    /// Write all (components and resources)
    pub fn writeAll(self: *AccessCore) void {
        self.writeAllComponents();
        self.writeAllResources();
    }

    /// Check if has read all components
    pub fn hasReadAllComponents(self: *const AccessCore) bool {
        return self.component_read_and_writes_inverted and self.component_read_and_writes == 0;
    }

    /// Check if has write all components
    pub fn hasWriteAllComponents(self: *const AccessCore) bool {
        return self.component_writes_inverted and self.component_writes == 0;
    }

    /// Check if has read all resources
    pub fn hasReadAllResources(self: *const AccessCore) bool {
        return self.reads_all_resources;
    }

    /// Check if has write all resources
    pub fn hasWriteAllResources(self: *const AccessCore) bool {
        return self.writes_all_resources;
    }

    /// Clear all writes
    pub fn clearWrites(self: *AccessCore) void {
        self.writes_all_resources = false;
        self.component_writes_inverted = false;
        self.component_writes = 0;
        self.resource_writes = 0;
    }

    /// Clear all access
    pub fn clear(self: *AccessCore) void {
        self.reads_all_resources = false;
        self.writes_all_resources = false;
        self.component_read_and_writes_inverted = false;
        self.component_writes_inverted = false;
        self.component_read_and_writes = 0;
        self.component_writes = 0;
        self.resource_read_and_writes = 0;
        self.resource_writes = 0;
    }

    /// Extend access with another access
    pub fn extend(self: *AccessCore, other: *const AccessCore) void {
        self.component_read_and_writes |= other.component_read_and_writes;
        self.component_writes |= other.component_writes;
        self.reads_all_resources = self.reads_all_resources or other.reads_all_resources;
        self.writes_all_resources = self.writes_all_resources or other.writes_all_resources;
        self.resource_read_and_writes |= other.resource_read_and_writes;
        self.resource_writes |= other.resource_writes;
        self.archetypal |= other.archetypal;
    }

    /// Remove conflicting access
    pub fn removeConflictingAccess(self: *AccessCore, other: *const AccessCore) void {
        self.component_read_and_writes &= ~other.component_writes;
        self.component_writes &= ~other.component_read_and_writes;

        if (other.reads_all_resources) {
            self.writes_all_resources = false;
            self.resource_writes = 0;
        }
        if (other.writes_all_resources) {
            self.reads_all_resources = false;
            self.resource_read_and_writes = 0;
        }

        self.resource_read_and_writes &= ~other.resource_writes;
        self.resource_writes &= ~other.resource_read_and_writes;
    }

    /// Check if components are compatible
    pub fn isComponentsCompatible(self: *const AccessCore, other: *const AccessCore) bool {
        const writes_intersect_reads = (self.component_writes & other.component_read_and_writes) != 0;
        const other_writes_intersect_reads = (other.component_writes & self.component_read_and_writes) != 0;
        return !writes_intersect_reads and !other_writes_intersect_reads;
    }

    /// Check if resources are compatible
    pub fn isResourcesCompatible(self: *const AccessCore, other: *const AccessCore) bool {
        if (self.writes_all_resources) {
            return !other.hasAnyResourceRead();
        }
        if (other.writes_all_resources) {
            return !self.hasAnyResourceRead();
        }

        const writes_intersect_reads = (self.resource_writes & other.resource_read_and_writes) != 0;
        const other_writes_intersect_reads = (other.resource_writes & self.resource_read_and_writes) != 0;
        return !writes_intersect_reads and !other_writes_intersect_reads;
    }

    /// Check if compatible with another access
    pub fn isCompatible(self: *const AccessCore, other: *const AccessCore) bool {
        return self.isComponentsCompatible(other) and self.isResourcesCompatible(other);
    }

    /// Check if component access is subset of another
    pub fn isSubsetComponents(self: *const AccessCore, other: *const AccessCore) bool {
        const reads_subset = (self.component_read_and_writes & ~other.component_read_and_writes) == 0;
        const writes_subset = (self.component_writes & ~other.component_writes) == 0;
        return reads_subset and writes_subset;
    }

    /// Check if resource access is subset of another
    pub fn isSubsetResources(self: *const AccessCore, other: *const AccessCore) bool {
        if (self.writes_all_resources) {
            return other.writes_all_resources;
        }
        const reads_subset = (self.resource_read_and_writes & ~other.resource_read_and_writes) == 0;
        const writes_subset = (self.resource_writes & ~other.resource_writes) == 0;
        return reads_subset and writes_subset;
    }

    /// Check if is subset of another access
    pub fn isSubset(self: *const AccessCore, other: *const AccessCore) bool {
        return self.isSubsetComponents(other) and self.isSubsetResources(other);
    }
};

// Export C ABI functions for Rust FFI
export fn access_core_create() *AccessCore {
    const allocator = std.heap.c_allocator;
    const access = allocator.create(AccessCore) catch unreachable;
    access.* = AccessCore.init();
    return access;
}

export fn access_core_destroy(access: *AccessCore) void {
    const allocator = std.heap.c_allocator;
    allocator.destroy(access);
}

export fn access_core_add_component_read(access: *AccessCore, index: ComponentId) void {
    access.addComponentRead(index);
}

export fn access_core_add_component_write(access: *AccessCore, index: ComponentId) void {
    access.addComponentWrite(index);
}

export fn access_core_add_resource_read(access: *AccessCore, index: ComponentId) void {
    access.addResourceRead(index);
}

export fn access_core_add_resource_write(access: *AccessCore, index: ComponentId) void {
    access.addResourceWrite(index);
}

export fn access_core_has_component_read(access: *const AccessCore, index: ComponentId) bool {
    return access.hasComponentRead(index);
}

export fn access_core_has_component_write(access: *const AccessCore, index: ComponentId) bool {
    return access.hasComponentWrite(index);
}

export fn access_core_has_resource_read(access: *const AccessCore, index: ComponentId) bool {
    return access.hasResourceRead(index);
}

export fn access_core_has_resource_write(access: *const AccessCore, index: ComponentId) bool {
    return access.hasResourceWrite(index);
}

export fn access_core_is_compatible(access: *const AccessCore, other: *const AccessCore) bool {
    return access.isCompatible(other);
}

export fn access_core_extend(access: *AccessCore, other: *const AccessCore) void {
    access.extend(other);
}

export fn access_core_clear(access: *AccessCore) void {
    access.clear();
}
