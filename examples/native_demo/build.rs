fn main() -> anyhow::Result<()> {
    // 检查是否为 WASM 目标
    let target = std::env::var("TARGET").unwrap_or_default();
    
    if target.contains("wasm") {
        println!("cargo:warning=正在为 WASM64 目标编译并生成 TypeScript 绑定...");
        
        // 强制使用 MODULAR_BUILDZIG 模式避免文件重复
        std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    }
    
    // 🎯 一行搞定！对于 WASM 目标，build() 会自动：
    // 1. 编译 Zig 代码（如果有 autozig! 宏）
    // 2. 生成 TypeScript 绑定（对于 #[autozig_export] 函数）
    autozig_build::build("src")?;
    
    if target.contains("wasm") {
        println!("cargo:warning=✅ WASM 编译和 TypeScript 绑定生成完成");
    }
    
    Ok(())
}