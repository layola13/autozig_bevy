const std = @import("std");

// ============================================================================
// Vertex Format
// ============================================================================

pub const VertexFormat = enum(u32) {
    Float32x2 = 0,
    Float32x3 = 1,
    Float32x4 = 2,
    Uint32 = 3,
    Uint32x2 = 4,
};

// ============================================================================
// Vertex Step Mode
// ============================================================================

pub const VertexStepMode = enum(u32) {
    Vertex = 0,
    Instance = 1,
};

// ============================================================================
// Vertex Attribute Descriptor
// ============================================================================

pub const VertexAttributeDesc = extern struct {
    format: VertexFormat,
    offset: u32,
    shader_location: u32,
};

// ============================================================================
// Vertex Buffer Layout
// ============================================================================

pub const VertexBufferLayout = extern struct {
    attributes: [8]VertexAttributeDesc,
    attribute_count: u32,
    stride: u32,
    step_mode: VertexStepMode,

    /// 标准顶点布局（包含所有属性）
    pub fn standard() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 5,
            .stride = 64, // 3*4 + 3*4 + 2*4 + 4*4 + 4*4 = 64 bytes
            .step_mode = .Vertex,
        };

        // Position (location 0)
        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        // Normal (location 1)
        layout.attributes[1] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 12,
            .shader_location = 1,
        };

        // UV (location 2)
        layout.attributes[2] = VertexAttributeDesc{
            .format = .Float32x2,
            .offset = 24,
            .shader_location = 2,
        };

        // Tangent (location 3)
        layout.attributes[3] = VertexAttributeDesc{
            .format = .Float32x4,
            .offset = 32,
            .shader_location = 3,
        };

        // Color (location 4)
        layout.attributes[4] = VertexAttributeDesc{
            .format = .Float32x4,
            .offset = 48,
            .shader_location = 4,
        };

        return layout;
    }

    /// 只有位置的顶点布局
    pub fn positionOnly() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 1,
            .stride = 12, // 3 * 4 = 12 bytes
            .step_mode = .Vertex,
        };

        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        return layout;
    }

    /// 位置和法线的顶点布局
    pub fn positionNormal() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 2,
            .stride = 24, // (3 + 3) * 4 = 24 bytes
            .step_mode = .Vertex,
        };

        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        layout.attributes[1] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 12,
            .shader_location = 1,
        };

        return layout;
    }

    /// 位置和UV的顶点布局
    pub fn positionUv() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 2,
            .stride = 20, // (3 + 2) * 4 = 20 bytes
            .step_mode = .Vertex,
        };

        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        layout.attributes[1] = VertexAttributeDesc{
            .format = .Float32x2,
            .offset = 12,
            .shader_location = 2,
        };

        return layout;
    }
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn vertex_layout_standard() VertexBufferLayout {
    return VertexBufferLayout.standard();
}

export fn vertex_layout_position_only() VertexBufferLayout {
    return VertexBufferLayout.positionOnly();
}

export fn vertex_layout_position_normal() VertexBufferLayout {
    return VertexBufferLayout.positionNormal();
}

export fn vertex_layout_position_uv() VertexBufferLayout {
    return VertexBufferLayout.positionUv();
}

export fn vertex_layout_stride(layout: *const VertexBufferLayout) u32 {
    return layout.stride;
}

export fn vertex_layout_attribute_count(layout: *const VertexBufferLayout) u32 {
    return layout.attribute_count;
}
