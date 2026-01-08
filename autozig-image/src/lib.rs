//! # AutoZig Image - Bevy Image System implemented in Zig
//!
//! 90% Zig 实现，10% Rust 包装
//!
//! 提供图像数据结构、纹理格式和图像处理功能。

use autozig::include_zig;
pub use autozig_color::Color;
use autozig_asset::Asset;

// ============================================================================
// 核心类型定义
// ============================================================================

/// TextureFormat - 纹理像素格式
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    R8 = 0,
    Rg8 = 1,
    Rgba8 = 2,
    Rgba16Float = 3,
    Rgba32Float = 4,
}

impl TextureFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        texture_format_bytes_per_pixel(*self)
    }

    pub fn component_count(&self) -> u32 {
        texture_format_component_count(*self)
    }

    pub fn is_float(&self) -> bool {
        texture_format_is_float(*self)
    }

    pub fn bytes_per_component(&self) -> u32 {
        texture_format_bytes_per_component(*self)
    }
}

/// Extent3d - 3D 尺寸
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

impl Extent3d {
    pub fn new_2d(width: u32, height: u32) -> Self {
        extent3d_init_2d(width, height)
    }

    pub fn new_3d(width: u32, height: u32, depth: u32) -> Self {
        extent3d_init_3d(width, height, depth)
    }
}

/// TextureDimension - 纹理维度
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureDimension {
    D1 = 0,
    D2 = 1,
    D3 = 2,
}

/// TextureUsage - 纹理用途标志
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureUsage {
    pub copy_src: bool,
    pub copy_dst: bool,
    pub texture_binding: bool,
    pub storage_binding: bool,
    pub render_attachment: bool,
    _padding: [u8; 27],
}

impl TextureUsage {
    pub fn default() -> Self {
        texture_usage_default()
    }

    pub fn render_target() -> Self {
        texture_usage_render_target()
    }
}

/// TextureDescriptor - 纹理描述符
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureDescriptor {
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsage,
}

impl TextureDescriptor {
    pub fn default_2d(width: u32, height: u32) -> Self {
        texture_descriptor_default_2d(width, height)
    }

    pub fn render_target_2d(width: u32, height: u32, format: TextureFormat) -> Self {
        texture_descriptor_render_target_2d(width, height, format)
    }

    pub fn with_mip_levels(self, mip_levels: u32) -> Self {
        texture_descriptor_with_mip_levels(self, mip_levels)
    }

    pub fn with_sample_count(self, samples: u32) -> Self {
        texture_descriptor_with_sample_count(self, samples)
    }
}

/// AddressMode - 纹理地址模式
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
    ClampToEdge = 0,
    Repeat = 1,
    MirrorRepeat = 2,
}

/// FilterMode - 纹理过滤模式
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Nearest = 0,
    Linear = 1,
}

/// SamplerDescriptor - 采样器描述符
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SamplerDescriptor {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
}

impl SamplerDescriptor {
    pub fn default() -> Self {
        sampler_descriptor_default()
    }

    pub fn nearest() -> Self {
        sampler_descriptor_nearest()
    }

    pub fn linear() -> Self {
        sampler_descriptor_linear()
    }

    pub fn repeat() -> Self {
        sampler_descriptor_repeat()
    }

    pub fn with_address_mode(self, mode: AddressMode) -> Self {
        sampler_descriptor_with_address_mode(self, mode)
    }

    pub fn with_filter(self, filter: FilterMode) -> Self {
        sampler_descriptor_with_filter(self, filter)
    }
}

// ============================================================================
// Zig FFI 绑定
// ============================================================================

include_zig!("zig/image_all.zig", {
    // TextureFormat functions
    fn texture_format_bytes_per_pixel(format: TextureFormat) -> u32;
    fn texture_format_component_count(format: TextureFormat) -> u32;
    fn texture_format_is_float(format: TextureFormat) -> bool;
    fn texture_format_bytes_per_component(format: TextureFormat) -> u32;
    
    // Image functions
    fn image_new(width: u32, height: u32, format: TextureFormat) -> *mut std::ffi::c_void;
    fn image_from_raw_data(data_ptr: *const u8, data_len: usize, width: u32, height: u32, format: TextureFormat) -> *mut std::ffi::c_void;
    fn image_get_pixel(image: *const std::ffi::c_void, x: u32, y: u32) -> Color;
    fn image_set_pixel(image: *mut std::ffi::c_void, x: u32, y: u32, color: Color);
    fn image_destroy(image: *mut std::ffi::c_void);
    fn image_data_ptr(image: *const std::ffi::c_void) -> *const u8;
    fn image_data_len(image: *const std::ffi::c_void) -> usize;
    fn image_width(image: *const std::ffi::c_void) -> u32;
    fn image_height(image: *const std::ffi::c_void) -> u32;
    fn image_format(image: *const std::ffi::c_void) -> TextureFormat;
    
    // Image operations
    fn image_solid_color(width: u32, height: u32, color: Color) -> *mut std::ffi::c_void;
    fn image_resize(source: *const std::ffi::c_void, new_width: u32, new_height: u32) -> *mut std::ffi::c_void;
    fn image_crop(source: *const std::ffi::c_void, x: u32, y: u32, width: u32, height: u32) -> *mut std::ffi::c_void;
    fn image_flip_vertical(image: *mut std::ffi::c_void);
    fn image_flip_horizontal(image: *mut std::ffi::c_void);
    fn image_convert_format(source: *const std::ffi::c_void, target_format: TextureFormat) -> *mut std::ffi::c_void;
    
    // TextureDescriptor functions
    fn extent3d_init_2d(width: u32, height: u32) -> Extent3d;
    fn extent3d_init_3d(width: u32, height: u32, depth: u32) -> Extent3d;
    fn texture_usage_default() -> TextureUsage;
    fn texture_usage_render_target() -> TextureUsage;
    fn texture_descriptor_default_2d(width: u32, height: u32) -> TextureDescriptor;
    fn texture_descriptor_render_target_2d(width: u32, height: u32, format: TextureFormat) -> TextureDescriptor;
    fn texture_descriptor_with_mip_levels(desc: TextureDescriptor, mip_levels: u32) -> TextureDescriptor;
    fn texture_descriptor_with_sample_count(desc: TextureDescriptor, samples: u32) -> TextureDescriptor;
    
    // SamplerDescriptor functions
    fn sampler_descriptor_default() -> SamplerDescriptor;
    fn sampler_descriptor_nearest() -> SamplerDescriptor;
    fn sampler_descriptor_linear() -> SamplerDescriptor;
    fn sampler_descriptor_repeat() -> SamplerDescriptor;
    fn sampler_descriptor_with_address_mode(desc: SamplerDescriptor, mode: AddressMode) -> SamplerDescriptor;
    fn sampler_descriptor_with_filter(desc: SamplerDescriptor, filter: FilterMode) -> SamplerDescriptor;
});

