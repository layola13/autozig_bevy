//! UniqueEntityArray - Array that guarantees entity uniqueness

use super::Entity;
use std::fmt;

/// Error for duplicate entities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DuplicateEntityError(pub Entity);

impl fmt::Display for DuplicateEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Duplicate entity: {:?}", self.0)
    }
}

impl std::error::Error for DuplicateEntityError {}

/// UniqueEntityArray - Fixed-size array of unique entities
#[derive(Clone)]
pub struct UniqueEntityArray<const N: usize> {
    entities: [Entity; N],
}

impl<const N: usize> UniqueEntityArray<N> {
    /// Creates a new UniqueEntityArray from an array, checking for duplicates
    pub fn new(entities: [Entity; N]) -> Result<Self, DuplicateEntityError> {
        // Check for duplicates
        for i in 0..N {
            for j in (i + 1)..N {
                if entities[i] == entities[j] {
                    return Err(DuplicateEntityError(entities[i]));
                }
            }
        }
        Ok(Self { entities })
    }

    /// Creates a new UniqueEntityArray without checking for duplicates
    /// 
    /// # Safety
    /// The caller must ensure there are no duplicate entities
    pub unsafe fn new_unchecked(entities: [Entity; N]) -> Self {
        Self { entities }
    }

    /// Returns the number of entities
    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns true if the array is empty
    #[inline]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Returns a reference to an entity at the given index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&Entity> {
        self.entities.get(index)
    }

    /// Returns true if the array contains the entity
    pub fn contains(&self, entity: &Entity) -> bool {
        self.entities.iter().any(|e| e == entity)
    }

    /// Returns an iterator over the entities
    pub fn iter(&self) -> impl Iterator<Item = &Entity> + ExactSizeIterator {
        self.entities.iter()
    }

    /// Returns a reference to the underlying array
    #[inline]
    pub fn as_array(&self) -> &[Entity; N] {
        &self.entities
    }

    /// Converts into the underlying array
    #[inline]
    pub fn into_array(self) -> [Entity; N] {
        self.entities
    }
}

impl<const N: usize> fmt::Debug for UniqueEntityArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.entities).finish()
    }
}

impl<const N: usize> PartialEq for UniqueEntityArray<N> {
    fn eq(&self, other: &Self) -> bool {
        self.entities == other.entities
    }
}

impl<const N: usize> Eq for UniqueEntityArray<N> {}

impl<const N: usize> std::ops::Index<usize> for UniqueEntityArray<N> {
    type Output = Entity;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entities[index]
    }
}

/// Trait for types that can be treated as Entity equivalents
pub trait EntityEquivalent {
    fn entity(&self) -> Entity;
}

impl EntityEquivalent for Entity {
    fn entity(&self) -> Entity {
        *self
    }
}

/// UniqueEntityEquivalentArray - Fixed-size array of unique entity equivalents
#[derive(Clone)]
pub struct UniqueEntityEquivalentArray<T: EntityEquivalent, const N: usize> {
    values: [T; N],
}

impl<T: EntityEquivalent + Clone, const N: usize> UniqueEntityEquivalentArray<T, N> {
    /// Creates a new UniqueEntityEquivalentArray from an array, checking for duplicates
    pub fn new(values: [T; N]) -> Result<Self, DuplicateEntityError> {
        // Check for duplicates
        for i in 0..N {
            for j in (i + 1)..N {
                if values[i].entity() == values[j].entity() {
                    return Err(DuplicateEntityError(values[i].entity()));
                }
            }
        }
        Ok(Self { values })
    }

    /// Creates a new UniqueEntityEquivalentArray without checking for duplicates
    /// 
    /// # Safety
    /// The caller must ensure there are no duplicate entities
    pub unsafe fn new_unchecked(values: [T; N]) -> Self {
        Self { values }
    }

    /// Returns the number of values
    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns true if the array is empty
    #[inline]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Returns a reference to a value at the given index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    /// Returns true if the array contains the entity
    pub fn contains(&self, entity: Entity) -> bool {
        self.values.iter().any(|v| v.entity() == entity)
    }

    /// Returns an iterator over the values
    pub fn iter(&self) -> impl Iterator<Item = &T> + ExactSizeIterator {
        self.values.iter()
    }

    /// Returns a reference to the underlying array
    #[inline]
    pub fn as_array(&self) -> &[T; N] {
        &self.values
    }

    /// Converts into the underlying array
    #[inline]
    pub fn into_array(self) -> [T; N] {
        self.values
    }
}

impl<T: EntityEquivalent + fmt::Debug, const N: usize> fmt::Debug for UniqueEntityEquivalentArray<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
    }
}

impl<T: EntityEquivalent + PartialEq, const N: usize> PartialEq for UniqueEntityEquivalentArray<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T: EntityEquivalent + Eq, const N: usize> Eq for UniqueEntityEquivalentArray<T, N> {}

impl<T: EntityEquivalent, const N: usize> std::ops::Index<usize> for UniqueEntityEquivalentArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}