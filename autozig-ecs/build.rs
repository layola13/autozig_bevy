use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    // Only rerun if Zig source files actually change
    println!("cargo:rerun-if-changed=src/zig");
    println!("cargo:rerun-if-changed=src/autozig_ecs.zig");
    println!("cargo:rerun-if-changed=src/query/fetch/zig");
    println!("cargo:rerun-if-changed=src/query/filter/zig");
    println!("cargo:rerun-if-changed=src/query/builder/zig");
    println!("cargo:rerun-if-changed=src/world/zig");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let zig_entry = manifest_dir.join("src/autozig_ecs.zig");
    let lib_output = out_dir.join("libautozig_ecs.a");
    let hash_file = out_dir.join(".zig_hash");
    
    // Compute hash of all Zig source files
    let current_hash = compute_zig_hash(&manifest_dir);
    
    // Check if we can skip compilation
    let should_compile = if lib_output.exists() && hash_file.exists() {
        let mut saved_hash = String::new();
        if let Ok(mut f) = fs::File::open(&hash_file) {
            f.read_to_string(&mut saved_hash).ok();
        }
        saved_hash.trim() != current_hash
    } else {
        true
    };
    
    if should_compile {
        eprintln!("cargo:warning=Zig source changed, recompiling...");
        
        let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
        let opt_mode = if profile == "release" { "ReleaseSafe" } else { "Debug" };

        let output = std::process::Command::new("zig")
            .arg("build-lib") 
            .arg("-lc")       
            .arg("-fPIC")     
            .arg("-target").arg("x86_64-linux") 
            .arg(format!("-O{}", opt_mode))
            .arg(format!("-femit-bin={}", lib_output.display()))
            .arg(&zig_entry)
            .output();

        match output {
            Ok(o) if o.status.success() => {
                // Save hash for next build
                if let Ok(mut f) = fs::File::create(&hash_file) {
                    f.write_all(current_hash.as_bytes()).ok();
                }
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                eprintln!("cargo:warning=Zig compilation failed: {}", stderr);
            }
            Err(e) => {
                eprintln!("cargo:warning=Failed to execute zig command: {}", e);
            }
        }
    } else {
        eprintln!("cargo:warning=Zig cache hit, skipping compilation");
    }
    
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=autozig_ecs");
}

/// Compute hash of all Zig files in the project
fn compute_zig_hash(manifest_dir: &PathBuf) -> String {
    let mut hasher = DefaultHasher::new();
    
    let zig_dirs = [
        "src/zig",
        "src/query/fetch/zig",
        "src/query/filter/zig", 
        "src/query/builder/zig",
        "src/world/zig",
    ];
    
    // Hash the main entry point
    if let Ok(content) = fs::read_to_string(manifest_dir.join("src/autozig_ecs.zig")) {
        content.hash(&mut hasher);
    }
    
    // Hash all zig files in directories
    for dir in zig_dirs {
        let path = manifest_dir.join(dir);
        if path.exists() {
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    if entry.path().extension().map_or(false, |e| e == "zig") {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            content.hash(&mut hasher);
                        }
                    }
                }
            }
        }
    }
    
    format!("{:x}", hasher.finish())
}
