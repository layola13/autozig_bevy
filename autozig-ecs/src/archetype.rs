//! Archetype - Storage optimization based on component combinations

use autozig_macro::include_zig;
use crate::entity::Entity;
use crate::component::ComponentId;
use std::collections::HashMap;

include_zig!("src/zig/archetype.zig", {
    fn archetype_create() -> *mut u8;
});

/// Unique identifier for an archetype
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ArchetypeId(pub u32);

impl ArchetypeId {
    pub const EMPTY: Self = Self(0);
    pub const INVALID: Self = Self(u32::MAX);
    
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

/// Generation counter for archetypes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ArchetypeGeneration(pub u64);

impl ArchetypeGeneration {
    pub fn initial() -> Self {
        Self(0)
    }
    
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// An archetype - a unique combination of components
#[repr(C)]
pub struct Archetype {
    id: ArchetypeId,
    generation: ArchetypeGeneration,
    components: Vec<ComponentId>,
    entities: Vec<ArchetypeEntity>,
    edges: Edges,
}

impl Archetype {
    pub fn new(id: ArchetypeId, components: Vec<ComponentId>) -> Self {
        Self {
            id,
            generation: ArchetypeGeneration::initial(),
            components,
            entities: Vec::new(),
            edges: Edges::default(),
        }
    }
    
    pub fn id(&self) -> ArchetypeId {
        self.id
    }
    
    pub fn generation(&self) -> ArchetypeGeneration {
        self.generation
    }
    
    pub fn components(&self) -> &[ComponentId] {
        &self.components
    }
    
    pub fn entities(&self) -> &[ArchetypeEntity] {
        &self.entities
    }
    
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

/// Container for all archetypes
pub struct Archetypes {
    archetypes: Vec<Archetype>,
    archetype_component_count: usize,
}

impl Archetypes {
    pub fn new() -> Self {
        let mut archetypes = Vec::new();
        // Add empty archetype
        archetypes.push(Archetype::new(ArchetypeId::EMPTY, Vec::new()));
        
        Self {
            archetypes,
            archetype_component_count: 0,
        }
    }
    
    pub fn get(&self, id: ArchetypeId) -> Option<&Archetype> {
        self.archetypes.get(id.index())
    }
    
    pub fn get_mut(&mut self, id: ArchetypeId) -> Option<&mut Archetype> {
        self.archetypes.get_mut(id.index())
    }
    
    pub fn len(&self) -> usize {
        self.archetypes.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.archetypes.is_empty()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Archetype> {
        self.archetypes.iter()
    }
}

/// Entity stored in an archetype
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ArchetypeEntity {
    pub entity: Entity,
    pub table_row: usize,
}

/// Record of which archetype and row an entity belongs to
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ArchetypeRecord {
    pub archetype_id: ArchetypeId,
    pub archetype_row: usize,
}

/// Component ID within an archetype
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ArchetypeComponentId(pub usize);

/// Bundle addition edges between archetypes
pub struct AddBundle {
    pub archetype_id: ArchetypeId,
    pub bundle_status: Vec<ComponentStatus>,
}

/// Status of a component in a bundle operation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentStatus {
    Added,
    Existing,
}

/// Edges between archetypes for component additions/removals
#[derive(Default)]
pub struct Edges {
    pub add_bundle: HashMap<Vec<ComponentId>, AddBundle>,
    pub remove_bundle: HashMap<Vec<ComponentId>, ArchetypeId>,
}

impl Edges {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Result of swapping and removing an entity from an archetype
pub struct ArchetypeSwapRemoveResult {
    pub swapped_entity: Option<Entity>,
    pub table_row: usize,
}