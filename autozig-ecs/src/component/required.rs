//! Required components system - allows components to automatically add other components

use crate::component::{Component, ComponentId, Components};
use crate::ptr::OwningPtr;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt;

/// Error type for required components operations
#[derive(Debug, Clone)]
pub enum RequiredComponentsError {
    /// Circular dependency detected
    CircularDependency {
        /// The component that would create a cycle
        component: ComponentId,
        /// The dependency chain
        chain: Vec<ComponentId>,
    },
    /// Component not found
    ComponentNotFound(ComponentId),
    /// Duplicate requirement
    DuplicateRequirement {
        /// The component with duplicate requirement
        component: ComponentId,
        /// The duplicate required component
        required: ComponentId,
    },
}

impl fmt::Display for RequiredComponentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CircularDependency { component, chain } => {
                write!(
                    f,
                    "Circular dependency detected for component {:?}, chain: {:?}",
                    component, chain
                )
            }
            Self::ComponentNotFound(id) => write!(f, "Component {:?} not found", id),
            Self::DuplicateRequirement { component, required } => {
                write!(
                    f,
                    "Component {:?} already requires {:?}",
                    component, required
                )
            }
        }
    }
}

impl std::error::Error for RequiredComponentsError {}

/// A constructor function for a required component
pub type RequiredComponentConstructor = unsafe fn(OwningPtr<'_>);

/// Represents a required component relationship
#[derive(Debug, Clone)]
pub struct RequiredComponent {
    /// The ID of the required component
    pub component_id: ComponentId,
    /// Optional constructor function
    pub constructor: Option<RequiredComponentConstructor>,
}

impl RequiredComponent {
    /// Create a new required component
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            component_id,
            constructor: None,
        }
    }

    /// Create a required component with a constructor
    pub fn with_constructor(
        component_id: ComponentId,
        constructor: RequiredComponentConstructor,
    ) -> Self {
        Self {
            component_id,
            constructor: Some(constructor),
        }
    }
}

/// Registry for required component relationships
#[derive(Debug, Default)]
pub struct RequiredComponents {
    /// Map from component ID to its required components
    requirements: HashMap<ComponentId, Vec<RequiredComponent>>,
}

impl RequiredComponents {
    /// Create a new required components registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register that `component` requires `required`
    pub fn register_required(
        &mut self,
        component: ComponentId,
        required: RequiredComponent,
    ) -> Result<(), RequiredComponentsError> {
        // Check for duplicates
        if let Some(existing) = self.requirements.get(&component) {
            if existing.iter().any(|r| r.component_id == required.component_id) {
                return Err(RequiredComponentsError::DuplicateRequirement {
                    component,
                    required: required.component_id,
                });
            }
        }

        // Check for circular dependencies
        if self.would_create_cycle(component, required.component_id) {
            return Err(RequiredComponentsError::CircularDependency {
                component,
                chain: vec![component, required.component_id],
            });
        }

        self.requirements
            .entry(component)
            .or_insert_with(Vec::new)
            .push(required);

        Ok(())
    }

    /// Register a requirement by component IDs
    pub fn register_required_by_id(
        &mut self,
        component: ComponentId,
        required: ComponentId,
    ) -> Result<(), RequiredComponentsError> {
        self.register_required(component, RequiredComponent::new(required))
    }

    /// Register a requirement with a dynamic constructor
    pub fn register_required_dynamic_with(
        &mut self,
        component: ComponentId,
        required: ComponentId,
        constructor: RequiredComponentConstructor,
    ) -> Result<(), RequiredComponentsError> {
        self.register_required(
            component,
            RequiredComponent::with_constructor(required, constructor),
        )
    }

    /// Get all required components for a given component
    pub fn get(&self, component: ComponentId) -> Option<&[RequiredComponent]> {
        self.requirements.get(&component).map(|v| v.as_slice())
    }

    /// Iterate over all component IDs that have requirements
    pub fn iter_ids(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.requirements.keys().copied()
    }

    /// Check if adding a requirement would create a cycle
    fn would_create_cycle(&self, from: ComponentId, to: ComponentId) -> bool {
        if from == to {
            return true;
        }

        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![to];

        while let Some(current) = stack.pop() {
            if current == from {
                return true;
            }

            if visited.insert(current) {
                if let Some(requirements) = self.requirements.get(&current) {
                    for req in requirements {
                        stack.push(req.component_id);
                    }
                }
            }
        }

        false
    }

    /// Get all transitive requirements for a component (including indirect)
    pub fn get_all_requirements(&self, component: ComponentId) -> Vec<ComponentId> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![component];

        while let Some(current) = stack.pop() {
            if visited.insert(current) {
                if let Some(requirements) = self.requirements.get(&current) {
                    for req in requirements {
                        result.push(req.component_id);
                        stack.push(req.component_id);
                    }
                }
            }
        }

        result
    }
}

/// Trait for registering required components
pub trait RequiredComponentsRegistrator {
    /// Register that this component requires another component
    fn register_required<T: Component, R: Component>(
        &mut self,
    ) -> Result<(), RequiredComponentsError>;

    /// Register a required component with a constructor
    fn register_required_with<T: Component, R: Component>(
        &mut self,
        constructor: RequiredComponentConstructor,
    ) -> Result<(), RequiredComponentsError>;
}

impl RequiredComponentsRegistrator for Components {
    fn register_required<T: Component, R: Component>(
        &mut self,
    ) -> Result<(), RequiredComponentsError> {
        let component_id = self.register_component::<T>();
        let required_id = self.register_component::<R>();

        // Note: In a full implementation, this would integrate with a RequiredComponents registry
        // For now, we just ensure both components are registered
        Ok(())
    }

    fn register_required_with<T: Component, R: Component>(
        &mut self,
        constructor: RequiredComponentConstructor,
    ) -> Result<(), RequiredComponentsError> {
        let component_id = self.register_component::<T>();
        let required_id = self.register_component::<R>();

        // Note: In a full implementation, this would store the constructor
        Ok(())
    }
}