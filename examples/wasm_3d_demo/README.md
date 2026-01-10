# AutoZig Bevy 3D Demo

> 纯Rust实现的WASM 3D Demo，采用Bevy风格API

## 项目特点

- ✅ **纯Rust实现**：Demo代码100%使用Rust编写，无需编写Zig代码
- ✅ **Bevy风格API**：使用`App::new().add_plugins().add_systems().run()`的熟悉API
- ✅ **90% Zig + 10% Rust架构**：底层模块使用Zig实现高性能计算，Rust提供类型安全的API
- ✅ **完全禁止unsafe**：所有代码遵守`#![forbid(unsafe_code)]`约束
- ✅ **WebAssembly支持**：编译为WASM在浏览器中运行

## 场景内容

### 3D物体
- **立方体**：位于(-3, 1, 0)
- **球体**：位于(0, 1, 0)，半径0.5
- **圆柱体**：位于(3, 1, 0)，半径0.5，高度1.0
- **地面平面**：10x10单位，位于原点

### 光照
- **点光源**：位于(4, 8, 4)，10,000流明强度，启用阴影

### 相机
- **透视相机**：位于(0, 5, 10)，朝向原点

### 动画
- **旋转系统**：所有物体自动旋转（每帧更新）

## 项目结构

```
wasm_3d_demo/
├── Cargo.toml          # 项目配置和依赖
├── src/
│   └── lib.rs          # 纯Rust实现的Demo代码
├── www/
│   ├── index.html      # WASM加载页面
│   └── build.sh        # 编译脚本
└── README.md           # 本文件
```

## 依赖模块

项目使用以下autozig_bevy模块：

| 模块 | 用途 |
|------|------|
| `autozig-app` | 应用框架、插件系统、调度系统 |
| `autozig-ecs` | 实体组件系统 |
| `autozig-math` | 数学库（向量、矩阵、四元数） |
| `autozig-mesh` | 网格生成（立方体、球体、圆柱体等） |
| `autozig-light` | 光照系统（点光源、方向光、聚光灯） |
| `autozig-transform` | 变换组件（位置、旋转、缩放） |
| `autozig-camera` | 相机系统 |
| 其他 | 渲染、材质、窗口等 |

## 构建和运行

### 前置要求

1. **Rust工具链**：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack**：
   ```bash
   cargo install wasm-pack
   ```

3. **HTTP服务器**（推荐Python）：
   ```bash
   # Python 3已预装在大多数系统上
   python3 --version
   ```

### 编译步骤

#### 方法1：使用build.sh脚本（推荐）

```bash
cd autozig_bevy/examples/wasm_3d_demo/www
chmod +x build.sh
./build.sh          # Release模式
./build.sh dev      # Dev模式（更快的编译）
```

#### 方法2：手动编译

```bash
cd autozig_bevy/examples/wasm_3d_demo

# Release模式（优化体积）
wasm-pack build --target web --release

# Dev模式（快速编译）
wasm-pack build --target web --dev
```

### 运行Demo

1. **启动HTTP服务器**：
   ```bash
   cd autozig_bevy/examples/wasm_3d_demo/www
   python3 -m http.server 8080
   ```

2. **打开浏览器**：
   访问 `http://localhost:8080`

3. **查看日志**：
   按`F12`打开开发者工具，查看Console标签中的详细日志

## 代码示例

### Bevy风格的App初始化

```rust
use autozig_app::{App, MainScheduleOrder, DefaultPlugins};

#[wasm_bindgen(start)]
pub fn main() {
    let mut app = App::new();
    
    // 添加默认插件
    app.add_plugins(DefaultPlugins.build().finish(&mut app));
    
    // 添加启动系统
    app.add_systems(MainScheduleOrder::Startup, setup_scene);
    
    // 添加更新系统
    app.add_systems(MainScheduleOrder::Update, rotate_shapes);
    
    // 运行应用
    app.run();
}
```

