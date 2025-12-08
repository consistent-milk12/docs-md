*[addr2line](../index.md) / [line](index.md)*

---

# Module `line`

## Structs

### `LazyLines`

```rust
struct LazyLines(core::cell::OnceCell<Result<Lines, gimli::Error>>);
```

#### Implementations

- `fn new() -> Self`

- `fn borrow<R: gimli::Reader>(self: &Self, dw_unit: gimli::UnitRef<'_, R>, ilnp: &gimli::IncompleteLineProgram<R, <R as >::Offset>) -> Result<&Lines, gimli::Error>` — [`Lines`](#lines)

### `LineSequence`

```rust
struct LineSequence {
    start: u64,
    end: u64,
    rows: alloc::boxed::Box<[LineRow]>,
}
```

### `LineRow`

```rust
struct LineRow {
    address: u64,
    file_index: u64,
    line: u32,
    column: u32,
}
```

### `Lines`

```rust
struct Lines {
    files: alloc::boxed::Box<[alloc::string::String]>,
    sequences: alloc::boxed::Box<[LineSequence]>,
}
```

#### Implementations

- `fn parse<R: gimli::Reader>(dw_unit: gimli::UnitRef<'_, R>, ilnp: gimli::IncompleteLineProgram<R, <R as >::Offset>) -> Result<Self, gimli::Error>`

- `fn file(self: &Self, index: u64) -> Option<&str>`

- `fn ranges(self: &Self) -> impl Iterator<Item = gimli::Range> + '_`

- `fn row_location(self: &Self, row: &LineRow) -> Location<'_>` — [`LineRow`](#linerow), [`Location`](../index.md)

- `fn find_location(self: &Self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>` — [`Location`](../index.md)

- `fn find_location_range(self: &Self, probe_low: u64, probe_high: u64) -> Result<LineLocationRangeIter<'_>, gimli::Error>` — [`LineLocationRangeIter`](#linelocationrangeiter)

### `LineLocationRangeIter<'ctx>`

```rust
struct LineLocationRangeIter<'ctx> {
    lines: &'ctx Lines,
    seq_idx: usize,
    row_idx: usize,
    probe_high: u64,
}
```

#### Trait Implementations

##### `impl<I> IntoIterator for LineLocationRangeIter<'ctx>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'ctx> Iterator for LineLocationRangeIter<'ctx>`

- `type Item = (u64, u64, Location<'ctx>)`

- `fn next(self: &mut Self) -> Option<(u64, u64, Location<'ctx>)>` — [`Location`](../index.md)

## Functions

### `render_file`

```rust
fn render_file<R: gimli::Reader>(dw_unit: gimli::UnitRef<'_, R>, file: &gimli::FileEntry<R, <R as >::Offset>, header: &gimli::LineProgramHeader<R, <R as >::Offset>) -> Result<alloc::string::String, gimli::Error>
```

### `path_push`

```rust
fn path_push(path: &mut alloc::string::String, p: &str)
```

### `has_forward_slash_root`

```rust
fn has_forward_slash_root(p: &str) -> bool
```

Check if the path in the given string has a unix style root

### `has_backward_slash_root`

```rust
fn has_backward_slash_root(p: &str) -> bool
```

Check if the path in the given string has a windows style root

