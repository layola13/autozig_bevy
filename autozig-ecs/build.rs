use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/zig");
    println!("cargo:rerun-if-changed=src/world/zig");
    println!("cargo:rerun-if-changed=src/query");
    println!("cargo:rerun-if-changed=src/change_detection/zig");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let zig_src = manifest_dir.join("src/zig");
    
    // Compile unified Zig entry point
    let zig_entry = manifest_dir.join("src/autozig_ecs.zig");
    let lib_name = "autozig_ecs";
    let obj_path = out_dir.join(format!("{}.o", lib_name));
    
    // Check profile
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let opt_mode = if profile == "release" { "ReleaseSafe" } else { "Debug" };

    println!("cargo:rerun-if-changed={}", zig_entry.display());

    let output = std::process::Command::new("zig")
        .arg("build-obj") 
        .arg("-lc")       
        .arg("-fPIC")     
        .arg("-target").arg("x86_64-linux") 
        .arg(format!("-O{}", opt_mode))
        .arg(format!("-femit-bin={}", obj_path.display()))
        .arg(&zig_entry)
        .output();

    match output {
        Ok(o) if o.status.success() => {
            println!("cargo:rustc-link-arg={}", obj_path.display());
        }
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            println!("cargo:warning=Zig compilation failed: {}", stderr);
        }
        Err(e) => {
                println!("cargo:warning=Failed to execute zig command: {}", e);
        }
    }
}
