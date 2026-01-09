//! # AutoZig ECS - Bevy ECS implemented in Zig
//!
//! 90% Zig实现，10% Rust包装

use autozig::include_zig;

// Include common.zig to ensure it's copied to build directory for modular imports
include_zig!("src/zig/common.zig", {
    fn common_noop();
});

pub mod entity;
pub mod component;
pub mod bundle;
pub mod storage;
pub mod world;
pub mod query;
pub mod system;
pub mod resource;
pub mod event;
pub mod command;
pub mod plugin;
pub mod system_param;
pub mod into_system;
pub mod change_detection;

pub mod prelude {
    pub use crate::{
        entity::Entity,
        component::{Component, SparseSet},
        bundle::Bundle,
        storage::{StorageType, Archetype, Table, SparseSet as StorageSparseSet},
        world::World,
        query::{Query, QueryState, QueryData, QueryFilter, ReadOnlyQueryData,
                Read, Write, With, Without, QueryEntityError},
        system::Schedule,
        resource::{Res, ResMut, ResourceRegistry},
        event::{Events, EventWriter, EventReader},
        command::{Commands, CommandBuffer},
        plugin::{Plugin, App, CorePlugin, TimePlugin, DefaultPlugins},
        system_param::SystemParam,
        into_system::IntoSystem,
        change_detection::{Tick, ComponentTicks, ChangeDetectionContext,
                          RemovedComponents, Changed, Added},
    };
}

