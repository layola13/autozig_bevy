//! Comprehensive tests for autozig-shader module
//! Tests cover ShaderStage, ShaderSource, ShaderDef, ShaderModule, 
//! UniformLayout, BuiltinShaders, and BindGroupLayoutEntry

use autozig_shader::*;

// ============================================================================
// ShaderStage Tests
// ============================================================================

#[test]
fn test_shader_stage_vertex() {
    let stage = ShaderStage::Vertex;
    assert_eq!(stage.to_wgpu_flags(), 1);
}

#[test]
fn test_shader_stage_fragment() {
    let stage = ShaderStage::Fragment;
    assert_eq!(stage.to_wgpu_flags(), 2);
}

#[test]
fn test_shader_stage_compute() {
    let stage = ShaderStage::Compute;
    assert_eq!(stage.to_wgpu_flags(), 4);
}

#[test]
fn test_shader_stage_equality() {
    assert_eq!(ShaderStage::Vertex, ShaderStage::Vertex);
    assert_ne!(ShaderStage::Vertex, ShaderStage::Fragment);
    assert_ne!(ShaderStage::Fragment, ShaderStage::Compute);
}

// ============================================================================
// ShaderSource Tests
// ============================================================================

#[test]
fn test_shader_source_creation() {
    let source = ShaderSource::new("fn main() {}", ShaderStage::Vertex);
    assert_eq!(source.code, "fn main() {}");
    assert_eq!(source.stage, ShaderStage::Vertex);
    assert_eq!(source.entry_point, "main");
}

#[test]
fn test_shader_source_with_custom_entry_point() {
    let source = ShaderSource::new("fn vs_main() {}", ShaderStage::Vertex)
        .with_entry_point("vs_main");
    assert_eq!(source.entry_point, "vs_main");
}

#[test]
fn test_shader_source_as_bytes() {
    let source = ShaderSource::new("test", ShaderStage::Fragment);
    let bytes = source.as_bytes();
    assert_eq!(bytes, b"test");
}

#[test]
fn test_shader_source_empty_code() {
    let source = ShaderSource::new("", ShaderStage::Compute);
    assert_eq!(source.code.len(), 0);
    assert_eq!(source.as_bytes().len(), 0);
}

// ============================================================================
// ShaderDef Tests
// ============================================================================

#[test]
fn test_shader_def_simple() {
    let def = ShaderDef::new("USE_TEXTURE");
    assert_eq!(def.name, "USE_TEXTURE");
    assert!(def.value.is_none());
}

#[test]
fn test_shader_def_with_value() {
    let def = ShaderDef::with_value("MAX_LIGHTS", "32");
    assert_eq!(def.name, "MAX_LIGHTS");
    assert_eq!(def.value, Some("32".to_string()));
}

#[test]
fn test_shader_def_equality() {
    let def1 = ShaderDef::new("TEST");
    let def2 = ShaderDef::new("TEST");
    let def3 = ShaderDef::with_value("TEST", "1");
    
    assert_eq!(def1, def2);
    assert_ne!(def1, def3);
}

#[test]
fn test_shader_def_numeric_value() {
    let def = ShaderDef::with_value("WORK_GROUP_SIZE", "256");
    assert_eq!(def.value.as_ref().unwrap(), "256");
}

// ============================================================================
// Shader Tests
// ============================================================================

#[test]
fn test_shader_from_wgsl() {
    let shader = Shader::from_wgsl("@vertex fn main() {}", ShaderStage::Vertex);
    assert_eq!(shader.source.stage, ShaderStage::Vertex);
    assert!(shader.label.is_none());
    assert_eq!(shader.defs.len(), 0);
}

#[test]
fn test_shader_with_label() {
    let shader = Shader::from_wgsl("fn main() {}", ShaderStage::Fragment)
        .with_label("MyShader");
    assert_eq!(shader.label, Some("MyShader".to_string()));
}

#[test]
fn test_shader_with_def() {
    let shader = Shader::from_wgsl("fn main() {}", ShaderStage::Compute)
        .with_def(ShaderDef::new("USE_FEATURE"));
    assert_eq!(shader.defs.len(), 1);
    assert_eq!(shader.defs[0].name, "USE_FEATURE");
}

