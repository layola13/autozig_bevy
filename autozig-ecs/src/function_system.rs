//! Function systems - Regular functions as systems

use crate::world::World;
use crate::system::System;

/// Function system wrapper
pub struct FunctionSystem<F, Marker> {
    func: F,
    _marker: std::marker::PhantomData<Marker>,
}

impl<F, Marker> FunctionSystem<F, Marker> {
    pub fn new(func: F) -> Self {
        Self {
            func,
            _marker: std::marker::PhantomData,
        }
    }
}