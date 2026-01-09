# AutoZig Bevy 完成度评估 (WASM/Web 平台)

> 目标平台: WebAssembly + Web Browser  
> 评估日期: 2026-01-09

## 概要

| 指标 | 当前状态 |
|------|---------|
| **已实现 crates** | 27 个 |
| **Bevy 总 crates** | 55 个 |
| **完成度** | ~50% |
| **WASM 3D Demo** | ⚠️ 需补充 4 个核心 crate |

---

## Crate 对比表

### ✅ 已实现 (27个)

| AutoZig | Bevy 对应 | WASM 就绪 |
|---------|----------|-----------|
| `autozig-app` | `bevy_app` | ✅ |
| `autozig-asset` | `bevy_asset` | ⚠️ 需测试 |
| `autozig-color` | `bevy_color` | ✅ |
| `autozig-derive` | `bevy_derive` | ✅ |
| `autozig-diagnostic` | `bevy_diagnostic` | ✅ |
| `autozig-ecs` | `bevy_ecs` | ✅ |
| `autozig-hierarchy` | (新增) | ✅ |
| `autozig-image` | `bevy_image` | ⚠️ 需测试 |
| `autozig-input` | `bevy_input` | ⚠️ Web 事件待绑定 |
| `autozig-light` | `bevy_light` | ✅ |
| `autozig-log` | `bevy_log` | ✅ |
| `autozig-macro-utils` | `bevy_macro_utils` | ✅ |
| `autozig-math` | `bevy_math` | ✅ |
| `autozig-mesh` | `bevy_mesh` | ✅ |
| `autozig-pbr` | `bevy_pbr` | ⚠️ 光照计算 OK |
| `autozig-ptr` | `bevy_ptr` | ✅ |
| `autozig-reflect` | `bevy_reflect` | ✅ |
| `autozig-render` | `bevy_render` | ⚠️ 类型定义 OK,需接 WebGPU |
| `autozig-sprite` | `bevy_sprite` | ⚠️ 数据 OK |
| `autozig-state` | `bevy_state` | ✅ |
| `autozig-tasks` | `bevy_tasks` | ✅ WASM 单线程兼容 |
| `autozig-text` | `bevy_text` | ⚠️ 待实现 |
| `autozig-time` | `bevy_time` | ✅ |
| `autozig-transform` | `bevy_transform` | ✅ |
| `autozig-ui` | `bevy_ui` | ⚠️ 布局 OK |
| `autozig-utils` | `bevy_utils` | ✅ |
| `autozig-window` | `bevy_window` | ⚠️ 抽象 OK |

---

### ❌ 缺失 - WASM 关键 (需新增)

| 需新增 | Bevy 对应 | 说明 |
|--------|----------|------|
| 🔴 `autozig-winit` | `bevy_winit` | **WASM: requestAnimationFrame / 桌面: winit** |
| 🔴 `autozig-core-pipeline` | `bevy_core_pipeline` | **渲染阶段编排 (Extract/Prepare/Render)** |
| 🔴 `autozig-camera` | `bevy_camera` | **Camera2d/Camera3d 组件** |
| 🔴 `autozig-shader` | `bevy_shader` | **WGSL 着色器加载与编译** |

> 💡 WASM 模式下使用 `requestAnimationFrame`，桌面模式使用原生 winit

---

### ❌ 缺失 - 渲染执行层

| 需新增 | Bevy 对应 | 说明 |
|--------|----------|------|
| 🟡 `autozig-sprite-render` | `bevy_sprite_render` | Sprite 批处理 + GPU 绘制 |
| 🟡 `autozig-ui-render` | `bevy_ui_render` | UI 渲染到 GPU |

---

### ❌ 缺失 - 可选功能 (WASM 不必须)

| Bevy Crate | 重要性 | 备注 |
|------------|--------|------|
| `bevy_animation` | 中等 | 骨骼/关键帧动画 |
| `bevy_gltf` | 中等 | 3D 模型加载 |
| `bevy_audio` | 中等 | Web Audio API |
| `bevy_gizmos` | 低 | 调试绘制 |
| `bevy_picking` | 低 | 射线拾取 |
| `bevy_post_process` | 低 | 后处理效果 |
| `bevy_a11y` | 低 | 无障碍 |
| `bevy_android` | N/A | 不需要 |
| `bevy_gilrs` | N/A | 手柄 (可用 Gamepad API) |

---

## WASM 3D Demo 实现路线图

> **架构原则: 90% Zig + 10% Rust**  
> Zig 直接调用 WebGPU C API，无需通过 Rust 中转

### Phase 1: 核心渲染能力 (1周)

