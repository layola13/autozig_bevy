use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let zig_src = manifest_dir.join("src/zig");
    
    println!("cargo:rerun-if-changed=src/zig");
    
    // 编译 Zig 渲染核心
    let zig_status = Command::new("zig")
        .args(&[
            "build-lib",
            "src/zig/render.zig",
            "-dynamic",
            "-O", "ReleaseFast",
            "--name", "autozig_render_core",
        ])
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to compile Zig code");
    
    if !zig_status.success() {
        panic!("Zig compilation failed");
    }
    
    // 链接 Zig 库
    println!("cargo:rustc-link-search=native={}", manifest_dir.display());
    println!("cargo:rustc-link-lib=dylib=autozig_render_core");
    
    // 生成 C 绑定头文件
    let _header_status = Command::new("zig")
        .args(&[
            "build-lib",
            "src/zig/render.zig",
            "-femit-h=src/zig/autozig_render.h",
            "-fno-emit-bin",
        ])
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to generate header file");
}