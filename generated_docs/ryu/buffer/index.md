*[ryu](../index.md) / [buffer](index.md)*

---

# Module `buffer`

## Structs

### `Buffer`

```rust
struct Buffer {
    bytes: [core::mem::MaybeUninit<u8>; 24],
}
```

Safe API for formatting floating point numbers to text.

## Example

```rust
let mut buffer = ryu::Buffer::new();
let printed = buffer.format_finite(1.234);
assert_eq!(printed, "1.234");
```

#### Implementations

- `fn new() -> Self`

- `fn format<F: Float>(self: &mut Self, f: F) -> &str`

- `fn format_finite<F: Float>(self: &mut Self, f: F) -> &str`

#### Trait Implementations

##### `impl Clone for Buffer`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- `fn default() -> Self`

## Traits

### `Float`

```rust
trait Float: Sealed { ... }
```

A floating point number, f32 or f64, that can be written into a
[`ryu::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of the
`ryu` crate.

### `Sealed`

```rust
trait Sealed: Copy { ... }
```

#### Required Methods

- `fn is_nonfinite(self: Self) -> bool`

- `fn format_nonfinite(self: Self) -> &'static str`

- `fn write_to_ryu_buffer(self: Self, result: *mut u8) -> usize`

## Constants

### `NAN`

```rust
const NAN: &str;
```

### `INFINITY`

```rust
const INFINITY: &str;
```

### `NEG_INFINITY`

```rust
const NEG_INFINITY: &str;
```

