//! AutoZig WASM 3D Demo - 完全按照 wasm64bit 示例

use autozig::include_zig;
use wasm_bindgen::prelude::*;

// 使用 include_zig! 宏引入 Zig 实现
// 使用 #[autozig(strategy = "dual")] 自动生成双重绑定
include_zig!("src/demo.zig", {
    // 测试函数 - 返回 u32
    #[autozig(strategy = "dual")]
    fn test_simple() -> u32;
    
    // 获取版本信息
    #[autozig(strategy = "dual")]
    fn get_version() -> u32;
});

// Optional: panic hook for debugging in browser console
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
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