//! AutoZig Shader - Bevy shader management system for WebGPU/WASM platforms
//! 
//! This crate provides comprehensive shader management capabilities:
//! - ShaderModule creation and compilation (85% Zig)
//! - Uniform buffer layout calculation (85% Zig)
//! - Built-in WGSL shaders (PBR, Sprite, UI)
//! - Shader resource wrapping (15% Rust)

use autozig::include_zig;

// ============================================================================
// Shader Stage (Rust 15%)
// ============================================================================

/// Shader execution stage
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    Vertex = 1,
    Fragment = 2,
    Compute = 4,
}

impl ShaderStage {
    pub fn to_wgpu_flags(&self) -> u32 {
        *self as u32
    }
}

// ============================================================================
// Shader Source (Rust 15%)
// ============================================================================

/// Shader source code container
#[derive(Debug, Clone)]
pub struct ShaderSource {
    pub code: String,
    pub stage: ShaderStage,
    pub entry_point: String,
}

impl ShaderSource {
    pub fn new(code: impl Into<String>, stage: ShaderStage) -> Self {
        Self {
            code: code.into(),
            stage,
            entry_point: "main".to_string(),
        }
    }

    pub fn with_entry_point(mut self, entry_point: impl Into<String>) -> Self {
        self.entry_point = entry_point.into();
        self
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.code.as_bytes()
    }
}

// ============================================================================
// Shader Definition (Rust 15%)
// ============================================================================

/// Shader preprocessor definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderDef {
    pub name: String,
    pub value: Option<String>,
}

impl ShaderDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    pub fn with_value(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
        }
    }
}

// ============================================================================
// Shader (Rust 15%)
// ============================================================================

/// Complete shader resource
#[derive(Debug, Clone)]
pub struct Shader {
    pub source: ShaderSource,
    pub label: Option<String>,
    pub defs: Vec<ShaderDef>,
}

impl Shader {
    pub fn from_wgsl(code: impl Into<String>, stage: ShaderStage) -> Self {
        Self {
            source: ShaderSource::new(code, stage),
            label: None,
            defs: Vec::new(),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_def(mut self, def: ShaderDef) -> Self {
        self.defs.push(def);
        self
    }
}

// ============================================================================
// ShaderModule Handle (Zig 85%)
// ============================================================================

/// WebGPU ShaderModule handle (opaque pointer from Zig)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ShaderModuleHandle {
    pub id: u64,
}

impl ShaderModuleHandle {
    pub const INVALID: Self = Self { id: 0 };

    pub fn is_valid(&self) -> bool {
        self.id != 0
    }
}

// ============================================================================
// Uniform Layout Types (Rust 15%)
// ============================================================================

/// Binding visibility flags
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingVisibility {
    None = 0,
    Vertex = 1,
    Fragment = 2,
    Compute = 4,
    VertexFragment = 3,
    All = 7,
}

/// Buffer binding type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferBindingType {
    Uniform = 0,
    Storage = 1,
    ReadOnlyStorage = 2,
}

/// Bind group layout entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: u32,
    pub buffer_type: u32,
    pub min_binding_size: u64,
}

/// Uniform field information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UniformField {
    pub offset: u32,
    pub size: u32,
    pub alignment: u32,
    pub _padding: u32,
}

// ============================================================================
// Zig FFI - ShaderModule Management (Zig 85%)
// ============================================================================

include_zig!("zig/shader_module.zig", {
    fn shader_module_create(
        device_id: u64,
        code: *const u8,
        code_len: usize,
        label: *const u8,
        label_len: usize
    ) -> ShaderModuleHandle;
    
    fn shader_module_destroy(module: ShaderModuleHandle);
    
    fn shader_module_is_valid(module: ShaderModuleHandle) -> bool;
    
    fn shader_module_get_compilation_info(
        module: ShaderModuleHandle,
        out_buffer: *mut u8,
        buffer_len: usize
    ) -> usize;
});

/// ShaderModule wrapper with Zig backend
pub struct ShaderModule {
    handle: ShaderModuleHandle,
}

