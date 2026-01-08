//! Build script for autozig-reflect
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    // Use modular_buildzig mode for better Zig file organization
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    
    // Scan src directory for include_zig! macros按照autozig风格
    autozig_build::build("src").expect("Failed to build Zig code");
    
    // Tell cargo to rerun if source files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=src/zig/");
    println!("cargo:rerun-if-changed=build.rs");
}