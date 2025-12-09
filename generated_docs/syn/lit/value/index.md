*[syn](../../index.md) / [lit](../index.md) / [value](index.md)*

---

# Module `value`

## Contents

- [Functions](#functions)
  - [`byte`](#byte)
  - [`next_chr`](#next_chr)
  - [`parse_lit_str`](#parse_lit_str)
  - [`parse_lit_str_cooked`](#parse_lit_str_cooked)
  - [`parse_lit_str_raw`](#parse_lit_str_raw)
  - [`parse_lit_byte_str`](#parse_lit_byte_str)
  - [`parse_lit_byte_str_cooked`](#parse_lit_byte_str_cooked)
  - [`parse_lit_byte_str_raw`](#parse_lit_byte_str_raw)
  - [`parse_lit_c_str`](#parse_lit_c_str)
  - [`parse_lit_c_str_cooked`](#parse_lit_c_str_cooked)
  - [`parse_lit_c_str_raw`](#parse_lit_c_str_raw)
  - [`parse_lit_byte`](#parse_lit_byte)
  - [`parse_lit_char`](#parse_lit_char)
  - [`backslash_x`](#backslash_x)
  - [`backslash_u`](#backslash_u)
  - [`parse_lit_int`](#parse_lit_int)
  - [`parse_lit_float`](#parse_lit_float)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`byte`](#byte) | fn | Get the byte at offset idx, or a default of `b'\0'` if we're looking past the end of the input buffer. |
| [`next_chr`](#next_chr) | fn |  |
| [`parse_lit_str`](#parse_lit_str) | fn |  |
| [`parse_lit_str_cooked`](#parse_lit_str_cooked) | fn |  |
| [`parse_lit_str_raw`](#parse_lit_str_raw) | fn |  |
| [`parse_lit_byte_str`](#parse_lit_byte_str) | fn |  |
| [`parse_lit_byte_str_cooked`](#parse_lit_byte_str_cooked) | fn |  |
| [`parse_lit_byte_str_raw`](#parse_lit_byte_str_raw) | fn |  |
| [`parse_lit_c_str`](#parse_lit_c_str) | fn |  |
| [`parse_lit_c_str_cooked`](#parse_lit_c_str_cooked) | fn |  |
| [`parse_lit_c_str_raw`](#parse_lit_c_str_raw) | fn |  |
| [`parse_lit_byte`](#parse_lit_byte) | fn |  |
| [`parse_lit_char`](#parse_lit_char) | fn |  |
| [`backslash_x`](#backslash_x) | fn |  |
| [`backslash_u`](#backslash_u) | fn |  |
| [`parse_lit_int`](#parse_lit_int) | fn |  |
| [`parse_lit_float`](#parse_lit_float) | fn |  |

## Functions

### `byte`

```rust
fn byte<S: AsRef<[u8]> + ?Sized>(s: &S, idx: usize) -> u8
```

*Defined in [`syn-2.0.111/src/lit.rs:1286-1293`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1286-L1293)*

Get the byte at offset idx, or a default of `b'\0'` if we're looking
past the end of the input buffer.

### `next_chr`

```rust
fn next_chr(s: &str) -> char
```

*Defined in [`syn-2.0.111/src/lit.rs:1295-1297`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1295-L1297)*

### `parse_lit_str`

```rust
fn parse_lit_str(s: &str) -> Option<(Box<str>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1300-1306`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1300-L1306)*

### `parse_lit_str_cooked`

```rust
fn parse_lit_str_cooked(s: &str) -> Option<(Box<str>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1308-1375`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1308-L1375)*

### `parse_lit_str_raw`

```rust
fn parse_lit_str_raw(s: &str) -> Option<(Box<str>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1377-1399`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1377-L1399)*

### `parse_lit_byte_str`

```rust
fn parse_lit_byte_str(s: &str) -> Option<(Vec<u8>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1402-1409`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1402-L1409)*

### `parse_lit_byte_str_cooked`

```rust
fn parse_lit_byte_str_cooked(s: &str) -> Option<(Vec<u8>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1411-1472`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1411-L1472)*

### `parse_lit_byte_str_raw`

```rust
fn parse_lit_byte_str_raw(s: &str) -> Option<(Vec<u8>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1474-1478`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1474-L1478)*

### `parse_lit_c_str`

```rust
fn parse_lit_c_str(s: &str) -> Option<(std::ffi::CString, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1481-1488`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1481-L1488)*

### `parse_lit_c_str_cooked`

```rust
fn parse_lit_c_str_cooked(s: &str) -> Option<(std::ffi::CString, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1490-1565`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1490-L1565)*

### `parse_lit_c_str_raw`

```rust
fn parse_lit_c_str_raw(s: &str) -> Option<(std::ffi::CString, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1567-1572`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1567-L1572)*

### `parse_lit_byte`

```rust
fn parse_lit_byte(s: &str) -> Option<(u8, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1575-1617`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1575-L1617)*

### `parse_lit_char`

```rust
fn parse_lit_char(s: &str) -> Option<(char, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1620-1669`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1620-L1669)*

### `backslash_x`

```rust
fn backslash_x<S>(s: &S) -> Option<(u8, &S)>
where
    S: Index<std::ops::RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized
```

*Defined in [`syn-2.0.111/src/lit.rs:1671-1692`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1671-L1692)*

### `backslash_u`

```rust
fn backslash_u<S>(s: &S) -> Option<(char, &S)>
where
    S: Index<std::ops::RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized
```

*Defined in [`syn-2.0.111/src/lit.rs:1694-1734`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1694-L1734)*

### `parse_lit_int`

```rust
fn parse_lit_int(s: &str) -> Option<(Box<str>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1737-1825`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1737-L1825)*

### `parse_lit_float`

```rust
fn parse_lit_float(input: &str) -> Option<(Box<str>, Box<str>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:1828-1917`](../../../../.source_1765210505/syn-2.0.111/src/lit.rs#L1828-L1917)*

