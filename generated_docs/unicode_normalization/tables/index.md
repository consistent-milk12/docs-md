*[unicode_normalization](../index.md) / [tables](index.md)*

---

# Module `tables`

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

