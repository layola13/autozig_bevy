//! Build script for autozig-utils
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    // Scan src directory for include_zig! macros按照autozig风格
    autozig_build::build("src").expect("Failed to build Zig code");
    
    // Tell cargo to rerun if source files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
}