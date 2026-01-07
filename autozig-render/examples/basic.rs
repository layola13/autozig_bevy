//! 基础渲染示例
//! 
//! 演示如何使用 autozig-render 创建简单的三角形渲染

use autozig_render::{
    Renderer, RenderResult, BufferUsages, TextureFormat, TextureUsages,
    TextureDescriptor, RenderPipelineDescriptor, RenderPassDescriptor,
    RenderPassColorAttachment, Operations, Extent3d,
};

fn main() -> RenderResult<()> {
    env_logger::init();
    
    println!("AutoZig Render - 基础示例");
    println!("=========================");
    println!("90% Zig 核心 + 10% Rust wrapper");
    println!("参考 bevy_render 架构\n");
    
    // 创建渲染器
    println!("创建渲染器...");
    let renderer = Renderer::new()?;
    println!("✓ 渲染器创建成功");
    
    // 创建顶点缓冲区
    println!("\n创建顶点缓冲区...");
    let vertices: &[f32] = &[
        // 位置           // 颜色
        0.0, 0.5, 0.0,   1.0, 0.0, 0.0, 1.0,  // 顶点1: 红色
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0, 1.0,  // 顶点2: 绿色
        0.5, -0.5, 0.0,  0.0, 0.0, 1.0, 1.0,  // 顶点3: 蓝色
    ];
    
    let vertex_buffer = renderer.create_buffer(
        (vertices.len() * std::mem::size_of::<f32>()) as u64,
        BufferUsages::VERTEX | BufferUsages::COPY_DST,
        false,
    )?;
    
    // 将顶点数据写入缓冲区
    let vertex_bytes = unsafe {
        std::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * std::mem::size_of::<f32>(),
        )
    };
    vertex_buffer.write(0, vertex_bytes);
    println!("✓ 顶点缓冲区创建成功 ({} 字节)", vertex_buffer.size());
    
    // 创建纹理
    println!("\n创建渲染目标纹理...");
    let texture_desc = TextureDescriptor::new_2d(
        800,
        600,
        TextureFormat::Bgra8UnormSrgb,
        TextureUsages::RENDER_ATTACHMENT,
    );
    let _texture = renderer.create_texture(&texture_desc)?;
    println!("✓ 纹理创建成功 ({}x{})", texture_desc.size.width, texture_desc.size.height);
    
    // 创建渲染管线
    println!("\n创建渲染管线...");
    let pipeline_desc = RenderPipelineDescriptor {
        vertex_shader: include_str!("shaders/basic.vert.wgsl").to_string(),
        fragment_shader: include_str!("shaders/basic.frag.wgsl").to_string(),
    };
    let _pipeline = renderer.create_render_pipeline(&pipeline_desc)?;
    println!("✓ 渲染管线创建成功");
    
    println!("\n所有渲染资源初始化完成!");
    println!("\n注意: 这是一个基础示例，展示了 API 的使用方式");
    println!("完整的渲染循环需要窗口系统集成 (如 winit)");
    
    Ok(())
}