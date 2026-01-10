#!/bin/bash
# AutoZig-Bevy Native Demo - WASM64 Build Script

set -e

echo "🚀 AutoZig-Bevy Native Demo - WASM64 构建脚本"
echo "================================================"

# 检查 nightly 工具链
echo ""
echo "📦 检查 Rust nightly 工具链..."
if ! rustup toolchain list | grep -q "nightly"; then
    echo "❌ 未找到 nightly 工具链，正在安装..."
    rustup install nightly
else
    echo "✅ nightly 工具链已安装"
fi

# 检查 rust-src
echo ""
echo "📦 检查 rust-src 组件..."
if ! rustup component list --toolchain nightly | grep -q "rust-src (installed)"; then
    echo "⚙️  正在安装 rust-src..."
    rustup component add rust-src --toolchain nightly
else
    echo "✅ rust-src 已安装"
fi

# 编译 WASM64
echo ""
echo "🔨 开始编译 WASM64..."
echo "目标: wasm64-unknown-unknown"
echo "模式: release (lib target)"
echo ""

cargo +nightly build \
    --lib \
    --target wasm64-unknown-unknown \
    -Z build-std=std,panic_abort \
    --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ WASM64 编译成功！"
    echo ""
    echo "📁 WASM 文件位置:"
    echo "   target/wasm64-unknown-unknown/release/native_demo.wasm"
    
    # 检查文件大小
    WASM_FILE="target/wasm64-unknown-unknown/release/native_demo.wasm"
    if [ -f "$WASM_FILE" ]; then
        SIZE=$(du -h "$WASM_FILE" | cut -f1)
        echo "   大小: $SIZE"
        
        # 复制到 www 目录
        echo ""
        echo "📋 复制 WASM 文件到 www 目录..."
        cp "$WASM_FILE" www/
        echo "✅ 已复制到 www/native_demo.wasm"
        
        # 复制 TypeScript 绑定文件
        echo ""
        echo "📋 复制 TypeScript 绑定文件到 www 目录..."
        BINDINGS_DIR=$(find target/wasm64-unknown-unknown/release/build/native_demo-*/out -name "bindings.js" -exec dirname {} \; | head -1)
        if [ -n "$BINDINGS_DIR" ]; then
            cp "$BINDINGS_DIR/bindings.js" www/
            cp "$BINDINGS_DIR/bindings.d.ts" www/
            echo "✅ 已复制 bindings.js 和 bindings.d.ts 到 www/"
        else
            echo "⚠️  警告：未找到 TypeScript 绑定文件"
        fi
        
        echo ""
        echo "🎉 构建完成！"
        echo ""
        echo "📝 运行演示:"
        echo "   cd www"
        echo "   python3 -m http.server 8000"
        echo "   然后在浏览器中打开: http://localhost:8000"
    fi
else
    echo ""
    echo "❌ WASM64 编译失败"
    exit 1
fi