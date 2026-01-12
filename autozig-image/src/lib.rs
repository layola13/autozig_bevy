
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

/// TextureFormat - 纹理像素格式（完整的GPU格式支持）
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    // 基础格式
    R8 = 0,
    Rg8 = 1,
    Rgba8 = 2,
    // 扩展GPU格式
    R8Unorm = 5,
    R8Snorm = 6,
    R8Uint = 7,
    R8Sint = 8,
    R16Uint = 9,
    R16Sint = 10,
    R16Unorm = 11,
    R16Snorm = 12,
    R16Float = 13,
    Rg8Unorm = 14,
    Rg8Snorm = 15,
    Rg8Uint = 16,
    Rg8Sint = 17,
    R32Uint = 18,
    R32Sint = 19,
    R32Float = 20,
    Rg16Uint = 21,
    Rg16Sint = 22,
    Rg16Unorm = 23,
    Rg16Snorm = 24,
    Rg16Float = 25,
    Rgba8Unorm = 26,
    Rgba8UnormSrgb = 27,
    Rgba8Snorm = 28,
    Rgba8Uint = 29,
    Rgba8Sint = 30,
    Bgra8Unorm = 31,
    Bgra8UnormSrgb = 32,
    Rgb10a2Uint = 33,
    Rgb10a2Unorm = 34,
    Rg11b10Float = 35,
    Rg32Uint = 36,
    Rg32Sint = 37,
    Rg32Float = 38,
    Rgba16Uint = 39,
    Rgba16Sint = 40,
    Rgba16Unorm = 41,
    Rgba16Snorm = 42,
    Rgba16Float = 43,
    Rgba32Uint = 44,
    Rgba32Sint = 45,
    Rgba32Float = 46,
    // 深度/模板格式
    Depth32Float = 47,
    Depth32FloatStencil8 = 48,
    Depth24Plus = 49,
    Depth24PlusStencil8 = 50,
    // 压缩格式 - BC
    Bc1RgbaUnorm = 51,
    Bc1RgbaUnormSrgb = 52,
    Bc2RgbaUnorm = 53,
    Bc2RgbaUnormSrgb = 54,
    Bc3RgbaUnorm = 55,
    Bc3RgbaUnormSrgb = 56,
    Bc4RUnorm = 57,
    Bc4RSnorm = 58,
    Bc5RgUnorm = 59,
    Bc5RgSnorm = 60,
    Bc6hRgbUfloat = 61,
    Bc6hRgbFloat = 62,
    Bc7RgbaUnorm = 63,
    Bc7RgbaUnormSrgb = 64,
    // 压缩格式 - ETC2
    Etc2Rgb8Unorm = 65,
    Etc2Rgb8UnormSrgb = 66,
    Etc2Rgb8A1Unorm = 67,
    Etc2Rgb8A1UnormSrgb = 68,
    Etc2Rgba8Unorm = 69,
    Etc2Rgba8UnormSrgb = 70,
    EacR11Unorm = 71,
    EacR11Snorm = 72,
    EacRg11Unorm = 73,
    EacRg11Snorm = 74,
    // 压缩格式 - ASTC
    Astc4x4Unorm = 75,
    Astc4x4UnormSrgb = 76,
    Astc5x4Unorm = 77,
    Astc5x4UnormSrgb = 78,
    Astc5x5Unorm = 79,
    Astc5x5UnormSrgb = 80,
    Astc6x5Unorm = 81,
    Astc6x5UnormSrgb = 82,
    Astc6x6Unorm = 83,
    Astc6x6UnormSrgb = 84,
    Astc8x5Unorm = 85,
    Astc8x5UnormSrgb = 86,
    Astc8x6Unorm = 87,
    Astc8x6UnormSrgb = 88,
    Astc8x8Unorm = 89,
    Astc8x8UnormSrgb = 90,
    Astc10x5Unorm = 91,
    Astc10x5UnormSrgb = 92,
    Astc10x6Unorm = 93,
    Astc10x6UnormSrgb = 94,
    Astc10x8Unorm = 95,
    Astc10x8UnormSrgb = 96,
    Astc10x10Unorm = 97,
    Astc10x10UnormSrgb = 98,
    Astc12x10Unorm = 99,
    Astc12x10UnormSrgb = 100,
    Astc12x12Unorm = 101,
    Astc12x12UnormSrgb = 102,
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
// 新增：完整的Image API类型（44个API）
// ============================================================================

