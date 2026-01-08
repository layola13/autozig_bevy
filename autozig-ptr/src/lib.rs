//! Type-erased pointers for safe low-level memory manipulation
//!
//! This crate provides type-erased pointer types that maintain safety invariants
//! while allowing operations on data without knowing its concrete type at compile time.

#![no_std]
#![allow(clippy::missing_safety_doc)]

use core::{
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter, Pointer},
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::{self, NonNull},
};

// Import Zig FFI functions using autozig's include_zig! macro
use autozig::include_zig;

include_zig!("src/zig/ptr.zig", {
    fn bevy_aligned_read_ptr(ptr: *const u8, size: usize, output: *mut u8);
    fn bevy_aligned_copy_nonoverlapping(src: *const u8, dst: *mut u8, count: usize);
    fn bevy_unaligned_read_ptr(ptr: *const u8, size: usize, output: *mut u8);
    fn bevy_unaligned_copy_nonoverlapping(src: *const u8, dst: *mut u8, count: usize);
});

#[inline]
fn read_aligned(ptr: *const u8, size: usize, output: *mut u8) {
    unsafe { bevy_aligned_read_ptr(ptr, size, output) };
}

#[inline]
fn copy_nonoverlapping(src: *const u8, dst: *mut u8, count: usize) {
    unsafe { bevy_aligned_copy_nonoverlapping(src, dst, count) };
}

#[inline]
fn read_unaligned(ptr: *const u8, size: usize, output: *mut u8) {
    unsafe { bevy_unaligned_read_ptr(ptr, size, output) };
}

#[inline]
fn copy_bytes(src: *const u8, dst: *mut u8, count: usize) {
    unsafe { bevy_unaligned_copy_nonoverlapping(src, dst, count) };
}

/// Alignment marker: pointer is guaranteed to be aligned
#[derive(Debug, Copy, Clone)]
pub struct Aligned;

/// Alignment marker: pointer may not be aligned
#[derive(Debug, Copy, Clone)]
pub struct Unaligned;

/// Trait implemented only for [`Aligned`] and [`Unaligned`]
pub trait IsAligned: sealed::Sealed {
    #[doc(hidden)]
    fn read_ptr<T>(ptr: *const T) -> T;
    
    #[doc(hidden)]
    fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
    
    #[doc(hidden)]
    fn drop_in_place<T>(ptr: *mut T);
}

impl IsAligned for Aligned {
    #[inline]
    fn read_ptr<T>(ptr: *const T) -> T {
        let size = mem::size_of::<T>();
        let mut result = MaybeUninit::<T>::uninit();
        read_aligned(ptr.cast::<u8>(), size, result.as_mut_ptr().cast::<u8>());
        unsafe { result.assume_init() }
    }

    #[inline]
    fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
        let byte_count = count * mem::size_of::<T>();
        copy_nonoverlapping(src.cast::<u8>(), dst.cast::<u8>(), byte_count);
    }

    #[inline]
    fn drop_in_place<T>(ptr: *mut T) {
        let value = Self::read_ptr(ptr);
        drop(value);
    }
}

impl IsAligned for Unaligned {
    #[inline]
    fn read_ptr<T>(ptr: *const T) -> T {
        let size = mem::size_of::<T>();
        let mut result = MaybeUninit::<T>::uninit();
        read_unaligned(ptr.cast::<u8>(), size, result.as_mut_ptr().cast::<u8>());
        unsafe { result.assume_init() }
    }

    #[inline]
    fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
        let byte_count = count * mem::size_of::<T>();
        copy_bytes(src.cast::<u8>(), dst.cast::<u8>(), byte_count);
    }

    #[inline]
    fn drop_in_place<T>(ptr: *mut T) {
        let value = Self::read_ptr(ptr);
        drop(value);
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Aligned {}
    impl Sealed for super::Unaligned {}
}

/// Type-erased immutable borrow
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Ptr<'a, A: IsAligned = Aligned>(NonNull<u8>, PhantomData<(&'a u8, A)>);

/// Type-erased mutable borrow
#[repr(transparent)]
pub struct PtrMut<'a, A: IsAligned = Aligned>(NonNull<u8>, PhantomData<(&'a mut u8, A)>);

/// Type-erased owning pointer (like Box but doesn't free memory)
#[repr(transparent)]
pub struct OwningPtr<'a, A: IsAligned = Aligned>(NonNull<u8>, PhantomData<(&'a mut u8, A)>);

