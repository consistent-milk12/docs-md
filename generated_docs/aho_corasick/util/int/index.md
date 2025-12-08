*[aho_corasick](../../index.md) / [util](../index.md) / [int](index.md)*

---

# Module `int`

This module provides several integer oriented traits for converting between
both fixed size integers and integers whose size varies based on the target
(like `usize`).

The main design principle for this module is to centralize all uses of `as`.
The thinking here is that `as` makes it very easy to perform accidental lossy
conversions, and if we centralize all its uses here under more descriptive
higher level operations, its use and correctness becomes easier to audit.

This was copied mostly wholesale from `regex-automata`.

NOTE: for simplicity, we don't take target pointer width into account here for
`usize` conversions. Since we currently only panic in debug mode, skipping the
check when it can be proven it isn't needed at compile time doesn't really
matter. Now, if we wind up wanting to do as many checks as possible in release
mode, then we would want to skip those when we know the conversions are always
non-lossy.

## Contents

- [Traits](#traits)
  - [`U8`](#u8)
  - [`U16`](#u16)
  - [`U32`](#u32)
  - [`U64`](#u64)
  - [`I8`](#i8)
  - [`I32`](#i32)
  - [`I64`](#i64)
  - [`Usize`](#usize)
  - [`Pointer`](#pointer)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`U8`](#u8) | trait |  |
| [`U16`](#u16) | trait |  |
| [`U32`](#u32) | trait |  |
| [`U64`](#u64) | trait |  |
| [`I8`](#i8) | trait |  |
| [`I32`](#i32) | trait |  |
| [`I64`](#i64) | trait |  |
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

### `I8`

```rust
trait I8 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

- `fn to_bits(self) -> u8`

- `fn from_bits(n: u8) -> i8`

### `I32`

```rust
trait I32 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

- `fn to_bits(self) -> u32`

- `fn from_bits(n: u32) -> i32`

### `I64`

```rust
trait I64 { ... }
```

#### Required Methods

- `fn as_usize(self) -> usize`

- `fn to_bits(self) -> u64`

- `fn from_bits(n: u64) -> i64`

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

