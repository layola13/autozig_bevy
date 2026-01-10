#!/bin/bash
# AutoZig WASM Hello World - 构建脚本

set -e

echo "🚀 AutoZig WASM Hello World 构建"
echo "===================================="
echo ""

# 检查必要工具
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ 错误: $1 未安装"
        echo "   请先安装 $1"
        exit 1
    fi
    echo "✓ 检测到 $1"
}

echo "📦 检查依赖工具..."
check_tool cargo
check_tool rustc

echo ""
echo "🔧 配置 Rust 工具链..."

# 检查是否有 nightly 工具链
if ! rustup toolchain list | grep -q "nightly"; then
    echo "   安装 nightly 工具链..."
    rustup toolchain install nightly
fi

# 检查是否有rust-src组件
if ! rustup component list --installed | grep -q "rust-src"; then
    echo "   安装 rust-src 组件..."
    rustup component add rust-src
fi

echo ""
echo "🔨 使用 wasm64-unknown-unknown 构建..."
echo "   ⚠️  注意: wasm64 target需要从源码构建标准库"
echo ""

# 检查Rust版本
rust_version=$(rustc --version | grep -oP '\d+\.\d+' | head -1)
echo "   检测到 Rust 版本: $rust_version"

# 使用 build-std 构建
echo "   正在构建（这可能需要几分钟）..."
cargo +nightly build \
    --target wasm64-unknown-unknown \
    -Z build-std=std,panic_abort \
    --release \
    --lib

echo ""
echo "✅ 构建完成！"
echo "   输出: target/wasm64-unknown-unknown/release/autozig_wasm_hello_world.wasm"

# 复制wasm文件到www目录
echo ""
echo "📦 准备Web部署文件..."
mkdir -p www
cp target/wasm64-unknown-unknown/release/autozig_wasm_hello_world.wasm www/

# 复制 autozig 自动生成的 TypeScript 绑定文件
BUILD_OUT_DIR=$(find target/wasm64-unknown-unknown/release/build/autozig-wasm-hello-world-*/out -maxdepth 0 2>/dev/null | head -1)
if [ -n "$BUILD_OUT_DIR" ] && [ -f "$BUILD_OUT_DIR/bindings.d.ts" ]; then
    cp "$BUILD_OUT_DIR/bindings.d.ts" www/
    cp "$BUILD_OUT_DIR/bindings.js" www/
    echo "   ✅ AutoZig 生成的绑定已复制: bindings.d.ts, bindings.js"
fi

echo "   ✅ WASM 文件已复制到 www/"
echo "   ℹ️  使用 AutoZig 自动绑定：无需 wasm-bindgen"

echo ""
echo "📝 后续步骤:"
echo "   1. 启动开发服务器:"
echo "      cd www && python3 -m http.server 8080"
echo ""
echo "   2. 在浏览器中打开:"
echo "      http://localhost:8080"
echo ""
echo "   3. 确保启用 Memory64 支持:"
echo "      Chrome: chrome://flags/#enable-webassembly-memory64"
echo "      Firefox: about:config -> javascript.options.wasm_memory64"
echo ""

echo "🎉 构建脚本完成！"
echo ""
echo "💡 提示: 本项目使用 AutoZig 自动绑定方案"
echo "   - 不依赖 wasm-bindgen/wasm-pack"
echo "   - AutoZig 自动生成 JavaScript/TypeScript 绑定"
echo "   - 直接通过 WebAssembly.instantiate 加载"
echo "   - 支持完整的 wasm64 特性"