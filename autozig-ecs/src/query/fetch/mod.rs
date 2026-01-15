//! Fetch module for query data retrieval
//! 查询数据获取模块
//!
//! Architecture: 90% Zig + 10% Rust
//! Core fetch logic implemented in Zig for performance

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
};
use std::marker::PhantomData;

/// Zig core integration
#[repr(C)]
pub struct FetchCoreOpaque {
    _private: [u8; 0],
}

use autozig_macro::include_zig;

use crate::change_detection::{Tick, ComponentTicks, Mut, Ref};

include_zig!("src/query/fetch/zig/fetch.zig", {
    fn fetch_create() -> *mut FetchCoreOpaque;
    fn fetch_destroy(fetch: *mut FetchCoreOpaque);
    fn fetch_configure(fetch: *mut FetchCoreOpaque, data: *const u8, ticks: *const ComponentTicks, size: usize, stride: usize);
    fn fetch_get_at(fetch: *mut FetchCoreOpaque, index: usize) -> *const u8;
    fn fetch_get_ticks_at(fetch: *mut FetchCoreOpaque, index: usize) -> *mut ComponentTicks;
    fn fetch_set_table(fetch: *mut FetchCoreOpaque, table: *mut crate::storage::table::TableOpaque, component_id: u32);
});

/// Entity fetch - fetches entity IDs
pub struct EntityFetch {
    inner: *mut FetchCoreOpaque,
}

impl EntityFetch {
    pub fn new() -> Self {
        Self {
            inner: fetch_create(),
        }
    }
}

impl Drop for EntityFetch {
    fn drop(&mut self) {
        fetch_destroy(self.inner);
    }
}

unsafe impl Send for EntityFetch {}
unsafe impl Sync for EntityFetch {}

impl Default for EntityFetch {
    fn default() -> Self {
        Self::new()
    }
}

/// Read fetch - fetches immutable component data
pub struct ReadFetch<T: Component> {
    pub(crate) inner: *mut FetchCoreOpaque,
    pub(crate) ptr: *const T,
    component_id: ComponentId,
    pub(crate) last_run: Tick,
    pub(crate) this_run: Tick,
    _phantom: PhantomData<T>,
}

impl<T: Component> ReadFetch<T> {
    pub fn new(component_id: ComponentId, last_run: Tick, this_run: Tick) -> Self {
        Self {
            inner: fetch_create(),
            ptr: std::ptr::null(),
            component_id,
            last_run,
            this_run,
            _phantom: PhantomData,
        }
    }
    
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl<T: Component> Drop for ReadFetch<T> {
    fn drop(&mut self) {
        fetch_destroy(self.inner);
    }
}

/// Write fetch - fetches mutable component data
pub struct WriteFetch<T: Component> {
    pub(crate) inner: *mut FetchCoreOpaque,
    pub(crate) ptr: *mut T,
    pub(crate) ticks_ptr: *mut ComponentTicks,
    component_id: ComponentId,
    pub(crate) last_run: Tick,
    pub(crate) this_run: Tick,
    _phantom: PhantomData<T>,
}

unsafe impl<T: Component> Send for ReadFetch<T> {}
unsafe impl<T: Component> Sync for ReadFetch<T> {}
unsafe impl<T: Component> Send for WriteFetch<T> {}
unsafe impl<T: Component> Sync for WriteFetch<T> {}

impl<T: Component> WriteFetch<T> {
    pub fn new(component_id: ComponentId, last_run: Tick, this_run: Tick) -> Self {
        Self {
            inner: fetch_create(),
            ptr: std::ptr::null_mut(),
            ticks_ptr: std::ptr::null_mut(),
            component_id,
            last_run,
            this_run,
            _phantom: PhantomData,
        }
    }
    
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl<T: Component> Drop for WriteFetch<T> {
    fn drop(&mut self) {
        fetch_destroy(self.inner);
    }
}

// ... RefFetch omitted for brevity, focusing on Read/Write first ...




pub struct RefFetch<T: Component> {
    pub(crate) inner: *mut FetchCoreOpaque,
    component_id: ComponentId,
    pub(crate) last_run: Tick,
    pub(crate) this_run: Tick,
    _phantom: PhantomData<T>,
}

unsafe impl<T: Component> Send for RefFetch<T> {}
unsafe impl<T: Component> Sync for RefFetch<T> {}

impl<T: Component> RefFetch<T> {
    pub fn new(component_id: ComponentId, last_run: Tick, this_run: Tick) -> Self {
        Self {
            inner: fetch_create(),
            component_id,
            last_run,
            this_run,
            _phantom: PhantomData,
        }
    }
    
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl<T: Component> Drop for RefFetch<T> {
    fn drop(&mut self) {
        fetch_destroy(self.inner);
    }
}

impl<'w, T: Component> Fetch<'w> for RefFetch<T> {
    type Item = Ref<'w, T>;
    type State = ComponentId;

