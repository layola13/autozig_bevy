//! AutoZig WASM 3D Demo - 使用 #[autozig_export] 导出到 WASM64

use autozig::{include_zig, autozig_export};

// 使用 include_zig! 宏引入 Zig 实现
include_zig!("src/demo.zig", {
    // 测试函数 - 返回 u32
    fn test_simple() -> u32;
    
    // 获取版本信息
    fn get_version() -> u32;
});

// 导出函数供 JavaScript 调用
#[autozig_export]
pub fn wasm_test_simple() -> u32 {
    test_simple()
}

#[autozig_export]
pub fn wasm_get_version() -> u32 {
    get_version()
}

#[cfg(test)]
mod tests {
    // Tests only work on non-WASM targets
    #[cfg(not(target_family = "wasm"))]
    use super::*;
    
    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn test_version() {
        let version = wasm_get_version();
        assert_eq!(version, 100); // v1.0.0
    }
    
    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn test_simple_func() {
        let result = wasm_test_simple();
        assert_eq!(result, 43); // 42 + 1
    }
}