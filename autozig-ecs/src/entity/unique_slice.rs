//! UniqueEntitySlice - Slice that guarantees entity uniqueness

use super::Entity;
use crate::entity::unique_array::{DuplicateEntityError, EntityEquivalent, UniqueEntityEquivalentArray};
use crate::entity::unique_vec::{self, UniqueEntityEquivalentVec};
use std::fmt;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive, Index, IndexMut, Bound};
use std::slice::{self, SliceIndex};
use std::iter::{FusedIterator, DoubleEndedIterator};
use std::cmp::Ordering;
use std::borrow::{Borrow, ToOwned};
use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;

/// UniqueEntitySlice - Dynamically-sized slice of unique entities
#[repr(transparent)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniqueEntityEquivalentSlice<T: EntityEquivalent> {
    values: [T],
}

/// A slice that contains only unique [`Entity`].
pub type UniqueEntitySlice = UniqueEntityEquivalentSlice<Entity>;

impl<T: EntityEquivalent> UniqueEntityEquivalentSlice<T> {
    /// Constructs a `UniqueEntityEquivalentSlice` from a [`&[T]`] unsafely.
    ///
    /// # Safety
    ///
    /// `slice` must contain only unique elements.
    pub const unsafe fn from_slice_unchecked(slice: &[T]) -> &Self {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { &*(slice as *const [T] as *const Self) }
    }

    /// Constructs a `UniqueEntityEquivalentSlice` from a [`&mut [T]`] unsafely.
    ///
    /// # Safety
    ///
    /// `slice` must contain only unique elements.
    pub const unsafe fn from_slice_unchecked_mut(slice: &mut [T]) -> &mut Self {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { &mut *(slice as *mut [T] as *mut Self) }
    }

    /// Casts to `self` to a standard slice.
    pub const fn as_inner(&self) -> &[T] {
        &self.values
    }

    /// Casts to `self` to a mutable standard slice.
    pub fn as_mut_inner(&mut self) -> &mut [T] {
        &mut self.values
    }

    /// Constructs a `UniqueEntityEquivalentSlice` from a [`Box<[T]>`] unsafely.
    ///
    /// # Safety
    ///
    /// `slice` must contain only unique elements.
    pub unsafe fn from_boxed_slice_unchecked(slice: Box<[T]>) -> Box<Self> {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { Box::from_raw(Box::into_raw(slice) as *mut Self) }
    }

