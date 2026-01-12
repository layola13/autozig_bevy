//! EntityHashMap - HashMap specialized for Entity keys with optimized hashing

use super::{Entity, EntityHash};
use std::collections::HashMap;
use std::hash::BuildHasher;

/// EntityHashMap - HashMap optimized for Entity keys
#[derive(Clone)]
pub struct EntityHashMap<V> {
    inner: HashMap<Entity, V, EntityHash>,
}

impl<V> EntityHashMap<V> {
    /// Creates an empty EntityHashMap
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashMap::with_hasher(EntityHash),
        }
    }

    /// Creates an empty EntityHashMap with specified capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity_and_hasher(capacity, EntityHash),
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

    /// Returns the number of elements the map can hold without reallocating
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the map, removing all key-value pairs
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
    pub fn insert(&mut self, k: Entity, v: V) -> Option<V> {
        self.inner.insert(k, v)
    }

    /// Removes a key from the map, returning the value if it existed
    #[inline]
    pub fn remove(&mut self, k: &Entity) -> Option<V> {
        self.inner.remove(k)
    }

    /// Removes a key from the map, returning the stored key and value if it existed
    #[inline]
    pub fn remove_entry(&mut self, k: &Entity) -> Option<(Entity, V)> {
        self.inner.remove_entry(k)
    }

    /// Returns a reference to the value corresponding to the key
    #[inline]
    pub fn get(&self, k: &Entity) -> Option<&V> {
        self.inner.get(k)
    }

    /// Returns a mutable reference to the value corresponding to the key
    #[inline]
    pub fn get_mut(&mut self, k: &Entity) -> Option<&mut V> {
        self.inner.get_mut(k)
    }

    /// Returns the key-value pair corresponding to the supplied key
    #[inline]
    pub fn get_key_value(&self, k: &Entity) -> Option<(&Entity, &V)> {
        self.inner.get_key_value(k)
    }

    /// Returns true if the map contains the key
    #[inline]
    pub fn contains_key(&self, k: &Entity) -> bool {
        self.inner.contains_key(k)
    }

    /// Gets the given key's corresponding entry for in-place manipulation
    #[inline]
    pub fn entry(&mut self, key: Entity) -> std::collections::hash_map::Entry<'_, Entity, V> {
        self.inner.entry(key)
    }

    /// An iterator visiting all keys
    #[inline]
    pub fn keys(&self) -> Keys<'_, V> {
        Keys {
            inner: self.inner.keys(),
        }
    }

    /// An iterator visiting all values
    #[inline]
    pub fn values(&self) -> Values<'_, V> {
        Values {
            inner: self.inner.values(),
        }
    }

    /// An iterator visiting all values mutably
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, V> {
        ValuesMut {
            inner: self.inner.values_mut(),
        }
    }

    /// An iterator visiting all key-value pairs
    #[inline]
    pub fn iter(&self) -> Iter<'_, V> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    /// An iterator visiting all key-value pairs with mutable references to values
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        IterMut {
            inner: self.inner.iter_mut(),
        }
    }

    /// Retains only the elements specified by the predicate
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Entity, &mut V) -> bool,
    {
        self.inner.retain(f);
    }

    /// Clears the map, returning all key-value pairs as an iterator
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, V> {
        Drain {
            inner: self.inner.drain(),
        }
    }

    /// Creates a consuming iterator
    #[inline]
    pub fn into_keys(self) -> IntoKeys<V> {
        IntoKeys {
            inner: self.inner.into_keys(),
        }
    }

    /// Creates a consuming iterator visiting all values
    #[inline]
    pub fn into_values(self) -> IntoValues<V> {
        IntoValues {
            inner: self.inner.into_values(),
        }
    }
}

impl<V> Default for EntityHashMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> std::iter::FromIterator<(Entity, V)> for EntityHashMap<V> {
    fn from_iter<T: IntoIterator<Item = (Entity, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        map.extend(iter);
        map
    }
}

impl<V> Extend<(Entity, V)> for EntityHashMap<V> {
    fn extend<T: IntoIterator<Item = (Entity, V)>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

impl<'a, V> IntoIterator for &'a EntityHashMap<V> {
    type Item = (&'a Entity, &'a V);
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut EntityHashMap<V> {
    type Item = (&'a Entity, &'a mut V);
    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V> IntoIterator for EntityHashMap<V> {
    type Item = (Entity, V);
    type IntoIter = IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

impl<V: std::fmt::Debug> std::fmt::Debug for EntityHashMap<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<V: PartialEq> PartialEq for EntityHashMap<V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<V: Eq> Eq for EntityHashMap<V> {}

// ============================================================================
// Iterators
// ============================================================================

/// Iterator over keys
pub struct Keys<'a, V> {
    inner: std::collections::hash_map::Keys<'a, Entity, V>,
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for Keys<'a, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Iterator over values
pub struct Values<'a, V> {
    inner: std::collections::hash_map::Values<'a, Entity, V>,
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for Values<'a, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Iterator over mutable values
pub struct ValuesMut<'a, V> {
    inner: std::collections::hash_map::ValuesMut<'a, Entity, V>,
}

impl<'a, V> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for ValuesMut<'a, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Iterator over key-value pairs
pub struct Iter<'a, V> {
    inner: std::collections::hash_map::Iter<'a, Entity, V>,
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a Entity, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for Iter<'a, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Iterator over mutable key-value pairs
pub struct IterMut<'a, V> {
    inner: std::collections::hash_map::IterMut<'a, Entity, V>,
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (&'a Entity, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for IterMut<'a, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Owning iterator over key-value pairs
pub struct IntoIter<V> {
    inner: std::collections::hash_map::IntoIter<Entity, V>,
}

impl<V> Iterator for IntoIter<V> {
    type Item = (Entity, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<V> ExactSizeIterator for IntoIter<V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Owning iterator over keys
pub struct IntoKeys<V> {
    inner: std::collections::hash_map::IntoKeys<Entity, V>,
}

impl<V> Iterator for IntoKeys<V> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<V> ExactSizeIterator for IntoKeys<V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Owning iterator over values
pub struct IntoValues<V> {
    inner: std::collections::hash_map::IntoValues<Entity, V>,
}

impl<V> Iterator for IntoValues<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<V> ExactSizeIterator for IntoValues<V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Draining iterator
pub struct Drain<'a, V> {
    inner: std::collections::hash_map::Drain<'a, Entity, V>,
}

impl<'a, V> Iterator for Drain<'a, V> {
    type Item = (Entity, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, V> ExactSizeIterator for Drain<'a, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}