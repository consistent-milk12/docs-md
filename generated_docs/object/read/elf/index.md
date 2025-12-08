*[object](../../index.md) / [read](../index.md) / [elf](index.md)*

---

# Module `elf`

Support for reading ELF files.

Traits are used to abstract over the difference between 32-bit and 64-bit ELF.
The primary trait for this is [`FileHeader`](#fileheader).

## High level API

[`ElfFile`](#elffile) implements the [`Object`](crate::read::Object) trait for ELF files.
[`ElfFile`](#elffile) is parameterised by [`FileHeader`](#fileheader) to allow reading both 32-bit and
64-bit ELF. There are type aliases for these parameters ([`ElfFile32`](#elffile32) and
[`ElfFile64`](#elffile64)).

## Low level API

The [`FileHeader`](#fileheader) trait can be directly used to parse both [`elf::FileHeader32`](../../elf/index.md)
and [`elf::FileHeader64`](../../elf/index.md).

### Example for low level API
 ```no_run
use object::elf;
use object::read::elf::{FileHeader, Sym};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let elf = elf::FileHeader64::<object::Endianness>::parse(&*data)?;
    let endian = elf.endian()?;
    let sections = elf.sections(endian, &*data)?;
    let symbols = sections.symbols(endian, &*data, elf::SHT_SYMTAB)?;
    for symbol in symbols.iter() {
        let name = symbol.name(endian, symbols.strings())?;
        println!("{}", String::from_utf8_lossy(name));
    }
  }
    Ok(())
}
```

## Modules

- [`file`](file/index.md) - 
- [`segment`](segment/index.md) - 
- [`section`](section/index.md) - 
- [`symbol`](symbol/index.md) - 
- [`relocation`](relocation/index.md) - 
- [`comdat`](comdat/index.md) - 
- [`dynamic`](dynamic/index.md) - 
- [`compression`](compression/index.md) - 
- [`note`](note/index.md) - 
- [`hash`](hash/index.md) - 
- [`version`](version/index.md) - 
- [`attributes`](attributes/index.md) - 

## Structs

### `ElfFile<'data, Elf, R>`

```rust
struct ElfFile<'data, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    data: R,
    header: &'data Elf,
    segments: &'data [<Elf as >::ProgramHeader],
    sections: super::SectionTable<'data, Elf, R>,
    relocations: super::RelocationSections,
    symbols: super::SymbolTable<'data, Elf, R>,
    dynamic_symbols: super::SymbolTable<'data, Elf, R>,
}
```

A partially parsed ELF file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> read::Result<Self>` — [`Result`](../../index.md)

- `fn endian(self: &Self) -> <Elf as >::Endian` — [`FileHeader`](#fileheader)

- `fn data(self: &Self) -> R`

- `fn raw_header(self: &Self) -> &'data Elf`

- `fn raw_segments(self: &Self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](#fileheader)

- `fn elf_header(self: &Self) -> &'data Elf`

- `fn elf_program_headers(self: &Self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](#fileheader)

- `fn elf_section_table(self: &Self) -> &SectionTable<'data, Elf, R>` — [`SectionTable`](#sectiontable)

- `fn elf_symbol_table(self: &Self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

- `fn elf_dynamic_symbol_table(self: &Self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

- `fn elf_relocation_sections(self: &Self) -> &RelocationSections` — [`RelocationSections`](#relocationsections)

- `fn raw_section_by_name<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

- `fn zdebug_section_by_name<'file>(self: &'file Self, _section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

#### Trait Implementations

##### `impl<'data, Elf, R> Debug for ElfFile<'data, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Elf, R> Object for ElfFile<'data, Elf, R>`

- `type Segment = ElfSegment<'data, 'file, Elf, R>`

- `type SegmentIterator = ElfSegmentIterator<'data, 'file, Elf, R>`

- `type Section = ElfSection<'data, 'file, Elf, R>`

- `type SectionIterator = ElfSectionIterator<'data, 'file, Elf, R>`

- `type Comdat = ElfComdat<'data, 'file, Elf, R>`

- `type ComdatIterator = ElfComdatIterator<'data, 'file, Elf, R>`

- `type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- `type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- `type SymbolTable = ElfSymbolTable<'data, 'file, Elf, R>`

- `type DynamicRelocationIterator = ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- `fn segments(self: &Self) -> ElfSegmentIterator<'data, '_, Elf, R>` — [`ElfSegmentIterator`](#elfsegmentiterator)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

- `fn section_by_index(self: &Self, index: SectionIndex) -> read::Result<ElfSection<'data, '_, Elf, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`ElfSection`](#elfsection)

- `fn sections(self: &Self) -> ElfSectionIterator<'data, '_, Elf, R>` — [`ElfSectionIterator`](#elfsectioniterator)

- `fn comdats(self: &Self) -> ElfComdatIterator<'data, '_, Elf, R>` — [`ElfComdatIterator`](#elfcomdatiterator)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<ElfSymbol<'data, '_, Elf, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ElfSymbol`](#elfsymbol)

- `fn symbols(self: &Self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](#elfsymboliterator)

- `fn symbol_table(self: &Self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](#elfsymboltable)

- `fn dynamic_symbols(self: &Self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](#elfsymboliterator)

- `fn dynamic_symbol_table(self: &Self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](#elfsymboltable)

- `fn dynamic_relocations<'file>(self: &'file Self) -> Option<ElfDynamicRelocationIterator<'data, 'file, Elf, R>>` — [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator)

- `fn imports(self: &Self) -> read::Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- `fn exports(self: &Self) -> read::Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn build_id(self: &Self) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn gnu_debuglink(self: &Self) -> read::Result<Option<(&'data [u8], u32)>>` — [`Result`](../../index.md)

- `fn gnu_debugaltlink(self: &Self) -> read::Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../index.md)

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, Elf, R> Sealed for ElfFile<'data, Elf, R>`

### `ElfSegmentIterator<'data, 'file, Elf, R>`

```rust
struct ElfSegmentIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: slice::Iter<'data, <Elf as >::ProgramHeader>,
}
```

An iterator for the segments in an [`ElfFile`](#elffile).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSegmentIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- `type Item = ElfSegment<'data, 'file, Elf, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfSegment<'data, 'file, Elf, R>`

```rust
struct ElfSegment<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    segment: &'data <Elf as >::ProgramHeader,
}
```

A segment in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

- `fn elf_program_header(self: &Self) -> &'data <Elf as >::ProgramHeader` — [`FileHeader`](#fileheader)

- `fn bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSegment<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectSegment for ElfSegment<'data, 'file, Elf, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> read::Result<Option<&str>>` — [`Result`](../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfSegment<'data, 'file, Elf, R>`

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

- `fn new(sections: &'data [<Elf as >::SectionHeader], strings: StringTable<'data, R>) -> Self` — [`FileHeader`](#fileheader), [`StringTable`](../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, <Elf as >::SectionHeader>` — [`FileHeader`](#fileheader)

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SectionIndex, &'data <Elf as >::SectionHeader)>` — [`SectionIndex`](../../index.md), [`FileHeader`](#fileheader)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data <Elf as >::SectionHeader>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn section_by_name(self: &Self, endian: <Elf as >::Endian, name: &[u8]) -> Option<(SectionIndex, &'data <Elf as >::SectionHeader)>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md)

- `fn section_name(self: &Self, endian: <Elf as >::Endian, section: &<Elf as >::SectionHeader) -> read::Result<&'data [u8]>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- `fn strings(self: &Self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<StringTable<'data, R>>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`StringTable`](../index.md)

- `fn symbols(self: &Self, endian: <Elf as >::Endian, data: R, sh_type: u32) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`SymbolTable`](#symboltable)

- `fn symbol_table_by_index(self: &Self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`SymbolTable`](#symboltable)

- `fn relocation_sections(self: &Self, endian: <Elf as >::Endian, symbol_section: SectionIndex) -> read::Result<RelocationSections>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`RelocationSections`](#relocationsections)

- `fn dynamic(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [<Elf as >::Dyn], SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`SectionIndex`](../../index.md)

- `fn hash_header(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Elf as >::Endian>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`HashHeader`](../../elf/index.md)

- `fn hash(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`HashTable`](#hashtable), [`SectionIndex`](../../index.md)

- `fn gnu_hash_header(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Elf as >::Endian>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`GnuHashHeader`](../../elf/index.md)

- `fn gnu_hash(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`GnuHashTable`](#gnuhashtable), [`SectionIndex`](../../index.md)

- `fn gnu_versym(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Elf as >::Endian>], SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`Versym`](../../elf/index.md), [`SectionIndex`](../../index.md)

- `fn gnu_verdef(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`VerdefIterator`](#verdefiterator), [`SectionIndex`](../../index.md)

- `fn gnu_verneed(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`VerneedIterator`](#verneediterator), [`SectionIndex`](../../index.md)

- `fn versions(self: &Self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<VersionTable<'data, Elf>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`VersionTable`](#versiontable)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader, R> Clone for SectionTable<'data, Elf, R>`

- `fn clone(self: &Self) -> SectionTable<'data, Elf, R>` — [`SectionTable`](#sectiontable)

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

An iterator for the sections in an [`ElfFile`](#elffile).

#### Implementations

- `fn new(file: &'file ElfFile<'data, Elf, R>) -> Self` — [`ElfFile`](#elffile)

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

A section in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

- `fn elf_section_header(self: &Self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](#fileheader)

- `fn elf_relocation_section_index(self: &Self) -> read::Result<Option<SectionIndex>>` — [`Result`](../../index.md), [`SectionIndex`](../../index.md)

- `fn elf_relocation_section(self: &Self) -> read::Result<Option<&'data <Elf as >::SectionHeader>>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn elf_linked_rel(self: &Self) -> read::Result<&'data [<Elf as >::Rel]>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn elf_linked_rela(self: &Self) -> read::Result<&'data [<Elf as >::Rela]>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn maybe_compressed(self: &Self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- `fn maybe_compressed_gnu(self: &Self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSection<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectSection for ElfSection<'data, 'file, Elf, R>`

- `type RelocationIterator = ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn compressed_file_range(self: &Self) -> read::Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- `fn compressed_data(self: &Self) -> read::Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- `fn segment_name_bytes(self: &Self) -> read::Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn segment_name(self: &Self) -> read::Result<Option<&str>>` — [`Result`](../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../index.md)

- `fn relocations(self: &Self) -> ElfSectionRelocationIterator<'data, 'file, Elf, R>` — [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfSection<'data, 'file, Elf, R>`

### `SymbolTable<'data, Elf: FileHeader, R>`

```rust
struct SymbolTable<'data, Elf: FileHeader, R>
where
    R: ReadRef<'data> {
    section: crate::read::SectionIndex,
    string_section: crate::read::SectionIndex,
    shndx_section: crate::read::SectionIndex,
    symbols: &'data [<Elf as >::Sym],
    strings: crate::read::util::StringTable<'data, R>,
    shndx: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A table of symbol entries in an ELF file.

Also includes the string table used for the symbol names.

Returned by `SectionTable::symbols`.

#### Implementations

- `fn parse(endian: <Elf as >::Endian, data: R, sections: &SectionTable<'data, Elf, R>, section_index: SectionIndex, section: &<Elf as >::SectionHeader) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`SectionTable`](#sectiontable), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`SymbolTable`](#symboltable)

- `fn section(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn shndx_section(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn string_section(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- `fn symbols(self: &Self) -> &'data [<Elf as >::Sym]` — [`FileHeader`](#fileheader)

- `fn iter(self: &Self) -> slice::Iter<'data, <Elf as >::Sym>` — [`FileHeader`](#fileheader)

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SymbolIndex, &'data <Elf as >::Sym)>` — [`SymbolIndex`](../../index.md), [`FileHeader`](#fileheader)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn symbol(self: &Self, index: SymbolIndex) -> read::Result<&'data <Elf as >::Sym>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn shndx(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> Option<u32>` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- `fn symbol_section(self: &Self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym, index: SymbolIndex) -> read::Result<Option<SectionIndex>>` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`SectionIndex`](../../index.md)

- `fn symbol_name(self: &Self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym) -> read::Result<&'data [u8]>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Elf as >::Sym) -> Option<Entry>>(self: &Self, endian: <Elf as >::Endian, f: F) -> SymbolMap<Entry>` — [`FileHeader`](#fileheader), [`SymbolMap`](../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader, R> Clone for SymbolTable<'data, Elf, R>`

- `fn clone(self: &Self) -> SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

##### `impl<'data, Elf: $crate::marker::Copy + FileHeader, R> Copy for SymbolTable<'data, Elf, R>`

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader, R> Debug for SymbolTable<'data, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Elf: FileHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Elf, R>`

- `fn default() -> Self`

### `ElfSymbolTable<'data, 'file, Elf, R>`

```rust
struct ElfSymbolTable<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
}
```

A symbol table in an [`ElfFile`](super::ElfFile).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Clone for ElfSymbolTable<'data, 'file, Elf, R>`

- `fn clone(self: &Self) -> ElfSymbolTable<'data, 'file, Elf, R>` — [`ElfSymbolTable`](#elfsymboltable)

##### `impl<'data, 'file, Elf, R> Copy for ElfSymbolTable<'data, 'file, Elf, R>`

##### `impl<'data, 'file, Elf, R> Debug for ElfSymbolTable<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for ElfSymbolTable<'data, 'file, Elf, R>`

- `type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- `type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbolTable<'data, 'file, Elf, R>`

### `ElfSymbolIterator<'data, 'file, Elf, R>`

```rust
struct ElfSymbolIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in an [`ElfFile`](super::ElfFile).

#### Implementations

- `fn new(endian: <Elf as >::Endian, symbols: &'file SymbolTable<'data, Elf, R>) -> Self` — [`FileHeader`](#fileheader), [`SymbolTable`](#symboltable)

#### Trait Implementations

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Debug for ElfSymbolIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Iterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- `type Item = ElfSymbol<'data, 'file, Elf, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfSymbol<'data, 'file, Elf, R>`

```rust
struct ElfSymbol<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Elf as >::Sym,
}
```

A symbol in an [`ElfFile`](super::ElfFile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- `fn endian(self: &Self) -> <Elf as >::Endian` — [`FileHeader`](#fileheader)

- `fn raw_symbol(self: &Self) -> &'data <Elf as >::Sym` — [`FileHeader`](#fileheader)

- `fn elf_symbol(self: &Self) -> &'data <Elf as >::Sym` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Clone for ElfSymbol<'data, 'file, Elf, R>`

- `fn clone(self: &Self) -> ElfSymbol<'data, 'file, Elf, R>` — [`ElfSymbol`](#elfsymbol)

##### `impl<'data, 'file, Elf, R> Copy for ElfSymbol<'data, 'file, Elf, R>`

##### `impl<'data, 'file, Elf, R> Debug for ElfSymbol<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> ObjectSymbol for ElfSymbol<'data, 'file, Elf, R>`

- `fn index(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn kind(self: &Self) -> SymbolKind` — [`SymbolKind`](../../index.md)

- `fn section(self: &Self) -> SymbolSection` — [`SymbolSection`](../../index.md)

- `fn is_undefined(self: &Self) -> bool`

- `fn is_definition(self: &Self) -> bool`

- `fn is_common(self: &Self) -> bool`

- `fn is_weak(self: &Self) -> bool`

- `fn scope(self: &Self) -> SymbolScope` — [`SymbolScope`](../../index.md)

- `fn is_global(self: &Self) -> bool`

- `fn is_local(self: &Self) -> bool`

- `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md), [`SectionIndex`](../../index.md), [`SymbolIndex`](../../index.md)

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbol<'data, 'file, Elf, R>`

### `RelocationSections`

```rust
struct RelocationSections {
    relocations: alloc::vec::Vec<usize>,
}
```

A mapping from section index to associated relocation sections.

#### Implementations

- `fn parse<'data, Elf: FileHeader, R: ReadRef<'data>>(endian: <Elf as >::Endian, sections: &SectionTable<'data, Elf, R>, symbol_section: SectionIndex) -> read::Result<Self>` — [`FileHeader`](#fileheader), [`SectionTable`](#sectiontable), [`SectionIndex`](../../index.md), [`Result`](../../index.md)

- `fn get(self: &Self, index: SectionIndex) -> Option<SectionIndex>` — [`SectionIndex`](../../index.md)

#### Trait Implementations

##### `impl Debug for RelocationSections`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RelocationSections`

- `fn default() -> RelocationSections` — [`RelocationSections`](#relocationsections)

### `ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

```rust
struct ElfDynamicRelocationIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    section_index: crate::read::SectionIndex,
    file: &'file super::ElfFile<'data, Elf, R>,
    relocations: Option<ElfRelocationIterator<'data, Elf>>,
}
```

An iterator for the dynamic relocations in an [`ElfFile`](#elffile).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current relocation section index.

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfSectionRelocationIterator<'data, 'file, Elf, R>`

```rust
struct ElfSectionRelocationIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    section_index: crate::read::SectionIndex,
    file: &'file super::ElfFile<'data, Elf, R>,
    relocations: Option<ElfRelocationIterator<'data, Elf>>,
}
```

An iterator for the relocations for an [`ElfSection`](super::ElfSection).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current pointer in the chain of relocation sections.

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `RelrIterator<'data, Elf: FileHeader>`

```rust
struct RelrIterator<'data, Elf: FileHeader> {
    offset: <Elf as >::Word,
    bits: <Elf as >::Word,
    count: u8,
    iter: slice::Iter<'data, <Elf as >::Relr>,
    endian: <Elf as >::Endian,
}
```

An iterator over the relative relocations in an ELF `SHT_RELR` section.

Returned by [`SectionHeader::relr`](super::SectionHeader::relr).

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [<Elf as >::Relr]) -> Self` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for RelrIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for RelrIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for RelrIterator<'data, Elf>`

- `type Item = <Elf as FileHeader>::Word`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Crel`

```rust
struct Crel {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
    pub r_addend: i64,
}
```

Compact relocation

The specification has been submited here: <https://groups.google.com/g/generic-abi/c/ppkaxtLb0P0/m/awgqZ_1CBAAJ>.

#### Fields

- **`r_offset`**: `u64`

  Relocation offset.

- **`r_sym`**: `u32`

  Relocation symbol index.

- **`r_type`**: `u32`

  Relocation type.

- **`r_addend`**: `i64`

  Relocation addend.
  
  Only set if `CrelIterator::is_rela()` returns `true`.

#### Implementations

- `fn symbol(self: &Self) -> Option<SymbolIndex>` — [`SymbolIndex`](../../index.md)

- `fn from_rel<R: Rel>(r: &R, endian: <R as >::Endian) -> Crel` — [`Rel`](#rel), [`Crel`](#crel)

- `fn from_rela<R: Rela>(r: &R, endian: <R as >::Endian, is_mips64el: bool) -> Crel` — [`Rela`](#rela), [`Crel`](#crel)

#### Trait Implementations

##### `impl Clone for Crel`

- `fn clone(self: &Self) -> Crel` — [`Crel`](#crel)

##### `impl Copy for Crel`

##### `impl Debug for Crel`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CrelIteratorHeader`

```rust
struct CrelIteratorHeader {
    count: usize,
    flag_bits: u64,
    shift: u64,
    is_rela: bool,
}
```

#### Fields

- **`count`**: `usize`

  The number of encoded relocations.

- **`flag_bits`**: `u64`

  The number of flag bits each relocation uses.

- **`shift`**: `u64`

  Shift of the relocation value.

- **`is_rela`**: `bool`

  True if the relocation format encodes addend.

#### Trait Implementations

##### `impl Clone for CrelIteratorHeader`

- `fn clone(self: &Self) -> CrelIteratorHeader` — [`CrelIteratorHeader`](relocation/index.md)

##### `impl Debug for CrelIteratorHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CrelIteratorState`

```rust
struct CrelIteratorState {
    index: usize,
    offset: u64,
    addend: i64,
    symidx: u32,
    typ: u32,
}
```

#### Fields

- **`index`**: `usize`

  Index of the current relocation.

- **`offset`**: `u64`

  Offset of the latest relocation.

- **`addend`**: `i64`

  Addend of the latest relocation.

- **`symidx`**: `u32`

  Symbol index of the latest relocation.

- **`typ`**: `u32`

  Type of the latest relocation.

#### Trait Implementations

##### `impl Clone for CrelIteratorState`

- `fn clone(self: &Self) -> CrelIteratorState` — [`CrelIteratorState`](relocation/index.md)

##### `impl Debug for CrelIteratorState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for CrelIteratorState`

- `fn default() -> CrelIteratorState` — [`CrelIteratorState`](relocation/index.md)

### `CrelIterator<'data>`

```rust
struct CrelIterator<'data> {
    data: crate::read::Bytes<'data>,
    header: CrelIteratorHeader,
    state: CrelIteratorState,
}
```

Compact relocation iterator.

#### Fields

- **`data`**: `crate::read::Bytes<'data>`

  Input stream reader.

- **`header`**: `CrelIteratorHeader`

  Parsed header information.

- **`state`**: `CrelIteratorState`

  State of the iterator.

#### Implementations

- `fn new(data: &'data [u8]) -> Result<Self, Error>` — [`Error`](../../index.md)

- `fn is_rela(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn parse(self: &mut Self) -> read::Result<Crel>` — [`Result`](../../index.md), [`Crel`](#crel)

#### Trait Implementations

##### `impl<'data> Clone for CrelIterator<'data>`

- `fn clone(self: &Self) -> CrelIterator<'data>` — [`CrelIterator`](#creliterator)

##### `impl<'data> Debug for CrelIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for CrelIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for CrelIterator<'data>`

- `type Item = Result<Crel, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `ElfComdatIterator<'data, 'file, Elf, R>`

```rust
struct ElfComdatIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Elf as >::SectionHeader>>,
}
```

An iterator for the COMDAT section groups in an [`ElfFile`](#elffile).

#### Implementations

- `fn new(file: &'file ElfFile<'data, Elf, R>) -> ElfComdatIterator<'data, 'file, Elf, R>` — [`ElfFile`](#elffile), [`ElfComdatIterator`](#elfcomdatiterator)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdatIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ElfComdatIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatIterator<'data, 'file, Elf, R>`

- `type Item = ElfComdat<'data, 'file, Elf, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfComdat<'data, 'file, Elf, R>`

```rust
struct ElfComdat<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    section: &'data <Elf as >::SectionHeader,
    sections: &'data [crate::endian::U32Bytes<<Elf as >::Endian>],
}
```

A COMDAT section group in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Implementations

- `fn parse(file: &'file ElfFile<'data, Elf, R>, section: &'data <Elf as >::SectionHeader) -> Option<ElfComdat<'data, 'file, Elf, R>>` — [`ElfFile`](#elffile), [`FileHeader`](#fileheader), [`ElfComdat`](#elfcomdat)

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

- `fn elf_section_header(self: &Self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdat<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectComdat for ElfComdat<'data, 'file, Elf, R>`

- `type SectionIterator = ElfComdatSectionIterator<'data, 'file, Elf, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfComdat<'data, 'file, Elf, R>`

### `ElfComdatSectionIterator<'data, 'file, Elf, R>`

```rust
struct ElfComdatSectionIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    sections: slice::Iter<'data, crate::endian::U32Bytes<<Elf as >::Endian>>,
}
```

An iterator for the sections in a COMDAT section group in an [`ElfFile`](#elffile).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `NoteIterator<'data, Elf>`

```rust
struct NoteIterator<'data, Elf>
where
    Elf: FileHeader {
    endian: <Elf as >::Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator over the notes in an ELF section or segment.

Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)
and [`SectionHeader::notes`](super::SectionHeader::notes).

#### Implementations

- `fn new(endian: <Elf as >::Endian, align: <Elf as >::Word, data: &'data [u8]) -> read::Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- `fn next(self: &mut Self) -> read::Result<Option<Note<'data, Elf>>>` — [`Result`](../../index.md), [`Note`](#note)

- `fn parse(self: &mut Self) -> read::Result<Note<'data, Elf>>` — [`Result`](../../index.md), [`Note`](#note)

#### Trait Implementations

##### `impl<'data, Elf> Debug for NoteIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for NoteIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for NoteIterator<'data, Elf>`

- `type Item = Result<Note<'data, Elf>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Note<'data, Elf>`

```rust
struct Note<'data, Elf>
where
    Elf: FileHeader {
    header: &'data <Elf as >::NoteHeader,
    name: &'data [u8],
    desc: &'data [u8],
}
```

A parsed [`NoteHeader`](#noteheader).

#### Implementations

- `fn n_type(self: &Self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

- `fn n_namesz(self: &Self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

- `fn n_descsz(self: &Self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

- `fn name_bytes(self: &Self) -> &'data [u8]`

- `fn name(self: &Self) -> &'data [u8]`

- `fn desc(self: &Self) -> &'data [u8]`

- `fn gnu_properties(self: &Self, endian: <Elf as >::Endian) -> Option<GnuPropertyIterator<'data, <Elf as >::Endian>>` — [`FileHeader`](#fileheader), [`GnuPropertyIterator`](#gnupropertyiterator)

#### Trait Implementations

##### `impl<'data, Elf> Debug for Note<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GnuPropertyIterator<'data, Endian: endian::Endian>`

```rust
struct GnuPropertyIterator<'data, Endian: endian::Endian> {
    endian: Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../elf/index.md) note.

Returned by `Note::gnu_properties`.

#### Implementations

- `fn next(self: &mut Self) -> read::Result<Option<GnuProperty<'data>>>` — [`Result`](../../index.md), [`GnuProperty`](#gnuproperty)

- `fn parse(self: &mut Self) -> read::Result<GnuProperty<'data>>` — [`Result`](../../index.md), [`GnuProperty`](#gnuproperty)

#### Trait Implementations

##### `impl<'data, Endian: $crate::fmt::Debug + endian::Endian> Debug for GnuPropertyIterator<'data, Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for GnuPropertyIterator<'data, Endian>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Endian: endian::Endian> Iterator for GnuPropertyIterator<'data, Endian>`

- `type Item = Result<GnuProperty<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `GnuProperty<'data>`

```rust
struct GnuProperty<'data> {
    pr_type: u32,
    pr_data: &'data [u8],
}
```

A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../elf/index.md) note.

#### Implementations

- `fn pr_type(self: &Self) -> u32`

- `fn pr_data(self: &Self) -> &'data [u8]`

- `fn data_u32<E: endian::Endian>(self: &Self, endian: E) -> read::Result<u32>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Debug for GnuProperty<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HashTable<'data, Elf: FileHeader>`

```rust
struct HashTable<'data, Elf: FileHeader> {
    buckets: &'data [crate::endian::U32<<Elf as >::Endian>],
    chains: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A SysV symbol hash table in an ELF file.

Returned by [`SectionHeader::hash`](super::SectionHeader::hash).

#### Implementations

- `fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- `fn symbol_table_length(self: &Self) -> u32`

- `fn bucket(self: &Self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- `fn chain(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- `fn find<R: ReadRef<'data>>(self: &Self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](#fileheader), [`Version`](#version), [`SymbolTable`](#symboltable), [`VersionTable`](#versiontable), [`SymbolIndex`](../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for HashTable<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GnuHashTable<'data, Elf: FileHeader>`

```rust
struct GnuHashTable<'data, Elf: FileHeader> {
    symbol_base: u32,
    bloom_shift: u32,
    bloom_filters: &'data [u8],
    buckets: &'data [crate::endian::U32<<Elf as >::Endian>],
    values: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A GNU symbol hash table in an ELF file.

Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash).

#### Implementations

- `fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- `fn symbol_base(self: &Self) -> u32`

- `fn symbol_table_length(self: &Self, endian: <Elf as >::Endian) -> Option<u32>` — [`FileHeader`](#fileheader)

- `fn bucket(self: &Self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- `fn find<R: ReadRef<'data>>(self: &Self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](#fileheader), [`Version`](#version), [`SymbolTable`](#symboltable), [`VersionTable`](#versiontable), [`SymbolIndex`](../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for GnuHashTable<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `VersionIndex`

```rust
struct VersionIndex(u16);
```

A version index.

#### Implementations

- `fn index(self: &Self) -> u16`

- `fn is_local(self: &Self) -> bool`

- `fn is_global(self: &Self) -> bool`

- `fn is_hidden(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for VersionIndex`

- `fn clone(self: &Self) -> VersionIndex` — [`VersionIndex`](#versionindex)

##### `impl Copy for VersionIndex`

##### `impl Debug for VersionIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for VersionIndex`

- `fn default() -> VersionIndex` — [`VersionIndex`](#versionindex)

### `Version<'data>`

```rust
struct Version<'data> {
    name: &'data [u8],
    hash: u32,
    valid: bool,
    file: Option<&'data [u8]>,
}
```

A version definition or requirement.

This is derived from entries in the [`elf::SHT_GNU_VERDEF`](../../elf/index.md) and [`elf::SHT_GNU_VERNEED`](../../elf/index.md) sections.

#### Implementations

- `fn name(self: &Self) -> &'data [u8]`

- `fn hash(self: &Self) -> u32`

- `fn file(self: &Self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for Version<'data>`

- `fn clone(self: &Self) -> Version<'data>` — [`Version`](#version)

##### `impl<'data> Copy for Version<'data>`

##### `impl<'data> Debug for Version<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for Version<'data>`

- `fn default() -> Version<'data>` — [`Version`](#version)

### `VersionTable<'data, Elf: FileHeader>`

```rust
struct VersionTable<'data, Elf: FileHeader> {
    symbols: &'data [elf::Versym<<Elf as >::Endian>],
    versions: alloc::vec::Vec<Version<'data>>,
}
```

A table of version definitions and requirements.

It allows looking up the version information for a given symbol index.

This is derived from entries in the [`elf::SHT_GNU_VERSYM`](../../elf/index.md), [`elf::SHT_GNU_VERDEF`](../../elf/index.md)
and [`elf::SHT_GNU_VERNEED`](../../elf/index.md) sections.

Returned by [`SectionTable::versions`](super::SectionTable::versions).

#### Implementations

- `fn parse<R: ReadRef<'data>>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` — [`FileHeader`](#fileheader), [`Versym`](../../elf/index.md), [`VerdefIterator`](#verdefiterator), [`VerneedIterator`](#verneediterator), [`StringTable`](../index.md), [`Result`](../../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn version_index(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md), [`VersionIndex`](#versionindex)

- `fn version(self: &Self, index: VersionIndex) -> Result<Option<&Version<'data>>>` — [`VersionIndex`](#versionindex), [`Result`](../../index.md), [`Version`](#version)

- `fn matches(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version<'_>>) -> bool` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md), [`Version`](#version)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VersionTable<'data, Elf>`

- `fn clone(self: &Self) -> VersionTable<'data, Elf>` — [`VersionTable`](#versiontable)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VersionTable<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Elf: FileHeader> Default for VersionTable<'data, Elf>`

- `fn default() -> Self`

### `VerdefIterator<'data, Elf: FileHeader>`

```rust
struct VerdefIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`](../../elf/index.md) section.

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](#fileheader)

- `fn next(self: &mut Self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` — [`Result`](../../index.md), [`Verdef`](../../elf/index.md), [`FileHeader`](#fileheader), [`VerdauxIterator`](#verdauxiterator)

- `fn parse(self: &mut Self) -> Result<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>` — [`Result`](../../index.md), [`Verdef`](../../elf/index.md), [`FileHeader`](#fileheader), [`VerdauxIterator`](#verdauxiterator)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VerdefIterator<'data, Elf>`

- `fn clone(self: &Self) -> VerdefIterator<'data, Elf>` — [`VerdefIterator`](#verdefiterator)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VerdefIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VerdefIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdefIterator<'data, Elf>`

- `type Item = Result<(&'data Verdef<<Elf as FileHeader>::Endian>, VerdauxIterator<'data, Elf>), Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `VerdauxIterator<'data, Elf: FileHeader>`

```rust
struct VerdauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`](../../elf/index.md) section.

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](#fileheader)

- `fn next(self: &mut Self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` — [`Result`](../../index.md), [`Verdaux`](../../elf/index.md), [`FileHeader`](#fileheader)

- `fn parse(self: &mut Self) -> Result<&'data elf::Verdaux<<Elf as >::Endian>>` — [`Result`](../../index.md), [`Verdaux`](../../elf/index.md), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VerdauxIterator<'data, Elf>`

- `fn clone(self: &Self) -> VerdauxIterator<'data, Elf>` — [`VerdauxIterator`](#verdauxiterator)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VerdauxIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VerdauxIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdauxIterator<'data, Elf>`

- `type Item = Result<&'data Verdaux<<Elf as FileHeader>::Endian>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `VerneedIterator<'data, Elf: FileHeader>`

```rust
struct VerneedIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`](../../elf/index.md) section.

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](#fileheader)

- `fn next(self: &mut Self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` — [`Result`](../../index.md), [`Verneed`](../../elf/index.md), [`FileHeader`](#fileheader), [`VernauxIterator`](#vernauxiterator)

- `fn parse(self: &mut Self) -> Result<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>` — [`Result`](../../index.md), [`Verneed`](../../elf/index.md), [`FileHeader`](#fileheader), [`VernauxIterator`](#vernauxiterator)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VerneedIterator<'data, Elf>`

- `fn clone(self: &Self) -> VerneedIterator<'data, Elf>` — [`VerneedIterator`](#verneediterator)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VerneedIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VerneedIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerneedIterator<'data, Elf>`

- `type Item = Result<(&'data Verneed<<Elf as FileHeader>::Endian>, VernauxIterator<'data, Elf>), Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `VernauxIterator<'data, Elf: FileHeader>`

```rust
struct VernauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`](../../elf/index.md) section.

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](#fileheader)

- `fn next(self: &mut Self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` — [`Result`](../../index.md), [`Vernaux`](../../elf/index.md), [`FileHeader`](#fileheader)

- `fn parse(self: &mut Self) -> Result<&'data elf::Vernaux<<Elf as >::Endian>>` — [`Result`](../../index.md), [`Vernaux`](../../elf/index.md), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VernauxIterator<'data, Elf>`

- `fn clone(self: &Self) -> VernauxIterator<'data, Elf>` — [`VernauxIterator`](#vernauxiterator)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VernauxIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VernauxIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VernauxIterator<'data, Elf>`

- `type Item = Result<&'data Vernaux<<Elf as FileHeader>::Endian>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `AttributesSection<'data, Elf: FileHeader>`

```rust
struct AttributesSection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    version: u8,
    data: crate::read::Bytes<'data>,
}
```

An ELF attributes section.

This may be a GNU attributes section, or an architecture specific attributes section.

An attributes section contains a series of [`AttributesSubsection`](#attributessubsection).

Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)
and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes).

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- `fn version(self: &Self) -> u8`

- `fn subsections(self: &Self) -> Result<AttributesSubsectionIterator<'data, Elf>>` — [`Result`](../../index.md), [`AttributesSubsectionIterator`](#attributessubsectioniterator)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSection<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSection<'data, Elf>` — [`AttributesSection`](#attributessection)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSection<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AttributesSubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the subsections in an [`AttributesSection`](#attributessection).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<AttributesSubsection<'data, Elf>>>` — [`Result`](../../index.md), [`AttributesSubsection`](#attributessubsection)

- `fn parse(self: &mut Self) -> Result<AttributesSubsection<'data, Elf>>` — [`Result`](../../index.md), [`AttributesSubsection`](#attributessubsection)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSubsectionIterator<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSubsectionIterator<'data, Elf>` — [`AttributesSubsectionIterator`](#attributessubsectioniterator)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSubsectionIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for AttributesSubsectionIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for AttributesSubsectionIterator<'data, Elf>`

- `type Item = Result<AttributesSubsection<'data, Elf>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `AttributesSubsection<'data, Elf: FileHeader>`

```rust
struct AttributesSubsection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    length: u32,
    vendor: &'data [u8],
    data: crate::read::Bytes<'data>,
}
```

A subsection in an [`AttributesSection`](#attributessection).

A subsection is identified by a vendor name.  It contains a series of
[`AttributesSubsubsection`](#attributessubsubsection).

#### Implementations

- `fn length(self: &Self) -> u32`

- `fn vendor(self: &Self) -> &'data [u8]`

- `fn subsubsections(self: &Self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSubsection<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSubsection<'data, Elf>` — [`AttributesSubsection`](#attributessubsection)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSubsection<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AttributesSubsubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the sub-subsections in an [`AttributesSubsection`](#attributessubsection).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<AttributesSubsubsection<'data>>>` — [`Result`](../../index.md), [`AttributesSubsubsection`](#attributessubsubsection)

- `fn parse(self: &mut Self) -> Result<AttributesSubsubsection<'data>>` — [`Result`](../../index.md), [`AttributesSubsubsection`](#attributessubsubsection)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSubsubsectionIterator<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSubsubsectionIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for AttributesSubsubsectionIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for AttributesSubsubsectionIterator<'data, Elf>`

- `type Item = Result<AttributesSubsubsection<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `AttributesSubsubsection<'data>`

```rust
struct AttributesSubsubsection<'data> {
    tag: u8,
    length: u32,
    indices: crate::read::Bytes<'data>,
    data: crate::read::Bytes<'data>,
}
```

A sub-subsection in an [`AttributesSubsection`](#attributessubsection).

A sub-subsection is identified by a tag.  It contains an optional series of indices,
followed by a series of attributes.

#### Implementations

- `fn tag(self: &Self) -> u8`

- `fn length(self: &Self) -> u32`

- `fn indices_data(self: &Self) -> &'data [u8]`

- `fn indices(self: &Self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](#attributeindexiterator)

- `fn attributes_data(self: &Self) -> &'data [u8]`

- `fn attributes(self: &Self) -> AttributeReader<'data>` — [`AttributeReader`](#attributereader)

#### Trait Implementations

##### `impl<'data> Clone for AttributesSubsubsection<'data>`

- `fn clone(self: &Self) -> AttributesSubsubsection<'data>` — [`AttributesSubsubsection`](#attributessubsubsection)

##### `impl<'data> Debug for AttributesSubsubsection<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AttributeIndexIterator<'data>`

```rust
struct AttributeIndexIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

An iterator over the indices in an [`AttributesSubsubsection`](#attributessubsubsection).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<u32>>` — [`Result`](../../index.md)

- `fn parse(self: &mut Self) -> Result<u32>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for AttributeIndexIterator<'data>`

- `fn clone(self: &Self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](#attributeindexiterator)

##### `impl<'data> Debug for AttributeIndexIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for AttributeIndexIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for AttributeIndexIterator<'data>`

- `type Item = Result<u32, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `AttributeReader<'data>`

```rust
struct AttributeReader<'data> {
    data: crate::read::Bytes<'data>,
}
```

A parser for the attributes in an [`AttributesSubsubsection`](#attributessubsubsection).

The parser relies on the caller to know the format of the data for each attribute tag.

#### Implementations

- `fn read_tag(self: &mut Self) -> Result<Option<u64>>` — [`Result`](../../index.md)

- `fn read_integer(self: &mut Self) -> Result<u64>` — [`Result`](../../index.md)

- `fn read_string(self: &mut Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for AttributeReader<'data>`

- `fn clone(self: &Self) -> AttributeReader<'data>` — [`AttributeReader`](#attributereader)

##### `impl<'data> Debug for AttributeReader<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `ElfRelocationIterator<'data, Elf: FileHeader>`

```rust
enum ElfRelocationIterator<'data, Elf: FileHeader> {
    Rel(slice::Iter<'data, <Elf as >::Rel>, <Elf as >::Endian),
    Rela(slice::Iter<'data, <Elf as >::Rela>, <Elf as >::Endian, bool),
    Crel(CrelIterator<'data>),
}
```

#### Implementations

- `fn is_rel(self: &Self) -> bool`

#### Trait Implementations

##### `impl<I> IntoIterator for ElfRelocationIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for ElfRelocationIterator<'data, Elf>`

- `type Item = Crel`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::FileHeader32`](../../elf/index.md) and [`elf::FileHeader64`](../../elf/index.md).

#### Required Methods

- `type Word: 3`

- `type Sword: 1`

- `type Endian: 1`

- `type ProgramHeader: 1`

- `type SectionHeader: 1`

- `type CompressionHeader: 1`

- `type NoteHeader: 1`

- `type Dyn: 1`

- `type Sym: 1`

- `type Rel: 1`

- `type Rela: 2`

- `type Relr: 1`

- `fn is_type_64(self: &Self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_type_64_sized() -> bool`

  Return true if this type is a 64-bit header.

- `fn e_ident(self: &Self) -> &elf::Ident`

- `fn e_type(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_machine(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_version(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_entry(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_phoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_shoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_ehsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shstrndx(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(self: &Self) -> bool`

  Check that the ident field in the file header is a supported format.

- `fn is_class_32(self: &Self) -> bool`

- `fn is_class_64(self: &Self) -> bool`

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_big_endian(self: &Self) -> bool`

- `fn endian(self: &Self) -> read::Result<<Self as >::Endian>`

- `fn section_0<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data <Self as >::SectionHeader>>`

  Return the first section header, if present.

- `fn phnum<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<usize>`

  Return the `e_phnum` field of the header. Handles extended values.

- `fn shnum<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<usize>`

  Return the `e_shnum` field of the header. Handles extended values.

- `fn shstrndx<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<u32>`

  Return the `e_shstrndx` field of the header. Handles extended values.

- `fn program_headers<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [<Self as >::ProgramHeader]>`

  Return the slice of program headers.

- `fn section_headers<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [<Self as >::SectionHeader]>`

  Return the slice of section headers.

- `fn section_strings_index<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<SectionIndex>`

  Get the section index of the section header string table.

- `fn section_strings<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, sections: &[<Self as >::SectionHeader]) -> read::Result<StringTable<'data, R>>`

  Return the string table for the section headers.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<SectionTable<'data, Self, R>>`

  Return the section table.

- `fn is_mips64el(self: &Self, endian: <Self as >::Endian) -> bool`

  Returns whether this is a mips64el elf file.

### `ProgramHeader`

```rust
trait ProgramHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::ProgramHeader32`](../../elf/index.md) and [`elf::ProgramHeader64`](../../elf/index.md).

#### Required Methods

- `type Elf: 1`

- `type Word: 1`

- `type Endian: 1`

- `fn p_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_vaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_paddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_filesz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_memsz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_align(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn file_range(self: &Self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> Result<&'data [u8], ()>`

  Return the segment data.

- `fn data_as_array<'data, T: Pod, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> Result<&'data [T], ()>`

  Return the segment data as a slice of the given type.

- `fn data_range<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, address: u64, size: u64) -> Result<Option<&'data [u8]>, ()>`

  Return the segment data in the given virtual address range

- `fn dynamic<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [<<Self as >::Elf as FileHeader>::Dyn]>>`

  Return entries in a dynamic segment.

- `fn interpreter<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [u8]>>`

  Return the data in an interpreter segment.

- `fn notes<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<NoteIterator<'data, <Self as >::Elf>>>`

  Return a note iterator for the segment data.

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::SectionHeader32`](../../elf/index.md) and [`elf::SectionHeader64`](../../elf/index.md).

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

### `Sym`

```rust
trait Sym: Debug + Pod { ... }
```

A trait for generic access to [`elf::Sym32`](../../elf/index.md) and [`elf::Sym64`](../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn st_name(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn st_info(self: &Self) -> u8`

- `fn st_bind(self: &Self) -> u8`

- `fn st_type(self: &Self) -> u8`

- `fn st_other(self: &Self) -> u8`

- `fn st_visibility(self: &Self) -> u8`

- `fn st_shndx(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn st_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn st_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn name<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> read::Result<&'data [u8]>`

  Parse the symbol name from the string table.

- `fn is_undefined(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_UNDEF`.

- `fn is_definition(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn is_common(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_COMMON`.

- `fn is_absolute(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_ABS`.

- `fn is_local(self: &Self) -> bool`

  Return true if the symbol binding is `STB_LOCAL`.

- `fn is_weak(self: &Self) -> bool`

  Return true if the symbol binding is `STB_WEAK`.

### `Rel`

```rust
trait Rel: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Rel32`](../../elf/index.md) and [`elf::Rel64`](../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_sym(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn symbol(self: &Self, endian: <Self as >::Endian) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

### `Rela`

```rust
trait Rela: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Rela32`](../../elf/index.md) and [`elf::Rela64`](../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word`

- `fn r_addend(self: &Self, endian: <Self as >::Endian) -> <Self as >::Sword`

- `fn r_sym(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn symbol(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

### `Relr`

```rust
trait Relr: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Relr32`](../../elf/index.md) and [`elf::Relr64`](../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `const COUNT: u8`

- `fn get(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

  Get the relocation entry.

- `fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>`

  Return the offset corresponding to the next bit in the bit mask.

### `Dyn`

```rust
trait Dyn: Debug + Pod { ... }
```

A trait for generic access to [`elf::Dyn32`](../../elf/index.md) and [`elf::Dyn64`](../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn d_tag(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn d_val(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn tag32(self: &Self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the tag to a `u32`.

- `fn val32(self: &Self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the value to a `u32`.

- `fn is_string(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an offset in the dynamic string table.

- `fn string<'data>(self: &Self, endian: <Self as >::Endian, strings: StringTable<'data>) -> Result<&'data [u8]>`

  Use the value to get a string in a string table.

- `fn is_address(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an address.

### `CompressionHeader`

```rust
trait CompressionHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::CompressionHeader32`](../../elf/index.md) and [`elf::CompressionHeader64`](../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn ch_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn ch_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn ch_addralign(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

### `NoteHeader`

```rust
trait NoteHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::NoteHeader32`](../../elf/index.md) and [`elf::NoteHeader64`](../../elf/index.md).

#### Required Methods

- `type Endian: 1`

- `fn n_namesz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_type(self: &Self, endian: <Self as >::Endian) -> u32`

## Functions

### `parse_relocation`

```rust
fn parse_relocation<Elf: FileHeader>(header: &Elf, endian: <Elf as >::Endian, reloc: Crel, implicit_addend: bool) -> crate::read::Relocation
```

## Type Aliases

### `ElfFile32<'data, Endian, R>`

```rust
type ElfFile32<'data, Endian, R> = ElfFile<'data, elf::FileHeader32<Endian>, R>;
```

A 32-bit ELF object file.

This is a file that starts with [`elf::FileHeader32`](../../elf/index.md), and corresponds
to [`crate::FileKind::Elf32`](../../index.md).

### `ElfFile64<'data, Endian, R>`

```rust
type ElfFile64<'data, Endian, R> = ElfFile<'data, elf::FileHeader64<Endian>, R>;
```

A 64-bit ELF object file.

This is a file that starts with [`elf::FileHeader64`](../../elf/index.md), and corresponds
to [`crate::FileKind::Elf64`](../../index.md).

### `ElfSegmentIterator32<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator32<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the segments in an [`ElfFile32`](super::ElfFile32).

### `ElfSegmentIterator64<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator64<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the segments in an [`ElfFile64`](super::ElfFile64).

### `ElfSegment32<'data, 'file, Endian, R>`

```rust
type ElfSegment32<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A segment in an [`ElfFile32`](super::ElfFile32).

### `ElfSegment64<'data, 'file, Endian, R>`

```rust
type ElfSegment64<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A segment in an [`ElfFile64`](super::ElfFile64).

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

### `ElfSymbolTable32<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable32<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolTable64<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable64<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator32<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator32<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator64<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator64<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile64`](super::ElfFile64).

### `ElfSymbol32<'data, 'file, Endian, R>`

```rust
type ElfSymbol32<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbol64<'data, 'file, Endian, R>`

```rust
type ElfSymbol64<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol in an [`ElfFile64`](super::ElfFile64).

### `ElfDynamicRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator32<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32).

### `ElfDynamicRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator64<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64).

### `ElfSectionRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator32<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the relocations for an [`ElfSection32`](super::ElfSection32).

### `ElfSectionRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator64<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the relocations for an [`ElfSection64`](super::ElfSection64).

### `ElfComdatIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator32<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator64<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64).

### `ElfComdat32<'data, 'file, Endian, R>`

```rust
type ElfComdat32<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdat64<'data, 'file, Endian, R>`

```rust
type ElfComdat64<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A COMDAT section group in an [`ElfFile64`](super::ElfFile64).

### `ElfComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator32<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator64<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64).

