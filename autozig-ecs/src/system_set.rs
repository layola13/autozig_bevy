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

impl<S: SystemSet + Sized> IntoSystemSet for S {
    type Set = S;
    fn into_system_set(self) -> Self::Set {
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
    pub(crate) set: Box<dyn SystemSet>,
    pub(crate) before: Vec<String>,
    pub(crate) after: Vec<String>,
    pub(crate) in_sets: Vec<String>,
}

impl SystemSetConfig {
    pub fn new(set: Box<dyn SystemSet>) -> Self {
        Self {
            set,
            before: Vec::new(),
            after: Vec::new(),
            in_sets: Vec::new(),
        }
    }
}

/// Multiple system set configs
pub struct SystemSetConfigs {
    pub(crate) configs: Vec<SystemSetConfig>,
    pub(crate) chained: bool,
}

/// Trait for converting into system set config
pub trait IntoSystemSetConfig {
    fn into_config(self) -> SystemSetConfig;
}

impl<S: IntoSystemSet> IntoSystemSetConfig for S {
    fn into_config(self) -> SystemSetConfig {
        SystemSetConfig::new(Box::new(self.into_system_set()))
    }
}

/// Trait for converting into multiple system set configs
pub trait IntoSystemSetConfigs {
    fn into_configs(self) -> SystemSetConfigs;
    
    fn chain(self) -> SystemSetConfigs 
    where Self: Sized 
    {
        let mut configs = self.into_configs();
        configs.chained = true;
        configs
    }

    fn in_set(self, set: impl SystemSet) -> SystemSetConfigs 
    where Self: Sized 
    {
        let mut configs = self.into_configs();
        let name = set.as_str().to_string();
        for config in &mut configs.configs {
            config.in_sets.push(name.clone());
        }
        configs
    }

    fn before<S>(self, set: S) -> SystemSetConfigs 
    where 
        Self: Sized,
        S: IntoSystemSet,
    {
        let mut configs = self.into_configs();
        let set = set.into_system_set();
        let name = set.as_str().to_string();
        for config in &mut configs.configs {
            config.before.push(name.clone());
        }
        configs
    }

    fn after<S>(self, set: S) -> SystemSetConfigs 
    where 
        Self: Sized,
        S: IntoSystemSet,
    {
        let mut configs = self.into_configs();
        let set = set.into_system_set();
        let name = set.as_str().to_string();
        for config in &mut configs.configs {
            config.after.push(name.clone());
        }
        configs
    }
}

impl<S: IntoSystemSetConfig> IntoSystemSetConfigs for S {
    fn into_configs(self) -> SystemSetConfigs {
        SystemSetConfigs {
            configs: vec![self.into_config()],
            chained: false,
        }
    }
}

impl IntoSystemSetConfigs for SystemSetConfigs {
    fn into_configs(self) -> SystemSetConfigs {
        self
    }
}

macro_rules! impl_system_set_configs_tuple {
    ($($param:ident),*) => {
        impl<$($param),*> IntoSystemSetConfigs for ($($param,)*)
        where
            $($param: IntoSystemSetConfigs),*
        {
            fn into_configs(self) -> SystemSetConfigs {
                #[allow(non_snake_case)]
                let ($($param,)*) = self;
                let mut configs = Vec::new();
                $(
                    configs.extend($param.into_configs().configs);
                )*
                SystemSetConfigs {
                    configs,
                    chained: false,
                }
            }
        }
    }
}

impl_system_set_configs_tuple!(P1, P2);
impl_system_set_configs_tuple!(P1, P2, P3);
impl_system_set_configs_tuple!(P1, P2, P3, P4);


/// System set based on system type
pub struct SystemTypeSet<T>(PhantomData<fn() -> T>);

/// Anonymous system set
pub struct AnonymousSet(usize);

impl SystemSet for AnonymousSet {
    fn as_str(&self) -> &str {
        "AnonymousSet"
    }
}