    /// Casts `self` to the inner slice.
    pub fn into_boxed_inner(self: Box<Self>) -> Box<[T]> {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [T]) }
    }

    /// Constructs a `UniqueEntityEquivalentSlice` from a [`Arc<[T]>`] unsafely.
    ///
    /// # Safety
    ///
    /// `slice` must contain only unique elements.
    pub unsafe fn from_arc_slice_unchecked(slice: Arc<[T]>) -> Arc<Self> {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { Arc::from_raw(Arc::into_raw(slice) as *const Self) }
    }

    /// Casts `self` to the inner slice.
    pub fn into_arc_inner(this: Arc<Self>) -> Arc<[T]> {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { Arc::from_raw(Arc::into_raw(this) as *const [T]) }
    }

    /// Constructs a `UniqueEntityEquivalentSlice` from a [`Rc<[T]>`] unsafely.
    ///
    /// # Safety
    ///
    /// `slice` must contain only unique elements.
    pub unsafe fn from_rc_slice_unchecked(slice: Rc<[T]>) -> Rc<Self> {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { Rc::from_raw(Rc::into_raw(slice) as *const Self) }
    }

    /// Casts `self` to the inner slice.
    pub fn into_rc_inner(self: Rc<Self>) -> Rc<[T]> {
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        unsafe { Rc::from_raw(Rc::into_raw(self) as *const [T]) }
    }

    /// Returns a reference to the underlying slice
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Returns a mutable reference to the underlying slice
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.values
    }

    /// Returns the number of values
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if the slice is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the first value
    #[inline]
    pub fn first(&self) -> Option<&T> {
        self.values.first()
    }

    /// Returns the last value
    #[inline]
    pub fn last(&self) -> Option<&T> {
        self.values.last()
    }

    /// Returns a reference to a value at the given index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    /// Returns a mutable reference to a value at the given index
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.values.get_mut(index)
    }

    /// Returns a reference to a subslice, without doing bounds checking.
    ///
    /// # Safety
    ///
    /// `index` must be safe to use with [`[T]::get_unchecked`]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &Self
    where
        I: SliceIndex<[T], Output = [T]>,
    {
        // SAFETY: All elements in the original slice are unique
        unsafe { Self::from_slice_unchecked(self.values.get_unchecked(index)) }
    }

    /// Returns a mutable reference to a subslice, without doing bounds checking.
    ///
    /// # Safety
    ///
    /// `index` must be safe to use with [`[T]::get_unchecked_mut`]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut Self
    where
        I: SliceIndex<[T], Output = [T]>,
    {
        // SAFETY: All elements in the original slice are unique
        unsafe { Self::from_slice_unchecked_mut(self.values.get_unchecked_mut(index)) }
    }

    /// Returns an unsafe mutable pointer to the slice's buffer.
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.values.as_mut_ptr()
    }

    /// Returns the two unsafe mutable pointers spanning the slice.
    pub const fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        self.values.as_mut_ptr_range()
    }

    /// Returns true if the slice contains the entity
    pub fn contains(&self, entity: Entity) -> bool
    where
        T: PartialEq<Entity>,
    {
        self.values.iter().any(|v| v == &entity)
    }

    /// Returns an iterator over the values
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.values.iter(),
        }
    }

    /// Returns a mutable iterator over the values
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self.values.iter_mut(),
        }
    }

    /// Returns an iterator over windows of N values
    pub fn windows(&self, size: usize) -> Windows<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.windows(size)) }
    }

    /// Returns an iterator over chunks of N values
    pub fn chunks(&self, chunk_size: usize) -> Chunks<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.chunks(chunk_size)) }
    }

    /// Returns an iterator over mutable chunks of N values
    pub fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.chunks_mut(chunk_size)) }
    }

    /// Returns an iterator over chunks of exactly N values
    pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.chunks_exact(chunk_size)) }
    }

    /// Returns an iterator over mutable chunks of exactly N values
    pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.chunks_exact_mut(chunk_size)) }
    }

    /// Returns an iterator over chunks from the end
    pub fn rchunks(&self, chunk_size: usize) -> RChunks<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.rchunks(chunk_size)) }
    }

    /// Returns an iterator over mutable chunks from the end
    pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.rchunks_mut(chunk_size)) }
    }

    /// Returns an iterator over exact chunks from the end
    pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.rchunks_exact(chunk_size)) }
    }

    /// Returns an iterator over exact mutable chunks from the end
    pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, T> {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.rchunks_exact_mut(chunk_size)) }
    }

    /// Returns an iterator over chunks separated by a predicate
    pub fn chunk_by<F>(&self, pred: F) -> ChunkBy<'_, F, T>
    where
        F: FnMut(&T, &T) -> bool,
    {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.chunk_by(pred)) }
    }

    /// Returns an iterator over mutable chunks separated by a predicate
    pub fn chunk_by_mut<F>(&mut self, pred: F) -> ChunkByMut<'_, F, T>
    where
        F: FnMut(&T, &T) -> bool,
    {
        // SAFETY: Any subslice of a unique slice is also unique
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.chunk_by_mut(pred)) }
    }

    /// Divides one slice into two at an index
    pub const fn split_at(&self, mid: usize) -> (&Self, &Self) {
        let (left, right) = self.values.split_at(mid);
        unsafe { (Self::from_slice_unchecked(left), Self::from_slice_unchecked(right)) }
    }

    /// Divides one mutable slice into two at an index
    pub const fn split_at_mut(&mut self, mid: usize) -> (&mut Self, &mut Self) {
        let (left, right) = self.values.split_at_mut(mid);
        unsafe { (Self::from_slice_unchecked_mut(left), Self::from_slice_unchecked_mut(right)) }
    }

    /// Divides one slice into two at an index, returning `None` if the slice is too short
    pub const fn split_at_checked(&self, mid: usize) -> Option<(&Self, &Self)> {
        let Some((left, right)) = self.values.split_at_checked(mid) else {
            return None;
        };
        unsafe { Some((Self::from_slice_unchecked(left), Self::from_slice_unchecked(right))) }
    }

    /// Divides one mutable slice into two at an index, returning `None` if the slice is too short
    pub const fn split_at_mut_checked(&mut self, mid: usize) -> Option<(&mut Self, &mut Self)> {
        let Some((left, right)) = self.values.split_at_mut_checked(mid) else {
            return None;
        };
        unsafe { Some((Self::from_slice_unchecked_mut(left), Self::from_slice_unchecked_mut(right))) }
    }

    /// Returns the first value and the rest of the slice
    pub const fn split_first(&self) -> Option<(&T, &Self)> {
        let Some((first, rest)) = self.values.split_first() else {
            return None;
        };
        Some((first, unsafe { Self::from_slice_unchecked(rest) }))
    }

    /// Returns the last value and the rest of the slice
    pub const fn split_last(&self) -> Option<(&T, &Self)> {
        let Some((last, rest)) = self.values.split_last() else {
            return None;
        };
        Some((last, unsafe { Self::from_slice_unchecked(rest) }))
    }

    /// Returns an array reference to the first `N` items in the slice
    pub const fn first_chunk<const N: usize>(&self) -> Option<&UniqueEntityEquivalentArray<T, N>>
    where
        T: Clone,
    {
        let Some(chunk) = self.values.first_chunk() else {
            return None;
        };
        Some(unsafe { UniqueEntityEquivalentArray::from_array_ref_unchecked(chunk) })
    }

    /// Returns an array reference to the last `N` items in the slice
    pub const fn last_chunk<const N: usize>(&self) -> Option<&UniqueEntityEquivalentArray<T, N>>
    where
        T: Clone,
    {
        let Some(chunk) = self.values.last_chunk() else {
            return None;
        };
        Some(unsafe { UniqueEntityEquivalentArray::from_array_ref_unchecked(chunk) })
    }

    /// Returns an array reference to the first `N` items and the remaining slice
    pub const fn split_first_chunk<const N: usize>(&self) -> Option<(&UniqueEntityEquivalentArray<T, N>, &Self)>
    where
        T: Clone,
    {
        let Some((chunk, rest)) = self.values.split_first_chunk() else {
            return None;
        };
        unsafe {
            Some((
                UniqueEntityEquivalentArray::from_array_ref_unchecked(chunk),
                Self::from_slice_unchecked(rest),
            ))
        }
    }

    /// Returns an array reference to the last `N` items and the remaining slice
    pub const fn split_last_chunk<const N: usize>(&self) -> Option<(&Self, &UniqueEntityEquivalentArray<T, N>)>
    where
        T: Clone,
    {
        let Some((rest, chunk)) = self.values.split_last_chunk() else {
            return None;
        };
        unsafe {
            Some((
                Self::from_slice_unchecked(rest),
                UniqueEntityEquivalentArray::from_array_ref_unchecked(chunk),
            ))
        }
    }

    /// Returns an iterator over subslices separated by elements that match pred
    pub fn split<F>(&self, pred: F) -> Split<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.split(pred)) }
    }

    /// Returns an iterator over mutable subslices separated by elements that match pred
    pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.split_mut(pred)) }
    }

    /// Returns an iterator over subslices separated by elements that match pred (inclusive)
    pub fn split_inclusive<F>(&self, pred: F) -> SplitInclusive<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.split_inclusive(pred)) }
    }

    /// Returns an iterator over mutable subslices separated by elements that match pred (inclusive)
    pub fn split_inclusive_mut<F>(&mut self, pred: F) -> SplitInclusiveMut<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.split_inclusive_mut(pred)) }
    }

    /// Returns an iterator over subslices separated by elements that match pred, from the end
    pub fn rsplit<F>(&self, pred: F) -> RSplit<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.rsplit(pred)) }
    }

    /// Returns an iterator over mutable subslices separated by elements that match pred, from the end
    pub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.rsplit_mut(pred)) }
    }

    /// Returns an iterator over subslices separated by elements that match pred, limited to n items
    pub fn splitn<F>(&self, n: usize, pred: F) -> SplitN<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.splitn(n, pred)) }
    }

    /// Returns an iterator over mutable subslices separated by elements that match pred, limited to n items
    pub fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.splitn_mut(n, pred)) }
    }

    /// Returns an iterator over subslices separated by elements that match pred, limited to n items, from the end
    pub fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIter::from_slice_iterator_unchecked(self.values.rsplitn(n, pred)) }
    }

    /// Returns an iterator over mutable subslices separated by elements that match pred, limited to n items, from the end
    pub fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<'_, F, T>
    where
        F: FnMut(&T) -> bool,
    {
        unsafe { UniqueEntityEquivalentSliceIterMut::from_mut_slice_iterator_unchecked(self.values.rsplitn_mut(n, pred)) }
    }

    /// Swaps two elements in the slice
    pub fn swap(&mut self, a: usize, b: usize) {
        self.values.swap(a, b);
    }

    /// Reverses the order of elements in the slice
    pub fn reverse(&mut self) {
        self.values.reverse();
    }

    /// Rotates the slice in-place to the left
    pub fn rotate_left(&mut self, mid: usize) {
        self.values.rotate_left(mid);
    }

    /// Rotates the slice in-place to the right
    pub fn rotate_right(&mut self, mid: usize) {
self.values.rotate_right(mid);
    }

    /// Sorts the slice **without** preserving the initial order of equal elements
    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.values.sort_unstable();
    }

    /// Sorts the slice with a comparison function, **without** preserving the initial order
    pub fn sort_unstable_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.values.sort_unstable_by(compare);
    }

    /// Sorts the slice with a key extraction function, **without** preserving the initial order
    pub fn sort_unstable_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.values.sort_unstable_by_key(f);
    }

    /// Sorts the slice, preserving initial order of equal elements
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.values.sort();
    }

    /// Sorts the slice with a comparison function, preserving initial order of equal elements
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.values.sort_by(compare);
    }

    /// Sorts the slice with a key extraction function, preserving initial order of equal elements
    pub fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.values.sort_by_key(f);
    }

    /// Sorts the slice with a key extraction function, preserving initial order of equal elements
    pub fn sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.values.sort_by_cached_key(f);
    }

    /// Copies self into a new `UniqueEntityEquivalentVec`
    pub fn to_vec(&self) -> UniqueEntityEquivalentVec<T>
    where
        T: Clone,
    {
        // SAFETY: All elements in the original slice are unique
        unsafe { UniqueEntityEquivalentVec::from_vec_unchecked(self.values.to_vec()) }
    }

    /// Converts `self` into a vector without clones or allocation
    pub fn into_vec(self: Box<Self>) -> UniqueEntityEquivalentVec<T> {
        // SAFETY: All elements in the original slice are unique
        unsafe {
            let len = self.len();
            let vec = Vec::from_raw_parts(Box::into_raw(self).cast::<T>(), len, len);
            UniqueEntityEquivalentVec::from_vec_unchecked(vec)
        }
    }

    /// Converts `self` into a slice
    pub fn into_slice(self: Box<Self>) -> Box<[T]> {
        self.into_boxed_inner()
    }
}

