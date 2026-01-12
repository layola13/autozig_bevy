//! EntityIndexMap - IndexMap specialized for Entity keys

use super::{Entity, EntityHash};
use indexmap::IndexMap;

/// EntityIndexMap - IndexMap optimized for Entity keys with preserved insertion order
#[derive(Clone)]
pub struct EntityIndexMap<V> {
    inner: IndexMap<Entity, V, EntityHash>,
}

impl<V> EntityIndexMap<V> {
    /// Creates an empty EntityIndexMap
    #[inline]
    pub fn new() -> Self {
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

    /// Gets the last entry
    #[inline]
    pub fn last(&self) -> Option<(&Entity, &V)> {
        self.inner.last()
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

    /// An iterator visiting all keys
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &Entity> + ExactSizeIterator {
        self.inner.keys()
    }

    /// An iterator visiting all values
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> + ExactSizeIterator {
        self.inner.values()
    }

    /// An iterator visiting all values mutably
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> + ExactSizeIterator {
        self.inner.values_mut()
    }

    /// An iterator visiting all key-value pairs
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &V)> + ExactSizeIterator {
        self.inner.iter()
    }

    /// An iterator visiting all key-value pairs with mutable values
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Entity, &mut V)> + ExactSizeIterator {
        self.inner.iter_mut()
    }

    /// Clears the map, returning all key-value pairs as an iterator
    #[inline]
    pub fn drain<R>(&mut self, range: R) -> impl Iterator<Item = (Entity, V)> + '_
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.inner.drain(range)
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
    type IntoIter = indexmap::map::IntoIter<Entity, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, V> IntoIterator for &'a EntityIndexMap<V> {
    type Item = (&'a Entity, &'a V);
    type IntoIter = indexmap::map::Iter<'a, Entity, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut EntityIndexMap<V> {
    type Item = (&'a Entity, &'a mut V);
    type IntoIter = indexmap::map::IterMut<'a, Entity, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl<V: std::fmt::Debug> std::fmt::Debug for EntityIndexMap<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<V: PartialEq> PartialEq for EntityIndexMap<V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<V: Eq> Eq for EntityIndexMap<V> {}