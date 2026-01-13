//! IntoSystem - Convert closures into systems

use autozig_macro::include_zig;
use crate::system_param::{SystemParam, WorldAccessFlags};
use crate::world::World;
use std::marker::PhantomData;

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
pub trait IntoSystem<Params> {
    fn into_system(self) -> crate::system::BoxedSystem;
}

use crate::system::System;
use crate::system::SystemMeta;

// SystemParamFunction trait
pub trait SystemParamFunction<Marker>: Send + Sync + 'static {
    type Param: SystemParam;
    fn run(&mut self, param: <Self::Param as SystemParam>::Item<'_>);
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
    F: SystemParamFunction<Marker>,
    Marker: Send + Sync + 'static,
{
    fn initialize(&mut self, world: &mut World) {
        self.state = Some(F::Param::init_state(world, &mut self.meta));
    }

    fn run(&mut self, world: &mut World) {
        let change_tick = world.change_tick().0;
        let state = self.state.as_mut().expect("System not initialized");
        unsafe {
            let params = F::Param::get_param(state, &self.meta, world, change_tick);
            self.func.run(params);
        }
        self.meta.last_run = change_tick;
    }
}

// Implement IntoSystem for SystemParamFunction
impl<Marker, F> IntoSystem<Marker> for F
where
    F: SystemParamFunction<Marker>,
    Marker: Send + Sync + 'static,
{
    fn into_system(self) -> crate::system::BoxedSystem {
        let name = "function_system"; // TODO: Use better naming or type_name
        let system = ParamFunctionSystem::new(self, name);
        crate::system::BoxedSystem::new(system, name)
    }
}


// Macro to implement SystemParamFunction for tuples
macro_rules! impl_system_param_function {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<F, $($param),*> SystemParamFunction<($($param,)*)> for F
        where
            F: FnMut($($param::Item<'_>),*) + Send + Sync + 'static,
            $($param: SystemParam),*
        {
            type Param = ($($param),*);
            fn run(&mut self, param: ($($param::Item<'_>),*)) {
                #[allow(non_snake_case)]
                let ($($param),*) = param;
                self($($param),*);
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
