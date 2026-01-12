//! Query iterator module
//! 查询迭代器模块

use crate::{
    entity::Entity,
    query::{QueryData, QueryFilter},
};
use std::marker::PhantomData;

/// Query iterator - iterates over query results
pub struct QueryIter<'w, 's, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entities: Vec<Entity>,
    start: usize,
    end: usize,
}

impl<'w, 's, Q: QueryData, F: QueryFilter> QueryIter<'w, 's, Q, F> {
    pub fn new(entities: Vec<Entity>) -> Self {
        let len = entities.len();
        Self {
            _phantom: PhantomData,
            entities,
            start: 0,
            end: len,
        }
    }
    
    /// Get remaining entities count
    pub fn remaining(&self) -> usize {
        if self.end > self.start {
            self.end - self.start
        } else {
            0
        }
    }
    
    /// Fetch next entity
    pub fn fetch_next(&mut self) -> Option<Entity> {
        if self.start < self.end {
            let entity = self.entities[self.start];
            self.start += 1;
            Some(entity)
        } else {
            None
        }
    }
    
    /// Fetch next entity from back
    pub fn fetch_next_back(&mut self) -> Option<Entity> {
        if self.start < self.end {
            self.end -= 1;
            let entity = self.entities[self.end];
            Some(entity)
        } else {
            None
        }
    }
}

impl<'w, 's, Q: QueryData, F: QueryFilter> Iterator for QueryIter<'w, 's, Q, F> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.fetch_next()
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }
}

impl<'w, 's, Q: QueryData, F: QueryFilter> DoubleEndedIterator for QueryIter<'w, 's, Q, F> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.fetch_next_back()
    }
}

impl<'w, 's, Q: QueryData, F: QueryFilter> ExactSizeIterator for QueryIter<'w, 's, Q, F> {
    fn len(&self) -> usize {
        self.remaining()
    }
}

/// QueryCombinationIter - iterates over combinations of query results
pub struct QueryCombinationIter<'w, 's, Q: QueryData, F: QueryFilter, const N: usize> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entities: Vec<Entity>,
    indices: [usize; N],
    done: bool,
}

impl<'w, 's, Q: QueryData, F: QueryFilter, const N: usize> QueryCombinationIter<'w, 's, Q, F, N> {
    pub fn new(entities: Vec<Entity>) -> Self {
        let mut indices = [0; N];
        for i in 0..N.min(entities.len()) {
            indices[i] = i;
        }
        Self {
            _phantom: PhantomData,
            entities,
            indices,
            done: false,
        }
    }
}

impl<'w, 's, Q: QueryData, F: QueryFilter, const N: usize> Iterator for QueryCombinationIter<'w, 's, Q, F, N> {
    type Item = [Entity; N];
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.entities.len() < N {
            return None;
        }
        
        let mut result = [Entity::from_raw(0); N];
        for (i, &idx) in self.indices.iter().enumerate() {
            result[i] = self.entities[idx];
        }
        
        // Advance to next combination
        self.done = true; // Simplified - would implement proper combination logic
        
        Some(result)
    }
}

/// QueryManyIter - iterates over many entities
pub struct QueryManyIter<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entity_iter: I,
}

impl<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> QueryManyIter<'w, 's, Q, F, I> {
    pub fn new(entity_iter: I) -> Self {
        Self {
            _phantom: PhantomData,
            entity_iter,
        }
    }
}

impl<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> Iterator for QueryManyIter<'w, 's, Q, F, I> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.entity_iter.next()
    }
}

/// QueryManyUniqueIter - iterates over many unique entities
pub struct QueryManyUniqueIter<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entity_iter: I,
    seen: Vec<Entity>,
}

impl<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> QueryManyUniqueIter<'w, 's, Q, F, I> {
    pub fn new(entity_iter: I) -> Self {
        Self {
            _phantom: PhantomData,
            entity_iter,
            seen: Vec::new(),
        }
    }
}

impl<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> Iterator for QueryManyUniqueIter<'w, 's, Q, F, I> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entity) = self.entity_iter.next() {
            if !self.seen.contains(&entity) {
                self.seen.push(entity);
                return Some(entity);
            }
        }
        None
    }
}

/// QuerySortedIter - iterates over sorted query results
pub struct QuerySortedIter<'w, 's, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entities: Vec<Entity>,
    index: usize,
}

impl<'w, 's, Q: QueryData, F: QueryFilter> QuerySortedIter<'w, 's, Q, F> {
    pub fn new(mut entities: Vec<Entity>) -> Self {
        // Sort entities by ID
        entities.sort_by_key(|e| e.index());
        Self {
            _phantom: PhantomData,
            entities,
            index: 0,
        }
    }
    
    /// Sort by a custom key function
    pub fn sort_by<K, G>(mut entities: Vec<Entity>, mut f: G) -> Self
    where
        K: Ord,
        G: FnMut(&Entity) -> K,
    {
        entities.sort_by_key(|e| f(e));
        Self {
            _phantom: PhantomData,
            entities,
            index: 0,
        }
    }
    
    /// Sort by a custom comparator
    pub fn sort_by_cmp<H>(mut entities: Vec<Entity>, f: H) -> Self
    where
        H: FnMut(&Entity, &Entity) -> std::cmp::Ordering,
    {
        entities.sort_by(f);
        Self {
            _phantom: PhantomData,
            entities,
            index: 0,
        }
    }
}

impl<'w, 's, Q: QueryData, F: QueryFilter> Iterator for QuerySortedIter<'w, 's, Q, F> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Component;

    #[derive(Debug, Clone, Copy)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[derive(Debug, Clone, Copy)]
    struct Velocity { x: f32, y: f32 }
    impl Component for Velocity {}

    #[test]
    fn test_query_iter() {
        let entities = vec![Entity::from_raw(0), Entity::from_raw(1), Entity::from_raw(2)];
        let mut iter = QueryIter::<(), ()>::new(entities.clone());
        
        assert_eq!(iter.remaining(), 3);
        assert_eq!(iter.next(), Some(entities[0]));
        assert_eq!(iter.remaining(), 2);
    }

    #[test]
    fn test_query_iter_double_ended() {
        let entities = vec![Entity::from_raw(0), Entity::from_raw(1), Entity::from_raw(2)];
        let mut iter = QueryIter::<(), ()>::new(entities.clone());
        
        assert_eq!(iter.next(), Some(entities[0]));
        assert_eq!(iter.next_back(), Some(entities[2]));
    }

    #[test]
    fn test_query_sorted_iter() {
        let entities = vec![Entity::from_raw(2), Entity::from_raw(0), Entity::from_raw(1)];
        let mut iter = QuerySortedIter::<(), ()>::new(entities);
        
        assert_eq!(iter.next(), Some(Entity::from_raw(0)));
        assert_eq!(iter.next(), Some(Entity::from_raw(1)));
        assert_eq!(iter.next(), Some(Entity::from_raw(2)));
    }
}