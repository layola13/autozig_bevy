
# Autozig Bevy App 链式调用改进方案

## 当前实现评估

### 已实现功能

autozig_bevy/autozig-app 模块当前已经实现了以下核心功能：

1. **基础结构** ✅
   - `App` 结构：包含 main_sub_app、sub_apps、runner、plugins 等
   - Zig 后端：`ZigApp` 提供高性能状态管理
   - 插件系统：`Plugin` trait 和 `PluginGroup` 支持

2. **Zig FFI 集成** ✅
   - 完整的 FFI 导出函数
   - 内存安全的资源管理
   - SubApp 支持

3. **插件架构** ✅
   - Plugin trait with build/ready/finish/cleanup hooks
   - PluginGroupBuilder for plugin ordering
   - Plugin state management

### 缺失功能（阻碍链式调用）

对比 `bevy/crates/bevy_app/src/app.rs` (2022行) 的实现，当前 `autozig_bevy/autozig-app/src/lib.rs` (227行) 缺少：

## 🎯 核心问题：缺少返回 `&mut Self` 的方法

**当前状态**：
```rust
// autozig_bevy/autozig-app/src/lib.rs 中的方法都不返回 &mut Self
pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String> {
    // ... 实现
    Ok(())  // ❌ 返回 Result，无法链式调用
}

pub fn insert_resource<R: Resource>(&mut self, resource: R) {
    // ... 实现
    // ❌ 没有返回值，无法链式调用
}
```

**期望状态** (参考 bevy_app):
```rust
pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
    // ... 实现
    self  // ✅ 返回 &mut Self，支持链式调用
}

pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
    // ... 实现
    self  // ✅ 返回 &mut Self，支持链式调用
}
```

## 📋 改进任务清单

### Phase 1: 核心链式调用支持 (优先级: 🔴 高)

#### 1.1 修改所有配置方法返回 `&mut Self`

**文件**: `autozig_bevy/autozig-app/src/lib.rs`

需要修改的方法：

- [ ] `add_plugin(&mut self, plugin: impl Plugin) -> &mut Self`
- [ ] `add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self`
- [ ] `insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self`
- [ ] `init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self`
- [ ] `add_systems<M>(&mut self, schedule: impl ScheduleLabel, systems: impl IntoSystemConfigs<M>) -> &mut Self`
- [ ] `configure_sets<M>(&mut self, schedule: impl ScheduleLabel, sets: impl IntoSystemConfigs<M>) -> &mut Self`
- [ ] `set_runner(&mut self, runner: impl FnOnce(App) -> AppExit + 'static) -> &mut Self`
- [ ] `add_schedule(&mut self, schedule: Schedule) -> &mut Self`
- [ ] `init_schedule(&mut self, label: impl ScheduleLabel) -> &mut Self`
- [ ] `edit_schedule(&mut self, label: impl ScheduleLabel, f: impl FnMut(&mut Schedule)) -> &mut Self`

**示例重构**：
```rust
// 当前
pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String> {
    self.zig_app.add_plugin(plugin)?;
    Ok(())
}

// 改为
pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
    if let Err(e) = self.add_boxed_plugin(Box::new(plugin)) {
        panic!("Failed to add plugin: {}", e);
    }
    self
}
```

#### 1.2 实现 `Plugins` Trait 系统

**文件**: 新建 `autozig_bevy/autozig-app/src/plugins.rs`

- [ ] 定义 `Plugins<Marker>` trait
- [ ] 为单个 `Plugin` 实现
- [ ] 为 `PluginGroup` 实现
- [ ] 为元组实现 (支持 `add_plugins((PluginA, PluginB, PluginC))`)

