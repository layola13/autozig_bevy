//! EntityIndexMap - IndexMap specialized for Entity keys

use super::{Entity, EntityHash};
use indexmap::IndexMap;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::ops::{Index, IndexMut};
use std::{fmt, ptr};
use std::marker::PhantomData;
use std::iter::FusedIterator;

/// EntityIndexMap - IndexMap optimized for Entity keys with preserved insertion order
#[derive(Clone)]
pub struct EntityIndexMap<V> {
    inner: IndexMap<Entity, V, EntityHash>,
}

impl<V> EntityIndexMap<V> {
    /// Creates an empty EntityIndexMap
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: IndexMap::with_hasher(EntityHash),
        }
    }

    /// Creates an empty EntityIndexMap with specified capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity_and_hasher(capacity, EntityHash),
        }
    }

    /// Returns the number of elements in the map
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the map is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the capacity of the map
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the map
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Reserves capacity for at least additional more elements
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

    /// Inserts a key-value pair
    #[inline]
    pub fn insert(&mut self, key: Entity, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// Removes a key from the map
    #[inline]
    pub fn remove(&mut self, key: &Entity) -> Option<V> {
        self.inner.shift_remove(key)
    }

    /// Removes a key-value pair by swapping it with the last element
    #[inline]
    pub fn swap_remove(&mut self, key: &Entity) -> Option<V> {
        self.inner.swap_remove(key)
    }

    /// Removes and returns the entry at the index
    #[inline]
    pub fn swap_remove_index(&mut self, index: usize) -> Option<(Entity, V)> {
        self.inner.swap_remove_index(index)
    }

    /// Removes and returns the entry at the index, shifting all later entries
    #[inline]
    pub fn shift_remove_index(&mut self, index: usize) -> Option<(Entity, V)> {
        self.inner.shift_remove_index(index)
    }

    /// Gets a reference to the value
    #[inline]
    pub fn get(&self, key: &Entity) -> Option<&V> {
        self.inner.get(key)
    }

    /// Gets a mutable reference to the value
    #[inline]
    pub fn get_mut(&mut self, key: &Entity) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    /// Gets the key-value pair
    #[inline]
    pub fn get_key_value(&self, key: &Entity) -> Option<(&Entity, &V)> {
        self.inner.get_key_value(key)
    }

    /// Returns true if the map contains the key
    #[inline]
    pub fn contains_key(&self, key: &Entity) -> bool {
        self.inner.contains_key(key)
    }

    /// Gets the index of a key
    #[inline]
    pub fn get_index_of(&self, key: &Entity) -> Option<usize> {
        self.inner.get_index_of(key)
    }

    /// Gets the entry at an index
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<(&Entity, &V)> {
        self.inner.get_index(index)
    }

    /// Gets a mutable entry at an index
    #[inline]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&Entity, &mut V)> {
        self.inner.get_index_mut(index)
    }

    /// Gets the first entry
    #[inline]
    pub fn first(&self) -> Option<(&Entity, &V)> {
        self.inner.first()
    }

    /// Gets the first entry with mutable value
    #[inline]
    pub fn first_mut(&mut self) -> Option<(&Entity, &mut V)> {
        self.inner.first_mut()
    }

    /// Gets the last entry
    #[inline]
    pub fn last(&self) -> Option<(&Entity, &V)> {
        self.inner.last()
    }

    /// Gets the last entry with mutable value
    #[inline]
    pub fn last_mut(&mut self) -> Option<(&Entity, &mut V)> {
        self.inner.last_mut()
    }

    /// Swaps the position of two key-value pairs
    #[inline]
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        self.inner.swap_indices(a, b)
    }

    /// Removes the last element
    #[inline]
    pub fn pop(&mut self) -> Option<(Entity, V)> {
        self.inner.pop()
    }

    /// Retains only the elements specified by the predicate
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Entity, &mut V) -> bool,
    {
        self.inner.retain(f);
    }

    /// Sorts the map by keys
    #[inline]
    pub fn sort_keys(&mut self)
    where
        Entity: Ord,
    {
        self.inner.sort_keys();
    }

    /// Sorts the map by the predicate
    #[inline]
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Entity, &V, &Entity, &V) -> std::cmp::Ordering,
    {
        self.inner.sort_by(compare);
    }

    /// Reverses the order of the map in-place
    #[inline]
    pub fn reverse(&mut self) {
        self.inner.reverse();
    }

    /// Returns a slice of all the key-value pairs in the map
    #[inline]
    pub fn as_slice(&self) -> &Slice<V> {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { Slice::from_slice_unchecked(self.inner.as_slice()) }
    }

    /// Returns a mutable slice of all the key-value pairs in the map
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut Slice<V> {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { Slice::from_slice_unchecked_mut(self.inner.as_mut_slice()) }
    }

    /// Converts into a boxed slice of all the key-value pairs in the map
    #[inline]
    pub fn into_boxed_slice(self) -> Box<Slice<V>> {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { Slice::from_boxed_slice_unchecked(self.inner.into_boxed_slice()) }
    }

    /// Returns a slice of key-value pairs in the given range of indices
    #[inline]
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice<V>> {
        self.inner.get_range(range).map(|slice|
            // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
            unsafe { Slice::from_slice_unchecked(slice) })
    }

    /// Returns a mutable slice of key-value pairs in the given range of indices
    #[inline]
    pub fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<V>> {
        self.inner.get_range_mut(range).map(|slice|
            // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
            unsafe { Slice::from_slice_unchecked_mut(slice) })
    }

    /// An iterator visiting all keys
    #[inline]
    pub fn keys(&self) -> Keys<'_, V> {
        Keys(self.inner.keys(), PhantomData)
    }

    /// An owning iterator over the keys of the map
    #[inline]
    pub fn into_keys(self) -> IntoKeys<V> {
        IntoKeys(self.inner.into_keys(), PhantomData)
    }

    /// An iterator visiting all values
    #[inline]
    pub fn values(&self) -> indexmap::map::Values<'_, Entity, V> {
        self.inner.values()
    }

    /// An iterator visiting all values mutably
    #[inline]
    pub fn values_mut(&mut self) -> indexmap::map::ValuesMut<'_, Entity, V> {
        self.inner.values_mut()
    }

    /// An iterator visiting all key-value pairs
    #[inline]
    pub fn iter(&self) -> Iter<'_, V> {
        Iter(self.inner.iter(), PhantomData)
    }

    /// An iterator visiting all key-value pairs with mutable values
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        IterMut(self.inner.iter_mut(), PhantomData)
    }

    /// Clears the map, returning all key-value pairs as an iterator
    #[inline]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, V>
    where
        R: RangeBounds<usize>,
    {
        Drain(self.inner.drain(range), PhantomData)
    }
}

