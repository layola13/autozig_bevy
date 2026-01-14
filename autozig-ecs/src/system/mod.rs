// System placeholder - to be implemented

use autozig_macro::include_zig;

pub mod meta;
pub use meta::SystemMeta;


#[repr(C)]
pub struct ScheduleOpaque {
    _private: u8,
}

// System function pointer type matching Zig
pub type SystemFn = extern "C" fn(*mut std::ffi::c_void);

include_zig!("src/zig/system.zig", {
    fn schedule_create() -> *mut ScheduleOpaque;
    fn schedule_destroy(schedule: *mut ScheduleOpaque);
    fn schedule_add_system(schedule: *mut ScheduleOpaque, name_ptr: *const u8, name_len: usize, func: SystemFn) -> bool;
    fn schedule_run(schedule: *mut ScheduleOpaque, world_ptr: *mut std::ffi::c_void);
    fn schedule_system_count(schedule: *const ScheduleOpaque) -> usize;
});

pub struct Schedule {
    inner: *mut ScheduleOpaque,
}

impl Schedule {
    pub fn new() -> Self {
        let inner = schedule_create();
        Self { inner }
    }
    
    pub fn add_system(&mut self, name: &str, func: SystemFn) -> bool {
        schedule_add_system(self.inner, name.as_ptr(), name.len(), func)
    }
    
    pub fn run(&mut self, world_ptr: *mut std::ffi::c_void) {
        schedule_run(self.inner, world_ptr);
    }
    
    pub fn system_count(&self) -> usize {
        schedule_system_count(self.inner)
    }
}

impl Drop for Schedule {
    fn drop(&mut self) {
        schedule_destroy(self.inner);
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

// System trait for user-defined systems
// System trait for user-defined systems
pub trait System: Send + Sync {
    type In;
    type Out;
    fn initialize(&mut self, _world: &mut crate::world::World) {}
    fn run(&mut self, input: Self::In, world: &mut crate::world::World) -> Self::Out;
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}


// ============================================================================
// System Advanced Types - System高级类型
// ============================================================================
// ... (lines 74-266 unchanged)

// BoxedSystem - 类型擦除的系统（Box包装）
pub struct BoxedSystem {
    inner: Box<dyn System<In=(), Out=()>>,
    meta: SystemMeta,
}

impl BoxedSystem {
    pub fn new<S: System<In=(), Out=()> + 'static>(system: S, name: &'static str) -> Self {
        Self {
            inner: Box::new(system),
            meta: SystemMeta::new(name),
        }
    }
    
    pub fn from_inner(inner: Box<dyn System<In=(), Out=()>>, meta: SystemMeta) -> Self {
        Self { inner, meta }
    }
    
    pub fn meta(&self) -> &SystemMeta {
        &self.meta
    }
    
    pub fn into_raw_parts(self) -> (*mut u8, *mut u8) {
        let raw = Box::into_raw(self.inner);
        // Split into data ptr and vtable ptr
        let (data_ptr, vtable_ptr) = (raw as *mut u8, std::ptr::null_mut::<u8>());
        (data_ptr, vtable_ptr)
    }
    
    pub fn name(&self) -> &str {
        self.meta.name()
    }
    
    pub fn run(&mut self, world: &mut crate::world::World) {
        self.inner.run((), world);
    }
}

impl System for BoxedSystem {
    type In = ();
    type Out = ();
    fn initialize(&mut self, world: &mut crate::world::World) {
        self.inner.initialize(world);
    }

    fn run(&mut self, _input: (), world: &mut crate::world::World) {
        self.inner.run((), world);
    }
    
    fn name(&self) -> &str {
        self.meta.name()
    }
}

// ... AdapterSystem (needs update if used) ...

/// ChainSystem - Chained system execution A -> B
pub struct ChainSystem<A, B> {
    system_a: A,
    system_b: B,
}

impl<A, B> ChainSystem<A, B> {
    pub fn new(system_a: A, system_b: B) -> Self {
        Self { system_a, system_b }
    }
}

impl<A, B> System for ChainSystem<A, B> 
where 
    A: System, 
    B: System<In = A::Out>,
{
    type In = A::In;
    type Out = B::Out;

    fn initialize(&mut self, world: &mut crate::world::World) {
        self.system_a.initialize(world);
        self.system_b.initialize(world);
    }

    fn run(&mut self, input: Self::In, world: &mut crate::world::World) -> Self::Out {
        // Direct passing, no resource hack needed if run signature supports it!
        let out_a = self.system_a.run(input, world);
        self.system_b.run(out_a, world)
    }
    
    fn name(&self) -> &str {
        "ChainSystem" // could combine names
    }
}

// CombinatorSystem removed/replaced by ChainSystem or similar?
// Keeping CombinatorSystem as generic (if used). 
// But generic CombinatorSystem<A, B> implies independent execution?
// If independent, A::Out and B::Out discarded?
// Assuming CombinatorSystem was previous attempt.
// ChainSystem is what we want.


// ============================================================================
// System Advanced Types - System高级类型
// ============================================================================

use std::marker::PhantomData;

/// SystemId - 系统ID，用于唯一标识已注册的系统
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemId(pub u64);

impl SystemId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// CachedSystemId - 缓存的系统ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CachedSystemId {
    id: SystemId,
    generation: u32,
}

