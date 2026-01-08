//! Basic usage examples for autozig-ptr
//! 
//! This example demonstrates safe pointer operations using type-erased pointers.

use autozig_ptr::{Ptr, PtrMut, OwningPtr, MovingPtr, ThinSlicePtr};
use core::mem::MaybeUninit;

fn main() {
    println!("=== autozig-ptr Examples ===\n");
    
    example_ptr();
    example_ptrmut();
    example_owning_ptr();
    example_moving_ptr();
    example_thin_slice();
    
    println!("\n=== All examples completed successfully! ===");
}

/// Example 1: Using Ptr for type-erased immutable borrows
fn example_ptr() {
    println!("Example 1: Ptr (immutable type-erased pointer)");
    
    let value = 42u32;
    let ptr = Ptr::from(&value);
    
    // Deref to get the value back
    let deref_value: &u32 = ptr.deref();
    println!("  Original value: {}", value);
    println!("  Dereferenced: {}", deref_value);
    
    // Pointer arithmetic
    let array = [1u32, 2, 3, 4, 5];
    let ptr = Ptr::from(&array[0]);
    let ptr2 = ptr.byte_add(core::mem::size_of::<u32>());
    let value2: &u32 = ptr2.deref();
    println!("  Array[1] via byte_add: {}", value2);
    
    println!();
}

/// Example 2: Using PtrMut for type-erased mutable borrows
fn example_ptrmut() {
    println!("Example 2: PtrMut (mutable type-erased pointer)");
    
    let mut value = 42u32;
    println!("  Initial value: {}", value);
    
    let ptr = PtrMut::from(&mut value);
    let deref_value: &mut u32 = ptr.deref_mut();
    *deref_value = 100;
    
    println!("  Modified value: {}", value);
    
    // Reborrow example
    let mut value2 = 50u32;
    let mut ptr2 = PtrMut::from(&mut value2);
    {
        let ptr3 = ptr2.reborrow();
        *ptr3.deref_mut::<u32>() = 200;
    }
    println!("  Reborrowed and modified: {}", value2);
    
    println!();
}

/// Example 3: Using OwningPtr for type-erased ownership
fn example_owning_ptr() {
    println!("Example 3: OwningPtr (type-erased ownership)");
    
    let result = OwningPtr::make(42u32, |ptr| {
        println!("  Inside OwningPtr::make closure");
        let value: u32 = ptr.read();
        println!("  Read value: {}", value);
        value * 2
    });
    
    println!("  Result: {}", result);
    println!();
}

/// Example 4: Using MovingPtr for moving values
fn example_moving_ptr() {
    println!("Example 4: MovingPtr (moving values without passing by value)");
    
    #[derive(Debug)]
    struct LargeStruct {
        data: [u64; 100],
    }
    
    let large = LargeStruct { data: [42; 100] };
    let mut uninit = MaybeUninit::new(large);
    
    let ptr = MovingPtr::from_value(&mut uninit);
    
    // Move to a new location
    let mut target = MaybeUninit::<LargeStruct>::uninit();
    ptr.write_to(target.as_mut_ptr());
    
    let moved_value = unsafe { target.assume_init() };
    println!("  Moved large struct, first element: {}", moved_value.data[0]);
    println!("  Moved large struct, last element: {}", moved_value.data[99]);
    
    println!();
}

/// Example 5: Using ThinSlicePtr for slices without length
fn example_thin_slice() {
    println!("Example 5: ThinSlicePtr (slice pointer without length)");
    
    let values = [10u32, 20, 30, 40, 50];
    let thin = ThinSlicePtr::from(&values[..]);
    
    println!("  Accessing elements via ThinSlicePtr:");
    for i in 0..5 {
        let value = thin.get_unchecked(i);
        println!("    Index {}: {}", i, value);
    }
    
    println!();
}