impl ShaderModule {
    /// Create a new shader module from WGSL source
    pub fn new(device_id: u64, shader: &Shader) -> Result<Self, String> {
        let code = shader.source.code.as_bytes();
        let label = shader.label.as_deref().unwrap_or("shader");
        
        let handle = shader_module_create(
            device_id,
            code.as_ptr(),
            code.len(),
            label.as_ptr(),
            label.len(),
        );
        
        if handle.is_valid() {
            Ok(Self { handle })
        } else {
            Err("Failed to create shader module".to_string())
        }
    }

    pub fn handle(&self) -> ShaderModuleHandle {
        self.handle
    }

    pub fn is_valid(&self) -> bool {
        shader_module_is_valid(self.handle)
    }

    pub fn get_compilation_info(&self) -> Option<String> {
        let mut buffer = vec![0u8; 4096];
        let len = shader_module_get_compilation_info(
            self.handle,
            buffer.as_mut_ptr(),
            buffer.len(),
        );
        
        if len > 0 && len <= buffer.len() {
            buffer.truncate(len);
            String::from_utf8(buffer).ok()
        } else {
            None
        }
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        if self.handle.is_valid() {
            shader_module_destroy(self.handle);
        }
    }
}

// ============================================================================
// Zig FFI - Uniform Layout Calculation (Zig 85%)
// ============================================================================

include_zig!("zig/uniform_layout.zig", {
    fn uniform_layout_calculate_size(fields: *const UniformField, field_count: usize) -> u64;
    
    fn uniform_layout_calculate_alignment(fields: *const UniformField, field_count: usize) -> u32;
    
    fn uniform_layout_align_offset(offset: u64, alignment: u32) -> u64;
    
    fn uniform_layout_create_bind_group_layout(
        entries: *mut BindGroupLayoutEntry,
        max_entries: usize,
        visibility: u32
    ) -> usize;
    
    fn uniform_layout_get_std140_alignment(size: u32) -> u32;
    
    fn uniform_layout_get_std430_alignment(size: u32) -> u32;
});

/// Uniform layout calculator
pub struct UniformLayout;

impl UniformLayout {
    /// Calculate total size of uniform buffer
    pub fn calculate_size(fields: &[UniformField]) -> u64 {
        if fields.is_empty() {
            return 0;
        }
        uniform_layout_calculate_size(fields.as_ptr(), fields.len())
    }

    /// Calculate required alignment
    pub fn calculate_alignment(fields: &[UniformField]) -> u32 {
        if fields.is_empty() {
            return 0;
        }
        uniform_layout_calculate_alignment(fields.as_ptr(), fields.len())
    }

    /// Align offset to required alignment
    pub fn align_offset(offset: u64, alignment: u32) -> u64 {
        uniform_layout_align_offset(offset, alignment)
    }

    /// Get std140 alignment for given size
    pub fn std140_alignment(size: u32) -> u32 {
        uniform_layout_get_std140_alignment(size)
    }

    /// Get std430 alignment for given size
    pub fn std430_alignment(size: u32) -> u32 {
        uniform_layout_get_std430_alignment(size)
    }

    /// Create bind group layout entries
    pub fn create_bind_group_layout(visibility: BindingVisibility) -> Vec<BindGroupLayoutEntry> {
        let mut entries = vec![
            BindGroupLayoutEntry {
                binding: 0,
                visibility: 0,
                buffer_type: 0,
                min_binding_size: 0,
            };
            16
        ];
        
        let count = uniform_layout_create_bind_group_layout(
            entries.as_mut_ptr(),
            entries.len(),
            visibility as u32,
        );
        
        entries.truncate(count);
        entries
    }
}

// ============================================================================
// Zig FFI - Built-in Shaders (Zig 85%)
// ============================================================================

