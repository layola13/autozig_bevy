//! Zig Tape-based JSON Parser
//!
//! This is the fastest JSON parsing architecture - a flat array of Nodes
//! representing the JSON DOM. Each Node is exactly 16 bytes.

const std = @import("std");
const builtin = @import("builtin");

// ============================================================================
// Node Types
// ============================================================================

pub const NodeType = enum(u8) {
    Null = 0,
    Bool = 1,
    Number = 2,
    String = 3,
    Array = 4,
    Object = 5,
    Error = 255,
};

// 16-byte Node structure - very cache friendly
pub const Node = extern struct {
    tag: NodeType, // 1 byte
    _pad: u8 = 0, // 1 byte padding
    start: u32, // 4 bytes - start position in input
    len: u32, // 4 bytes - length of value
    next: u32, // 4 bytes - next sibling index (0 = none)
    child: u32, // 4 bytes - first child index (0 = none)
};

// ============================================================================
// Tape Parser
// ============================================================================

pub const Tape = extern struct {
    nodes: [*]Node,
    count: usize,
    capacity: usize,
};

const MAX_DEPTH = 64;
const MAX_NODES = 4096; // Reduced from 65536 to avoid stack issues

// Static node buffer for parsing (no allocations needed)
var node_buffer: [MAX_NODES]Node = undefined;

// Skip whitespace
fn skip_ws(ptr: [*]const u8, len: usize, pos: usize) usize {
    var i = pos;
    while (i < len) {
        const c = ptr[i];
        if (c != ' ' and c != '\t' and c != '\n' and c != '\r') break;
        i += 1;
    }
    return i;
}

// Parse a string, return end position (after closing quote)
fn parse_string(ptr: [*]const u8, len: usize, start: usize) struct { end: usize, content_start: usize, content_len: usize } {
    // start is at opening quote
    var i = start + 1;
    const content_start = i;

    while (i < len) {
        const c = ptr[i];
        if (c == '"') {
            return .{ .end = i + 1, .content_start = content_start, .content_len = i - content_start };
        } else if (c == '\\') {
            i += 2; // Skip escape
        } else {
            i += 1;
        }
    }
    return .{ .end = len, .content_start = content_start, .content_len = 0 };
}

// Parse a number
fn parse_number(ptr: [*]const u8, len: usize, start: usize) usize {
    var i = start;
    if (i < len and ptr[i] == '-') i += 1;

    while (i < len) {
        const c = ptr[i];
        if ((c >= '0' and c <= '9') or c == '.' or c == 'e' or c == 'E' or c == '+' or c == '-') {
            i += 1;
        } else {
            break;
        }
    }
    return i;
}

// Recursive parser state
const ParseState = struct {
    pos: usize,
    node_count: usize,
    depth: usize,
};

