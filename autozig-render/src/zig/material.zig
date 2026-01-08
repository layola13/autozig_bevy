//! Material System
//! Handles material properties and textures

const std = @import("std");

/// Material component
pub const Material = extern struct {
    base_color: [4]f32,
    metallic: f32,
    roughness: f32,
    emissive: [3]f32,
    padding: f32, // for alignment
    textures: [4]?*anyopaque, // Texture handles
    texture_count: u32,
    has_base_color_texture: bool,
    has_normal_texture: bool,
    has_metallic_roughness_texture: bool,
    has_emissive_texture: bool,
};

/// Texture slot types
pub const TextureSlot = enum(u32) {
    BaseColor = 0,
    Normal = 1,
    MetallicRoughness = 2,
    Emissive = 3,
};

/// Create default material
export fn material_create() Material {
    return Material{
        .base_color = [_]f32{ 1.0, 1.0, 1.0, 1.0 },
        .metallic = 0.0,
        .roughness = 0.5,
        .emissive = [_]f32{ 0.0, 0.0, 0.0 },
        .padding = 0.0,
        .textures = [_]?*anyopaque{null} ** 4,
        .texture_count = 0,
        .has_base_color_texture = false,
        .has_normal_texture = false,
        .has_metallic_roughness_texture = false,
        .has_emissive_texture = false,
    };
}

/// Create material with color
export fn material_from_color(r: f32, g: f32, b: f32, a: f32) Material {
    var mat = material_create();
    mat.base_color = [_]f32{ r, g, b, a };
    return mat;
}

/// Create material with RGB color
export fn material_from_rgb(r: f32, g: f32, b: f32) Material {
    return material_from_color(r, g, b, 1.0);
}

/// Create metallic material
export fn material_metallic(r: f32, g: f32, b: f32, metallic: f32, roughness: f32) Material {
    var mat = material_from_rgb(r, g, b);
    mat.metallic = metallic;
    mat.roughness = roughness;
    return mat;
}

/// Set base color
export fn material_set_base_color(mat: *Material, r: f32, g: f32, b: f32, a: f32) void {
    mat.base_color = [_]f32{ r, g, b, a };
}

/// Set metallic
export fn material_set_metallic(mat: *Material, metallic: f32) void {
    mat.metallic = std.math.clamp(metallic, 0.0, 1.0);
}

/// Set roughness
export fn material_set_roughness(mat: *Material, roughness: f32) void {
    mat.roughness = std.math.clamp(roughness, 0.0, 1.0);
}

/// Set emissive color
export fn material_set_emissive(mat: *Material, r: f32, g: f32, b: f32) void {
    mat.emissive = [_]f32{ r, g, b };
}

/// Set texture at slot
export fn material_set_texture(mat: *Material, slot: u32, texture: ?*anyopaque) void {
    if (slot >= 4) return;

    const old_texture = mat.textures[slot];
    mat.textures[slot] = texture;

    // Update texture flags
    switch (slot) {
        0 => mat.has_base_color_texture = texture != null,
        1 => mat.has_normal_texture = texture != null,
        2 => mat.has_metallic_roughness_texture = texture != null,
        3 => mat.has_emissive_texture = texture != null,
        else => {},
    }

    // Update texture count
    if (old_texture == null and texture != null) {
        mat.texture_count += 1;
    } else if (old_texture != null and texture == null) {
        mat.texture_count -= 1;
    }
}

/// Get texture at slot
export fn material_get_texture(mat: *const Material, slot: u32) ?*anyopaque {
    if (slot >= 4) return null;
    return mat.textures[slot];
}

/// Check if has texture at slot
export fn material_has_texture(mat: *const Material, slot: u32) bool {
    if (slot >= 4) return false;
    return mat.textures[slot] != null;
}

/// Clear texture at slot
export fn material_clear_texture(mat: *Material, slot: u32) void {
    material_set_texture(mat, slot, null);
}

/// Clear all textures
export fn material_clear_all_textures(mat: *Material) void {
    var i: u32 = 0;
    while (i < 4) : (i += 1) {
        material_set_texture(mat, i, null);
    }
}

/// Get base color
export fn material_get_base_color(mat: *const Material, out_color: *[4]f32) void {
    @memcpy(out_color, &mat.base_color);
}

/// Get emissive color
export fn material_get_emissive(mat: *const Material, out_color: *[3]f32) void {
    @memcpy(out_color, &mat.emissive);
}

/// Get metallic value
export fn material_get_metallic(mat: *const Material) f32 {
    return mat.metallic;
}

/// Get roughness value
export fn material_get_roughness(mat: *const Material) f32 {
    return mat.roughness;
}

/// Get texture count
export fn material_get_texture_count(mat: *const Material) u32 {
    return mat.texture_count;
}

/// Check if material has any textures
export fn material_has_any_texture(mat: *const Material) bool {
    return mat.texture_count > 0;
}

/// Copy material
export fn material_copy(dest: *Material, src: *const Material) void {
    dest.base_color = src.base_color;
    dest.metallic = src.metallic;
    dest.roughness = src.roughness;
    dest.emissive = src.emissive;
    dest.textures = src.textures;
    dest.texture_count = src.texture_count;
    dest.has_base_color_texture = src.has_base_color_texture;
    dest.has_normal_texture = src.has_normal_texture;
    dest.has_metallic_roughness_texture = src.has_metallic_roughness_texture;
    dest.has_emissive_texture = src.has_emissive_texture;
}

/// Check if two materials are equal (ignoring textures)
export fn material_equals(a: *const Material, b: *const Material) bool {
    if (!std.mem.eql(f32, &a.base_color, &b.base_color)) return false;
    if (a.metallic != b.metallic) return false;
    if (a.roughness != b.roughness) return false;
    if (!std.mem.eql(f32, &a.emissive, &b.emissive)) return false;
    return true;
}
