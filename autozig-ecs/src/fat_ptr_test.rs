#[cfg(test)]
mod tests {
    use super::*;

    trait TestTrait {
        fn foo(&self) -> i32;
    }

    struct TestStruct(i32);
    impl TestTrait for TestStruct {
        fn foo(&self) -> i32 { self.0 }
    }

    #[test]
    fn test_fat_ptr_layout() {
        let obj = TestStruct(42);
        let ptr: *const dyn TestTrait = &obj;
        let size = std::mem::size_of_val(&ptr);
        println!("Size of fat pointer: {}", size);
        
        let parts: [usize; 2] = unsafe { std::mem::transmute(ptr) };
        println!("Parts: [0x{:x}, 0x{:x}]", parts[0], parts[1]);
        
        let data_ptr = &obj as *const TestStruct as usize;
        println!("Actual data ptr: 0x{:x}", data_ptr);
        
        if parts[0] == data_ptr {
            println!("Order is (data, vtable)");
        } else if parts[1] == data_ptr {
            println!("Order is (vtable, data)");
        } else {
            println!("Data pointer not found in parts!");
        }
    }
}