```
需新增:
├── autozig-winit/
│   ├── zig/
│   │   ├── runner.zig           # 90% Zig: 帧循环、时间管理
│   │   └── webgpu_bindings.zig  # WebGPU C API 绑定 (extern import)
│   └── src/lib.rs               # 10% Rust: wasm-bindgen 入口导出
│
├── autozig-camera/
│   ├── zig/
│   │   ├── camera.zig           # 90% Zig: 投影/视图矩阵计算
│   │   └── frustum.zig          # 视锥体剔除
│   └── src/lib.rs               # 10% Rust: Camera2d/Camera3d 组件
│
├── autozig-core-pipeline/
│   ├── zig/
│   │   ├── render_context.zig   # 90% Zig: GPU 上下文管理
│   │   ├── render_pass.zig      # RenderPass 编码
│   │   ├── command_buffer.zig   # 命令缓冲组织
│   │   └── pipeline_cache.zig   # Pipeline 缓存
│   └── src/lib.rs               # 10% Rust: RenderApp 调度
│
└── autozig-shader/
    ├── zig/
    │   ├── shader_module.zig    # 90% Zig: wgpuDeviceCreateShaderModule
    │   └── uniform_layout.zig   # Uniform 布局计算
    └── src/lib.rs               # 10% Rust: 着色器资源包装
```

### Phase 2: WebGPU 集成

```zig
// zig/webgpu_bindings.zig - Zig 直接调用 WebGPU
// 参考 Mach Engine 的 WebGPU 绑定

pub const WGPUDevice = *opaque {};
pub const WGPUBuffer = *opaque {};
pub const WGPURenderPassEncoder = *opaque {};

// WASM 导入浏览器 WebGPU 实现
pub extern "webgpu" fn wgpuDeviceCreateBuffer(
    device: WGPUDevice,
    descriptor: *const WGPUBufferDescriptor
) WGPUBuffer;

pub extern "webgpu" fn wgpuRenderPassEncoderDraw(
    encoder: WGPURenderPassEncoder,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32
) void;

pub extern "webgpu" fn wgpuRenderPassEncoderSetPipeline(
    encoder: WGPURenderPassEncoder,
    pipeline: WGPURenderPipeline
) void;
```

### Phase 3: 完整 Demo

```rust
// 目标 API (WASM)
fn main() {
    App::new()
        .add_plugins(WebDefaultPlugins)  // WASM 专用插件集
        .add_systems(Startup, setup)
        .run_wasm();  // requestAnimationFrame 循环
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Mesh3d::cube(1.0),
        StandardMaterial::default(),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.0, 5.0).looking_at(Vec3::ZERO),
    ));
    commands.spawn(PointLight::default());
}
```

---

## 架构对比

### Bevy 架构 (100% Rust)
```
bevy_winit (Rust) → Event Loop → bevy_render (Rust) → wgpu-rs → GPU
```

### AutoZig 架构 (90% Zig + 10% Rust)
```
autozig-winit
├── zig/runner.zig ────────┐
│   (帧循环、时间管理)       │
└── src/lib.rs ◄───────────┤ wasm-bindgen 入口
    (10% Rust 导出)         │
                           ▼
autozig-core-pipeline
├── zig/render_pass.zig ───► wgpuRenderPassEncoderDraw (Zig 直接调用)
│   (命令编码)               │
└── zig/webgpu_bindings.zig ► WebGPU C API
                           │
                           ▼
                    Browser WebGPU
```

### 关键区别

| 层级 | Bevy | AutoZig |
|------|------|---------|
| GPU API 调用 | Rust (wgpu-rs) | **Zig 直接调用** |
| 矩阵计算 | Rust (glam) | **Zig** |
| 命令编码 | Rust | **Zig** |
| WASM 导出 | Rust | Rust (仅入口) |

---

## 优先级建议

| 优先级 | Crate | Zig 比例 | 工时预估 |
|--------|-------|---------|---------|
| P0 | `autozig-winit` | 90% | 2天 |
| P0 | `autozig-camera` | 95% | 1天 |
| P0 | `autozig-core-pipeline` | 90% | 3天 |
| P1 | `autozig-shader` | 85% | 2天 |
| P2 | `autozig-sprite-render` | 90% | 2天 |
| P2 | `autozig-ui-render` | 90% | 2天 |

**总计: ~12 工作日可完成 WASM 3D Demo**

---

## 结论

当前 AutoZig Bevy 的 **数据层** 基本完整 (math, ecs, mesh, pbr, transform)。

缺失的是 **渲染执行层** (4个核心 crate):
1. 事件循环 (`autozig-winit`) - Zig 帧循环 + WASM 入口
2. 渲染管线编排 (`autozig-core-pipeline`) - Zig 直接调用 WebGPU
3. 相机组件 (`autozig-camera`) - Zig 矩阵计算
4. 着色器管理 (`autozig-shader`) - Zig 创建 ShaderModule

**核心理念**: Zig 直接绑定 WebGPU C API，与 Mach Engine 相同架构，保持 90% Zig 比例。
