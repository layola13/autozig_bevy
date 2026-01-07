//! # AutoZig ECS - Bevy ECS implemented in Zig
//!
//! 90% Zig实现，10% Rust包装 

pub mod entity;
pub mod component;
pub mod world;
pub mod query;
pub mod system;
pub mod resource;
pub mod event;
pub mod command;
pub mod plugin;
pub mod system_param;
pub mod into_system;

pub mod prelude {
    pub use crate::{
        entity::Entity,
        component::{Component, SparseSet},
        world::World,
        query::QueryState,
        system::Schedule,
        resource::{Res, ResMut, ResourceRegistry},
        event::{Events, EventWriter, EventReader},
        command::{Commands, CommandBuffer},
        plugin::{Plugin, App, CorePlugin, TimePlugin, DefaultPlugins},
        system_param::SystemParam,
        into_system::IntoSystem,
    };
}

