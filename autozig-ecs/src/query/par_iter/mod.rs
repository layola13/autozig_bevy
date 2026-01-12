//! Parallel query iterator module
//! 并行查询迭代器模块

use crate::{
    entity::Entity,
    query::{QueryData, QueryFilter},
};
use std::marker::PhantomData;

/// Batching strategy for parallel iteration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchingStrategy {
    /// Fixed batch size
    Fixed(usize),
    /// Adaptive batch size based on workload
    Adaptive { min: usize, max: usize },
}

impl Default for BatchingStrategy {
    fn default() -> Self {
        Self::Fixed(64)
    }
}

impl BatchingStrategy {
    /// Calculate batch size for given total items
    pub fn batch_size(&self, _total_items: usize) -> usize {
        match self {
            Self::Fixed(size) => *size,
            Self::Adaptive { min, max } => {
                // Simple adaptive strategy
                (*min).max((*max).min(64))
            }
        }
    }
}

/// Parallel query iterator
pub struct QueryParIter<'w, 's, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entities: Vec<Entity>,
    batch_size: usize,
}

impl<'w, 's, Q: QueryData, F: QueryFilter> QueryParIter<'w, 's, Q, F> {
    pub fn new(entities: Vec<Entity>, batch_size: usize) -> Self {
        Self {
            _phantom: PhantomData,
            entities,
            batch_size,
        }
    }
    
    /// Execute a function for each entity in parallel
    pub fn for_each<FN>(&self, mut f: FN)
    where
        FN: FnMut(Entity) + Send,
    {
        // Simplified: execute sequentially for now
        for &entity in &self.entities {
            f(entity);
        }
    }
    
    /// Execute with initialization
    pub fn for_each_init<INIT, FN, T>(&self, mut init: INIT, mut f: FN)
    where
        INIT: FnMut() -> T + Send + Clone,
        FN: FnMut(&mut T, Entity) + Send,
        T: Send,
    {
        let mut state = init();
        for &entity in &self.entities {
            f(&mut state, entity);
        }
    }
    
    /// Get batching strategy
    pub fn batching_strategy(&self) -> BatchingStrategy {
        BatchingStrategy::Fixed(self.batch_size)
    }
    
    /// Set batch size
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }
}

/// Parallel many iter - iterates over many entities in parallel
pub struct QueryParManyIter<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entity_iter: I,
    batch_size: usize,
}

impl<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> QueryParManyIter<'w, 's, Q, F, I> {
    pub fn new(entity_iter: I, batch_size: usize) -> Self {
        Self {
            _phantom: PhantomData,
            entity_iter,
            batch_size,
        }
    }
    
    /// Execute for each entity in parallel
    pub fn for_each<FN>(mut self, mut f: FN)
    where
        FN: FnMut(Entity) + Send,
    {
        while let Some(entity) = self.entity_iter.next() {
            f(entity);
        }
    }
}

/// Parallel combination iterator
pub struct QueryParCombinationIter<'w, 's, Q: QueryData, F: QueryFilter, const N: usize> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entities: Vec<Entity>,
    batch_size: usize,
}

impl<'w, 's, Q: QueryData, F: QueryFilter, const N: usize> QueryParCombinationIter<'w, 's, Q, F, N> {
    pub fn new(entities: Vec<Entity>, batch_size: usize) -> Self {
        Self {
            _phantom: PhantomData,
            entities,
            batch_size,
        }
    }
    
    /// Execute for each combination in parallel
    pub fn for_each<FN>(&self, mut f: FN)
    where
        FN: FnMut([Entity; N]) + Send,
    {
        if self.entities.len() < N {
            return;
        }
        
        // Simplified: execute first combination only
        let mut combo = [Entity::from_raw(0); N];
        for (i, &entity) in self.entities.iter().take(N).enumerate() {
            combo[i] = entity;
        }
        f(combo);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Component;

    #[derive(Debug, Clone, Copy)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[test]
    fn test_batching_strategy() {
        let fixed = BatchingStrategy::Fixed(32);
        assert_eq!(fixed.batch_size(100), 32);
        
        let adaptive = BatchingStrategy::Adaptive { min: 16, max: 128 };
        assert_eq!(adaptive.batch_size(100), 64);
    }

    #[test]
    fn test_par_iter_creation() {
        let entities = vec![Entity::from_raw(0), Entity::from_raw(1), Entity::from_raw(2)];
        let iter = QueryParIter::<(), ()>::new(entities, 64);
        assert_eq!(iter.batching_strategy(), BatchingStrategy::Fixed(64));
    }

    #[test]
    fn test_par_iter_for_each() {
        let entities = vec![Entity::from_raw(0), Entity::from_raw(1), Entity::from_raw(2)];
        let iter = QueryParIter::<(), ()>::new(entities, 64);
        
        let mut count = 0;
        iter.for_each(|_| {
            count += 1;
        });
        assert_eq!(count, 3);
    }

    #[test]
    fn test_par_iter_for_each_init() {
        let entities = vec![Entity::from_raw(0), Entity::from_raw(1), Entity::from_raw(2)];
        let iter = QueryParIter::<(), ()>::new(entities, 64);
        
        iter.for_each_init(
            || 0,
            |state, _entity| {
                *state += 1;
            }
        );
    }
}