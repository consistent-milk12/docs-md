*[unicode_normalization](../index.md) / [lookups](index.md)*

---

# Module `lookups`

Lookups of unicode properties using minimal perfect hashing.

## Contents

- [Functions](#functions)
  - [`canonical_combining_class`](#canonical_combining_class)
  - [`composition_table`](#composition_table)
  - [`canonical_fully_decomposed`](#canonical_fully_decomposed)
  - [`compatibility_fully_decomposed`](#compatibility_fully_decomposed)
  - [`cjk_compat_variants_fully_decomposed`](#cjk_compat_variants_fully_decomposed)
  - [`is_combining_mark`](#is_combining_mark)
  - [`stream_safe_trailing_nonstarters`](#stream_safe_trailing_nonstarters)
  - [`u8_lookup_fk`](#u8_lookup_fk)
  - [`u8_lookup_fv`](#u8_lookup_fv)
  - [`bool_lookup_fk`](#bool_lookup_fk)
  - [`bool_lookup_fv`](#bool_lookup_fv)
  - [`pair_lookup_fk`](#pair_lookup_fk)
  - [`pair_lookup_fv_opt`](#pair_lookup_fv_opt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`canonical_combining_class`](#canonical_combining_class) | fn | Look up the canonical combining class for a codepoint. |
| [`composition_table`](#composition_table) | fn |  |
| [`canonical_fully_decomposed`](#canonical_fully_decomposed) | fn |  |
| [`compatibility_fully_decomposed`](#compatibility_fully_decomposed) | fn |  |
| [`cjk_compat_variants_fully_decomposed`](#cjk_compat_variants_fully_decomposed) | fn |  |
| [`is_combining_mark`](#is_combining_mark) | fn | Return whether the given character is a combining mark (`General_Category=Mark`) |
| [`stream_safe_trailing_nonstarters`](#stream_safe_trailing_nonstarters) | fn |  |
| [`u8_lookup_fk`](#u8_lookup_fk) | fn | Extract the key in a 24 bit key and 8 bit value packed in a u32. |
| [`u8_lookup_fv`](#u8_lookup_fv) | fn | Extract the value in a 24 bit key and 8 bit value packed in a u32. |
| [`bool_lookup_fk`](#bool_lookup_fk) | fn | Extract the key for a boolean lookup. |
| [`bool_lookup_fv`](#bool_lookup_fv) | fn | Extract the value for a boolean lookup. |
| [`pair_lookup_fk`](#pair_lookup_fk) | fn | Extract the key in a pair. |
| [`pair_lookup_fv_opt`](#pair_lookup_fv_opt) | fn | Extract the value in a pair, returning an option. |

## Functions

### `canonical_combining_class`

```rust
fn canonical_combining_class(c: char) -> u8
```

Look up the canonical combining class for a codepoint.

The value returned is as defined in the Unicode Character Database.

### `composition_table`

```rust
fn composition_table(c1: char, c2: char) -> Option<char>
```

### `canonical_fully_decomposed`

```rust
fn canonical_fully_decomposed(c: char) -> Option<&'static [char]>
```

### `compatibility_fully_decomposed`

```rust
fn compatibility_fully_decomposed(c: char) -> Option<&'static [char]>
```

### `cjk_compat_variants_fully_decomposed`

```rust
fn cjk_compat_variants_fully_decomposed(c: char) -> Option<&'static [char]>
```

### `is_combining_mark`

```rust
fn is_combining_mark(c: char) -> bool
```

Return whether the given character is a combining mark (`General_Category=Mark`)

### `stream_safe_trailing_nonstarters`

```rust
fn stream_safe_trailing_nonstarters(c: char) -> usize
```

### `u8_lookup_fk`

```rust
fn u8_lookup_fk(kv: u32) -> u32
```

Extract the key in a 24 bit key and 8 bit value packed in a u32.

### `u8_lookup_fv`

```rust
fn u8_lookup_fv(kv: u32) -> u8
```

Extract the value in a 24 bit key and 8 bit value packed in a u32.

### `bool_lookup_fk`

```rust
fn bool_lookup_fk(kv: u32) -> u32
```

Extract the key for a boolean lookup.

### `bool_lookup_fv`

```rust
fn bool_lookup_fv(_kv: u32) -> bool
```

Extract the value for a boolean lookup.

### `pair_lookup_fk`

```rust
fn pair_lookup_fk<T>(kv: (u32, T)) -> u32
```

Extract the key in a pair.

### `pair_lookup_fv_opt`

```rust
fn pair_lookup_fv_opt<T>(kv: (u32, T)) -> Option<T>
```

Extract the value in a pair, returning an option.