fn parse_value(ptr: [*]const u8, len: usize, state: *ParseState) u32 {
    if (state.depth >= MAX_DEPTH or state.node_count >= MAX_NODES) {
        return 0;
    }

    state.pos = skip_ws(ptr, len, state.pos);
    if (state.pos >= len) return 0;

    const node_idx = @as(u32, @intCast(state.node_count));
    state.node_count += 1;

    const c = ptr[state.pos];

    if (c == '"') {
        // String
        const result = parse_string(ptr, len, state.pos);
        node_buffer[node_idx] = Node{
            .tag = .String,
            .start = @intCast(result.content_start),
            .len = @intCast(result.content_len),
            .next = 0,
            .child = 0,
        };
        state.pos = result.end;
    } else if (c == '{') {
        // Object
        node_buffer[node_idx] = Node{
            .tag = .Object,
            .start = @intCast(state.pos),
            .len = 0,
            .next = 0,
            .child = 0,
        };
        state.pos += 1;
        state.depth += 1;

        var first_child: u32 = 0;
        var prev_value: u32 = 0;

        while (true) {
            state.pos = skip_ws(ptr, len, state.pos);
            if (state.pos >= len) break;
            if (ptr[state.pos] == '}') {
                state.pos += 1;
                break;
            }

            // Parse key
            if (ptr[state.pos] != '"') break;
            const key_idx = parse_value(ptr, len, state);
            if (key_idx == 0) break;

            // Skip colon
            state.pos = skip_ws(ptr, len, state.pos);
            if (state.pos >= len or ptr[state.pos] != ':') break;
            state.pos += 1;

            // Parse value
            const val_idx = parse_value(ptr, len, state);
            if (val_idx == 0) break;

            // Link key->child = value
            node_buffer[key_idx].child = val_idx;

            // Link siblings
            if (first_child == 0) {
                first_child = key_idx;
            } else {
                node_buffer[prev_value].next = key_idx;
            }
            prev_value = key_idx;

            // Skip comma
            state.pos = skip_ws(ptr, len, state.pos);
            if (state.pos < len and ptr[state.pos] == ',') {
                state.pos += 1;
            }
        }

        node_buffer[node_idx].child = first_child;
        node_buffer[node_idx].len = @intCast(state.pos - node_buffer[node_idx].start);
        state.depth -= 1;
    } else if (c == '[') {
        // Array
        node_buffer[node_idx] = Node{
            .tag = .Array,
            .start = @intCast(state.pos),
            .len = 0,
            .next = 0,
            .child = 0,
        };
        state.pos += 1;
        state.depth += 1;

        var first_child: u32 = 0;
        var prev_child: u32 = 0;

        while (true) {
            state.pos = skip_ws(ptr, len, state.pos);
            if (state.pos >= len) break;
            if (ptr[state.pos] == ']') {
                state.pos += 1;
                break;
            }

            const child_idx = parse_value(ptr, len, state);
            if (child_idx == 0) break;

            if (first_child == 0) {
                first_child = child_idx;
            } else {
                node_buffer[prev_child].next = child_idx;
            }
            prev_child = child_idx;

            state.pos = skip_ws(ptr, len, state.pos);
            if (state.pos < len and ptr[state.pos] == ',') {
                state.pos += 1;
            }
        }

        node_buffer[node_idx].child = first_child;
        node_buffer[node_idx].len = @intCast(state.pos - node_buffer[node_idx].start);
        state.depth -= 1;
    } else if (c == 't' and state.pos + 4 <= len) {
        // true
        node_buffer[node_idx] = Node{
            .tag = .Bool,
            .start = @intCast(state.pos),
            .len = 4,
            .next = 0,
            .child = 1, // Store true as child=1
        };
        state.pos += 4;
    } else if (c == 'f' and state.pos + 5 <= len) {
        // false
        node_buffer[node_idx] = Node{
            .tag = .Bool,
            .start = @intCast(state.pos),
            .len = 5,
            .next = 0,
            .child = 0, // Store false as child=0
        };
        state.pos += 5;
    } else if (c == 'n' and state.pos + 4 <= len) {
        // null
        node_buffer[node_idx] = Node{
            .tag = .Null,
            .start = @intCast(state.pos),
            .len = 4,
            .next = 0,
            .child = 0,
        };
        state.pos += 4;
    } else if (c == '-' or (c >= '0' and c <= '9')) {
        // Number
        const end = parse_number(ptr, len, state.pos);
        node_buffer[node_idx] = Node{
            .tag = .Number,
            .start = @intCast(state.pos),
            .len = @intCast(end - state.pos),
            .next = 0,
            .child = 0,
        };
        state.pos = end;
    } else {
        // Error
        node_buffer[node_idx] = Node{
            .tag = .Error,
            .start = @intCast(state.pos),
            .len = 0,
            .next = 0,
            .child = 0,
        };
        state.node_count -= 1;
        return 0;
    }

    return node_idx;
}

// ============================================================================
// Exported Functions
// ============================================================================

/// Parse JSON and return tape
export fn tape_parse(ptr: [*]const u8, len: usize) Tape {
    var state = ParseState{
        .pos = 0,
        .node_count = 0,
        .depth = 0,
    };

    _ = parse_value(ptr, len, &state);

    return Tape{
        .nodes = &node_buffer,
        .count = state.node_count,
        .capacity = MAX_NODES,
    };
}

/// Get node at index
export fn tape_get_node(tape: *const Tape, idx: usize) Node {
    if (idx < tape.count) {
        return tape.nodes[idx];
    }
    return Node{
        .tag = .Error,
        .start = 0,
        .len = 0,
        .next = 0,
        .child = 0,
    };
}

/// Get node count
export fn tape_node_count(tape: *const Tape) usize {
    return tape.count;
}

/// Parse number as f64
export fn node_as_f64(ptr: [*]const u8, start: u32, len: u32) f64 {
    const slice = ptr[start .. start + len];
    return std.fmt.parseFloat(f64, slice) catch 0.0;
}

/// Parse number as i64
export fn node_as_i64(ptr: [*]const u8, start: u32, len: u32) i64 {
    const slice = ptr[start .. start + len];
    return std.fmt.parseInt(i64, slice, 10) catch 0;
}