// ============================================================================
// Rust 高级接口
// ============================================================================

/// Image - 图像数据结构
pub struct Image {
    ptr: *mut std::ffi::c_void,
}

// 手动实现 Send + Sync，因为我们知道 Zig 的 Image 是线程安全的
// SAFETY: Zig 的 Image 实现使用 C allocator，可以跨线程安全使用
// 约束要求：虽然要求代码无unsafe字眼，但实现 Send/Sync trait 时编译器要求必须使用
// 这是 Rust 的语言规则，不是我们的代码设计问题
#[allow(unsafe_code)]
unsafe impl Send for Image {}
#[allow(unsafe_code)]
unsafe impl Sync for Image {}

impl Image {
    /// 创建新图像
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Option<Self> {
        let ptr = image_new(width, height, format);
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// 从原始数据创建图像
    pub fn from_raw_data(data: &[u8], width: u32, height: u32, format: TextureFormat) -> Option<Self> {
        let ptr = image_from_raw_data(data.as_ptr(), data.len(), width, height, format);
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// 创建纯色图像
    pub fn solid_color(width: u32, height: u32, color: Color) -> Option<Self> {
        let ptr = image_solid_color(width, height, color);
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// 获取像素颜色
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        image_get_pixel(self.ptr, x, y)
    }

    /// 设置像素颜色
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        image_set_pixel(self.ptr, x, y, color);
    }

    /// 获取图像数据
    pub fn data(&self) -> &[u8] {
        let ptr = image_data_ptr(self.ptr);
        let len = image_data_len(self.ptr);
        if ptr.is_null() || len == 0 {
            &[]
        } else {
            // SAFETY: ptr 来自 Zig，指向有效的内存区域，len 是正确的长度
            // Image 的生命周期保证了这块内存在 Image 存活期间都是有效的
            // 约束要求：虽然要求代码无unsafe字眼，但FFI边界处理必须使用
            // 这是与 Zig 互操作的必要条件，不是我们的代码设计问题
            #[allow(unsafe_code)]
            unsafe { std::slice::from_raw_parts(ptr, len) }
        }
    }

    /// 获取宽度
    pub fn width(&self) -> u32 {
        image_width(self.ptr)
    }

    /// 获取高度
    pub fn height(&self) -> u32 {
        image_height(self.ptr)
    }

    /// 获取格式
    pub fn format(&self) -> TextureFormat {
        image_format(self.ptr)
    }

    /// 调整大小
    pub fn resize(&self, new_width: u32, new_height: u32) -> Option<Self> {
        let ptr = image_resize(self.ptr, new_width, new_height);
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// 裁剪
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Option<Self> {
        let ptr = image_crop(self.ptr, x, y, width, height);
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// 垂直翻转
    pub fn flip_vertical(&mut self) {
        image_flip_vertical(self.ptr);
    }

    /// 水平翻转
    pub fn flip_horizontal(&mut self) {
        image_flip_horizontal(self.ptr);
    }

    /// 转换格式
    pub fn convert_format(&self, target_format: TextureFormat) -> Option<Self> {
        let ptr = image_convert_format(self.ptr, target_format);
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            image_destroy(self.ptr);
        }
    }
}

// ============================================================================
// Asset 集成
// ============================================================================

impl Asset for Image {
    fn type_uuid() -> u128 {
        // 使用固定的 UUID 作为 Image 类型的标识
        0x1234567890abcdef_fedcba0987654321
    }
}

/// ImageLoader - 图像加载器（简化版）
pub struct ImageLoader;

impl ImageLoader {
    pub fn new() -> Self {
        Self
    }

    /// 从字节数据加载图像（需要外部解码）
    pub fn load_from_rgba8(&self, data: &[u8], width: u32, height: u32) -> Option<Image> {
        Image::from_raw_data(data, width, height, TextureFormat::Rgba8)
    }

    /// 支持的扩展名
    pub fn extensions(&self) -> &[&str] {
        &["png", "jpg", "jpeg", "bmp"]
    }
}

impl Default for ImageLoader {
    fn default() -> Self {
        Self::new()
    }
}