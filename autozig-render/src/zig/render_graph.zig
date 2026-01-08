//! Render Graph
//! Manages render nodes and execution flow

const std = @import("std");

/// Render node descriptor
pub const RenderNode = extern struct {
    name: [64]u8,
    name_len: u32,
    inputs: [8]u32,
    input_count: u32,
    outputs: [8]u32,
    output_count: u32,
    execute_fn: ?*const fn (*anyopaque, *anyopaque) callconv(.c) void,
    user_data: ?*anyopaque,
    is_enabled: bool,
};

/// Render graph
pub const RenderGraph = extern struct {
    nodes: [32]RenderNode,
    node_count: u32,
    execution_order: [32]u32,
    is_dirty: bool,
};

/// Create empty render node
export fn render_node_create() RenderNode {
    return RenderNode{
        .name = [_]u8{0} ** 64,
        .name_len = 0,
        .inputs = [_]u32{0} ** 8,
        .input_count = 0,
        .outputs = [_]u32{0} ** 8,
        .output_count = 0,
        .execute_fn = null,
        .user_data = null,
        .is_enabled = true,
    };
}

/// Set render node name
export fn render_node_set_name(node: *RenderNode, name: [*]const u8, len: u32) void {
    const copy_len = @min(len, 63);
    @memcpy(node.name[0..copy_len], name[0..copy_len]);
    node.name[copy_len] = 0;
    node.name_len = copy_len;
}

/// Add input to render node
export fn render_node_add_input(node: *RenderNode, input_id: u32) bool {
    if (node.input_count >= 8) return false;
    node.inputs[node.input_count] = input_id;
    node.input_count += 1;
    return true;
}

/// Add output to render node
export fn render_node_add_output(node: *RenderNode, output_id: u32) bool {
    if (node.output_count >= 8) return false;
    node.outputs[node.output_count] = output_id;
    node.output_count += 1;
    return true;
}

/// Set execute function
export fn render_node_set_execute_fn(
    node: *RenderNode,
    execute_fn: ?*const fn (*anyopaque, *anyopaque) callconv(.c) void,
) void {
    node.execute_fn = execute_fn;
}

/// Set user data
export fn render_node_set_user_data(node: *RenderNode, user_data: ?*anyopaque) void {
    node.user_data = user_data;
}

/// Enable/disable render node
export fn render_node_set_enabled(node: *RenderNode, enabled: bool) void {
    node.is_enabled = enabled;
}

/// Check if node is enabled
export fn render_node_is_enabled(node: *const RenderNode) bool {
    return node.is_enabled;
}

/// Create empty render graph
export fn render_graph_create() RenderGraph {
    return RenderGraph{
        .nodes = [_]RenderNode{render_node_create()} ** 32,
        .node_count = 0,
        .execution_order = [_]u32{0} ** 32,
        .is_dirty = true,
    };
}

/// Add node to render graph
export fn render_graph_add_node(graph: *RenderGraph, node: RenderNode) bool {
    if (graph.node_count >= 32) return false;
    graph.nodes[graph.node_count] = node;
    graph.node_count += 1;
    graph.is_dirty = true;
    return true;
}

/// Get node by index
export fn render_graph_get_node(graph: *RenderGraph, index: u32) ?*RenderNode {
    if (index >= graph.node_count) return null;
    return &graph.nodes[index];
}

/// Find node by name
export fn render_graph_find_node(graph: *RenderGraph, name: [*]const u8, len: u32) ?*RenderNode {
    var i: u32 = 0;
    while (i < graph.node_count) : (i += 1) {
        const node = &graph.nodes[i];
        if (node.name_len == len) {
            if (std.mem.eql(u8, node.name[0..len], name[0..len])) {
                return node;
            }
        }
    }
    return null;
}

/// Remove node by index
export fn render_graph_remove_node(graph: *RenderGraph, index: u32) bool {
    if (index >= graph.node_count) return false;

    // Shift nodes down
    var i = index;
    while (i < graph.node_count - 1) : (i += 1) {
        graph.nodes[i] = graph.nodes[i + 1];
    }
    graph.node_count -= 1;
    graph.is_dirty = true;
    return true;
}

/// Clear all nodes
export fn render_graph_clear(graph: *RenderGraph) void {
    graph.node_count = 0;
    graph.is_dirty = true;
}

/// Simple topological sort for execution order
export fn render_graph_update_execution_order(graph: *RenderGraph) void {
    // Simple implementation: just use insertion order for now
    // In a real implementation, this would do a topological sort based on dependencies
    var i: u32 = 0;
    while (i < graph.node_count) : (i += 1) {
        graph.execution_order[i] = i;
    }
    graph.is_dirty = false;
}

/// Execute render graph
export fn render_graph_execute(graph: *RenderGraph, context: *anyopaque) void {
    if (graph.is_dirty) {
        render_graph_update_execution_order(graph);
    }

    var i: u32 = 0;
    while (i < graph.node_count) : (i += 1) {
        const node_index = graph.execution_order[i];
        const node = &graph.nodes[node_index];

        if (node.is_enabled and node.execute_fn != null) {
            if (node.execute_fn) |execute_fn| {
                execute_fn(node.user_data orelse context, context);
            }
        }
    }
}

/// Get node count
export fn render_graph_get_node_count(graph: *const RenderGraph) u32 {
    return graph.node_count;
}

/// Check if graph is dirty
export fn render_graph_is_dirty(graph: *const RenderGraph) bool {
    return graph.is_dirty;
}

/// Mark graph as dirty
export fn render_graph_mark_dirty(graph: *RenderGraph) void {
    graph.is_dirty = true;
}

/// Get execution order
export fn render_graph_get_execution_order(
    graph: *const RenderGraph,
    out_order: [*]u32,
    max_count: u32,
) u32 {
    const count = @min(graph.node_count, max_count);
    @memcpy(out_order[0..count], graph.execution_order[0..count]);
    return count;
}
