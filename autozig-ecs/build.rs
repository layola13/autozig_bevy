//! Build script for autozig-ecs
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=src/zig/");
    println!("cargo:rerun-if-changed=build.rs");
    
    // CRITICAL: Force clean build to avoid corrupted archive
    // The "truncated or malformed archive" error indicates a corrupted libautozig.a
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let lib_path = std::path::Path::new(&out_dir).join("libautozig.a");
    let build_zig_path = std::path::Path::new(&out_dir).join("build.zig");
    let generated_main = std::path::Path::new(&out_dir).join("generated_main.zig");
    
    // Remove potentially corrupted files before build
    if lib_path.exists() {
        let _ = std::fs::remove_file(&lib_path);
        println!("cargo:warning=Removed potentially corrupted libautozig.a");
    }
    if build_zig_path.exists() {
        let _ = std::fs::remove_file(&build_zig_path);
    }
    if generated_main.exists() {
        let _ = std::fs::remove_file(&generated_main);
    }
    
    // Use modular_buildzig mode (recommended, handles dependencies correctly)
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    println!("cargo:warning=Using MODULAR_BUILDZIG compilation mode for autozig-ecs");
    
    // Scan src directory for include_zig! macros
    autozig_build::build("src").expect("Failed to build Zig code");
}
