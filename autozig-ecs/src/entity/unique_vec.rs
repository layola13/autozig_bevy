//! UniqueEntityVec - Vec that guarantees entity uniqueness

use super::Entity;
use crate::entity::unique_array::{DuplicateEntityError, EntityEquivalent};
use crate::entity::unique_slice::{UniqueEntitySlice, UniqueEntityEquivalentSlice};
use std::fmt;
use std::mem::MaybeUninit;

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

    /// Reserves capacity for exactly additional more elements
    pub fn reserve_exact(&mut self, additional: usize) {
        self.entities.reserve_exact(additional);
    }

    /// Tries to reserve capacity for at least additional more elements
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        self.entities.try_reserve(additional)
    }

    /// Tries to reserve capacity for exactly additional more elements
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        self.entities.try_reserve_exact(additional)
    }

    /// Returns a reference to the underlying Vec
    #[inline]
    pub fn as_vec(&self) -> &Vec<Entity> {
        &self.entities
    }

    /// Returns a mutable reference to the underlying Vec
    ///
    /// # Safety
    /// Caller must ensure no duplicate entities are introduced
    #[inline]
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<Entity> {
        &mut self.entities
    }

    /// Returns a reference to the underlying slice
    #[inline]
    pub fn as_slice(&self) -> &[Entity] {
        &self.entities
    }

    /// Returns a mutable slice of the entities
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Entity] {
        &mut self.entities
    }

    /// Returns a raw pointer to the vector's buffer
    #[inline]
    pub fn as_ptr(&self) -> *const Entity {
        self.entities.as_ptr()
    }

    /// Returns a mutable raw pointer to the vector's buffer
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut Entity {
        self.entities.as_mut_ptr()
    }

    /// Returns a mutable reference to the spare capacity
    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<Entity>] {
        self.entities.spare_capacity_mut()
    }

    /// Returns a UniqueEntitySlice
    #[inline]
    pub fn as_unique_slice(&self) -> &UniqueEntitySlice {
        // SAFETY: UniqueEntityVec maintains the uniqueness invariant
        unsafe { std::mem::transmute::<&[Entity], &UniqueEntitySlice>(&self.entities[..]) }
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

    /// Removes duplicate entities using a key extraction function
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut Entity) -> K,
        K: PartialEq,
    {
        self.entities.dedup_by_key(key);
    }

    /// Removes duplicate entities using a comparison function
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut Entity, &mut Entity) -> bool,
    {
        self.entities.dedup_by(same_bucket);
    }

    /// Retains only elements specified by the predicate (with mutable access)
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut Entity) -> bool,
    {
        self.entities.retain_mut(f);
    }

    /// Resizes the vec with the given generator function
    pub fn resize_with<F>(&mut self, new_len: usize, f: F) -> Result<(), DuplicateEntityError>
    where
        F: FnMut() -> Entity,
    {
        let old_len = self.len();
        if new_len <= old_len {
            self.truncate(new_len);
            Ok(())
        } else {
            let mut generator = f;
            for _ in old_len..new_len {
                self.push(generator())?;
            }
            Ok(())
        }
    }

    /// Appends all entities from another UniqueEntityVec
    pub fn append(&mut self, other: &mut Self) -> Result<(), DuplicateEntityError> {
        for entity in other.entities.drain(..) {
            self.push(entity)?;
        }
        Ok(())
    }

    /// Splits the vec at the given index
    pub fn split_off(&mut self, at: usize) -> Self {
        Self {
            entities: self.entities.split_off(at),
        }
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
    pub fn drain<R>(&mut self, range: R) -> Drain<'_>
    where
        R: std::ops::RangeBounds<usize>,
    {
        Drain {
            inner: self.entities.drain(range),
        }
    }

    /// Converts into the underlying Vec
    pub fn into_vec(self) -> Vec<Entity> {
        self.entities
    }

    /// Creates a UniqueEntityVec from a Vec without checking for duplicates
    ///
    /// # Safety
    /// Caller must ensure the Vec contains no duplicate entities
    pub unsafe fn from_vec_unchecked(entities: Vec<Entity>) -> Self {
        Self { entities }
    }

    /// Creates a UniqueEntityVec from raw parts
    ///
    /// # Safety
    /// Caller must ensure:
    /// - The pointer is valid and properly aligned
    /// - The length and capacity are correct
    /// - No duplicate entities exist in the data
    pub unsafe fn from_raw_parts(ptr: *mut Entity, length: usize, capacity: usize) -> Self {
        Self {
            entities: Vec::from_raw_parts(ptr, length, capacity),
        }
    }

    /// Converts into a boxed slice
    pub fn into_boxed_slice(self) -> Box<[Entity]> {
        self.entities.into_boxed_slice()
    }

    /// Leaks the vec and returns a mutable reference with static lifetime
    pub fn leak<'a>(self) -> &'a mut [Entity] {
        self.entities.leak()
    }

    /// Forces the length of the vector to new_len
    ///
    /// # Safety
    /// - new_len must be less than or equal to capacity()
    /// - The elements at old_len..new_len must be initialized
    /// - No duplicate entities may be introduced
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.entities.set_len(new_len);
    }

    /// Replaces a range with an iterator and returns the removed elements
    pub fn splice<'a, R, I>(&'a mut self, range: R, replace_with: I) -> impl Iterator<Item = Entity> + DoubleEndedIterator + ExactSizeIterator + 'a
    where
        R: std::ops::RangeBounds<usize>,
        I: IntoIterator<Item = Entity>,
        <I as IntoIterator>::IntoIter: 'a,
    {
        self.entities.splice(range, replace_with)
    }
}