/// Pointer for moving values
#[repr(transparent)]
pub struct MovingPtr<'a, T, A: IsAligned = Aligned>(NonNull<T>, PhantomData<(&'a mut T, A)>);

macro_rules! impl_ptr {
    ($ptr:ident) => {
        impl<'a> $ptr<'a, Aligned> {
            pub fn to_unaligned(self) -> $ptr<'a, Unaligned> {
                $ptr(self.0, PhantomData)
            }
        }

        impl<'a, A: IsAligned> From<$ptr<'a, A>> for NonNull<u8> {
            fn from(ptr: $ptr<'a, A>) -> Self {
                ptr.0
            }
        }

        impl<A: IsAligned> $ptr<'_, A> {
            #[inline]
            pub fn byte_offset(self, count: isize) -> Self {
                Self(
                    unsafe { NonNull::new_unchecked(self.as_ptr().offset(count)) },
                    PhantomData,
                )
            }

            #[inline]
            pub fn byte_add(self, count: usize) -> Self {
                Self(
                    unsafe { NonNull::new_unchecked(self.as_ptr().add(count)) },
                    PhantomData,
                )
            }
        }

        impl<A: IsAligned> Pointer for $ptr<'_, A> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Pointer::fmt(&self.0, f)
            }
        }

        impl Debug for $ptr<'_, Aligned> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}<Aligned>({:?})", stringify!($ptr), self.0)
            }
        }

        impl Debug for $ptr<'_, Unaligned> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}<Unaligned>({:?})", stringify!($ptr), self.0)
            }
        }
    };
}

impl_ptr!(Ptr);
impl_ptr!(PtrMut);
impl_ptr!(OwningPtr);

impl<'a, T> MovingPtr<'a, T, Aligned> {
    #[inline]
    pub fn to_unaligned(self) -> MovingPtr<'a, T, Unaligned> {
        let value = MovingPtr(self.0, PhantomData);
        mem::forget(self);
        value
    }

    #[inline]
    pub fn from_value(value: &'a mut MaybeUninit<T>) -> Self {
        MovingPtr(NonNull::from(value).cast::<T>(), PhantomData)
    }
}

impl<'a, T, A: IsAligned> MovingPtr<'a, T, A> {
    #[inline]
    pub fn new(inner: NonNull<T>) -> Self {
        Self(inner, PhantomData)
    }

    #[inline]
    pub fn read(self) -> T {
        let value = A::read_ptr(self.0.as_ptr());
        mem::forget(self);
        value
    }

    #[inline]
    pub fn write_to(self, dst: *mut T) {
        let src = self.0.as_ptr();
        mem::forget(self);
        A::copy_nonoverlapping(src, dst, 1);
    }

    #[inline]
    pub fn assign_to(self, dst: &mut T) {
        unsafe { ptr::drop_in_place(dst) };
        self.write_to(dst);
    }
}

impl<T, A: IsAligned> Pointer for MovingPtr<'_, T, A> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Pointer::fmt(&self.0, f)
    }
}

impl<T> Debug for MovingPtr<'_, T, Aligned> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MovingPtr<Aligned>({:?})", self.0)
    }
}

impl<T> Debug for MovingPtr<'_, T, Unaligned> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MovingPtr<Unaligned>({:?})", self.0)
    }
}

impl<'a, T, A: IsAligned> From<MovingPtr<'a, T, A>> for OwningPtr<'a, A> {
    #[inline]
    fn from(value: MovingPtr<'a, T, A>) -> Self {
        let ptr = OwningPtr::new(value.0.cast::<u8>());
        mem::forget(value);
        ptr
    }
}

impl<T, A: IsAligned> Drop for MovingPtr<'_, T, A> {
    fn drop(&mut self) {
        A::drop_in_place(self.0.as_ptr());
    }
}

impl<'a, A: IsAligned> Ptr<'a, A> {
    #[inline]
    pub fn new(inner: NonNull<u8>) -> Self {
        Self(inner, PhantomData)
    }

    #[inline]
    pub fn assert_unique(self) -> PtrMut<'a, A> {
        PtrMut(self.0, PhantomData)
    }

    #[inline]
    pub fn deref<T>(self) -> &'a T {
        let ptr = self.as_ptr().cast::<T>();
        unsafe { &*ptr }
    }

    #[inline]
    pub fn as_ptr(self) -> *mut u8 {
        self.0.as_ptr()
    }
}

impl<'a, T: ?Sized> From<&'a T> for Ptr<'a> {
    #[inline]
    fn from(val: &'a T) -> Self {
        Ptr::new(NonNull::from(val).cast())
    }
}