/// ImageAddressMode - 图像寻址模式
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageAddressMode {
    #[default]
    ClampToEdge = 0,
    Repeat = 1,
    MirrorRepeat = 2,
    ClampToBorder = 3,
}

/// ImageFilterMode - 图像过滤模式
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFilterMode {
    #[default]
    Nearest = 0,
    Linear = 1,
}

/// ImageCompareFunction - 比较函数
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageCompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

/// ImageSamplerBorderColor - 边框颜色
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageSamplerBorderColor {
    TransparentBlack = 0,
    OpaqueBlack = 1,
    OpaqueWhite = 2,
    Zero = 3,
}

/// ImageSamplerDescriptor - 图像采样器描述符
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ImageSamplerDescriptor {
    pub address_mode_u: ImageAddressMode,
    pub address_mode_v: ImageAddressMode,
    pub address_mode_w: ImageAddressMode,
    pub mag_filter: ImageFilterMode,
    pub min_filter: ImageFilterMode,
    pub mipmap_filter: ImageFilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<ImageCompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<ImageSamplerBorderColor>,
}

impl Default for ImageSamplerDescriptor {
    fn default() -> Self {
        Self {
            address_mode_u: ImageAddressMode::ClampToEdge,
            address_mode_v: ImageAddressMode::ClampToEdge,
            address_mode_w: ImageAddressMode::ClampToEdge,
            mag_filter: ImageFilterMode::Nearest,
            min_filter: ImageFilterMode::Nearest,
            mipmap_filter: ImageFilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }
}

impl ImageSamplerDescriptor {
    pub fn linear() -> Self {
        Self {
            mag_filter: ImageFilterMode::Linear,
            min_filter: ImageFilterMode::Linear,
            mipmap_filter: ImageFilterMode::Linear,
            ..Default::default()
        }
    }

    pub fn nearest() -> Self {
        Self::default()
    }
}

/// ImageSampler - 图像采样器枚举
#[derive(Debug, Clone, Default)]
pub enum ImageSampler {
    #[default]
    Default,
    Descriptor(ImageSamplerDescriptor),
}

impl ImageSampler {
    pub fn linear() -> Self {
        ImageSampler::Descriptor(ImageSamplerDescriptor::linear())
    }

    pub fn nearest() -> Self {
        ImageSampler::Descriptor(ImageSamplerDescriptor::nearest())
    }
}

/// ImageFormat - 图像文件格式
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Basis = 0,
    Bmp = 1,
    Dds = 2,
    Farbfeld = 3,
    Gif = 4,
    OpenExr = 5,
    Hdr = 6,
    Ico = 7,
    Jpeg = 8,
    Ktx2 = 9,
    Png = 10,
    Pnm = 11,
    Qoi = 12,
    Tga = 13,
    Tiff = 14,
    WebP = 15,
}

/// ImageFormatSetting - 图像格式设置
#[derive(Debug, Clone, Default)]
pub enum ImageFormatSetting {
    #[default]
    FromExtension,
    Format(ImageFormat),
    Guess,
}

/// ImageArrayLayout - 图像数组布局
#[derive(Debug, Clone, Copy)]
pub enum ImageArrayLayout {
    RowCount { rows: u32 },
    RowHeight { pixels: u32 },
}

/// ImageType - 图像类型标识
#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    MimeType,
    Extension,
    Format(ImageFormat),
}

/// DataFormat - 数据格式
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DataFormat {
    Rgb = 0,
    Rgba = 1,
    Rrr = 2,
    Rrrg = 3,
    Rg = 4,
}

/// TranscodeFormat - 转码格式
#[derive(Debug, Clone, Copy)]
pub enum TranscodeFormat {
    Etc1s,
    Uastc(DataFormat),
    R8UnormSrgb,
    Rg8UnormSrgb,
    Rgb8,
}

/// TextureError - 纹理错误
#[derive(Debug)]
pub enum TextureError {
    InvalidImageMimeType(String),
    InvalidImageExtension(String),
    ImageError(String),
    UnsupportedTextureFormat(String),
    SuperCompressionNotSupported(String),
    SuperDecompressionError(String),
    InvalidData(String),
    TranscodeError(String),
    FormatRequiresTranscodingError(TranscodeFormat),
    IncompleteCubemap,
}