impl<V> Default for EntityIndexMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> std::iter::FromIterator<(Entity, V)> for EntityIndexMap<V> {
    fn from_iter<T: IntoIterator<Item = (Entity, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        map.extend(iter);
        map
    }
}

impl<V> Extend<(Entity, V)> for EntityIndexMap<V> {
    fn extend<T: IntoIterator<Item = (Entity, V)>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

impl<V> IntoIterator for EntityIndexMap<V> {
    type Item = (Entity, V);
    type IntoIter = IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.inner.into_iter(), PhantomData)
    }
}

impl<'a, V> IntoIterator for &'a EntityIndexMap<V> {
    type Item = (&'a Entity, &'a V);
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.inner.iter(), PhantomData)
    }
}

impl<'a, V> IntoIterator for &'a mut EntityIndexMap<V> {
    type Item = (&'a Entity, &'a mut V);
    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.inner.iter_mut(), PhantomData)
    }
}

impl<V: fmt::Debug> fmt::Debug for EntityIndexMap<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<V: PartialEq> PartialEq for EntityIndexMap<V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<V: Eq> Eq for EntityIndexMap<V> {}

// ============================================================================
// Slice type and implementations
// ============================================================================

/// A dynamically-sized slice of key-value pairs in an EntityIndexMap
#[repr(transparent)]
pub struct Slice<V>(PhantomData<EntityHash>, indexmap::map::Slice<Entity, V>);

impl<V> Slice<V> {
    /// Returns an empty slice
    #[inline]
    pub const fn new<'a>() -> &'a Self {
        // SAFETY: The source slice is empty
        unsafe { Self::from_slice_unchecked(indexmap::map::Slice::new()) }
    }

    /// Returns an empty mutable slice
    #[inline]
    pub fn new_mut<'a>() -> &'a mut Self {
        // SAFETY: The source slice is empty
        unsafe { Self::from_slice_unchecked_mut(indexmap::map::Slice::new_mut()) }
    }