/// Iterator returned by [`UniqueEntityVec::drain`]
pub struct Drain<'a> {
    inner: std::vec::Drain<'a, Entity>,
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

impl<'a> DoubleEndedIterator for Drain<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'a> ExactSizeIterator for Drain<'a> {}

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

    /// Shrinks capacity with a lower bound
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.values.shrink_to(min_capacity);
    }

    /// Reserves capacity for exactly additional more elements
    pub fn reserve_exact(&mut self, additional: usize) {
        self.values.reserve_exact(additional);
    }

    /// Tries to reserve capacity for at least additional more elements
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        self.values.try_reserve(additional)
    }

    /// Tries to reserve capacity for exactly additional more elements
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        self.values.try_reserve_exact(additional)
    }

    /// Returns a reference to the underlying Vec
    #[inline]
    pub fn as_vec(&self) -> &Vec<T> {
        &self.values
    }

    /// Returns a mutable reference to the underlying Vec
    ///
    /// # Safety
    /// Caller must ensure no duplicate entities are introduced
    #[inline]
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.values
    }

    /// Returns a reference to the underlying slice
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Returns a mutable slice of the values
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.values
    }

    /// Returns a raw pointer to the vector's buffer
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.values.as_ptr()
    }

    /// Returns a mutable raw pointer to the vector's buffer
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.values.as_mut_ptr()
    }

    /// Returns a mutable reference to the spare capacity
    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        self.values.spare_capacity_mut()
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

    /// Returns true if the vec contains the entity
    pub fn contains(&self, entity: Entity) -> bool {
        self.values.iter().any(|v| v.entity() == entity)
    }

    /// Returns an iterator over the values
    pub fn iter(&self) -> impl Iterator<Item = &T> + ExactSizeIterator + DoubleEndedIterator {
        self.values.iter()
    }

    /// Returns a mutable iterator over the values
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + ExactSizeIterator + DoubleEndedIterator {
        self.values.iter_mut()
    }

    /// Clears the vec
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Truncates the vec to the specified length
    pub fn truncate(&mut self, len: usize) {
        self.values.truncate(len);
    }

    /// Retains only the elements specified by the predicate
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.values.retain(|v| f(v));
    }

    /// Retains only elements specified by the predicate (with mutable access)
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.values.retain_mut(|v| f(v));
    }

    /// Inserts a value at the given index
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), DuplicateEntityError> {
        let entity = value.entity();
        if self.values.iter().any(|v| v.entity() == entity) {
            return Err(DuplicateEntityError(entity));
        }
        self.values.insert(index, value);
        Ok(())
    }

    /// Removes and returns the value at the given index
    pub fn remove(&mut self, index: usize) -> T {
        self.values.remove(index)
    }

    /// Removes and returns the value at the given index by swapping with the last
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.values.swap_remove(index)
    }

    /// Appends all values from another UniqueEntityEquivalentVec
    pub fn append(&mut self, other: &mut Self) -> Result<(), DuplicateEntityError> {
        for value in other.values.drain(..) {
            self.push(value)?;
        }
        Ok(())
    }

    /// Splits the vec at the given index
    pub fn split_off(&mut self, at: usize) -> Self {
        Self {
            values: self.values.split_off(at),
        }
    }

    /// Drains the vec in the given range
    pub fn drain<R>(&mut self, range: R) -> std::vec::Drain<'_, T>
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.values.drain(range)
    }

    /// Extends the vec with the contents of an iterator, checking for duplicates
    pub fn try_extend<I>(&mut self, iter: I) -> Result<(), DuplicateEntityError>
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            self.push(value)?;
        }
        Ok(())
    }

    /// Converts into the underlying Vec
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
    
    /// Creates from Vec without checking for duplicates
    ///
    /// # Safety
    /// Caller must ensure no duplicate entities
    pub unsafe fn from_vec_unchecked(values: Vec<T>) -> Self {
        Self { values }
    }
    
    /// Converts into boxed slice
    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.values.into_boxed_slice()
    }

    /// Leaks the vec and returns a mutable reference with static lifetime
    pub fn leak<'a>(self) -> &'a mut [T] {
        self.values.leak()
    }

    /// Creates a UniqueEntityEquivalentVec from raw parts
    ///
    /// # Safety
    /// Caller must ensure:
    /// - The pointer is valid and properly aligned
    /// - The length and capacity are correct
    /// - No duplicate entities exist in the data
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        Self {
            values: Vec::from_raw_parts(ptr, length, capacity),
        }
    }

    /// Forces the length of the vector to new_len
    ///
    /// # Safety
    /// - new_len must be less than or equal to capacity()
    /// - The elements at old_len..new_len must be initialized
    /// - No duplicate entities may be introduced
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.values.set_len(new_len);
    }

    /// Replaces a range with an iterator and returns the removed elements
    pub fn splice<'a, R, I>(&'a mut self, range: R, replace_with: I) -> impl Iterator<Item = T> + DoubleEndedIterator + ExactSizeIterator + 'a
    where
        R: std::ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
        <I as IntoIterator>::IntoIter: 'a,
    {
        self.values.splice(range, replace_with)
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

impl<T: EntityEquivalent> std::ops::Index<usize> for UniqueEntityEquivalentVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T: EntityEquivalent> IntoIterator for UniqueEntityEquivalentVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T: EntityEquivalent> IntoIterator for &'a UniqueEntityEquivalentVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<T: EntityEquivalent> std::borrow::Borrow<UniqueEntityEquivalentSlice<T>> for UniqueEntityEquivalentVec<T> {
    fn borrow(&self) -> &UniqueEntityEquivalentSlice<T> {
        unsafe { UniqueEntityEquivalentSlice::from_slice_unchecked(&self.values) }
    }
}