const std = @import("std");
const Allocator = std.mem.Allocator;

pub const NodeId = usize;

pub const DependencyGraph = struct {
    allocator: Allocator,
    adjacency: std.ArrayList(std.ArrayList(NodeId)),
    in_degree: std.ArrayList(usize),
    node_count: usize,

    pub fn init(allocator: Allocator) DependencyGraph {
        return .{
            .allocator = allocator,
            .adjacency = std.ArrayList(std.ArrayList(NodeId)){},
            .in_degree = std.ArrayList(usize){},
            .node_count = 0,
        };
    }

    pub fn deinit(self: *DependencyGraph) void {
        for (self.adjacency.items) |*list| {
            list.deinit(self.allocator);
        }
        self.adjacency.deinit(self.allocator);
        self.in_degree.deinit(self.allocator);
    }

    pub fn addNode(self: *DependencyGraph) !NodeId {
        const id = self.node_count;
        const list = std.ArrayList(NodeId){};
        try self.adjacency.append(self.allocator, list);
        try self.in_degree.append(self.allocator, 0);
        self.node_count += 1;
        return id;
    }

    pub fn addEdge(self: *DependencyGraph, from: NodeId, to: NodeId) !void {
        if (from >= self.node_count or to >= self.node_count) return error.InvalidNodeId;

        // Check for duplicates
        for (self.adjacency.items[from].items) |neighbor| {
            if (neighbor == to) return;
        }

        try self.adjacency.items[from].append(self.allocator, to);
        self.in_degree.items[to] += 1;
    }

    // Returns a list of NodeId in topological order
    // Caller owns the returned ArrayList
    pub fn topologicalSort(self: *DependencyGraph) !std.ArrayList(NodeId) {
        var queue = std.ArrayList(NodeId){};
        // defer queue.deinit(self.allocator); // defer happens at end of scope

        var result = std.ArrayList(NodeId){};
        errdefer result.deinit(self.allocator);

        // Working copy of in-degrees
        var current_in_degree = try std.ArrayList(usize).initCapacity(self.allocator, self.node_count);
        defer current_in_degree.deinit(self.allocator);
        current_in_degree.appendSliceAssumeCapacity(self.in_degree.items);

        // Init queue with nodes having 0 in-degree
        for (current_in_degree.items, 0..) |degree, i| {
            if (degree == 0) {
                try queue.append(self.allocator, i);
            }
        }

        while (queue.items.len > 0) {
            const u = queue.pop() orelse unreachable;
            try result.append(self.allocator, u);

            for (self.adjacency.items[u].items) |v| {
                current_in_degree.items[v] -= 1;
                if (current_in_degree.items[v] == 0) {
                    try queue.append(self.allocator, v);
                }
            }
        }

        queue.deinit(self.allocator); // Explicit deinit since defer implies cleanup? Wait, defer runs at exit.

        if (result.items.len != self.node_count) {
            return error.CyclicDependency;
        }

        return result;
    }
};
