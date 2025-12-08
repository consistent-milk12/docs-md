*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`PeSegmentIterator`](#pesegmentiterator)
  - [`PeSegment`](#pesegment)
  - [`PeSectionIterator`](#pesectioniterator)
  - [`PeSection`](#pesection)
  - [`PeRelocationIterator`](#perelocationiterator)
- [Type Aliases](#type-aliases)
  - [`PeSegmentIterator32`](#pesegmentiterator32)
  - [`PeSegmentIterator64`](#pesegmentiterator64)
  - [`PeSegment32`](#pesegment32)
  - [`PeSegment64`](#pesegment64)
  - [`PeSectionIterator32`](#pesectioniterator32)
  - [`PeSectionIterator64`](#pesectioniterator64)
  - [`PeSection32`](#pesection32)
  - [`PeSection64`](#pesection64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PeSegmentIterator`](#pesegmentiterator) | struct | An iterator for the loadable sections in a [`PeFile`]. |
| [`PeSegment`](#pesegment) | struct | A loadable section in a [`PeFile`]. |
| [`PeSectionIterator`](#pesectioniterator) | struct | An iterator for the sections in a [`PeFile`]. |
| [`PeSection`](#pesection) | struct | A section in a [`PeFile`]. |
| [`PeRelocationIterator`](#perelocationiterator) | struct | An iterator for the relocations in an [`PeSection`]. |
| [`PeSegmentIterator32`](#pesegmentiterator32) | type | An iterator for the loadable sections in a [`PeFile32`](super::PeFile32). |
| [`PeSegmentIterator64`](#pesegmentiterator64) | type | An iterator for the loadable sections in a [`PeFile64`](super::PeFile64). |
| [`PeSegment32`](#pesegment32) | type | A loadable section in a [`PeFile32`](super::PeFile32). |
| [`PeSegment64`](#pesegment64) | type | A loadable section in a [`PeFile64`](super::PeFile64). |
| [`PeSectionIterator32`](#pesectioniterator32) | type | An iterator for the sections in a [`PeFile32`](super::PeFile32). |
| [`PeSectionIterator64`](#pesectioniterator64) | type | An iterator for the sections in a [`PeFile64`](super::PeFile64). |
| [`PeSection32`](#pesection32) | type | A section in a [`PeFile32`](super::PeFile32). |
| [`PeSection64`](#pesection64) | type | A section in a [`PeFile64`](super::PeFile64). |

## Structs

### `PeSegmentIterator<'data, 'file, Pe, R>`

```rust
struct PeSegmentIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

An iterator for the loadable sections in a [`PeFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesegmentiterator-intoiter"></span>`type IntoIter = I`

- <span id="pesegmentiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-item"></span>`type Item = PeSegment<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `PeSegment<'data, 'file, Pe, R>`

```rust
struct PeSegment<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    section: &'data pe::ImageSectionHeader,
}
```

A loadable section in a [`PeFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- <span id="pesegment-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](../index.md)

- <span id="pesegment-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSegment for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-address"></span>`fn address(&self) -> u64`

- <span id="pesegment-size"></span>`fn size(&self) -> u64`

- <span id="pesegment-align"></span>`fn align(&self) -> u64`

- <span id="pesegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="pesegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="pesegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- <span id="pesegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- <span id="pesegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- <span id="pesegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeSegment<'data, 'file, Pe, R>`

### `PeSectionIterator<'data, 'file, Pe, R>`

```rust
struct PeSectionIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

An iterator for the sections in a [`PeFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesectioniterator-intoiter"></span>`type IntoIter = I`

- <span id="pesectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-item"></span>`type Item = PeSection<'data, 'file, Pe, R>`

- <span id="pesectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `PeSection<'data, 'file, Pe, R>`

```rust
struct PeSection<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    index: crate::read::SectionIndex,
    section: &'data pe::ImageSectionHeader,
}
```

A section in a [`PeFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="pesection-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](../index.md)

- <span id="pesection-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSection for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-relocationiterator"></span>`type RelocationIterator = PeRelocationIterator<'data, 'file, R>`

- <span id="pesection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- <span id="pesection-address"></span>`fn address(&self) -> u64`

- <span id="pesection-size"></span>`fn size(&self) -> u64`

- <span id="pesection-align"></span>`fn align(&self) -> u64`

- <span id="pesection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="pesection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="pesection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- <span id="pesection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- <span id="pesection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- <span id="pesection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="pesection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md)

- <span id="pesection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- <span id="pesection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- <span id="pesection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md)

- <span id="pesection-relocations"></span>`fn relocations(&self) -> PeRelocationIterator<'data, 'file, R>` — [`PeRelocationIterator`](../index.md)

- <span id="pesection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- <span id="pesection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeSection<'data, 'file, Pe, R>`

### `PeRelocationIterator<'data, 'file, R>`

```rust
struct PeRelocationIterator<'data, 'file, R>(core::marker::PhantomData<(&'data (), &'file (), R)>);
```

An iterator for the relocations in an [`PeSection`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug> Debug for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="perelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="perelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R> Iterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-item"></span>`type Item = (u64, Relocation)`

- <span id="perelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `PeSegmentIterator32<'data, 'file, R>`

```rust
type PeSegmentIterator32<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the loadable sections in a [`PeFile32`](super::PeFile32).

### `PeSegmentIterator64<'data, 'file, R>`

```rust
type PeSegmentIterator64<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the loadable sections in a [`PeFile64`](super::PeFile64).

### `PeSegment32<'data, 'file, R>`

```rust
type PeSegment32<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders32, R>;
```

A loadable section in a [`PeFile32`](super::PeFile32).

### `PeSegment64<'data, 'file, R>`

```rust
type PeSegment64<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders64, R>;
```

A loadable section in a [`PeFile64`](super::PeFile64).

### `PeSectionIterator32<'data, 'file, R>`

```rust
type PeSectionIterator32<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the sections in a [`PeFile32`](super::PeFile32).

### `PeSectionIterator64<'data, 'file, R>`

```rust
type PeSectionIterator64<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the sections in a [`PeFile64`](super::PeFile64).

### `PeSection32<'data, 'file, R>`

```rust
type PeSection32<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders32, R>;
```

A section in a [`PeFile32`](super::PeFile32).

### `PeSection64<'data, 'file, R>`

```rust
type PeSection64<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders64, R>;
```

A section in a [`PeFile64`](super::PeFile64).

