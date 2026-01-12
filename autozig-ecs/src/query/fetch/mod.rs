//! Fetch system - 泛型数据获取系统
//! 
//! 这个模块定义了Fetch trait及其各种实现（ReadFetch, WriteFetch等）。

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
};
use std::marker::PhantomData;

/// Zig核心实现 - Fetch的底层操作
#[repr(C)]
pub struct FetchCoreOpaque {
    _private: [u8; 0],
}

use autozig_macro::include_zig;

include_zig!("src/zig/fetch.zig", {
    fn fetch_create() -> *mut FetchCoreOpaque;
    fn fetch_destroy(fetch_ptr: *mut FetchCoreOpaque);
    fn fetch_configure(fetch_ptr: *mut FetchCoreOpaque, data_ptr: *const u8, size: usize, stride: usize);
    fn fetch_get_at(fetch_ptr: *const FetchCoreOpaque, index: usize) -> *mut u8;
});

/// Fetch state initialization trait
pub trait FetchState: Send + Sync + 'static {
    fn init(world: &crate::world::World) -> Self;
}

impl FetchState for () {
    fn init(_world: &crate::world::World) -> Self { () }
}

impl FetchState for ComponentId {
    fn init(_world: &crate::world::World) -> Self { ComponentId::new(0) }
}

impl<A: FetchState> FetchState for (A,) {
    fn init(world: &crate::world::World) -> Self { (A::init(world),) }
}

impl<A: FetchState, B: FetchState> FetchState for (A, B) {
    fn init(world: &crate::world::World) -> Self { (A::init(world), B::init(world)) }
}

/// Fetch trait for retrieving component data
pub trait Fetch<'w>: Send + Sync + Sized {
    type Item;
    type State: FetchState;
    
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self;
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table);
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table);
    fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item;
}

/// Read fetch - fetches immutable component data
pub struct ReadFetch<T: Component> {
    inner: *mut FetchCoreOpaque,
    _phantom: PhantomData<T>,
}

unsafe impl<T: Component> Send for ReadFetch<T> {}
unsafe impl<T: Component> Sync for ReadFetch<T> {}

impl<T: Component> ReadFetch<T> {
    pub fn new() -> Self {
        Self {
            inner: unsafe { fetch_create() },
            _phantom: PhantomData,
        }
    }

    pub unsafe fn configure(&mut self, data: *const u8, size: usize, stride: usize) {
        fetch_configure(self.inner, data, size, stride);
    }

    pub fn init<'w>(state: &ComponentId, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _last_run: crate::change_detection::Tick, _this_run: crate::change_detection::Tick) -> Self {
        Self::new()
    }

    pub unsafe fn set_table(&mut self, state: &ComponentId, table: &crate::storage::Table) {
        if let Some(column) = table.get_column(*state) {
            self.configure(
                column.get_ptr_unchecked(0),
                std::mem::size_of::<T>(),
                std::mem::size_of::<T>(),
            );
        }
    }

    pub unsafe fn set_archetype(&mut self, state: &ComponentId, _archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        self.set_table(state, table);
    }
}

impl<T: Component> Drop for ReadFetch<T> {
    fn drop(&mut self) { unsafe { fetch_destroy(self.inner) }; }
}

impl<'w, T: Component> Fetch<'w> for ReadFetch<T> {
    type Item = &'w T;
    type State = ComponentId;
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self { Self::init(state, world, last_run, this_run) }
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) { self.set_table(state, table); }
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { self.set_archetype(state, archetype, table); }
    fn fetch(&mut self, _entity: Entity, index: usize) -> Self::Item { unsafe { &*(fetch_get_at(self.inner, index) as *const T) } }
}

/// Write fetch - fetches mutable component data
pub struct WriteFetch<T: Component> {
    inner: *mut FetchCoreOpaque,
    _phantom: PhantomData<T>,
}

unsafe impl<T: Component> Send for WriteFetch<T> {}
unsafe impl<T: Component> Sync for WriteFetch<T> {}

impl<T: Component> WriteFetch<T> {
    pub fn new() -> Self {
        Self {
            inner: unsafe { fetch_create() },
            _phantom: PhantomData,
        }
    }

    pub unsafe fn configure(&mut self, data: *mut u8, size: usize, stride: usize) {
        fetch_configure(self.inner, data, size, stride);
    }

