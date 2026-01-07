//! AutoZig Render - WGPU 渲染库
//! 
//! 90% Zig 核心 + 10% Rust wrapper，参考 bevy_render 架构

use std::ffi::c_void;
use std::marker::PhantomData;
use thiserror::Error;

// ============================================================================
// 错误类型
// ============================================================================

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Failed to create renderer")]
    RendererCreationFailed,
    
    #[error("Failed to create buffer")]
    BufferCreationFailed,
    
    #[error("Failed to create texture")]
    TextureCreationFailed,
    
    #[error("Failed to create pipeline")]
    PipelineCreationFailed,
    
    #[error("Invalid resource handle")]
    InvalidHandle,
    
    #[error("Render pass error: {0}")]
    RenderPassError(String),
}

pub type RenderResult<T> = Result<T, RenderError>;

// ============================================================================
// FFI 绑定到 Zig 核心
// ============================================================================

extern "C" {
    fn renderer_create(allocator: *mut c_void) -> *mut c_void;
    fn renderer_destroy(renderer: *mut c_void);
    
    fn create_buffer(
        renderer: *mut c_void,
        size: u64,
        usage: u32,
        mapped_at_creation: bool,
    ) -> *mut c_void;
    fn destroy_buffer(renderer: *mut c_void, buffer: *mut c_void);
    fn write_buffer(
        renderer: *mut c_void,
        buffer: *mut c_void,
        offset: u64,
        data: *const u8,
        size: u64,
    );
    
    fn create_texture(
        renderer: *mut c_void,
        width: u32,
        height: u32,
        depth: u32,
        format: u32,
        dimension: u32,
        usage: u32,
    ) -> *mut c_void;
    fn destroy_texture(renderer: *mut c_void, texture: *mut c_void);
    
    fn create_render_pipeline(
        renderer: *mut c_void,
        desc: *const c_void,
    ) -> *mut c_void;
    fn destroy_render_pipeline(renderer: *mut c_void, pipeline: *mut c_void);
    
    fn begin_render_pass(
        renderer: *mut c_void,
        color_attachments: *const c_void,
        color_attachment_count: u32,
        depth_stencil_attachment: *const c_void,
    ) -> *mut c_void;
    fn end_render_pass(renderer: *mut c_void, pass: *mut c_void);
    
    fn set_pipeline(pass: *mut c_void, pipeline: *mut c_void);
    fn set_vertex_buffer(
        pass: *mut c_void,
        slot: u32,
        buffer: *mut c_void,
        offset: u64,
        size: u64,
    );
    fn set_index_buffer(
        pass: *mut c_void,
        buffer: *mut c_void,
        format: u32,
        offset: u64,
        size: u64,
    );
    fn set_bind_group(
        pass: *mut c_void,
        index: u32,
        bind_group: *mut c_void,
        dynamic_offsets: *const u32,
        dynamic_offset_count: u32,
    );
    
    fn draw(
        pass: *mut c_void,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    fn draw_indexed(
        pass: *mut c_void,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    );
    
    fn submit_commands(renderer: *mut c_void);
    fn present_frame(renderer: *mut c_void);
}

// ============================================================================
// Rust 类型封装 - 参考 bevy_render 接口
// ============================================================================

/// 渲染器
pub struct Renderer {
    handle: *mut c_void,
    _marker: PhantomData<*mut c_void>,
}

impl Renderer {
    /// 创建新的渲染器实例
    pub fn new() -> RenderResult<Self> {
        unsafe {
            let handle = renderer_create(std::ptr::null_mut());
            if handle.is_null() {
                return Err(RenderError::RendererCreationFailed);
            }
            Ok(Self {
                handle,
                _marker: PhantomData,
            })
        }
    }
    
    /// 创建缓冲区
    pub fn create_buffer(
        &self,
        size: u64,
        usage: BufferUsages,
        mapped_at_creation: bool,
    ) -> RenderResult<Buffer> {
        unsafe {
            let buffer = create_buffer(
                self.handle,
                size,
                usage.bits(),
                mapped_at_creation,
            );
            if buffer.is_null() {
                return Err(RenderError::BufferCreationFailed);
            }
            Ok(Buffer {
                handle: buffer,
                renderer: self.handle,
                size,
                _marker: PhantomData,
            })
        }
    }
    
    /// 创建纹理
    pub fn create_texture(&self, desc: &TextureDescriptor) -> RenderResult<Texture> {
        unsafe {
            let texture = create_texture(
                self.handle,
                desc.size.width,
                desc.size.height,
                desc.size.depth_or_array_layers,
                desc.format as u32,
                desc.dimension as u32,
                desc.usage.bits(),
            );
            if texture.is_null() {
                return Err(RenderError::TextureCreationFailed);
            }
            Ok(Texture {
                handle: texture,
                renderer: self.handle,
                _marker: PhantomData,
            })
        }
    }
    
    /// 创建渲染管线
    pub fn create_render_pipeline(
        &self,
        desc: &RenderPipelineDescriptor,
    ) -> RenderResult<RenderPipeline> {
        unsafe {
            let pipeline = create_render_pipeline(
                self.handle,
                desc as *const _ as *const c_void,
            );
            if pipeline.is_null() {
                return Err(RenderError::PipelineCreationFailed);
            }
            Ok(RenderPipeline {
                handle: pipeline,
                renderer: self.handle,
                _marker: PhantomData,
            })
        }
    }
    
    /// 开始渲染通道
    pub fn begin_render_pass<'a>(
        &'a self,
        desc: &RenderPassDescriptor,
    ) -> RenderResult<RenderPass<'a>> {
        unsafe {
            let pass = begin_render_pass(
                self.handle,
                desc.color_attachments.as_ptr() as *const c_void,
                desc.color_attachments.len() as u32,
                match &desc.depth_stencil_attachment {
                    Some(ds) => ds as *const _ as *const c_void,
                    None => std::ptr::null(),
                },
            );
            if pass.is_null() {
                return Err(RenderError::RenderPassError("Failed to begin".into()));
            }
            Ok(RenderPass {
                handle: pass,
                renderer: self.handle,
                _marker: PhantomData,
            })
        }
    }
    
    /// 提交命令
    pub fn submit(&self) {
        unsafe {
            submit_commands(self.handle);
        }
    }
    
    /// 呈现帧
    pub fn present(&self) {
        unsafe {
            present_frame(self.handle);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            renderer_destroy(self.handle);
        }
    }
}

