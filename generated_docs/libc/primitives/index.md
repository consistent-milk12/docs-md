*[libc](../index.md) / [primitives](index.md)*

---

# Module `primitives`

This module contains type aliases for C's platform-specific types
and fixed-width integer types.

The platform-specific types definitions were taken from rust-lang/rust in
library/core/src/ffi/primitives.rs

The fixed-width integer aliases are deprecated: use the Rust types instead.

## Contents

- [Type Aliases](#type-aliases)
  - [`c_schar`](#c_schar)
  - [`c_uchar`](#c_uchar)
  - [`c_short`](#c_short)
  - [`c_ushort`](#c_ushort)
  - [`c_longlong`](#c_longlong)
  - [`c_ulonglong`](#c_ulonglong)
  - [`c_float`](#c_float)
  - [`c_double`](#c_double)
  - [`c_char`](#c_char)
  - [`c_int`](#c_int)
  - [`c_uint`](#c_uint)
  - [`c_long`](#c_long)
  - [`c_ulong`](#c_ulong)
  - [`int8_t`](#int8_t)
  - [`int16_t`](#int16_t)
  - [`int32_t`](#int32_t)
  - [`int64_t`](#int64_t)
  - [`uint8_t`](#uint8_t)
  - [`uint16_t`](#uint16_t)
  - [`uint32_t`](#uint32_t)
  - [`uint64_t`](#uint64_t)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`c_schar`](#c_schar) | type |  |
| [`c_uchar`](#c_uchar) | type |  |
| [`c_short`](#c_short) | type |  |
| [`c_ushort`](#c_ushort) | type |  |
| [`c_longlong`](#c_longlong) | type |  |
| [`c_ulonglong`](#c_ulonglong) | type |  |
| [`c_float`](#c_float) | type |  |
| [`c_double`](#c_double) | type |  |
| [`c_char`](#c_char) | type |  |
| [`c_int`](#c_int) | type |  |
| [`c_uint`](#c_uint) | type |  |
| [`c_long`](#c_long) | type |  |
| [`c_ulong`](#c_ulong) | type |  |
| [`int8_t`](#int8_t) | type |  |
| [`int16_t`](#int16_t) | type |  |
| [`int32_t`](#int32_t) | type |  |
| [`int64_t`](#int64_t) | type |  |
| [`uint8_t`](#uint8_t) | type |  |
| [`uint16_t`](#uint16_t) | type |  |
| [`uint32_t`](#uint32_t) | type |  |
| [`uint64_t`](#uint64_t) | type |  |

## Type Aliases

### `c_schar`

```rust
type c_schar = i8;
```

### `c_uchar`

```rust
type c_uchar = u8;
```

### `c_short`

```rust
type c_short = i16;
```

### `c_ushort`

```rust
type c_ushort = u16;
```

### `c_longlong`

```rust
type c_longlong = i64;
```

### `c_ulonglong`

```rust
type c_ulonglong = u64;
```

### `c_float`

```rust
type c_float = f32;
```

### `c_double`

```rust
type c_double = f64;
```

### `c_char`

```rust
type c_char = i8;
```

### `c_int`

```rust
type c_int = i32;
```

### `c_uint`

```rust
type c_uint = u32;
```

### `c_long`

```rust
type c_long = i64;
```

### `c_ulong`

```rust
type c_ulong = u64;
```

### `int8_t`

```rust
type int8_t = i8;
```

### `int16_t`

```rust
type int16_t = i16;
```

### `int32_t`

```rust
type int32_t = i32;
```

### `int64_t`

```rust
type int64_t = i64;
```

### `uint8_t`

```rust
type uint8_t = u8;
```

### `uint16_t`

```rust
type uint16_t = u16;
```

### `uint32_t`

```rust
type uint32_t = u32;
```

### `uint64_t`

```rust
type uint64_t = u64;
```

