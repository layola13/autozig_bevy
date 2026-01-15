
use crate::system::{BoxedSystem, System};
use crate::system_set::SystemSet;
use crate::condition::{BoxedCondition, IntoCondition};

/// Configuration for a single system
pub struct SystemConfig {
    pub(crate) system: BoxedSystem,
    pub(crate) before: Vec<String>,
    pub(crate) after: Vec<String>,
    pub(crate) in_sets: Vec<String>,
    pub(crate) conditions: Vec<BoxedCondition>,
}

impl SystemConfig {
    pub fn new(system: BoxedSystem) -> Self {
        Self {
            system,
            before: Vec::new(),
            after: Vec::new(),
            in_sets: Vec::new(),
            conditions: Vec::new(),
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

    fn in_set(self, set: impl SystemSet) -> SystemConfigs 
    where Self: Sized 
    {
        let mut configs = self.into_configs();
        let name = set.as_str().to_string();
        for config in &mut configs.configs {
            config.in_sets.push(name.clone());
        }
        configs
    }

    fn before<S>(self, set: S) -> SystemConfigs 
    where 
        Self: Sized,
        S: crate::system_set::IntoSystemSet,
    {
        let mut configs = self.into_configs();
        let set = set.into_system_set();
        let name = set.as_str().to_string();
        for config in &mut configs.configs {
            config.before.push(name.clone());
        }
        configs
    }

    fn after<S>(self, set: S) -> SystemConfigs 
    where 
        Self: Sized,
        S: crate::system_set::IntoSystemSet,
    {
        let mut configs = self.into_configs();
        let set = set.into_system_set();
        let name = set.as_str().to_string();
        for config in &mut configs.configs {
            config.after.push(name.clone());
        }
        configs
    }
    
    fn chain(self) -> SystemConfigs
    where Self: Sized
    {
        let mut configs = self.into_configs();
        configs.chained = true;
        configs
    }

    fn run_if<M>(self, condition: impl IntoCondition<M>) -> SystemConfigs 
    where Self: Sized
    {
        let mut configs = self.into_configs();
        let condition = condition.into_condition();
        let boxed: BoxedCondition = Box::new(condition);
        
        if configs.configs.len() > 1 {
            // TODO: Require Clone for Condition to support multiple systems
             panic!("run_if on tuple of systems not supported yet (requires Clone Condition)");
        }
        
        if let Some(config) = configs.configs.first_mut() {
             config.conditions.push(boxed);
        }
        configs
    }
    
    fn ambiguous_with(self, _system: impl Into<String>) -> SystemConfigs
    where Self: Sized
    {
        // TODO: Implement ambiguity sets
        self.into_configs()
    }
}

// Implement for single system
impl<S, P> IntoSystemConfigs<P> for S
where
    S: crate::into_system::IntoSystem<P, (), ()>,
    <S as crate::into_system::IntoSystem<P, (), ()>>::System: 'static,
{
    fn into_configs(self) -> SystemConfigs {
        let sys = self.into_system();
        // Box into type-erased system
        let boxed = crate::system::BoxedSystem::new(sys, std::any::type_name::<S>());
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
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9), (P10, M10));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9), (P10, M10), (P11, M11));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9), (P10, M10), (P11, M11), (P12, M12));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9), (P10, M10), (P11, M11), (P12, M12), (P13, M13));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9), (P10, M10), (P11, M11), (P12, M12), (P13, M13), (P14, M14));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9), (P10, M10), (P11, M11), (P12, M12), (P13, M13), (P14, M14), (P15, M15));
impl_system_configs_tuple!((P1, M1), (P2, M2), (P3, M3), (P4, M4), (P5, M5), (P6, M6), (P7, M7), (P8, M8), (P9, M9), (P10, M10), (P11, M11), (P12, M12), (P13, M13), (P14, M14), (P15, M15), (P16, M16));
// Add more as needed...
