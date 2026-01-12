fn main() {
    // 强制使用 MODULAR_BUILDZIG 模式避免 Zig 代码重复定义（wasm64 支持）
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    
    autozig_build::build("src").expect("Failed to build Zig code");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=zig/");
}