unsafe impl Send for Renderer {}
unsafe impl Sync for Renderer {}

/// 缓冲区
pub struct Buffer {
    handle: *mut c_void,
    renderer: *mut c_void,
    size: u64,
    _marker: PhantomData<*mut c_void>,
}

impl Buffer {
    /// 写入数据到缓冲区
    pub fn write(&self, offset: u64, data: &[u8]) {
        unsafe {
            write_buffer(
                self.renderer,
                self.handle,
                offset,
                data.as_ptr(),
                data.len() as u64,
            );
        }
    }
    
    /// 获取缓冲区大小
    pub fn size(&self) -> u64 {
        self.size
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            destroy_buffer(self.renderer, self.handle);
        }
    }
}

/// 纹理
pub struct Texture {
    handle: *mut c_void,
    renderer: *mut c_void,
    _marker: PhantomData<*mut c_void>,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            destroy_texture(self.renderer, self.handle);
        }
    }
}

/// 渲染管线
pub struct RenderPipeline {
    handle: *mut c_void,
    renderer: *mut c_void,
    _marker: PhantomData<*mut c_void>,
}

impl Drop for RenderPipeline {
    fn drop(&mut self) {
        unsafe {
            destroy_render_pipeline(self.renderer, self.handle);
        }
    }
}

/// 渲染通道
pub struct RenderPass<'a> {
    handle: *mut c_void,
    renderer: *mut c_void,
    _marker: PhantomData<&'a Renderer>,
}

impl<'a> RenderPass<'a> {
    /// 设置渲染管线
    pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        unsafe {
            set_pipeline(self.handle, pipeline.handle);
        }
    }
    
    /// 设置顶点缓冲区
    pub fn set_vertex_buffer(&mut self, slot: u32, buffer: &Buffer, offset: u64, size: u64) {
        unsafe {
            set_vertex_buffer(self.handle, slot, buffer.handle, offset, size);
        }
    }
    
    /// 设置索引缓冲区
    pub fn set_index_buffer(&mut self, buffer: &Buffer, format: IndexFormat, offset: u64, size: u64) {
        unsafe {
            set_index_buffer(self.handle, buffer.handle, format as u32, offset, size);
        }
    }
    
    /// 绘制
    pub fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        unsafe {
            draw(
                self.handle,
                vertices.end - vertices.start,
                instances.end - instances.start,
                vertices.start,
                instances.start,
            );
        }
    }
    
    /// 绘制索引
    pub fn draw_indexed(
        &mut self,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        unsafe {
            draw_indexed(
                self.handle,
                indices.end - indices.start,
                instances.end - instances.start,
                indices.start,
                base_vertex,
                instances.start,
            );
        }
    }
}

impl<'a> Drop for RenderPass<'a> {
    fn drop(&mut self) {
        unsafe {
            end_render_pass(self.renderer, self.handle);
        }
    }
}

// ============================================================================
// 类型定义 - 兼容 bevy_render 接口
// ============================================================================

