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
  - [`c_schar`](#c-schar)
  - [`c_uchar`](#c-uchar)
  - [`c_short`](#c-short)
  - [`c_ushort`](#c-ushort)
  - [`c_longlong`](#c-longlong)
  - [`c_ulonglong`](#c-ulonglong)
  - [`c_float`](#c-float)
  - [`c_double`](#c-double)
  - [`c_char`](#c-char)
  - [`c_int`](#c-int)
  - [`c_uint`](#c-uint)
  - [`c_long`](#c-long)
  - [`c_ulong`](#c-ulong)
  - [`int8_t`](#int8-t)
  - [`int16_t`](#int16-t)
  - [`int32_t`](#int32-t)
  - [`int64_t`](#int64-t)
  - [`uint8_t`](#uint8-t)
  - [`uint16_t`](#uint16-t)
  - [`uint32_t`](#uint32-t)
  - [`uint64_t`](#uint64-t)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`c_schar`](#c-schar) | type |  |
| [`c_uchar`](#c-uchar) | type |  |
| [`c_short`](#c-short) | type |  |
| [`c_ushort`](#c-ushort) | type |  |
| [`c_longlong`](#c-longlong) | type |  |
| [`c_ulonglong`](#c-ulonglong) | type |  |
| [`c_float`](#c-float) | type |  |
| [`c_double`](#c-double) | type |  |
| [`c_char`](#c-char) | type |  |
| [`c_int`](#c-int) | type |  |
| [`c_uint`](#c-uint) | type |  |
| [`c_long`](#c-long) | type |  |
| [`c_ulong`](#c-ulong) | type |  |
| [`int8_t`](#int8-t) | type |  |
| [`int16_t`](#int16-t) | type |  |
| [`int32_t`](#int32-t) | type |  |
| [`int64_t`](#int64-t) | type |  |
| [`uint8_t`](#uint8-t) | type |  |
| [`uint16_t`](#uint16-t) | type |  |
| [`uint32_t`](#uint32-t) | type |  |
| [`uint64_t`](#uint64-t) | type |  |

## Type Aliases

### `c_schar`

```rust
type c_schar = i8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:9`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L9)*

### `c_uchar`

```rust
type c_uchar = u8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:10`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L10)*

### `c_short`

```rust
type c_short = i16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:11`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L11)*

### `c_ushort`

```rust
type c_ushort = u16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:12`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L12)*

### `c_longlong`

```rust
type c_longlong = i64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:14`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L14)*

### `c_ulonglong`

```rust
type c_ulonglong = u64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:15`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L15)*

### `c_float`

```rust
type c_float = f32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:17`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L17)*

### `c_double`

```rust
type c_double = f64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:18`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L18)*

### `c_char`

```rust
type c_char = i8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:42`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L42)*

### `c_int`

```rust
type c_int = i32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:51`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L51)*

### `c_uint`

```rust
type c_uint = u32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:52`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L52)*

### `c_long`

```rust
type c_long = i64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:58`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L58)*

### `c_ulong`

```rust
type c_ulong = u64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:59`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L59)*

### `int8_t`

```rust
type int8_t = i8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:68`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L68)*

### `int16_t`

```rust
type int16_t = i16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:70`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L70)*

### `int32_t`

```rust
type int32_t = i32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:72`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L72)*

### `int64_t`

```rust
type int64_t = i64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:74`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L74)*

### `uint8_t`

```rust
type uint8_t = u8;
```

*Defined in [`libc-0.2.178/src/primitives.rs:76`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L76)*

### `uint16_t`

```rust
type uint16_t = u16;
```

*Defined in [`libc-0.2.178/src/primitives.rs:78`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L78)*

### `uint32_t`

```rust
type uint32_t = u32;
```

*Defined in [`libc-0.2.178/src/primitives.rs:80`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L80)*

### `uint64_t`

```rust
type uint64_t = u64;
```

*Defined in [`libc-0.2.178/src/primitives.rs:82`](../../../.source_1765210505/libc-0.2.178/src/primitives.rs#L82)*

