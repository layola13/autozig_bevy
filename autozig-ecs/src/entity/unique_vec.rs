//! UniqueEntityVec - Vec that guarantees entity uniqueness

use super::Entity;
use crate::entity::unique_array::{DuplicateEntityError, EntityEquivalent};
use crate::entity::unique_slice::UniqueEntitySlice;
use std::fmt;

/// UniqueEntityVec - Growable vector of unique entities
#[derive(Clone)]
pub struct UniqueEntityVec {
    entities: Vec<Entity>,
}

impl UniqueEntityVec {
    /// Creates an empty UniqueEntityVec
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Creates an empty UniqueEntityVec with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: Vec::with_capacity(capacity),
        }
    }

    /// Creates a UniqueEntityVec from a Vec, checking for duplicates
    pub fn from_vec(mut entities: Vec<Entity>) -> Result<Self, DuplicateEntityError> {
        entities.sort();
        for window in entities.windows(2) {
            if window[0] == window[1] {
                return Err(DuplicateEntityError(window[0]));
            }
        }
        Ok(Self { entities })
    }

    /// Creates a UniqueEntityVec from a slice, checking for duplicates
    pub fn from_slice(slice: &[Entity]) -> Result<Self, DuplicateEntityError> {
        Self::from_vec(slice.to_vec())
    }

    /// Pushes an entity to the vec
    pub fn push(&mut self, entity: Entity) -> Result<(), DuplicateEntityError> {
        if self.entities.contains(&entity) {
            return Err(DuplicateEntityError(entity));
        }
        self.entities.push(entity);
        Ok(())
    }

    /// Pops the last entity from the vec
    pub fn pop(&mut self) -> Option<Entity> {
        self.entities.pop()
    }

    /// Inserts an entity at the given index
    pub fn insert(&mut self, index: usize, entity: Entity) -> Result<(), DuplicateEntityError> {
        if self.entities.contains(&entity) {
            return Err(DuplicateEntityError(entity));
        }
        self.entities.insert(index, entity);
        Ok(())
    }

    /// Removes and returns the entity at the given index
    pub fn remove(&mut self, index: usize) -> Entity {
        self.entities.remove(index)
    }

    /// Removes and returns the entity at the given index by swapping with the last
    pub fn swap_remove(&mut self, index: usize) -> Entity {
        self.entities.swap_remove(index)
    }

    /// Retains only the elements specified by the predicate
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Entity) -> bool,
    {
        self.entities.retain(f);
    }

    /// Clears the vec
    pub fn clear(&mut self) {
        self.entities.clear();
    }

    /// Truncates the vec to the specified length
    pub fn truncate(&mut self, len: usize) {
        self.entities.truncate(len);
    }

    /// Returns the number of entities
    #[inline]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns true if the vec is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Returns the capacity
    #[inline]
    pub fn capacity(&self) -> usize {
        self.entities.capacity()
    }

    /// Reserves capacity for at least additional more elements
    pub fn reserve(&mut self, additional: usize) {
        self.entities.reserve(additional);
    }

    /// Shrinks the capacity as much as possible
    pub fn shrink_to_fit(&mut self) {
        self.entities.shrink_to_fit();
    }

    /// Shrinks capacity with a lower bound
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.entities.shrink_to(min_capacity);
    }

    /// Returns a reference to the underlying Vec
    #[inline]
    pub fn as_vec(&self) -> &Vec<Entity> {
        &self.entities
    }

    /// Returns a reference to the underlying slice
    #[inline]
    pub fn as_slice(&self) -> &[Entity] {
        &self.entities
    }

    /// Returns a UniqueEntitySlice
    #[inline]
    pub fn as_unique_slice(&self) -> &UniqueEntitySlice {
        unsafe { UniqueEntitySlice::new_unchecked(&self.entities) }
    }

    /// Returns a reference to an entity at the given index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&Entity> {
        self.entities.get(index)
    }

    /// Returns a mutable reference to an entity at the given index
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Entity> {
        self.entities.get_mut(index)
    }

    /// Returns the first entity
    #[inline]
    pub fn first(&self) -> Option<&Entity> {
        self.entities.first()
    }

    /// Returns the last entity
    #[inline]
    pub fn last(&self) -> Option<&Entity> {
        self.entities.last()
    }

    /// Returns true if the vec contains the entity
    pub fn contains(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns an iterator over the entities
    pub fn iter(&self) -> impl Iterator<Item = &Entity> + ExactSizeIterator + DoubleEndedIterator {
        self.entities.iter()
    }

    /// Returns a mutable iterator over the entities
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> + ExactSizeIterator + DoubleEndedIterator {
        self.entities.iter_mut()
    }

    /// Sorts the vec
    pub fn sort(&mut self)
    where
        Entity: Ord,
    {
        self.entities.sort();
    }

    /// Sorts the vec by a comparison function
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Entity, &Entity) -> std::cmp::Ordering,
    {
        self.entities.sort_by(compare);
    }

    /// Removes consecutive duplicate entities (vec must be sorted)
    pub fn dedup(&mut self) {
        self.entities.dedup();
    }

    /// Extends the vec with the contents of an iterator, checking for duplicates
    pub fn try_extend<I>(&mut self, iter: I) -> Result<(), DuplicateEntityError>
    where
        I: IntoIterator<Item = Entity>,
    {
        for entity in iter {
            self.push(entity)?;
        }
        Ok(())
    }

    /// Drains the vec in the given range
    pub fn drain<R>(&mut self, range: R) -> impl Iterator<Item = Entity> + '_
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.entities.drain(range)
    }

    /// Converts into the underlying Vec
    pub fn into_vec(self) -> Vec<Entity> {
        self.entities
    }
}

