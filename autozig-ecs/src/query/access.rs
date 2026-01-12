//! Access control module for query system
//! 查询系统的访问控制模块
//! 
//! This module implements 90% Zig + 10% Rust architecture:
//! - Core logic in Zig (access_core.zig, access_set.zig)
//! - Thin Rust wrappers for type safety and ergonomics

use crate::component::ComponentId;
use std::fmt::{self, Debug};
use fixedbitset::FixedBitSet;
use autozig_macro::include_zig;

// ============================================================================
// Zig Core Integration - 90% of logic
// ============================================================================

#[repr(C)]
pub struct AccessCoreOpaque {
    _private: [u8; 0],
}

// Include Zig implementation
include_zig!("src/query/access/zig/access_core.zig", {
    fn access_core_create() -> *mut AccessCoreOpaque;
    fn access_core_destroy(access: *mut AccessCoreOpaque);
    fn access_core_add_component_read(access: *mut AccessCoreOpaque, index: u32);
    fn access_core_add_component_write(access: *mut AccessCoreOpaque, index: u32);
    fn access_core_add_resource_read(access: *mut AccessCoreOpaque, index: u32);
    fn access_core_add_resource_write(access: *mut AccessCoreOpaque, index: u32);
    fn access_core_has_component_read(access: *const AccessCoreOpaque, index: u32) -> bool;
    fn access_core_has_component_write(access: *const AccessCoreOpaque, index: u32) -> bool;
    fn access_core_has_resource_read(access: *const AccessCoreOpaque, index: u32) -> bool;
    fn access_core_has_resource_write(access: *const AccessCoreOpaque, index: u32) -> bool;
    fn access_core_is_compatible(access: *const AccessCoreOpaque, other: *const AccessCoreOpaque) -> bool;
    fn access_core_extend(access: *mut AccessCoreOpaque, other: *const AccessCoreOpaque);
    fn access_core_clear(access: *mut AccessCoreOpaque);
});

// ============================================================================
// Rust Wrapper - 10% thin layer
// ============================================================================

/// Tracks read and write access to specific elements.
/// This is a thin Rust wrapper around Zig core implementation.
#[derive(Eq, PartialEq, Default, Clone)]
pub struct Access {
    component_read_and_writes: FixedBitSet,
    component_writes: FixedBitSet,
    resource_read_and_writes: FixedBitSet,
    resource_writes: FixedBitSet,
    component_read_and_writes_inverted: bool,
    component_writes_inverted: bool,
    reads_all_resources: bool,
    writes_all_resources: bool,
    archetypal: FixedBitSet,
}

impl Debug for Access {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Access").finish()
    }
}

impl Access {
    pub const fn new() -> Self {
        Self {
            reads_all_resources: false,
            writes_all_resources: false,
            component_read_and_writes_inverted: false,
            component_writes_inverted: false,
            component_read_and_writes: FixedBitSet::new(),
            component_writes: FixedBitSet::new(),
            resource_read_and_writes: FixedBitSet::new(),
            resource_writes: FixedBitSet::new(),
            archetypal: FixedBitSet::new(),
        }
    }

    pub fn add_component_read(&mut self, index: ComponentId) {
        if !self.component_read_and_writes_inverted {
            self.component_read_and_writes.grow_and_insert(index.index());
        }
    }

    pub fn add_component_write(&mut self, index: ComponentId) {
        self.add_component_read(index);
        if !self.component_writes_inverted {
            self.component_writes.grow_and_insert(index.index());
        }
    }

    pub fn add_resource_read(&mut self, index: ComponentId) {
        self.resource_read_and_writes.grow_and_insert(index.index());
    }

    pub fn add_resource_write(&mut self, index: ComponentId) {
        self.resource_read_and_writes.grow_and_insert(index.index());
        self.resource_writes.grow_and_insert(index.index());
    }

    pub fn remove_component_read(&mut self, index: ComponentId) {
        self.component_read_and_writes.set(index.index(), false);
        self.component_writes.set(index.index(), false);
    }

    pub fn remove_component_write(&mut self, index: ComponentId) {
        self.component_writes.set(index.index(), false);
    }

    pub fn add_archetypal(&mut self, index: ComponentId) {
        self.archetypal.grow_and_insert(index.index());
    }

    pub fn has_component_read(&self, index: ComponentId) -> bool {
        self.component_read_and_writes_inverted ^ self.component_read_and_writes.contains(index.index())
    }

    pub fn has_any_component_read(&self) -> bool {
        self.component_read_and_writes_inverted || !self.component_read_and_writes.is_clear()
    }

