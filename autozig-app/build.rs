//! Build script for autozig-app
//! Scans src directory for include_zig! macros and compiles Zig code

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    
    // CRITICAL: Force clean build to avoid corrupted archive
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
    println!("cargo:warning=Using MODULAR_BUILDZIG compilation mode for autozig-app");
    
    // Copy all Zig source files to OUT_DIR so they can be imported by the generated main.zig
    let src_zig_dir = std::path::Path::new("src/zig");
    if src_zig_dir.exists() {
        for entry in std::fs::read_dir(src_zig_dir).expect("Failed to read src/zig directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "zig") {
                let file_name = path.file_name().unwrap();
                let dest = std::path::Path::new(&out_dir).join(file_name);
                std::fs::copy(&path, &dest).expect("Failed to copy zig file");
                println!("cargo:warning=Copied {} to OUT_DIR", file_name.to_string_lossy()); 
            }
        }
    }

    // Scan src directory for include_zig! macros
    autozig_build::build("src").expect("Failed to build Zig code");
}