#[test]
fn test_shader_with_multiple_defs() {
    let shader = Shader::from_wgsl("fn main() {}", ShaderStage::Vertex)
        .with_def(ShaderDef::new("FEATURE_A"))
        .with_def(ShaderDef::with_value("FEATURE_B", "10"))
        .with_def(ShaderDef::new("FEATURE_C"));
    
    assert_eq!(shader.defs.len(), 3);
    assert_eq!(shader.defs[0].name, "FEATURE_A");
    assert_eq!(shader.defs[1].name, "FEATURE_B");
    assert_eq!(shader.defs[2].name, "FEATURE_C");
}

// ============================================================================
// ShaderModuleHandle Tests
// ============================================================================

#[test]
fn test_shader_module_handle_invalid() {
    let handle = ShaderModuleHandle::INVALID;
    assert_eq!(handle.id, 0);
    assert!(!handle.is_valid());
}

#[test]
fn test_shader_module_handle_valid() {
    let handle = ShaderModuleHandle { id: 42 };
    assert_eq!(handle.id, 42);
    assert!(handle.is_valid());
}

#[test]
fn test_shader_module_handle_zero_is_invalid() {
    let handle = ShaderModuleHandle { id: 0 };
    assert!(!handle.is_valid());
}

// ============================================================================
// UniformLayout Tests - std140 Alignment
// ============================================================================

#[test]
fn test_uniform_layout_std140_scalar() {
    let alignment = UniformLayout::std140_alignment(4); // f32
    assert_eq!(alignment, 4);
}

#[test]
fn test_uniform_layout_std140_vec2() {
    let alignment = UniformLayout::std140_alignment(8); // vec2
    assert_eq!(alignment, 8);
}

#[test]
fn test_uniform_layout_std140_vec3() {
    let alignment = UniformLayout::std140_alignment(12); // vec3
    assert_eq!(alignment, 16); // vec3 aligned as vec4
}

#[test]
fn test_uniform_layout_std140_vec4() {
    let alignment = UniformLayout::std140_alignment(16); // vec4
    assert_eq!(alignment, 16);
}

#[test]
fn test_uniform_layout_std140_mat4x4() {
    let alignment = UniformLayout::std140_alignment(64); // mat4x4
    assert_eq!(alignment, 16);
}

// ============================================================================
// UniformLayout Tests - std430 Alignment
// ============================================================================

#[test]
fn test_uniform_layout_std430_scalar() {
    let alignment = UniformLayout::std430_alignment(4);
    assert_eq!(alignment, 4);
}

#[test]
fn test_uniform_layout_std430_vec2() {
    let alignment = UniformLayout::std430_alignment(8);
    assert_eq!(alignment, 8);
}

#[test]
fn test_uniform_layout_std430_vec3() {
    let alignment = UniformLayout::std430_alignment(12);
    assert_eq!(alignment, 16);
}

#[test]
fn test_uniform_layout_std430_vec4() {
    let alignment = UniformLayout::std430_alignment(16);
    assert_eq!(alignment, 16);
}

// ============================================================================
// UniformLayout Tests - Offset Alignment
// ============================================================================

#[test]
fn test_uniform_layout_align_offset_zero() {
    let aligned = UniformLayout::align_offset(0, 16);
    assert_eq!(aligned, 0);
}

#[test]
fn test_uniform_layout_align_offset_round_up() {
    let aligned = UniformLayout::align_offset(1, 16);
    assert_eq!(aligned, 16);
    
    let aligned = UniformLayout::align_offset(15, 16);
    assert_eq!(aligned, 16);
}

#[test]
fn test_uniform_layout_align_offset_already_aligned() {
    let aligned = UniformLayout::align_offset(16, 16);
    assert_eq!(aligned, 16);
    
    let aligned = UniformLayout::align_offset(32, 16);
    assert_eq!(aligned, 32);
}

#[test]
fn test_uniform_layout_align_offset_multiple_alignments() {
    assert_eq!(UniformLayout::align_offset(5, 4), 8);
    assert_eq!(UniformLayout::align_offset(9, 8), 16);
    assert_eq!(UniformLayout::align_offset(17, 16), 32);
}

// ============================================================================
// UniformLayout Tests - Size Calculation
// ============================================================================

#[test]
fn test_uniform_layout_calculate_size_empty() {
    let fields: Vec<UniformField> = vec![];
    let size = UniformLayout::calculate_size(&fields);
    assert_eq!(size, 0);
}

