#!/bin/bash
set -e

# Project root
ROOT_DIR=$(pwd)/autozig_bevy

# List of crates to verify
CRATES=($(find "$ROOT_DIR" -maxdepth 2 -name Cargo.toml -exec dirname {} \; | xargs -n 1 basename | sort | grep -v "autozig_bevy"))

echo "🚀 Verifying Bevy crates for WASM64..."
echo "========================================"

FAILED=0
PASSED=0

for crate in "${CRATES[@]}"; do
    echo ""
    echo "🔍 Checking $crate..."
    
    if [ ! -d "$ROOT_DIR/$crate" ]; then
        echo "⚠️  Directory not found: $ROOT_DIR/$crate"
        FAILED=$((FAILED + 1))
        continue
    fi
    
    cd "$ROOT_DIR/$crate"
    
    # Attempt to build for WASM64
    if cargo +nightly build --target wasm64-unknown-unknown -Z build-std=std,panic_abort --release 2>&1 | tee /tmp/build_${crate}.log | grep -qE "(error:|error\[)"; then
        echo "❌ $crate FAILED"
        echo "   See log: /tmp/build_${crate}.log"
        FAILED=$((FAILED + 1))
    else
        echo "✅ $crate PASSED"
        PASSED=$((PASSED + 1))
    fi
done

echo ""
echo "========================================"
echo "Summary: $PASSED passed, $FAILED failed"

if [ $FAILED -gt 0 ]; then
    exit 1
fi
