*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [section](index.md)*

---

# Module `section`

## Structs

### `SectionTable<'data>`

```rust
struct SectionTable<'data> {
    sections: &'data [pe::ImageSectionHeader],
}
```

The table of section headers in a COFF or PE file.

Returned by `CoffHeader::sections` and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- `fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../../pe/index.md)

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../../index.md), [`ImageSectionHeader`](../../../pe/index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageSectionHeader`](../../../pe/index.md)

- `fn section_by_name<R: ReadRef<'data>>(self: &Self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../../index.md), [`SectionIndex`](../../../index.md), [`ImageSectionHeader`](../../../pe/index.md)

- `fn max_section_file_offset(self: &Self) -> u64`

#### Trait Implementations

##### `impl<'data> Clone for SectionTable<'data>`

- `fn clone(self: &Self) -> SectionTable<'data>` — [`SectionTable`](../../pe/index.md)

##### `impl<'data> Copy for SectionTable<'data>`

##### `impl<'data> Debug for SectionTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for SectionTable<'data>`

- `fn default() -> SectionTable<'data>` — [`SectionTable`](../../pe/index.md)

### `CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

An iterator for the loadable sections in a [`CoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffSegmentIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- `type Item = CoffSegment<'data, 'file, R, Coff>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    section: &'data pe::ImageSectionHeader,
}
```

A loadable section in a [`CoffFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- `fn coff_file(self: &Self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](../index.md)

- `fn coff_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffSegment<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSegment for CoffSegment<'data, 'file, R, Coff>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSegment<'data, 'file, R, Coff>`

### `CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

An iterator for the sections in a [`CoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffSectionIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for CoffSectionIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSectionIterator<'data, 'file, R, Coff>`

- `type Item = CoffSection<'data, 'file, R, Coff>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    index: crate::read::SectionIndex,
    section: &'data pe::ImageSectionHeader,
}
```

A section in a [`CoffFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- `fn coff_file(self: &Self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](../index.md)

- `fn coff_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

- `fn coff_relocations(self: &Self) -> Result<&'data [pe::ImageRelocation]>` — [`Result`](../../../index.md), [`ImageRelocation`](../../../pe/index.md)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffSection<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSection for CoffSection<'data, 'file, R, Coff>`

- `type RelocationIterator = CoffRelocationIterator<'data, 'file, R, Coff>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../../index.md)

- `fn relocations(self: &Self) -> CoffRelocationIterator<'data, 'file, R, Coff>` — [`CoffRelocationIterator`](../index.md)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSection<'data, 'file, R, Coff>`

## Type Aliases

### `CoffBigSegmentIterator<'data, 'file, R>`

```rust
type CoffBigSegmentIterator<'data, 'file, R> = CoffSegmentIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSegment<'data, 'file, R>`

```rust
type CoffBigSegment<'data, 'file, R> = CoffSegment<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A loadable section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

### `CoffBigSectionIterator<'data, 'file, R>`

```rust
type CoffBigSectionIterator<'data, 'file, R> = CoffSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSection<'data, 'file, R>`

```rust
type CoffBigSection<'data, 'file, R> = CoffSection<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

