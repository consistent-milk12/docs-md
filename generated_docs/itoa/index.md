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
  - [`DEC_DIGITS_LUT`](#dec-digits-lut)
- [Macros](#macros)
  - [`impl_Integer!`](#impl-integer)
  - [`impl_Integer_size!`](#impl-integer-size)
  - [`impl_Integer128!`](#impl-integer128)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`udiv128`](#udiv128) | mod |  |
| [`private`](#private) | mod |  |
| [`Buffer`](#buffer) | struct | A correctly sized stack allocation for the formatted integer to be written into. |
| [`Integer`](#integer) | trait | An integer that can be written into an [`itoa::Buffer`][Buffer]. |
| [`DEC_DIGITS_LUT`](#dec-digits-lut) | const |  |
| [`impl_Integer!`](#impl-integer) | macro |  |
| [`impl_Integer_size!`](#impl-integer-size) | macro |  |
| [`impl_Integer128!`](#impl-integer128) | macro |  |

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

*Defined in [`itoa-1.0.15/src/lib.rs:63-65`](../../.source_1765633015/itoa-1.0.15/src/lib.rs#L63-L65)*

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

  This is a cheap operation; you don't need to worry about reusing buffers

  for efficiency.

- <span id="buffer-format"></span>`fn format<I: Integer>(&mut self, i: I) -> &str`

  Print an integer into this buffer and return a reference to its string

  representation within the buffer.

#### Trait Implementations

##### `impl Any for Buffer`

- <span id="buffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Buffer`

- <span id="buffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Buffer`

- <span id="buffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Buffer`

- <span id="buffer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Buffer`

- <span id="buffer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- <span id="buffer-default"></span>`fn default() -> Buffer` — [`Buffer`](#buffer)

##### `impl<T> From for Buffer`

- <span id="buffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Buffer`

- <span id="buffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Buffer`

- <span id="buffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Buffer`

- <span id="buffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Integer`

```rust
trait Integer: private::Sealed { ... }
```

*Defined in [`itoa-1.0.15/src/lib.rs:112-116`](../../.source_1765633015/itoa-1.0.15/src/lib.rs#L112-L116)*

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

*Defined in [`itoa-1.0.15/src/lib.rs:128-133`](../../.source_1765633015/itoa-1.0.15/src/lib.rs#L128-L133)*

## Macros

### `impl_Integer!`

*Defined in [`itoa-1.0.15/src/lib.rs:137-212`](../../.source_1765633015/itoa-1.0.15/src/lib.rs#L137-L212)*

### `impl_Integer_size!`

*Defined in [`itoa-1.0.15/src/lib.rs:223-241`](../../.source_1765633015/itoa-1.0.15/src/lib.rs#L223-L241)*

### `impl_Integer128!`

*Defined in [`itoa-1.0.15/src/lib.rs:250-326`](../../.source_1765633015/itoa-1.0.15/src/lib.rs#L250-L326)*

