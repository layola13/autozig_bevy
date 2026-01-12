//! ParamSet - Access multiple mutable system parameters

use std::marker::PhantomData;

/// Set of system parameters that cannot conflict
pub struct ParamSet<'w, 's, T> {
    _marker: PhantomData<(&'w (), &'s (), T)>,
}