//! Conditions - Runtime conditions for system execution

use crate::world::World;

/// Trait for system run conditions
pub trait Condition: Send + Sync + 'static {
    fn check(&mut self, world: &World) -> bool;
}

/// Trait for converting into conditions
pub trait IntoCondition<Marker> {
    type Condition: Condition;
    fn into_condition(self) -> Self::Condition;
}

/// Run criteria for system scheduling  
pub trait RunCriteria: Send + Sync + 'static {
    fn should_run(&mut self, world: &World) -> ShouldRun;
}

/// Whether a system should run
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShouldRun {
    Yes,
    No,
    YesAndCheckAgain,
    NoAndCheckAgain,
}

/// Common condition implementations
pub mod common_conditions {
    use super::*;
    
    pub fn run_once() -> impl Condition {
        RunOnceCondition { has_run: false }
    }
    
    struct RunOnceCondition {
        has_run: bool,
    }
    
    impl Condition for RunOnceCondition {
        fn check(&mut self, _world: &World) -> bool {
            if self.has_run {
                false
            } else {
                self.has_run = true;
                true
            }
        }
    }
}