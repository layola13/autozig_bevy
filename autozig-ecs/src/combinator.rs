//! System combinators - Compose systems together

use crate::world::World;
use crate::system::System;

/// NOT combinator
pub struct NotSystem<S> {
    system: S,
}

/// AND THEN combinator
pub struct AndThenSystem<A, B> {
    first: A,
    second: B,
}

/// OR ELSE combinator
pub struct OrElseSystem<A, B> {
    first: A,
    second: B,
}

/// CHAIN combinator - pipe output to input
pub struct ChainSystem<A, B> {
    first: A,
    second: B,
}

/// PIPE combinator - similar to chain
pub struct PipeSystem<A, B> {
    first: A,
    second: B,
}