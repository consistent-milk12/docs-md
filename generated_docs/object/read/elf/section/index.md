*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`SectionTable`](#sectiontable)
  - [`ElfSectionIterator`](#elfsectioniterator)
  - [`ElfSection`](#elfsection)
- [Traits](#traits)
  - [`SectionHeader`](#sectionheader)
- [Type Aliases](#type-aliases)
  - [`ElfSectionIterator32`](#elfsectioniterator32)
  - [`ElfSectionIterator64`](#elfsectioniterator64)
  - [`ElfSection32`](#elfsection32)
  - [`ElfSection64`](#elfsection64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SectionTable`](#sectiontable) | struct | The table of section headers in an ELF file. |
| [`ElfSectionIterator`](#elfsectioniterator) | struct | An iterator for the sections in an [`ElfFile`]. |
| [`ElfSection`](#elfsection) | struct | A section in an [`ElfFile`]. |
| [`SectionHeader`](#sectionheader) | trait | A trait for generic access to [`elf::SectionHeader32`] and [`elf::SectionHeader64`]. |
| [`ElfSectionIterator32`](#elfsectioniterator32) | type | An iterator for the sections in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSectionIterator64`](#elfsectioniterator64) | type | An iterator for the sections in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSection32`](#elfsection32) | type | A section in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSection64`](#elfsection64) | type | A section in an [`ElfFile64`](super::ElfFile64). |

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

*Defined in [`object-0.37.3/src/read/elf/section.rs:25-31`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L25-L31)*

The table of section headers in an ELF file.

Also includes the string table used for the section names.

Returned by `FileHeader::sections`.

#### Implementations

- <span id="sectiontable-new"></span>`fn new(sections: &'data [<Elf as >::SectionHeader], strings: StringTable<'data, R>) -> Self` — [`FileHeader`](../index.md#fileheader), [`StringTable`](../../index.md#stringtable)

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Elf as >::SectionHeader>` — [`FileHeader`](../index.md#fileheader)

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data <Elf as >::SectionHeader)>` — [`SectionIndex`](../../../index.md#sectionindex), [`FileHeader`](../index.md#fileheader)

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Elf as >::SectionHeader>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="sectiontable-section-by-name"></span>`fn section_by_name(&self, endian: <Elf as >::Endian, name: &[u8]) -> Option<(SectionIndex, &'data <Elf as >::SectionHeader)>` — [`FileHeader`](../index.md#fileheader), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="sectiontable-section-name"></span>`fn section_name(&self, endian: <Elf as >::Endian, section: &<Elf as >::SectionHeader) -> read::Result<&'data [u8]>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

- <span id="sectiontable-strings"></span>`fn strings(&self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<StringTable<'data, R>>` — [`FileHeader`](../index.md#fileheader), [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`StringTable`](../../index.md#stringtable)

- <span id="sectiontable-symbols"></span>`fn symbols(&self, endian: <Elf as >::Endian, data: R, sh_type: u32) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`SymbolTable`](../index.md#symboltable)

- <span id="sectiontable-symbol-table-by-index"></span>`fn symbol_table_by_index(&self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](../index.md#fileheader), [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`SymbolTable`](../index.md#symboltable)

- <span id="sectiontable-relocation-sections"></span>`fn relocation_sections(&self, endian: <Elf as >::Endian, symbol_section: SectionIndex) -> read::Result<RelocationSections>` — [`FileHeader`](../index.md#fileheader), [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`RelocationSections`](../index.md#relocationsections)

- <span id="sectiontable-dynamic"></span>`fn dynamic(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [<Elf as >::Dyn], SectionIndex)>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="sectiontable-hash-header"></span>`fn hash_header(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Elf as >::Endian>>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`HashHeader`](../../../elf/index.md#hashheader)

- <span id="sectiontable-hash"></span>`fn hash(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`HashTable`](../index.md#hashtable), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="sectiontable-gnu-hash-header"></span>`fn gnu_hash_header(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Elf as >::Endian>>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`GnuHashHeader`](../../../elf/index.md#gnuhashheader)

- <span id="sectiontable-gnu-hash"></span>`fn gnu_hash(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`GnuHashTable`](../index.md#gnuhashtable), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="sectiontable-gnu-versym"></span>`fn gnu_versym(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Elf as >::Endian>], SectionIndex)>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`Versym`](../../../elf/index.md#versym), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="sectiontable-gnu-verdef"></span>`fn gnu_verdef(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`VerdefIterator`](../index.md#verdefiterator), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="sectiontable-gnu-verneed"></span>`fn gnu_verneed(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`VerneedIterator`](../index.md#verneediterator), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="sectiontable-versions"></span>`fn versions(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<VersionTable<'data, Elf>>>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result), [`VersionTable`](../index.md#versiontable)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader, R> Clone for SectionTable<'data, Elf, R>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Elf, R>` — [`SectionTable`](../index.md#sectiontable)

##### `impl<'data, Elf: marker::Copy + FileHeader, R> Copy for SectionTable<'data, Elf, R>`

##### `impl<'data, Elf: fmt::Debug + FileHeader, R> Debug for SectionTable<'data, Elf, R>`

- <span id="sectiontable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Elf: FileHeader, R: ReadRef<'data>> Default for SectionTable<'data, Elf, R>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

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

*Defined in [`object-0.37.3/src/read/elf/section.rs:358-365`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L358-L365)*

An iterator for the sections in an [`ElfFile`](../index.md).

#### Implementations

- <span id="elfsectioniterator-new"></span>`fn new(file: &'file ElfFile<'data, Elf, R>) -> Self` — [`ElfFile`](../index.md#elffile)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-type-item"></span>`type Item = ElfSection<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/elf/section.rs:406-414`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L406-L414)*

A section in an [`ElfFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="elfsection-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](../index.md#elffile)

- <span id="elfsection-elf-section-header"></span>`fn elf_section_header(&self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](../index.md#fileheader)

- <span id="elfsection-elf-relocation-section-index"></span>`fn elf_relocation_section_index(&self) -> read::Result<Option<SectionIndex>>` — [`Result`](../../../index.md#result), [`SectionIndex`](../../../index.md#sectionindex)

- <span id="elfsection-elf-relocation-section"></span>`fn elf_relocation_section(&self) -> read::Result<Option<&'data <Elf as >::SectionHeader>>` — [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="elfsection-elf-linked-rel"></span>`fn elf_linked_rel(&self) -> read::Result<&'data [<Elf as >::Rel]>` — [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="elfsection-elf-linked-rela"></span>`fn elf_linked_rela(&self) -> read::Result<&'data [<Elf as >::Rela]>` — [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="elfsection-bytes"></span>`fn bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="elfsection-maybe-compressed"></span>`fn maybe_compressed(&self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

- <span id="elfsection-maybe-compressed-gnu"></span>`fn maybe_compressed_gnu(&self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectSection for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-type-relocationiterator"></span>`type RelocationIterator = ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

- <span id="elfsection-address"></span>`fn address(&self) -> u64`

- <span id="elfsection-size"></span>`fn size(&self) -> u64`

- <span id="elfsection-align"></span>`fn align(&self) -> u64`

- <span id="elfsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="elfsection-data"></span>`fn data(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="elfsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="elfsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> read::Result<CompressedFileRange>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

- <span id="elfsection-compressed-data"></span>`fn compressed_data(&self) -> read::Result<CompressedData<'data>>` — [`Result`](../../../index.md#result), [`CompressedData`](../../../index.md#compresseddata)

- <span id="elfsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="elfsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="elfsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> read::Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="elfsection-segment-name"></span>`fn segment_name(&self) -> read::Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="elfsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md#sectionkind)

- <span id="elfsection-relocations"></span>`fn relocations(&self) -> ElfSectionRelocationIterator<'data, 'file, Elf, R>` — [`ElfSectionRelocationIterator`](../index.md#elfsectionrelocationiterator)

- <span id="elfsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md#result), [`RelocationMap`](../../../index.md#relocationmap)

- <span id="elfsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md#sectionflags)

##### `impl<'data, 'file, Elf, R> Sealed for ElfSection<'data, 'file, Elf, R>`

## Traits

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:686-1170`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L686-L1170)*

A trait for generic access to [`elf::SectionHeader32`](../../../elf/index.md) and [`elf::SectionHeader64`](../../../elf/index.md).

#### Associated Types

- `type Elf: 1`

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn sh_name(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_type(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_flags(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_addr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_link(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_info(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_entsize(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> read::Result<&'data [u8]>`

  Parse the section name from the string table.

- `fn link(&self, endian: <Self as >::Endian) -> SectionIndex`

  Get the `sh_link` field as a section index.

- `fn has_info_link(&self, endian: <Self as >::Endian) -> bool`

  Return true if the `SHF_INFO_LINK` flag is set.

- `fn info_link(&self, endian: <Self as >::Endian) -> SectionIndex`

  Get the `sh_info` field as a section index.

- `fn file_range(&self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [u8]>`

  Return the section data.

- `fn data_as_array<'data, T: Pod, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [T]>`

  Return the section data as a slice of the given type.

- `fn strings<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<StringTable<'data, R>>>`

  Return the strings in the section.

- `fn symbols<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, sections: &SectionTable<'data, <Self as >::Elf, R>, section_index: SectionIndex) -> read::Result<Option<SymbolTable<'data, <Self as >::Elf, R>>>`

  Return the symbols in the section.

- `fn rel<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Rel], SectionIndex)>>`

  Return the `Elf::Rel` entries in the section.

- `fn rela<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Rela], SectionIndex)>>`

  Return the `Elf::Rela` entries in the section.

- `fn relr<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<RelrIterator<'data, <Self as >::Elf>>>`

  Return the `Elf::Relr` entries in the section.

- `fn crel<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(CrelIterator<'data>, SectionIndex)>>`

  Return the `Crel` entries in the section.

- `fn dynamic<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Dyn], SectionIndex)>>`

  Return entries in a dynamic section.

- `fn notes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<NoteIterator<'data, <Self as >::Elf>>>`

  Return a note iterator for the section data.

- `fn group<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(u32, &'data [U32Bytes<<Self as >::Endian>])>>`

  Return the contents of a group section.

- `fn hash_header<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Self as >::Endian>>>`

  Return the header of a SysV hash section.

- `fn hash<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, <Self as >::Elf>, SectionIndex)>>`

  Return the contents of a SysV hash section.

- `fn gnu_hash_header<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Self as >::Endian>>>`

  Return the header of a GNU hash section.

- `fn gnu_hash<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, <Self as >::Elf>, SectionIndex)>>`

  Return the contents of a GNU hash section.

- `fn gnu_versym<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Self as >::Endian>], SectionIndex)>>`

  Return the contents of a `SHT_GNU_VERSYM` section.

- `fn gnu_verdef<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, <Self as >::Elf>, SectionIndex)>>`

  Return an iterator for the entries of a `SHT_GNU_VERDEF` section.

- `fn gnu_verneed<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, <Self as >::Elf>, SectionIndex)>>`

  Return an iterator for the entries of a `SHT_GNU_VERNEED` section.

- `fn gnu_attributes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<AttributesSection<'data, <Self as >::Elf>>>`

  Return the contents of a `SHT_GNU_ATTRIBUTES` section.

- `fn attributes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<AttributesSection<'data, <Self as >::Elf>>`

  Parse the contents of the section as attributes.

- `fn compression<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data <<Self as >::Elf as FileHeader>::CompressionHeader, u64, u64)>>`

  Parse the compression header if present.

#### Implementors

- [`SectionHeader32`](../../../elf/index.md#sectionheader32)
- [`SectionHeader64`](../../../elf/index.md#sectionheader64)

## Type Aliases

### `ElfSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator32<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:350-351`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L350-L351)*

An iterator for the sections in an [`ElfFile32`](super::ElfFile32).

### `ElfSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator64<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:353-354`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L353-L354)*

An iterator for the sections in an [`ElfFile64`](super::ElfFile64).

### `ElfSection32<'data, 'file, Endian, R>`

```rust
type ElfSection32<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:396-397`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L396-L397)*

A section in an [`ElfFile32`](super::ElfFile32).

### `ElfSection64<'data, 'file, Endian, R>`

```rust
type ElfSection64<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:399-400`](../../../../../.source_1765210505/object-0.37.3/src/read/elf/section.rs#L399-L400)*

A section in an [`ElfFile64`](super::ElfFile64).

