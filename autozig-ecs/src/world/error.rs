//! World operation errors

use crate::entity::Entity;
use crate::component::ComponentId;
use std::fmt;

/// EntityWorldMutError - 实体可变访问错误
#[derive(Debug, Clone)]
pub enum EntityWorldMutError {
    EntityNotFound(Entity),
    ComponentNotFound(ComponentId),
    InvalidOperation(String),
}

impl fmt::Display for EntityWorldMutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityNotFound(entity) => write!(f, "Entity {:?} not found", entity),
            Self::ComponentNotFound(id) => write!(f, "Component {:?} not found", id),
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for EntityWorldMutError {}

/// EntityRefError - 实体只读访问错误
#[derive(Debug, Clone)]
pub enum EntityRefError {
    EntityNotFound(Entity),
    ComponentNotFound(ComponentId),
}

impl fmt::Display for EntityRefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityNotFound(entity) => write!(f, "Entity {:?} not found", entity),
            Self::ComponentNotFound(id) => write!(f, "Component {:?} not found", id),
        }
    }
}

impl std::error::Error for EntityRefError {}

/// WorldError - World操作错误
#[derive(Debug, Clone)]
pub enum WorldError {
    ResourceNotFound(String),
    EntityNotFound(Entity),
    ComponentNotFound(ComponentId),
    InvalidOperation(String),
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceNotFound(name) => write!(f, "Resource {} not found", name),
            Self::EntityNotFound(entity) => write!(f, "Entity {:?} not found", entity),
            Self::ComponentNotFound(id) => write!(f, "Component {:?} not found", id),
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for WorldError {}

/// QueryError - 查询错误
#[derive(Debug, Clone)]
pub enum QueryError {
    ComponentNotFound(ComponentId),
    InvalidQuery(String),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComponentNotFound(id) => write!(f, "Component {:?} not found in query", id),
            Self::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
        }
    }
}

impl std::error::Error for QueryError {}

/// EntityMutableFetchError - 实体可变获取错误
#[derive(Debug, Clone)]
pub enum EntityMutableFetchError {
    /// 实体未生成
    NotSpawned(crate::entity::EntityNotSpawnedError),
    /// 实体已被另一个可变借用占用
    AlreadyBorrowed(Entity),
}

impl std::fmt::Display for EntityMutableFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSpawned(err) => write!(f, "Entity not spawned: {}", err),
            Self::AlreadyBorrowed(entity) => write!(f, "Entity {:?} already borrowed mutably", entity),
        }
    }
}

impl std::error::Error for EntityMutableFetchError {}