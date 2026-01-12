//! Type-erased pointer types for safe component manipulation
//!
//! This module provides safe wrappers around raw pointers for manipulating
//! components in a type-erased manner, commonly needed in ECS operations.

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// A type-erased immutable pointer to some unknown type.
///
/// This pointer is guaranteed to be valid and properly aligned for the type it points to.
/// It is used when you need to pass around a reference to a component without knowing
/// its concrete type at compile time.
///
/// # Safety
///
/// - The pointer must be valid for reads of the pointee type
/// - The pointer must be properly aligned
/// - The pointer must point to an initialized value
pub struct Ptr<'a> {
    ptr: NonNull<u8>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Ptr<'a> {
    /// Creates a new `Ptr` from a raw pointer.
    ///
    /// # Safety
    ///
    /// - `ptr` must be non-null, properly aligned, and valid for reads
    /// - `ptr` must point to an initialized value of type `T`
    /// - The pointer must remain valid for the lifetime `'a`
    #[inline]
    pub unsafe fn new(ptr: NonNull<u8>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a `Ptr` from a reference.
    #[inline]
    pub fn from_ref<T>(value: &'a T) -> Self {
        // SAFETY: References are always non-null, properly aligned, and valid
        unsafe { Self::new(NonNull::from(value).cast()) }
    }

    /// Dereferences the pointer to a concrete type.
    ///
    /// # Safety
    ///
    /// - The pointer must point to a valid instance of `T`
    /// - `T` must be the correct type that this pointer was created from
    #[inline]
    pub unsafe fn deref<T>(self) -> &'a T {
        // SAFETY: Caller ensures the pointer is valid for type T
        unsafe { &*self.ptr.cast::<T>().as_ptr() }
    }

    /// Gets the raw pointer.
    #[inline]
    pub fn as_ptr(self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

impl<'a> Clone for Ptr<'a> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Ptr<'a> {}

// Debug implementation that just shows the pointer address
impl<'a> std::fmt::Debug for Ptr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ptr")
            .field("ptr", &self.ptr)
            .finish()
    }
}

/// A type-erased mutable pointer with unique ownership of some unknown type.
///
/// This pointer has unique ownership semantics and is used for writing to
/// components in a type-erased manner. It is typically used when initializing
/// or modifying component data.
///
/// # Safety
///
/// - The pointer must be valid for both reads and writes
/// - The pointer must be properly aligned
/// - The pointer must have unique access (no aliasing mutable references)
pub struct OwningPtr<'a> {
    ptr: NonNull<u8>,
    _marker: PhantomData<&'a mut ()>,
}

impl<'a> OwningPtr<'a> {
    /// Creates a new `OwningPtr` from a raw pointer.
    ///
    /// # Safety
    ///
    /// - `ptr` must be non-null, properly aligned, and valid for reads and writes
    /// - The pointer must have unique access (no aliasing)
    /// - The pointer must remain valid for the lifetime `'a`
    #[inline]
    pub unsafe fn new(ptr: NonNull<u8>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Creates an `OwningPtr` from a mutable reference.
    #[inline]
    pub fn from_mut<T>(value: &'a mut T) -> Self {
        // SAFETY: Mutable references guarantee unique access and validity
        unsafe { Self::new(NonNull::from(value).cast()) }
    }

    /// Writes a value to the pointer.
    ///
    /// # Safety
    ///
    /// - The pointer must point to valid memory for type `T`
    /// - `T` must be the correct type that this pointer was created for
    /// - The memory may be uninitialized; this will initialize it
    /// - Any existing value will be overwritten without dropping
    #[inline]
    pub unsafe fn write<T>(self, value: T) {
        // SAFETY: Caller ensures the pointer is valid for type T
        unsafe {
            self.ptr.cast::<T>().as_ptr().write(value);
        }
    }

    /// Reads the value from the pointer.
    ///
    /// # Safety
    ///
    /// - The pointer must point to a valid, initialized instance of `T`
    /// - `T` must be the correct type that this pointer was created from
    /// - This moves the value out, leaving the memory uninitialized
    #[inline]
    pub unsafe fn read<T>(self) -> T {
        // SAFETY: Caller ensures the pointer is valid and initialized
        unsafe { self.ptr.cast::<T>().as_ptr().read() }
    }

    /// Dereferences the pointer to a concrete type.
    ///
    /// # Safety
    ///
    /// - The pointer must point to a valid, initialized instance of `T`
    /// - `T` must be the correct type that this pointer was created from
    #[inline]
    pub unsafe fn deref_mut<T>(self) -> &'a mut T {
        // SAFETY: Caller ensures the pointer is valid for type T
        unsafe { &mut *self.ptr.cast::<T>().as_ptr() }
    }

