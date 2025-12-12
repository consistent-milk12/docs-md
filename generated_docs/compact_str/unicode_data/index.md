*[compact_str](../index.md) / [unicode_data](index.md)*

---

# Module `unicode_data`

Adapted from
<https://doc.rust-lang.org/nightly/src/core/unicode/unicode_data.rs.html>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`case_ignorable`](#case-ignorable) | mod |  |
| [`cased`](#cased) | mod |  |
| [`skip_search`](#skip-search) | fn |  |
| [`decode_prefix_sum`](#decode-prefix-sum) | fn |  |
| [`decode_length`](#decode-length) | fn |  |

## Modules

- [`case_ignorable`](case_ignorable/index.md)
- [`cased`](cased/index.md)

## Functions

### `skip_search`

```rust
fn skip_search<const SOR: usize, const OFFSETS: usize>(needle: u32, short_offset_runs: &[u32; SOR], offsets: &[u8; OFFSETS]) -> bool
```

*Defined in [`compact_str-0.9.0/src/unicode_data.rs:5-45`](../../../.source_1765210505/compact_str-0.9.0/src/unicode_data.rs#L5-L45)*

### `decode_prefix_sum`

```rust
const fn decode_prefix_sum(short_offset_run_header: u32) -> u32
```

*Defined in [`compact_str-0.9.0/src/unicode_data.rs:48-50`](../../../.source_1765210505/compact_str-0.9.0/src/unicode_data.rs#L48-L50)*

### `decode_length`

```rust
const fn decode_length(short_offset_run_header: u32) -> usize
```

*Defined in [`compact_str-0.9.0/src/unicode_data.rs:53-55`](../../../.source_1765210505/compact_str-0.9.0/src/unicode_data.rs#L53-L55)*

