//! EntityIndexSet - IndexSet specialized for Entity

use super::{Entity, EntityHash};
use indexmap::IndexSet;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::ops::Index;
use std::{fmt, ptr};
use std::marker::PhantomData;
use std::iter::FusedIterator;

/// EntityIndexSet - IndexSet optimized for Entity with preserved insertion order
#[derive(Clone)]
pub struct EntityIndexSet {
    inner: IndexSet<Entity, EntityHash>,
}

impl EntityIndexSet {
    /// Creates an empty EntityIndexSet
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: IndexSet::with_hasher(EntityHash),
        }
    }

    /// Creates an empty EntityIndexSet with specified capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: IndexSet::with_capacity_and_hasher(capacity, EntityHash),
        }
    }

    /// Returns the number of elements in the set
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the capacity
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the set
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Reserves capacity
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Shrinks the capacity as much as possible
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Shrinks capacity with a lower bound
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    /// Inserts a value
    #[inline]
    pub fn insert(&mut self, value: Entity) -> bool {
        self.inner.insert(value)
    }

    /// Removes a value
    #[inline]
    pub fn remove(&mut self, value: &Entity) -> bool {
        self.inner.shift_remove(value)
    }

    /// Removes a value by swapping it with the last element
    #[inline]
    pub fn swap_remove(&mut self, value: &Entity) -> bool {
        self.inner.swap_remove(value)
    }

    /// Removes and returns the value at the index
    #[inline]
    pub fn swap_remove_index(&mut self, index: usize) -> Option<Entity> {
        self.inner.swap_remove_index(index)
    }

    /// Removes and returns the value at the index, shifting all later values
    #[inline]
    pub fn shift_remove_index(&mut self, index: usize) -> Option<Entity> {
        self.inner.shift_remove_index(index)
    }

    /// Returns true if the set contains the value
    #[inline]
    pub fn contains(&self, value: &Entity) -> bool {
        self.inner.contains(value)
    }

    /// Gets a reference to the value
    #[inline]
    pub fn get(&self, value: &Entity) -> Option<&Entity> {
        self.inner.get(value)
    }

    /// Gets the index of a value
    #[inline]
    pub fn get_index_of(&self, value: &Entity) -> Option<usize> {
        self.inner.get_index_of(value)
    }

    /// Gets the value at an index
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&Entity> {
        self.inner.get_index(index)
    }

    /// Gets the first value
    #[inline]
    pub fn first(&self) -> Option<&Entity> {
        self.inner.first()
    }

    /// Gets the last value
    #[inline]
    pub fn last(&self) -> Option<&Entity> {
        self.inner.last()
    }

    /// Swaps the position of two values
    #[inline]
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        self.inner.swap_indices(a, b)
    }

    /// Removes the last element
    #[inline]
    pub fn pop(&mut self) -> Option<Entity> {
        self.inner.pop()
    }

    /// Retains only the elements specified by the predicate
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Entity) -> bool,
    {
        self.inner.retain(f);
    }

    /// Sorts the set
    #[inline]
    pub fn sort(&mut self)
    where
        Entity: Ord,
    {
        self.inner.sort();
    }

    /// Sorts the set by the predicate
    #[inline]
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Entity, &Entity) -> std::cmp::Ordering,
    {
        self.inner.sort_by(compare);
    }

    /// Reverses the order of the set in-place
    #[inline]
    pub fn reverse(&mut self) {
        self.inner.reverse();
    }

    /// Returns a slice of all the values in the set
    #[inline]
    pub fn as_slice(&self) -> &Slice {
        // SAFETY: Slice is a transparent wrapper around indexmap::set::Slice
        unsafe { Slice::from_slice_unchecked(self.inner.as_slice()) }
    }

    /// Converts into a boxed slice of all the values in the set
    #[inline]
    pub fn into_boxed_slice(self) -> Box<Slice> {
        // SAFETY: Slice is a transparent wrapper around indexmap::set::Slice
        unsafe { Slice::from_boxed_slice_unchecked(self.inner.into_boxed_slice()) }
    }

    /// Returns a slice of values in the given range of indices
    #[inline]
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice> {
        self.inner.get_range(range).map(|slice|
            // SAFETY: Slice is a transparent wrapper around indexmap::set::Slice
            unsafe { Slice::from_slice_unchecked(slice) })
    }

    /// An iterator visiting all values
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.inner.iter(), PhantomData)
    }

    /// Clears the set, returning all values as an iterator
    #[inline]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_>
    where
        R: RangeBounds<usize>,
    {
        Drain(self.inner.drain(range), PhantomData)
    }

    /// Visits the values representing the difference
    #[inline]
    pub fn difference<'a>(&'a self, other: &'a EntityIndexSet) -> impl Iterator<Item = &'a Entity> {
        self.inner.difference(&other.inner)
    }

    /// Visits the values representing the symmetric difference
    #[inline]
    pub fn symmetric_difference<'a>(&'a self, other: &'a EntityIndexSet) -> impl Iterator<Item = &'a Entity> {
        self.inner.symmetric_difference(&other.inner)
    }

    /// Visits the values representing the intersection
    #[inline]
    pub fn intersection<'a>(&'a self, other: &'a EntityIndexSet) -> impl Iterator<Item = &'a Entity> {
        self.inner.intersection(&other.inner)
    }

    /// Visits the values representing the union
    #[inline]
    pub fn union<'a>(&'a self, other: &'a EntityIndexSet) -> impl Iterator<Item = &'a Entity> {
        self.inner.union(&other.inner)
    }

    /// Returns true if the set has no elements in common with other
    #[inline]
    pub fn is_disjoint(&self, other: &EntityIndexSet) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    /// Returns true if the set is a subset of another
    #[inline]
    pub fn is_subset(&self, other: &EntityIndexSet) -> bool {
        self.inner.is_subset(&other.inner)
    }

    /// Returns true if the set is a superset of another
    #[inline]
    pub fn is_superset(&self, other: &EntityIndexSet) -> bool {
        self.inner.is_superset(&other.inner)
    }
}

