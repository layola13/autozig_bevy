use core::fmt::Display;

use crate::{
    component::{ComponentId, Components},
    query::{Access, QueryData},
};

/// Check if `Q` has any internal conflicts.
///
/// This is a thin Rust wrapper around Zig implementation for performance.
#[inline(never)]
pub fn has_conflicts<Q: QueryData>(components: &Components) -> Result<(), QueryAccessError> {
    const MAX_SIZE: usize = 16;
    let Some(state) = Q::get_state(components) else {
        return Err(QueryAccessError::ComponentNotRegistered);
    };
    let iter = Q::iter_access(&state).enumerate();
    let size = iter.size_hint().1.unwrap_or(MAX_SIZE);

    if size > MAX_SIZE {
        for (i, access) in iter {
            for access_other in Q::iter_access(&state).take(i) {
                if let Err(err) = access.is_compatible(access_other) {
                    panic!("{}", err);
                }
            }
        }
    } else {
        // Optimize small sizes by caching iteration result in stack array
        let mut inner_access = [EcsAccessType::Empty; MAX_SIZE];
        for (i, access) in iter {
            for access_other in inner_access.iter().take(i) {
                if let Err(err) = access.is_compatible(*access_other) {
                    panic!("{}", err);
                }
            }
            inner_access[i] = access;
        }
    }

    Ok(())
}

/// The data storage type that is being accessed.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EcsAccessType<'a> {
    /// Accesses Component data
    Component(EcsAccessLevel),
    /// Accesses Resource data
    Resource(ResourceAccessLevel),
    /// Borrowed access from WorldQuery::State
    Access(&'a Access),
    /// Does not access any data that can conflict
    Empty,
}

impl<'a> EcsAccessType<'a> {
    /// Returns `Ok(())` if `self` and `other` are compatible. Returns a [`AccessConflictError`] otherwise.
    #[inline(never)]
    pub fn is_compatible(&self, other: Self) -> Result<(), AccessConflictError<'_>> {
        use EcsAccessLevel::*;
        use EcsAccessType::*;

        match (*self, other) {
            (Component(ReadAll), Component(Write(_)))
            | (Component(Write(_)), Component(ReadAll))
            | (Component(_), Component(WriteAll))
            | (Component(WriteAll), Component(_)) => Err(AccessConflictError(*self, other)),

            (Empty, _)
            | (_, Empty)
            | (Component(_), Resource(_))
            | (Resource(_), Component(_))
            // Read only access doesn't conflict
            | (Component(Read(_)), Component(Read(_)))
            | (Component(ReadAll), Component(Read(_)))
            | (Component(Read(_)), Component(ReadAll))
            | (Component(ReadAll), Component(ReadAll))
            | (Resource(ResourceAccessLevel::Read(_)), Resource(ResourceAccessLevel::Read(_))) => {
                Ok(())
            }

            (Component(Read(id)), Component(Write(id_other)))
            | (Component(Write(id)), Component(Read(id_other)))
            | (Component(Write(id)), Component(Write(id_other)))
            | (
                Resource(ResourceAccessLevel::Read(id)),
                Resource(ResourceAccessLevel::Write(id_other)),
            )
            | (
                Resource(ResourceAccessLevel::Write(id)),
                Resource(ResourceAccessLevel::Read(id_other)),
            )
            | (
                Resource(ResourceAccessLevel::Write(id)),
                Resource(ResourceAccessLevel::Write(id_other)),
            ) => if id == id_other {
                Err(AccessConflictError(*self, other))
            } else {
                Ok(())
            },

            // Borrowed Access
            (Component(Read(component_id)), Access(access))
            | (Access(access), Component(Read(component_id))) => if access.has_component_write(component_id) {
                Err(AccessConflictError(*self, other))
            } else {
                Ok(())
            },

            (Component(Write(component_id)), Access(access))
            | (Access(access), Component(Write(component_id))) => if access.has_component_read(component_id) {
                Err(AccessConflictError(*self, other))
            } else {
                Ok(())
            },

            (Component(ReadAll), Access(access))
            | (Access(access), Component(ReadAll)) => if access.has_any_component_write() {
                Err(AccessConflictError(*self, other))
            } else {
                Ok(())
            },

            (Component(WriteAll), Access(access))
            | (Access(access), Component(WriteAll))=> if access.has_any_component_read() {
                Err(AccessConflictError(*self, other))
            } else {
                Ok(())
            },

            (Resource(ResourceAccessLevel::Read(component_id)), Access(access))
            | (Access(access), Resource(ResourceAccessLevel::Read(component_id))) => if access.has_resource_write(component_id) {
                Err(AccessConflictError(*self, other))
            } else {
                Ok(())
            },
            (Resource(ResourceAccessLevel::Write(component_id)), Access(access))
            | (Access(access), Resource(ResourceAccessLevel::Write(component_id))) => if access.has_resource_read(component_id) {
                Err(AccessConflictError(*self, other))
            } else {
                Ok(())
            },

            (Access(access), Access(other_access)) => if access.is_compatible(other_access) {
                Ok(())
            } else {
                Err(AccessConflictError(*self, other))
            },
        }
    }
}

