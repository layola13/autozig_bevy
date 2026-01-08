//! Comprehensive tests for autozig-ptr
//! Tests all pointer types and operations

use autozig_ptr::{Ptr, PtrMut, OwningPtr, MovingPtr, ThinSlicePtr};
use core::mem::MaybeUninit;

#[test]
fn test_ptr_basic() {
    let value = 42u32;
    let ptr = Ptr::from(&value);
    
    // Test deref
    let deref_value: &u32 = ptr.deref();
    assert_eq!(*deref_value, 42);
}

#[test]
fn test_ptr_byte_operations() {
    let values = [1u32, 2, 3, 4];
    let ptr = Ptr::from(&values[0]);
    
    // Test byte_add
    let ptr2 = ptr.byte_add(core::mem::size_of::<u32>());
    let value2: &u32 = ptr2.deref();
    assert_eq!(*value2, 2);
    
    // Test byte_offset
    let ptr3 = ptr.byte_offset(core::mem::size_of::<u32>() as isize * 2);
    let value3: &u32 = ptr3.deref();
    assert_eq!(*value3, 3);
}

#[test]
fn test_ptrmut_basic() {
    let mut value = 42u32;
    let ptr = PtrMut::from(&mut value);
    
    // Test deref_mut
    let deref_value: &mut u32 = ptr.deref_mut();
    *deref_value = 100;
    
    assert_eq!(value, 100);
}

#[test]
fn test_ptrmut_reborrow() {
    let mut value = 42u32;
    let mut ptr = PtrMut::from(&mut value);
    
    {
        let ptr2 = ptr.reborrow();
        let val: &mut u32 = ptr2.deref_mut();
        *val = 100;
    }
    
    assert_eq!(value, 100);
}

#[test]
fn test_ptrmut_as_ref() {
    let mut value = 42u32;
    let ptr = PtrMut::from(&mut value);
    
    let ptr_ref = ptr.as_ref();
    let val: &u32 = ptr_ref.deref();
    assert_eq!(*val, 42);
}

#[test]
fn test_owningptr_make() {
    let result = OwningPtr::make(42u32, |ptr| {
        let value: u32 = ptr.read();
        value * 2
    });
    
    assert_eq!(result, 84);
}

#[test]
fn test_owningptr_cast() {
    OwningPtr::make(42u32, |ptr| {
        let moving: MovingPtr<u32> = ptr.cast();
        let value = moving.read();
        assert_eq!(value, 42);
    });
}

#[test]
fn test_movingptr_read() {
    let mut value = MaybeUninit::new(42u32);
    let ptr = MovingPtr::from_value(&mut value);
    
    let result = ptr.read();
    assert_eq!(result, 42);
}

#[test]
fn test_movingptr_write_to() {
    let mut value = MaybeUninit::new(42u32);
    let ptr = MovingPtr::from_value(&mut value);
    
    let mut target = 0u32;
    ptr.write_to(&mut target);
    
    assert_eq!(target, 42);
}

#[test]
fn test_movingptr_assign_to() {
    let mut value = MaybeUninit::new(100u32);
    let ptr = MovingPtr::from_value(&mut value);
    
    let mut target = 42u32;
    ptr.assign_to(&mut target);
    
    assert_eq!(target, 100);
}

#[test]
fn test_thin_slice_ptr() {
    let values = [1u32, 2, 3, 4, 5];
    let thin = ThinSlicePtr::from(&values[..]);
    
    // Test get_unchecked
    assert_eq!(*thin.get_unchecked(0), 1);
    assert_eq!(*thin.get_unchecked(2), 3);
    assert_eq!(*thin.get_unchecked(4), 5);
}

#[test]
fn test_aligned_unaligned_conversion() {
    let value = 42u32;
    let ptr_aligned = Ptr::from(&value);
    let ptr_unaligned = ptr_aligned.to_unaligned();
    
    let val: &u32 = ptr_unaligned.deref();
    assert_eq!(*val, 42);
}