impl CachedSystemId {
    pub fn new(id: SystemId, generation: u32) -> Self {
        Self { id, generation }
    }
    
    pub fn id(&self) -> SystemId {
        self.id
    }
    
    pub fn generation(&self) -> u32 {
        self.generation
    }
}

// SystemMeta moved to meta.rs

/// SystemName - 系统名称包装器
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemName(pub String);

impl SystemName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SystemName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for SystemName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// SystemTypeSet - 系统类型集合（用于系统排序）
pub struct SystemTypeSet<T> {
    _phantom: PhantomData<T>,
}

impl<T> SystemTypeSet<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for SystemTypeSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// System Input/Output Types - 系统输入输出类型
// ============================================================================

/// In<T> - 系统输入参数包装器
pub struct In<T>(pub T);

impl<T> In<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
    
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// SystemIn - 系统输入trait
pub trait SystemInput: Send + Sync + 'static {
    type Inner;
}

impl<T: Send + Sync + 'static> SystemInput for In<T> {
    type Inner = T;
}

/// SystemOut - 系统输出trait
pub trait SystemOut: Send + Sync + 'static {}

// 泛型实现已经覆盖了所有类型包括()
impl<T: Send + Sync + 'static> SystemOut for T {}

// ============================================================================
// System Error Types - 系统错误类型
// ============================================================================

/// RegisteredSystemError - 系统注册错误
#[derive(Debug, Clone)]
pub enum RegisteredSystemError {
    /// 系统ID不存在
    SystemIdNotFound(SystemId),
    /// 系统名称冲突
    DuplicateSystemName(String),
    /// 系统已被移除
    SystemRemoved(SystemId),
}

impl std::fmt::Display for RegisteredSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemIdNotFound(id) => write!(f, "System ID {:?} not found", id),
            Self::DuplicateSystemName(name) => write!(f, "Duplicate system name: {}", name),
            Self::SystemRemoved(id) => write!(f, "System {:?} has been removed", id),
        }
    }
}

impl std::error::Error for RegisteredSystemError {}

/// RunSystemError - 系统运行错误
#[derive(Debug, Clone)]
pub enum RunSystemError {
    /// 系统不存在
    SystemNotFound(SystemId),
    /// 系统访问冲突
    AccessConflict(String),
    /// 系统执行失败
    ExecutionFailed(String),
}

impl std::fmt::Display for RunSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemNotFound(id) => write!(f, "System {:?} not found", id),
            Self::AccessConflict(msg) => write!(f, "Access conflict: {}", msg),
            Self::ExecutionFailed(msg) => write!(f, "System execution failed: {}", msg),
        }
    }
}

impl std::error::Error for RunSystemError {}

// ============================================================================
// Advanced System Types - 高级系统类型
// ============================================================================

#[repr(C)]
pub struct RawClosure {
    pub data: *mut std::ffi::c_void,
    pub vtable: *mut std::ffi::c_void,
}

pub type SystemTrampolineFn = unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void);

unsafe extern "C" fn system_trampoline(closure: *mut std::ffi::c_void, world_ptr: *mut std::ffi::c_void) {
    use std::io::Write;
    let closure_ptr = closure as *mut RawClosure;
    if closure_ptr.is_null() {
        let _ = writeln!(std::io::stderr(), "ERROR: Closure pointer is null");
        return;
    }

    let data = unsafe { (*closure_ptr).data };
    let vtable = unsafe { (*closure_ptr).vtable };
    
    // Transmute to trait object with In=(), Out=()
    let ptr: *mut dyn System<In=(), Out=()> = unsafe { std::mem::transmute((data, vtable)) };
    
    let world = unsafe { &mut *(world_ptr as *mut crate::world::World) };
    (*ptr).run((), world);
}

// Note: BoxedSystem, ChainSystem are already defined above (via previous insert).
// We should NOT redefine them here to avoid conflicts.
// But we need to redefine FunctionSystem etc.

// FunctionSystem
pub struct FunctionSystem<F> {
    func: F,
    meta: SystemMeta,
}

impl<F> FunctionSystem<F>
where
    F: FnMut(&mut crate::world::World) + Send + Sync + 'static,
{
    pub fn new(func: F, name: &'static str) -> Self {
        Self {
            func,
            meta: SystemMeta::new(name),
        }
    }
}

impl<F> System for FunctionSystem<F>
where
    F: FnMut(&mut crate::world::World) + Send + Sync + 'static,
{
    type In = ();
    type Out = ();
    
    fn run(&mut self, _input: (), world: &mut crate::world::World) {
        (self.func)(world);
    }
    
    fn name(&self) -> &str {
        self.meta.name()
    }
}

// ExclusiveFunctionSystem
pub struct ExclusiveFunctionSystem<F> {
    func: F,
    meta: SystemMeta,
}