bitflags::bitflags! {
    /// 缓冲区使用标志
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BufferUsages: u32 {
        const MAP_READ = 1 << 0;
        const MAP_WRITE = 1 << 1;
        const COPY_SRC = 1 << 2;
        const COPY_DST = 1 << 3;
        const INDEX = 1 << 4;
        const VERTEX = 1 << 5;
        const UNIFORM = 1 << 6;
        const STORAGE = 1 << 7;
        const INDIRECT = 1 << 8;
        const QUERY_RESOLVE = 1 << 9;
    }
}

bitflags::bitflags! {
    /// 纹理使用标志
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TextureUsages: u32 {
        const COPY_SRC = 1 << 0;
        const COPY_DST = 1 << 1;
        const TEXTURE_BINDING = 1 << 2;
        const STORAGE_BINDING = 1 << 3;
        const RENDER_ATTACHMENT = 1 << 4;
    }
}

/// 纹理格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureFormat {
    Rgba8Unorm = 0,
    Rgba8UnormSrgb = 1,
    Bgra8Unorm = 2,
    Bgra8UnormSrgb = 3,
    Depth24Plus = 4,
    Depth32Float = 5,
    Depth24PlusStencil8 = 6,
}

/// 纹理维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureDimension {
    D1 = 0,
    D2 = 1,
    D3 = 2,
}

/// 索引格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IndexFormat {
    Uint16 = 0,
    Uint32 = 1,
}

/// 纹理尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

/// 纹理描述符
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    pub size: Extent3d,
    pub format: TextureFormat,
    pub dimension: TextureDimension,
    pub usage: TextureUsages,
    pub mip_level_count: u32,
    pub sample_count: u32,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
            size: Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            format: TextureFormat::Rgba8Unorm,
            dimension: TextureDimension::D2,
            usage: TextureUsages::TEXTURE_BINDING,
            mip_level_count: 1,
            sample_count: 1,
        }
    }
}

/// 渲染管线描述符
#[derive(Debug, Clone)]
pub struct RenderPipelineDescriptor {
    pub vertex_shader: String,
    pub fragment_shader: String,
}

/// 颜色附件
#[derive(Debug, Clone)]
pub struct RenderPassColorAttachment {
    pub view: *mut c_void,
    pub resolve_target: Option<*mut c_void>,
    pub ops: Operations<[f32; 4]>,
}

/// 深度模板附件
#[derive(Debug, Clone)]
pub struct RenderPassDepthStencilAttachment {
    pub view: *mut c_void,
    pub depth_ops: Option<Operations<f32>>,
    pub stencil_ops: Option<Operations<u32>>,
}

/// 操作
#[derive(Debug, Clone, Copy)]
pub struct Operations<T> {
    pub load: LoadOp<T>,
    pub store: StoreOp,
}

/// 加载操作
#[derive(Debug, Clone, Copy)]
pub enum LoadOp<T> {
    Clear(T),
    Load,
}

/// 存储操作
#[derive(Debug, Clone, Copy)]
pub enum StoreOp {
    Store,
    Discard,
}

/// 渲染通道描述符
#[derive(Debug, Clone)]
pub struct RenderPassDescriptor {
    pub color_attachments: Vec<RenderPassColorAttachment>,
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment>,
}

// ============================================================================
// 便捷构造器 - bevy 风格
// ============================================================================

impl TextureDescriptor {
    /// 创建 2D 纹理描述符
    pub fn new_2d(
        width: u32,
        height: u32,
        format: TextureFormat,
        usage: TextureUsages,
    ) -> Self {
        Self {
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            format,
            dimension: TextureDimension::D2,
            usage,
            mip_level_count: 1,
            sample_count: 1,
        }
    }
}

impl<T: Clone> Operations<T> {
    /// 清除操作
    pub fn clear(clear_value: T) -> Self {
        Self {
            load: LoadOp::Clear(clear_value),
            store: StoreOp::Store,
        }
    }
    
    /// 加载操作
    pub fn load() -> Self
    where
        T: Default,
    {
        Self {
            load: LoadOp::Load,
            store: StoreOp::Store,
        }
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_usages() {
        let usage = BufferUsages::VERTEX | BufferUsages::UNIFORM;
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::UNIFORM));
        assert!(!usage.contains(BufferUsages::INDEX));
    }

    #[test]
    fn test_texture_descriptor() {
        let desc = TextureDescriptor::new_2d(
            1024,
            768,
            TextureFormat::Rgba8Unorm,
            TextureUsages::RENDER_ATTACHMENT,
        );
        assert_eq!(desc.size.width, 1024);
        assert_eq!(desc.size.height, 768);
    }
}