
# Autozig Bevy App - 链式调用实现 TODO

> 参考文档: `autozig_bevy/docs/AUTOZIG_BEVY_APP_CHAINING.md`
> 
> 对比参考: `bevy/crates/bevy_app/src/app.rs` (2022 lines)
> 
> 当前状态: `autozig_bevy/autozig-app/src/lib.rs` (227 lines)

## ✅ 已完成

- [x] 基础 App 结构
- [x] Zig 后端 FFI 集成
- [x] Plugin trait 定义
- [x] PluginGroup 基础实现
- [x] SubApp 支持
- [x] 基础资源管理

## 🔴 Phase 1: 核心链式调用支持 (紧急)

### 1.1 方法返回值修改 (预计: 2-3小时)

**文件**: `autozig_bevy/autozig-app/src/lib.rs`

- [ ] **修改 `add_plugin` 返回 `&mut Self`**
  ```rust
  // 当前: pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String>
  // 改为: pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self
  ```
  - 移除 Result 返回类型
  - 失败时使用 panic! 而非返回错误
  - 在方法末尾返回 `self`

- [ ] **修改 `insert_resource` 返回 `&mut Self`**
  ```rust
  // 当前: pub fn insert_resource<R: Resource>(&mut self, resource: R)
  // 改为: pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self
  ```

- [ ] **修改 `init_resource` 返回 `&mut Self`**
  ```rust
  pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self
  ```

- [ ] **添加 `set_runner` 返回 `&mut Self`**
  ```rust
  pub fn set_runner(&mut self, runner: impl FnOnce(App) -> AppExit + 'static) -> &mut Self
  ```

- [ ] **添加 `add_schedule` 返回 `&mut Self`**
  ```rust
  pub fn add_schedule(&mut self, schedule: Schedule) -> &mut Self
  ```

- [ ] **添加 `init_schedule` 返回 `&mut Self`**
  ```rust
  pub fn init_schedule(&mut self, label: impl ScheduleLabel) -> &mut Self
  ```

### 1.2 实现 Plugins Trait (预计: 3-4小时)

**新文件**: `autozig_bevy/autozig-app/src/plugins_trait.rs`

- [ ] **定义 `Plugins<Marker>` trait**
  ```rust
  pub trait Plugins<Marker>: sealed::Plugins<Marker> {}
  impl<Marker, T> Plugins<Marker> for T where T: sealed::Plugins<Marker> {}
  ```

- [ ] **实现 sealed module**
  ```rust
  mod sealed {
      pub trait Plugins<Marker> {
          fn add_to_app(self, app: &mut App);
      }
      
      pub struct PluginMarker;
      pub struct PluginGroupMarker;
      pub struct PluginsTupleMarker;
  }
  ```

- [ ] **为单个 Plugin 实现**
  ```rust
  impl<P: Plugin> Plugins<PluginMarker> for P {
      fn add_to_app(self, app: &mut App) {
          app.add_plugin(self);
      }
  }
  ```

- [ ] **为 PluginGroup 实现**
  ```rust
  impl<P: PluginGroup> Plugins<PluginGroupMarker> for P {
      fn add_to_app(self, app: &mut App) {
          self.build().finish(app);
      }
  }
  ```

- [ ] **为元组实现 (2-15个元素)**
  - 使用 macro_rules! 生成
  - 或使用 variadics_please crate
  ```rust
  impl<P1: Plugin, P2: Plugin> Plugins<(P1, P2)> for (P1, P2) {
      fn add_to_app(self, app: &mut App) {
          app.add_plugin(self.0);
          app.add_plugin(self.1);
      }
  }
  ```

- [ ] **添加 `add_plugins` 方法到 App**
  ```rust
  pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
      plugins.add_to_app(self);
      self
  }
  ```

### 1.3 错误处理优化 (预计: 1-2小时)

**文件**: `autozig_bevy/autozig-app/src/lib.rs`

- [ ] **定义 AppError 枚举**
  ```rust
  #[derive(Debug, thiserror::Error)]
  pub enum AppError {
      #[error("duplicate plugin {plugin_name:?}")]
      DuplicatePlugin { plugin_name: String },
  }
  ```

- [ ] **实现 `add_boxed_plugin` (内部方法)**
  ```rust
  pub(crate) fn add_boxed_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<&mut Self, AppError>
  ```

- [ ] **提供 try_* 版本的方法**
  ```rust
  pub fn try_add_plugin(&mut self, plugin: impl Plugin) -> Result<&mut Self, AppError>
  ```

### 1.4 测试链式调用 (预计: 1小时)

**新文件**: `autozig_bevy/autozig-app/tests/chaining_test.rs`

- [ ] **测试基本链式调用**
  ```rust
  #[test]
  fn test_basic_chaining() {
      App::new()
          .add_plugin(TestPlugin)
          .insert_resource(TestResource)
          .init_resource::<AnotherResource>();
  }
  ```