/// TextureAccessError - 纹理访问错误
#[derive(Debug)]
pub enum TextureAccessError {
    OutOfBounds { x: u32, y: u32, z: u32 },
    UnsupportedTextureFormat(TextureFormat),
    WrongDimension,
}

/// TextureReinterpretationError - 纹理重新解释错误
#[derive(Debug)]
pub enum TextureReinterpretationError {
    IncompatibleSizes { old: Extent3d, new: Extent3d },
    WrongDimension,
    InvalidLayerCount,
    HeightNotDivisibleByLayers { height: u32, layers: u32 },
}

/// ImageLoaderError - 图像加载器错误
#[derive(Debug)]
pub enum ImageLoaderError {
    Io(String),
    FileTexture(FileTextureError),
    ArrayLayout(TextureReinterpretationError),
}

/// FileTextureError - 文件纹理错误
#[derive(Debug)]
pub struct FileTextureError {
    pub error: TextureError,
    pub path: String,
}

/// CompressedImageSaverError - 压缩图像保存错误
#[derive(Debug)]
pub enum CompressedImageSaverError {
    Io(String),
    UnsupportedFormat(String),
}

/// ExrTextureLoaderError - EXR纹理加载器错误
#[derive(Debug)]
pub enum ExrTextureLoaderError {
    Io(String),
    InvalidData(String),
}

/// HdrTextureLoaderError - HDR纹理加载器错误
#[derive(Debug)]
pub enum HdrTextureLoaderError {
    Io(String),
    InvalidData(String),
}

/// TextureAtlasBuilderError - 纹理图集构建器错误
#[derive(Debug)]
pub enum TextureAtlasBuilderError {
    NotEnoughSpace,
    WrongFormat,
    UninitializedAtlas,
    UninitializedSourceTexture,
    TextureAccess(TextureAccessError),
}

/// DynamicTextureAtlasBuilderError - 动态纹理图集构建器错误
#[derive(Debug)]
pub enum DynamicTextureAtlasBuilderError {
FailedToAllocateSpace,
    UninitializedAtlas,
    UninitializedSourceTexture,
    TextureAccess(TextureAccessError),
}

/// IntoDynamicImageError - 转换为动态图像错误
#[derive(Debug)]
pub enum IntoDynamicImageError {
    UnsupportedFormat(TextureFormat),
    InvalidData(String),
}

// ============================================================================
// Struct类型 (18个)
// ============================================================================

/// CompressedImageFormatSupport - 压缩图像格式支持
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CompressedImageFormatSupport {
    pub astc_ldr: bool,
    pub bc: bool,
    pub etc2: bool,
}

impl Default for CompressedImageFormatSupport {
    fn default() -> Self {
        Self {
            astc_ldr: false,
            bc: false,
            etc2: false,
        }
    }
}

/// CompressedImageFormats - 压缩图像格式标志
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CompressedImageFormats {
    pub supported: CompressedImageFormatSupport,
}

/// CompressedImageSaver - 压缩图像保存器
#[derive(Debug, Default)]
pub struct CompressedImageSaver;

impl CompressedImageSaver {
    pub fn new() -> Self {
        Self
    }
}

/// DynamicTextureAtlasBuilder - 动态纹理图集构建器
#[derive(Debug)]
pub struct DynamicTextureAtlasBuilder {
    pub size: (u32, u32),
    pub padding: u32,
}

impl DynamicTextureAtlasBuilder {
    pub fn new(size: (u32, u32), padding: u32) -> Self {
        Self { size, padding }
    }
}

/// ExrTextureLoader - EXR纹理加载器
#[derive(Debug, Default)]
pub struct ExrTextureLoader;

/// ExrTextureLoaderSettings - EXR纹理加载器设置
#[derive(Debug, Clone, Default)]
pub struct ExrTextureLoaderSettings {
    pub is_srgb: bool,
}

/// HdrTextureLoader - HDR纹理加载器
#[derive(Debug, Default)]
pub struct HdrTextureLoader;