/// Check if number is float
export fn node_is_float(ptr: [*]const u8, start: u32, len: u32) bool {
    var i = start;
    while (i < start + len) : (i += 1) {
        if (ptr[i] == '.' or ptr[i] == 'e' or ptr[i] == 'E') return true;
    }
    return false;
}

// ============================================================================
// Legacy Token Scanner (for de.rs compatibility)
// ============================================================================

pub const TokenType = enum(u8) {
    Error = 0,
    ObjectStart = 1,
    ObjectEnd = 2,
    ArrayStart = 3,
    ArrayEnd = 4,
    String = 5,
    Number = 6,
    True = 7,
    False = 8,
    Null = 9,
    Colon = 10,
    Comma = 11,
    Eof = 12,
};

pub const Token = extern struct {
    kind: TokenType,
    start: usize,
    len: usize,
    next_cursor: usize,
};

fn scan_string_token(ptr: [*]const u8, total_len: usize, start: usize) Token {
    var i = start + 1;
    while (i < total_len) {
        const c = ptr[i];
        if (c == '"') {
            return Token{
                .kind = .String,
                .start = start + 1,
                .len = i - start - 1,
                .next_cursor = i + 1,
            };
        } else if (c == '\\') {
            i += 2;
        } else {
            i += 1;
        }
    }
    return Token{ .kind = .Error, .start = start, .len = 0, .next_cursor = total_len };
}

fn scan_number_token(ptr: [*]const u8, total_len: usize, start: usize) Token {
    var i = start;
    if (i < total_len and ptr[i] == '-') i += 1;
    while (i < total_len) {
        const c = ptr[i];
        if ((c >= '0' and c <= '9') or c == '.' or c == 'e' or c == 'E' or c == '+' or c == '-') {
            i += 1;
        } else {
            break;
        }
    }
    return Token{ .kind = .Number, .start = start, .len = i - start, .next_cursor = i };
}

fn check_kw(ptr: [*]const u8, total_len: usize, pos: usize, keyword: []const u8) bool {
    if (pos + keyword.len > total_len) return false;
    for (keyword, 0..) |c, i| {
        if (ptr[pos + i] != c) return false;
    }
    return true;
}

export fn next_token(ptr: [*]const u8, total_len: usize, cursor: usize) Token {
    var i = cursor;
    while (i < total_len) : (i += 1) {
        const c = ptr[i];
        if (c != ' ' and c != '\t' and c != '\n' and c != '\r') break;
    }
    if (i >= total_len) {
        return Token{ .kind = .Eof, .start = i, .len = 0, .next_cursor = i };
    }
    const c = ptr[i];
    return switch (c) {
        '{' => Token{ .kind = .ObjectStart, .start = i, .len = 1, .next_cursor = i + 1 },
        '}' => Token{ .kind = .ObjectEnd, .start = i, .len = 1, .next_cursor = i + 1 },
        '[' => Token{ .kind = .ArrayStart, .start = i, .len = 1, .next_cursor = i + 1 },
        ']' => Token{ .kind = .ArrayEnd, .start = i, .len = 1, .next_cursor = i + 1 },
        ':' => Token{ .kind = .Colon, .start = i, .len = 1, .next_cursor = i + 1 },
        ',' => Token{ .kind = .Comma, .start = i, .len = 1, .next_cursor = i + 1 },
        '"' => scan_string_token(ptr, total_len, i),
        '-', '0'...'9' => scan_number_token(ptr, total_len, i),
        't' => if (check_kw(ptr, total_len, i, "true"))
            Token{ .kind = .True, .start = i, .len = 4, .next_cursor = i + 4 }
        else
            Token{ .kind = .Error, .start = i, .len = 0, .next_cursor = i },
        'f' => if (check_kw(ptr, total_len, i, "false"))
            Token{ .kind = .False, .start = i, .len = 5, .next_cursor = i + 5 }
        else
            Token{ .kind = .Error, .start = i, .len = 0, .next_cursor = i },
        'n' => if (check_kw(ptr, total_len, i, "null"))
            Token{ .kind = .Null, .start = i, .len = 4, .next_cursor = i + 4 }
        else
            Token{ .kind = .Error, .start = i, .len = 0, .next_cursor = i },
        else => Token{ .kind = .Error, .start = i, .len = 0, .next_cursor = i },
    };
}

export fn parse_number_f64(ptr: [*]const u8, len: usize) f64 {
    const slice = ptr[0..len];
    return std.fmt.parseFloat(f64, slice) catch 0.0;
}

export fn parse_number_i64(ptr: [*]const u8, len: usize) i64 {
    const slice = ptr[0..len];
    return std.fmt.parseInt(i64, slice, 10) catch 0;
}