- [ ] **测试多插件链式调用**
  ```rust
  #[test]
  fn test_multi_plugin_chaining() {
      App::new()
          .add_plugins((PluginA, PluginB, PluginC));
  }
  ```

- [ ] **测试错误处理**
  ```rust
  #[test]
  #[should_panic]
  fn test_duplicate_plugin_panics() {
      App::new()
          .add_plugin(UniquePlugin)
          .add_plugin(UniquePlugin); // Should panic
  }
  ```

## 🟡 Phase 2: Schedule 系统 (重要)

### 2.1 Schedule API 实现 (预计: 4-5小时)

**文件**: `autozig_bevy/autozig-app/src/schedule.rs`

- [ ] **实现 ScheduleLabel trait**
  ```rust
  pub trait ScheduleLabel: Debug + Hash + Eq + Clone + Send + Sync + 'static {}
  ```

- [ ] **添加 `add_systems` 方法**
  ```rust
  pub fn add_systems<M>(
      &mut self,
      schedule: impl ScheduleLabel,
      systems: impl IntoSystemConfigs<M>
  ) -> &mut Self
  ```

- [ ] **添加 `configure_sets` 方法**
  ```rust
  pub fn configure_sets<M>(
      &mut self,
      schedule: impl ScheduleLabel,
      sets: impl IntoSystemConfigs<M>
  ) -> &mut Self
  ```

- [ ] **实现 `edit_schedule` 方法**
  ```rust
  pub fn edit_schedule(
      &mut self,
      label: impl ScheduleLabel,
      f: impl FnMut(&mut Schedule)
  ) -> &mut Self
  ```

- [ ] **添加 `configure_schedules` 方法**
  ```rust
  pub fn configure_schedules(&mut self, settings: ScheduleBuildSettings) -> &mut Self
  ```

### 2.2 System 配置支持 (预计: 3-4小时)

- [ ] **实现 IntoSystemConfigs trait**
  ```rust
  pub trait IntoSystemConfigs<Marker> {
      fn into_configs(self) -> SystemConfigs;
  }
  ```

- [ ] **实现 SystemConfigs 结构**
  ```rust
  pub struct SystemConfigs {
      systems: Vec<SystemConfig>,
  }
  ```

- [ ] **支持 system 元组**
  ```rust
  // 支持: .add_systems(Update, (system_a, system_b, system_c))
  impl<S1, S2> IntoSystemConfigs<(S1, S2)> for (S1, S2) where ...
  ```

### 2.3 Schedule 测试 (预计: 2小时)

**文件**: `autozig_bevy/autozig-app/tests/schedule_test.rs`

- [ ] **测试 add_systems**
- [ ] **测试 system 顺序**
- [ ] **测试 system sets**

## 🟡 Phase 3: Resource 管理增强 (重要)

### 3.1 完善 Resource API (预计: 2-3小时)

**文件**: `autozig_bevy/autozig-app/src/lib.rs`

- [ ] **添加 NonSend Resource 支持**
  ```rust
  pub fn insert_non_send_resource<R: 'static>(&mut self, resource: R) -> &mut Self
  pub fn init_non_send_resource<R: 'static + FromWorld>(&mut self) -> &mut Self
  ```

- [ ] **改进类型安全**
  - 当前使用 `u64` 作为 type_id
  - 改为使用 `TypeId` 或泛型约束

### 3.2 Component 要求系统 (预计: 3-4小时)

**新文件**: `autozig_bevy/autozig-app/src/required_components.rs`

- [ ] **实现 `register_required_components`**
  ```rust
  pub fn register_required_components<T: Component, R: Component + Default>(&mut self) -> &mut Self
  ```

- [ ] **实现 `register_required_components_with`**
  ```rust
  pub fn register_required_components_with<T: Component, R: Component>(
      &mut self,
      constructor: fn() -> R
  ) -> &mut Self
  ```

- [ ] **实现 try_* 版本**
  ```rust
  pub fn try_register_required_components<T, R>(&mut self) -> Result<(), RequiredComponentsError>
  ```

## 🟢 Phase 4: SubApp 管理 (可选)

### 4.1 SubApp 访问方法 (预计: 2小时)

**文件**: `autozig_bevy/autozig-app/src/sub_app.rs`

- [ ] **实现 AppLabel trait**
  ```rust
  pub trait AppLabel: Debug + Hash + Eq + Clone + Send + Sync + 'static {}
  ```

- [ ] **添加 SubApp 访问方法**
  ```rust
  pub fn sub_app(&self, label: impl AppLabel) -> &SubApp
  pub fn sub_app_mut(&mut self, label: impl AppLabel) -> &mut SubApp
  pub fn get_sub_app(&self, label: impl AppLabel) -> Option<&SubApp>
  pub fn get_sub_app_mut(&mut self, label: impl AppLabel) -> Option<&mut SubApp>
  ```