/// The way the data will be accessed and whether we take access on all components
/// on an entity or just one component.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EcsAccessLevel {
    /// Reads Component with ComponentId
    Read(ComponentId),
    /// Writes Component with ComponentId
    Write(ComponentId),
    /// Potentially reads all Components in the World
    ReadAll,
    /// Potentially writes all Components in the World
    WriteAll,
}

/// Access level needed by QueryData fetch to the resource.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceAccessLevel {
    /// Reads the resource with ComponentId
    Read(ComponentId),
    /// Writes the resource with ComponentId
    Write(ComponentId),
}

/// Error returned from [`EcsAccessType::is_compatible`]
pub struct AccessConflictError<'a>(EcsAccessType<'a>, EcsAccessType<'a>);

impl Display for AccessConflictError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use EcsAccessLevel::*;
        use EcsAccessType::*;

        let AccessConflictError(a, b) = self;
        match (a, b) {
            // ReadAll/WriteAll + Component conflicts
            (Component(ReadAll), Component(Write(id)))
            | (Component(Write(id)), Component(ReadAll)) => {
                write!(
                    f,
                    "Component read all access conflicts with component {id:?} write."
                )
            }
            (Component(WriteAll), Component(Write(id)))
            | (Component(Write(id)), Component(WriteAll)) => {
                write!(
                    f,
                    "Component write all access conflicts with component {id:?} write."
                )
            }
            (Component(WriteAll), Component(Read(id)))
            | (Component(Read(id)), Component(WriteAll)) => {
                write!(
                    f,
                    "Component write all access conflicts with component {id:?} read."
                )
            }
            (Component(WriteAll), Component(ReadAll))
            | (Component(ReadAll), Component(WriteAll)) => {
                write!(f, "Component write all conflicts with component read all.")
            }
            (Component(WriteAll), Component(WriteAll)) => {
                write!(f, "Component write all conflicts with component write all.")
            }

            // Component + Component conflicts
            (Component(Read(id)), Component(Write(id_other)))
            | (Component(Write(id_other)), Component(Read(id))) => write!(
                f,
                "Component {id:?} read conflicts with component {id_other:?} write."
            ),
            (Component(Write(id)), Component(Write(id_other))) => write!(
                f,
                "Component {id:?} write conflicts with component {id_other:?} write."
            ),

            // Borrowed Access conflicts
            (Access(_), Component(Read(id))) | (Component(Read(id)), Access(_)) => write!(
                f,
                "Access has a write that conflicts with component {id:?} read."
            ),
            (Access(_), Component(Write(id))) | (Component(Write(id)), Access(_)) => write!(
                f,
                "Access has a read that conflicts with component {id:?} write."
            ),
            (Access(_), Component(ReadAll)) | (Component(ReadAll), Access(_)) => write!(
                f,
                "Access has a write that conflicts with component read all"
            ),
            (Access(_), Component(WriteAll)) | (Component(WriteAll), Access(_)) => write!(
                f,
                "Access has a read that conflicts with component write all"
            ),
            (Access(_), Resource(ResourceAccessLevel::Read(id)))
            | (Resource(ResourceAccessLevel::Read(id)), Access(_)) => write!(
                f,
                "Access has a write that conflicts with resource {id:?} read."
            ),
            (Access(_), Resource(ResourceAccessLevel::Write(id)))
            | (Resource(ResourceAccessLevel::Write(id)), Access(_)) => write!(
                f,
                "Access has a read that conflicts with resource {id:?} write."
            ),
            (Access(_), Access(_)) => write!(f, "Access conflicts with other Access"),

            _ => {
                unreachable!("Other accesses should be compatible");
            }
        }
    }
}

/// Error returned from [`has_conflicts`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QueryAccessError {
    /// Component was not registered on world
    ComponentNotRegistered,
    /// Entity did not have the requested components
    EntityDoesNotMatch,
}

impl core::error::Error for QueryAccessError {}

impl Display for QueryAccessError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            QueryAccessError::ComponentNotRegistered => {
                write!(
                    f,
                    "At least one component in Q was not registered in world. \
                    Consider calling `World::register_component`"
                )
            }
            QueryAccessError::EntityDoesNotMatch => {
                write!(f, "Entity does not match Q")
            }
        }
    }
}