//! Wireframe Material - 线框渲染材质系统

const std = @import("std");

pub const WireframeMaterial = extern struct {
    color: [4]f32,
};

export fn wireframe_material_init() WireframeMaterial {
    return WireframeMaterial{
        .color = [_]f32{ 1.0, 1.0, 1.0, 1.0 },
    };
}

export fn wireframe_material_new(color: *const [4]f32) WireframeMaterial {
    return WireframeMaterial{
        .color = color.*,
    };
}

export fn wireframe_material_set_color(mat: *WireframeMaterial, color: *const [4]f32) void {
    mat.color = color.*;
}