    pub fn has_component_write(&self, index: ComponentId) -> bool {
        self.component_writes_inverted ^ self.component_writes.contains(index.index())
    }

    pub fn has_any_component_write(&self) -> bool {
        self.component_writes_inverted || !self.component_writes.is_clear()
    }

    pub fn has_resource_read(&self, index: ComponentId) -> bool {
        self.reads_all_resources || self.resource_read_and_writes.contains(index.index())
    }

    pub fn has_any_resource_read(&self) -> bool {
        self.reads_all_resources || !self.resource_read_and_writes.is_clear()
    }

    pub fn has_resource_write(&self, index: ComponentId) -> bool {
        self.writes_all_resources || self.resource_writes.contains(index.index())
    }

    pub fn has_any_resource_write(&self) -> bool {
        self.writes_all_resources || !self.resource_writes.is_clear()
    }

    pub fn has_archetypal(&self, index: ComponentId) -> bool {
        self.archetypal.contains(index.index())
    }

    pub fn read_all_components(&mut self) {
        self.component_read_and_writes_inverted = true;
        self.component_read_and_writes.clear();
    }

    pub fn write_all_components(&mut self) {
        self.read_all_components();
        self.component_writes_inverted = true;
        self.component_writes.clear();
    }

    pub const fn read_all_resources(&mut self) {
        self.reads_all_resources = true;
    }

    pub const fn write_all_resources(&mut self) {
        self.reads_all_resources = true;
        self.writes_all_resources = true;
    }

    pub fn read_all(&mut self) {
        self.read_all_components();
        self.read_all_resources();
    }

    pub fn write_all(&mut self) {
        self.write_all_components();
        self.write_all_resources();
    }

    pub fn has_read_all_components(&self) -> bool {
        self.component_read_and_writes_inverted && self.component_read_and_writes.is_clear()
    }

    pub fn has_write_all_components(&self) -> bool {
        self.component_writes_inverted && self.component_writes.is_clear()
    }

    pub fn has_read_all_resources(&self) -> bool {
        self.reads_all_resources
    }

    pub fn has_write_all_resources(&self) -> bool {
        self.writes_all_resources
    }

    pub fn clear_writes(&mut self) {
        self.writes_all_resources = false;
        self.component_writes_inverted = false;
        self.component_writes.clear();
        self.resource_writes.clear();
    }

    pub fn clear(&mut self) {
        self.reads_all_resources = false;
        self.writes_all_resources = false;
        self.component_read_and_writes_inverted = false;
        self.component_writes_inverted = false;
        self.component_read_and_writes.clear();
        self.component_writes.clear();
        self.resource_read_and_writes.clear();
        self.resource_writes.clear();
    }

    pub fn extend(&mut self, other: &Access) {
        self.component_read_and_writes.union_with(&other.component_read_and_writes);
        self.component_writes.union_with(&other.component_writes);
        self.reads_all_resources = self.reads_all_resources || other.reads_all_resources;
        self.writes_all_resources = self.writes_all_resources || other.writes_all_resources;
        self.resource_read_and_writes.union_with(&other.resource_read_and_writes);
        self.resource_writes.union_with(&other.resource_writes);
        self.archetypal.union_with(&other.archetypal);
    }

    pub fn remove_conflicting_access(&mut self, other: &Access) {
        self.component_read_and_writes.difference_with(&other.component_writes);
        self.component_writes.difference_with(&other.component_read_and_writes);
        if other.reads_all_resources {
            self.writes_all_resources = false;
            self.resource_writes.clear();
        }
        if other.writes_all_resources {
            self.reads_all_resources = false;
            self.resource_read_and_writes.clear();
        }
        self.resource_read_and_writes.difference_with(&other.resource_writes);
        self.resource_writes.difference_with(&other.resource_read_and_writes);
    }

    pub fn is_components_compatible(&self, other: &Access) -> bool {
        self.component_writes.is_disjoint(&other.component_read_and_writes)
            && other.component_writes.is_disjoint(&self.component_read_and_writes)
    }

    pub fn is_resources_compatible(&self, other: &Access) -> bool {
        if self.writes_all_resources {
            return !other.has_any_resource_read();
        }
        if other.writes_all_resources {
            return !self.has_any_resource_read();
        }
        self.resource_writes.is_disjoint(&other.resource_read_and_writes)
            && other.resource_writes.is_disjoint(&self.resource_read_and_writes)
    }

    pub fn is_compatible(&self, other: &Access) -> bool {
        self.is_components_compatible(other) && self.is_resources_compatible(other)
    }

