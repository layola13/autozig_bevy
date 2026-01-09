# AutoZig App/ECS 1:1 Bevy 评估报告

> 评估日期: 2026-01-09

## 总结

| 模块 | Bevy 复杂度 | AutoZig 实现 | 完成度 |
|------|------------|-------------|--------|
| `autozig-app` | 12 文件, ~200KB | 1 文件, 12KB | **30%** |
| `autozig-ecs` | 28 模块, ~1.5MB | 12 文件, ~30KB | **15%** |

**结论**: 当前实现是 **简化版本**，非 1:1 Bevy 仿制。

---

## autozig-app vs bevy_app

### Bevy 结构 (12 文件)

```
bevy_app/src/
├── app.rs              # 70KB - 核心 App 结构
├── main_schedule.rs    # 20KB - 主调度标签 (First, Update, Last 等)
├── plugin.rs           # 6KB  - Plugin trait
├── plugin_group.rs     # 28KB - PluginGroup, DefaultPlugins
├── sub_app.rs          # 18KB - SubApp 管理
├── schedule_runner.rs  # 7KB  - 调度执行器
├── task_pool_plugin.rs # 11KB - 任务池插件
├── propagate.rs        # 27KB - 组件传播
├── panic_handler.rs    # 3KB  - panic 处理
├── hotpatch.rs         # 2KB  - 热更新
├── terminal_ctrl_c_handler.rs # 3KB
└── lib.rs              # 2KB
```

### AutoZig 结构 (1 文件 + 4 Zig 文件)

```
autozig-app/src/
├── lib.rs              # 12KB - 全部实现
└── zig/
    ├── app.zig
    ├── sub_app.zig
    ├── plugin.zig
    └── runner.zig
```

### 功能对比

| Bevy 功能 | AutoZig | 状态 |
|----------|---------|------|
| `App::new()` | ✅ | 已实现 |
| `App::run()` | ✅ | 已实现 |
| `App::update()` | ✅ | 已实现 |
| `App::add_plugins()` | ✅ | 已实现 |
| `App::add_systems()` | ❌ | **缺失** |
| `App::insert_resource()` | ⚠️ | 简化版 |
| `SubApp` | ✅ | 基础实现 |
| `Plugin` trait | ⚠️ | 简化版 |
| `PluginGroup` | ❌ | **缺失** |
| `DefaultPlugins` | ❌ | **缺失** |
| 调度标签 (First, Update, Last) | ❌ | **缺失** |
| `ScheduleRunner` | ❌ | **缺失** |
| `TaskPoolPlugin` | ❌ | **缺失** |
| 热更新 | ❌ | 不需要 |

### 缺失的关键 API

```rust
// Bevy 支持，AutoZig 不支持:
app.add_systems(Startup, setup_system);
app.add_systems(Update, (system_a, system_b).chain());
app.configure_sets(Update, MySet::default());
app.init_resource::<MyResource>();
```

---

## autozig-ecs vs bevy_ecs

### Bevy 结构 (28 模块，1.5MB+)

```
bevy_ecs/src/
├── archetype.rs           # 35KB - Archetype 存储
├── bundle/                # 7 文件
├── change_detection/      # 5 文件 - 变更检测
├── component/             # 5 文件
├── entity/                # 12 文件
├── error/                 # 4 文件
├── event/                 # 2 文件
├── hierarchy.rs           # 41KB - 层级关系
├── message/               # 10 文件
├── observer/              # 6 文件 - Observer 系统
├── query/                 # 11 文件, 500KB+ - 查询系统
├── reflect/               # 8 文件
├── relationship/          # 4 文件
├── schedule/              # 17 文件 - 调度系统
├── storage/               # 7 文件 - 存储后端
├── system/                # 19 文件, 400KB+ - 系统
├── world/                 # 18 文件, 350KB+ - World
└── ...
```

### AutoZig 结构 (12 文件，~30KB)

```
autozig-ecs/src/
├── entity.rs       # 1KB
├── component.rs    # 3KB
├── world.rs        # 2KB
├── query.rs        # 2KB
├── system.rs       # 2KB
├── resource.rs     # 3KB
├── event.rs        # 4KB
├── command.rs      # 4KB
├── plugin.rs       # 4KB
├── system_param.rs # 4KB
├── into_system.rs  # 5KB
└── lib.rs          # 1KB
```