/// Converts a reference to T into a slice of length 1 (without copying)
pub const fn from_ref<T: EntityEquivalent>(s: &T) -> &UniqueEntityEquivalentSlice<T> {
    // SAFETY: A slice with a length of 1 is always unique
    unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(slice::from_ref(s)) }
}

/// Converts a mutable reference to T into a slice of length 1 (without copying)
pub const fn from_mut<T: EntityEquivalent>(s: &mut T) -> &mut UniqueEntityEquivalentSlice<T> {
    // SAFETY: A slice with a length of 1 is always unique
    unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(slice::from_mut(s)) }
}

/// Casts a slice of entity slices to a slice of [`UniqueEntityEquivalentSlice`]s
///
/// # Safety
///
/// All elements in each of the cast slices must be unique
pub unsafe fn cast_slice_of_unique_entity_slice<'a, 'b, T: EntityEquivalent + 'a>(
    slice: &'b [&'a [T]],
) -> &'b [&'a UniqueEntityEquivalentSlice<T>] {
    unsafe { &*(slice as *const [&[T]] as *const [&UniqueEntityEquivalentSlice<T>]) }
}

/// Casts a mutable slice of entity slices to a slice of [`UniqueEntityEquivalentSlice`]s
///
/// # Safety
///
/// All elements in each of the cast slices must be unique
pub unsafe fn cast_slice_of_unique_entity_slice_mut<'a, 'b, T: EntityEquivalent + 'a>(
    slice: &'b mut [&'a [T]],
) -> &'b mut [&'a UniqueEntityEquivalentSlice<T>] {
    unsafe { &mut *(slice as *mut [&[T]] as *mut [&UniqueEntityEquivalentSlice<T>]) }
}

