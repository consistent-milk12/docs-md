*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`SectionTable`](#sectiontable)
  - [`CoffSegmentIterator`](#coffsegmentiterator)
  - [`CoffSegment`](#coffsegment)
  - [`CoffSectionIterator`](#coffsectioniterator)
  - [`CoffSection`](#coffsection)
- [Type Aliases](#type-aliases)
  - [`CoffBigSegmentIterator`](#coffbigsegmentiterator)
  - [`CoffBigSegment`](#coffbigsegment)
  - [`CoffBigSectionIterator`](#coffbigsectioniterator)
  - [`CoffBigSection`](#coffbigsection)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SectionTable`](#sectiontable) | struct | The table of section headers in a COFF or PE file. |
| [`CoffSegmentIterator`](#coffsegmentiterator) | struct | An iterator for the loadable sections in a [`CoffFile`]. |
| [`CoffSegment`](#coffsegment) | struct | A loadable section in a [`CoffFile`]. |
| [`CoffSectionIterator`](#coffsectioniterator) | struct | An iterator for the sections in a [`CoffFile`]. |
| [`CoffSection`](#coffsection) | struct | A section in a [`CoffFile`]. |
| [`CoffBigSegmentIterator`](#coffbigsegmentiterator) | type | An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSegment`](#coffbigsegment) | type | A loadable section in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSectionIterator`](#coffbigsectioniterator) | type | An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSection`](#coffbigsection) | type | A section in a [`CoffBigFile`](super::CoffBigFile). |

## Structs

### `SectionTable<'data>`

```rust
struct SectionTable<'data> {
    sections: &'data [pe::ImageSectionHeader],
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:19-21`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L19-L21)*

The table of section headers in a COFF or PE file.

Returned by `CoffHeader::sections` and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../../index.md)

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../../pe/index.md)

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../../index.md), [`ImageSectionHeader`](../../../pe/index.md)

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageSectionHeader`](../../../pe/index.md)

- <span id="sectiontable-section-by-name"></span>`fn section_by_name<R: ReadRef<'data>>(&self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../../index.md), [`SectionIndex`](../../../index.md), [`ImageSectionHeader`](../../../pe/index.md)

- <span id="sectiontable-max-section-file-offset"></span>`fn max_section_file_offset(&self) -> u64`

#### Trait Implementations

##### `impl Clone for SectionTable<'data>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data>` — [`SectionTable`](../index.md)

##### `impl Copy for SectionTable<'data>`

##### `impl Debug for SectionTable<'data>`

- <span id="sectiontable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionTable<'data>`

- <span id="sectiontable-default"></span>`fn default() -> SectionTable<'data>` — [`SectionTable`](../index.md)

### `CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:121-129`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L121-L129)*

An iterator for the loadable sections in a [`CoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsegmentiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffsegmentiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-type-item"></span>`type Item = CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    section: &'data pe::ImageSectionHeader,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:154-162`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L154-L162)*

A loadable section in a [`CoffFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- <span id="coffsegment-coff-file"></span>`fn coff_file(&self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](../index.md)

- <span id="coffsegment-coff-section"></span>`fn coff_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

- <span id="coffsegment-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSegment for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-address"></span>`fn address(&self) -> u64`

- <span id="coffsegment-size"></span>`fn size(&self) -> u64`

- <span id="coffsegment-align"></span>`fn align(&self) -> u64`

- <span id="coffsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="coffsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="coffsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- <span id="coffsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- <span id="coffsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- <span id="coffsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSegment<'data, 'file, R, Coff>`

### `CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:253-261`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L253-L261)*

An iterator for the sections in a [`CoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-type-item"></span>`type Item = CoffSection<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    index: crate::read::SectionIndex,
    section: &'data pe::ImageSectionHeader,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:287-296`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L287-L296)*

A section in a [`CoffFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="coffsection-coff-file"></span>`fn coff_file(&self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](../index.md)

- <span id="coffsection-coff-section"></span>`fn coff_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

- <span id="coffsection-coff-relocations"></span>`fn coff_relocations(&self) -> Result<&'data [pe::ImageRelocation]>` — [`Result`](../../../index.md), [`ImageRelocation`](../../../pe/index.md)

- <span id="coffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSection for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-type-relocationiterator"></span>`type RelocationIterator = CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- <span id="coffsection-address"></span>`fn address(&self) -> u64`

- <span id="coffsection-size"></span>`fn size(&self) -> u64`

- <span id="coffsection-align"></span>`fn align(&self) -> u64`

- <span id="coffsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="coffsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="coffsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- <span id="coffsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- <span id="coffsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- <span id="coffsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="coffsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md)

- <span id="coffsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- <span id="coffsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- <span id="coffsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md)

- <span id="coffsection-relocations"></span>`fn relocations(&self) -> CoffRelocationIterator<'data, 'file, R, Coff>` — [`CoffRelocationIterator`](../index.md)

- <span id="coffsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- <span id="coffsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSection<'data, 'file, R, Coff>`

## Type Aliases

### `CoffBigSegmentIterator<'data, 'file, R>`

```rust
type CoffBigSegmentIterator<'data, 'file, R> = CoffSegmentIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:116-117`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L116-L117)*

An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSegment<'data, 'file, R>`

```rust
type CoffBigSegment<'data, 'file, R> = CoffSegment<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:147-148`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L147-L148)*

A loadable section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

### `CoffBigSectionIterator<'data, 'file, R>`

```rust
type CoffBigSectionIterator<'data, 'file, R> = CoffSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:248-249`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L248-L249)*

An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSection<'data, 'file, R>`

```rust
type CoffBigSection<'data, 'file, R> = CoffSection<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:280-281`](../../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L280-L281)*

A section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

