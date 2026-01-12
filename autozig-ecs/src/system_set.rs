//! System sets - Grouping and ordering systems

use std::marker::PhantomData;

/// Trait for system sets
pub trait SystemSet: Send + Sync + 'static {
    fn as_str(&self) -> &str;
}

impl SystemSet for &'static str {
    fn as_str(&self) -> &str {
        self
    }
}

/// Trait for converting into system sets
pub trait IntoSystemSet {
    type Set: SystemSet;
    fn into_system_set(self) -> Self::Set;
}

/// Configuration for a system set
pub struct SystemSetConfig {
    set: Box<dyn SystemSet>,
}

/// Multiple system set configs
pub struct SystemSetConfigs {
    configs: Vec<SystemSetConfig>,
}

/// Trait for converting into system set config
pub trait IntoSystemSetConfig {
    fn into_config(self) -> SystemSetConfig;
}

/// Trait for converting into multiple system set configs
pub trait IntoSystemSetConfigs {
    fn into_configs(self) -> SystemSetConfigs;
}

/// System set based on system type
pub struct SystemTypeSet<T>(PhantomData<fn() -> T>);

/// Anonymous system set
pub struct AnonymousSet(usize);

impl SystemSet for AnonymousSet {
    fn as_str(&self) -> &str {
        "AnonymousSet"
    }
}