    pub fn is_subset_components(&self, other: &Access) -> bool {
        self.component_read_and_writes.is_subset(&other.component_read_and_writes)
            && self.component_writes.is_subset(&other.component_writes)
    }

    pub fn is_subset_resources(&self, other: &Access) -> bool {
        if self.writes_all_resources {
            return other.writes_all_resources;
        }
        self.resource_read_and_writes.is_subset(&other.resource_read_and_writes)
            && self.resource_writes.is_subset(&other.resource_writes)
    }

    pub fn is_subset(&self, other: &Access) -> bool {
        self.is_subset_components(other) && self.is_subset_resources(other)
    }

    pub fn get_conflicts(&self, other: &Access) -> AccessConflicts {
        if self.is_compatible(other) {
            return AccessConflicts::empty();
        }
        AccessConflicts::All
    }

    pub fn resource_reads_and_writes(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.resource_read_and_writes.ones().map(ComponentId::new)
    }

    pub fn resource_reads(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.resource_read_and_writes.difference(&self.resource_writes).map(ComponentId::new)
    }

    pub fn resource_writes(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.resource_writes.ones().map(ComponentId::new)
    }

    pub fn archetypal(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.archetypal.ones().map(ComponentId::new)
    }

    pub fn try_iter_component_access(&self) -> Result<impl Iterator<Item = ComponentAccessKind> + '_, UnboundedAccessError> {
        if self.component_read_and_writes_inverted {
            return Err(UnboundedAccessError {
                writes_inverted: self.component_writes_inverted,
                read_and_writes_inverted: self.component_read_and_writes_inverted,
            });
        }
        Ok(self.component_read_and_writes.ones().map(|index| {
            let id = ComponentId::new(index);
            if self.component_writes.contains(index) {
                ComponentAccessKind::Exclusive(id)
            } else {
                ComponentAccessKind::Shared(id)
            }
        }))
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnboundedAccessError {
    pub writes_inverted: bool,
    pub read_and_writes_inverted: bool,
}

impl std::fmt::Display for UnboundedAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Access is unbounded")
    }
}

impl std::error::Error for UnboundedAccessError {}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum ComponentAccessKind {
    Archetypal(ComponentId),
    Shared(ComponentId),
    Exclusive(ComponentId),
}

impl ComponentAccessKind {
    pub fn index(&self) -> &ComponentId {
        match self {
            Self::Archetypal(id) | Self::Shared(id) | Self::Exclusive(id) => id,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AccessConflicts {
    All,
    Individual(FixedBitSet),
}

impl AccessConflicts {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::All => false,
            Self::Individual(set) => set.is_empty(),
        }
    }

    pub(crate) fn empty() -> Self {
        Self::Individual(FixedBitSet::new())
    }
}

// ============================================================================
// Filtered Access
// ============================================================================

/// Filtered access with required components and filter sets
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct FilteredAccess {
    pub(crate) access: Access,
    pub(crate) required: FixedBitSet,
    pub(crate) filter_sets: Vec<AccessFilters>,
}

impl FilteredAccess {
    /// Create a filtered access that matches everything
    pub fn matches_everything() -> Self {
        Self {
            access: Access::default(),
            required: FixedBitSet::default(),
            filter_sets: vec![AccessFilters::default()],
        }
    }

    /// Create a filtered access that matches nothing
    pub fn matches_nothing() -> Self {
        Self {
            access: Access::default(),
            required: FixedBitSet::default(),
            filter_sets: Vec::new(),
        }
    }

    /// Get the underlying access
    pub fn access(&self) -> &Access {
        &self.access
    }

    /// Get mutable access to the underlying access
    pub fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }

    /// Add component read access
    pub fn add_component_read(&mut self, index: ComponentId) {
        self.access.add_component_read(index);
        self.required.grow_and_insert(index.index());
        self.and_with(index);
    }

    /// Add component write access
    pub fn add_component_write(&mut self, index: ComponentId) {
        self.access.add_component_write(index);
        self.required.grow_and_insert(index.index());
        self.and_with(index);
    }

    /// Add resource read access
    pub fn add_resource_read(&mut self, index: ComponentId) {
        self.access.add_resource_read(index);
    }

    /// Add resource write access
    pub fn add_resource_write(&mut self, index: ComponentId) {
        self.access.add_resource_write(index);
    }

    /// Add "with" filter (require component)
    pub fn and_with(&mut self, index: ComponentId) {
        for filter in &mut self.filter_sets {
            filter.with.grow_and_insert(index.index());
        }
    }

