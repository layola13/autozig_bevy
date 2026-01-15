//! IntoSystem - Convert closures into systems

use autozig_macro::include_zig;
use crate::system_param::{SystemParam, WorldAccessFlags};
use crate::world::World;
use std::marker::PhantomData;

pub struct FunctionMarker<T>(pub PhantomData<T>);

// Opaque pointer to Zig ClosureSystemRegistry
#[repr(C)]
pub struct ClosureRegistryOpaque {
    _private: u8,
}

// Import Zig functions
include_zig!("src/zig/system_closure.zig", {
    fn closure_registry_create() -> *mut ClosureRegistryOpaque;
    fn closure_registry_destroy(registry: *mut ClosureRegistryOpaque);
    fn closure_registry_register(
        registry: *mut ClosureRegistryOpaque,
        name_ptr: *const u8,
        name_len: usize,
        data_ptr: *mut std::ffi::c_void,
        vtable_ptr: *mut std::ffi::c_void,
        trampoline: TrampolineFn,
        access_flags: u8,
    ) -> bool;
    fn closure_registry_run_all(
        registry: *mut ClosureRegistryOpaque,
        world: *mut std::ffi::c_void,
    );
    fn closure_registry_system_count(registry: *const ClosureRegistryOpaque) -> usize;
});

// Trampoline function type - bridges from Zig to Rust closure
type TrampolineFn = fn(
    closure_ptr: *mut std::ffi::c_void,
    world_ptr: *mut std::ffi::c_void,
);

/// Closure system registry - manages closure-based systems
pub struct ClosureRegistry {
    inner: *mut ClosureRegistryOpaque,
}

impl ClosureRegistry {
    pub fn new() -> Self {
        let inner = closure_registry_create();
        Self { inner }
    }

    pub fn register(&mut self, system: BoxedSystem) -> bool {
        let name = "closure_system"; // TODO: generate unique names
        // Access flags now handled via SystemMeta but we need to ensure initialization
        // let access_flags = system.access.to_u8();
        let access_flags = 0; // TODO: properly extracting access from meta requires init state first
        
        // TODO: Implement proper fat pointer extraction and trampoline registration
        // This is a placeholder - actual implementation needs to properly handle
        // the closure fat pointer and create appropriate trampoline
        false
    }

    pub fn run_all(&mut self, world: &mut World) {
        let world_ptr = world as *mut World as *mut std::ffi::c_void;
        closure_registry_run_all(self.inner, world_ptr);
    }

    pub fn system_count(&self) -> usize {
        closure_registry_system_count(self.inner)
    }
}

impl Drop for ClosureRegistry {
    fn drop(&mut self) {
        closure_registry_destroy(self.inner);
    }
}

impl Default for ClosureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// BoxedSystem is imported from crate::system


// Re-export types used by schedule.rs
pub use crate::system::{BoxedSystem, RawClosure, SystemTrampolineFn};

/// IntoSystem trait - converts closures into systems
pub trait IntoSystem<Marker, In = (), Out = ()> {
    type System: crate::system::System<In = In, Out = Out>;
    fn into_system(self) -> Self::System;
}

use crate::system::System;
use crate::system::SystemMeta;

// SystemParamFunction trait
pub trait SystemParamFunction<Marker>: Send + Sync + 'static {
    type Param: SystemParam;
    type In;
    type Out;
    fn run(&mut self, input: Self::In, param: <Self::Param as SystemParam>::Item<'_>) -> Self::Out;
}

// ParamFunctionSystem - System wrapper for functions with SystemParams
pub struct ParamFunctionSystem<Marker, F>
where
    F: SystemParamFunction<Marker>,
{
    func: F,
    state: Option<<F::Param as SystemParam>::State>,
    meta: SystemMeta,
    _marker: PhantomData<fn(Marker)>, // Use fn(Marker) to avoid Send/Sync requirements for Marker
}

impl<Marker, F> ParamFunctionSystem<Marker, F>
where
    F: SystemParamFunction<Marker>,
{
    pub fn new(func: F, name: &str) -> Self {
        Self {
            func,
            state: None,
            meta: SystemMeta::new(name),
            _marker: PhantomData,
        }
    }
}

