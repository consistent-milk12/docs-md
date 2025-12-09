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

*Defined in [`addr2line-0.25.1/src/line.rs:10`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L10)*

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

*Defined in [`addr2line-0.25.1/src/line.rs:29-33`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L29-L33)*

### `LineRow`

```rust
struct LineRow {
    address: u64,
    file_index: u64,
    line: u32,
    column: u32,
}
```

*Defined in [`addr2line-0.25.1/src/line.rs:35-40`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L35-L40)*

### `Lines`

```rust
struct Lines {
    files: alloc::boxed::Box<[alloc::string::String]>,
    sequences: alloc::boxed::Box<[LineSequence]>,
}
```

*Defined in [`addr2line-0.25.1/src/line.rs:42-45`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L42-L45)*

#### Implementations

- <span id="lines-parse"></span>`fn parse<R: gimli::Reader>(dw_unit: gimli::UnitRef<'_, R>, ilnp: gimli::IncompleteLineProgram<R, <R as >::Offset>) -> Result<Self, gimli::Error>`

- <span id="lines-file"></span>`fn file(&self, index: u64) -> Option<&str>`

- <span id="lines-ranges"></span>`fn ranges(&self) -> impl Iterator<Item = gimli::Range> + '_`

- <span id="lines-row-location"></span>`fn row_location(&self, row: &LineRow) -> Location<'_>` — [`LineRow`](#linerow), [`Location`](../frame/index.md)

- <span id="lines-find-location"></span>`fn find_location(&self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>` — [`Location`](../frame/index.md)

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

*Defined in [`addr2line-0.25.1/src/line.rs:209-214`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L209-L214)*

#### Trait Implementations

##### `impl IntoIterator for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="linelocationrangeiter-type-intoiter"></span>`type IntoIter = I`

- <span id="linelocationrangeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-type-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="linelocationrangeiter-next"></span>`fn next(&mut self) -> Option<(u64, u64, Location<'ctx>)>` — [`Location`](../frame/index.md)

## Functions

### `render_file`

```rust
fn render_file<R: gimli::Reader>(dw_unit: gimli::UnitRef<'_, R>, file: &gimli::FileEntry<R, <R as >::Offset>, header: &gimli::LineProgramHeader<R, <R as >::Offset>) -> Result<alloc::string::String, gimli::Error>
```

*Defined in [`addr2line-0.25.1/src/line.rs:256-286`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L256-L286)*

### `path_push`

```rust
fn path_push(path: &mut alloc::string::String, p: &str)
```

*Defined in [`addr2line-0.25.1/src/line.rs:288-303`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L288-L303)*

### `has_forward_slash_root`

```rust
fn has_forward_slash_root(p: &str) -> bool
```

*Defined in [`addr2line-0.25.1/src/line.rs:306-308`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L306-L308)*

Check if the path in the given string has a unix style root

### `has_backward_slash_root`

```rust
fn has_backward_slash_root(p: &str) -> bool
```

*Defined in [`addr2line-0.25.1/src/line.rs:311-313`](../../../.source_1765210505/addr2line-0.25.1/src/line.rs#L311-L313)*

Check if the path in the given string has a windows style root

