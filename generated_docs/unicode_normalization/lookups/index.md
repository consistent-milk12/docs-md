*[unicode_normalization](../index.md) / [lookups](index.md)*

---

# Module `lookups`

Lookups of unicode properties using minimal perfect hashing.

## Contents

- [Functions](#functions)
  - [`canonical_combining_class`](#canonical-combining-class)
  - [`composition_table`](#composition-table)
  - [`canonical_fully_decomposed`](#canonical-fully-decomposed)
  - [`compatibility_fully_decomposed`](#compatibility-fully-decomposed)
  - [`cjk_compat_variants_fully_decomposed`](#cjk-compat-variants-fully-decomposed)
  - [`is_combining_mark`](#is-combining-mark)
  - [`stream_safe_trailing_nonstarters`](#stream-safe-trailing-nonstarters)
  - [`u8_lookup_fk`](#u8-lookup-fk)
  - [`u8_lookup_fv`](#u8-lookup-fv)
  - [`bool_lookup_fk`](#bool-lookup-fk)
  - [`bool_lookup_fv`](#bool-lookup-fv)
  - [`pair_lookup_fk`](#pair-lookup-fk)
  - [`pair_lookup_fv_opt`](#pair-lookup-fv-opt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`canonical_combining_class`](#canonical-combining-class) | fn | Look up the canonical combining class for a codepoint. |
| [`composition_table`](#composition-table) | fn |  |
| [`canonical_fully_decomposed`](#canonical-fully-decomposed) | fn |  |
| [`compatibility_fully_decomposed`](#compatibility-fully-decomposed) | fn |  |
| [`cjk_compat_variants_fully_decomposed`](#cjk-compat-variants-fully-decomposed) | fn |  |
| [`is_combining_mark`](#is-combining-mark) | fn | Return whether the given character is a combining mark (`General_Category=Mark`) |
| [`stream_safe_trailing_nonstarters`](#stream-safe-trailing-nonstarters) | fn |  |
| [`u8_lookup_fk`](#u8-lookup-fk) | fn | Extract the key in a 24 bit key and 8 bit value packed in a u32. |
| [`u8_lookup_fv`](#u8-lookup-fv) | fn | Extract the value in a 24 bit key and 8 bit value packed in a u32. |
| [`bool_lookup_fk`](#bool-lookup-fk) | fn | Extract the key for a boolean lookup. |
| [`bool_lookup_fv`](#bool-lookup-fv) | fn | Extract the value for a boolean lookup. |
| [`pair_lookup_fk`](#pair-lookup-fk) | fn | Extract the key in a pair. |
| [`pair_lookup_fv_opt`](#pair-lookup-fv-opt) | fn | Extract the value in a pair, returning an option. |

## Functions

### `canonical_combining_class`

```rust
fn canonical_combining_class(c: char) -> u8
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:19-28`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L19-L28)*

Look up the canonical combining class for a codepoint.

The value returned is as defined in the Unicode Character Database.

### `composition_table`

```rust
fn composition_table(c1: char, c2: char) -> Option<char>
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:30-43`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L30-L43)*

### `canonical_fully_decomposed`

```rust
fn canonical_fully_decomposed(c: char) -> Option<&'static [char]>
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:45-55`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L45-L55)*

### `compatibility_fully_decomposed`

```rust
fn compatibility_fully_decomposed(c: char) -> Option<&'static [char]>
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:57-67`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L57-L67)*

### `cjk_compat_variants_fully_decomposed`

```rust
fn cjk_compat_variants_fully_decomposed(c: char) -> Option<&'static [char]>
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:69-79`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L69-L79)*

### `is_combining_mark`

```rust
fn is_combining_mark(c: char) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:82-91`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L82-L91)*

Return whether the given character is a combining mark (`General_Category=Mark`)

### `stream_safe_trailing_nonstarters`

```rust
fn stream_safe_trailing_nonstarters(c: char) -> usize
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:93-102`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L93-L102)*

### `u8_lookup_fk`

```rust
fn u8_lookup_fk(kv: u32) -> u32
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:106-108`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L106-L108)*

Extract the key in a 24 bit key and 8 bit value packed in a u32.

### `u8_lookup_fv`

```rust
fn u8_lookup_fv(kv: u32) -> u8
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:112-114`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L112-L114)*

Extract the value in a 24 bit key and 8 bit value packed in a u32.

### `bool_lookup_fk`

```rust
fn bool_lookup_fk(kv: u32) -> u32
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:118-120`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L118-L120)*

Extract the key for a boolean lookup.

### `bool_lookup_fv`

```rust
fn bool_lookup_fv(_kv: u32) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:124-126`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L124-L126)*

Extract the value for a boolean lookup.

### `pair_lookup_fk`

```rust
fn pair_lookup_fk<T>(kv: (u32, T)) -> u32
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:130-132`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L130-L132)*

Extract the key in a pair.

### `pair_lookup_fv_opt`

```rust
fn pair_lookup_fv_opt<T>(kv: (u32, T)) -> Option<T>
```

*Defined in [`unicode-normalization-0.1.25/src/lookups.rs:136-138`](../../../.source_1765521767/unicode-normalization-0.1.25/src/lookups.rs#L136-L138)*

Extract the value in a pair, returning an option.

