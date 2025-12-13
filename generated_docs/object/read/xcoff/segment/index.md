*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [segment](index.md)*

---

# Module `segment`

TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XcoffSegmentIterator`](#xcoffsegmentiterator) | struct | An iterator for the segments in an [`XcoffFile`]. |
| [`XcoffSegment`](#xcoffsegment) | struct | A loadable section in an [`XcoffFile`]. |
| [`XcoffSegmentIterator32`](#xcoffsegmentiterator32) | type | An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegmentIterator64`](#xcoffsegmentiterator64) | type | An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegment32`](#xcoffsegment32) | type | A segment in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegment64`](#xcoffsegment64) | type | A segment in an [`XcoffFile64`](super::XcoffFile64). |

## Structs

### `XcoffSegmentIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegmentIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:22-29`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/segment.rs#L22-L29)*

An iterator for the segments in an [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-type-item"></span>`type Item = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsegmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsegmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSegment<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegment<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:54-61`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/segment.rs#L54-L61)*

A loadable section in an [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff, R> ObjectSegment for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="xcoffsegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-data-range"></span>`fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md#segmentflags)

##### `impl<Xcoff, R> Sealed for XcoffSegment<'data, 'file, Xcoff, R>`

##### `impl<U> TryFrom for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `XcoffSegmentIterator32<'data, 'file, R>`

```rust
type XcoffSegmentIterator32<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:12-13`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/segment.rs#L12-L13)*

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegmentIterator64<'data, 'file, R>`

```rust
type XcoffSegmentIterator64<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:15-16`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/segment.rs#L15-L16)*

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegment32<'data, 'file, R>`

```rust
type XcoffSegment32<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:44-45`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/segment.rs#L44-L45)*

A segment in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegment64<'data, 'file, R>`

```rust
type XcoffSegment64<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:47-48`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/segment.rs#L47-L48)*

A segment in an [`XcoffFile64`](super::XcoffFile64).

