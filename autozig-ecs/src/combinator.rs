//! System combinators - Compose systems together

use crate::world::World;

use crate::condition::Condition;

/// NOT condition combinator
pub struct Not<S> {
    pub system: S,
}

impl<S: Condition> Condition for Not<S> {
    fn run(&mut self, world: &mut World) -> bool {
        !self.system.run(world)
    }
}

/// AND condition combinator
pub struct And<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Condition, B: Condition> Condition for And<A, B> {
    fn run(&mut self, world: &mut World) -> bool {
        self.a.run(world) && self.b.run(world)
    }
}

/// OR condition combinator (renamed to avoid collision with query::filter::Or)
pub struct OrCond<A, B> {
    pub a: A,
    pub b: B,
}

impl<A: Condition, B: Condition> Condition for OrCond<A, B> {
    fn run(&mut self, world: &mut World) -> bool {
        self.a.run(world) || self.b.run(world)
    }
}

// Aliases for legacy
pub type Or<A, B> = OrCond<A, B>;