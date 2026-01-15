use std::marker::PhantomData;

use crate::system_param::SystemParam;

/// Set of system parameters that cannot conflict
pub struct ParamSet<'w, 's, T: SystemParam> {
    param: T::Item<'w>,
    _marker: PhantomData<&'s ()>,
}

impl<'w, 's, T: SystemParam> ParamSet<'w, 's, T> {
   pub fn new(param: T::Item<'w>) -> Self {
       Self {
           param,
           _marker: PhantomData,
       }
   }
}

// Manual implementation for common tuple sizes without using paste! macro

impl<'w, 's, T0: SystemParam> ParamSet<'w, 's, (T0,)> {
    pub fn p0(&mut self) -> &mut T0::Item<'w> {
        &mut self.param.0
    }
}

impl<'w, 's, T0: SystemParam, T1: SystemParam> ParamSet<'w, 's, (T0, T1)> {
    pub fn p0(&mut self) -> &mut T0::Item<'w> {
        &mut self.param.0
    }
    pub fn p1(&mut self) -> &mut T1::Item<'w> {
        &mut self.param.1
    }
}

impl<'w, 's, T0: SystemParam, T1: SystemParam, T2: SystemParam> ParamSet<'w, 's, (T0, T1, T2)> {
    pub fn p0(&mut self) -> &mut T0::Item<'w> { &mut self.param.0 }
    pub fn p1(&mut self) -> &mut T1::Item<'w> { &mut self.param.1 }
    pub fn p2(&mut self) -> &mut T2::Item<'w> { &mut self.param.2 }
}

impl<'w, 's, T0: SystemParam, T1: SystemParam, T2: SystemParam, T3: SystemParam> ParamSet<'w, 's, (T0, T1, T2, T3)> {
    pub fn p0(&mut self) -> &mut T0::Item<'w> { &mut self.param.0 }
    pub fn p1(&mut self) -> &mut T1::Item<'w> { &mut self.param.1 }
    pub fn p2(&mut self) -> &mut T2::Item<'w> { &mut self.param.2 }
    pub fn p3(&mut self) -> &mut T3::Item<'w> { &mut self.param.3 }
}