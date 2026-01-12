//! World identifier - 90% Zig + 10% Rust架构
//!
//! WorldId provides unique identification for World instances,
//! enabling safe multi-world scenarios and world access validation.

use autozig_macro::include_zig;
use std::sync::atomic::{AtomicU32, Ordering};

// Zig核心实现 - WorldId的底层操作
include_zig!("src/world/zig/world_id.zig", {
    fn world_id_create(index: u32) -> WorldId;
    fn world_id_index(id: WorldId) -> u32;
    fn world_id_equals(a: WorldId, b: WorldId) -> bool;
});

/// Static counter for generating unique WorldIds
static WORLD_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Unique identifier for a World instance
///
/// Each World gets a unique ID when created, allowing:
/// - Safe validation of world access
/// - Multi-world scenarios
/// - World-specific resource management
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldId {
    index: u32,
}

impl WorldId {
    /// Creates a new unique WorldId
    ///
    /// # Errors
    /// Returns None if WorldId counter has wrapped (extremely unlikely)
    pub fn new() -> Option<Self> {
        let index = WORLD_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        if index == u32::MAX {
            None // Overflow protection
        } else {
            Some(world_id_create(index))
        }
    }

    /// Creates a WorldId from a raw index (unsafe - for testing only)
    ///
    /// # Safety
    /// Caller must ensure the index is valid and doesn't conflict with existing WorldIds
    pub const unsafe fn from_raw(index: u32) -> Self {
        Self { index }
    }

    /// Returns the raw index of this WorldId
    #[inline]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// Returns the raw index (alternative method matching Bevy API)
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.index
    }

    /// Checks if two WorldIds are equal using Zig implementation
    #[inline]
    pub fn equals(self, other: Self) -> bool {
        world_id_equals(self, other)
    }
}

impl std::fmt::Display for WorldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorldId({})", self.index)
    }
}

// Implement Default to match Bevy (though typically not used)
impl Default for WorldId {
    fn default() -> Self {
        // Default WorldId with index 0 (usually should use new() instead)
        Self { index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_id_uniqueness() {
        let id1 = WorldId::new().unwrap();
        let id2 = WorldId::new().unwrap();
        assert_ne!(id1, id2);
        assert!(id1.index() < id2.index());
    }

    #[test]
    fn world_id_equality() {
        let id1 = WorldId::new().unwrap();
        let id2 = id1;
        assert!(id1.equals(id2));
        assert_eq!(id1, id2);
    }

    #[test]
    fn world_id_display() {
        let id = unsafe { WorldId::from_raw(42) };
        assert_eq!(format!("{}", id), "WorldId(42)");
    }
}