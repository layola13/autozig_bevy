//! Build script for autozig-reflect
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=src/zig/");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Use modular_buildzig mode for better Zig file organization
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    println!("cargo:warning=Using MODULAR_BUILDZIG compilation mode for autozig-reflect");
    
    // WASM64 fix: Disable safety checks that use Thread/POSIX
    // In WASM freestanding environment, std.ArrayList and std.AutoHashMap's debug code
    // uses Thread.getCurrentId() and POSIX calls which are unavailable
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm") {
        std::env::set_var("AUTOZIG_OPTIMIZE", "ReleaseFast");
        println!("cargo:warning=WASM target detected: using ReleaseFast optimization to bypass Thread/POSIX requirements");
    }
    
    // Scan src directory for include_zig! macros按照autozig风格
    autozig_build::build("src").expect("Failed to build Zig code");
}