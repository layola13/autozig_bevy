//! Build script for autozig-shader
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=zig/");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Use modular_buildzig mode (recommended, handles dependencies correctly)
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    
    // Scan src directory for include_zig! macros
    autozig_build::build("src").expect("Failed to build Zig code");
}