    /// Promote this `OwningPtr` to a `PtrMut`.
    ///
    /// # Safety
    ///
    /// - The pointer must point to a valid, initialized value
    #[inline]
    pub unsafe fn promote(self) -> PtrMut<'a> {
        // SAFETY: OwningPtr guarantees are sufficient for PtrMut
        unsafe { PtrMut::new(self.ptr) }
    }

    /// Gets the raw pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Converts this `OwningPtr` into a `Ptr`.
    #[inline]
    pub fn into_ptr(self) -> Ptr<'a> {
        // SAFETY: OwningPtr is always valid for reads
        unsafe { Ptr::new(self.ptr) }
    }
}

// Debug implementation
impl<'a> std::fmt::Debug for OwningPtr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwningPtr")
            .field("ptr", &self.ptr)
            .finish()
    }
}

/// A type-erased mutable pointer to some unknown type.
///
/// Similar to `OwningPtr` but without unique ownership guarantees.
/// Used when you need mutable access but not necessarily unique access.
///
/// # Safety
///
/// - The pointer must be valid for both reads and writes
/// - The pointer must be properly aligned
/// - Caller must ensure proper aliasing rules are followed
pub struct PtrMut<'a> {
    ptr: NonNull<u8>,
    _marker: PhantomData<&'a mut ()>,
}

impl<'a> PtrMut<'a> {
    /// Creates a new `PtrMut` from a raw pointer.
    ///
    /// # Safety
    ///
    /// - `ptr` must be non-null, properly aligned, and valid for reads and writes
    /// - The pointer must remain valid for the lifetime `'a`
    /// - Caller must ensure proper aliasing
    #[inline]
    pub unsafe fn new(ptr: NonNull<u8>) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// Creates a `PtrMut` from a mutable reference.
    #[inline]
    pub fn from_mut<T>(value: &'a mut T) -> Self {
        // SAFETY: Mutable references are always valid
        unsafe { Self::new(NonNull::from(value).cast()) }
    }

    /// Dereferences the pointer to a concrete type.
    ///
    /// # Safety
    ///
    /// - The pointer must point to a valid, initialized instance of `T`
    /// - `T` must be the correct type that this pointer was created from
    #[inline]
    pub unsafe fn deref_mut<T>(self) -> &'a mut T {
        // SAFETY: Caller ensures the pointer is valid for type T
        unsafe { &mut *self.ptr.cast::<T>().as_ptr() }
    }

    /// Gets an immutable view of this pointer.
    #[inline]
    pub fn as_ref(self) -> Ptr<'a> {
        // SAFETY: PtrMut is always valid for reads
        unsafe { Ptr::new(self.ptr) }
    }

    /// Gets the raw pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }
}

// Debug implementation
impl<'a> std::fmt::Debug for PtrMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtrMut")
            .field("ptr", &self.ptr)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_from_ref() {
        let value = 42i32;
        let ptr = Ptr::from_ref(&value);
        unsafe {
            assert_eq!(*ptr.deref::<i32>(), 42);
        }
    }

    #[test]
    fn test_owning_ptr_write_read() {
        let mut storage = 0i32;
        let ptr = OwningPtr::from_mut(&mut storage);
        unsafe {
            ptr.write(42i32);
        }
        assert_eq!(storage, 42);
    }

    #[test]
    fn test_ptr_mut_deref() {
        let mut value = 10i32;
        let ptr = PtrMut::from_mut(&mut value);
        unsafe {
            *ptr.deref_mut::<i32>() = 20;
        }
        assert_eq!(value, 20);
    }
}