    /// Constructs a Slice from an indexmap::map::Slice unsafely
    ///
    /// # Safety
    ///
    /// `slice` must stem from an IndexMap using EntityHash
    #[inline]
    pub const unsafe fn from_slice_unchecked(slice: &indexmap::map::Slice<Entity, V>) -> &Self {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { &*(ptr::from_ref(slice) as *const Self) }
    }

    /// Constructs a mutable Slice from an indexmap::map::Slice unsafely
    ///
    /// # Safety
    ///
    /// `slice` must stem from an IndexMap using EntityHash
    #[inline]
    pub const unsafe fn from_slice_unchecked_mut(slice: &mut indexmap::map::Slice<Entity, V>) -> &mut Self {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { &mut *(ptr::from_mut(slice) as *mut Self) }
    }

    /// Casts self to the inner slice
    #[inline]
    pub const fn as_inner(&self) -> &indexmap::map::Slice<Entity, V> {
        &self.1
    }

    /// Constructs a boxed Slice from a boxed indexmap::map::Slice unsafely
    ///
    /// # Safety
    ///
    /// `slice` must stem from an IndexMap using EntityHash
    #[inline]
    pub unsafe fn from_boxed_slice_unchecked(slice: Box<indexmap::map::Slice<Entity, V>>) -> Box<Self> {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { Box::from_raw(Box::into_raw(slice) as *mut Self) }
    }

    /// Casts a reference to self to the inner boxed slice
    #[inline]
    #[allow(clippy::borrowed_box)]
    pub fn as_boxed_inner(self: &Box<Self>) -> &Box<indexmap::map::Slice<Entity, V>> {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { &*(ptr::from_ref(self).cast::<Box<indexmap::map::Slice<Entity, V>>>()) }
    }

    /// Casts self to the inner boxed slice
    #[inline]
    pub fn into_boxed_inner(self: Box<Self>) -> Box<indexmap::map::Slice<Entity, V>> {
        // SAFETY: Slice is a transparent wrapper around indexmap::map::Slice
        unsafe { Box::from_raw(Box::into_raw(self) as *mut indexmap::map::Slice<Entity, V>) }
    }

    /// Get a key-value pair by index, with mutable access to the value
    #[inline]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&Entity, &mut V)> {
        self.1.get_index_mut(index)
    }

    /// Returns a slice of key-value pairs in the given range of indices
    #[inline]
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Self> {
        self.1.get_range(range).map(|slice|
            // SAFETY: This is a subslice of a valid slice
            unsafe { Self::from_slice_unchecked(slice) })
    }

    /// Returns a mutable slice of key-value pairs in the given range of indices
    #[inline]
    pub fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
        self.1.get_range_mut(range).map(|slice|
            // SAFETY: This is a subslice of a valid slice
            unsafe { Self::from_slice_unchecked_mut(slice) })
    }

    /// Get the first key-value pair, with mutable access to the value
    #[inline]
    pub fn first_mut(&mut self) -> Option<(&Entity, &mut V)> {
        self.1.first_mut()
    }

    /// Get the last key-value pair, with mutable access to the value
    #[inline]
    pub fn last_mut(&mut self) -> Option<(&Entity, &mut V)> {
        self.1.last_mut()
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

    /// Divides one mutable slice into two at an index
    #[inline]
    pub fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
        let (slice_1, slice_2) = self.1.split_at_mut(index);
        // SAFETY: These are subslices of a valid slice
        unsafe {
            (
                Self::from_slice_unchecked_mut(slice_1),
                Self::from_slice_unchecked_mut(slice_2),
            )
        }
    }

    /// Returns the first key-value pair and the rest of the slice, or None if empty
    #[inline]
    pub fn split_first(&self) -> Option<((&Entity, &V), &Self)> {
        self.1.split_first().map(|(first, rest)| {
            (
                first,
                // SAFETY: This is a subslice of a valid slice
                unsafe { Self::from_slice_unchecked(rest) },
            )
        })
    }

    /// Returns the first key-value pair and the rest of the slice, with mutable access, or None if empty
    #[inline]
    pub fn split_first_mut(&mut self) -> Option<((&Entity, &mut V), &mut Self)> {
        self.1.split_first_mut().map(|(first, rest)| {
            (
                first,
                // SAFETY: This is a subslice of a valid slice
                unsafe { Self::from_slice_unchecked_mut(rest) },
            )
        })
    }

    /// Returns the last key-value pair and the rest of the slice, or None if empty
    #[inline]
    pub fn split_last(&self) -> Option<((&Entity, &V), &Self)> {
        self.1.split_last().map(|(last, rest)| {
            (
                last,
                // SAFETY: This is a subslice of a valid slice
                unsafe { Self::from_slice_unchecked(rest) },
            )
        })
    }

    /// Returns the last key-value pair and the rest of the slice, with mutable access, or None if empty
    #[inline]
    pub fn split_last_mut(&mut self) -> Option<((&Entity, &mut V), &mut Self)> {
        self.1.split_last_mut().map(|(last, rest)| {
            (
                last,
                // SAFETY: This is a subslice of a valid slice
                unsafe { Self::from_slice_unchecked_mut(rest) },
            )
        })
    }

    /// Return an iterator over the key-value pairs of the map slice
    #[inline]
    pub fn iter(&self) -> Iter<'_, V> {
        Iter(self.1.iter(), PhantomData)
    }

    /// Return an iterator over the key-value pairs of the map slice with mutable values
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        IterMut(self.1.iter_mut(), PhantomData)
    }

    /// Return an iterator over the keys of the map slice
    #[inline]
    pub fn keys(&self) -> Keys<'_, V> {
        Keys(self.1.keys(), PhantomData)
    }

    /// Return an owning iterator over the keys of the map slice
    #[inline]
    pub fn into_keys(self: Box<Self>) -> IntoKeys<V> {
        IntoKeys(self.into_boxed_inner().into_keys(), PhantomData)
    }

    /// Return an iterator over mutable references to the values of the map slice
    #[inline]
    pub fn values_mut(&mut self) -> indexmap::map::ValuesMut<'_, Entity, V> {
        self.1.values_mut()
    }

    /// Return an owning iterator over the values of the map slice
    #[inline]
    pub fn into_values(self: Box<Self>) -> indexmap::map::IntoValues<Entity, V> {
        self.into_boxed_inner().into_values()
    }
}

