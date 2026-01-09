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

## Architecture

- **Zig Core**: Math, mesh, render pipeline, PBR lighting (SIMD optimized)
- **Rust Wrapper**: Type-safe Bevy-compatible API

## License

MIT / Apache-2.0
