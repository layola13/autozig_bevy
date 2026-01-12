//! System adapters - Adapt systems for different contexts

use crate::system::System;

/// System adapter
pub struct SystemAdapter<S> {
    system: S,
}

/// Combinator system
pub struct CombinatorSystem<A, B> {
    first: A,
    second: B,
}