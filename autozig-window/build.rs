//! Build script for autozig-window
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    // 强制使用 MODULAR_BUILDZIG 模式避免 Zig 代码重复定义（wasm64 支持）
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Scan src directory for include_zig! macros
    autozig_build::build("src").expect("Failed to build Zig code");
}
