//! Build script for autozig-text
//! Compiles Zig text rendering code

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=src/zig/");
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
    
    // CRITICAL: Also clean target/*/deps archives to prevent EFAULT (error 14)
    // The "Bad address" error occurs during final linking, not Zig compilation
    // We need to clean ALL matching archives INCLUDING the final link product
    let target = std::env::var("TARGET").unwrap_or_default();
    if let Ok(profile) = std::env::var("PROFILE") {
        let target_dir = std::env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| format!("{}/target", std::env::var("CARGO_MANIFEST_DIR").unwrap()));
        
        // Clean both deps and build output directories
        let dirs_to_clean = vec![
            std::path::Path::new(&target_dir).join(&target).join(&profile).join("deps"),
            std::path::Path::new(&target_dir).join(&target).join(&profile),
        ];
        
        for deps_dir in dirs_to_clean {
            if deps_dir.exists() {
                // Remove all libautozig_text-*.a and autozig_text-*.a archives
                if let Ok(entries) = std::fs::read_dir(&deps_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            // Clean ONLY this crate's artifacts
                            let should_remove = (name.starts_with("libautozig_text-") ||
                                                name.starts_with("autozig_text-")) &&
                                               (name.ends_with(".a") || name.ends_with(".rlib"));
                            if should_remove {
                                if let Ok(()) = std::fs::remove_file(&path) {
                                    println!("cargo:warning=Cleaned potentially corrupted archive: {:?}", name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Use modular_buildzig mode (recommended, handles dependencies correctly)
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    println!("cargo:warning=Using MODULAR_BUILDZIG compilation mode for autozig-text");
    
    // WASM64 fix: Disable safety checks that use Thread/POSIX
    // In WASM freestanding environment, Thread.spawn and related APIs are unavailable
    // Using ReleaseFast optimization bypasses these runtime checks
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm") {
        std::env::set_var("AUTOZIG_OPTIMIZE", "ReleaseFast");
        println!("cargo:warning=WASM target detected: using ReleaseFast optimization to bypass Thread/POSIX requirements");
    }
    
    // Build Zig code
    autozig_build::build("src").expect("Failed to build Zig code");
}