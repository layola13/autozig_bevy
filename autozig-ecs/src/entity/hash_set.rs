//! EntityHashSet - HashSet specialized for Entity with optimized hashing

use super::{Entity, EntityHash};
use std::collections::HashSet;

/// EntityHashSet - HashSet optimized for Entity
#[derive(Clone)]
pub struct EntityHashSet {
    inner: HashSet<Entity, EntityHash>,
}

impl EntityHashSet {
    /// Creates an empty EntityHashSet
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HashSet::with_hasher(EntityHash),
        }
    }

    /// Creates an empty EntityHashSet with specified capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashSet::with_capacity_and_hasher(capacity, EntityHash),
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

    /// Returns the number of elements the set can hold without reallocating
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the set, removing all values
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

    /// Adds a value to the set
    #[inline]
    pub fn insert(&mut self, value: Entity) -> bool {
        self.inner.insert(value)
    }

    /// Removes a value from the set
    #[inline]
    pub fn remove(&mut self, value: &Entity) -> bool {
        self.inner.remove(value)
    }

    /// Removes and returns the value in the set, if any, that is equal to the given one
    #[inline]
    pub fn take(&mut self, value: &Entity) -> Option<Entity> {
        self.inner.take(value)
    }

    /// Returns true if the set contains the value
    #[inline]
    pub fn contains(&self, value: &Entity) -> bool {
        self.inner.contains(value)
    }

    /// Returns a reference to the value in the set, if any
    #[inline]
    pub fn get(&self, value: &Entity) -> Option<&Entity> {
        self.inner.get(value)
    }

    /// Retains only the elements specified by the predicate
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Entity) -> bool,
    {
        self.inner.retain(f);
    }

    /// An iterator visiting all elements
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    /// Clears the set, returning all elements in an iterator
    #[inline]
    pub fn drain(&mut self) -> Drain<'_> {
        Drain {
            inner: self.inner.drain(),
        }
    }

    /// Visits the values representing the difference
    #[inline]
    pub fn difference<'a>(&'a self, other: &'a EntityHashSet) -> Difference<'a> {
        Difference {
            inner: self.inner.difference(&other.inner),
        }
    }

    /// Visits the values representing the symmetric difference
    #[inline]
    pub fn symmetric_difference<'a>(&'a self, other: &'a EntityHashSet) -> SymmetricDifference<'a> {
        SymmetricDifference {
            inner: self.inner.symmetric_difference(&other.inner),
        }
    }

    /// Visits the values representing the intersection
    #[inline]
    pub fn intersection<'a>(&'a self, other: &'a EntityHashSet) -> Intersection<'a> {
        Intersection {
            inner: self.inner.intersection(&other.inner),
        }
    }

    /// Visits the values representing the union
    #[inline]
    pub fn union<'a>(&'a self, other: &'a EntityHashSet) -> Union<'a> {
        Union {
            inner: self.inner.union(&other.inner),
        }
    }

    /// Returns true if the set has no elements in common with other
    #[inline]
    pub fn is_disjoint(&self, other: &EntityHashSet) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    /// Returns true if the set is a subset of another
    #[inline]
    pub fn is_subset(&self, other: &EntityHashSet) -> bool {
        self.inner.is_subset(&other.inner)
    }

    /// Returns true if the set is a superset of another
    #[inline]
    pub fn is_superset(&self, other: &EntityHashSet) -> bool {
        self.inner.is_superset(&other.inner)
    }
}

impl Default for EntityHashSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::iter::FromIterator<Entity> for EntityHashSet {
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl Extend<Entity> for EntityHashSet {
    fn extend<T: IntoIterator<Item = Entity>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

impl<'a> IntoIterator for &'a EntityHashSet {
    type Item = &'a Entity;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for EntityHashSet {
    type Item = Entity;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

impl std::fmt::Debug for EntityHashSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl PartialEq for EntityHashSet {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for EntityHashSet {}

// ============================================================================
// Iterators
// ============================================================================

/// Iterator over the entities in a set
pub struct Iter<'a> {
    inner: std::collections::hash_set::Iter<'a, Entity>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Owning iterator over the entities in a set
pub struct IntoIter {
    inner: std::collections::hash_set::IntoIter<Entity>,
}

impl Iterator for IntoIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Draining iterator for EntityHashSet
pub struct Drain<'a> {
    inner: std::collections::hash_set::Drain<'a, Entity>,
}

impl<'a> Iterator for Drain<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> ExactSizeIterator for Drain<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Iterator over the difference of two sets
pub struct Difference<'a> {
    inner: std::collections::hash_set::Difference<'a, Entity, EntityHash>,
}

impl<'a> Iterator for Difference<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// Iterator over the symmetric difference of two sets
pub struct SymmetricDifference<'a> {
    inner: std::collections::hash_set::SymmetricDifference<'a, Entity, EntityHash>,
}

impl<'a> Iterator for SymmetricDifference<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// Iterator over the intersection of two sets
pub struct Intersection<'a> {
    inner: std::collections::hash_set::Intersection<'a, Entity, EntityHash>,
}

impl<'a> Iterator for Intersection<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// Iterator over the union of two sets
pub struct Union<'a> {
    inner: std::collections::hash_set::Union<'a, Entity, EntityHash>,
}

impl<'a> Iterator for Union<'a> {
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// ExtractIf iterator - extracts matching elements
pub struct ExtractIf<'a, F>
where
    F: FnMut(&Entity) -> bool,
{
    inner: std::collections::hash_set::ExtractIf<'a, Entity, F>,
}

impl<'a, F> Iterator for ExtractIf<'a, F>
where
    F: FnMut(&Entity) -> bool,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl EntityHashSet {
    /// Creates an iterator that extracts matching elements
    pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, F>
    where
        F: FnMut(&Entity) -> bool,
    {
        ExtractIf {
            inner: self.inner.extract_if(f),
        }
    }
}