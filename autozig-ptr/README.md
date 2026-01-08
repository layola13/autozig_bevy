# autozig-ptr

Type-erased pointers with Zig implementation for safe low-level memory operations.

## Overview

`autozig-ptr` provides type-erased pointer types that maintain safety invariants while allowing operations on data without knowing its concrete type at compile time. This crate is designed for `no_std` environments and uses Zig for 90% of its implementation to avoid `unsafe` code in Rust.

## Features

- **Type-erased pointers**: Work with pointers without compile-time type information
- **Zero unsafe in Rust**: All pointer operations implemented in Zig
- **No-std compatible**: Works in embedded and WASM environments
- **Alignment tracking**: Compile-time alignment guarantees via type parameters
- **Lifetime safety**: Leverages Rust's lifetime system for memory safety

## Core Types

### `Ptr<'a, A>`
Type-erased immutable borrow. Think of it as `&'a dyn Any` but without metadata.

```rust
use autozig_ptr::Ptr;

let value = 42u32;
let ptr = Ptr::from(&value);
let deref: &u32 = ptr.deref();
assert_eq!(*deref, 42);
```

### `PtrMut<'a, A>`
Type-erased mutable borrow. Similar to `&'a mut dyn Any` without metadata.

```rust
use autozig_ptr::PtrMut;

let mut value = 42u32;
let ptr = PtrMut::from(&mut value);
let deref: &mut u32 = ptr.deref_mut();
*deref = 100;
assert_eq!(value, 100);
```

### `OwningPtr<'a, A>`
Type-erased owning pointer. Like `Box<dyn Any>` but doesn't free memory (useful for custom allocators).

```rust
use autozig_ptr::OwningPtr;

let result = OwningPtr::make(42u32, |ptr| {
    let value: u32 = ptr.read();
    value * 2
});
assert_eq!(result, 84);
```

### `MovingPtr<'a, T, A>`
Pointer for moving values without passing by value. Useful for large structs.

```rust
use autozig_ptr::MovingPtr;
use core::mem::MaybeUninit;

let mut value = MaybeUninit::new(42u32);
let ptr = MovingPtr::from_value(&mut value);

let mut target = 0u32;
ptr.write_to(&mut target);
assert_eq!(target, 42);
```

### `ThinSlicePtr<'a, T>`
Slice pointer without length information. Useful when length is stored separately.

```rust
use autozig_ptr::ThinSlicePtr;

let values = [1u32, 2, 3, 4, 5];
let thin = ThinSlicePtr::from(&values[..]);

assert_eq!(*thin.get_unchecked(0), 1);
assert_eq!(*thin.get_unchecked(4), 5);
```

## Alignment

The type parameter `A` tracks whether the pointer is guaranteed to be aligned:

- `Aligned`: Pointer is guaranteed to be properly aligned for its type
- `Unaligned`: Pointer may not be aligned

```rust
use autozig_ptr::{Ptr, Aligned, Unaligned};

let value = 42u64;
let ptr: Ptr<Aligned> = Ptr::from(&value);
let unaligned: Ptr<Unaligned> = ptr.to_unaligned();
```

## Architecture

### 90% Zig, 10% Rust

The core pointer operations are implemented in Zig (`src/zig/ptr.zig`), which provides:
- Memory copy operations
- Pointer arithmetic
- Alignment checking
- Type conversion utilities

The Rust layer (`src/lib.rs`) provides:
- Type-safe API wrapping Zig FFI
- Lifetime management
- Trait implementations
- Documentation

### Build System

Uses `autozig::include_zig!` macro to seamlessly integrate Zig code:

```rust
autozig::include_zig!("src/zig/ptr.zig");
```

The `build.rs` script automatically compiles Zig code during the Rust build process.

## Safety

Despite working with raw pointers, `autozig-ptr` maintains safety through:

1. **Lifetime tracking**: All pointers carry Rust lifetimes
2. **Type erasure at API boundary**: Internal operations don't need type information
3. **Zig implementation**: Core operations in Zig avoid `unsafe` in Rust
4. **Alignment tracking**: Compile-time guarantees via type parameters

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
autozig-ptr = { path = "path/to/autozig-ptr" }
```

## Examples

Run the basic example:

```bash
cargo run --example basic_ptr
```

## Testing

Run the test suite:

```bash
cargo test
```

The test suite includes:
- Basic pointer operations
- Pointer arithmetic
- Lifetime safety
- Alignment handling
- Complex types (structs, arrays, nested pointers)

## Comparison with bevy_ptr

`autozig-ptr` is API-compatible with `bevy_ptr` but uses autozig's build system:

| Feature | bevy_ptr | autozig-ptr |
|---------|----------|-------------|
| Core implementation | Zig | Zig |
| Build system | Custom | autozig |
| FFI approach | Manual extern | include_zig! macro |
| API compatibility | ✓ | ✓ |
| no_std support | ✓ | ✓ |

## License

MIT OR Apache-2.0

## Contributing

This is part of the autozig-bevy project. Contributions should follow the autozig code style and ensure:
- No `unsafe` keyword in Rust code
- All tests pass
- Examples compile and run
- Documentation is complete