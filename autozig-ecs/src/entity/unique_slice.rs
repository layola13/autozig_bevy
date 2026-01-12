//! UniqueEntitySlice - Slice that guarantees entity uniqueness

use super::Entity;
use crate::entity::unique_array::{DuplicateEntityError, EntityEquivalent};
use std::fmt;

/// UniqueEntitySlice - Dynamically-sized slice of unique entities
#[repr(transparent)]
pub struct UniqueEntitySlice {
    entities: [Entity],
}

impl UniqueEntitySlice {
    /// Creates a UniqueEntitySlice from a slice, checking for duplicates
    pub fn new(entities: &[Entity]) -> Result<&Self, DuplicateEntityError> {
        // Check for duplicates
        for (i, &entity_i) in entities.iter().enumerate() {
            for &entity_j in &entities[(i + 1)..] {
                if entity_i == entity_j {
                    return Err(DuplicateEntityError(entity_i));
                }
            }
        }
        // SAFETY: UniqueEntitySlice is repr(transparent) over [Entity]
        Ok(unsafe { &*(entities as *const [Entity] as *const Self) })
    }

    /// Creates a UniqueEntitySlice without checking for duplicates
    /// 
    /// # Safety
    /// The caller must ensure there are no duplicate entities
    pub unsafe fn new_unchecked(entities: &[Entity]) -> &Self {
        &*(entities as *const [Entity] as *const Self)
    }

    /// Creates a mutable UniqueEntitySlice from a mutable slice
    pub fn from_mut(entities: &mut [Entity]) -> Result<&mut Self, DuplicateEntityError> {
        // Check for duplicates
        for (i, &entity_i) in entities.iter().enumerate() {
            for &entity_j in &entities[(i + 1)..] {
                if entity_i == entity_j {
                    return Err(DuplicateEntityError(entity_i));
                }
            }
        }
        // SAFETY: UniqueEntitySlice is repr(transparent) over [Entity]
        Ok(unsafe { &mut *(entities as *mut [Entity] as *mut Self) })
    }

    /// Returns a reference to the underlying slice
    #[inline]
    pub fn as_slice(&self) -> &[Entity] {
        &self.entities
    }

    /// Returns a mutable reference to the underlying slice
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Entity] {
        &mut self.entities
    }

    /// Returns the number of entities
    #[inline]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns true if the slice is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
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

    /// Returns a reference to an entity at the given index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&Entity> {
        self.entities.get(index)
    }

    /// Returns true if the slice contains the entity
    pub fn contains(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }

    /// Returns an iterator over the entities
    pub fn iter(&self) -> impl Iterator<Item = &Entity> + ExactSizeIterator + DoubleEndedIterator {
        self.entities.iter()
    }

    /// Returns an iterator over windows of N entities
    pub fn windows(&self, size: usize) -> impl Iterator<Item = &Self> {
        self.entities.windows(size).map(|w| unsafe { Self::new_unchecked(w) })
    }

    /// Returns an iterator over chunks of N entities
    pub fn chunks(&self, chunk_size: usize) -> impl Iterator<Item = &Self> {
        self.entities.chunks(chunk_size).map(|c| unsafe { Self::new_unchecked(c) })
    }

    /// Divides one slice into two at an index
    pub fn split_at(&self, mid: usize) -> (&Self, &Self) {
        let (left, right) = self.entities.split_at(mid);
        unsafe { (Self::new_unchecked(left), Self::new_unchecked(right)) }
    }

    /// Returns the first entity and the rest of the slice
    pub fn split_first(&self) -> Option<(&Entity, &Self)> {
        self.entities.split_first().map(|(first, rest)| {
            (first, unsafe { Self::new_unchecked(rest) })
        })
    }

    /// Returns the last entity and the rest of the slice
    pub fn split_last(&self) -> Option<(&Entity, &Self)> {
        self.entities.split_last().map(|(last, rest)| {
            (last, unsafe { Self::new_unchecked(rest) })
        })
    }

    /// Binary search for an entity (slice must be sorted)
    pub fn binary_search(&self, entity: &Entity) -> Result<usize, usize> {
        self.entities.binary_search(entity)
    }

    /// Binary search by a key
    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&Entity) -> B,
        B: Ord,
    {
        self.entities.binary_search_by_key(b, f)
    }

    /// Sorts the slice
    pub fn sort(&mut self)
    where
        Entity: Ord,
    {
        self.entities.sort();
    }

    /// Sorts the slice by a comparison function
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Entity, &Entity) -> std::cmp::Ordering,
    {
        self.entities.sort_by(compare);
    }

    /// Reverses the order of entities in the slice
    pub fn reverse(&mut self) {
        self.entities.reverse();
    }

    /// Converts a slice of entities to a UniqueEntitySlice, sorting and removing duplicates
    pub fn sort_and_dedup(entities: &mut [Entity]) -> &mut Self {
        if entities.is_empty() {
            return unsafe { &mut *(entities as *mut [Entity] as *mut Self) };
        }
        
        entities.sort();
        
        // Manual deduplication
        let mut write_index = 1;
        for read_index in 1..entities.len() {
            if entities[read_index] != entities[write_index - 1] {
                entities[write_index] = entities[read_index];
                write_index += 1;
            }
        }
        
        let deduplicated = &mut entities[..write_index];
        unsafe { &mut *(deduplicated as *mut [Entity] as *mut Self) }
    }
}

