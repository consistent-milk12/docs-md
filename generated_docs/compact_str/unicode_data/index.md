*[compact_str](../index.md) / [unicode_data](index.md)*

---

# Module `unicode_data`

Adapted from
<https://doc.rust-lang.org/nightly/src/core/unicode/unicode_data.rs.html>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`case_ignorable`](#case_ignorable) | mod |  |
| [`cased`](#cased) | mod |  |
| [`skip_search`](#skip_search) | fn |  |
| [`decode_prefix_sum`](#decode_prefix_sum) | fn |  |
| [`decode_length`](#decode_length) | fn |  |

## Modules

- [`case_ignorable`](case_ignorable/index.md)
- [`cased`](cased/index.md)

## Functions

### `skip_search`

```rust
fn skip_search<const SOR: usize, const OFFSETS: usize>(needle: u32, short_offset_runs: &[u32; SOR], offsets: &[u8; OFFSETS]) -> bool
```

### `decode_prefix_sum`

```rust
const fn decode_prefix_sum(short_offset_run_header: u32) -> u32
```

### `decode_length`

```rust
const fn decode_length(short_offset_run_header: u32) -> usize
```

