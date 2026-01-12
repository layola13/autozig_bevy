//! Component cloning functionality

use crate::component::Component;
use crate::ptr::{OwningPtr, Ptr};
use std::marker::PhantomData;

/// Function pointer type for cloning components
pub type ComponentCloneFn = unsafe fn(source: Ptr<'_>, destination: OwningPtr<'_>);

/// The clone behavior to use when cloning or moving a [`Component`].
#[derive(Clone, Debug, Default)]
pub enum ComponentCloneBehavior {
    /// Uses the default behavior (which is passed to [`ComponentCloneBehavior::resolve`])
    #[default]
    Default,
    /// Do not clone/move this component.
    Ignore,
    /// Uses a custom [`ComponentCloneFn`].
    Custom(ComponentCloneFn),
}

impl ComponentCloneBehavior {
    /// Set clone handler based on `Clone` trait.
    ///
    /// If set as a handler for a component that is not the same as the one used to create this handler, it will panic.
    pub fn clone<C: Component + Clone>() -> Self {
        Self::Custom(component_clone_via_clone::<C>)
    }

    /// Set clone handler based on `Reflect` trait.
    #[cfg(feature = "bevy_reflect")]
    pub fn reflect() -> Self {
        Self::Custom(component_clone_via_reflect)
    }

    /// Returns the "global default"
    pub fn global_default_fn() -> ComponentCloneFn {
        #[cfg(feature = "bevy_reflect")]
        return component_clone_via_reflect;
        #[cfg(not(feature = "bevy_reflect"))]
        return component_clone_ignore;
    }

    /// Resolves the [`ComponentCloneBehavior`] to a [`ComponentCloneFn`]. If [`ComponentCloneBehavior::Default`] is
    /// specified, the given `default` function will be used.
    pub fn resolve(&self, default: ComponentCloneFn) -> ComponentCloneFn {
        match self {
            ComponentCloneBehavior::Default => default,
            ComponentCloneBehavior::Ignore => component_clone_ignore,
            ComponentCloneBehavior::Custom(custom) => *custom,
        }
    }
}

/// Component clone handler function implemented using the [`Clone`] trait.
/// Can be set as clone handler for the specific component it is implemented for.
/// It will panic if set as handler for any other component.
pub unsafe fn component_clone_via_clone<C: Clone + Component>(
    source: Ptr<'_>,
    destination: OwningPtr<'_>,
) {
    // SAFETY: source points to a valid C instance
    let component = unsafe { source.deref::<C>() };
    let cloned = component.clone();
    // SAFETY: destination points to valid uninitialized memory for C
    unsafe {
        destination.write(cloned);
    }
}

/// Component clone handler function implemented using reflect.
/// Can be set as clone handler for any registered component,
/// but only reflected components will be cloned.
#[cfg(feature = "bevy_reflect")]
pub unsafe fn component_clone_via_reflect(
    _source: Ptr<'_>,
    _destination: OwningPtr<'_>,
) {
    // TODO: Implement reflection-based cloning when bevy_reflect integration is complete
    // For now, this is a stub that does nothing
}

/// Noop implementation of component clone handler function.
pub unsafe fn component_clone_ignore(_source: Ptr<'_>, _destination: OwningPtr<'_>) {}

/// Wrapper for components clone specialization using autoderef.
#[doc(hidden)]
pub struct DefaultCloneBehaviorSpecialization<T>(PhantomData<T>);

impl<T> Default for DefaultCloneBehaviorSpecialization<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Base trait for components clone specialization using autoderef.
#[doc(hidden)]
pub trait DefaultCloneBehaviorBase {
    fn default_clone_behavior(&self) -> ComponentCloneBehavior;
}

impl<C> DefaultCloneBehaviorBase for DefaultCloneBehaviorSpecialization<C> {
    fn default_clone_behavior(&self) -> ComponentCloneBehavior {
        ComponentCloneBehavior::Default
    }
}

/// Specialized trait for components clone specialization using autoderef.
#[doc(hidden)]
pub trait DefaultCloneBehaviorViaClone {
    fn default_clone_behavior(&self) -> ComponentCloneBehavior;
}

impl<C: Clone + Component> DefaultCloneBehaviorViaClone for &DefaultCloneBehaviorSpecialization<C> {
    fn default_clone_behavior(&self) -> ComponentCloneBehavior {
        ComponentCloneBehavior::clone::<C>()
    }
}

// Re-export functions at module level for convenience
pub use self::{
    component_clone_ignore as ignore,
    component_clone_via_clone as clone,
};

#[cfg(feature = "bevy_reflect")]
pub use self::component_clone_via_reflect as reflect;

/// Returns the global default clone function
pub fn global_default_fn() -> ComponentCloneFn {
    ComponentCloneBehavior::global_default_fn()
}

/// Returns a ComponentCloneBehavior that uses reflection
#[cfg(feature = "bevy_reflect")]
pub fn reflect() -> ComponentCloneBehavior {
    ComponentCloneBehavior::reflect()
}

/// Resolves a ComponentCloneBehavior to a ComponentCloneFn
pub fn resolve(behavior: &ComponentCloneBehavior, default: ComponentCloneFn) -> ComponentCloneFn {
    behavior.resolve(default)
}