### 功能对比

| Bevy 功能 | AutoZig | 状态 |
|----------|---------|------|
| `Entity` | ✅ | 已实现 (简化) |
| `Component` trait | ⚠️ | 空 trait |
| `World` | ⚠️ | 仅 spawn/despawn |
| `Query<T>` | ❌ | **仅 entity 列表** |
| `Query<&T, With<U>>` | ❌ | **缺失** |
| `Commands` | ⚠️ | 基础实现 |
| `Res<T>` / `ResMut<T>` | ⚠️ | 简化版 |
| `Events<T>` | ✅ | 双缓冲实现 |
| `Schedule` | ⚠️ | 仅执行函数 |
| `SystemParam` | ⚠️ | 框架有，实现不完整 |
| `Archetype` 存储 | ❌ | **缺失** |
| `SparseSet` 存储 | ✅ | 已实现 |
| `Table` 存储 | ❌ | **缺失** |
| 变更检测 | ❌ | **缺失** |
| `Observer` | ❌ | **缺失** |
| `Bundle` | ❌ | **缺失** |
| `QueryState` | ⚠️ | 仅 entity 索引列表 |
| 并行查询 | ❌ | **缺失** |

### Bevy Query 的强大之处 (AutoZig 完全缺失)

```rust
// Bevy 的类型安全查询 - AutoZig 不支持
fn system(
    query: Query<(&Transform, &mut Velocity), With<Player>>,
    mut commands: Commands,
) {
    for (transform, mut vel) in &mut query {
        vel.0 += transform.translation;
    }
}

// Bevy 的过滤器组合 - AutoZig 不支持
Query<(&A, &B), (With<C>, Without<D>, Changed<A>)>
```

### AutoZig Query 的实际能力

```rust
// AutoZig 当前只能做到
let mut query = QueryState::new();
for entity_idx in positions.iter_entities() {
    query.add_entity(entity_idx);
}
// 无法直接获取组件数据！
```

---

## 代码量对比

### bevy_ecs 关键模块代码量

| 模块 | 文件数 | 总代码量 |
|------|-------|---------|
| `world/` | 11 文件 | ~350KB |
| `query/` | 11 文件 | ~540KB |
| `system/` | 19 文件 | ~430KB |
| `schedule/` | 17 文件 | ~300KB |
| `component/` | 5 文件 | ~50KB |
| **总计** | ~70 文件 | **~1.7MB** |

### autozig-ecs 总代码量

| 位置 | 文件数 | 总代码量 |
|------|-------|---------|
| `src/*.rs` | 12 文件 | ~30KB |
| `zig/*.zig` | 10 文件 | ~20KB |
| **总计** | 22 文件 | **~50KB** |

**比例: AutoZig = Bevy 的 ~3%**

---

## 结论

### ❌ 不是 1:1 Bevy 仿制

当前实现是:
- **App**: 基础框架，缺少调度标签、PluginGroup
- **ECS**: 极简版，缺少 Query 类型系统、Archetype 存储、变更检测

### 要达到 1:1 需要补充

#### autozig-app
1. `main_schedule.rs` - 调度标签 (First, PreUpdate, Update, PostUpdate, Last, Startup)
2. `plugin_group.rs` - PluginGroup trait + DefaultPlugins
3. `schedule_runner.rs` - 调度执行器

#### autozig-ecs
1. **Query 类型系统** - `Query<&T, With<U>>` 泛型支持
2. **Archetype 存储** - Table + SparseSet 双模式
3. **变更检测** - `Changed<T>`, `Added<T>`
4. **Bundle** - 组件批量添加
5. **System 参数解析** - 自动从 World 提取参数

### 建议

如果目标是"能跑 Bevy 示例代码"，需要:

| 模块 | 预估工时 |
|------|---------|
| 调度标签 (Schedule Labels) | 1天 |
| PluginGroup + DefaultPlugins | 1天 |
| Query 泛型系统 | 3-5天 |
| Archetype 存储 | 3天 |
| 变更检测 | 2天 |
| Bundle 系统 | 1天 |
| **总计** | **11-13天** |
