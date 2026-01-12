//! SystemParam - System parameter trait and implementations for dependency injection

use crate::world::World;
use crate::resource::{Res, ResMut};
use crate::command::Commands;
use std::marker::PhantomData;

/// World access flags for system scheduling
#[derive(Debug, Clone, Copy, Default)]
pub struct WorldAccessFlags {
    pub reads_resources: bool,
    pub writes_resources: bool,
    pub reads_components: bool,
    pub writes_components: bool,
}

impl WorldAccessFlags {
    /// Merge multiple access flags
    pub fn merge(flags: &[WorldAccessFlags]) -> Self {
        let mut result = WorldAccessFlags::default();
        for flag in flags {
            result.reads_resources |= flag.reads_resources;
            result.writes_resources |= flag.writes_resources;
            result.reads_components |= flag.reads_components;
            result.writes_components |= flag.writes_components;
        }
        result
    }

    /// Convert to u8 for FFI
    pub fn to_u8(&self) -> u8 {
        let mut flags = 0u8;
        if self.reads_resources {
            flags |= 0b0001;
        }
        if self.writes_resources {
            flags |= 0b0010;
        }
        if self.reads_components {
            flags |= 0b0100;
        }
        if self.writes_components {
            flags |= 0b1000;
        }
        flags
    }
}

/// SystemParam - trait for types that can be used as system parameters
pub trait SystemParam: Sized {
    /// The item type that will be passed to the system function
    type Item<'w>;

    /// Fetch the parameter from the world
    fn fetch<'w>(world: &'w World) -> Self::Item<'w>;

    /// Get the access flags for this parameter
    fn access_flags() -> WorldAccessFlags;
}

/// Marker trait for SystemParam types that only read data (no mutable access)
/// This is used for system scheduling to detect data dependencies
pub trait ReadOnlySystemParam: SystemParam {}

/// Marker trait for SystemParam types with 'static lifetime (no lifetime dependencies)
/// This allows certain optimizations in system scheduling
pub trait StaticSystemParam: SystemParam {}

// Note: Res and ResMut SystemParam implementations will be added
// once we extend World with resource access methods

/// Marker for unit type (no parameters)
impl SystemParam for () {
    type Item<'w> = ();

    fn fetch<'w>(_world: &'w World) -> Self::Item<'w> {
        ()
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags::default()
    }
}

// Implement ReadOnlySystemParam and StaticSystemParam for unit type
impl ReadOnlySystemParam for () {}
impl StaticSystemParam for () {}

// Tuple implementations for multiple parameters
macro_rules! impl_system_param_tuple {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($param: SystemParam),*> SystemParam for ($($param,)*) {
            type Item<'w> = ($($param::Item<'w>,)*);

            fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
                ($($param::fetch(world),)*)
            }

            fn access_flags() -> WorldAccessFlags {
                WorldAccessFlags::merge(&[
                    $($param::access_flags(),)*
                ])
            }
        }
    };
}

// Implement for tuples up to 16 elements
impl_system_param_tuple!(P1);
impl_system_param_tuple!(P1, P2);
impl_system_param_tuple!(P1, P2, P3);
impl_system_param_tuple!(P1, P2, P3, P4);
impl_system_param_tuple!(P1, P2, P3, P4, P5);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16);