impl<V> std::ops::Deref for Slice<V> {
    type Target = indexmap::map::Slice<Entity, V>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<V: fmt::Debug> fmt::Debug for Slice<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Slice")
            .field(&self.0)
            .field(&&self.1)
            .finish()
    }
}

impl<V: Clone> Clone for Box<Slice<V>> {
    fn clone(&self) -> Self {
        // SAFETY: This is a clone of a valid slice
        unsafe { Slice::from_boxed_slice_unchecked(self.as_boxed_inner().clone()) }
    }
}

impl<V> Default for &Slice<V> {
    fn default() -> Self {
        // SAFETY: The source slice is empty
        unsafe { Slice::from_slice_unchecked(<&indexmap::map::Slice<Entity, V>>::default()) }
    }
}

impl<V> Default for &mut Slice<V> {
    fn default() -> Self {
        // SAFETY: The source slice is empty
        unsafe { Slice::from_slice_unchecked_mut(<&mut indexmap::map::Slice<Entity, V>>::default()) }
    }
}

impl<V> Default for Box<Slice<V>> {
    fn default() -> Self {
        // SAFETY: The source slice is empty
        unsafe { Slice::from_boxed_slice_unchecked(<Box<indexmap::map::Slice<Entity, V>>>::default()) }
    }
}

impl<'a, V> IntoIterator for &'a Slice<V> {
    type Item = (&'a Entity, &'a V);
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.1.iter(), PhantomData)
    }
}

impl<'a, V> IntoIterator for &'a mut Slice<V> {
    type Item = (&'a Entity, &'a mut V);
    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.1.iter_mut(), PhantomData)
    }
}

impl<V> IntoIterator for Box<Slice<V>> {
    type Item = (Entity, V);
    type IntoIter = IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.into_boxed_inner().into_iter(), PhantomData)
    }
}

// ============================================================================
// Iterator types
// ============================================================================

/// An iterator over the entries of an EntityIndexMap
pub struct Iter<'a, V>(indexmap::map::Iter<'a, Entity, V>, PhantomData<EntityHash>);

impl<'a, V> Iter<'a, V> {
    /// Returns a slice of the remaining entries in the iterator
    #[inline]
    pub fn as_slice(&self) -> &Slice<V> {
        // SAFETY: The source IndexMap uses EntityHash
        unsafe { Slice::from_slice_unchecked(self.0.as_slice()) }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a Entity, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> DoubleEndedIterator for Iter<'_, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<V> ExactSizeIterator for Iter<'_, V> {}

impl<V> FusedIterator for Iter<'_, V> {}

impl<V> Clone for Iter<'_, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<V: fmt::Debug> fmt::Debug for Iter<'_, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.0).finish()
    }
}