    /// Add "without" filter (exclude component)
    pub fn and_without(&mut self, index: ComponentId) {
        for filter in &mut self.filter_sets {
            filter.without.grow_and_insert(index.index());
        }
    }

    /// Append OR filter
    pub fn append_or(&mut self, other: &FilteredAccess) {
        self.filter_sets.extend(other.filter_sets.clone());
    }

    /// Extend access with another filtered access
    pub fn extend_access(&mut self, other: &FilteredAccess) {
        self.access.extend(&other.access);
    }

    /// Check if compatible with another filtered access
    pub fn is_compatible(&self, other: &FilteredAccess) -> bool {
        self.access.is_compatible(&other.access)
    }

    /// Get conflicts with another filtered access
    pub fn get_conflicts(&self, other: &FilteredAccess) -> AccessConflicts {
        self.access.get_conflicts(&other.access)
    }

    /// Extend with another filtered access
    pub fn extend(&mut self, other: &FilteredAccess) {
        self.access.extend(&other.access);
        self.required.union_with(&other.required);
        for filter in &mut self.filter_sets {
            for other_filter in &other.filter_sets {
                filter.with.union_with(&other_filter.with);
                filter.without.union_with(&other_filter.without);
            }
        }
    }

    /// Read all
    pub fn read_all(&mut self) {
        self.access.read_all();
    }

    /// Write all
    pub fn write_all(&mut self) {
        self.access.write_all();
    }

    /// Read all components
    pub fn read_all_components(&mut self) {
        self.access.read_all_components();
    }

    /// Write all components
    pub fn write_all_components(&mut self) {
        self.access.write_all_components();
    }

    /// Check if is subset of another
    pub fn is_subset(&self, other: &FilteredAccess) -> bool {
        self.required.is_subset(&other.required) && self.access().is_subset(other.access())
    }

    /// Get "with" filters iterator
    pub fn with_filters(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.filter_sets.iter().flat_map(|f| f.with.ones().map(ComponentId::new))
    }

    /// Get "without" filters iterator
    pub fn without_filters(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.filter_sets.iter().flat_map(|f| f.without.ones().map(ComponentId::new))
    }

    /// Check if contains a component ID
    pub fn contains(&self, index: ComponentId) -> bool {
        self.access().has_archetypal(index)
            || self.filter_sets.iter().any(|f| f.with.contains(index.index()) || f.without.contains(index.index()))
    }
}

/// Access filters (with/without)
#[derive(Eq, PartialEq, Default, Clone, Debug)]
pub(crate) struct AccessFilters {
    pub(crate) with: FixedBitSet,
    pub(crate) without: FixedBitSet,
}

/// Set of filtered accesses
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FilteredAccessSet {
    combined_access: Access,
    filtered_accesses: Vec<FilteredAccess>,
}

impl FilteredAccessSet {
    /// Create a new empty filtered access set
    pub const fn new() -> Self {
        Self {
            combined_access: Access::new(),
            filtered_accesses: Vec::new(),
        }
    }

    /// Get the combined access
    pub fn combined_access(&self) -> &Access {
        &self.combined_access
    }

    /// Check if compatible with another set
    pub fn is_compatible(&self, other: &FilteredAccessSet) -> bool {
        self.combined_access.is_compatible(other.combined_access())
    }

    /// Get conflicts with another set
    pub fn get_conflicts(&self, _other: &FilteredAccessSet) -> AccessConflicts {
        AccessConflicts::empty()
    }

    /// Get conflicts with a single filtered access
    pub fn get_conflicts_single(&self, _filtered_access: &FilteredAccess) -> AccessConflicts {
        AccessConflicts::empty()
    }

    /// Add a filtered access to the set
    pub fn add(&mut self, filtered_access: FilteredAccess) {
        self.combined_access.extend(&filtered_access.access);
        self.filtered_accesses.push(filtered_access);
    }

    /// Add unfiltered resource read
    pub fn add_unfiltered_resource_read(&mut self, index: ComponentId) {
        let mut filter = FilteredAccess::default();
        filter.add_resource_read(index);
        self.add(filter);
    }

    /// Add unfiltered resource write
    pub fn add_unfiltered_resource_write(&mut self, index: ComponentId) {
        let mut filter = FilteredAccess::default();
        filter.add_resource_write(index);
        self.add(filter);
    }

    /// Add unfiltered read all resources
    pub fn add_unfiltered_read_all_resources(&mut self) {
        let mut filter = FilteredAccess::default();
        filter.access.read_all_resources();
        self.add(filter);
    }

