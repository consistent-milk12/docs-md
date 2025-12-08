*[proc_macro2](../../index.md) / [probe](../index.md) / [proc_macro_span](index.md)*

---

# Module `proc_macro_span`

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

