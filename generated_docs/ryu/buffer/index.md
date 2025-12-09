*[ryu](../index.md) / [buffer](index.md)*

---

# Module `buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buffer`](#buffer) | struct | Safe API for formatting floating point numbers to text. |
| [`Float`](#float) | trait | A floating point number, f32 or f64, that can be written into a [`ryu::Buffer`][Buffer]. |
| [`Sealed`](#sealed) | trait |  |
| [`NAN`](#nan) | const |  |
| [`INFINITY`](#infinity) | const |  |
| [`NEG_INFINITY`](#neg_infinity) | const |  |

## Structs

### `Buffer`

```rust
struct Buffer {
    bytes: [core::mem::MaybeUninit<u8>; 24],
}
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:20-22`](../../../.source_1765210505/ryu-1.0.20/src/buffer/mod.rs#L20-L22)*

Safe API for formatting floating point numbers to text.

## Example

```rust
let mut buffer = ryu::Buffer::new();
let printed = buffer.format_finite(1.234);
assert_eq!(printed, "1.234");
```

#### Implementations

- <span id="buffer-new"></span>`fn new() -> Self`

- <span id="buffer-format"></span>`fn format<F: Float>(&mut self, f: F) -> &str`

- <span id="buffer-format-finite"></span>`fn format_finite<F: Float>(&mut self, f: F) -> &str`

#### Trait Implementations

##### `impl Clone for Buffer`

- <span id="buffer-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- <span id="buffer-default"></span>`fn default() -> Self`

## Traits

### `Float`

```rust
trait Float: Sealed { ... }
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:105`](../../../.source_1765210505/ryu-1.0.20/src/buffer/mod.rs#L105)*

A floating point number, f32 or f64, that can be written into a
[`ryu::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of the
`ryu` crate.

#### Implementors

- `f32`
- `f64`

### `Sealed`

```rust
trait Sealed: Copy { ... }
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:109-113`](../../../.source_1765210505/ryu-1.0.20/src/buffer/mod.rs#L109-L113)*

#### Required Methods

- `fn is_nonfinite(self) -> bool`

- `fn format_nonfinite(self) -> &'static str`

- `fn write_to_ryu_buffer(self, result: *mut u8) -> usize`

#### Implementors

- `f32`
- `f64`

## Constants

### `NAN`
```rust
const NAN: &str;
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:7`](../../../.source_1765210505/ryu-1.0.20/src/buffer/mod.rs#L7)*

### `INFINITY`
```rust
const INFINITY: &str;
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:8`](../../../.source_1765210505/ryu-1.0.20/src/buffer/mod.rs#L8)*

### `NEG_INFINITY`
```rust
const NEG_INFINITY: &str;
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:9`](../../../.source_1765210505/ryu-1.0.20/src/buffer/mod.rs#L9)*