#[test]
fn test_uniform_layout_calculate_size_single_field() {
    let fields = vec![
        UniformField {
            offset: 0,
            size: 16,
            alignment: 16,
            _padding: 0,
        },
    ];
    let size = UniformLayout::calculate_size(&fields);
    assert!(size >= 16);
    assert_eq!(size % 16, 0); // Must be 16-byte aligned
}

#[test]
fn test_uniform_layout_calculate_size_multiple_fields() {
    let fields = vec![
        UniformField { offset: 0, size: 64, alignment: 16, _padding: 0 },  // mat4x4
        UniformField { offset: 64, size: 16, alignment: 16, _padding: 0 }, // vec4
        UniformField { offset: 80, size: 4, alignment: 4, _padding: 0 },   // f32
    ];
    let size = UniformLayout::calculate_size(&fields);
    assert!(size >= 84);
    assert_eq!(size % 16, 0);
}

#[test]
fn test_uniform_layout_calculate_alignment_empty() {
    let fields: Vec<UniformField> = vec![];
    let alignment = UniformLayout::calculate_alignment(&fields);
    assert_eq!(alignment, 0);
}

#[test]
fn test_uniform_layout_calculate_alignment_single_field() {
    let fields = vec![
        UniformField {
            offset: 0,
            size: 16,
            alignment: 16,
            _padding: 0,
        },
    ];
    let alignment = UniformLayout::calculate_alignment(&fields);
    assert!(alignment >= 16);
}

// ============================================================================
// BuiltinShaders Tests
// ============================================================================

#[test]
fn test_builtin_shader_pbr_vertex() {
    let shader = BuiltinShaders::pbr_vertex();
    assert_eq!(shader.source.stage, ShaderStage::Vertex);
    assert!(!shader.source.code.is_empty());
    assert!(shader.source.code.contains("@vertex"));
    assert_eq!(shader.label, Some("PBR Vertex Shader".to_string()));
}

#[test]
fn test_builtin_shader_pbr_fragment() {
    let shader = BuiltinShaders::pbr_fragment();
    assert_eq!(shader.source.stage, ShaderStage::Fragment);
    assert!(!shader.source.code.is_empty());
    assert!(shader.source.code.contains("@fragment"));
    assert_eq!(shader.label, Some("PBR Fragment Shader".to_string()));
}

#[test]
fn test_builtin_shader_sprite_vertex() {
    let shader = BuiltinShaders::sprite_vertex();
    assert_eq!(shader.source.stage, ShaderStage::Vertex);
    assert!(!shader.source.code.is_empty());
    assert!(shader.source.code.contains("@vertex"));
    assert_eq!(shader.label, Some("Sprite Vertex Shader".to_string()));
}

#[test]
fn test_builtin_shader_sprite_fragment() {
    let shader = BuiltinShaders::sprite_fragment();
    assert_eq!(shader.source.stage, ShaderStage::Fragment);
    assert!(!shader.source.code.is_empty());
    assert!(shader.source.code.contains("@fragment"));
    assert_eq!(shader.label, Some("Sprite Fragment Shader".to_string()));
}

#[test]
fn test_builtin_shader_ui_vertex() {
    let shader = BuiltinShaders::ui_vertex();
    assert_eq!(shader.source.stage, ShaderStage::Vertex);
    assert!(!shader.source.code.is_empty());
    assert!(shader.source.code.contains("@vertex"));
    assert_eq!(shader.label, Some("UI Vertex Shader".to_string()));
}

#[test]
fn test_builtin_shader_ui_fragment() {
    let shader = BuiltinShaders::ui_fragment();
    assert_eq!(shader.source.stage, ShaderStage::Fragment);
    assert!(!shader.source.code.is_empty());
    assert!(shader.source.code.contains("@fragment"));
    assert_eq!(shader.label, Some("UI Fragment Shader".to_string()));
}

#[test]
fn test_builtin_shader_fullscreen_vertex() {
    let shader = BuiltinShaders::fullscreen_vertex();
    assert_eq!(shader.source.stage, ShaderStage::Vertex);
    assert!(!shader.source.code.is_empty());
    assert!(shader.source.code.contains("@vertex"));
    assert_eq!(shader.label, Some("Fullscreen Vertex Shader".to_string()));
}

