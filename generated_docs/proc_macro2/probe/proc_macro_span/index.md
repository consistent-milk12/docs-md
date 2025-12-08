*[proc_macro2](../../index.md) / [probe](../index.md) / [proc_macro_span](index.md)*

---

# Module `proc_macro_span`

## Contents

- [Functions](#functions)
  - [`byte_range`](#byte_range)
  - [`start`](#start)
  - [`end`](#end)
  - [`line`](#line)
  - [`column`](#column)
  - [`file`](#file)
  - [`local_file`](#local_file)
  - [`join`](#join)
  - [`subspan`](#subspan)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`byte_range`](#byte_range) | fn |  |
| [`start`](#start) | fn |  |
| [`end`](#end) | fn |  |
| [`line`](#line) | fn |  |
| [`column`](#column) | fn |  |
| [`file`](#file) | fn |  |
| [`local_file`](#local_file) | fn |  |
| [`join`](#join) | fn |  |
| [`subspan`](#subspan) | fn |  |

## Functions

### `byte_range`

```rust
fn byte_range(this: &proc_macro::Span) -> core::ops::Range<usize>
```

### `start`

```rust
fn start(this: &proc_macro::Span) -> proc_macro::Span
```

### `end`

```rust
fn end(this: &proc_macro::Span) -> proc_macro::Span
```

### `line`

```rust
fn line(this: &proc_macro::Span) -> usize
```

### `column`

```rust
fn column(this: &proc_macro::Span) -> usize
```

### `file`

```rust
fn file(this: &proc_macro::Span) -> String
```

### `local_file`

```rust
fn local_file(this: &proc_macro::Span) -> Option<std::path::PathBuf>
```

### `join`

```rust
fn join(this: &proc_macro::Span, other: proc_macro::Span) -> Option<proc_macro::Span>
```

### `subspan`

```rust
fn subspan<R: RangeBounds<usize>>(this: &proc_macro::Literal, range: R) -> Option<proc_macro::Span>
```

