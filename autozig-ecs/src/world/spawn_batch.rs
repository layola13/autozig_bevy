//! Batch entity spawning - 90% Zig + 10% Rust架构
//!
//! SpawnBatch provides efficient batch spawning of entities with bundles,
//! optimized for bulk entity creation scenarios.

use crate::{
    bundle::Bundle,
    entity::Entity,
    world::World,
};
use autozig_macro::include_zig;

// Zig核心实现 - 批量spawn的底层优化
include_zig!("src/world/zig/spawn_batch.zig", {
    fn spawn_batch_reserve(world_ptr: *mut crate::world::WorldOpaque, count: usize);
    fn spawn_batch_alloc_entities(world_ptr: *mut crate::world::WorldOpaque, count: usize) -> *mut Entity;
});

/// Iterator that spawns a batch of entities with the same Bundle components
///
/// This is more efficient than spawning entities one-by-one as it:
/// - Pre-allocates entity IDs in bulk
/// - Reserves archetype storage upfront
/// - Minimizes memory allocations
pub struct SpawnBatchIter<'w, I>
where
    I: Iterator,
    I::Item: Bundle,
{
    inner: I,
    world: &'w mut World,
}

impl<'w, I> SpawnBatchIter<'w, I>
where
    I: Iterator,
    I::Item: Bundle,
{
    /// Creates a new SpawnBatchIter
    pub(crate) fn new(world: &'w mut World, iter: I) -> Self {
        Self { inner: iter, world }
    }
}

impl<'w, I> Iterator for SpawnBatchIter<'w, I>
where
    I: Iterator,
    I::Item: Bundle,
{
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let bundle = self.inner.next()?;
        let entity = self.world.spawn(bundle).id();
        Some(entity)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'w, I> ExactSizeIterator for SpawnBatchIter<'w, I>
where
    I: ExactSizeIterator,
    I::Item: Bundle,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Batch entity spawner for efficient bulk entity creation
///
/// # Example
/// ```ignore
/// let mut world = World::new();
/// let entities: Vec<Entity> = world
///     .spawn_batch((0..1000).map(|i| (Position { x: i as f32, y: 0.0 },)))
///     .collect();
/// ```
pub struct SpawnBatch<'w> {
    world: &'w mut World,
}

impl<'w> SpawnBatch<'w> {
    /// Creates a new SpawnBatch for the given world
    pub(crate) fn new(world: &'w mut World) -> Self {
        Self { world }
    }

    /// Spawns entities from an iterator of Bundles
    pub fn spawn_batch<I>(&mut self, iter: I) -> SpawnBatchIter<'_, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Bundle,
    {
        let iter = iter.into_iter();
        
        // Pre-allocate if size is known
        if let (_, Some(upper)) = iter.size_hint() {
            spawn_batch_reserve(self.world.inner, upper);
        }
        
        SpawnBatchIter::new(self.world, iter)
    }
}

impl World {
    /// Spawns a batch of entities with the same Bundle type
    ///
    /// This is more efficient than calling spawn() repeatedly as it:
    /// - Pre-allocates entity IDs
    /// - Reserves archetype storage
    /// - Minimizes allocations
    ///
    /// # Example
    /// ```ignore
    /// let entities: Vec<Entity> = world
    ///     .spawn_batch(vec![
    ///         (Position { x: 1.0, y: 2.0 }, Velocity { x: 0.0, y: 0.0 }),
    ///         (Position { x: 3.0, y: 4.0 }, Velocity { x: 1.0, y: 1.0 }),
    ///     ])
    ///     .collect();
    /// ```
    pub fn spawn_batch<I>(&mut self, iter: I) -> SpawnBatchIter<'_, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Bundle,
    {
        let iter = iter.into_iter();
        
        // Pre-allocate if size is known
        if let (_, Some(upper)) = iter.size_hint() {
            spawn_batch_reserve(self.inner, upper);
        }
        
        SpawnBatchIter::new(self, iter)
    }

    /// Spawns a batch of empty entities and returns their IDs
    ///
    /// More efficient than calling spawn_empty() repeatedly.
    ///
    /// # Example
    /// ```ignore
    /// let entities = world.spawn_empty_batch(1000);
    /// assert_eq!(entities.len(), 1000);
    /// ```
    pub fn spawn_empty_batch(&mut self, count: usize) -> Vec<Entity> {
        if count == 0 {
            return Vec::new();
        }

        // Allocate entities in bulk using Zig
        let entities_ptr = spawn_batch_alloc_entities(self.inner, count);
        
        // Safety: spawn_batch_alloc_entities returns a valid pointer to `count` entities
        let entities = unsafe {
            std::slice::from_raw_parts(entities_ptr, count).to_vec()
        };
        
        // Free the temporary allocation from Zig
        unsafe {
            let layout = std::alloc::Layout::array::<Entity>(count).unwrap();
            std::alloc::dealloc(entities_ptr as *mut u8, layout);
        }
        
        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_batch_empty() {
        let mut world = World::new();
        let entities = world.spawn_empty_batch(10);
        assert_eq!(entities.len(), 10);
        
        // Verify all entities are valid
        for entity in entities {
            assert!(world.get_entity(entity).is_ok());
        }
    }

    #[test]
    fn spawn_batch_zero_count() {
        let mut world = World::new();
        let entities = world.spawn_empty_batch(0);
        assert_eq!(entities.len(), 0);
    }
}