impl Default for EntityIndexSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::iter::FromIterator<Entity> for EntityIndexSet {
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl Extend<Entity> for EntityIndexSet {
    fn extend<T: IntoIterator<Item = Entity>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

impl IntoIterator for EntityIndexSet {
    type Item = Entity;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.inner.into_iter(), PhantomData)
    }
}

impl<'a> IntoIterator for &'a EntityIndexSet {
    type Item = &'a Entity;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.inner.iter(), PhantomData)
    }
}

impl fmt::Debug for EntityIndexSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl PartialEq for EntityIndexSet {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for EntityIndexSet {}

// ============================================================================
// Slice type and implementations
// ============================================================================

/// A dynamically-sized slice of values in an EntityIndexSet
#[repr(transparent)]
pub struct Slice(PhantomData<EntityHash>, indexmap::set::Slice<Entity>);

impl Slice {
    /// Returns an empty slice
    #[inline]
    pub const fn new<'a>() -> &'a Self {
        // SAFETY: The source slice is empty
        unsafe { Self::from_slice_unchecked(indexmap::set::Slice::new()) }
    }

    /// Constructs a Slice from an indexmap::set::Slice unsafely
    ///
    /// # Safety
    ///
    /// `slice` must stem from an IndexSet using EntityHash
    #[inline]
    pub const unsafe fn from_slice_unchecked(slice: &indexmap::set::Slice<Entity>) -> &Self {
        // SAFETY: Slice is a transparent wrapper around indexmap::set::Slice
        unsafe { &*(ptr::from_ref(slice) as *const Self) }
    }

    /// Casts self to the inner slice
    #[inline]
    pub const fn as_inner(&self) -> &indexmap::set::Slice<Entity> {
        &self.1
    }

    /// Constructs a boxed Slice from a boxed indexmap::set::Slice unsafely
    ///
    /// # Safety
    ///
    /// `slice` must stem from an IndexSet using EntityHash
    #[inline]
    pub unsafe fn from_boxed_slice_unchecked(slice: Box<indexmap::set::Slice<Entity>>) -> Box<Self> {
        // SAFETY: Slice is a transparent wrapper around indexmap::set::Slice
        unsafe { Box::from_raw(Box::into_raw(slice) as *mut Self) }
    }