impl Default for UniqueEntityVec {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for UniqueEntityVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.entities).finish()
    }
}

impl PartialEq for UniqueEntityVec {
    fn eq(&self, other: &Self) -> bool {
        self.entities == other.entities
    }
}

impl Eq for UniqueEntityVec {}

impl std::ops::Index<usize> for UniqueEntityVec {
    type Output = Entity;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl IntoIterator for UniqueEntityVec {
    type Item = Entity;
    type IntoIter = std::vec::IntoIter<Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.entities.into_iter()
    }
}

impl<'a> IntoIterator for &'a UniqueEntityVec {
    type Item = &'a Entity;
    type IntoIter = std::slice::Iter<'a, Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.entities.iter()
    }
}

/// UniqueEntityEquivalentVec - Growable vector of unique entity equivalents
#[derive(Clone)]
pub struct UniqueEntityEquivalentVec<T: EntityEquivalent> {
    values: Vec<T>,
}

impl<T: EntityEquivalent> UniqueEntityEquivalentVec<T> {
    /// Creates an empty UniqueEntityEquivalentVec
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    /// Creates an empty UniqueEntityEquivalentVec with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    /// Pushes a value to the vec
    pub fn push(&mut self, value: T) -> Result<(), DuplicateEntityError> {
        let entity = value.entity();
        if self.values.iter().any(|v| v.entity() == entity) {
            return Err(DuplicateEntityError(entity));
        }
        self.values.push(value);
        Ok(())
    }

    /// Pops the last value from the vec
    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    /// Returns the number of values
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if the vec is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the capacity
    #[inline]
    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    /// Reserves capacity
    pub fn reserve(&mut self, additional: usize) {
        self.values.reserve(additional);
    }

    /// Shrinks the capacity as much as possible
    pub fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
    }

    /// Returns a reference to the underlying Vec
    #[inline]
    pub fn as_vec(&self) -> &Vec<T> {
        &self.values
    }

    /// Returns a reference to the underlying slice
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Returns a reference to a value at the given index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    /// Returns true if the vec contains the entity
    pub fn contains(&self, entity: Entity) -> bool {
        self.values.iter().any(|v| v.entity() == entity)
    }

    /// Returns an iterator over the values
    pub fn iter(&self) -> impl Iterator<Item = &T> + ExactSizeIterator {
        self.values.iter()
    }

    /// Clears the vec
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Converts into the underlying Vec
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

impl<T: EntityEquivalent> Default for UniqueEntityEquivalentVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: EntityEquivalent + fmt::Debug> fmt::Debug for UniqueEntityEquivalentVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
    }
}

impl<T: EntityEquivalent + PartialEq> PartialEq for UniqueEntityEquivalentVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T: EntityEquivalent + Eq> Eq for UniqueEntityEquivalentVec<T> {}

impl<T: EntityEquivalent> IntoIterator for UniqueEntityEquivalentVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}