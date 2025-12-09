*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`MachOSectionIterator`](#machosectioniterator)
  - [`MachOSection`](#machosection)
  - [`MachOSectionInternal`](#machosectioninternal)
- [Traits](#traits)
  - [`Section`](#section)
- [Type Aliases](#type-aliases)
  - [`MachOSectionIterator32`](#machosectioniterator32)
  - [`MachOSectionIterator64`](#machosectioniterator64)
  - [`MachOSection32`](#machosection32)
  - [`MachOSection64`](#machosection64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MachOSectionIterator`](#machosectioniterator) | struct | An iterator for the sections in a [`MachOFile`]. |
| [`MachOSection`](#machosection) | struct | A section in a [`MachOFile`]. |
| [`MachOSectionInternal`](#machosectioninternal) | struct |  |
| [`Section`](#section) | trait | A trait for generic access to [`macho::Section32`] and [`macho::Section64`]. |
| [`MachOSectionIterator32`](#machosectioniterator32) | type | An iterator for the sections in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSectionIterator64`](#machosectioniterator64) | type | An iterator for the sections in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSection32`](#machosection32) | type | A section in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSection64`](#machosection64) | type | A section in a [`MachOFile64`](super::MachOFile64). |

## Structs

### `MachOSectionIterator<'data, 'file, Mach, R>`

```rust
struct MachOSectionIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    iter: slice::Iter<'file, MachOSectionInternal<'data, Mach, R>>,
}
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:22-29`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L22-L29)*

An iterator for the sections in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-type-item"></span>`type Item = MachOSection<'data, 'file, Mach, R>`

- <span id="machosectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `MachOSection<'data, 'file, Mach, R>`

```rust
struct MachOSection<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    internal: MachOSectionInternal<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:68-75`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L68-L75)*

A section in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="machosection-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md)

- <span id="machosection-macho-section"></span>`fn macho_section(&self) -> &'data <Mach as >::Section` — [`MachHeader`](../index.md)

- <span id="machosection-macho-relocations"></span>`fn macho_relocations(&self) -> Result<&'data [macho::Relocation<<Mach as >::Endian>]>` — [`Result`](../../../index.md), [`Relocation`](../../../macho/index.md), [`MachHeader`](../index.md)

- <span id="machosection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="machosection-maybe-compressed-gnu"></span>`fn maybe_compressed_gnu(&self) -> Result<Option<CompressedFileRange>>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSection for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-type-relocationiterator"></span>`type RelocationIterator = MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machosection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- <span id="machosection-address"></span>`fn address(&self) -> u64`

- <span id="machosection-size"></span>`fn size(&self) -> u64`

- <span id="machosection-align"></span>`fn align(&self) -> u64`

- <span id="machosection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="machosection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="machosection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- <span id="machosection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- <span id="machosection-compressed-data"></span>`fn compressed_data(&self) -> read::Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- <span id="machosection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="machosection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md)

- <span id="machosection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- <span id="machosection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- <span id="machosection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md)

- <span id="machosection-relocations"></span>`fn relocations(&self) -> MachORelocationIterator<'data, 'file, Mach, R>` — [`MachORelocationIterator`](../index.md)

- <span id="machosection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- <span id="machosection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSection<'data, 'file, Mach, R>`

### `MachOSectionInternal<'data, Mach: MachHeader, R: ReadRef<'data>>`

```rust
struct MachOSectionInternal<'data, Mach: MachHeader, R: ReadRef<'data>> {
    pub index: crate::read::SectionIndex,
    pub kind: crate::read::SectionKind,
    pub section: &'data <Mach as >::Section,
    pub data: R,
}
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:241-250`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L241-L250)*

#### Fields

- **`data`**: `R`

  The data for the file that contains the section data.
  
  This is required for dyld caches, where this may be a different subcache
  from the file containing the Mach-O load commands.

#### Implementations

- <span id="machosectioninternal-parse"></span>`fn parse(index: SectionIndex, section: &'data <Mach as >::Section, data: R) -> Self` — [`SectionIndex`](../../../index.md), [`MachHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Mach: clone::Clone + MachHeader, R: clone::Clone + ReadRef<'data>> Clone for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-clone"></span>`fn clone(&self) -> MachOSectionInternal<'data, Mach, R>` — [`MachOSectionInternal`](#machosectioninternal)

##### `impl<'data, Mach: marker::Copy + MachHeader, R: marker::Copy + ReadRef<'data>> Copy for MachOSectionInternal<'data, Mach, R>`

##### `impl<'data, Mach: fmt::Debug + MachHeader, R: fmt::Debug + ReadRef<'data>> Debug for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Section`

```rust
trait Section: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:285-354`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L285-L354)*

A trait for generic access to [`macho::Section32`](../../../macho/index.md) and [`macho::Section64`](../../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn sectname(&self) -> &[u8; 16]`

- `fn segname(&self) -> &[u8; 16]`

- `fn addr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn offset(&self, endian: <Self as >::Endian) -> u32`

- `fn align(&self, endian: <Self as >::Endian) -> u32`

- `fn reloff(&self, endian: <Self as >::Endian) -> u32`

- `fn nreloc(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the `sectname` bytes up until the null terminator.

- `fn segment_name(&self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(&self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

- `fn relocations<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> Result<&'data [macho::Relocation<<Self as >::Endian>]>`

  Return the relocation array.

#### Implementors

- [`Section32`](../../../macho/index.md)
- [`Section64`](../../../macho/index.md)

## Type Aliases

### `MachOSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator32<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:15-16`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L15-L16)*

An iterator for the sections in a [`MachOFile32`](super::MachOFile32).

### `MachOSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator64<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:18-19`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L18-L19)*

An iterator for the sections in a [`MachOFile64`](super::MachOFile64).

### `MachOSection32<'data, 'file, Endian, R>`

```rust
type MachOSection32<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:58-59`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L58-L59)*

A section in a [`MachOFile32`](super::MachOFile32).

### `MachOSection64<'data, 'file, Endian, R>`

```rust
type MachOSection64<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:61-62`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L61-L62)*

A section in a [`MachOFile64`](super::MachOFile64).