impl<V> Default for Iter<'_, V> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// A mutable iterator over the entries of an EntityIndexMap
pub struct IterMut<'a, V>(indexmap::map::IterMut<'a, Entity, V>, PhantomData<EntityHash>);

impl<'a, V> IterMut<'a, V> {
    /// Returns a slice of the remaining entries in the iterator
    #[inline]
    pub fn as_slice(&self) -> &Slice<V> {
        // SAFETY: The source IndexMap uses EntityHash
        unsafe { Slice::from_slice_unchecked(self.0.as_slice()) }
    }

    /// Returns a mutable slice of the remaining entries in the iterator
    #[inline]
    pub fn into_slice(self) -> &'a mut Slice<V> {
        // SAFETY: The source IndexMap uses EntityHash
        unsafe { Slice::from_slice_unchecked_mut(self.0.into_slice()) }
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (&'a Entity, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> DoubleEndedIterator for IterMut<'_, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<V> ExactSizeIterator for IterMut<'_, V> {}

impl<V> FusedIterator for IterMut<'_, V> {}

impl<V: fmt::Debug> fmt::Debug for IterMut<'_, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut").field(&self.0).finish()
    }
}

impl<V> Default for IterMut<'_, V> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// An owning iterator over the entries of an EntityIndexMap
pub struct IntoIter<V>(indexmap::map::IntoIter<Entity, V>, PhantomData<EntityHash>);

impl<V> IntoIter<V> {
    /// Returns a slice of the remaining entries in the iterator
    #[inline]
    pub fn as_slice(&self) -> &Slice<V> {
        // SAFETY: The source IndexMap uses EntityHash
        unsafe { Slice::from_slice_unchecked(self.0.as_slice()) }
    }

    /// Returns a mutable slice of the remaining entries in the iterator
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut Slice<V> {
        // SAFETY: The source IndexMap uses EntityHash
        unsafe { Slice::from_slice_unchecked_mut(self.0.as_mut_slice()) }
    }
}

impl<V> Iterator for IntoIter<V> {
    type Item = (Entity, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> DoubleEndedIterator for IntoIter<V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<V> ExactSizeIterator for IntoIter<V> {}

impl<V> FusedIterator for IntoIter<V> {}

impl<V: Clone> Clone for IntoIter<V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<V: fmt::Debug> fmt::Debug for IntoIter<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.0).finish()
    }
}

impl<V> Default for IntoIter<V> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// A draining iterator over the entries of an EntityIndexMap
pub struct Drain<'a, V>(indexmap::map::Drain<'a, Entity, V>, PhantomData<EntityHash>);

impl<'a, V> Drain<'a, V> {
    /// Returns a slice of the remaining entries in the iterator
    #[inline]
    pub fn as_slice(&self) -> &Slice<V> {
        // SAFETY: The source IndexMap uses EntityHash
        unsafe { Slice::from_slice_unchecked(self.0.as_slice()) }
    }
}

impl<V> Iterator for Drain<'_, V> {
    type Item = (Entity, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> DoubleEndedIterator for Drain<'_, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<V> ExactSizeIterator for Drain<'_, V> {}

impl<V> FusedIterator for Drain<'_, V> {}

impl<V: fmt::Debug> fmt::Debug for Drain<'_, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Drain").field(&self.0).finish()
    }
}

/// An iterator over the keys of an EntityIndexMap
pub struct Keys<'a, V>(indexmap::map::Keys<'a, Entity, V>, PhantomData<EntityHash>);

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> DoubleEndedIterator for Keys<'_, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<V> ExactSizeIterator for Keys<'_, V> {}

impl<V> FusedIterator for Keys<'_, V> {}

impl<V> Clone for Keys<'_, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<V: fmt::Debug> fmt::Debug for Keys<'_, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Keys").field(&self.0).finish()
    }
}

impl<V> Default for Keys<'_, V> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

/// An owning iterator over the keys of an EntityIndexMap
pub struct IntoKeys<V>(indexmap::map::IntoKeys<Entity, V>, PhantomData<EntityHash>);

impl<V> Iterator for IntoKeys<V> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<V> DoubleEndedIterator for IntoKeys<V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<V> ExactSizeIterator for IntoKeys<V> {}

impl<V> FusedIterator for IntoKeys<V> {}

impl<V: fmt::Debug> fmt::Debug for IntoKeys<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoKeys").field(&self.0).finish()
    }
}

impl<V> Default for IntoKeys<V> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }}