impl<F> ExclusiveFunctionSystem<F>
where
    F: FnMut(&mut crate::world::World) + Send + Sync + 'static,
{
    pub fn new(func: F, name: &'static str) -> Self {
        let mut meta = SystemMeta::new(name);
        meta.is_exclusive = true;
        Self { func, meta }
    }
}

impl<F> System for ExclusiveFunctionSystem<F>
where
    F: FnMut(&mut crate::world::World) + Send + Sync + 'static,
{
    type In = ();
    type Out = ();
    
    fn run(&mut self, _input: (), world: &mut crate::world::World) {
        (self.func)(world);
    }
    
    fn name(&self) -> &str {
        self.meta.name()
    }
}

// Removed: CombinatorSystem (replaced by ChainSystem), AdapterSystem (simplified out), 
// BoxedSystem (defined above)


/// ExclusiveSystem - 独占系统trait（需要独占访问World）
pub trait ExclusiveSystem: Send + Sync + 'static {
    fn run(&mut self, world: &mut crate::world::World);
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

/// SystemAdapter - 系统适配器trait，用于包装和转换系统
pub trait SystemAdapter: Send + Sync + 'static {
    type System: System;
    
    fn adapt(&mut self, system: &mut Self::System, world: &mut crate::world::World);
}

// ============================================================================
// System Parameter Types - 系统参数类型
// ============================================================================

/// NonSend<T> - 非Send资源参数
pub struct NonSend<'w, T: 'static> {
    _phantom: PhantomData<(&'w T, T)>,
}

/// NonSendMut<T> - 非Send可变资源参数
pub struct NonSendMut<'w, T: 'static> {
    _phantom: PhantomData<(&'w mut T, T)>,
}

/// CheckChangeTicks - 变更检查tick检查器
pub struct CheckChangeTicks {
    last_tick: u32,
    this_tick: u32,
}

impl CheckChangeTicks {
    pub fn new(last_tick: u32, this_tick: u32) -> Self {
        Self { last_tick, this_tick }
    }
    
    pub fn check_tick(&self, component_tick: u32) -> bool {
        component_tick > self.last_tick && component_tick <= self.this_tick
    }
}

/// EncapsulatedParam - 封装的系统参数
pub struct EncapsulatedParam<T> {
    value: T,
}

impl<T> EncapsulatedParam<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
    
    pub fn get(&self) -> &T {
        &self.value
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// DynSystemParam - 动态系统参数
pub struct DynSystemParam {
    _private: (),
}

/// DynSystemParamState - 动态系统参数状态
pub struct DynSystemParamState {
    _private: (),
}

/// DynParamBuilder - 动态参数构建器
pub struct DynParamBuilder {
    _private: (),
}

/// SystemState - 系统状态（用于手动系统管理）
pub struct SystemState<Params> {
    _phantom: PhantomData<Params>,
    meta: SystemMeta,
}

impl<Params> SystemState<Params> {
    pub fn new(world: &mut crate::world::World, name: &'static str) -> Self {
        Self {
            _phantom: PhantomData,
            meta: SystemMeta::new(name),
        }
    }
    
    pub fn meta(&self) -> &SystemMeta {
        &self.meta
    }
}

// ============================================================================
// System Traits - 系统trait扩展
// ============================================================================

/// SystemParamFunction - 系统参数函数trait
pub trait SystemParamFunction<Params>: Send + Sync + 'static {
    type Out;
    fn run(&mut self, params: Params) -> Self::Out;
}

/// ExclusiveSystemParamFunction - 独占系统参数函数trait
pub trait ExclusiveSystemParamFunction<Params>: Send + Sync + 'static {
    type Out;
    fn run(&mut self, world: &mut crate::world::World, params: Params) -> Self::Out;
}

/// RunSystemOnce - 运行系统一次trait
pub trait RunSystemOnce {
    type Out;
    fn run_once(self, world: &mut crate::world::World) -> Self::Out;
}

/// SystemBuffer - 系统缓冲区trait
pub trait SystemBuffer: Send + Sync + 'static {
    fn apply(&mut self, world: &mut crate::world::World);
}

/// SystemCondition - 系统条件trait
pub trait SystemCondition: Send + Sync + 'static {
    fn should_run(&mut self, world: &crate::world::World) -> bool;
}

/// FnRet - 函数返回值trait
pub trait FnRet {
    type Ret;
}

// 泛型实现已经覆盖所有类型包括()
impl<T> FnRet for T {
    type Ret = T;
}

/// FromInput - 从输入转换trait
pub trait FromInput<I> {
    fn from_input(input: I) -> Self;
}

/// IntoResult - 转换为Result trait
pub trait IntoResult<T, E> {
    fn into_result(self) -> Result<T, E>;
}

impl<T> IntoResult<T, ()> for T {
    fn into_result(self) -> Result<T, ()> {
        Ok(self)
    }
}

impl<T, E> IntoResult<T, E> for Result<T, E> {
    fn into_result(self) -> Result<T, E> {
        self
    }
}
