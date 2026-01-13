use crate::component::ComponentId;
use crate::archetype::ArchetypeId;
use crate::change_detection::Tick;
use std::collections::HashSet;

/// Metadata for a system, including access information and last run tick.
#[derive(Debug, Default, Clone)]
pub struct SystemMeta {
    pub(crate) name: String,
    pub(crate) component_access_set: HashSet<ComponentId>,
    pub(crate) archetype_component_access: HashSet<(ArchetypeId, ComponentId)>,
    pub(crate) is_exclusive: bool,
    pub(crate) last_run: Tick,
    pub(crate) param_warn_policy: (), // Placeholder
}

impl SystemMeta {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_exclusive: false,
            last_run: Tick::default(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn last_run(&self) -> Tick {
        self.last_run
    }

    pub fn last_run_tick(&self) -> u32 {
        self.last_run.get()
    }

    pub fn set_last_run(&mut self, tick: Tick) {
        self.last_run = tick;
    }
}
