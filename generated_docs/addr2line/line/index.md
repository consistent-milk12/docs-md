*[addr2line](../index.md) / [line](index.md)*

---

# Module `line`

## Contents

- [Structs](#structs)
  - [`LazyLines`](#lazylines)
  - [`LineSequence`](#linesequence)
  - [`LineRow`](#linerow)
  - [`Lines`](#lines)
  - [`LineLocationRangeIter`](#linelocationrangeiter)
- [Functions](#functions)
  - [`render_file`](#render_file)
  - [`path_push`](#path_push)
  - [`has_forward_slash_root`](#has_forward_slash_root)
  - [`has_backward_slash_root`](#has_backward_slash_root)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LazyLines`](#lazylines) | struct |  |
| [`LineSequence`](#linesequence) | struct |  |
| [`LineRow`](#linerow) | struct |  |
| [`Lines`](#lines) | struct |  |
| [`LineLocationRangeIter`](#linelocationrangeiter) | struct |  |
| [`render_file`](#render_file) | fn |  |
| [`path_push`](#path_push) | fn |  |
| [`has_forward_slash_root`](#has_forward_slash_root) | fn | Check if the path in the given string has a unix style root |
| [`has_backward_slash_root`](#has_backward_slash_root) | fn | Check if the path in the given string has a windows style root |

## Structs

### `LazyLines`

```rust
struct LazyLines(core::cell::OnceCell<Result<Lines, gimli::Error>>);
```

#### Implementations

- <span id="lazylines-new"></span>`fn new() -> Self`

- <span id="lazylines-borrow"></span>`fn borrow<R: gimli::Reader>(&self, dw_unit: gimli::UnitRef<'_, R>, ilnp: &gimli::IncompleteLineProgram<R, <R as >::Offset>) -> Result<&Lines, gimli::Error>` — [`Lines`](#lines)

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

- <span id="lines-parse"></span>`fn parse<R: gimli::Reader>(dw_unit: gimli::UnitRef<'_, R>, ilnp: gimli::IncompleteLineProgram<R, <R as >::Offset>) -> Result<Self, gimli::Error>`

- <span id="lines-file"></span>`fn file(&self, index: u64) -> Option<&str>`

- <span id="lines-ranges"></span>`fn ranges(&self) -> impl Iterator<Item = gimli::Range> + '_`

- <span id="lines-row-location"></span>`fn row_location(&self, row: &LineRow) -> Location<'_>` — [`LineRow`](#linerow), [`Location`](../index.md)

- <span id="lines-find-location"></span>`fn find_location(&self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>` — [`Location`](../index.md)

- <span id="lines-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64) -> Result<LineLocationRangeIter<'_>, gimli::Error>` — [`LineLocationRangeIter`](#linelocationrangeiter)

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

- <span id="linelocationrangeiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="linelocationrangeiter-intoiter"></span>`type IntoIter = I`

- <span id="linelocationrangeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'ctx> Iterator for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="linelocationrangeiter-next"></span>`fn next(&mut self) -> Option<(u64, u64, Location<'ctx>)>` — [`Location`](../index.md)

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

