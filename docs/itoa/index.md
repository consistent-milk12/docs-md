# Crate `itoa`

[![github](#github)](https://github.com/dtolnay/itoa)&ensp;[![crates-io]](https://crates.io/crates/itoa)&ensp;[![docs-rs]](https://docs.rs/itoa)



<br>

This crate provides a fast conversion of integer primitives to decimal
strings. The implementation comes straight from [libcore](#libcore) but avoids the
performance penalty of going through `core::fmt::Formatter`.

See also [`ryu`](#ryu) for printing floating point primitives.


# Example

```rust
fn main() {
    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    assert_eq!(printed, "128");
}
```

# Performance (lower is better)

![performance](https://raw.githubusercontent.com/dtolnay/itoa/master/performance.png)

## Structs

### `Buffer`

```rust
struct Buffer {
    bytes: [core::mem::MaybeUninit<u8>; 40],
}
```

A correctly sized stack allocation for the formatted integer to be written
into.

# Example

```rust
let mut buffer = itoa::Buffer::new();
let printed = buffer.format(1234);
assert_eq!(printed, "1234");
```

#### Implementations

- `fn new() -> Buffer` — [`Buffer`](#buffer)

- `fn format<I: Integer>(self: &mut Self, i: I) -> &str`

#### Trait Implementations

##### `impl Clone for Buffer`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- `fn default() -> Buffer` — [`Buffer`](#buffer)

## Traits

### `Integer`

```rust
trait Integer: private::Sealed { ... }
```

An integer that can be written into an [`itoa::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of itoa.

#### Required Methods

- `const MAX_STR_LEN: usize`

