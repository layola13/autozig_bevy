//! Error types for query system
//! 查询系统的错误类型

use crate::entity::Entity;
use std::fmt;

/// Errors that can occur when querying for a single entity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuerySingleError {
    /// No entities matched the query
    NoEntities(&'static str),
    /// Multiple entities matched the query
    MultipleEntities(&'static str),
}

impl fmt::Display for QuerySingleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoEntities(query) => {
                write!(f, "No entities matched the query: {}", query)
            }
            Self::MultipleEntities(query) => {
                write!(f, "Multiple entities matched the query: {}", query)
            }
        }
    }
}

impl std::error::Error for QuerySingleError {}

/// Errors that can occur when accessing a specific entity in a query
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryEntityError {
    /// The entity does not exist in the world
    NoSuchEntity(Entity),
    /// The entity does not match the query's filters
    QueryDoesNotMatch(Entity),
    /// Attempting to access the same component mutably multiple times
    AliasedMutability(Entity),
}

impl fmt::Display for QueryEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSuchEntity(entity) => {
                write!(f, "Entity {:?} does not exist", entity)
            }
            Self::QueryDoesNotMatch(entity) => {
                write!(f, "Entity {:?} does not match query", entity)
            }
            Self::AliasedMutability(entity) => {
                write!(f, "Mutable aliasing detected for entity {:?}", entity)
            }
        }
    }
}

impl std::error::Error for QueryEntityError {}

/// Errors related to component access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryComponentError {
    /// Component was not found on the entity
    MissingComponent,
    /// Component was found but could not be accessed due to borrowing rules
    CannotAccess,
    /// Component type mismatch
    TypeMismatch,
}

impl fmt::Display for QueryComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingComponent => write!(f, "Component not found on entity"),
            Self::CannotAccess => write!(f, "Cannot access component due to borrowing rules"),
            Self::TypeMismatch => write!(f, "Component type mismatch"),
        }
    }
}

impl std::error::Error for QueryComponentError {}

/// Errors that can occur when building queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryBuildError {
    /// Query contains conflicting access
    ConflictingAccess(String),
    /// Query is invalid
    InvalidQuery(String),
    /// Component not registered
    ComponentNotRegistered(String),
}

impl fmt::Display for QueryBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictingAccess(msg) => write!(f, "Conflicting access: {}", msg),
            Self::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
            Self::ComponentNotRegistered(msg) => write!(f, "Component not registered: {}", msg),
        }
    }
}

impl std::error::Error for QueryBuildError {}

/// Errors during query iteration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryIterError {
    /// Iterator was invalidated
    Invalidated,
    /// Out of bounds access
    OutOfBounds,
}

impl fmt::Display for QueryIterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalidated => write!(f, "Query iterator was invalidated"),
            Self::OutOfBounds => write!(f, "Query iterator out of bounds"),
        }
    }
}

impl std::error::Error for QueryIterError {}

/// General query error type
#[derive(Debug)]
pub enum QueryError {
    /// Single entity error
    Single(QuerySingleError),
    /// Entity access error
    Entity(QueryEntityError),
    /// Component error
    Component(QueryComponentError),
    /// Build error
    Build(QueryBuildError),
    /// Iterator error
    Iter(QueryIterError),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(e) => write!(f, "{}", e),
            Self::Entity(e) => write!(f, "{}", e),
            Self::Component(e) => write!(f, "{}", e),
            Self::Build(e) => write!(f, "{}", e),
            Self::Iter(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for QueryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Single(e) => Some(e),
            Self::Entity(e) => Some(e),
            Self::Component(e) => Some(e),
            Self::Build(e) => Some(e),
            Self::Iter(e) => Some(e),
        }
    }
}

impl From<QuerySingleError> for QueryError {
    fn from(e: QuerySingleError) -> Self {
        Self::Single(e)
    }
}

impl From<QueryEntityError> for QueryError {
    fn from(e: QueryEntityError) -> Self {
        Self::Entity(e)
    }
}

impl From<QueryComponentError> for QueryError {
    fn from(e: QueryComponentError) -> Self {
        Self::Component(e)
    }
}

impl From<QueryBuildError> for QueryError {
    fn from(e: QueryBuildError) -> Self {
        Self::Build(e)
    }
}

impl From<QueryIterError> for QueryError {
    fn from(e: QueryIterError) -> Self {
        Self::Iter(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_single_error_display() {
        let err = QuerySingleError::NoEntities("test");
        assert!(format!("{}", err).contains("No entities"));

        let err = QuerySingleError::MultipleEntities("test");
        assert!(format!("{}", err).contains("Multiple entities"));
    }

    #[test]
    fn test_query_entity_error_display() {
        let entity = Entity::from_raw(42);
        
        let err = QueryEntityError::NoSuchEntity(entity);
        assert!(format!("{}", err).contains("does not exist"));

        let err = QueryEntityError::QueryDoesNotMatch(entity);
        assert!(format!("{}", err).contains("does not match"));

        let err = QueryEntityError::AliasedMutability(entity);
        assert!(format!("{}", err).contains("aliasing"));
    }

    #[test]
    fn test_query_error_conversion() {
        let single_err = QuerySingleError::NoEntities("test");
        let query_err: QueryError = single_err.into();
        assert!(matches!(query_err, QueryError::Single(_)));

        let entity_err = QueryEntityError::NoSuchEntity(Entity::from_raw(0));
        let query_err: QueryError = entity_err.into();
        assert!(matches!(query_err, QueryError::Entity(_)));
    }
}