**参考实现**：
```rust
pub trait Plugins<Marker>: sealed::Plugins<Marker> {}

impl<Marker, T> Plugins<Marker> for T where T: sealed::Plugins<Marker> {}

mod sealed {
    pub trait Plugins<Marker> {
        fn add_to_app(self, app: &mut App);
    }
    
    // 为单个 Plugin 实现
    impl<P: Plugin> Plugins<PluginMarker> for P {
        fn add_to_app(self, app: &mut App) {
            app.add_plugin(self);
        }
    }
    
    // 为元组实现 (使用 variadics_please 或手动实现)
    impl<P1: Plugin, P2: Plugin> Plugins<(P1, P2)> for (P1, P2) {
        fn add_to_app(self, app: &mut App) {
            app.add_plugin(self.0);
            app.add_plugin(self.1);
        }
    }
}
```

#### 1.3 优化错误处理

**当前问题**：返回 `Result<(), String>` 阻碍链式调用

**解决方案**：
- [ ] 使用 panic! 处理不可恢复错误（插件重复添加）
- [ ] 为需要错误处理的场景提供 `try_*` 版本
- [ ] 参考 bevy_app 的 `add_boxed_plugin` 错误处理模式

```rust
// 提供两个版本
pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
    if let Err(e) = self.try_add_plugin(plugin) {
        panic!("Error adding plugin: {}", e);
    }
    self
}

pub fn try_add_plugin(&mut self, plugin: impl Plugin) -> Result<&mut Self, AppError> {
    // ... 实现
    Ok(self)
}
```

### Phase 2: Schedule 系统集成 (优先级: 🟡 中)

#### 2.1 实现完整的 Schedule API

**文件**: `autozig_bevy/autozig-app/src/schedule.rs`

- [ ] `add_systems<M>(&mut self, schedule: impl ScheduleLabel, systems: impl IntoSystemConfigs<M>) -> &mut Self`
- [ ] `remove_systems_in_set<M>(&mut self, schedule: impl ScheduleLabel, set: impl IntoSystemSet<M>, policy: ScheduleCleanupPolicy) -> Result<usize, ScheduleError>`
- [ ] `configure_sets<M>(&mut self, schedule: impl ScheduleLabel, sets: impl IntoScheduleConfigs<M>) -> &mut Self`
- [ ] `configure_schedules(&mut self, settings: ScheduleBuildSettings) -> &mut Self`
- [ ] `allow_ambiguous_component<T: Component>(&mut self) -> &mut Self`
- [ ] `allow_ambiguous_resource<T: Resource>(&mut self) -> &mut Self`
- [ ] `ignore_ambiguity<M1, M2, S1, S2>(&mut self, schedule: impl ScheduleLabel, a: S1, b: S2) -> &mut Self`

#### 2.2 System 注册系统

- [ ] `register_system<I, O, M>(&mut self, system: impl IntoSystem<I, O, M>) -> SystemId<I, O>`
- [ ] 支持 `IntoSystemConfigs` trait
- [ ] 支持 `IntoSystemSet` trait

### Phase 3: Resource 管理增强 (优先级: 🟡 中)

#### 3.1 完善 Resource API

**文件**: `autozig_bevy/autozig-app/src/lib.rs`

- [ ] `insert_non_send_resource<R: 'static>(&mut self, resource: R) -> &mut Self`
- [ ] `init_non_send_resource<R: 'static + FromWorld>(&mut self) -> &mut Self`
- [ ] 类型安全的资源获取（当前使用 `u64` type_id）

#### 3.2 Component 要求系统

- [ ] `register_required_components<T: Component, R: Component + Default>(&mut self) -> &mut Self`
- [ ] `register_required_components_with<T: Component, R: Component>(&mut self, constructor: fn() -> R) -> &mut Self`
- [ ] `try_register_required_components<T: Component, R: Component + Default>(&mut self) -> Result<(), RequiredComponentsError>`
- [ ] `register_disabling_component<C: Component>(&mut self)`

### Phase 4: SubApp 管理完善 (优先级: 🟢 低)

#### 4.1 SubApp 访问方法

