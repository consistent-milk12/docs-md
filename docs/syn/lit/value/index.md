*[syn](../../index.md) / [lit](../index.md) / [value](index.md)*

---

# Module `value`

## Functions

### `byte`

```rust
fn byte<S: AsRef<[u8]> + ?Sized>(s: &S, idx: usize) -> u8
```

Get the byte at offset idx, or a default of `b'\0'` if we're looking
past the end of the input buffer.

### `next_chr`

```rust
fn next_chr(s: &str) -> char
```

### `parse_lit_str`

```rust
fn parse_lit_str(s: &str) -> Option<(Box<str>, Box<str>)>
```

### `parse_lit_str_cooked`

```rust
fn parse_lit_str_cooked(s: &str) -> Option<(Box<str>, Box<str>)>
```

### `parse_lit_str_raw`

```rust
fn parse_lit_str_raw(s: &str) -> Option<(Box<str>, Box<str>)>
```

### `parse_lit_byte_str`

```rust
fn parse_lit_byte_str(s: &str) -> Option<(Vec<u8>, Box<str>)>
```

### `parse_lit_byte_str_cooked`

```rust
fn parse_lit_byte_str_cooked(s: &str) -> Option<(Vec<u8>, Box<str>)>
```

### `parse_lit_byte_str_raw`

```rust
fn parse_lit_byte_str_raw(s: &str) -> Option<(Vec<u8>, Box<str>)>
```

### `parse_lit_c_str`

```rust
fn parse_lit_c_str(s: &str) -> Option<(std::ffi::CString, Box<str>)>
```

### `parse_lit_c_str_cooked`

```rust
fn parse_lit_c_str_cooked(s: &str) -> Option<(std::ffi::CString, Box<str>)>
```

### `parse_lit_c_str_raw`

```rust
fn parse_lit_c_str_raw(s: &str) -> Option<(std::ffi::CString, Box<str>)>
```

### `parse_lit_byte`

```rust
fn parse_lit_byte(s: &str) -> Option<(u8, Box<str>)>
```

### `parse_lit_char`

```rust
fn parse_lit_char(s: &str) -> Option<(char, Box<str>)>
```

### `backslash_x`

```rust
fn backslash_x<S>(s: &S) -> Option<(u8, &S)>
where
    S: Index<std::ops::RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized
```

### `backslash_u`

```rust
fn backslash_u<S>(s: &S) -> Option<(char, &S)>
where
    S: Index<std::ops::RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized
```

### `parse_lit_int`

```rust
fn parse_lit_int(s: &str) -> Option<(Box<str>, Box<str>)>
```

### `parse_lit_float`

```rust
fn parse_lit_float(input: &str) -> Option<(Box<str>, Box<str>)>
```