    fn init(state: &Self::State, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self {
        Self::new(*state, last_run, this_run)
    }

    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
        fetch_set_table(self.inner, table.inner, state.index() as u32);
    }

    unsafe fn set_archetype(&mut self, state: &Self::State, _archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        self.set_table(state, table);
    }

    fn fetch(&mut self, _entity: Entity, index: usize) -> Self::Item {
        unsafe {
            let ptr = fetch_get_at(self.inner, index);
            let ticks_ptr = fetch_get_ticks_at(self.inner, index);
            
            if ptr.is_null() {
                panic!("RefFetch: component pointer is null at index {}", index);
            }
            if ticks_ptr.is_null() {
                panic!("RefFetch: ticks pointer is null at index {}", index);
            }
            
            Ref::new(
                &*(ptr as *const T),
                &*(ticks_ptr as *const ComponentTicks),
                self.last_run,
                self.this_run,
            )
        }
    }

    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        archetype.components().contains(state)
    }
}

/// Option fetch - fetches optional component data
pub struct OptionFetch<F> {
    pub(crate) inner: F,
    pub(crate) matches: bool,
}

impl<F> OptionFetch<F> {
    pub fn new(inner: F) -> Self {
        Self { inner, matches: false }
    }
}

/// Fetch state trait
pub trait FetchState: Send + Sync + 'static {
    fn init(world: &crate::world::World) -> Self;
}

impl FetchState for () {
    fn init(_world: &crate::world::World) -> Self {
        ()
    }
}

impl FetchState for ComponentId {
    fn init(_world: &crate::world::World) -> Self {
        ComponentId::new(0)
    }
}

/// Fetch trait for retrieving component data
pub trait Fetch<'w>: Sized {
    type Item;
    type State: FetchState;
    
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self;
    
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table);
    
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table);
    
    fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item;

    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool;
}

impl<'w> Fetch<'w> for EntityFetch {
    type Item = Entity;
    type State = ();

    fn init(_state: &Self::State, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _last_run: crate::change_detection::Tick, _this_run: crate::change_detection::Tick) -> Self {
        Self::new()
    }

    unsafe fn set_table(&mut self, _state: &Self::State, _table: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _state: &Self::State, _archetype: &crate::archetype::Archetype, _table: &crate::storage::Table) {}

    fn fetch(&mut self, entity: Entity, _index: usize) -> Self::Item {
        entity
    }

    fn matches_archetype(_state: &Self::State, _archetype: &crate::archetype::Archetype) -> bool {
        true
    }
}

impl<'w, T: Component> Fetch<'w> for ReadFetch<T> {
    type Item = &'w T;
    type State = ComponentId;

    fn init(state: &Self::State, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self {
        Self::new(*state, last_run, this_run)
    }

    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
        fetch_set_table(self.inner, table.inner, state.index() as u32);
    }

    unsafe fn set_archetype(&mut self, state: &Self::State, _archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        self.set_table(state, table);
    }

    fn fetch(&mut self, _entity: Entity, index: usize) -> Self::Item {
        unsafe {
            let ptr = fetch_get_at(self.inner, index);
            &*(ptr as *const T)
        }
    }

    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        archetype.components().contains(state)
    }
}

impl<'w, T: Component> Fetch<'w> for WriteFetch<T> {
    type Item = Mut<'w, T>;
    type State = ComponentId;

    fn init(state: &Self::State, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self {
        Self::new(*state, last_run, this_run)
    }

    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
        fetch_set_table(self.inner, table.inner, state.index() as u32);
    }

    unsafe fn set_archetype(&mut self, state: &Self::State, _archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        self.set_table(state, table);
    }

    fn fetch(&mut self, _entity: Entity, index: usize) -> Self::Item {
        unsafe {
            let ptr = fetch_get_at(self.inner, index);
            let ticks_ptr = fetch_get_ticks_at(self.inner, index);
            Mut::new(
                &mut *(ptr as *mut T),
                &mut *ticks_ptr,
                self.this_run, // current_tick
                self.last_run,
                self.this_run,
            )
        }
    }

    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        archetype.components().contains(state)
    }
}

impl<'w, F: Fetch<'w>> Fetch<'w> for OptionFetch<F> {
    type Item = Option<F::Item>;
    type State = F::State;
    
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self {
        Self::new(F::init(state, world, last_run, this_run))
    }

    unsafe fn set_table(&mut self, _state: &Self::State, _table: &crate::storage::Table) {
        // OptionFetch doesn't update matches in set_table because it needs Archetype.
        // It should be driven by set_archetype.
    }
    
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        self.matches = F::matches_archetype(state, archetype);
        if self.matches {
            self.inner.set_archetype(state, archetype, table);
        }
    }

    fn fetch(&mut self, entity: Entity, row: usize) -> Self::Item {
        if self.matches {
            Some(self.inner.fetch(entity, row))
        } else {
            None
        }
    }

    fn matches_archetype(_state: &Self::State, _archetype: &crate::archetype::Archetype) -> bool {
        true
    }
}

