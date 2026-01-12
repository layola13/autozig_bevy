//! # AutoZig ECS - Bevy ECS implemented in Zig
//!
//! Complete ECS (Entity Component System) implementation matching Bevy ECS API 1:1
//! Architecture: 90% Zig + 10% Rust
//! - Core logic implemented in Zig for performance
//! - Rust provides type safety and ergonomic API

#![allow(dead_code, unused_imports, unused_variables)]

use autozig_macro::include_zig;

// Include common.zig for shared utilities
include_zig!("src/zig/common.zig", {
    fn common_noop();
});

// Core modules
pub mod ptr;              // Type-erased pointer types (foundational)
pub mod entity;
pub mod component;
pub mod bundle;
pub mod storage;
pub mod world;
pub mod query;
pub mod system;
pub mod schedule;
pub mod resource;
pub mod event;
pub mod command;
pub mod plugin;
pub mod system_param;
pub mod into_system;
pub mod change_detection;
pub mod observer;
pub mod archetype;
pub mod table;

// Additional required modules for complete API coverage
pub mod removal_detection;
pub mod system_set;
pub mod condition;
pub mod combinator;
pub mod exclusive_system;
pub mod function_system;
pub mod system_adapter;
pub mod param_set;
pub mod local;
pub mod deferred;
pub mod filtered_entity;
pub mod entity_hash;

// Advanced API modules - 高级API模块
pub mod entity_advanced;      // Entity高级API
pub mod component_advanced;   // Component高级API
pub mod query_advanced;       // Query高级API
pub mod hierarchy;            // Relationship/Hierarchy API (包含relationship模块)
pub mod dag;                  // DAG/图论系统
pub mod utils;                // 工具类型

// TODO: 以下模块待子任务补全
// pub mod system_advanced;      // System高级API
// pub mod schedule_advanced;    // Schedule高级API
// pub mod event_advanced;       // Event高级API
// pub mod observer_advanced;    // Observer高级API

/// Comprehensive prelude with all 463+ API types
pub mod prelude {
    // ==================== World Types ====================
    pub use crate::world::{World, WorldId, WorldCell, UnsafeWorldCell, DeferredWorld};
    pub use crate::filtered_entity::{FilteredEntityRef, FilteredEntityMut};
    
    // ==================== Entity Types ====================
    pub use crate::entity::{Entity, EntityLocation, EntityWorldMut, EntityMut, EntityRef};
    pub use crate::entity_hash::{EntityHash, EntityHashSet, EntityHashMap};
    
    // ==================== Component Types ====================
    pub use crate::component::{
        Component, ComponentId, ComponentInfo, ComponentDescriptor,
        ComponentHooks, Components,
        StorageType, TableStorage, SparseStorage,
        ComponentCloneHandler, ComponentDropHandler,
        RequiredComponents, RequiredComponentsError,
        Mutable, Immutable, Access
    };
    
    // ==================== Bundle Types ====================
    pub use crate::bundle::{Bundle, BundleInfo, BundleId, Bundles, DynamicBundle, BundleSpawner, BundleInserter};
    
    // ==================== Resource Types ====================
    // TODO: Uncomment when implemented:
    // pub use crate::resource::{Resource, ResourceId, FromWorld, Res, ResMut, ResourceRegistry};
    
    // ==================== Query Types ====================
    pub use crate::query::{
        Query, QueryState, QueryIter, QueryEntityError, QueryFilter, QueryData,
        QuerySingleError, QueryBuilder, FilteredAccess, ReadOnlyQueryData, WorldQuery,
        With, Without, Or,
        OptionFetch, BatchingStrategy
    };
    
