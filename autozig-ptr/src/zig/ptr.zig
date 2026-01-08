const std = @import("std");

// ============================================================================
// TYPE DEFINITIONS
// ============================================================================

/// Opaque pointer type (equivalent to Rust's Ptr)
pub const BevyPtr = extern struct {
    ptr: [*c]u8,
    _phantom: u8 = 0,
};

/// Mutable opaque pointer (equivalent to Rust's PtrMut)
pub const BevyPtrMut = extern struct {
    ptr: [*c]u8,
    _phantom: u8 = 0,
};

/// Owning pointer (equivalent to Rust's OwningPtr)
pub const BevyOwningPtr = extern struct {
    ptr: [*c]u8,
    _phantom: u8 = 0,
};

// ============================================================================
// PTR OPERATIONS
// ============================================================================

export fn bevy_ptr_new(ptr: [*c]u8) BevyPtr {
    return BevyPtr{ .ptr = ptr };
}

export fn bevy_ptr_as_ptr(self: BevyPtr) [*c]u8 {
    return self.ptr;
}

export fn bevy_ptr_byte_offset(self: BevyPtr, offset: isize) BevyPtr {
    const new_ptr = if (offset >= 0)
        self.ptr + @as(usize, @intCast(offset))
    else
        self.ptr - @as(usize, @intCast(-offset));
    return BevyPtr{ .ptr = new_ptr };
}

export fn bevy_ptr_byte_add(self: BevyPtr, count: usize) BevyPtr {
    return BevyPtr{ .ptr = self.ptr + count };
}

export fn bevy_ptr_deref(self: BevyPtr, size: usize, output: [*c]u8) void {
    if (size > 0) {
        @memcpy(output[0..size], self.ptr[0..size]);
    }
}

// ============================================================================
// PTRMUT OPERATIONS
// ============================================================================

export fn bevy_ptrmut_new(ptr: [*c]u8) BevyPtrMut {
    return BevyPtrMut{ .ptr = ptr };
}

export fn bevy_ptrmut_as_ptr(self: BevyPtrMut) [*c]u8 {
    return self.ptr;
}

export fn bevy_ptrmut_byte_offset(self: BevyPtrMut, offset: isize) BevyPtrMut {
    const new_ptr = if (offset >= 0)
        self.ptr + @as(usize, @intCast(offset))
    else
        self.ptr - @as(usize, @intCast(-offset));
    return BevyPtrMut{ .ptr = new_ptr };
}

export fn bevy_ptrmut_byte_add(self: BevyPtrMut, count: usize) BevyPtrMut {
    return BevyPtrMut{ .ptr = self.ptr + count };
}

export fn bevy_ptrmut_deref_mut(self: BevyPtrMut, size: usize, output: [*c]u8) void {
    if (size > 0) {
        @memcpy(output[0..size], self.ptr[0..size]);
    }
}

export fn bevy_ptrmut_promote(self: BevyPtrMut) BevyOwningPtr {
    return BevyOwningPtr{ .ptr = self.ptr };
}

// ============================================================================
// OWNINGPTR OPERATIONS
// ============================================================================

export fn bevy_owningptr_new(ptr: [*c]u8) BevyOwningPtr {
    return BevyOwningPtr{ .ptr = ptr };
}

export fn bevy_owningptr_read(self: BevyOwningPtr, size: usize, output: [*c]u8) void {
    if (size > 0) {
        @memcpy(output[0..size], self.ptr[0..size]);
    }
}

export fn bevy_owningptr_as_ref(self: BevyOwningPtr) BevyPtr {
    return BevyPtr{ .ptr = self.ptr };
}

export fn bevy_owningptr_as_mut(self: BevyOwningPtr) BevyPtrMut {
    return BevyPtrMut{ .ptr = self.ptr };
}

export fn bevy_owningptr_as_ptr(self: BevyOwningPtr) [*c]u8 {
    return self.ptr;
}

// ============================================================================
// ISALIGNED TRAIT IMPLEMENTATIONS
// ============================================================================

/// Aligned read operation
export fn bevy_aligned_read_ptr(ptr: [*c]const u8, size: usize, output: [*c]u8) void {
    if (size > 0) {
        @memcpy(output[0..size], ptr[0..size]);
    }
}

/// Aligned copy non-overlapping operation
export fn bevy_aligned_copy_nonoverlapping(src: [*c]const u8, dst: [*c]u8, count: usize) void {
    if (count > 0) {
        @memcpy(dst[0..count], src[0..count]);
    }
}

/// Unaligned read operation
export fn bevy_unaligned_read_ptr(ptr: [*c]const u8, size: usize, output: [*c]u8) void {
    if (size > 0) {
        @memcpy(output[0..size], ptr[0..size]);
    }
}

/// Unaligned copy non-overlapping operation (byte-wise)
export fn bevy_unaligned_copy_nonoverlapping(src: [*c]const u8, dst: [*c]u8, count: usize) void {
    if (count > 0) {
        @memcpy(dst[0..count], src[0..count]);
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Check if pointer is null
export fn bevy_ptr_is_null(ptr: [*c]u8) bool {
    return ptr == null;
}

/// Create NonNull pointer (panics if null)
export fn bevy_nonnull_new(ptr: [*c]u8) [*c]u8 {
    if (ptr == null) @panic("Attempted to create NonNull from null pointer");
    return ptr;
}

/// Create zeroed memory
export fn bevy_mem_zeroed(output: [*c]u8, size: usize) void {
    if (size > 0) {
        @memset(output[0..size], 0);
    }
}

/// Get element from thin slice
export fn bevy_thin_slice_get(ptr: [*c]const u8, index: usize, elem_size: usize, output: [*c]u8) void {
    const offset = index * elem_size;
    if (elem_size > 0) {
        @memcpy(output[0..elem_size], (ptr + offset)[0..elem_size]);
    }
}

/// Debug check for alignment
export fn bevy_ptr_is_aligned(ptr: [*c]const u8, alignment: usize) bool {
    const addr = @intFromPtr(ptr);
    return (addr % alignment) == 0;
}

// ============================================================================
// MOVING PTR HELPER FUNCTIONS
// ============================================================================

/// Write from source to destination
export fn bevy_moving_write_to(src: [*c]const u8, dst: [*c]u8, size: usize) void {
    if (size > 0) {
        @memcpy(dst[0..size], src[0..size]);
    }
}

/// Read value from pointer
export fn bevy_moving_read(ptr: [*c]const u8, size: usize, output: [*c]u8) void {
    if (size > 0) {
        @memcpy(output[0..size], ptr[0..size]);
    }
}

// ============================================================================
// UNSAFE CELL OPERATIONS
// ============================================================================

/// Read from UnsafeCell
export fn bevy_unsafecell_read(ptr: [*c]const u8, size: usize, output: [*c]u8) void {
    if (size > 0) {
        @memcpy(output[0..size], ptr[0..size]);
    }
}

/// Deref UnsafeCell to immutable
export fn bevy_unsafecell_deref(ptr: [*c]const u8) [*c]const u8 {
    return ptr;
}

/// Deref UnsafeCell to mutable
export fn bevy_unsafecell_deref_mut(ptr: [*c]u8) [*c]u8 {
    return ptr;
}