### 创建3D物体

```rust
use autozig_mesh::MeshPrimitives;
use autozig_transform::Transform;

fn create_shapes() {
    // 创建立方体
    let cube_mesh = MeshPrimitives::cube(1.0);
    let cube_transform = Transform::from_translation([-3.0, 1.0, 0.0]);
    
    // 创建球体
    let sphere_mesh = MeshPrimitives::sphere(0.5, 32, 16);
    let sphere_transform = Transform::from_translation([0.0, 1.0, 0.0]);
    
    // 创建圆柱体
    let cylinder_mesh = MeshPrimitives::cylinder(0.5, 1.0, 32);
    let cylinder_transform = Transform::from_translation([3.0, 1.0, 0.0]);
}
```

### 创建光照

```rust
use autozig_light::PointLight;

fn create_lights() {
    let mut point_light = PointLight::new(
        [1.0, 1.0, 1.0],  // 白色
        10_000.0,          // 10,000流明
        100.0              // 100单位范围
    );
    
    point_light.enable_shadows();
}
```

## 性能指标

### 编译输出

- **Release模式**：体积最小化（~100-200 KB WASM）
- **Dev模式**：编译速度快（~500-800 KB WASM）

### 运行时性能

- **目标帧率**：30+ FPS @ 1920x1080
- **内存占用**：< 50 MB

## 技术细节

### 架构分层

```
┌─────────────────────────────────────┐
│   Demo代码（纯Rust）                 │
│   lib.rs - Bevy风格API               │
├─────────────────────────────────────┤
│   autozig_bevy模块（10% Rust FFI）   │
│   类型安全的Rust API包装              │
├─────────────────────────────────────┤
│   Zig核心实现（90% Zig）             │
│   SIMD优化的数学/网格/光照计算        │
└─────────────────────────────────────┘
```

### 安全保证

- ✅ 所有Demo代码禁止`unsafe`：`#![forbid(unsafe_code)]`
- ✅ FFI边界经过严格验证
- ✅ 使用`Cell`/`RefCell`进行内部可变性
- ✅ 所有指针操作封装在autozig_bevy模块中

## 已知限制

由于autozig_bevy处于早期开发阶段，当前实现存在以下限制：

1. **简化的ECS**：Query系统功能有限，无法像Bevy那样进行复杂的组件查询
2. **无完整的调度系统**：系统调度较简单，缺少依赖排序
3. **渲染未完全集成**：当前Demo主要演示API风格，渲染管线待完善

这些限制将在后续版本中逐步改进。

## 故障排除

### 编译错误

**问题**：`wasm-pack build` 失败
**解决**：
```bash
# 清理缓存
cargo clean
rm -rf pkg/

# 重新编译
wasm-pack build --target web --release
```

### 运行时错误

**问题**：浏览器显示WASM加载失败
**解决**：
1. 确保使用HTTP服务器（而非`file://`协议）
2. 检查浏览器Console中的错误信息
3. 确认pkg目录包含`.wasm`和`.js`文件

### 性能问题

**问题**：帧率低于预期
**解决**：
1. 使用Release模式编译：`wasm-pack build --release`
2. 在现代浏览器中测试（Chrome、Firefox、Edge）
3. 检查浏览器硬件加速是否启用

## 开发路线图

- [ ] 完整的ECS Query系统
- [ ] WebGPU渲染管线集成
- [ ] 材质系统和纹理支持
- [ ] 用户交互（鼠标/键盘控制相机）
- [ ] 更多几何体和特效
- [ ] 性能优化和基准测试

## 贡献

欢迎提交Issue和Pull Request！

## 许可证

MIT / Apache-2.0（与Bevy保持一致）

## 致谢

- [Bevy Engine](https://bevyengine.org/) - API设计灵感
- [Zig](https://ziglang.org/) - 高性能计算核心
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) - WASM工具链