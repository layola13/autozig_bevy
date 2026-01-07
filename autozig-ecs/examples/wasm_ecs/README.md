# AutoZig ECS WASM Demo

演示AutoZig ECS在WebAssembly环境下的运行。

## 🎯 特性

- ✅ **90% Zig实现** - ECS核心逻辑用Zig编写
- ✅ **10% Rust包装** - Rust提供WASM绑定
- ✅ **零unsafe代码** - 用户侧完全safe
- ✅ **WebAssembly兼容** - 在浏览器中运行ECS
- ✅ **include_zig!桥接** - 使用autozig宏系统

## 🚀 快速开始

### 构建WASM

```bash
# 确保已安装wasm-pack
cargo install wasm-pack

# 构建WASM模块
cd examples/wasm_ecs
wasm-pack build --target web --out-dir www/pkg
```

### 本地运行

```bash
cd www
python3 -m http.server 8080
```

然后访问 http://localhost:8080

## 🎮 使用方法

1. 点击 "Initialize ECS" 初始化ECS系统
2. 点击 "Create 10 Entities" 创建entities和components
3. 点击 "Simulate Movement" 模拟移动系统
4. 点击 "Destroy Random Entity" 随机删除entity
5. 点击 "Refresh Stats" 刷新统计信息

## 📊 演示功能

- **Entity管理**: spawn/despawn entities
- **Component存储**: Position和Velocity组件
- **SparseSet**: 高效稀疏组件存储
- **System模拟**: 简单的移动系统

## 🔧 技术栈

- **Zig 0.12+**: 核心ECS实现
- **Rust**: WASM绑定层
- **wasm-bindgen**: Rust-JavaScript互操作
- **AutoZig**: Zig-Rust FFI桥接

## 📝 API

### JavaScript接口

```javascript
// 初始化ECS
init_ecs(): string

// 创建entity
create_entity(x: f32, y: f32, vx: f32, vy: f32): u32

// 获取entity数量
get_entity_count(): u32

// 获取组件统计
get_component_counts(): string

// 删除entity
destroy_entity(entity_idx: u32): bool

// 模拟移动
simulate_movement(dt: f32): string

// 版本信息
get_version(): string
```

## 🏗️ 项目结构

```
wasm_ecs/
├── Cargo.toml          # 项目配置
├── build.rs            # 构建脚本
├── .cargo/
│   └── config.toml     # WASM构建配置
├── src/
│   └── lib.rs          # WASM绑定代码
└── www/
    ├── index.html      # 前端界面
    └── pkg/            # WASM输出（构建后）
```

## 📄 许可证

MIT OR Apache-2.0
