//! EntityIndexSet - IndexSet specialized for Entity

use super::{Entity, EntityHash};
use indexmap::IndexSet;

/// EntityIndexSet - IndexSet optimized for Entity with preserved insertion order
#[derive(Clone)]
pub struct EntityIndexSet {
    inner: IndexSet<Entity, EntityHash>,
}

impl EntityIndexSet {
    /// Creates an empty EntityIndexSet
    #[inline]
    pub fn new() -> Self {
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

    /// An iterator visiting all values
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Entity> + ExactSizeIterator {
        self.inner.iter()
    }

    /// Clears the set, returning all values as an iterator
    #[inline]
    pub fn drain<R>(&mut self, range: R) -> impl Iterator<Item = Entity> + '_
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.inner.drain(range)
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
    type IntoIter = indexmap::set::IntoIter<Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a EntityIndexSet {
    type Item = &'a Entity;
    type IntoIter = indexmap::set::Iter<'a, Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl std::fmt::Debug for EntityIndexSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl PartialEq for EntityIndexSet {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for EntityIndexSet {}