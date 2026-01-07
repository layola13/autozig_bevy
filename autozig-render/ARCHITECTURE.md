
# AutoZig Render 架构设计

## 概述

AutoZig Render 是一个基于 WGPU 的现代渲染库，采用 **90% Zig 核心 + 10% Rust wrapper** 的混合架构。设计灵感来自 [bevy_render](https://github.com/bevyengine/bevy/tree/main/crates/bevy_render)，但针对性能和可维护性进行了优化。

## 设计目标

1. **高性能** - Zig 核心提供接近 C 的性能
2. **类型安全** - Rust wrapper 提供编译时保证
3. **API 兼容** - 与 bevy_render 接口保持一致
4. **易于集成** - 可与其他 autozig-* 库无缝配合

## 层次结构

```
┌─────────────────────────────────────────────────────────────┐
│                   Application Layer                          │
│              (Rust application code)                         │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                  Rust Wrapper Layer (10%)                    │
│  ┌───────────────┐  ┌───────────────┐  ┌─────────────────┐ │
│  │   Renderer    │  │    Buffer     │  │    Texture      │ │
│  │   (管理器)    │  │   (缓冲区)    │  │    (纹理)       │ │
│  └───────┬───────┘  └───────┬───────┘  └────────┬────────┘ │
│          │                  │                     │          │
│  ┌───────┴───────┐  ┌───────┴───────┐  ┌────────┴────────┐ │
│  │RenderPipeline │  │  RenderPass   │  │  Type Safety    │ │
│  │  (渲染管线)   │  │  (渲染通道)   │  │  (类型安全)     │ │
│  └───────────────┘  └───────────────┘  └─────────────────┘ │
└──────────────────────────┬──────────────────────────────────┘
                           │ FFI (C ABI)
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   Zig Core Layer (90%)                       │
│  ┌───────────────┐  ┌───────────────┐  ┌─────────────────┐ │
│  │render_context │  │buffer_manager │  │texture_manager  │ │
│  │  (渲染上下文) │  │ (缓冲区管理)  │  │  (纹理管理)     │ │
│  └───────┬───────┘  └───────┬───────┘  └────────┬────────┘ │
│          │                  │                     │          │
│  ┌───────┴───────┐  ┌───────┴───────┐  ┌────────┴────────┐ │
│  │pipeline_cache │  │command_buffer │  │resource_pool    │ │
│  │ (管线缓存)    │  │  (命令缓冲)   │  │  (资源池)       │ │
│  └───────────────┘  └───────────────┘  └─────────────────┘ │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                      WGPU Backend                            │
│         (Vulkan / Metal / DX12 / WebGPU)                     │
└─────────────────────────────────────────────────────────────┘
```

## 核心组件

### 1. Zig 核心层 (90%)

#### 1.1 render.zig - 主模块

```zig
// 核心类型定义
pub const Renderer = opaque {};
pub const RenderContext = struct { ... };

// 资源管理
pub export fn renderer_create(...) ?*Renderer;
pub export fn renderer_destroy(renderer: *Renderer) void;

// 缓冲区操作
pub export fn create_buffer(...) ?*anyopaque;
pub export fn write_buffer(...) void;

// 纹理操作
pub export fn create_texture(...) ?*anyopaque;

// 渲染管线
pub export fn create_render_pipeline(...) ?*anyopaque;

// 渲染通道
pub export fn begin_render_pass(...) ?*anyopaque;
pub export fn end_render_pass(...) void;
```

**职责:**
- 底层渲染逻辑实现
- 资源生命周期管理
- 内存分配和优化
- 命令编码和提交

**优势:**
- 显式内存管理，无 GC 开销
- 编译时优化，零成本抽象
- 高性能数值计算
- 与 C/C++ 库无缝互操作

#### 1.2 资源管理器

```zig
const ResourceManager = struct {
    buffers: std.ArrayList(BufferHandle),
    textures: std.ArrayList(TextureHandle),
    pipelines: std.ArrayList(PipelineHandle),
    allocator: std.mem.Allocator,
    
    pub fn allocBuffer(...) !BufferHandle { ... }
    pub fn freeBuffer(handle: BufferHandle) void { ... }
};
```

**特性:**
- 池化资源分配
- 引用计数管理
- 自动回收未使用资源

#### 1.3 命令缓冲系统

```zig
const CommandBuffer = struct {
    commands: std.ArrayList(Command),
    state: RenderState,
    
    pub fn draw(...) void { ... }
    pub fn drawIndexed(...) void { ... }
    pub fn submit() void { ... }
};
```

### 2. Rust Wrapper 层 (10%)

#### 2.1 lib.rs - 主模块

```rust
// FFI 绑定
extern "C" {
    fn renderer_create(...) -> *mut c_void;
    fn create_buffer(...) -> *mut c_void;
    // ... 其他 FFI 函数
}

// Rust 封装
pub struct Renderer {
    handle: *mut c_void,
    _marker: PhantomData<*mut c_void>,
}

impl Renderer {
    pub fn new() -> RenderResult<Self> { ... }
    pub fn create_buffer(...) -> RenderResult<Buffer> { ... }
}
```

**职责:**
- 类型安全封装
- RAII 资源管理
- 错误处理
- API 人体工程学

**优势:**
- 编译时类型检查
- 自动资源清理 (Drop trait)
- 丰富的错误信息
- 符合 Rust 惯用法

#### 2.2 类型系统

```rust
// 使用 bitflags 提供类型安全的标志位
bitflags::bitflags! {
    pub struct BufferUsages: u32 {
        const VERTEX = 1 << 0;
        const INDEX = 1 << 1;
        const UNIFORM = 1 << 2;
        // ...
    }
}

// 强类型枚举
#[repr(u32)]
pub enum TextureFormat {
    Rgba8Unorm,
    Bgra8Unorm,
    // ...
}
```

#### 2.3 生命周期管理

```rust
// 自动资源清理
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            destroy_buffer(self.renderer, self.handle);
        }
    }
}

// 借用检查
pub struct RenderPass<'a> {
    handle: *mut c_void,
    _marker: PhantomData<&'a Renderer>,
}
```

## 数据流

### 创建缓冲区流程

```
1. Rust Application
   └─> renderer.create_buffer(size, usage, mapped)
   
2. Rust Wrapper
   └─> 验证参数
   └─> 调用 FFI: create_buffer(handle, size, usage_bits, mapped)
   
3. Zig Core
   └─> 分配内存
   └─> 创建 WGPU 缓冲区
   └─> 注册到资源管理器
   └─> 返回句柄
   
4. Rust Wrapper
   └─> 检查句柄有效性
   └─> 包装为 Buffer 类型
   └─> 返回 Result<Buffer>
   
5. Rust Application
   └─> 使用 Buffer (自动管理生命周期)
```

### 渲染循环流程

```
Loop:
  1. Begin Frame
     └─> renderer.begin_render_pass(&desc)
     
  2. Record Commands
     └─> pass.set_pipeline(&pipeline)
     └─> pass.set_vertex_buffer(slot, &buffer, ...)
     └─> pass.draw(vertices, instances)
     
  3. End Pass
     └─> pass 离开作用域，自动调用 end_render_pass
     
  4. Submit & Present
     └─> renderer.submit()
     └─> renderer.present()
```

## 内存管理

### Zig 侧

```zig
// 使用 Arena allocator 进行批量分配
var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
defer arena.deinit();

const allocator = arena.allocator();

// 创建资源
const buffer = try allocator.create(BufferData);
```

### Rust 侧

```rust
// RAII 自动管理
{
    let buffer = renderer.create_buffer(...)?;
    // 使用 buffer
} // buffer 自动析构，调用 Zig destroy_buffer
```

## 错误处理

### Zig 侧

```zig
// 使用 Zig 的错误联合类型
pub fn createBuffer(...) !BufferHandle {
    if (size == 0) return error.InvalidSize;
    // ...
    return handle;
}
```

### Rust 侧

```rust
// 转换为 Result
pub fn create_buffer(...) -> RenderResult<Buffer> {
    let handle = unsafe { create_buffer(...) };
    if handle.is_null() {
        return Err(RenderError::BufferCreationFailed);
    }
    Ok(Buffer { handle, ... })
}
```

## 性能考虑

### 1. FFI 开销最小化

- **直接指针传递** - 避免数据拷贝
- **批量操作** - 减少跨语言调用次数
- **内联小函数** - 编译器优化

### 2. 缓存友好

```zig
// 使用连续内存布局
const ResourcePool = struct {
    buffers: []BufferData,  // 连续数组
    free_list: []u32,       // 空闲索引
};
```

### 3. 零分配路径

```rust
// 渲染循环中避免分配
pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
    unsafe {
        draw(self.handle, 
             vertices.end - vertices.start,
             instances.end - instances.start,
             vertices.start,
             instances.start);
    }
}
```

## 与 bevy_render 的对比

| 特性 | bevy_render | autozig-render |
|------|-------------|----------------|
| **实现语言** | Rust 100% | Zig 90% + Rust 10% |
| **类型安全** | ✅ 完全 | ✅ 完全 |
| **性能** | 优秀 | 优秀+ (Zig 优化) |
| **ECS 集成** | ✅ 原生支持 | ❌ 独立库 |
| **学习曲线** | 中等 | 低 |
| **FFI 开销** | 无 | 极小 |
| **内存控制** | 中等 (Rust) | 高 (Zig) |

## 扩展性

### 添加新功能

1. **在 Zig 侧实现核心逻辑**
```zig
pub export fn compute_dispatch(...) void {
    // 实现计算着色器调度
}
```

2. **在 Rust 侧添加 FFI 绑定**
```rust
extern "C" {
    fn compute_dispatch(...);
}
```

3. **封装为 Rust API**
```rust
impl Renderer {
    pub fn dispatch_compute(&self, ...) {
        unsafe { compute_dispatch(...) }
    }
}
```

## 测试策略

### Zig 测试

```zig
test "buffer creation" {
    const renderer = renderer_create(std.testing.allocator);
    defer renderer_destroy(renderer);
    
    const buffer = create_buffer(renderer, 1024, usage, false);
    try std.testing.expect(buffer != null);
}
```

### Rust 测试

```rust
#[test]
fn test_buffer_usages() {
    let usage = BufferUsages::VERTEX | BufferUsages::UNIFORM;
    assert!(usage.contains(BufferUsages::VERTEX));
}
```

### 集成测试

```rust
#[test]
fn test_render_pipeline() -> RenderResult<()> {
    let renderer = Renderer::new()?;
    let pipeline = renderer.create_render_pipeline(&desc)?;
    Ok(())
}
```

## 未来改进

1. **多线程支持** - 并行命令编码
2. **渲染图** - 自动依赖管理和优化
3. **计算管线** - 通用 GPU 计算
4. **光线追踪** - 硬件加速光追
5. **高级材质** - PBR、SSR、GI 等

## 结论

AutoZig Render 通过结合 Zig 的性能优势和 Rust 的安全性，提供了一个高效、类型安全的渲染抽象层。90/10 的比例确保了核心性能的同时，保持了 API 的易用性和安全性。