/// Casts a mutable slice of mutable entity slices to a slice of mutable [`UniqueEntityEquivalentSlice`]s
///
/// # Safety
///
/// All elements in each of the cast slices must be unique
pub unsafe fn cast_slice_of_mut_unique_entity_slice_mut<'a, 'b, T: EntityEquivalent + 'a>(
    slice: &'b mut [&'a mut [T]],
) -> &'b mut [&'a mut UniqueEntityEquivalentSlice<T>] {
    unsafe { &mut *(slice as *mut [&mut [T]] as *mut [&mut UniqueEntityEquivalentSlice<T>]) }
}

impl<T: EntityEquivalent> std::ops::Deref for UniqueEntityEquivalentSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T: EntityEquivalent> AsRef<[T]> for UniqueEntityEquivalentSlice<T> {
    fn as_ref(&self) -> &[T] {
        &self.values
    }
}

impl<T: EntityEquivalent> Borrow<[T]> for UniqueEntityEquivalentSlice<T> {
    fn borrow(&self) -> &[T] {
        &self.values
    }
}

impl<T: EntityEquivalent> Index<usize> for UniqueEntityEquivalentSlice<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.values[index]
    }
}

impl<T: EntityEquivalent> Index<Range<usize>> for UniqueEntityEquivalentSlice<T> {
    type Output = Self;
    fn index(&self, key: Range<usize>) -> &Self {
        unsafe { Self::from_slice_unchecked(&self.values[key]) }
    }
}

