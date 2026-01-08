//! Build script for autozig-window
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Scan src directory for include_zig! macros
    autozig_build::build("src").expect("Failed to build Zig code");
}