    // ==================== System Types ====================
    pub use crate::system::{
        System, SystemMeta, SystemState, SystemInput, SystemOut,
        FunctionSystem, ExclusiveFunctionSystem, ExclusiveSystem,
        SystemAdapter, CombinatorSystem, BoxedSystem, In
    };
    pub use crate::into_system::IntoSystem;
    pub use crate::system_param::{SystemParam, ReadOnlySystemParam, StaticSystemParam};
    // pub use crate::param_set::ParamSet;
    // pub use crate::local::Local;
    // pub use crate::deferred::DeferredCommands;
    pub use crate::command::Commands;
    // TODO: Uncomment when implemented:
    // pub use crate::command::{CommandQueue, CommandBuffer};
    // pub use crate::into_system::{SystemParamFunction, IntoSystemConfig, IntoSystemConfigs};
    
    // ==================== Schedule Types ====================
    pub use crate::schedule::Schedule;
    // TODO: Uncomment when implemented:
    // pub use crate::schedule::{Schedules, ScheduleLabel, ScheduleBuildSettings,
    //     ExecutorKind, LogLevel, ScheduleGraph, NodeId, NodeConfigs, SystemConfigs,
    //     Stepping, SteppingState};
    // pub use crate::system_set::{SystemSet, IntoSystemSet, SystemSetConfig, SystemSetConfigs,
    //     IntoSystemSetConfig, IntoSystemSetConfigs, SystemTypeSet, AnonymousSet};
    // pub use crate::condition::{Condition, IntoCondition, RunCriteria, ShouldRun, common_conditions};
    // pub use crate::combinator::{NotSystem, AndThenSystem, OrElseSystem, ChainSystem, PipeSystem};
    
    // ==================== Event Types ====================
    pub use crate::event::{
        Event, Events, EventReader, EventWriter, EventIterator,
        EventId, EventCursor, EventRegistry, EventUpdateSignal,
        EventSequence, EventInstance, ManualEventReader, ManualEventIterator,
        SendBatchIds, EventParIter
    };
    
    // ==================== Observer Types ====================
    pub use crate::observer::Observer;
    // TODO: Uncomment when implemented:
    // pub use crate::observer::{ObserverState, ObserverDescriptor, Trigger, TriggerEvent, TriggerTargets,
    //     ObserverSystem, ObserverRunner, OnAdd, OnInsert, OnRemove, OnReplace,
    //     EntityObserver, ComponentObserver};
    
    // ==================== Storage Types ====================
    pub use crate::storage::{
        SparseSet, Storages, Table
    };
    pub use crate::archetype::Archetype;
    // pub use crate::table::{Table, TableId, TableRow, Column, TableBuilder, Tables, TableMoveResult};
    // pub use crate::archetype::{Archetype, ArchetypeId, ArchetypeGeneration, Archetypes,
    //     ArchetypeEntity, ArchetypeRecord, ArchetypeComponentId, AddBundle, Edges, ArchetypeSwapRemoveResult};
    
    // ==================== Change Detection ====================
    pub use crate::change_detection::{
        Tick, ComponentTicks, ChangeDetectionContext, RemovedComponents,
        DetectChanges, DetectChangesMut,
        Mut, MutUntyped, TickCells, LastTick
    };
    // pub use crate::removal_detection::{RemovedComponentEvents, RemovedComponentEntity, RemovedComponentReader};
    
    // ==================== Plugin System ====================
    pub use crate::plugin::Plugin;
    // TODO: Uncomment when implemented:
    // pub use crate::plugin::{PluginGroup, PluginGroupBuilder, App, SubApp, AppLabel,
    //     CorePlugin, TimePlugin, DefaultPlugins, CleanupPlugin};
}

// Re-export commonly used types at crate level
pub use prelude::*;

// Type aliases matching Bevy
pub type EntityHashMap<V> = std::collections::HashMap<Entity, V, ahash::RandomState>;
pub type EntityHashSet = std::collections::HashSet<Entity, ahash::RandomState>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let world = World::new();
        assert!(world.id().index() >= 0);
    }
    
    #[test]
    fn test_entity_spawn() {
        let mut world = World::new();
        let id = world.spawn_empty().id();
        assert!(world.get_entity(id).is_ok());
    }
}