    pub fn init<'w>(state: &ComponentId, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _last_run: crate::change_detection::Tick, _this_run: crate::change_detection::Tick) -> Self {
        Self::new()
    }

    pub unsafe fn set_table(&mut self, state: &ComponentId, table: &crate::storage::Table) {
        if let Some(column) = table.get_column(*state) {
            fetch_configure(self.inner, column.get_ptr_unchecked(0), std::mem::size_of::<T>(), std::mem::size_of::<T>());
        }
    }
}

impl<T: Component> Drop for WriteFetch<T> {
    fn drop(&mut self) { unsafe { fetch_destroy(self.inner) }; }
}

impl<'w, T: Component> Fetch<'w> for WriteFetch<T> {
    type Item = &'w mut T;
    type State = ComponentId;
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self { Self::new() }
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) { self.set_table(state, table); }
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { self.set_table(state, table); }
    fn fetch(&mut self, _entity: Entity, index: usize) -> Self::Item { unsafe { &mut *(fetch_get_at(self.inner, index) as *mut T) } }
}

pub struct EntityFetch;
unsafe impl Send for EntityFetch {}
unsafe impl Sync for EntityFetch {}
impl<'w> Fetch<'w> for EntityFetch {
    type Item = Entity;
    type State = ();
    fn init(_: &(), _: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _: crate::change_detection::Tick, _: crate::change_detection::Tick) -> Self { EntityFetch }
    unsafe fn set_table(&mut self, _: &(), _: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _: &(), _: &crate::archetype::Archetype, _: &crate::storage::Table) {}
    fn fetch(&mut self, entity: Entity, _: usize) -> Self::Item { entity }
}

pub struct OptionFetch<F> { pub inner: F }
impl<F> OptionFetch<F> { pub fn new(inner: F) -> Self { Self { inner } } }
unsafe impl<F: Send> Send for OptionFetch<F> {}
unsafe impl<F: Sync> Sync for OptionFetch<F> {}
impl<'w, F: Fetch<'w>> Fetch<'w> for OptionFetch<F> {
    type Item = Option<F::Item>;
    type State = F::State;
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self { Self::new(F::init(state, world, last_run, this_run)) }
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) { self.inner.set_table(state, table); }
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { self.inner.set_archetype(state, archetype, table); }
    fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item { Some(self.inner.fetch(entity, index)) }
}

impl<'w> Fetch<'w> for () {
    type Item = ();
    type State = ();
    fn init(_: &(), _: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _: crate::change_detection::Tick, _: crate::change_detection::Tick) -> Self { () }
    unsafe fn set_table(&mut self, _: &(), _: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _: &(), _: &crate::archetype::Archetype, _: &crate::storage::Table) {}
    fn fetch(&mut self, _: Entity, _: usize) -> Self::Item { () }
}

// Manual tuple implementations for Fetch
impl<'w, A: Fetch<'w>> Fetch<'w> for (A,) {
    type Item = (A::Item,);
    type State = (A::State,);
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self { (A::init(&state.0, world, last_run, this_run),) }
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) { self.0.set_table(&state.0, table); }
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { self.0.set_archetype(&state.0, archetype, table); }
    fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item { (self.0.fetch(entity, index),) }
}

impl<'w, A: Fetch<'w>, B: Fetch<'w>> Fetch<'w> for (A, B) {
    type Item = (A::Item, B::Item);
    type State = (A::State, B::State);
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self { (A::init(&state.0, world, last_run, this_run), B::init(&state.1, world, last_run, this_run)) }
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) { self.0.set_table(&state.0, table); self.1.set_table(&state.1, table); }
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { self.0.set_archetype(&state.0, archetype, table); self.1.set_archetype(&state.1, archetype, table); }
    fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item { (self.0.fetch(entity, index), self.1.fetch(entity, index)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::World;
    #[derive(Debug, Clone, Copy, PartialEq)] pub struct Position { pub x: f32, pub y: f32 }
    impl Component for Position {}
    #[test]
    fn test_read_fetch() {
        let mut data = vec![Position { x: 10.0, y: 20.0 }, Position { x: 30.0, y: 40.0 }];
        let component_id = ComponentId::new(1);
        let mut fetch = ReadFetch::<Position>::new();
        unsafe { fetch.configure(data.as_ptr() as *const u8, std::mem::size_of::<Position>(), std::mem::size_of::<Position>()); }
        let p0 = fetch.fetch(Entity::from_raw(0), 0);
        assert_eq!(p0.x, 10.0);
    }
}