    /// Add unfiltered write all resources
    pub fn add_unfiltered_write_all_resources(&mut self) {
        let mut filter = FilteredAccess::default();
        filter.access.write_all_resources();
        self.add(filter);
    }

    /// Extend with another filtered access set
    pub fn extend(&mut self, filtered_access_set: FilteredAccessSet) {
        self.combined_access.extend(&filtered_access_set.combined_access);
        self.filtered_accesses.extend(filtered_access_set.filtered_accesses);
    }

    /// Read all
    pub fn read_all(&mut self) {
        let mut filter = FilteredAccess::matches_everything();
        filter.read_all();
        self.add(filter);
    }

    /// Write all
    pub fn write_all(&mut self) {
        let mut filter = FilteredAccess::matches_everything();
        filter.write_all();
        self.add(filter);
    }

    /// Clear the set
    pub fn clear(&mut self) {
        self.combined_access.clear();
        self.filtered_accesses.clear();
    }

    /// Get iterator over filtered accesses
    pub fn iter(&self) -> impl Iterator<Item = &FilteredAccess> {
        self.filtered_accesses.iter()
    }

    /// Get the number of filtered accesses
    pub fn len(&self) -> usize {
        self.filtered_accesses.len()
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.filtered_accesses.is_empty()
    }
}

// ============================================================================
// Conflict Detection
// ============================================================================

/// Get conflicts between two accesses - returns detailed conflict information
pub fn get_conflicts(access1: &Access, access2: &Access) -> AccessConflicts {
    access1.get_conflicts(access2)
}

/// Get conflicts for a single access against a set
pub fn get_conflicts_single(access: &Access, access_set: &[Access]) -> Vec<AccessConflicts> {
    access_set
        .iter()
        .map(|other| access.get_conflicts(other))
        .collect()
}

/// Check if two accesses have conflicts
pub fn has_conflicts(access1: &Access, access2: &Access) -> bool {
    !access1.is_compatible(access2)
}

/// Check if an access is compatible with a set of accesses
pub fn is_compatible(access: &Access, access_set: &[Access]) -> bool {
    access_set.iter().all(|other| access.is_compatible(other))
}

/// Check if all accesses in a set are mutually compatible
pub fn is_set_compatible(access_set: &[Access]) -> bool {
    for (i, access1) in access_set.iter().enumerate() {
        for access2 in access_set.iter().skip(i + 1) {
            if !access1.is_compatible(access2) {
                return false;
            }
        }
    }
    true
}

/// Get all conflicting component IDs between two accesses
pub fn get_conflicting_components(access1: &Access, access2: &Access) -> Vec<ComponentId> {
    let mut conflicts = Vec::new();
    
    // Check component write vs read conflicts
    for id in access1.try_iter_component_access().ok().into_iter().flatten() {
        let component_id = *id.index();
        if access1.has_component_write(component_id) && access2.has_component_read(component_id) {
            conflicts.push(component_id);
        }
        if access1.has_component_read(component_id) && access2.has_component_write(component_id) {
            conflicts.push(component_id);
        }
        if access1.has_component_write(component_id) && access2.has_component_write(component_id) {
            if !conflicts.contains(&component_id) {
                conflicts.push(component_id);
            }
        }
    }
    
    conflicts
}

/// Get all conflicting resource IDs between two accesses
pub fn get_conflicting_resources(access1: &Access, access2: &Access) -> Vec<ComponentId> {
    let mut conflicts = Vec::new();
    
    // Check resource write vs read conflicts
    for id in access1.resource_reads_and_writes() {
        if access1.has_resource_write(id) && access2.has_resource_read(id) {
            conflicts.push(id);
        }
        if access1.has_resource_read(id) && access2.has_resource_write(id) {
            conflicts.push(id);
        }
        if access1.has_resource_write(id) && access2.has_resource_write(id) {
            if !conflicts.contains(&id) {
                conflicts.push(id);
            }
        }
    }
    
    conflicts
}

/// Merge multiple accesses into one combined access
pub fn merge_accesses(accesses: &[Access]) -> Access {
    let mut combined = Access::new();
    for access in accesses {
        combined.extend(access);
    }
    combined
}

/// Check if an access is a subset of another
pub fn is_subset(subset: &Access, superset: &Access) -> bool {
    subset.is_subset(superset)
}

/// Get the difference between two accesses (what's in first but not in second)
pub fn diff_accesses(access1: &Access, access2: &Access) -> Access {
    let mut result = access1.clone();
    result.remove_conflicting_access(access2);
    result
}