impl<T: EntityEquivalent> Index<RangeFrom<usize>> for UniqueEntityEquivalentSlice<T> {
    type Output = Self;
    fn index(&self, key: RangeFrom<usize>) -> &Self {
        unsafe { Self::from_slice_unchecked(&self.values[key]) }
    }
}

impl<T: EntityEquivalent> Index<RangeFull> for UniqueEntityEquivalentSlice<T> {
    type Output = Self;
    fn index(&self, _key: RangeFull) -> &Self {
        self
    }
}

impl<T: EntityEquivalent> Index<RangeInclusive<usize>> for UniqueEntityEquivalentSlice<T> {
    type Output = Self;
    fn index(&self, key: RangeInclusive<usize>) -> &Self {
        unsafe { Self::from_slice_unchecked(&self.values[key]) }
    }
}

impl<T: EntityEquivalent> Index<RangeTo<usize>> for UniqueEntityEquivalentSlice<T> {
    type Output = Self;
    fn index(&self, key: RangeTo<usize>) -> &Self {
        unsafe { Self::from_slice_unchecked(&self.values[key]) }
    }
}

impl<T: EntityEquivalent> Index<RangeToInclusive<usize>> for UniqueEntityEquivalentSlice<T> {
    type Output = Self;
    fn index(&self, key: RangeToInclusive<usize>) -> &Self {
        unsafe { Self::from_slice_unchecked(&self.values[key]) }
    }
}