impl fmt::Debug for UniqueEntitySlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.entities).finish()
    }
}

impl PartialEq for UniqueEntitySlice {
    fn eq(&self, other: &Self) -> bool {
        self.entities == other.entities
    }
}

impl Eq for UniqueEntitySlice {}

impl std::ops::Index<usize> for UniqueEntitySlice {
    type Output = Entity;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

impl<'a> IntoIterator for &'a UniqueEntitySlice {
    type Item = &'a Entity;
    type IntoIter = std::slice::Iter<'a, Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.entities.iter()
    }
}

/// UniqueEntityEquivalentSlice - Dynamically-sized slice of unique entity equivalents
#[repr(transparent)]
pub struct UniqueEntityEquivalentSlice<T: EntityEquivalent> {
    values: [T],
}

impl<T: EntityEquivalent> UniqueEntityEquivalentSlice<T> {
    /// Creates a UniqueEntityEquivalentSlice from a slice, checking for duplicates
    pub fn new(values: &[T]) -> Result<&Self, DuplicateEntityError> {
        // Check for duplicates
        for (i, value_i) in values.iter().enumerate() {
            for value_j in &values[(i + 1)..] {
                if value_i.entity() == value_j.entity() {
                    return Err(DuplicateEntityError(value_i.entity()));
                }
            }
        }
        // SAFETY: UniqueEntityEquivalentSlice is repr(transparent) over [T]
        Ok(unsafe { &*(values as *const [T] as *const Self) })
    }

    /// Creates a UniqueEntityEquivalentSlice without checking for duplicates
    /// 
    /// # Safety
    /// The caller must ensure there are no duplicate entities
    pub unsafe fn new_unchecked(values: &[T]) -> &Self {
        &*(values as *const [T] as *const Self)
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

    /// Returns true if the slice contains the entity
    pub fn contains(&self, entity: Entity) -> bool {
        self.values.iter().any(|v| v.entity() == entity)
    }

    /// Returns an iterator over the values
    pub fn iter(&self) -> impl Iterator<Item = &T> + ExactSizeIterator + DoubleEndedIterator {
        self.values.iter()
    }

    /// Returns an iterator over windows of N values
    pub fn windows(&self, size: usize) -> impl Iterator<Item = &Self> {
        self.values.windows(size).map(|w| unsafe { Self::new_unchecked(w) })
    }

    /// Returns an iterator over chunks of N values
    pub fn chunks(&self, chunk_size: usize) -> impl Iterator<Item = &Self> {
        self.values.chunks(chunk_size).map(|c| unsafe { Self::new_unchecked(c) })
    }

    /// Divides one slice into two at an index
    pub fn split_at(&self, mid: usize) -> (&Self, &Self) {
        let (left, right) = self.values.split_at(mid);
        unsafe { (Self::new_unchecked(left), Self::new_unchecked(right)) }
    }
}

impl<T: EntityEquivalent + fmt::Debug> fmt::Debug for UniqueEntityEquivalentSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
    }
}

impl<T: EntityEquivalent + PartialEq> PartialEq for UniqueEntityEquivalentSlice<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T: EntityEquivalent + Eq> Eq for UniqueEntityEquivalentSlice<T> {}

impl<T: EntityEquivalent> std::ops::Index<usize> for UniqueEntityEquivalentSlice<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<'a, T: EntityEquivalent> IntoIterator for &'a UniqueEntityEquivalentSlice<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}