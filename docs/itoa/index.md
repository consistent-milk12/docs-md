# Crate `itoa`

[![github](#github)
](https://github.com/dtolnay/itoa)&ensp;[![crates-io]](https://crates.io/crates/itoa)&ensp;[![docs-rs]](https://docs.rs/itoa)

[github](#github)
: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

This crate provides a fast conversion of integer primitives to decimal
strings. The implementation comes straight from [libcore](#libcore)
 but avoids the
performance penalty of going through `core::fmt::Formatter`.

See also [`ryu`](../ryu/index.md) for printing floating point primitives.

[libcore](#libcore)
: https://github.com/rust-lang/rust/blob/b8214dc6c6fc20d0a660fb5700dca9ebf51ebe89/src/libcore/fmt/num.rs#L201-L254

# Example

```
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
    // [REDACTED: Private Fields]
}
```

A correctly sized stack allocation for the formatted integer to be written
into.

# Example

```
let mut buffer = itoa::Buffer::new();
let printed = buffer.format(1234);
assert_eq!(printed, "1234");
```

#### Implementations

- `fn new() -> Buffer`
  This is a cheap operation; you don't need to worry about reusing buffers

- `fn format<I: Integer>(self: &mut Self, i: I) -> &str`
  Print an integer into this buffer and return a reference to its string

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default`

- `fn default() -> Buffer`

## Traits

### `Integer`

```rust
trait Integer: private::Sealed { ... }
```

An integer that can be written into an [`itoa::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of itoa.

#### Required Methods

- `const MAX_STR_LEN: usize`