**文件**: `autozig_bevy/autozig-app/src/sub_app.rs`

- [ ] `sub_app(&self, label: impl AppLabel) -> &SubApp` - panic 版本
- [ ] `sub_app_mut(&mut self, label: impl AppLabel) -> &mut SubApp` - panic 版本
- [ ] `get_sub_app(&self, label: impl AppLabel) -> Option<&SubApp>` - Option 版本
- [ ] `get_sub_app_mut(&mut self, label: impl AppLabel) -> Option<&mut SubApp>` - Option 版本
- [ ] `insert_sub_app(&mut self, label: impl AppLabel, sub_app: SubApp)`
- [ ] `remove_sub_app(&mut self, label: impl AppLabel) -> Option<SubApp>`
- [ ] `update_sub_app_by_label(&mut self, label: impl AppLabel)`

#### 4.2 SubApps 集合

- [ ] 实现 `SubApps` 结构管理多个 SubApp
- [ ] `sub_apps(&self) -> &SubApps`
- [ ] `sub_apps_mut(&mut self) -> &mut SubApps`

### Phase 5: 高级功能 (优先级: 🟢 低)

#### 5.1 Observer 系统

- [ ] `add_observer<E: Event, B: Bundle, M>(&mut self, observer: impl IntoObserverSystem<E, B, M>) -> &mut Self`
- [ ] Observer 与 Zig 后端集成

#### 5.2 Message/Event 系统

- [ ] `add_message<M: Message>(&mut self) -> &mut Self`
- [ ] `should_exit(&self) -> Option<AppExit>`
- [ ] AppExit 消息处理

#### 5.3 Reflection 支持

- [ ] `register_type<T: GetTypeRegistration>(&mut self) -> &mut Self`
- [ ] `register_type_data<T, D>(&mut self) -> &mut Self`
- [ ] `register_function<F, Marker>(&mut self, function: F) -> &mut Self`
- [ ] `register_function_with_name<F, Marker>(&mut self, name: impl Into<Cow<'static, str>>, function: F) -> &mut Self`

#### 5.4 错误处理

- [ ] `set_error_handler(&mut self, handler: ErrorHandler) -> &mut Self`
- [ ] `get_error_handler(&self) -> Option<ErrorHandler>`

### Phase 6: 测试和文档 (优先级: 🔴 高)

#### 6.1 示例代码

**文件**: `autozig_bevy/autozig-app/examples/chaining.rs`

```rust
use autozig_bevy_app::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)           // ✅ 链式调用
        .insert_resource(GameSettings {        // ✅ 链式调用
            difficulty: Difficulty::Normal,
        })
        .add_systems(Startup, setup)           // ✅ 链式调用
        .add_systems(Update, (                 // ✅ 链式调用
            player_movement,
            enemy_ai,
            collision_detection,
        ))
        .run();                                 // ✅ 最终执行
}
```

#### 6.2 单元测试

- [ ] 测试链式调用语法
- [ ] 测试插件重复添加错误处理
- [ ] 测试 SubApp 操作
- [ ] 测试 Schedule 配置
- [ ] 对比 bevy_app 的测试用例

#### 6.3 文档

- [ ] API 文档完善
- [ ] 链式调用使用指南
- [ ] 与 bevy_app 的对比文档
- [ ] Migration guide

## 🎨 设计原则

### 1. API 兼容性优先

尽可能保持与 bevy_app 的 API 一致性：
- 方法名相同
- 参数类型相同
- 返回类型相同（`&mut Self` for chaining）

### 2. 渐进式增强

分阶段实现，每个阶段都可以独立测试和使用：
1. Phase 1 完成后即可支持基本链式调用
2. Phase 2-4 增强功能
3. Phase 5-6 完善体验

### 3. 零成本抽象

Zig 后端提供性能，Rust 前端提供人机工程学：
- FFI 调用开销最小化
- 使用 `#[inline]` 