    /// Casts a reference to self to the inner boxed slice
    #[inline]
    #[allow(clippy::borrowed_box)]
    pub fn as_boxed_inner(self: &Box<Self>) -> &Box<indexmap::set::Slice<Entity>> {
        // SAFETY: Slice is a transparent wrapper around indexmap::set::Slice
        unsafe { &*(ptr::from_ref(self).cast::<Box<indexmap::set::Slice<Entity>>>()) }
    }

    /// Casts self to the inner boxed slice
    #[inline]
    pub fn into_boxed_inner(self: Box<Self>) -> Box<indexmap::set::Slice<Entity>> {
        // SAFETY: Slice is a transparent wrapper around indexmap::set::Slice
        unsafe { Box::from_raw(Box::into_raw(self) as *mut indexmap::set::Slice<Entity>) }
    }

    /// Returns a slice of values in the given range of indices
    #[inline]
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Self> {
        self.1.get_range(range).map(|slice|
            // SAFETY: This is a subslice of a valid slice
            unsafe { Self::from_slice_unchecked(slice) })
    }

    /// Divides one slice into two at an index
    #[inline]
    pub fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (slice_1, slice_2) = self.1.split_at(index);
        // SAFETY: These are subslices of a valid slice
        unsafe {
            (
                Self::from_slice_unchecked(slice_1),
                Self::from_slice_unchecked(slice_2),
            )
        }
    }

    /// Returns the first value and the rest of the slice, or None if empty
    #[inline]
    pub fn split_first(&self) -> Option<(&Entity, &Self)> {
        self.1.split_first().map(|(first, rest)| {
            (
                first,
                // SAFETY: This is a subslice of a valid slice
                unsafe { Self::from_slice_unchecked(rest) },
            )
        })
    }

    /// Returns the last value and the rest of the slice, or None if empty
    #[inline]
    pub fn split_last(&self) -> Option<(&Entity, &Self)> {
        self.1.split_last().map(|(last, rest)| {
            (
                last,
                // SAFETY: This is a subslice of a valid slice
                unsafe { Self::from_slice_unchecked(rest) },
            )
        })
    }

    /// Return an iterator over the values of the set slice
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.1.iter(), PhantomData)
    }
}

impl std::ops::Deref for Slice {
    type Target = indexmap::set::Slice<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<'a> IntoIterator for &'a Slice {
    type IntoIter = Iter<'a>;
    type Item = &'a Entity;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for Box<Slice> {
    type IntoIter = IntoIter;
    type Item = Entity;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.into_boxed_inner().into_iter(), PhantomData)
    }
}

impl Clone for Box<Slice> {
    fn clone(&self) -> Self {
        // SAFETY: This is a clone of a valid slice
        unsafe { Slice::from_boxed_slice_unchecked(self.as_boxed_inner().clone()) }
    }
}

impl Default for &Slice {
    fn default() -> Self {
        // SAFETY: The source slice is empty
        unsafe { Slice::from_slice_unchecked(<&indexmap::set::Slice<Entity>>::default()) }
    }
}

impl Default for Box<Slice> {
    fn default() -> Self {
        // SAFETY: The source slice is empty
        unsafe { Slice::from_boxed_slice_unchecked(<Box<indexmap::set::Slice<Entity>>>::default()) }
    }
}

impl fmt::Debug for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Slice")
            .field(&self.0)
            .field(&&self.1)
            .finish()
    }
}

impl From<&Slice> for Box<Slice> {
    fn from(value: &Slice) -> Self {
        // SAFETY: This slice is a copy of a valid slice
        unsafe { Slice::from_boxed_slice_unchecked(value.1.into()) }
    }
}

impl PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Slice {}

// ============================================================================
// Index implementations for Slice
// ============================================================================

impl Index<(Bound<usize>, Bound<usize>)> for Slice {
    type Output = Self;
    fn index(&self, key: (Bound<usize>, Bound<usize>)) -> &Self {
        // SAFETY: This is a subslice of a valid slice
        unsafe { Self::from_slice_unchecked(self.1.index(key)) }
    }
}

