*[proc_macro2](../../index.md) / [probe](../index.md) / [proc_macro_span](index.md)*

---

# Module `proc_macro_span`

## Contents

- [Functions](#functions)
  - [`byte_range`](#byte-range)
  - [`start`](#start)
  - [`end`](#end)
  - [`line`](#line)
  - [`column`](#column)
  - [`file`](#file)
  - [`local_file`](#local-file)
  - [`join`](#join)
  - [`subspan`](#subspan)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`byte_range`](#byte-range) | fn |  |
| [`start`](#start) | fn |  |
| [`end`](#end) | fn |  |
| [`line`](#line) | fn |  |
| [`column`](#column) | fn |  |
| [`file`](#file) | fn |  |
| [`local_file`](#local-file) | fn |  |
| [`join`](#join) | fn |  |
| [`subspan`](#subspan) | fn |  |

## Functions

### `byte_range`

```rust
fn byte_range(this: &proc_macro::Span) -> core::ops::Range<usize>
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:13-15`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L13-L15)*

### `start`

```rust
fn start(this: &proc_macro::Span) -> proc_macro::Span
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:17-19`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L17-L19)*

### `end`

```rust
fn end(this: &proc_macro::Span) -> proc_macro::Span
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:21-23`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L21-L23)*

### `line`

```rust
fn line(this: &proc_macro::Span) -> usize
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:25-27`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L25-L27)*

### `column`

```rust
fn column(this: &proc_macro::Span) -> usize
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:29-31`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L29-L31)*

### `file`

```rust
fn file(this: &proc_macro::Span) -> String
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:33-35`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L33-L35)*

### `local_file`

```rust
fn local_file(this: &proc_macro::Span) -> Option<std::path::PathBuf>
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:37-39`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L37-L39)*

### `join`

```rust
fn join(this: &proc_macro::Span, other: proc_macro::Span) -> Option<proc_macro::Span>
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:41-43`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L41-L43)*

### `subspan`

```rust
fn subspan<R: RangeBounds<usize>>(this: &proc_macro::Literal, range: R) -> Option<proc_macro::Span>
```

*Defined in [`proc-macro2-1.0.103/src/probe/proc_macro_span.rs:45-47`](../../../../.source_1765210505/proc-macro2-1.0.103/src/probe/proc_macro_span.rs#L45-L47)*