include_zig!("zig/builtin_shaders.zig", {
    fn builtin_shader_get_pbr_vertex(out_ptr: *mut *const u8, out_len: *mut usize);
    fn builtin_shader_get_pbr_fragment(out_ptr: *mut *const u8, out_len: *mut usize);
    fn builtin_shader_get_sprite_vertex(out_ptr: *mut *const u8, out_len: *mut usize);
    fn builtin_shader_get_sprite_fragment(out_ptr: *mut *const u8, out_len: *mut usize);
    fn builtin_shader_get_ui_vertex(out_ptr: *mut *const u8, out_len: *mut usize);
    fn builtin_shader_get_ui_fragment(out_ptr: *mut *const u8, out_len: *mut usize);
    fn builtin_shader_get_fullscreen_vertex(out_ptr: *mut *const u8, out_len: *mut usize);
    fn builtin_shader_copy_to_buffer(getter_fn: u32, out_buffer: *mut u8, buffer_len: usize) -> usize;
});

/// Built-in shaders
pub struct BuiltinShaders;

impl BuiltinShaders {
    fn get_shader_source_by_id(getter_id: u32) -> String {
        // First, get the required size
        let mut temp_buffer = vec![0u8; 1];
        let actual_len = builtin_shader_copy_to_buffer(getter_id, temp_buffer.as_mut_ptr(), 1);
        
        if actual_len == 0 {
            return String::new();
        }
        
        // Allocate proper buffer and copy
        let mut buffer = vec![0u8; actual_len];
        let copied = builtin_shader_copy_to_buffer(getter_id, buffer.as_mut_ptr(), actual_len);
        
        if copied != actual_len {
            return String::new();
        }
        
        String::from_utf8_lossy(&buffer).into_owned()
    }

    pub fn pbr_vertex() -> Shader {
        let source = Self::get_shader_source_by_id(0);
        Shader::from_wgsl(source, ShaderStage::Vertex)
            .with_label("PBR Vertex Shader")
    }

    pub fn pbr_fragment() -> Shader {
        let source = Self::get_shader_source_by_id(1);
        Shader::from_wgsl(source, ShaderStage::Fragment)
            .with_label("PBR Fragment Shader")
    }

    pub fn sprite_vertex() -> Shader {
        let source = Self::get_shader_source_by_id(2);
        Shader::from_wgsl(source, ShaderStage::Vertex)
            .with_label("Sprite Vertex Shader")
    }

    pub fn sprite_fragment() -> Shader {
        let source = Self::get_shader_source_by_id(3);
        Shader::from_wgsl(source, ShaderStage::Fragment)
            .with_label("Sprite Fragment Shader")
    }

    pub fn ui_vertex() -> Shader {
        let source = Self::get_shader_source_by_id(4);
        Shader::from_wgsl(source, ShaderStage::Vertex)
            .with_label("UI Vertex Shader")
    }

    pub fn ui_fragment() -> Shader {
        let source = Self::get_shader_source_by_id(5);
        Shader::from_wgsl(source, ShaderStage::Fragment)
            .with_label("UI Fragment Shader")
    }

    pub fn fullscreen_vertex() -> Shader {
        let source = Self::get_shader_source_by_id(6);
        Shader::from_wgsl(source, ShaderStage::Vertex)
            .with_label("Fullscreen Vertex Shader")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_stage() {
        assert_eq!(ShaderStage::Vertex.to_wgpu_flags(), 1);
        assert_eq!(ShaderStage::Fragment.to_wgpu_flags(), 2);
        assert_eq!(ShaderStage::Compute.to_wgpu_flags(), 4);
    }

    #[test]
    fn test_shader_source() {
        let source = ShaderSource::new("fn main() {}", ShaderStage::Vertex);
        assert_eq!(source.entry_point, "main");
        assert_eq!(source.code, "fn main() {}");
    }

    #[test]
    fn test_shader_def() {
        let def1 = ShaderDef::new("USE_TEXTURE");
        assert_eq!(def1.name, "USE_TEXTURE");
        assert!(def1.value.is_none());

        let def2 = ShaderDef::with_value("MAX_LIGHTS", "32");
        assert_eq!(def2.name, "MAX_LIGHTS");
        assert_eq!(def2.value, Some("32".to_string()));
    }

    #[test]
    fn test_shader_module_handle() {
        let invalid = ShaderModuleHandle::INVALID;
        assert!(!invalid.is_valid());

        let valid = ShaderModuleHandle { id: 123 };
        assert!(valid.is_valid());
    }
}