//! Local - System-local state

use std::ops::{Deref, DerefMut};

/// System-local state
pub struct Local<'s, T> {
    value: &'s mut T,
}

impl<'s, T> Deref for Local<'s, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
    }
}

impl<'s, T> DerefMut for Local<'s, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}