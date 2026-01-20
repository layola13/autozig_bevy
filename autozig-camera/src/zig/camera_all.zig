//! AutoZig Camera - Main entry point
//! Exports all camera functionality for FFI

// Re-export all public modules
pub const projection = @import("projection.zig");
pub const view = @import("view.zig");
pub const frustum = @import("frustum.zig");

// Re-export main types
pub const Frustum = frustum.Frustum;
pub const Plane = frustum.Plane;

// Force inclusion of exported functions
comptime {
    _ = projection;
    _ = view;
    _ = frustum;
}
