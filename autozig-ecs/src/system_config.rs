
use crate::system::{BoxedSystem, System};
use crate::system_set::SystemSet;

/// Configuration for a single system
pub struct SystemConfig {
    pub(crate) system: BoxedSystem,
    pub(crate) before: Vec<Box<dyn SystemSet>>,
    pub(crate) after: Vec<Box<dyn SystemSet>>,
    pub(crate) in_sets: Vec<Box<dyn SystemSet>>,
}

impl SystemConfig {
    pub fn new(system: BoxedSystem) -> Self {
        Self {
            system,
            before: Vec::new(),
            after: Vec::new(),
            in_sets: Vec::new(),
        }
    }
}

/// Collection of system configurations
pub struct SystemConfigs {
    pub(crate) configs: Vec<SystemConfig>,
    pub(crate) chained: bool,
}

impl From<SystemConfig> for SystemConfigs {
    fn from(config: SystemConfig) -> Self {
        Self {
            configs: vec![config],
            chained: false,
        }
    }
}

// Basic trait implementation for SystemConfigs to allow nesting
impl IntoSystemConfigs<()> for SystemConfigs {
    fn into_configs(self) -> SystemConfigs {
        self
    }
}

/// Trait for types that can be converted into SystemConfigs
pub trait IntoSystemConfigs<Marker> {
    fn into_configs(self) -> SystemConfigs;

    fn in_set(self, set: impl SystemSet + Clone) -> SystemConfigs 
    where Self: Sized 
    {
        let mut configs = self.into_configs();
        for config in &mut configs.configs {
            config.in_sets.push(Box::new(set.clone()));
        }
        configs
    }

    fn before<M>(self, set: impl IntoSystemConfigs<M>) -> SystemConfigs 
    where Self: Sized 
    {
        // Placeholder for now - normally would store dependency
        self.into_configs()
    }

    fn after<M>(self, set: impl IntoSystemConfigs<M>) -> SystemConfigs 
    where Self: Sized 
    {
        // Placeholder for now
        self.into_configs()
    }
    
    fn chain(self) -> SystemConfigs
    where Self: Sized
    {
        let mut configs = self.into_configs();
        configs.chained = true;
        configs
    }
}

// Implement for single system
impl<S, P> IntoSystemConfigs<(P,)> for S
where
    S: crate::into_system::IntoSystem<P>,
{
    fn into_configs(self) -> SystemConfigs {
        let sys = self.into_system();
        let boxed = crate::system::BoxedSystem::new(sys, "closure_system");
        SystemConfig::new(boxed).into()
    }
}

// Implement for tuples
macro_rules! impl_system_configs_tuple {
    ($(($param:ident, $marker:ident)),*) => {
        impl<$($param),*, $($marker),*> IntoSystemConfigs<($($marker,)*)> for ($($param,)*)
        where
            $($param: IntoSystemConfigs<$marker>),*
        {
            fn into_configs(self) -> SystemConfigs {
                #[allow(non_snake_case)]
                let ($($param,)*) = self;
                let mut configs = Vec::new();
                $(
                    configs.extend($param.into_configs().configs);
                )*
                SystemConfigs {
                    configs,
                    chained: false,
                }
            }
        }
    }
}

impl_system_configs_tuple!((P1, M1), (P2, M2));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4));
// Add more as needed...
