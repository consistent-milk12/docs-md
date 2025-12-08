*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [segment](index.md)*

---

# Module `segment`

TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready.

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

An iterator for the segments in an [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffSegment<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `XcoffSegment<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegment<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

A loadable section in an [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSegment<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectSegment for XcoffSegment<'data, 'file, Xcoff, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffSegment<'data, 'file, Xcoff, R>`

## Type Aliases

### `XcoffSegmentIterator32<'data, 'file, R>`

```rust
type XcoffSegmentIterator32<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegmentIterator64<'data, 'file, R>`

```rust
type XcoffSegmentIterator64<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegment32<'data, 'file, R>`

```rust
type XcoffSegment32<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader32, R>;
```

A segment in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegment64<'data, 'file, R>`

```rust
type XcoffSegment64<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader64, R>;
```

A segment in an [`XcoffFile64`](super::XcoffFile64).

