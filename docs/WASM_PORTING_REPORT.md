# AutoZig Bevy WASM64 Porting Report

## Check Result

| Crate | Status | Error Category | Details |
|-------|--------|----------------|---------|
| `autozig-app` | ✅ PASSED | - | Compiled successfully with `#[link(name="autozig")]` fix. |
| `autozig-ecs` | ❌ FAILED | **OS Dependency** | `std.Thread` and `posix` calls not available on `freestanding` (WASM). |
| `autozig-math` | ❌ FAILED | **Unknown** | Build failed, likely due to platform-specific code or ABI issues. |
| `autozig-sprite` | ❌ FAILED | **WASM ABI** | `export fn` cannot return or accept arrays (`[4]f32`) by value. |

## Root Cause Analysis

### 1. Networking/Linking Fix (Solved)
The initial failure of `autozig-app` was due to missing `#[link(name="autozig")]` attribute on WASM targets. This caused `rustc` to emit dynamic imports for FFI functions instead of resolving them against the statically linked `libautozig.a`.
**Fix Applied:** Updated `autozig-macro/src/lib.rs` to generate `#[link(name="autozig")]` for `extern "C"` blocks.

### 2. OS Dependencies (Unsolved)
`autozig-ecs` uses `std.Thread` (likely for task pools or parallel iteration). WASM is single-threaded by default and lacks OS thread APIs.
**Error Log:**
```
error: Unsupported operating system freestanding
referenced by: getCurrentId
```

### 3. WASM ABI Limitations (Unsolved)
`autozig-sprite` exposes functions that use Zig arrays (`[4]f32`) in their signature (params/return). The default WASM ABI for `export fn` cannot handle these complex value types directly.
**Error Log:**
```
error: return type '[4]f32' not allowed in function with calling convention 'wasm_mvp'
```

## Fix Strategy

### Phase 1: Macro & Build System (Done)
- [x] Enable FFI generation for WASM in `autozig-macro`.
- [x] Add `#[link(name="autozig")]` to ensure internal linking.
- [x] Verify `autozig-app` compiles.

### Phase 2: Crate-Specific Fixes (Next Steps)

#### `autozig-ecs`
- **Goal:** Remove/Guard threading usage.
- **Action:**
  - Audit `autozig-ecs` Zig code for `std.Thread`.
  - Use conditional compilation (`if (builtin.os.tag != .freestanding)`) to fallback to single-threaded logic or dummy implementations.

#### `autozig-sprite`
- **Goal:** Fix ABI compatibility.
- **Action:**
  - Change `export fn` signatures to use pointers (`*const [4]f32`) instead of value arrays.
  - Or wrap array operations in a struct if `extern struct` is supported (usually yes).
  - Update Rust-side signatures to match (pointers).

#### `autozig-math`
- **Goal:** Fix compilation.
- **Action:**
  - Investigate build log (re-run to capture output).
  - Fix any platform-specific math functions or dependencies.

## Verification
Run `autozig_bevy/scripts/verify_wasm64.sh` after each crate fix.