#[test]
fn test_ptr_from_slice() {
    // Note: Ptr::deref() requires T: Sized, so we can't deref to [u32]
    // Instead, we work with individual elements through pointer arithmetic
    let values = [1u32, 2, 3, 4];
    let ptr = Ptr::from(&values[0]);
    
    // Verify we can deref individual elements
    let first: &u32 = ptr.deref();
    assert_eq!(*first, 1);
    
    // Verify pointer arithmetic works
    let second_ptr = ptr.byte_add(core::mem::size_of::<u32>());
    let second: &u32 = second_ptr.deref();
    assert_eq!(*second, 2);
    
    let third_ptr = ptr.byte_add(core::mem::size_of::<u32>() * 2);
    let third: &u32 = third_ptr.deref();
    assert_eq!(*third, 3);
}

#[test]
fn test_ptrmut_promote() {
    let mut value = 42u32;
    let ptr = PtrMut::from(&mut value);
    
    let owning = ptr.promote();
    let result: u32 = owning.read();
    
    assert_eq!(result, 42);
}

#[test]
fn test_complex_struct() {
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        a: u32,
        b: u64,
        c: bool,
    }
    
    let test = TestStruct { a: 10, b: 20, c: true };
    let ptr = Ptr::from(&test);
    
    let deref: &TestStruct = ptr.deref();
    assert_eq!(deref.a, 10);
    assert_eq!(deref.b, 20);
    assert_eq!(deref.c, true);
}

#[test]
fn test_zero_sized_type() {
    struct ZeroSized;
    
    let zst = ZeroSized;
    let ptr = Ptr::from(&zst);
    
    let _deref: &ZeroSized = ptr.deref();
    // Just verify it compiles and doesn't crash
}

#[test]
fn test_multiple_ptr_operations() {
    let mut value = 10u32;
    
    // Create Ptr
    let ptr = Ptr::from(&value as &u32);
    assert_eq!(*ptr.deref::<u32>(), 10);
    
    // Create PtrMut
    let ptr_mut = PtrMut::from(&mut value);
    *ptr_mut.deref_mut::<u32>() = 20;
    assert_eq!(value, 20);
    
    // Verify through Ptr again
    let ptr2 = Ptr::from(&value as &u32);
    assert_eq!(*ptr2.deref::<u32>(), 20);
}

#[test]
fn test_alignment_requirements() {
    // Test with naturally aligned data
    let aligned_value = 42u64;
    let ptr = Ptr::from(&aligned_value);
    
    let deref: &u64 = ptr.deref();
    assert_eq!(*deref, 42);
    
    // Convert to unaligned and back
    let unaligned = ptr.to_unaligned();
    let val: &u64 = unaligned.deref();
    assert_eq!(*val, 42);
}

#[test]
fn test_nested_pointers() {
    let value = 42u32;
    let ref1 = &value;
    let ref2 = &ref1;
    
    let ptr = Ptr::from(ref2);
    let deref: &&u32 = ptr.deref();
    assert_eq!(**deref, 42);
}

#[test]
fn test_array_of_pointers() {
    let values = [1u32, 2, 3, 4, 5];
    let ptrs: [Ptr; 5] = [
        Ptr::from(&values[0]),
        Ptr::from(&values[1]),
        Ptr::from(&values[2]),
        Ptr::from(&values[3]),
        Ptr::from(&values[4]),
    ];
    
    for (i, ptr) in ptrs.iter().enumerate() {
        let val: &u32 = ptr.deref();
        assert_eq!(*val, (i + 1) as u32);
    }
}

#[test]
fn test_ptr_lifetime_safety() {
    fn process_ptr<'a>(ptr: Ptr<'a>) -> &'a u32 {
        ptr.deref()
    }
    
    let value = 42u32;
    let ptr = Ptr::from(&value);
    let result = process_ptr(ptr);
    
    assert_eq!(*result, 42);
}