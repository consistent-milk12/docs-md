*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [section](index.md)*

---

# Module `section`

## Structs

### `XcoffSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Xcoff as >::SectionHeader>>,
}
```

An iterator for the sections in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffSection<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `XcoffSection<'data, 'file, Xcoff, R>`

```rust
struct XcoffSection<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    section: &'data <Xcoff as >::SectionHeader,
    index: crate::read::SectionIndex,
}
```

A section in an [`XcoffFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- `fn xcoff_file(self: &Self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](../index.md)

- `fn xcoff_section(self: &Self) -> &'data <Xcoff as >::SectionHeader` — [`FileHeader`](../index.md)

- `fn xcoff_relocations(self: &Self) -> Result<&'data [<Xcoff as >::Rel]>` — [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSection<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectSection for XcoffSection<'data, 'file, Xcoff, R>`

- `type RelocationIterator = XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../../index.md)

- `fn relocations(self: &Self) -> <Self as >::RelocationIterator` — [`ObjectSection`](../../index.md)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

- `fn uncompressed_data(self: &Self) -> Result<alloc::borrow::Cow<'data, [u8]>>` — [`Result`](../../../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffSection<'data, 'file, Xcoff, R>`

### `SectionTable<'data, Xcoff: FileHeader>`

```rust
struct SectionTable<'data, Xcoff: FileHeader> {
    sections: &'data [<Xcoff as >::SectionHeader],
}
```

The table of section headers in an XCOFF file.

Returned by `FileHeader::sections`.

#### Implementations

- `fn parse<R: ReadRef<'data>>(header: &Xcoff, data: R, offset: &mut u64) -> Result<Self>` — [`Result`](../../../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` — [`FileHeader`](../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Xcoff: $crate::clone::Clone + FileHeader> Clone for SectionTable<'data, Xcoff>`

- `fn clone(self: &Self) -> SectionTable<'data, Xcoff>` — [`SectionTable`](../index.md)

##### `impl<'data, Xcoff: $crate::marker::Copy + FileHeader> Copy for SectionTable<'data, Xcoff>`

##### `impl<'data, Xcoff: $crate::fmt::Debug + FileHeader> Debug for SectionTable<'data, Xcoff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Xcoff> Default for SectionTable<'data, Xcoff>`

- `fn default() -> Self`

## Traits

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::SectionHeader32`](../../../xcoff/index.md) and [`xcoff::SectionHeader64`](../../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `type HalfWord: 1`

- `type Xcoff: 1`

- `type Rel: 1`

- `fn s_name(self: &Self) -> &[u8; 8]`

- `fn s_paddr(self: &Self) -> <Self as >::Word`

- `fn s_vaddr(self: &Self) -> <Self as >::Word`

- `fn s_size(self: &Self) -> <Self as >::Word`

- `fn s_scnptr(self: &Self) -> <Self as >::Word`

- `fn s_relptr(self: &Self) -> <Self as >::Word`

- `fn s_lnnoptr(self: &Self) -> <Self as >::Word`

- `fn s_nreloc(self: &Self) -> <Self as >::HalfWord`

- `fn s_nlnno(self: &Self) -> <Self as >::HalfWord`

- `fn s_flags(self: &Self) -> u32`

- `fn name(self: &Self) -> &[u8]`

  Return the section name.

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

- `fn relocations<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<&'data [<Self as >::Rel]>`

  Read the relocations.

## Type Aliases

### `XcoffSectionIterator32<'data, 'file, R>`

```rust
type XcoffSectionIterator32<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSectionIterator64<'data, 'file, R>`

```rust
type XcoffSectionIterator64<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSection32<'data, 'file, R>`

```rust
type XcoffSection32<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader32, R>;
```

A section in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSection64<'data, 'file, R>`

```rust
type XcoffSection64<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader64, R>;
```

A section in an [`XcoffFile64`](super::XcoffFile64).

