# AutoZig JSON

**High-performance Zig-powered JSON parser for Rust**

Zero serde dependencies. SIMD-accelerated Tape parser. Zero-copy borrowing.

## Features

| Feature | Description |
|---------|-------------|
| **Zig SIMD Backend** | Tape-based parser with SIMD whitespace scanning |
| **Zero Dependencies** | No serde, syn, or quote - just Zig + proc_macro |
| **Zero-Copy** | Borrow `&'a str` directly from JSON input |
| **Fast Compilation** | Hand-written macro with linear code generation |

## Performance

```
Complex Game Data (837 bytes): 369µs
Zero-Copy Player (196 bytes):   18µs
→ Strings point directly into JSON buffer!
```

## Quick Start

```rust
use autozig_json::{parse, AutoDeserialize};

#[derive(AutoDeserialize, Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

fn main() {
    let json = r#"{"id": 101, "name": "Alice", "active": true}"#;
    let user: User = parse(json).unwrap();
    println!("{:?}", user);
}
```

## Supported Types

- **Primitives**: `i32`, `u32`, `i64`, `f32`, `f64`, `bool`
- **Strings**: `String`, `&'a str` (zero-copy)
- **Collections**: `Vec<T>`
- **Optional**: `Option<T>`
- **Nested**: Any struct with `#[derive(AutoDeserialize)]`

## Zero-Copy Mode

For maximum performance, borrow strings directly:

```rust
use autozig_json::{parse_borrow, BorrowDeserialize, TapeRef};

struct Player<'a> {
    name: &'a str,  // Points into JSON buffer!
}

// Manual impl (derive macro in progress)
impl<'a> BorrowDeserialize<'a> for Player<'a> { ... }

let tape = TapeRef::parse(json)?;
let player = parse_borrow::<Player>(&json, &tape)?;
```

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌────────────────┐
│ Rust Struct     │ ──▶│ Proc-Macro       │ ──▶│ Zig Tape       │
│ #[derive(...)]  │    │ TokenStream only │    │ SIMD Parser    │
└─────────────────┘    └──────────────────┘    └────────────────┘
                              │                        │
                              └────── from_tape() ─────┘
```

## Examples

```bash
# Basic parsing
cargo run --example derive_demo

# Complex game data (nested, Vec, Option)
cargo run --example complex_game_data

# Zero-copy parsing
cargo run --example zero_copy
```

## License

MIT