impl Index<Range<usize>> for Slice {
    type Output = Self;
    fn index(&self, key: Range<usize>) -> &Self {
        // SAFETY: This is a subslice of a valid slice
        unsafe { Self::from_slice_unchecked(self.1.index(key)) }
    }
}

impl Index<RangeFrom<usize>> for Slice {
    type Output = Slice;
    fn index(&self, key: RangeFrom<usize>) -> &Self {
        // SAFETY: This is a subslice of a valid slice
        unsafe { Self::from_slice_unchecked(self.1.index(key)) }
    }
}

impl Index<RangeFull> for Slice {
    type Output = Self;
    fn index(&self, key: RangeFull) -> &Self {
        // SAFETY: This is a subslice of a valid slice
        unsafe { Self::from_slice_unchecked(self.1.index(key)) }
    }
}

impl Index<RangeInclusive<usize>> for Slice {
    type Output = Self;
    fn index(&self, key: RangeInclusive<usize>) -> &Self {
        // SAFETY: This is a subslice of a valid slice
        unsafe { Self::from_slice_unchecked(self.1.index(key)) }
    }
}

impl Index<RangeTo<usize>> for Slice {
    type Output = Self;
    fn index(&self, key: RangeTo<usize>) -> &Self {
        // SAFETY: This is a subslice of a valid slice
        unsafe { Self::from_slice_unchecked(self.1.index(key)) }
    }
}

impl Index<RangeToInclusive<usize>> for Slice {
    type Output = Self;
    fn index(&self, key: RangeToInclusive<usize>) -> &Self {
        // SAFETY: This is a subslice of a valid slice
        unsafe { Self::from_slice_unchecked(self.1.index(key)) }
    }
}

impl Index<usize> for Slice {
    type Output = Entity;
    fn index(&self, key: usize) -> &Entity {
        self.1.index(key)
    }
}

// ============================================================================
// Iterator types
// ============================================================================

/// An iterator over the items of an EntityIndexSet
pub struct Iter<'a>(indexmap::set::Iter<'a, Entity>, PhantomData<EntityHash>);

impl<'a> Iter<'a> {
/// Returns a slice of /// Returns a slice of the remaining entries in the iterator
    #[inline]
    pub fn as_slice(&self) -> &Slice {
        // SAFETY: The source IndexSet uses EntityHash
        unsafe { Slice::from_slice_unchecked(self.0.as_slice()) }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl ExactSizeIterator for Iter<'_> {}

impl FusedIterator for Iter<'_> {}

impl Clone for Iter<'_> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl fmt::Debug for Iter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.0).finish()
    }
}

impl Default for Iter<'_> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// Owning iterator over the items of an EntityIndexSet
pub struct IntoIter(indexmap::set::IntoIter<Entity>, PhantomData<EntityHash>);

impl IntoIter {
    /// Returns a slice of the remaining entries in the iterator
    #[inline]
    pub fn as_slice(&self) -> &Slice {
        // SAFETY: The source IndexSet uses EntityHash
        unsafe { Slice::from_slice_unchecked(self.0.as_slice()) }
    }
}

impl Iterator for IntoIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl ExactSizeIterator for IntoIter {}

impl FusedIterator for IntoIter {}

impl Clone for IntoIter {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl fmt::Debug for IntoIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.0).finish()
    }
}

impl Default for IntoIter {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// A draining iterator over the items of an EntityIndexSet
pub struct Drain<'a>(indexmap::set::Drain<'a, Entity>, PhantomData<EntityHash>);

impl<'a> Drain<'a> {
    /// Returns a slice of the remaining entries in the iterator
    #[inline]
    pub fn as_slice(&self) -> &Slice {
        // SAFETY: The source IndexSet uses EntityHash
        unsafe { Slice::from_slice_unchecked(self.0.as_slice()) }
    }
}

impl<'a> Iterator for Drain<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for Drain<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl ExactSizeIterator for Drain<'_> {}

impl FusedIterator for Drain<'_> {}

impl fmt::Debug for Drain<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Drain").field(&self.0).finish()
    }
}
