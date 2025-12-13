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
  - [`render_file`](#render-file)
  - [`path_push`](#path-push)
  - [`has_forward_slash_root`](#has-forward-slash-root)
  - [`has_backward_slash_root`](#has-backward-slash-root)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LazyLines`](#lazylines) | struct |  |
| [`LineSequence`](#linesequence) | struct |  |
| [`LineRow`](#linerow) | struct |  |
| [`Lines`](#lines) | struct |  |
| [`LineLocationRangeIter`](#linelocationrangeiter) | struct |  |
| [`render_file`](#render-file) | fn |  |
| [`path_push`](#path-push) | fn |  |
| [`has_forward_slash_root`](#has-forward-slash-root) | fn | Check if the path in the given string has a unix style root |
| [`has_backward_slash_root`](#has-backward-slash-root) | fn | Check if the path in the given string has a windows style root |

## Structs

### `LazyLines`

```rust
struct LazyLines(core::cell::OnceCell<Result<Lines, gimli::Error>>);
```

*Defined in [`addr2line-0.25.1/src/line.rs:10`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L10)*

#### Implementations

- <span id="lazylines-new"></span>`fn new() -> Self`

- <span id="lazylines-borrow"></span>`fn borrow<R: gimli::Reader>(&self, dw_unit: gimli::UnitRef<'_, R>, ilnp: &gimli::IncompleteLineProgram<R, <R as >::Offset>) -> Result<&Lines, gimli::Error>` — [`Lines`](#lines)

#### Trait Implementations

##### `impl Any for LazyLines`

- <span id="lazylines-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LazyLines`

- <span id="lazylines-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LazyLines`

- <span id="lazylines-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LazyLines`

- <span id="lazylines-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LazyLines`

- <span id="lazylines-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LazyLines`

- <span id="lazylines-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazylines-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LazyLines`

- <span id="lazylines-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazylines-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineSequence`

```rust
struct LineSequence {
    start: u64,
    end: u64,
    rows: alloc::boxed::Box<[LineRow]>,
}
```

*Defined in [`addr2line-0.25.1/src/line.rs:29-33`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L29-L33)*

#### Trait Implementations

##### `impl Any for LineSequence`

- <span id="linesequence-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineSequence`

- <span id="linesequence-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineSequence`

- <span id="linesequence-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LineSequence`

- <span id="linesequence-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineSequence`

- <span id="linesequence-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LineSequence`

- <span id="linesequence-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linesequence-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineSequence`

- <span id="linesequence-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linesequence-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineRow`

```rust
struct LineRow {
    address: u64,
    file_index: u64,
    line: u32,
    column: u32,
}
```

*Defined in [`addr2line-0.25.1/src/line.rs:35-40`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L35-L40)*

#### Trait Implementations

##### `impl Any for LineRow`

- <span id="linerow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineRow`

- <span id="linerow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineRow`

- <span id="linerow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LineRow`

- <span id="linerow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineRow`

- <span id="linerow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LineRow`

- <span id="linerow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linerow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineRow`

- <span id="linerow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linerow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lines`

```rust
struct Lines {
    files: alloc::boxed::Box<[alloc::string::String]>,
    sequences: alloc::boxed::Box<[LineSequence]>,
}
```

*Defined in [`addr2line-0.25.1/src/line.rs:42-45`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L42-L45)*

#### Implementations

- <span id="lines-parse"></span>`fn parse<R: gimli::Reader>(dw_unit: gimli::UnitRef<'_, R>, ilnp: gimli::IncompleteLineProgram<R, <R as >::Offset>) -> Result<Self, gimli::Error>`

- <span id="lines-file"></span>`fn file(&self, index: u64) -> Option<&str>`

- <span id="lines-ranges"></span>`fn ranges(&self) -> impl Iterator<Item = gimli::Range> + '_`

- <span id="lines-row-location"></span>`fn row_location(&self, row: &LineRow) -> Location<'_>` — [`LineRow`](#linerow), [`Location`](../frame/index.md#location)

- <span id="lines-find-location"></span>`fn find_location(&self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>` — [`Location`](../frame/index.md#location)

- <span id="lines-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64) -> Result<LineLocationRangeIter<'_>, gimli::Error>` — [`LineLocationRangeIter`](#linelocationrangeiter)

#### Trait Implementations

##### `impl Any for Lines`

- <span id="lines-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lines`

- <span id="lines-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lines`

- <span id="lines-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Lines`

- <span id="lines-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lines`

- <span id="lines-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Lines`

- <span id="lines-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lines-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lines`

- <span id="lines-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lines-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineLocationRangeIter<'ctx>`

```rust
struct LineLocationRangeIter<'ctx> {
    lines: &'ctx Lines,
    seq_idx: usize,
    row_idx: usize,
    probe_high: u64,
}
```

*Defined in [`addr2line-0.25.1/src/line.rs:209-214`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L209-L214)*

#### Trait Implementations

##### `impl Any for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="linelocationrangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="linelocationrangeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-iterator-type-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="linelocationrangeiter-iterator-next"></span>`fn next(&mut self) -> Option<(u64, u64, Location<'ctx>)>` — [`Location`](../frame/index.md#location)

##### `impl<U> TryFrom for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linelocationrangeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineLocationRangeIter<'ctx>`

- <span id="linelocationrangeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linelocationrangeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `render_file`

```rust
fn render_file<R: gimli::Reader>(dw_unit: gimli::UnitRef<'_, R>, file: &gimli::FileEntry<R, <R as >::Offset>, header: &gimli::LineProgramHeader<R, <R as >::Offset>) -> Result<alloc::string::String, gimli::Error>
```

*Defined in [`addr2line-0.25.1/src/line.rs:256-286`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L256-L286)*

### `path_push`

```rust
fn path_push(path: &mut alloc::string::String, p: &str)
```

*Defined in [`addr2line-0.25.1/src/line.rs:288-303`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L288-L303)*

### `has_forward_slash_root`

```rust
fn has_forward_slash_root(p: &str) -> bool
```

*Defined in [`addr2line-0.25.1/src/line.rs:306-308`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L306-L308)*

Check if the path in the given string has a unix style root

### `has_backward_slash_root`

```rust
fn has_backward_slash_root(p: &str) -> bool
```

*Defined in [`addr2line-0.25.1/src/line.rs:311-313`](../../../.source_1765633015/addr2line-0.25.1/src/line.rs#L311-L313)*

Check if the path in the given string has a windows style root

