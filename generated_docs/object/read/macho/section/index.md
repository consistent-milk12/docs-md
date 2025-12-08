*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [section](index.md)*

---

# Module `section`

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

An iterator for the sections in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSectionIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSectionIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSectionIterator<'data, 'file, Mach, R>`

- `type Item = MachOSection<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A section in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md)

- `fn macho_section(self: &Self) -> &'data <Mach as >::Section` — [`MachHeader`](../index.md)

- `fn macho_relocations(self: &Self) -> Result<&'data [macho::Relocation<<Mach as >::Endian>]>` — [`Result`](../../../index.md), [`Relocation`](../../../macho/index.md), [`MachHeader`](../index.md)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn maybe_compressed_gnu(self: &Self) -> Result<Option<CompressedFileRange>>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSection<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSection for MachOSection<'data, 'file, Mach, R>`

- `type RelocationIterator = MachORelocationIterator<'data, 'file, Mach, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- `fn compressed_data(self: &Self) -> read::Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../../index.md)

- `fn relocations(self: &Self) -> MachORelocationIterator<'data, 'file, Mach, R>` — [`MachORelocationIterator`](../index.md)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

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

#### Fields

- **`data`**: `R`

  The data for the file that contains the section data.
  
  This is required for dyld caches, where this may be a different subcache
  from the file containing the Mach-O load commands.

#### Implementations

- `fn parse(index: SectionIndex, section: &'data <Mach as >::Section, data: R) -> Self` — [`SectionIndex`](../../../index.md), [`MachHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Mach: $crate::clone::Clone + MachHeader, R: $crate::clone::Clone + ReadRef<'data>> Clone for MachOSectionInternal<'data, Mach, R>`

- `fn clone(self: &Self) -> MachOSectionInternal<'data, Mach, R>` — [`MachOSectionInternal`](#machosectioninternal)

##### `impl<'data, Mach: $crate::marker::Copy + MachHeader, R: $crate::marker::Copy + ReadRef<'data>> Copy for MachOSectionInternal<'data, Mach, R>`

##### `impl<'data, Mach: $crate::fmt::Debug + MachHeader, R: $crate::fmt::Debug + ReadRef<'data>> Debug for MachOSectionInternal<'data, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Section`

```rust
trait Section: Debug + Pod { ... }
```

A trait for generic access to [`macho::Section32`](../../../macho/index.md) and [`macho::Section64`](../../../macho/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn sectname(self: &Self) -> &[u8; 16]`

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn offset(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn align(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn reloff(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn nreloc(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn name(self: &Self) -> &[u8]`

  Return the `sectname` bytes up until the null terminator.

- `fn segment_name(self: &Self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(self: &Self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

- `fn relocations<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> Result<&'data [macho::Relocation<<Self as >::Endian>]>`

  Return the relocation array.

## Type Aliases

### `MachOSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator32<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the sections in a [`MachOFile32`](super::MachOFile32).

### `MachOSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator64<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the sections in a [`MachOFile64`](super::MachOFile64).

### `MachOSection32<'data, 'file, Endian, R>`

```rust
type MachOSection32<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A section in a [`MachOFile32`](super::MachOFile32).

### `MachOSection64<'data, 'file, Endian, R>`

```rust
type MachOSection64<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A section in a [`MachOFile64`](super::MachOFile64).

