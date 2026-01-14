use crate::entity::Entity;
use crate::component::ComponentId;

/// Context for component lifecycle hooks
#[derive(Clone, Copy, Debug)]
pub struct HookContext {
    pub entity: Entity,
    pub component_id: ComponentId,
    pub caller: Option<&'static std::panic::Location<'static>>,
}

impl HookContext {
    pub fn new(entity: Entity, component_id: ComponentId) -> Self {
        Self {
            entity,
            component_id,
            caller: None,
        }
    }
}
