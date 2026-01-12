//! Exclusive systems - Systems with exclusive World access

use crate::world::World;

/// Exclusive system that has full World access
pub trait ExclusiveSystem: Send + Sync + 'static {
    fn run(&mut self, world: &mut World);
}

/// Exclusive function system wrapper
pub struct ExclusiveFunctionSystem<F> {
    func: F,
}

impl<F> ExclusiveFunctionSystem<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> ExclusiveSystem for ExclusiveFunctionSystem<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
}