// Node.js 测试脚本用于验证 WASM 模块
const fs = require('fs');
const path = require('path');

async function testWasm() {
    console.log('🧪 开始测试 WASM 模块...\n');
    
    try {
        // 读取 WASM 文件
        const wasmPath = path.join(__dirname, 'autozig_wasm_3d_demo.wasm');
        const wasmBuffer = fs.readFileSync(wasmPath);
        
        console.log('✅ WASM 文件读取成功');
        console.log(`   文件大小: ${wasmBuffer.length} 字节\n`);
        
        // 实例化 WASM 模块
        const wasmModule = await WebAssembly.instantiate(wasmBuffer, {});
        const { exports } = wasmModule.instance;
        
        console.log('✅ WASM 模块实例化成功');
        console.log('   导出的函数:', Object.keys(exports).filter(k => typeof exports[k] === 'function'), '\n');
        
        // 测试 1: test_simple 函数
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        console.log('测试 1: wasm_test_simple()');
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        
        if (typeof exports.wasm_test_simple === 'function') {
            const result = exports.wasm_test_simple();
            console.log(`返回值: ${result}`);
            console.log(`预期值: 43`);
            console.log(`状态: ${result === 43 ? '✅ 通过' : '❌ 失败'}\n`);
        } else {
            console.log('❌ 函数未找到\n');
        }
        
        // 测试 2: get_version 函数
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        console.log('测试 2: wasm_get_version()');
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        
        if (typeof exports.wasm_get_version === 'function') {
            const version = exports.wasm_get_version();
            console.log(`返回值: ${version}`);
            console.log(`版本格式: v${Math.floor(version/100)}.${Math.floor((version%100)/10)}.${version%10}`);
            console.log(`状态: ${version === 100 ? '✅ 通过' : '❌ 失败'}\n`);
        } else {
            console.log('❌ 函数未找到\n');
        }
        
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        console.log('🎉 所有测试完成！');
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        
    } catch (error) {
        console.error('❌ 测试失败:', error.message);
        console.error('详细信息:', error.stack);
        process.exit(1);
    }
}

testWasm();