impl<'w> Fetch<'w> for () {
    type Item = ();
    type State = ();

    fn init(_state: &Self::State, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _last_run: crate::change_detection::Tick, _this_run: crate::change_detection::Tick) -> Self {
        ()
    }

    unsafe fn set_table(&mut self, _state: &Self::State, _table: &crate::storage::Table) {}
    
    unsafe fn set_archetype(&mut self, _state: &Self::State, _archetype: &crate::archetype::Archetype, _table: &crate::storage::Table) {}

    fn fetch(&mut self, _entity: Entity, _index: usize) -> Self::Item {
        ()
    }

    fn matches_archetype(_state: &Self::State, _archetype: &crate::archetype::Archetype) -> bool {
        true
    }
}

impl<'w, A: Fetch<'w>> Fetch<'w> for (A,) {
    type Item = (A::Item,);
    type State = (A::State,);

    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self {
        let (a,) = state;
        (A::init(a, world, last_run, this_run),)
    }

    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
        let (a_f,) = self;
        let (a_s,) = state;
        a_f.set_table(a_s, table);
    }
    
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        let (a_f,) = self;
        let (a_s,) = state;
        a_f.set_archetype(a_s, archetype, table);
    }

    fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item {
        let (a,) = self;
        (a.fetch(entity, index),)
    }

    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        A::matches_archetype(&state.0, archetype)
    }
}

impl<A: FetchState> FetchState for (A,) {
    fn init(world: &crate::world::World) -> Self {
        (A::init(world),)
    }
}

impl<'w, A: Fetch<'w>, B: Fetch<'w>> Fetch<'w> for (A, B) {
    type Item = (A::Item, B::Item);
    type State = (A::State, B::State);

    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self {
        let (a, b) = state;
        (A::init(a, world, last_run, this_run), B::init(b, world, last_run, this_run))
    }

    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
        let (a_f, b_f) = self;
        let (a_s, b_s) = state;
        a_f.set_table(a_s, table);
        b_f.set_table(b_s, table);
    }
    
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        let (a_f, b_f) = self;
        let (a_s, b_s) = state;
        a_f.set_archetype(a_s, archetype, table);
        b_f.set_archetype(b_s, archetype, table);
    }

    fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item {
        let (a, b) = self;
        (a.fetch(entity, index), b.fetch(entity, index))
    }

    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        A::matches_archetype(&state.0, archetype) && B::matches_archetype(&state.1, archetype)
    }
}

impl<A: FetchState, B: FetchState> FetchState for (A, B) {
    fn init(world: &crate::world::World) -> Self {
        (A::init(world), B::init(world))
    }
}

macro_rules! impl_fetch_tuple {
    ($(($name:ident, $state:ident)),*) => {
        impl<'w, $($name: Fetch<'w>),*> Fetch<'w> for ($($name,)*) {
            type Item = ($($name::Item,)*);
            type State = ($($name::State,)*);

            fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self {
                #[allow(non_snake_case)]
                let ($($state,)*) = state;
                ($($name::init($state, world, last_run, this_run),)*)
            }

            unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                #[allow(non_snake_case)]
                let ($($state,)*) = state;
                $($name.set_table($state, table);)*
            }
            
            unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                #[allow(non_snake_case)]
                let ($($state,)*) = state;
                $($name.set_archetype($state, archetype, table);)*
            }

            fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                ($($name.fetch(entity, index),)*)
            }

            fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
                #[allow(non_snake_case)]
                let ($($state,)*) = state;
                true $(&& $name::matches_archetype($state, archetype))*
            }
        }

        impl<$($state: FetchState),*> FetchState for ($($state,)*) {
            fn init(world: &crate::world::World) -> Self {
                ($($state::init(world),)*)
            }
        }
    };
}

impl_fetch_tuple!((A, SA), (B, SB), (C, SC));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE), (F, SF));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE), (F, SF), (G, SG));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE), (F, SF), (G, SG), (H, SH));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE), (F, SF), (G, SG), (H, SH), (I, SI));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE), (F, SF), (G, SG), (H, SH), (I, SI), (J, SJ));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE), (F, SF), (G, SG), (H, SH), (I, SI), (J, SJ), (K, SK));
impl_fetch_tuple!((A, SA), (B, SB), (C, SC), (D, SD), (E, SE), (F, SF), (G, SG), (H, SH), (I, SI), (J, SJ), (K, SK), (L, SL));