impl<T: EntityEquivalent> IndexMut<Range<usize>> for UniqueEntityEquivalentSlice<T> {
    fn index_mut(&mut self, key: Range<usize>) -> &mut Self {
        unsafe { Self::from_slice_unchecked_mut(&mut self.values[key]) }
    }
}

impl<T: EntityEquivalent> IndexMut<RangeFrom<usize>> for UniqueEntityEquivalentSlice<T> {
    fn index_mut(&mut self, key: RangeFrom<usize>) -> &mut Self {
        unsafe { Self::from_slice_unchecked_mut(&mut self.values[key]) }
    }
}

impl<T: EntityEquivalent> IndexMut<RangeFull> for UniqueEntityEquivalentSlice<T> {
    fn index_mut(&mut self, _key: RangeFull) -> &mut Self {
        self
    }
}

impl<T: EntityEquivalent> IndexMut<RangeInclusive<usize>> for UniqueEntityEquivalentSlice<T> {
    fn index_mut(&mut self, key: RangeInclusive<usize>) -> &mut Self {
        unsafe { Self::from_slice_unchecked_mut(&mut self.values[key]) }
    }
}

impl<T: EntityEquivalent> IndexMut<RangeTo<usize>> for UniqueEntityEquivalentSlice<T> {
    fn index_mut(&mut self, key: RangeTo<usize>) -> &mut Self {
        unsafe { Self::from_slice_unchecked_mut(&mut self.values[key]) }
    }
}

impl<T: EntityEquivalent> IndexMut<RangeToInclusive<usize>> for UniqueEntityEquivalentSlice<T> {
    fn index_mut(&mut self, key: RangeToInclusive<usize>) -> &mut Self {
        unsafe { Self::from_slice_unchecked_mut(&mut self.values[key]) }
    }
}

impl<'a, T: EntityEquivalent> IntoIterator for &'a UniqueEntityEquivalentSlice<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: EntityEquivalent> IntoIterator for &'a mut UniqueEntityEquivalentSlice<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: EntityEquivalent + Clone> ToOwned for UniqueEntityEquivalentSlice<T> {
    type Owned = UniqueEntityEquivalentVec<T>;

    fn to_owned(&self) -> Self::Owned {
        self.to_vec()
    }
}

/// Immutable slice iterator
pub struct Iter<'a, T: EntityEquivalent> {
    inner: slice::Iter<'a, T>,
}

impl<'a, T: EntityEquivalent> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T: EntityEquivalent> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T: EntityEquivalent> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'a, T: EntityEquivalent> FusedIterator for Iter<'a, T> {}

impl<'a, T: EntityEquivalent> Iter<'a, T> {
    /// Views the underlying data as a subslice
    pub fn as_slice(&self) -> &'a UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.inner.as_slice()) }
    }
}

/// Mutable slice iterator
pub struct IterMut<'a, T: EntityEquivalent> {
    inner: slice::IterMut<'a, T>,
}

impl<'a, T: EntityEquivalent> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T: EntityEquivalent> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T: EntityEquivalent> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'a, T: EntityEquivalent> FusedIterator for IterMut<'a, T> {}