- [ ] **实现 SubApp 管理方法**
  ```rust
  pub fn insert_sub_app(&mut self, label: impl AppLabel, sub_app: SubApp)
  pub fn remove_sub_app(&mut self, label: impl AppLabel) -> Option<SubApp>
  pub fn update_sub_app_by_label(&mut self, label: impl AppLabel)
  ```

### 4.2 SubApps 集合 (预计: 2小时)

- [ ] **实现 SubApps 结构**
  ```rust
  pub struct SubApps {
      main: SubApp,
      sub_apps: HashMap<InternedAppLabel, SubApp>,
  }
  ```

- [ ] **添加访问方法**
  ```rust
  pub fn sub_apps(&self) -> &SubApps
  pub fn sub_apps_mut(&mut self) -> &mut SubApps
  ```

## 🟢 Phase 5: 高级功能 (可选)

### 5.1 Observer 系统 (预计: 4-5小时)

- [ ] **实现 Observer trait**
- [ ] **添加 `add_observer` 方法**
  ```rust
  pub fn add_observer<E: Event, B: Bundle, M>(
      &mut self,
      observer: impl IntoObserverSystem<E, B, M>
  ) -> &mut Self
  ```

### 5.2 Event 系统 (预计: 3-4小时)

- [ ] **实现 Message trait**
- [ ] **添加 `add_message` 方法**
  ```rust
  pub fn add_message<M: Message>(&mut self) -> &mut Self
  ```
- [ ] **实现 `should_exit` 方法**
  ```rust
  pub fn should_exit(&self) -> Option<AppExit>
  ```

### 5.3 Reflection 支持 (预计: 5-6小时)

- [ ] **添加 bevy_reflect 集成**
- [ ] **实现 `register_type` 方法**
- [ ] **实现 `register_type_data` 方法**
- [ ] **实现函数注册方法**

### 5.4 错误处理器 (预计: 2小时)

- [ ] **实现 ErrorHandler 系统**
  ```rust
  pub fn set_error_handler(&mut self, handler: ErrorHandler) -> &mut Self
  pub fn get_error_handler(&self) -> Option<ErrorHandler>
  ```

## 📚 Phase 6: 文档和示例 (重要)

### 6.1 示例代码 (预计: 3-4小时)

**新文件**: `autozig_bevy/autozig-app/examples/`

- [ ] **basic_chaining.rs** - 基础链式调用示例
  ```rust
  fn main() {
      App::new()
          .add_plugins(DefaultPlugins)
          .insert_resource(GameSettings::default())
          .add_systems(Startup, setup)
          .add_systems(Update, (update_game, render))
          .run();
  }
  ```

- [ ] **multi_plugin.rs** - 多插件链式调用
  ```rust
  fn main() {
      App::new()
          .add_plugins((
              CorePlugin,
              PhysicsPlugin,
              RenderPlugin,
              AudioPlugin,
          ))
          .run();
  }
  ```

- [ ] **custom_schedule.rs** - 自定义 Schedule
  ```rust
  fn main() {
      App::new()
          .add_schedule(Schedule::new(MySchedule))
          .add_systems(MySchedule, my_systems)
          .run();
  }
  ```

- [ ] **sub_app.rs** - SubApp 使用示例
  ```rust
  fn main() {
      let mut app = App::new();
      app.insert_sub_app(RenderApp, SubApp::new());
      app.run();
  }
  ```

### 6.2 API 文档 (预计: 4-5小时)

- [ ] **为所有 public 方法添加文档注释**
- [ ] **添加使用示例到文档**
- [ ] **生成 rustdoc 并检查**

### 6.3 Migration Guide (预计: 2-3小时)

**新文件**: `autozig_bevy/autozig-app/MIGRATION.md`

- [ ] **从旧 API 迁移指南**
- [ ] **与 bevy_app 的差异说明**
- [ ] **常见问题解答**

### 6.4 集成测试 (预计: 4-5小时)

**新文件**: `autozig_bevy/autozig-app/tests/integration_test.rs`

- [ ] **完整应用流程测试**
- [ ] **插件系统测试**
- [ ] **资源管理测试**
- [ ] **Schedule 执行测试**
- [ ] **与 Zig 后端集成测试**

## 🔧 依赖项更新

**文件**: `autozig_bevy/autozig-app/Cargo.toml`

- [ ] **添加必要依赖**
  ```toml
  [dependencies]
  bevy_ecs = "0.15"  # 或最新版本
  thiserror = "1.0"
  # variadics_please = "0.1"  # 可选，用于元组实现
  ```

## 📊 进度追踪

- **总体进度**: 0% (0/80+ 任务完成)
- **Phase 1 (核心)**: 0% (0/20 完成) - 预计 10-12 小时
- **Phase 2 (Schedule)**: 0% (0/15 完成) - 预计 9-11 小时  
- **Phase 3 (Resource)**: 0% (0/10 完成) - 预计 5-7 小时
- **Phase 4 (SubApp)**: 0% 