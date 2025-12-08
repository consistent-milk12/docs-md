*[unicode_normalization](../index.md) / [tables](index.md)*

---

# Module `tables`

## Contents

- [Functions](#functions)
  - [`composition_table_astral`](#composition_table_astral)
  - [`is_public_assigned`](#is_public_assigned)
  - [`qc_nfc`](#qc_nfc)
  - [`qc_nfkc`](#qc_nfkc)
  - [`qc_nfd`](#qc_nfd)
  - [`qc_nfkd`](#qc_nfkd)
  - [`stream_safe_leading_nonstarters`](#stream_safe_leading_nonstarters)
- [Constants](#constants)
  - [`UNICODE_VERSION`](#unicode_version)
  - [`CANONICAL_COMBINING_CLASS_SALT`](#canonical_combining_class_salt)
  - [`CANONICAL_COMBINING_CLASS_KV`](#canonical_combining_class_kv)
  - [`COMPOSITION_TABLE_SALT`](#composition_table_salt)
  - [`COMPOSITION_TABLE_KV`](#composition_table_kv)
  - [`CANONICAL_DECOMPOSED_CHARS`](#canonical_decomposed_chars)
  - [`CANONICAL_DECOMPOSED_SALT`](#canonical_decomposed_salt)
  - [`CANONICAL_DECOMPOSED_KV`](#canonical_decomposed_kv)
  - [`COMPATIBILITY_DECOMPOSED_CHARS`](#compatibility_decomposed_chars)
  - [`COMPATIBILITY_DECOMPOSED_SALT`](#compatibility_decomposed_salt)
  - [`COMPATIBILITY_DECOMPOSED_KV`](#compatibility_decomposed_kv)
  - [`CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS`](#cjk_compat_variants_decomposed_chars)
  - [`CJK_COMPAT_VARIANTS_DECOMPOSED_SALT`](#cjk_compat_variants_decomposed_salt)
  - [`CJK_COMPAT_VARIANTS_DECOMPOSED_KV`](#cjk_compat_variants_decomposed_kv)
  - [`COMBINING_MARK_SALT`](#combining_mark_salt)
  - [`COMBINING_MARK_KV`](#combining_mark_kv)
  - [`TRAILING_NONSTARTERS_SALT`](#trailing_nonstarters_salt)
  - [`TRAILING_NONSTARTERS_KV`](#trailing_nonstarters_kv)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`composition_table_astral`](#composition_table_astral) | fn |  |
| [`is_public_assigned`](#is_public_assigned) | fn |  |
| [`qc_nfc`](#qc_nfc) | fn |  |
| [`qc_nfkc`](#qc_nfkc) | fn |  |
| [`qc_nfd`](#qc_nfd) | fn |  |
| [`qc_nfkd`](#qc_nfkd) | fn |  |
| [`stream_safe_leading_nonstarters`](#stream_safe_leading_nonstarters) | fn |  |
| [`UNICODE_VERSION`](#unicode_version) | const |  |
| [`CANONICAL_COMBINING_CLASS_SALT`](#canonical_combining_class_salt) | const |  |
| [`CANONICAL_COMBINING_CLASS_KV`](#canonical_combining_class_kv) | const |  |
| [`COMPOSITION_TABLE_SALT`](#composition_table_salt) | const |  |
| [`COMPOSITION_TABLE_KV`](#composition_table_kv) | const |  |
| [`CANONICAL_DECOMPOSED_CHARS`](#canonical_decomposed_chars) | const |  |
| [`CANONICAL_DECOMPOSED_SALT`](#canonical_decomposed_salt) | const |  |
| [`CANONICAL_DECOMPOSED_KV`](#canonical_decomposed_kv) | const |  |
| [`COMPATIBILITY_DECOMPOSED_CHARS`](#compatibility_decomposed_chars) | const |  |
| [`COMPATIBILITY_DECOMPOSED_SALT`](#compatibility_decomposed_salt) | const |  |
| [`COMPATIBILITY_DECOMPOSED_KV`](#compatibility_decomposed_kv) | const |  |
| [`CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS`](#cjk_compat_variants_decomposed_chars) | const |  |
| [`CJK_COMPAT_VARIANTS_DECOMPOSED_SALT`](#cjk_compat_variants_decomposed_salt) | const |  |
| [`CJK_COMPAT_VARIANTS_DECOMPOSED_KV`](#cjk_compat_variants_decomposed_kv) | const |  |
| [`COMBINING_MARK_SALT`](#combining_mark_salt) | const |  |
| [`COMBINING_MARK_KV`](#combining_mark_kv) | const |  |
| [`TRAILING_NONSTARTERS_SALT`](#trailing_nonstarters_salt) | const |  |
| [`TRAILING_NONSTARTERS_KV`](#trailing_nonstarters_kv) | const |  |

## Functions

### `composition_table_astral`

```rust
fn composition_table_astral(c1: char, c2: char) -> Option<char>
```

### `is_public_assigned`

```rust
fn is_public_assigned(c: char) -> bool
```

### `qc_nfc`

```rust
fn qc_nfc(c: char) -> crate::quick_check::IsNormalized
```

### `qc_nfkc`

```rust
fn qc_nfkc(c: char) -> crate::quick_check::IsNormalized
```

### `qc_nfd`

```rust
fn qc_nfd(c: char) -> crate::quick_check::IsNormalized
```

### `qc_nfkd`

```rust
fn qc_nfkd(c: char) -> crate::quick_check::IsNormalized
```

### `stream_safe_leading_nonstarters`

```rust
fn stream_safe_leading_nonstarters(c: char) -> usize
```

## Constants

### `UNICODE_VERSION`

```rust
const UNICODE_VERSION: (u8, u8, u8);
```

### `CANONICAL_COMBINING_CLASS_SALT`

```rust
const CANONICAL_COMBINING_CLASS_SALT: &[u16];
```

### `CANONICAL_COMBINING_CLASS_KV`

```rust
const CANONICAL_COMBINING_CLASS_KV: &[u32];
```

### `COMPOSITION_TABLE_SALT`

```rust
const COMPOSITION_TABLE_SALT: &[u16];
```

### `COMPOSITION_TABLE_KV`

```rust
const COMPOSITION_TABLE_KV: &[(u32, char)];
```

### `CANONICAL_DECOMPOSED_CHARS`

```rust
const CANONICAL_DECOMPOSED_CHARS: &[char];
```

### `CANONICAL_DECOMPOSED_SALT`

```rust
const CANONICAL_DECOMPOSED_SALT: &[u16];
```

### `CANONICAL_DECOMPOSED_KV`

```rust
const CANONICAL_DECOMPOSED_KV: &[(u32, (u16, u16))];
```

### `COMPATIBILITY_DECOMPOSED_CHARS`

```rust
const COMPATIBILITY_DECOMPOSED_CHARS: &[char];
```

### `COMPATIBILITY_DECOMPOSED_SALT`

```rust
const COMPATIBILITY_DECOMPOSED_SALT: &[u16];
```

### `COMPATIBILITY_DECOMPOSED_KV`

```rust
const COMPATIBILITY_DECOMPOSED_KV: &[(u32, (u16, u16))];
```

### `CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS`

```rust
const CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS: &[char];
```

### `CJK_COMPAT_VARIANTS_DECOMPOSED_SALT`

```rust
const CJK_COMPAT_VARIANTS_DECOMPOSED_SALT: &[u16];
```

### `CJK_COMPAT_VARIANTS_DECOMPOSED_KV`

```rust
const CJK_COMPAT_VARIANTS_DECOMPOSED_KV: &[(u32, (u16, u16))];
```

### `COMBINING_MARK_SALT`

```rust
const COMBINING_MARK_SALT: &[u16];
```

### `COMBINING_MARK_KV`

```rust
const COMBINING_MARK_KV: &[u32];
```

### `TRAILING_NONSTARTERS_SALT`

```rust
const TRAILING_NONSTARTERS_SALT: &[u16];
```

### `TRAILING_NONSTARTERS_KV`

```rust
const TRAILING_NONSTARTERS_KV: &[u32];
```

