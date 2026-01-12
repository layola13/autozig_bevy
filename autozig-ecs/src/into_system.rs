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
        let access_flags = system.access.to_u8();
        
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

/// Type-erased system containing a closure
pub struct BoxedSystem {
    pub(crate) closure: Box<dyn FnMut(&mut World) + Send + Sync>,
    pub(crate) access: WorldAccessFlags,
}

/// IntoSystem trait - converts closures into systems
pub trait IntoSystem<Params> {
    fn into_system(self) -> BoxedSystem;
}

use crate::system::System;

// Implementation for no parameters
impl<F> IntoSystem<()> for F
where
    F: FnMut() + Send + Sync + 'static,
{
    fn into_system(mut self) -> BoxedSystem {
        let closure = Box::new(move |_world: &mut World| {
            self();
        });

        BoxedSystem {
            closure,
            access: WorldAccessFlags::default(),
        }
    }
}

// ... RawClosure struct ... (keep it if needed, or remove)
// I'll keep the struct definitions but remove the impl block calling transmute on FnMut if I switch to System trait.
// Actually, I'll just append System impl.

impl System for BoxedSystem {
    fn run(&mut self, world: &mut World) {
        (self.closure)(world);
    }
}

#[repr(C)]
pub struct RawClosure {
    pub data: *mut std::ffi::c_void,
    pub vtable: *mut std::ffi::c_void,
}

pub type SystemTrampolineFn = unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void);

unsafe extern "C" fn system_trampoline(closure: *mut std::ffi::c_void, world_ptr: *mut std::ffi::c_void) {
    let closure = closure as *mut RawClosure;
    let ptr: *mut (dyn FnMut(&mut World) + Send + Sync) = std::mem::transmute(((*closure).data, (*closure).vtable));
    let world = &mut *(world_ptr as *mut World);
    (*ptr)(world);
}

impl BoxedSystem {
    pub fn into_raw_parts(self) -> (*mut std::ffi::c_void, *mut std::ffi::c_void, SystemTrampolineFn) {
        let ptr = Box::into_raw(self.closure);
        let (data, vtable): (*mut std::ffi::c_void, *mut std::ffi::c_void) = unsafe { std::mem::transmute(ptr) };
        (data, vtable, system_trampoline)
    }
}

// Implementation for single parameter
impl<F, P1> IntoSystem<(P1,)> for F
where
    F: FnMut(P1::Item<'_>) + Send + Sync + 'static,
    P1: SystemParam,
{
    fn into_system(mut self) -> BoxedSystem {
        let closure = Box::new(move |world: &mut World| {
            let p1 = P1::fetch(world);
            self(p1);
        });

        BoxedSystem {
            closure,
            access: P1::access_flags(),
        }
    }
}

// Macro to generate implementations for 2-16 parameters
macro_rules! impl_into_system {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<F, $($param),*> IntoSystem<($($param,)*)> for F
        where
            F: FnMut($($param::Item<'_>),*) + Send + Sync + 'static,
            $($param: SystemParam),*
        {
            fn into_system(mut self) -> BoxedSystem {
                let closure = Box::new(move |world: &mut World| {
                    $(let $param = $param::fetch(world);)*
                    self($($param),*);
                });

                BoxedSystem {
                    closure,
                    access: WorldAccessFlags::merge(&[
                        $($param::access_flags()),*
                    ]),
                }
            }
        }
    };
}

// Generate implementations for 2-16 parameters
impl_into_system!(P1, P2);
impl_into_system!(P1, P2, P3);
impl_into_system!(P1, P2, P3, P4);
impl_into_system!(P1, P2, P3, P4, P5);
impl_into_system!(P1, P2, P3, P4, P5, P6);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
impl_into_system!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16);
