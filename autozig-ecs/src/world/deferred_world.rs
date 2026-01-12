//! Deferred world operations

use crate::world::World;
use crate::entity::Entity;
use crate::component::ComponentId;

/// DeferredWorld - 延迟世界操作，用于在系统执行期间安全地访问World
pub struct DeferredWorld<'w> {
    world: &'w mut World,
}

impl<'w> DeferredWorld<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self { world }
    }
    
    /// 获取World的不可变引用
    pub fn as_world(&self) -> &World {
        self.world
    }
    
    /// 获取World的可变引用
    pub fn as_world_mut(&mut self) -> &mut World {
        self.world
    }
    
    /// 获取实体的可变访问
    pub fn entity_mut(&mut self, entity: Entity) -> Option<crate::world::EntityWorldMut<'_>> {
        if self.world.entities().contains(entity) {
            Some(crate::world::EntityWorldMut::new(entity, self.world))
        } else {
            None
        }
    }
    
    /// 获取资源的可变引用
    pub fn get_resource_mut<T: 'static + crate::resource::Resource>(&mut self) -> Option<crate::resource::ResMut<'_, T>> {
        self.world.get_resource_mut::<T>()
    }
    
    /// 获取或初始化资源
    pub fn resource_mut<T: 'static + Default + crate::resource::Resource>(&mut self) -> crate::resource::ResMut<'_, T> {
        self.world.resource_mut::<T>()
    }
    
    /// 通过ComponentId获取组件的可变引用
    pub fn get_mut_by_id(&mut self, entity: Entity, component_id: ComponentId) -> Option<*mut u8> {
        // TODO: 实现通过ID获取组件
        None
    }
    
    /// 查询World
    pub fn query<Q>(&mut self) -> crate::query::QueryState<Q>
    where
        Q: crate::query::QueryData,
    {
        crate::query::QueryState::new(self.world)
    }
    
    /// 转换为完全延迟的World
    pub fn into_deferred(self) -> &'w mut World {
        self.world
    }
    
    // TODO: 实现剩余的~10个deferred_world API
    // - entities_and_commands()
    // - trigger_raw()
    // - write_message系列
}