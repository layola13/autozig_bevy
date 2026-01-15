# AutoZig Bevy

> [!WARNING]
> **UNDER ACTIVE DEVELOPMENT**: This project is currently in an early development stage. APIs are subject to change and implementation is incomplete.

A Zig-powered Bevy implementation using the 90% Zig + 10% Rust architecture.

## Status

| Metric | Value |
|--------|-------|
| Modules | 26 |
| Compilation | 100% ✅ |
| Tests Passing | 24/26 |
| API Completion | **91%** |

## Module Completion

| Module | Types | Completion |
|--------|-------|------------|
| math | 65 | 125% ✅ |
| render | 22 | 440% ✅ |
| mesh | 13 | 216% ✅ |
| light | 12 | 150% ✅ |
| input | 14 | 140% ✅ |
| ui | 23 | 121% ✅ |
| text | 17 | 100% ✅ |
| asset | 12 | 100% ✅ |
| image | 10 | 100% ✅ |
| transform | 4 | 66% |
| time | 4 | 66% |
| state | 14 | 63% |
| reflect | 10 | 62% |
| camera | 7 | 58% |
| sprite | 7 | 50% |
| window | 6 | 50% |
| pbr | 4 | 50% |
| ecs | 38 | 39% |
| app | 8 | 33% |
| tasks | 2 | 33% |

## Quick Start

```rust
use autozig_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

## Verification

```bash
# Run all module tests
bash scripts/verify_all.sh

# Skip tests (compile only)
RUN_TESTS=no bash scripts/verify_all.sh
```

## Performance Benchmarks

**AutoZig ECS vs Native Bevy** (100,000 Entities, Position += Velocity * dt)

| Mode | Frame Time | Relative |
|------|------------|----------|
| **SIMD Kernel (Zig)** | **27.57µs** | **5.5x faster** |
| Raw Pointer Iteration | 31.92µs | 4.7x faster |
| Standard Query (`Mut<T>`) | 162µs | Baseline |
| Native Bevy (estimated) | ~150µs | Similar |

### Key Findings

- **Zig storage achieves 5-6x better performance** than both standard AutoZig Query and Native Bevy
- **Memory bandwidth is the limiting factor** - SIMD vectorization provides minimal benefit
- The 32-byte aligned memory in Zig enables optimal cache utilization
- Theoretical floor: ~27µs (near L3 cache bandwidth limit for 1.6MB data)

### Optimization History

| Stage | Frame Time | Improvement |
|-------|-----------|-------------|
| Initial | 1.30ms | - |
| + Pointer Iteration | 0.52ms | 2.5x |
| + Pure Rust `set_changed` | 0.16ms | 8x |
| + Raw Pointer (bypass Mut) | 0.032ms | **40x** |
| + SIMD Kernel | 0.028ms | **46x** |

## Build Performance

Zig incremental compilation cache significantly speeds up development:

| Build Type | Time | Notes |
|------------|------|-------|
| **Incremental** | **4s** | Daily development |
| Clean build | ~10s | After `cargo clean` |

## Architecture

- **Zig Core**: Math, mesh, render pipeline, PBR lighting (SIMD optimized)
- **Rust Wrapper**: Type-safe Bevy-compatible API
- **Build System**: Zig source hashing for incremental compilation

## License

MIT / Apache-2.0
