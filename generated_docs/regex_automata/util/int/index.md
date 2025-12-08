*[regex_automata](../../index.md) / [util](../index.md) / [int](index.md)*

---

# Module `int`

This module provides several integer oriented traits for converting between
both fixed size integers and integers whose size varies based on the target
(like `usize`).

The driving design principle of this module is to attempt to centralize as many
`as` casts as possible here. And in particular, we separate casts into two
buckets:

* Casts that we use for their truncating behavior. In this case, we use more
descriptive names, like `low_u32` and `high_u32`.
* Casts that we use for converting back-and-forth between `usize`. These
conversions are generally necessary because we often store indices in different
formats to save on memory, which requires converting to and from `usize`. In
this case, we very specifically do not want to overflow, and so the methods
defined here will panic if the `as` cast would be lossy in debug mode. (A
normal `as` cast will never panic!)

For `as` casts between raw pointers, we use `cast`, so `as` isn't needed there.

For regex engines, floating point is just never used, so we don't have to worry
about `as` casts for those.

Otherwise, this module pretty much covers all of our `as` needs except for one
thing: const contexts. There are a select few places in this crate where we
still need to use `as` because const functions on traits aren't stable yet.
If we wind up significantly expanding our const footprint in this crate, it
might be worth defining free functions to handle those cases. But at the time
of writing, that just seemed like too much ceremony. Instead, I comment each
such use of `as` in a const context with a "fixme" notice.

NOTE: for simplicity, we don't take target pointer width into account here for
`usize` conversions. Since we currently only panic in debug mode, skipping the
check when it can be proven it isn't needed at compile time doesn't really
matter. Now, if we wind up wanting to do as many checks as possible in release
mode, then we would want to skip those when we know the conversions are always
non-lossy.

NOTE: this module isn't an exhaustive API. For example, we still use things
like `u64::from` where possible, or even `usize::try_from()` for when we do
explicitly want to panic or when we want to return an error for overflow.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`U8`](#u8) | trait |  |
| [`U16`](#u16) | trait |  |
| [`U32`](#u32) | trait |  |
| [`U64`](#u64) | trait |  |
| [`I32`](#i32) | trait |  |
| [`Usize`](#usize) | trait |  |
| [`Pointer`](#pointer) | trait |  |

## Traits

### `U8`

```rust
trait U8 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

### `U16`

```rust
trait U16 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

- `fn low_u8(self) -> u8`

- `fn high_u8(self) -> u8`

### `U32`

```rust
trait U32 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

- `fn low_u8(self) -> u8`

- `fn low_u16(self) -> u16`

- `fn high_u16(self) -> u16`

### `U64`

```rust
trait U64 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

- `fn low_u8(self) -> u8`

- `fn low_u16(self) -> u16`

- `fn low_u32(self) -> u32`

- `fn high_u32(self) -> u32`

### `I32`

```rust
trait I32 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

- `fn to_bits(self) -> u32`

- `fn from_bits(n: u32) -> i32`

### `Usize`

```rust
trait Usize { ... }
```

#### Required Methods

- `fn as_u8(self) -> u8`

- `fn as_u16(self) -> u16`

- `fn as_u32(self) -> u32`

- `fn as_u64(self) -> u64`

### `Pointer`

```rust
trait Pointer { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

