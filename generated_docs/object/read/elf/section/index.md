*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [section](index.md)*

---

# Module `section`

## Structs

### `SectionTable<'data, Elf: FileHeader, R>`

```rust
struct SectionTable<'data, Elf: FileHeader, R>
where
    R: ReadRef<'data> {
    sections: &'data [<Elf as >::SectionHeader],
    strings: crate::read::StringTable<'data, R>,
}
```

The table of section headers in an ELF file.

Also includes the string table used for the section names.

Returned by `FileHeader::sections`.

#### Implementations

- `fn new(sections: &'data [<Elf as >::SectionHeader], strings: StringTable<'data, R>) -> Self` — [`FileHeader`](../index.md), [`StringTable`](../../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, <Elf as >::SectionHeader>` — [`FileHeader`](../index.md)

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SectionIndex, &'data <Elf as >::SectionHeader)>` — [`SectionIndex`](../../../index.md), [`FileHeader`](../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data <Elf as >::SectionHeader>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn section_by_name(self: &Self, endian: <Elf as >::Endian, name: &[u8]) -> Option<(SectionIndex, &'data <Elf as >::SectionHeader)>` — [`FileHeader`](../index.md), [`SectionIndex`](../../../index.md)

- `fn section_name(self: &Self, endian: <Elf as >::Endian, section: &<Elf as >::SectionHeader) -> read::Result<&'data [u8]>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- `fn strings(self: &Self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<StringTable<'data, R>>` — [`FileHeader`](../index.md), [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`StringTable`](../../index.md)

- `fn symbols(self: &Self, endian: <Elf as >::Endian, data: R, sh_type: u32) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`SymbolTable`](../index.md)

- `fn symbol_table_by_index(self: &Self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](../index.md), [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`SymbolTable`](../index.md)

- `fn relocation_sections(self: &Self, endian: <Elf as >::Endian, symbol_section: SectionIndex) -> read::Result<RelocationSections>` — [`FileHeader`](../index.md), [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`RelocationSections`](../index.md)

- `fn dynamic(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [<Elf as >::Dyn], SectionIndex)>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`SectionIndex`](../../../index.md)

- `fn hash_header(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Elf as >::Endian>>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`HashHeader`](../../../elf/index.md)

- `fn hash(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`HashTable`](../index.md), [`SectionIndex`](../../../index.md)

- `fn gnu_hash_header(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Elf as >::Endian>>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`GnuHashHeader`](../../../elf/index.md)

- `fn gnu_hash(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`GnuHashTable`](../index.md), [`SectionIndex`](../../../index.md)

- `fn gnu_versym(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Elf as >::Endian>], SectionIndex)>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`Versym`](../../../elf/index.md), [`SectionIndex`](../../../index.md)

- `fn gnu_verdef(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`VerdefIterator`](../index.md), [`SectionIndex`](../../../index.md)

- `fn gnu_verneed(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`VerneedIterator`](../index.md), [`SectionIndex`](../../../index.md)

- `fn versions(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<VersionTable<'data, Elf>>>` — [`FileHeader`](../index.md), [`Result`](../../../index.md), [`VersionTable`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader, R> Clone for SectionTable<'data, Elf, R>`

- `fn clone(self: &Self) -> SectionTable<'data, Elf, R>` — [`SectionTable`](../index.md)

##### `impl<'data, Elf: $crate::marker::Copy + FileHeader, R> Copy for SectionTable<'data, Elf, R>`

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader, R> Debug for SectionTable<'data, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Elf: FileHeader, R: ReadRef<'data>> Default for SectionTable<'data, Elf, R>`

- `fn default() -> Self`

### `ElfSectionIterator<'data, 'file, Elf, R>`

```rust
struct ElfSectionIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Elf as >::SectionHeader>>,
}
```

An iterator for the sections in an [`ElfFile`](../index.md).

#### Implementations

- `fn new(file: &'file ElfFile<'data, Elf, R>) -> Self` — [`ElfFile`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSectionIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ElfSectionIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSectionIterator<'data, 'file, Elf, R>`

- `type Item = ElfSection<'data, 'file, Elf, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfSection<'data, 'file, Elf, R>`

```rust
struct ElfSection<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    index: crate::read::SectionIndex,
    section: &'data <Elf as >::SectionHeader,
}
```

A section in an [`ElfFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](../index.md)

- `fn elf_section_header(self: &Self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](../index.md)

- `fn elf_relocation_section_index(self: &Self) -> read::Result<Option<SectionIndex>>` — [`Result`](../../../index.md), [`SectionIndex`](../../../index.md)

- `fn elf_relocation_section(self: &Self) -> read::Result<Option<&'data <Elf as >::SectionHeader>>` — [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn elf_linked_rel(self: &Self) -> read::Result<&'data [<Elf as >::Rel]>` — [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn elf_linked_rela(self: &Self) -> read::Result<&'data [<Elf as >::Rela]>` — [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn maybe_compressed(self: &Self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- `fn maybe_compressed_gnu(self: &Self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSection<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectSection for ElfSection<'data, 'file, Elf, R>`

- `type RelocationIterator = ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn compressed_file_range(self: &Self) -> read::Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- `fn compressed_data(self: &Self) -> read::Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../../index.md)

- `fn segment_name_bytes(self: &Self) -> read::Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn segment_name(self: &Self) -> read::Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../../index.md)

- `fn relocations(self: &Self) -> ElfSectionRelocationIterator<'data, 'file, Elf, R>` — [`ElfSectionRelocationIterator`](../index.md)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfSection<'data, 'file, Elf, R>`

## Traits

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::SectionHeader32`](../../../elf/index.md) and [`elf::SectionHeader64`](../../../elf/index.md).

#### Required Methods

- `type Elf: 1`

- `type Word: 1`

- `type Endian: 1`

- `fn sh_name(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_flags(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_link(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_info(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sh_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_entsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn name<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> read::Result<&'data [u8]>`

  Parse the section name from the string table.

- `fn link(self: &Self, endian: <Self as >::Endian) -> SectionIndex`

  Get the `sh_link` field as a section index.

- `fn has_info_link(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the `SHF_INFO_LINK` flag is set.

- `fn info_link(self: &Self, endian: <Self as >::Endian) -> SectionIndex`

  Get the `sh_info` field as a section index.

- `fn file_range(self: &Self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [u8]>`

  Return the section data.

- `fn data_as_array<'data, T: Pod, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [T]>`

  Return the section data as a slice of the given type.

- `fn strings<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<StringTable<'data, R>>>`

  Return the strings in the section.

- `fn symbols<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, sections: &SectionTable<'data, <Self as >::Elf, R>, section_index: SectionIndex) -> read::Result<Option<SymbolTable<'data, <Self as >::Elf, R>>>`

  Return the symbols in the section.

- `fn rel<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Rel], SectionIndex)>>`

  Return the `Elf::Rel` entries in the section.

- `fn rela<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Rela], SectionIndex)>>`

  Return the `Elf::Rela` entries in the section.

- `fn relr<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<RelrIterator<'data, <Self as >::Elf>>>`

  Return the `Elf::Relr` entries in the section.

- `fn crel<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(CrelIterator<'data>, SectionIndex)>>`

  Return the `Crel` entries in the section.

- `fn dynamic<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Dyn], SectionIndex)>>`

  Return entries in a dynamic section.

- `fn notes<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<NoteIterator<'data, <Self as >::Elf>>>`

  Return a note iterator for the section data.

- `fn group<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(u32, &'data [U32Bytes<<Self as >::Endian>])>>`

  Return the contents of a group section.

- `fn hash_header<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Self as >::Endian>>>`

  Return the header of a SysV hash section.

- `fn hash<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, <Self as >::Elf>, SectionIndex)>>`

  Return the contents of a SysV hash section.

- `fn gnu_hash_header<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Self as >::Endian>>>`

  Return the header of a GNU hash section.

- `fn gnu_hash<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, <Self as >::Elf>, SectionIndex)>>`

  Return the contents of a GNU hash section.

- `fn gnu_versym<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Self as >::Endian>], SectionIndex)>>`

  Return the contents of a `SHT_GNU_VERSYM` section.

- `fn gnu_verdef<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, <Self as >::Elf>, SectionIndex)>>`

  Return an iterator for the entries of a `SHT_GNU_VERDEF` section.

- `fn gnu_verneed<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, <Self as >::Elf>, SectionIndex)>>`

  Return an iterator for the entries of a `SHT_GNU_VERNEED` section.

- `fn gnu_attributes<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<AttributesSection<'data, <Self as >::Elf>>>`

  Return the contents of a `SHT_GNU_ATTRIBUTES` section.

- `fn attributes<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<AttributesSection<'data, <Self as >::Elf>>`

  Parse the contents of the section as attributes.

- `fn compression<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data <<Self as >::Elf as FileHeader>::CompressionHeader, u64, u64)>>`

  Parse the compression header if present.

## Type Aliases

### `ElfSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator32<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the sections in an [`ElfFile32`](super::ElfFile32).

### `ElfSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator64<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the sections in an [`ElfFile64`](super::ElfFile64).

### `ElfSection32<'data, 'file, Endian, R>`

```rust
type ElfSection32<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A section in an [`ElfFile32`](super::ElfFile32).

### `ElfSection64<'data, 'file, Endian, R>`

```rust
type ElfSection64<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A section in an [`ElfFile64`](super::ElfFile64).

