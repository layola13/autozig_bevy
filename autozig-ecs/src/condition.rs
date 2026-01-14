use crate::world::World;

/// Condition trait - runs logic and returns a boolean
pub trait Condition: Send + Sync + 'static {
    fn run(&mut self, world: &mut World) -> bool;
}

/// IntoCondition trait - converts a value into a Condition
pub trait IntoCondition<M>: Sized {
    type Condition: Condition;
    fn into_condition(self) -> Self::Condition;
}

// Re-export combinators
pub use crate::combinator::{And, Or, Not};

pub type BoxedCondition = Box<dyn Condition>;

impl Condition for BoxedCondition {
    fn run(&mut self, world: &mut World) -> bool {
        (**self).run(world)
    }
}

use crate::system::{BoxedSystem, System};

/// ConditionalSystem - System that runs only if conditions are met
pub struct ConditionalSystem {
    system: BoxedSystem,
    conditions: Vec<BoxedCondition>,
}

impl ConditionalSystem {
    pub fn new(system: BoxedSystem, conditions: Vec<BoxedCondition>) -> Self {
        Self { system, conditions }
    }
}

impl System for ConditionalSystem {
    type In = ();
    type Out = ();

    fn initialize(&mut self, world: &mut World) {
        self.system.initialize(world);
    }

    fn run(&mut self, _input: (), world: &mut World) {
        for condition in &mut self.conditions {
            if !condition.run(world) {
                return;
            }
        }
        self.system.run(world);
    }
    
    fn name(&self) -> &str {
        self.system.name()
    }
}

pub fn not<M>(condition: impl IntoCondition<M>) -> impl Condition {
    Not { system: condition.into_condition() }
}

pub trait ConditionMethods<M>: IntoCondition<M> {
    fn and<M2, Other>(self, other: Other) -> And<Self::Condition, Other::Condition>
    where
        Other: IntoCondition<M2>
    {
        And {
            a: self.into_condition(),
            b: other.into_condition(),
        }
    }
    
    fn or<M2, Other>(self, other: Other) -> crate::combinator::OrCond<Self::Condition, Other::Condition>
    where
        Other: IntoCondition<M2>
    {
        crate::combinator::OrCond {
            a: self.into_condition(),
            b: other.into_condition(),
        }
    }
}

impl<T, M> ConditionMethods<M> for T where T: IntoCondition<M> {}