/// HdrTextureLoaderSettings - HDR纹理加载器设置
#[derive(Debug, Clone, Default)]
pub struct HdrTextureLoaderSettings {
    pub is_srgb: bool,
}

/// ImageLoaderSettings - 图像加载器设置
#[derive(Debug, Clone, Default)]
pub struct ImageLoaderSettings {
    pub format: ImageFormatSetting,
    pub is_srgb: bool,
    pub sampler: ImageSampler,
    pub array_layout: Option<ImageArrayLayout>,
}

/// ImagePlugin - 图像插件
#[derive(Debug, Clone)]
pub struct ImagePlugin {
    pub default_sampler: ImageSamplerDescriptor,
}

impl Default for ImagePlugin {
    fn default() -> Self {
        Self {
            default_sampler: ImageSamplerDescriptor::linear(),
        }
    }
}

impl ImagePlugin {
    pub fn default_linear() -> Self {
        Self {
            default_sampler: ImageSamplerDescriptor::linear(),
        }
    }

    pub fn default_nearest() -> Self {
        Self {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }
    }
}

/// SerializedImage - 序列化图像
#[derive(Debug, Clone)]
pub struct SerializedImage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

/// TextureAtlas - 纹理图集
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextureAtlas {
    pub layout_id: u64,
    pub index: usize,
}

impl Default for TextureAtlas {
    fn default() -> Self {
        Self {
            layout_id: 0,
            index: 0,
        }
    }
}

/// TextureAtlasBuilder - 纹理图集构建器
#[derive(Debug)]
pub struct TextureAtlasBuilder {
    pub initial_size: (u32, u32),
    pub max_size: (u32, u32),
    pub format: TextureFormat,
    pub padding: (u32, u32),
}

impl Default for TextureAtlasBuilder {
    fn default() -> Self {
        Self {
            initial_size: (256, 256),
            max_size: (2048, 2048),
            format: TextureFormat::Rgba8UnormSrgb,
            padding: (0, 0),
        }
    }
}

/// TextureAtlasLayout - 纹理图集布局
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextureAtlasLayout {
    pub size: (u32, u32),
    pub texture_count: usize,
}

impl Default for TextureAtlasLayout {
    fn default() -> Self {
        Self {
            size: (0, 0),
            texture_count: 0,
        }
    }
}

/// TextureAtlasPlugin - 纹理图集插件
#[derive(Debug, Default)]
pub struct TextureAtlasPlugin;

/// TextureAtlasSources - 纹理图集源
#[derive(Debug, Default)]
pub struct TextureAtlasSources {
    pub texture_count: usize,
}

// ============================================================================
// Trait 类型 (5个)
// ============================================================================

/// BevyDefault - Bevy默认值trait
pub trait BevyDefault {
    fn bevy_default() -> Self;
}

impl BevyDefault for TextureFormat {
    fn bevy_default() -> Self {
        TextureFormat::Rgba8UnormSrgb
    }
}

/// TextureFormatPixelInfo - 纹理格式像素信息trait
pub trait TextureFormatPixelInfo {
    fn pixel_size(&self) -> Result<usize, TextureAccessError>;
}

impl TextureFormatPixelInfo for TextureFormat {
    fn pixel_size(&self) -> Result<usize, TextureAccessError> {
        Ok(self.bytes_per_pixel() as usize)
    }
}

/// TextureSrgbViewFormats - 纹理SRGB视图格式trait
pub trait TextureSrgbViewFormats {
    fn srgb_view_formats(&self) -> &'static [TextureFormat];
}

impl TextureSrgbViewFormats for TextureFormat {
    fn srgb_view_formats(&self) -> &'static [TextureFormat] {
        match self {
            TextureFormat::Rgba8Unorm => &[TextureFormat::Rgba8UnormSrgb],
            TextureFormat::Bgra8Unorm => &[TextureFormat::Bgra8UnormSrgb],
            _ => &[],
        }
    }
}

/// ToExtents - 转换为Extent3d trait
pub trait ToExtents {
    fn to_extents(self) -> Extent3d;
}

/// Volume - 计算体积trait
pub trait Volume {
    fn volume(&self) -> usize;
}

impl Volume for Extent3d {
    fn volume(&self) -> usize {
        (self.width * self.height * self.depth_or_array_layers) as usize
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

impl Asset for Image {}

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
    