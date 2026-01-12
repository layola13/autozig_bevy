//! Entity set utilities and iterators

use super::Entity;

/// UniqueEntityIter - Iterator over unique entities
pub struct UniqueEntityIter {
    entities: Vec<Entity>,
    index: usize,
}

impl UniqueEntityIter {
    /// Creates a new UniqueEntityIter from a vector
    pub fn new(mut entities: Vec<Entity>) -> Self {
        entities.sort();
        entities.dedup();
        Self { entities, index: 0 }
    }

    /// Creates a new UniqueEntityIter from an iterator
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Entity>,
    {
        Self::new(iter.into_iter().collect())
    }
}

impl Iterator for UniqueEntityIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;
            Some(entity)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.entities.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for UniqueEntityIter {
    fn len(&self) -> usize {
        self.entities.len() - self.index
    }
}

impl DoubleEndedIterator for UniqueEntityIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            self.entities.pop()
        } else {
            None
        }
    }
}

/// Creates a UniqueEntityIter from an entity set iterator
pub fn from_entity_set_iterator<I>(iter: I) -> UniqueEntityIter
where
    I: IntoIterator<Item = Entity>,
{
    UniqueEntityIter::from_iter(iter)
}

/// EntitySetIterator - Generic iterator adapter for entity sets
pub struct EntitySetIterator<I> {
    inner: I,
}

impl<I> EntitySetIterator<I>
where
    I: Iterator<Item = Entity>,
{
    /// Creates a new EntitySetIterator
    pub fn new(inner: I) -> Self {
        Self { inner }
    }

    /// Collects into a UniqueEntityIter
    pub fn collect_unique(self) -> UniqueEntityIter {
        UniqueEntityIter::from_iter(self.inner)
    }
}

impl<I> Iterator for EntitySetIterator<I>
where
    I: Iterator<Item = Entity>,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}