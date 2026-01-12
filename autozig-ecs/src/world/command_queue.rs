//! Command queue for deferred world operations

use std::collections::VecDeque;

/// Command - 延迟执行的命令trait
pub trait Command: Send + Sync {
    fn apply(self: Box<Self>, world: &mut crate::world::World);
}

/// CommandQueue - 命令队列，用于存储待执行的命令
pub struct CommandQueue {
    queue: VecDeque<Box<dyn Command>>,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    
    /// 添加命令到队列
    pub fn push<C: Command + 'static>(&mut self, command: C) {
        self.queue.push_back(Box::new(command));
    }
    
    /// 执行所有命令
    pub fn apply(&mut self, world: &mut crate::world::World) {
        while let Some(command) = self.queue.pop_front() {
            command.apply(world);
        }
    }
    
    /// 追加另一个命令队列
    pub fn append(&mut self, other: &mut CommandQueue) {
        self.queue.append(&mut other.queue);
    }
    
    /// 获取队列长度
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    
    /// 检查队列是否为空
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// 清空队列
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

// 实现一些基本的命令类型

/// SpawnCommand - 生成实体命令
pub struct SpawnCommand;

impl Command for SpawnCommand {
    fn apply(self: Box<Self>, world: &mut crate::world::World) {
        world.spawn_empty();
    }
}

/// DespawnCommand - 删除实体命令
pub struct DespawnCommand {
    pub entity: crate::entity::Entity,
}

impl Command for DespawnCommand {
    fn apply(self: Box<Self>, world: &mut crate::world::World) {
        let _ = world.despawn(self.entity);
    }
}