impl<'a, T: EntityEquivalent> IterMut<'a, T> {
    /// Views the underlying data as a subslice
    pub fn as_slice(&self) -> &UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.inner.as_slice()) }
    }

    /// Views the underlying data as a mutable subslice
    pub fn into_slice(self) -> &'a mut UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.inner.into_slice()) }
    }
}

/// An iterator that yields `&UniqueEntityEquivalentSlice`
#[derive(Debug)]
pub struct UniqueEntityEquivalentSliceIter<
    'a,
    T: EntityEquivalent + 'a,
    I: Iterator<Item = &'a [T]>,
> {
    pub(crate) iter: I,
}

impl<'a, T: EntityEquivalent + 'a, I: Iterator<Item = &'a [T]>>
    UniqueEntityEquivalentSliceIter<'a, T, I>
{
    /// Constructs a [`UniqueEntityEquivalentSliceIter`] from a slice iterator unsafely
    ///
    /// # Safety
    ///
    /// All elements in each of the slices must be unique
    pub unsafe fn from_slice_iterator_unchecked(iter: I) -> Self {
        Self { iter }
    }

    /// Returns the inner `I`
    pub fn into_inner(self) -> I {
        self.iter
    }

    /// Returns a reference to the inner `I`
    pub fn as_inner(&self) -> &I {
        &self.iter
    }

    /// Returns a mutable reference to the inner `I`
    ///
    /// # Safety
    ///
    /// `self` must always contain an iterator that yields unique elements
    pub unsafe fn as_mut_inner(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<'a, T: EntityEquivalent + 'a, I: Iterator<Item = &'a [T]>> Iterator
    for UniqueEntityEquivalentSliceIter<'a, T, I>
{
    type Item = &'a UniqueEntityEquivalentSlice<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|slice| unsafe {
            UniqueEntityEquivalentSlice::from_slice_unchecked(slice)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: EntityEquivalent + 'a, I: ExactSizeIterator<Item = &'a [T]>> ExactSizeIterator
    for UniqueEntityEquivalentSliceIter<'a, T, I>
{
}

impl<'a, T: EntityEquivalent + 'a, I: DoubleEndedIterator<Item = &'a [T]>> DoubleEndedIterator
    for UniqueEntityEquivalentSliceIter<'a, T, I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|slice| unsafe {
            UniqueEntityEquivalentSlice::from_slice_unchecked(slice)
        })
    }
}

impl<'a, T: EntityEquivalent + 'a, I: FusedIterator<Item = &'a [T]>> FusedIterator
    for UniqueEntityEquivalentSliceIter<'a, T, I>
{
}

/// An iterator that yields `&mut UniqueEntityEquivalentSlice`
#[derive(Debug)]
pub struct UniqueEntityEquivalentSliceIterMut<
    'a,
    T: EntityEquivalent + 'a,
    I: Iterator<Item = &'a mut [T]>,
> {
    pub(crate) iter: I,
}

impl<'a, T: EntityEquivalent + 'a, I: Iterator<Item = &'a mut [T]>>
    UniqueEntityEquivalentSliceIterMut<'a, T, I>
{
    /// Constructs a [`UniqueEntityEquivalentSliceIterMut`] from a mutable slice iterator unsafely
    ///
    /// # Safety
    ///
    /// All elements in each of the slices must be unique
    pub unsafe fn from_mut_slice_iterator_unchecked(iter: I) -> Self {
        Self { iter }
    }

    /// Returns the inner `I`
    pub fn into_inner(self) -> I {
        self.iter
    }

    /// Returns a reference to the inner `I`
    pub fn as_inner(&self) -> &I {
        &self.iter
    }

    /// Returns a mutable reference to the inner `I`
    ///
    /// # Safety
    ///
    /// `self` must always contain an iterator that yields unique elements
    pub unsafe fn as_mut_inner(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<'a, T: EntityEquivalent + 'a, I: Iterator<Item = &'a mut [T]>> Iterator
    for UniqueEntityEquivalentSliceIterMut<'a, T, I>
{
    type Item = &'a mut UniqueEntityEquivalentSlice<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|slice| unsafe {
            UniqueEntityEquivalentSlice::from_slice_unchecked_mut(slice)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: EntityEquivalent + 'a, I: ExactSizeIterator<Item = &'a mut [T]>> ExactSizeIterator
    for UniqueEntityEquivalentSliceIterMut<'a, T, I>
{
}

impl<'a, T: EntityEquivalent + 'a, I: DoubleEndedIterator<Item = &'a mut [T]>> DoubleEndedIterator
    for UniqueEntityEquivalentSliceIterMut<'a, T, I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|slice| unsafe {
            UniqueEntityEquivalentSlice::from_slice_unchecked_mut(slice)
        })
    }
}

