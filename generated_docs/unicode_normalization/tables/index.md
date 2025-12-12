*[unicode_normalization](../index.md) / [tables](index.md)*

---

# Module `tables`

## Contents

- [Functions](#functions)
  - [`composition_table_astral`](#composition-table-astral)
  - [`is_public_assigned`](#is-public-assigned)
  - [`qc_nfc`](#qc-nfc)
  - [`qc_nfkc`](#qc-nfkc)
  - [`qc_nfd`](#qc-nfd)
  - [`qc_nfkd`](#qc-nfkd)
  - [`stream_safe_leading_nonstarters`](#stream-safe-leading-nonstarters)
- [Constants](#constants)
  - [`UNICODE_VERSION`](#unicode-version)
  - [`CANONICAL_COMBINING_CLASS_SALT`](#canonical-combining-class-salt)
  - [`CANONICAL_COMBINING_CLASS_KV`](#canonical-combining-class-kv)
  - [`COMPOSITION_TABLE_SALT`](#composition-table-salt)
  - [`COMPOSITION_TABLE_KV`](#composition-table-kv)
  - [`CANONICAL_DECOMPOSED_CHARS`](#canonical-decomposed-chars)
  - [`CANONICAL_DECOMPOSED_SALT`](#canonical-decomposed-salt)
  - [`CANONICAL_DECOMPOSED_KV`](#canonical-decomposed-kv)
  - [`COMPATIBILITY_DECOMPOSED_CHARS`](#compatibility-decomposed-chars)
  - [`COMPATIBILITY_DECOMPOSED_SALT`](#compatibility-decomposed-salt)
  - [`COMPATIBILITY_DECOMPOSED_KV`](#compatibility-decomposed-kv)
  - [`CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS`](#cjk-compat-variants-decomposed-chars)
  - [`CJK_COMPAT_VARIANTS_DECOMPOSED_SALT`](#cjk-compat-variants-decomposed-salt)
  - [`CJK_COMPAT_VARIANTS_DECOMPOSED_KV`](#cjk-compat-variants-decomposed-kv)
  - [`COMBINING_MARK_SALT`](#combining-mark-salt)
  - [`COMBINING_MARK_KV`](#combining-mark-kv)
  - [`TRAILING_NONSTARTERS_SALT`](#trailing-nonstarters-salt)
  - [`TRAILING_NONSTARTERS_KV`](#trailing-nonstarters-kv)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`composition_table_astral`](#composition-table-astral) | fn |  |
| [`is_public_assigned`](#is-public-assigned) | fn |  |
| [`qc_nfc`](#qc-nfc) | fn |  |
| [`qc_nfkc`](#qc-nfkc) | fn |  |
| [`qc_nfd`](#qc-nfd) | fn |  |
| [`qc_nfkd`](#qc-nfkd) | fn |  |
| [`stream_safe_leading_nonstarters`](#stream-safe-leading-nonstarters) | fn |  |
| [`UNICODE_VERSION`](#unicode-version) | const |  |
| [`CANONICAL_COMBINING_CLASS_SALT`](#canonical-combining-class-salt) | const |  |
| [`CANONICAL_COMBINING_CLASS_KV`](#canonical-combining-class-kv) | const |  |
| [`COMPOSITION_TABLE_SALT`](#composition-table-salt) | const |  |
| [`COMPOSITION_TABLE_KV`](#composition-table-kv) | const |  |
| [`CANONICAL_DECOMPOSED_CHARS`](#canonical-decomposed-chars) | const |  |
| [`CANONICAL_DECOMPOSED_SALT`](#canonical-decomposed-salt) | const |  |
| [`CANONICAL_DECOMPOSED_KV`](#canonical-decomposed-kv) | const |  |
| [`COMPATIBILITY_DECOMPOSED_CHARS`](#compatibility-decomposed-chars) | const |  |
| [`COMPATIBILITY_DECOMPOSED_SALT`](#compatibility-decomposed-salt) | const |  |
| [`COMPATIBILITY_DECOMPOSED_KV`](#compatibility-decomposed-kv) | const |  |
| [`CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS`](#cjk-compat-variants-decomposed-chars) | const |  |
| [`CJK_COMPAT_VARIANTS_DECOMPOSED_SALT`](#cjk-compat-variants-decomposed-salt) | const |  |
| [`CJK_COMPAT_VARIANTS_DECOMPOSED_KV`](#cjk-compat-variants-decomposed-kv) | const |  |
| [`COMBINING_MARK_SALT`](#combining-mark-salt) | const |  |
| [`COMBINING_MARK_KV`](#combining-mark-kv) | const |  |
| [`TRAILING_NONSTARTERS_SALT`](#trailing-nonstarters-salt) | const |  |
| [`TRAILING_NONSTARTERS_KV`](#trailing-nonstarters-kv) | const |  |

## Functions

### `composition_table_astral`

```rust
fn composition_table_astral(c1: char, c2: char) -> Option<char>
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:1225-1262`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L1225-L1262)*

### `is_public_assigned`

```rust
fn is_public_assigned(c: char) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:20435-21172`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L20435-L21172)*

### `qc_nfc`

```rust
fn qc_nfc(c: char) -> crate::quick_check::IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:21176-21304`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L21176-L21304)*

### `qc_nfkc`

```rust
fn qc_nfkc(c: char) -> crate::quick_check::IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:21307-21756`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L21307-L21756)*

### `qc_nfd`

```rust
fn qc_nfd(c: char) -> crate::quick_check::IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:21759-22016`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L21759-L22016)*

### `qc_nfkd`

```rust
fn qc_nfkd(c: char) -> crate::quick_check::IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:22019-22583`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L22019-L22583)*

### `stream_safe_leading_nonstarters`

```rust
fn stream_safe_leading_nonstarters(c: char) -> usize
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:22585-22598`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L22585-L22598)*

## Constants

### `UNICODE_VERSION`
```rust
const UNICODE_VERSION: (u8, u8, u8);
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:18`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L18)*

### `CANONICAL_COMBINING_CLASS_SALT`
```rust
const CANONICAL_COMBINING_CLASS_SALT: &[u16];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:20-96`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L20-L96)*

### `CANONICAL_COMBINING_CLASS_KV`
```rust
const CANONICAL_COMBINING_CLASS_KV: &[u32];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:97-219`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L97-L219)*

### `COMPOSITION_TABLE_SALT`
```rust
const COMPOSITION_TABLE_SALT: &[u16];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:221-294`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L221-L294)*

### `COMPOSITION_TABLE_KV`
```rust
const COMPOSITION_TABLE_KV: &[(u32, char)];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:295-1224`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L295-L1224)*

### `CANONICAL_DECOMPOSED_CHARS`
```rust
const CANONICAL_DECOMPOSED_CHARS: &[char];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:1263-4714`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L1263-L4714)*

### `CANONICAL_DECOMPOSED_SALT`
```rust
const CANONICAL_DECOMPOSED_SALT: &[u16];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:4716-4878`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L4716-L4878)*

### `CANONICAL_DECOMPOSED_KV`
```rust
const CANONICAL_DECOMPOSED_KV: &[(u32, (u16, u16))];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:4879-6961`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L4879-L6961)*

### `COMPATIBILITY_DECOMPOSED_CHARS`
```rust
const COMPATIBILITY_DECOMPOSED_CHARS: &[char];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:6962-12735`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L6962-L12735)*

### `COMPATIBILITY_DECOMPOSED_SALT`
```rust
const COMPATIBILITY_DECOMPOSED_SALT: &[u16];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:12737-13035`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L12737-L13035)*

### `COMPATIBILITY_DECOMPOSED_KV`
```rust
const COMPATIBILITY_DECOMPOSED_KV: &[(u32, (u16, u16))];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:13036-16886`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L13036-L16886)*

### `CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS`
```rust
const CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS: &[char];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:16887-18892`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L16887-L18892)*

### `CJK_COMPAT_VARIANTS_DECOMPOSED_SALT`
```rust
const CJK_COMPAT_VARIANTS_DECOMPOSED_SALT: &[u16];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:18894-18973`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L18894-L18973)*

### `CJK_COMPAT_VARIANTS_DECOMPOSED_KV`
```rust
const CJK_COMPAT_VARIANTS_DECOMPOSED_KV: &[(u32, (u16, u16))];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:18974-19977`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L18974-L19977)*

### `COMBINING_MARK_SALT`
```rust
const COMBINING_MARK_SALT: &[u16];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:19979-20176`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L19979-L20176)*

### `COMBINING_MARK_KV`
```rust
const COMBINING_MARK_KV: &[u32];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:20177-20433`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L20177-L20433)*

### `TRAILING_NONSTARTERS_SALT`
```rust
const TRAILING_NONSTARTERS_SALT: &[u16];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:22600-22685`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L22600-L22685)*

### `TRAILING_NONSTARTERS_KV`
```rust
const TRAILING_NONSTARTERS_KV: &[u32];
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:22686-22824`](../../../.source_1765210505/unicode-normalization-0.1.25/src/tables.rs#L22686-L22824)*

