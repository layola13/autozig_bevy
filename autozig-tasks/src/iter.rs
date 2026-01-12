//! Parallel iteration support

use std::marker::PhantomData;

#[cfg(not(target_arch = "wasm32"))]
pub trait ConditionalSend: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> ConditionalSend for T {}

#[cfg(target_arch = "wasm32")]
pub trait ConditionalSend {}
#[cfg(target_arch = "wasm32")]
impl<T> ConditionalSend for T {}

#[cfg(not(target_arch = "wasm32"))]
pub trait ConditionalSendFuture: std::future::Future + Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: std::future::Future + Send> ConditionalSendFuture for T {}

#[cfg(target_arch = "wasm32")]
pub trait ConditionalSendFuture: std::future::Future {}
#[cfg(target_arch = "wasm32")]
impl<T: std::future::Future> ConditionalSendFuture for T {}

#[cfg(not(target_arch = "wasm32"))]
pub trait MaybeSend: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> MaybeSend for T {}

#[cfg(target_arch = "wasm32")]
pub trait MaybeSend {}
#[cfg(target_arch = "wasm32")]
impl<T> MaybeSend for T {}

#[cfg(not(target_arch = "wasm32"))]
pub trait MaybeSync: Sync {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Sync> MaybeSync for T {}

#[cfg(target_arch = "wasm32")]
pub trait MaybeSync {}
#[cfg(target_arch = "wasm32")]
impl<T> MaybeSync for T {}

pub trait ParallelIterator: Sized {
    type Item: Send;
    fn for_each<F>(self, f: F) where F: Fn(Self::Item) + Send + Sync;
}

pub trait ParallelSlice<T: Sync> {
    fn par_iter(&self) -> SliceIter<T>;
}

pub trait ParallelSliceMut<T: Send> {
    fn par_iter_mut(&mut self) -> SliceIterMut<T>;
}

impl<T: Sync> ParallelSlice<T> for [T] {
    fn par_iter(&self) -> SliceIter<T> {
        SliceIter { slice: self }
    }
}

impl<T: Send> ParallelSliceMut<T> for [T] {
    fn par_iter_mut(&mut self) -> SliceIterMut<T> {
        SliceIterMut { slice: self }
    }
}

pub struct SliceIter<'a, T> {
    slice: &'a [T],
}

impl<'a, T: Sync> ParallelIterator for SliceIter<'a, T> {
    type Item = &'a T;
    fn for_each<F>(self, f: F) where F: Fn(Self::Item) + Send + Sync {
        self.slice.iter().for_each(f);
    }
}

pub struct SliceIterMut<'a, T> {
    slice: &'a mut [T],
}

impl<'a, T: Send> ParallelIterator for SliceIterMut<'a, T> {
    type Item = &'a mut T;
    fn for_each<F>(self, _f: F) where F: Fn(Self::Item) + Send + Sync {}
}

pub struct Map<I, F> { pub iter: I, pub func: F }
pub struct Filter<I, P> { pub iter: I, pub predicate: P }
pub struct FilterMap<I, F> { pub iter: I, pub func: F }
pub struct FlatMap<I, F> { pub iter: I, pub func: F }
pub struct Flatten<I> { pub iter: I }
pub struct Inspect<I, F> { pub iter: I, pub func: F }
pub struct Chain<A, B> { pub first: A, pub second: B }
pub struct Cloned<I> { pub iter: I }
pub struct Copied<I> { pub iter: I }
pub struct Cycle<I> { pub iter: I, pub _marker: PhantomData<I> }
pub struct Fuse<I> { pub iter: I }