impl<'a, A: IsAligned> PtrMut<'a, A> {
    #[inline]
    pub fn new(inner: NonNull<u8>) -> Self {
        Self(inner, PhantomData)
    }

    #[inline]
    pub fn promote(self) -> OwningPtr<'a, A> {
        OwningPtr(self.0, PhantomData)
    }

    #[inline]
    pub fn deref_mut<T>(self) -> &'a mut T {
        let ptr = self.as_ptr().cast::<T>();
        unsafe { &mut *ptr }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn reborrow(&mut self) -> PtrMut<'_, A> {
        PtrMut::new(self.0)
    }

    #[inline]
    pub fn as_ref(&self) -> Ptr<'_, A> {
        Ptr::new(self.0)
    }
}

impl<'a, T: ?Sized> From<&'a mut T> for PtrMut<'a> {
    #[inline]
    fn from(val: &'a mut T) -> Self {
        PtrMut::new(NonNull::from(val).cast())
    }
}

impl<'a> OwningPtr<'a> {
    fn make_internal<T>(temp: &mut ManuallyDrop<T>) -> OwningPtr<'_> {
        PtrMut::from(&mut **temp).promote()
    }

    #[inline]
    pub fn make<T, F: FnOnce(OwningPtr<'_>) -> R, R>(val: T, f: F) -> R {
        let mut val = ManuallyDrop::new(val);
        f(Self::make_internal(&mut val))
    }
}

impl<'a, A: IsAligned> OwningPtr<'a, A> {
    #[inline]
    pub fn new(inner: NonNull<u8>) -> Self {
        Self(inner, PhantomData)
    }

    #[inline]
    pub fn read<T>(self) -> T {
        let ptr = self.as_ptr().cast::<T>();
        unsafe { ptr.read() }
    }

    #[inline]
    pub fn cast<T>(self) -> MovingPtr<'a, T, A> {
        MovingPtr(self.0.cast::<T>(), PhantomData)
    }

    #[inline]
    pub fn drop_as<T>(self) {
        let ptr = self.as_ptr().cast::<T>();
        unsafe { ptr.drop_in_place() };
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_ref(&self) -> Ptr<'_, A> {
        Ptr::new(self.0)
    }

    #[inline]
    pub fn as_mut(&mut self) -> PtrMut<'_, A> {
        PtrMut::new(self.0)
    }
}

impl<'a> OwningPtr<'a, Unaligned> {
    pub fn read_unaligned<T>(self) -> T {
        let ptr = self.as_ptr().cast::<T>();
        unsafe { ptr.read_unaligned() }
    }
}

/// Slice pointer without length information
pub struct ThinSlicePtr<'a, T> {
    ptr: NonNull<T>,
    #[cfg(debug_assertions)]
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

impl<'a, T> ThinSlicePtr<'a, T> {
    #[inline]
    pub fn get_unchecked(&self, index: usize) -> &'a T {
        #[cfg(debug_assertions)]
        assert!(index < self.len, "index out of bounds");
        
        unsafe { &*self.ptr.as_ptr().add(index) }
    }
}

impl<'a, T> Clone for ThinSlicePtr<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for ThinSlicePtr<'a, T> {}

impl<'a, T> From<&'a [T]> for ThinSlicePtr<'a, T> {
    #[inline]
    fn from(slice: &'a [T]) -> Self {
        let ptr = slice.as_ptr().cast_mut();
        
        Self {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            #[cfg(debug_assertions)]
            len: slice.len(),
            _marker: PhantomData,
        }
    }
}

mod private {
    use core::cell::UnsafeCell;
    
    pub trait SealedUnsafeCell {}
    impl<'a, T> SealedUnsafeCell for &'a UnsafeCell<T> {}
}

/// Extension trait for [`UnsafeCell`]
pub trait UnsafeCellDeref<'a, T>: private::SealedUnsafeCell {
    fn deref_mut(self) -> &'a mut T;
    fn deref(self) -> &'a T;
    fn read(self) -> T where T: Copy;
}

impl<'a, T> UnsafeCellDeref<'a, T> for &'a UnsafeCell<T> {
    #[inline]
    fn deref_mut(self) -> &'a mut T {
        unsafe { &mut *self.get() }
    }
    
    #[inline]
    fn deref(self) -> &'a T {
        unsafe { &*self.get() }
    }

    #[inline]
    fn read(self) -> T where T: Copy {
        unsafe { self.get().read() }
    }
}