impl<Marker, F> System for ParamFunctionSystem<Marker, F>
where
    Marker: 'static,
    F: SystemParamFunction<Marker, In = ()>,
{
    type In = ();
    type Out = F::Out;

    fn initialize(&mut self, world: &mut World) {
        if self.state.is_none() {
            self.state = Some(F::Param::init_state(world, &mut self.meta));
        }
    }

    fn run(&mut self, _input: (), world: &mut World) -> F::Out {
        self.run_with_out(world)
    }

    fn name(&self) -> &str {
        self.meta.name()
    }
}

pub struct SystemMarker;

impl<S: System> IntoSystem<SystemMarker, S::In, S::Out> for S {
    type System = S;
    fn into_system(self) -> Self::System {
        self
    }
}

impl<Marker, Out, F> IntoSystem<FunctionMarker<Marker>, (), Out> for F
where
    Marker: 'static,
    F: SystemParamFunction<FunctionMarker<Marker>, In = (), Out = Out>,
{
    type System = ParamFunctionSystem<FunctionMarker<Marker>, F>;
    fn into_system(self) -> Self::System {
        let name = std::any::type_name::<F>();
        ParamFunctionSystem::<FunctionMarker<Marker>, F>::new(self, name)
    }
}

impl<Marker, F> ParamFunctionSystem<Marker, F>
where
    Marker: 'static,
    F: SystemParamFunction<Marker, In = ()>,
{
    pub fn run_with_out(&mut self, world: &mut World) -> F::Out {
         if self.state.is_none() {
            self.state = Some(F::Param::init_state(world, &mut self.meta));
        }

        let change_tick = world.change_tick().0;
        let state = self.state.as_mut().unwrap();
        let params = F::Param::get_param(state, &self.meta, world, change_tick);
        let out = self.func.run((), params);
        F::Param::apply(state, &self.meta, world);
        self.meta.last_run = crate::change_detection::Tick::new(change_tick);
        out
    }
}

use crate::condition::{Condition, IntoCondition};

impl<Marker, F> Condition for ParamFunctionSystem<Marker, F>
where
    F: SystemParamFunction<Marker, In = (), Out = bool>,
    Marker: 'static,
{
    fn run(&mut self, world: &mut World) -> bool {
        self.run_with_out(world)
    }
}

impl<Marker, F> IntoCondition<Marker> for F
where
    F: SystemParamFunction<Marker, In = (), Out = bool>,
    Marker: 'static,
{
    type Condition = ParamFunctionSystem<Marker, F>;
    fn into_condition(self) -> Self::Condition {
        let name = std::any::type_name::<F>();
        ParamFunctionSystem::<Marker, F>::new(self, name)
    }
}


// Macro to implement SystemParamFunction for tuples
macro_rules! impl_system_param_function {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, F, $($param),*> SystemParamFunction<FunctionMarker<(Out, $($param,)*)>> for F
        where
            F: FnMut($($param::Item<'_>),*) -> Out + Send + Sync + 'static,
            $($param: SystemParam),*
        {
            type Param = ($($param,)*);
            type In = ();
            type Out = Out;
            fn run(&mut self, _input: (), param: <Self::Param as SystemParam>::Item<'_>) -> Out {
                #[allow(non_snake_case)]
                let ($($param,)*) = param;
                self($($param),*)
            }
        }
    };
}

// Generate implementations
impl_system_param_function!();
impl_system_param_function!(P1);
impl_system_param_function!(P1, P2);
impl_system_param_function!(P1, P2, P3);
impl_system_param_function!(P1, P2, P3, P4);
impl_system_param_function!(P1, P2, P3, P4, P5);
impl_system_param_function!(P1, P2, P3, P4, P5, P6);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
impl_system_param_function!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16);

pub struct ExclusiveSystemMarker;

impl<F> IntoSystem<ExclusiveSystemMarker, (), ()> for F
where
    F: FnMut(&mut crate::world::World) + Send + Sync + 'static,
{
    type System = crate::system::ExclusiveFunctionSystem<F>;
    fn into_system(self) -> Self::System {
        crate::system::ExclusiveFunctionSystem::new(self, std::any::type_name::<F>())
    }
}