impl<'a, T: EntityEquivalent + 'a, I: FusedIterator<Item = &'a mut [T]>> FusedIterator
    for UniqueEntityEquivalentSliceIterMut<'a, T, I>
{
}

// Additional methods for ChunksExact iterator
impl<'a, T: EntityEquivalent + Clone> UniqueEntityEquivalentSliceIter<'a, T, slice::ChunksExact<'a, T>> {
    /// Returns the remainder of the original slice
    pub fn remainder(&self) -> &'a UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.iter.remainder()) }
    }
}

// Additional methods for RChunksExact iterator
impl<'a, T: EntityEquivalent + Clone> UniqueEntityEquivalentSliceIter<'a, T, slice::RChunksExact<'a, T>> {
    /// Returns the remainder of the original slice
    pub fn remainder(&self) -> &'a UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(self.iter.remainder()) }
    }
}

// Additional methods for ChunksExactMut iterator
impl<'a, T: EntityEquivalent + Clone> UniqueEntityEquivalentSliceIterMut<'a, T, slice::ChunksExactMut<'a, T>> {
    /// Returns the remainder of the original slice
    pub fn into_remainder(self) -> &'a mut UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.iter.into_remainder()) }
    }
}

// Additional methods for RChunksExactMut iterator
impl<'a, T: EntityEquivalent + Clone> UniqueEntityEquivalentSliceIterMut<'a, T, slice::RChunksExactMut<'a, T>> {
    /// Returns the remainder of the original slice
    pub fn into_remainder(self) -> &'a mut UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked_mut(self.iter.into_remainder()) }
    }
}

/// Iterator type aliases for all the standard slice iterators
pub type Windows<'a, T = Entity> = 
        
UniqueEntityEquivalentSliceIter<'a, T, slice::Windows<'a, T>>;
pub type Chunks<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::Chunks<'a, T>>;
pub type ChunksExact<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::ChunksExact<'a, T>>;
pub type RChunks<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::RChunks<'a, T>>;
pub type RChunksExact<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::RChunksExact<'a, T>>;
pub type ChunkBy<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::ChunkBy<'a, T, P>>;
pub type Split<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::Split<'a, T, P>>;
pub type SplitInclusive<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::SplitInclusive<'a, T, P>>;
pub type RSplit<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::RSplit<'a, T, P>>;
pub type SplitN<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::SplitN<'a, T, P>>;
pub type RSplitN<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, slice::RSplitN<'a, T, P>>;

pub type ChunksMut<'a, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::ChunksMut<'a, T>>;
pub type ChunksExactMut<'a, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::ChunksExactMut<'a, T>>;
pub type RChunksMut<'a, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::RChunksMut<'a, T>>;
pub type RChunksExactMut<'a, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::RChunksExactMut<'a, T>>;
pub type ChunkByMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::ChunkByMut<'a, T, P>>;
pub type SplitMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::SplitMut<'a, T, P>>;
pub type SplitInclusiveMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::SplitInclusiveMut<'a, T, P>>;
pub type RSplitMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::RSplitMut<'a, T, P>>;
pub type SplitNMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::SplitNMut<'a, T, P>>;
pub type RSplitNMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, slice::RSplitNMut<'a, T, P>>;