#[test]
fn test_builtin_shaders_contain_wgsl_syntax() {
    let pbr_vertex = BuiltinShaders::pbr_vertex();
    assert!(pbr_vertex.source.code.contains("struct"));
    assert!(pbr_vertex.source.code.contains("fn main"));
    
    let sprite_frag = BuiltinShaders::sprite_fragment();
    assert!(sprite_frag.source.code.contains("texture"));
    assert!(sprite_frag.source.code.contains("sampler"));
}

// ============================================================================
// BindGroupLayoutEntry Tests
// ============================================================================

#[test]
fn test_bind_group_layout_entry_creation() {
    let entry = BindGroupLayoutEntry {
        binding: 0,
        visibility: BindingVisibility::Vertex as u32,
        buffer_type: BufferBindingType::Uniform as u32,
        min_binding_size: 256,
    };
    
    assert_eq!(entry.binding, 0);
    assert_eq!(entry.visibility, 1);
    assert_eq!(entry.buffer_type, 0);
    assert_eq!(entry.min_binding_size, 256);
}

#[test]
fn test_bind_group_layout_vertex_fragment_visibility() {
    let entries = UniformLayout::create_bind_group_layout(BindingVisibility::VertexFragment);
    assert!(!entries.is_empty());
    assert_eq!(entries[0].visibility, BindingVisibility::VertexFragment as u32);
}

#[test]
fn test_bind_group_layout_all_visibility() {
    let entries = UniformLayout::create_bind_group_layout(BindingVisibility::All);
    assert!(!entries.is_empty());
    assert_eq!(entries[0].visibility, BindingVisibility::All as u32);
}

#[test]
fn test_bind_group_layout_multiple_entries() {
    let entries = UniformLayout::create_bind_group_layout(BindingVisibility::VertexFragment);
    assert!(entries.len() >= 2);
    
    // Verify bindings are sequential
    for (i, entry) in entries.iter().enumerate() {
        assert_eq!(entry.binding, i as u32);
    }
}

// ============================================================================
// BindingVisibility Tests
// ============================================================================

#[test]
fn test_binding_visibility_values() {
    assert_eq!(BindingVisibility::None as u32, 0);
    assert_eq!(BindingVisibility::Vertex as u32, 1);
    assert_eq!(BindingVisibility::Fragment as u32, 2);
    assert_eq!(BindingVisibility::Compute as u32, 4);
    assert_eq!(BindingVisibility::VertexFragment as u32, 3);
    assert_eq!(BindingVisibility::All as u32, 7);
}

#[test]
fn test_buffer_binding_type_values() {
    assert_eq!(BufferBindingType::Uniform as u32, 0);
    assert_eq!(BufferBindingType::Storage as u32, 1);
    assert_eq!(BufferBindingType::ReadOnlyStorage as u32, 2);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_complete_shader_pipeline() {
    // Create a shader with all features
    let shader = Shader::from_wgsl(
        "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }",
        ShaderStage::Vertex
    )
    .with_label("IntegrationTestShader")
    .with_def(ShaderDef::new("USE_TEXTURE"))
    .with_def(ShaderDef::with_value("MAX_LIGHTS", "8"));
    
    assert_eq!(shader.label, Some("IntegrationTestShader".to_string()));
    assert_eq!(shader.defs.len(), 2);
    assert_eq!(shader.source.stage, ShaderStage::Vertex);
    assert!(!shader.source.code.is_empty());
}

#[test]
fn test_uniform_buffer_layout_pipeline() {
    // Create a typical uniform buffer layout
    let fields = vec![
        UniformField { offset: 0, size: 64, alignment: 16, _padding: 0 },   // view_proj matrix
        UniformField { offset: 64, size: 64, alignment: 16, _padding: 0 },  // model matrix
        UniformField { offset: 128, size: 16, alignment: 16, _padding: 0 }, // color vec4
        UniformField { offset: 144, size: 4, alignment: 4, _padding: 0 },   // roughness f32
    ];
    
    let size = UniformLayout::calculate_size(&fields);
    let alignment = UniformLayout::calculate_alignment(&fields);
    
    assert!(size >= 148);
    assert_eq!(size % 16, 0);
    assert!(alignment >= 16);
}