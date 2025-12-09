# Crate `itoa`

[![github]](https://github.com/dtolnay/itoa)&ensp;[![crates-io]](https://crates.io/crates/itoa)&ensp;[![docs-rs]](https://docs.rs/itoa)



<br>

This crate provides a fast conversion of integer primitives to decimal
strings. The implementation comes straight from [libcore] but avoids the
performance penalty of going through [`core::fmt::Formatter`](../regex_syntax/error/index.md).

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

## Contents

- [Modules](#modules)
  - [`udiv128`](#udiv128)
  - [`private`](#private)
- [Structs](#structs)
  - [`Buffer`](#buffer)
- [Traits](#traits)
  - [`Integer`](#integer)
- [Constants](#constants)
  - [`DEC_DIGITS_LUT`](#dec_digits_lut)
- [Macros](#macros)
  - [`impl_Integer!`](#impl_integer)
  - [`impl_Integer_size!`](#impl_integer_size)
  - [`impl_Integer128!`](#impl_integer128)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`udiv128`](#udiv128) | mod |  |
| [`private`](#private) | mod |  |
| [`Buffer`](#buffer) | struct | A correctly sized stack allocation for the formatted integer to be written |
| [`Integer`](#integer) | trait | An integer that can be written into an [`itoa::Buffer`][Buffer]. |
| [`DEC_DIGITS_LUT`](#dec_digits_lut) | const |  |
| [`impl_Integer!`](#impl_integer) | macro |  |
| [`impl_Integer_size!`](#impl_integer_size) | macro |  |
| [`impl_Integer128!`](#impl_integer128) | macro |  |

## Modules

- [`udiv128`](udiv128/index.md)
- [`private`](private/index.md)

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

- <span id="buffer-new"></span>`fn new() -> Buffer` — [`Buffer`](#buffer)

- <span id="buffer-format"></span>`fn format<I: Integer>(&mut self, i: I) -> &str`

#### Trait Implementations

##### `impl Clone for Buffer`

- <span id="buffer-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- <span id="buffer-default"></span>`fn default() -> Buffer` — [`Buffer`](#buffer)

## Traits

### `Integer`

```rust
trait Integer: private::Sealed { ... }
```

An integer that can be written into an [`itoa::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of itoa.

#### Associated Constants

- `const MAX_STR_LEN: usize`

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Constants

### `DEC_DIGITS_LUT`

```rust
const DEC_DIGITS_LUT: [u8; 200];
```

## Macros

### `impl_Integer!`

### `impl_Integer_size!`

### `impl_Integer128!`

