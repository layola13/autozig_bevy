const std = @import("std");

// ============================================================================
// Vertex Data Structure
// ============================================================================

/// 顶点数据结构 - 包含所有标准顶点属性
pub const Vertex = extern struct {
    position: [3]f32,
    normal: [3]f32,
    uv: [2]f32,
    tangent: [4]f32, // xyz + handedness (w component)
    color: [4]f32,

    /// 创建一个默认顶点
    pub fn init() Vertex {
        return Vertex{
            .position = [_]f32{ 0.0, 0.0, 0.0 },
            .normal = [_]f32{ 0.0, 1.0, 0.0 },
            .uv = [_]f32{ 0.0, 0.0 },
            .tangent = [_]f32{ 1.0, 0.0, 0.0, 1.0 },
            .color = [_]f32{ 1.0, 1.0, 1.0, 1.0 },
        };
    }

    /// 创建一个只有位置的顶点
    pub fn withPosition(pos: [3]f32) Vertex {
        var v = init();
        v.position = pos;
        return v;
    }

    /// 创建一个带位置和法线的顶点
    pub fn withPositionNormal(pos: [3]f32, norm: [3]f32) Vertex {
        var v = init();
        v.position = pos;
        v.normal = norm;
        return v;
    }

    /// 创建一个带位置、法线和UV的顶点
    pub fn withPositionNormalUv(pos: [3]f32, norm: [3]f32, uv_coords: [2]f32) Vertex {
        var v = init();
        v.position = pos;
        v.normal = norm;
        v.uv = uv_coords;
        return v;
    }
};

// ============================================================================
// Vertex Attribute Enumeration
// ============================================================================

/// 顶点属性枚举
pub const VertexAttribute = enum(u32) {
    Position = 0,
    Normal = 1,
    Uv = 2,
    Tangent = 3,
    Color = 4,
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn vertex_init() Vertex {
    return Vertex.init();
}

export fn vertex_with_position(x: f32, y: f32, z: f32) Vertex {
    return Vertex.withPosition([_]f32{ x, y, z });
}

export fn vertex_with_position_normal(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32) Vertex {
    return Vertex.withPositionNormal([_]f32{ px, py, pz }, [_]f32{ nx, ny, nz });
}

export fn vertex_with_position_normal_uv(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32, u: f32, v: f32) Vertex {
    return Vertex.withPositionNormalUv([_]f32{ px, py, pz }, [_]f32{ nx, ny, nz }, [_]f32{ u, v });
}
