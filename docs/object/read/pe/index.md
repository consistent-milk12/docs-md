*[object](../../index.md) / [read](../index.md) / [pe](index.md)*

---

# Module `pe`

Support for reading PE files.

Traits are used to abstract over the difference between PE32 and PE32+.
The primary trait for this is [`ImageNtHeaders`](#imagentheaders).

## High level API

[`PeFile`](#pefile) implements the [`Object`](crate::read::Object) trait for
PE files. [`PeFile`](#pefile) is parameterised by [`ImageNtHeaders`](#imagentheaders) to allow
reading both PE32 and PE32+. There are type aliases for these parameters
([`PeFile32`](#pefile32) and [`PeFile64`](#pefile64)).

## Low level API

The [`ImageNtHeaders`](#imagentheaders) trait can be directly used to parse both
[`pe::ImageNtHeaders32`](../../pe/index.md) and [`pe::ImageNtHeaders64`](../../pe/index.md).

### Example for low level API
 ```no_run
use object::pe;
use object::read::pe::ImageNtHeaders;
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let dos_header = pe::ImageDosHeader::parse(&*data)?;
    let mut offset = dos_header.nt_headers_offset().into();
    let (nt_headers, data_directories) = pe::ImageNtHeaders64::parse(&*data, &mut offset)?;
    let sections = nt_headers.sections(&*data, offset)?;
    let symbols = nt_headers.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
    }
  }
    Ok(())
}
```

## Modules

- [`file`](file/index.md) - 
- [`section`](section/index.md) - 
- [`data_directory`](data_directory/index.md) - 
- [`export`](export/index.md) - 
- [`import`](import/index.md) - 
- [`relocation`](relocation/index.md) - 
- [`resource`](resource/index.md) - 
- [`rich`](rich/index.md) - PE rich header handling

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

- `fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../pe/index.md)

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../index.md), [`ImageSectionHeader`](../../pe/index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`ImageSectionHeader`](../../pe/index.md)

- `fn section_by_name<R: ReadRef<'data>>(self: &Self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../index.md), [`SectionIndex`](../../index.md), [`ImageSectionHeader`](../../pe/index.md)

- `fn max_section_file_offset(self: &Self) -> u64`

#### Trait Implementations

##### `impl<'data> Clone for SectionTable<'data>`

- `fn clone(self: &Self) -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

##### `impl<'data> Copy for SectionTable<'data>`

##### `impl<'data> Debug for SectionTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for SectionTable<'data>`

- `fn default() -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

### `SymbolTable<'data, R, Coff>`

```rust
struct SymbolTable<'data, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    symbols: &'data [<Coff as >::ImageSymbolBytes],
    strings: crate::read::util::StringTable<'data, R>,
}
```

A table of symbol entries in a COFF or PE file.

Also includes the string table used for the symbol names.

Returned by `CoffHeader::symbols` and
[`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols).

#### Implementations

- `fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](../coff/index.md)

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`CoffHeader`](../coff/index.md)

- `fn aux_function(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ImageAuxSymbolFunction`](../../pe/index.md)

- `fn aux_section(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ImageAuxSymbolSection`](../../pe/index.md)

- `fn aux_weak_external(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ImageAuxSymbolWeak`](../../pe/index.md)

- `fn aux_file_name(self: &Self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md)

- `fn get<T: Pod>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md)

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(self: &Self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../index.md)

#### Trait Implementations

##### `impl<'data, R, Coff> Debug for SymbolTable<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R: ReadRef<'data>, Coff: CoffHeader> Default for SymbolTable<'data, R, Coff>`

- `fn default() -> Self`

### `PeFile<'data, Pe, R>`

```rust
struct PeFile<'data, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    dos_header: &'data pe::ImageDosHeader,
    nt_headers: &'data Pe,
    data_directories: super::DataDirectories<'data>,
    common: crate::read::coff::CoffCommon<'data, R>,
    data: R,
}
```

A PE image file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn data(self: &Self) -> R`

- `fn dos_header(self: &Self) -> &'data pe::ImageDosHeader` — [`ImageDosHeader`](../../pe/index.md)

- `fn nt_headers(self: &Self) -> &'data Pe`

- `fn rich_header_info(self: &Self) -> Option<RichHeaderInfo<'_>>` — [`RichHeaderInfo`](#richheaderinfo)

- `fn section_table(self: &Self) -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

- `fn data_directories(self: &Self) -> DataDirectories<'data>` — [`DataDirectories`](#datadirectories)

- `fn data_directory(self: &Self, id: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md)

- `fn export_table(self: &Self) -> Result<Option<ExportTable<'data>>>` — [`Result`](../../index.md), [`ExportTable`](#exporttable)

- `fn import_table(self: &Self) -> Result<Option<ImportTable<'data>>>` — [`Result`](../../index.md), [`ImportTable`](#importtable)

- `fn section_alignment(self: &Self) -> u64`

#### Trait Implementations

##### `impl<'data, Pe, R> Debug for PeFile<'data, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Pe, R> Object for PeFile<'data, Pe, R>`

- `type Segment = PeSegment<'data, 'file, Pe, R>`

- `type SegmentIterator = PeSegmentIterator<'data, 'file, Pe, R>`

- `type Section = PeSection<'data, 'file, Pe, R>`

- `type SectionIterator = PeSectionIterator<'data, 'file, Pe, R>`

- `type Comdat = PeComdat<'data, 'file, Pe, R>`

- `type ComdatIterator = PeComdatIterator<'data, 'file, Pe, R>`

- `type Symbol = CoffSymbol<'data, 'file, R>`

- `type SymbolIterator = CoffSymbolIterator<'data, 'file, R>`

- `type SymbolTable = CoffSymbolTable<'data, 'file, R>`

- `type DynamicRelocationIterator = NoDynamicRelocationIterator`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- `fn segments(self: &Self) -> PeSegmentIterator<'data, '_, Pe, R>` — [`PeSegmentIterator`](#pesegmentiterator)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<PeSection<'data, 'file, Pe, R>>` — [`PeSection`](#pesection)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<PeSection<'data, '_, Pe, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`PeSection`](#pesection)

- `fn sections(self: &Self) -> PeSectionIterator<'data, '_, Pe, R>` — [`PeSectionIterator`](#pesectioniterator)

- `fn comdats(self: &Self) -> PeComdatIterator<'data, '_, Pe, R>` — [`PeComdatIterator`](#pecomdatiterator)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`CoffSymbol`](../coff/index.md)

- `fn symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../coff/index.md)

- `fn symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../coff/index.md)

- `fn dynamic_symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../coff/index.md)

- `fn dynamic_symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../coff/index.md)

- `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- `fn pdb_info(self: &Self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../index.md), [`CodeView`](../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, Pe, R> Sealed for PeFile<'data, Pe, R>`

### `PeComdatIterator<'data, 'file, Pe, R>`

```rust
struct PeComdatIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

An iterator for the COMDAT section groups in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdatIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeComdatIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeComdatIterator<'data, 'file, Pe, R>`

- `type Item = PeComdat<'data, 'file, Pe, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `PeComdat<'data, 'file, Pe, R>`

```rust
struct PeComdat<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

A COMDAT section group in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdat<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectComdat for PeComdat<'data, 'file, Pe, R>`

- `type SectionIterator = PeComdatSectionIterator<'data, 'file, Pe, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeComdat<'data, 'file, Pe, R>`

### `PeComdatSectionIterator<'data, 'file, Pe, R>`

```rust
struct PeComdatSectionIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdatSectionIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

An iterator for the loadable sections in a [`PeFile`](#pefile).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegmentIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeSegmentIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSegmentIterator<'data, 'file, Pe, R>`

- `type Item = PeSegment<'data, 'file, Pe, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A loadable section in a [`PeFile`](#pefile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- `fn pe_file(self: &Self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](#pefile)

- `fn pe_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegment<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSegment for PeSegment<'data, 'file, Pe, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

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

An iterator for the sections in a [`PeFile`](#pefile).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSectionIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeSectionIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSectionIterator<'data, 'file, Pe, R>`

- `type Item = PeSection<'data, 'file, Pe, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A section in a [`PeFile`](#pefile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- `fn pe_file(self: &Self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](#pefile)

- `fn pe_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSection<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSection for PeSection<'data, 'file, Pe, R>`

- `type RelocationIterator = PeRelocationIterator<'data, 'file, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../index.md)

- `fn relocations(self: &Self) -> PeRelocationIterator<'data, 'file, R>` — [`PeRelocationIterator`](#perelocationiterator)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeSection<'data, 'file, Pe, R>`

### `PeRelocationIterator<'data, 'file, R>`

```rust
struct PeRelocationIterator<'data, 'file, R>(core::marker::PhantomData<(&'data (), &'file (), R)>);
```

An iterator for the relocations in an [`PeSection`](#pesection).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug> Debug for PeRelocationIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeRelocationIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R> Iterator for PeRelocationIterator<'data, 'file, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `DataDirectories<'data>`

```rust
struct DataDirectories<'data> {
    entries: &'data [pe::ImageDataDirectory],
}
```

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

#### Implementations

- `fn parse(data: &'data [u8], number: u32) -> Result<Self>` — [`Result`](../../index.md)

- `fn len(self: &Self) -> usize`

- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md)

- `fn enumerate(self: &Self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` — [`ImageDataDirectory`](../../pe/index.md)

- `fn get(self: &Self, index: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md)

- `fn export_directory<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` — [`SectionTable`](#sectiontable), [`Result`](../../index.md), [`ImageExportDirectory`](../../pe/index.md)

- `fn export_table<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` — [`SectionTable`](#sectiontable), [`Result`](../../index.md), [`ExportTable`](#exporttable)

- `fn import_table<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` — [`SectionTable`](#sectiontable), [`Result`](../../index.md), [`ImportTable`](#importtable)

- `fn delay_load_import_table<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` — [`SectionTable`](#sectiontable), [`Result`](../../index.md), [`DelayLoadImportTable`](#delayloadimporttable)

- `fn relocation_blocks<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` — [`SectionTable`](#sectiontable), [`Result`](../../index.md), [`RelocationBlockIterator`](#relocationblockiterator)

- `fn resource_directory<R: ReadRef<'data>>(self: &Self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` — [`SectionTable`](#sectiontable), [`Result`](../../index.md), [`ResourceDirectory`](#resourcedirectory)

#### Trait Implementations

##### `impl<'data> Clone for DataDirectories<'data>`

- `fn clone(self: &Self) -> DataDirectories<'data>` — [`DataDirectories`](#datadirectories)

##### `impl<'data> Copy for DataDirectories<'data>`

##### `impl<'data> Debug for DataDirectories<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Export<'data>`

```rust
struct Export<'data> {
    pub ordinal: u32,
    pub name: Option<&'data [u8]>,
    pub target: ExportTarget<'data>,
}
```

An export from a PE file.

There are multiple kinds of PE exports (with or without a name, and local or forwarded).

#### Fields

- **`ordinal`**: `u32`

  The ordinal of the export.
  
  These are sequential, starting at a base specified in the DLL.

- **`name`**: `Option<&'data [u8]>`

  The name of the export, if known.

- **`target`**: `ExportTarget<'data>`

  The target of this export.

#### Trait Implementations

##### `impl<'data> Clone for Export<'data>`

- `fn clone(self: &Self) -> Export<'data>` — [`Export`](#export)

##### `impl<'data> Copy for Export<'data>`

##### `impl<'a> Debug for Export<'a>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

### `ExportTable<'data>`

```rust
struct ExportTable<'data> {
    data: crate::read::Bytes<'data>,
    virtual_address: u32,
    directory: &'data pe::ImageExportDirectory,
    addresses: &'data [crate::endian::U32Bytes<crate::endian::LittleEndian>],
    names: &'data [crate::endian::U32Bytes<crate::endian::LittleEndian>],
    name_ordinals: &'data [crate::endian::U16Bytes<crate::endian::LittleEndian>],
}
```

A partially parsed PE export table.

Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table).

#### Implementations

- `fn parse(data: &'data [u8], virtual_address: u32) -> Result<Self>` — [`Result`](../../index.md)

- `fn parse_directory(data: &'data [u8]) -> Result<&'data pe::ImageExportDirectory>` — [`Result`](../../index.md), [`ImageExportDirectory`](../../pe/index.md)

- `fn directory(self: &Self) -> &'data pe::ImageExportDirectory` — [`ImageExportDirectory`](../../pe/index.md)

- `fn ordinal_base(self: &Self) -> u32`

- `fn addresses(self: &Self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- `fn name_pointers(self: &Self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- `fn name_ordinals(self: &Self) -> &'data [U16Bytes<LE>]` — [`U16Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- `fn name_iter(self: &Self) -> impl Iterator<Item = (u32, u16)> + 'data`

- `fn address_by_index(self: &Self, index: u32) -> Result<u32>` — [`Result`](../../index.md)

- `fn address_by_ordinal(self: &Self, ordinal: u32) -> Result<u32>` — [`Result`](../../index.md)

- `fn target_by_index(self: &Self, index: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md), [`ExportTarget`](#exporttarget)

- `fn target_by_ordinal(self: &Self, ordinal: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md), [`ExportTarget`](#exporttarget)

- `fn target_from_address(self: &Self, address: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md), [`ExportTarget`](#exporttarget)

- `fn forward_offset(self: &Self, address: u32) -> Option<usize>`

- `fn is_forward(self: &Self, address: u32) -> bool`

- `fn forward_string(self: &Self, address: u32) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn name_from_pointer(self: &Self, name_pointer: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](#export)

#### Trait Implementations

##### `impl<'data> Clone for ExportTable<'data>`

- `fn clone(self: &Self) -> ExportTable<'data>` — [`ExportTable`](#exporttable)

##### `impl<'data> Debug for ExportTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ImportTable<'data>`

```rust
struct ImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

Information for parsing a PE import table.

Returned by [`DataDirectories::import_table`](super::DataDirectories::import_table).

#### Implementations

- `fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- `fn descriptors(self: &Self) -> Result<ImportDescriptorIterator<'data>>` — [`Result`](../../index.md), [`ImportDescriptorIterator`](#importdescriptoriterator)

- `fn name(self: &Self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn thunks(self: &Self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../index.md), [`ImportThunkList`](#importthunklist)

- `fn import<Pe: ImageNtHeaders>(self: &Self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](#imagentheaders), [`Result`](../../index.md), [`Import`](#import)

- `fn hint_name(self: &Self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ImportTable<'data>`

- `fn clone(self: &Self) -> ImportTable<'data>` — [`ImportTable`](#importtable)

##### `impl<'data> Debug for ImportTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ImportDescriptorIterator<'data>`

```rust
struct ImportDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

A fallible iterator for the descriptors in the import data directory.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<&'data pe::ImageImportDescriptor>>` — [`Result`](../../index.md), [`ImageImportDescriptor`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data> Clone for ImportDescriptorIterator<'data>`

- `fn clone(self: &Self) -> ImportDescriptorIterator<'data>` — [`ImportDescriptorIterator`](#importdescriptoriterator)

##### `impl<'data> Debug for ImportDescriptorIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ImportDescriptorIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for ImportDescriptorIterator<'data>`

- `type Item = Result<&'data ImageImportDescriptor, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ImportThunkList<'data>`

```rust
struct ImportThunkList<'data> {
    data: crate::read::Bytes<'data>,
}
```

A list of import thunks.

These may be in the import lookup table, or the import address table.

#### Implementations

- `fn get<Pe: ImageNtHeaders>(self: &Self, index: usize) -> Result<<Pe as >::ImageThunkData>` — [`Result`](../../index.md), [`ImageNtHeaders`](#imagentheaders)

- `fn next<Pe: ImageNtHeaders>(self: &mut Self) -> Result<Option<<Pe as >::ImageThunkData>>` — [`Result`](../../index.md), [`ImageNtHeaders`](#imagentheaders)

#### Trait Implementations

##### `impl<'data> Clone for ImportThunkList<'data>`

- `fn clone(self: &Self) -> ImportThunkList<'data>` — [`ImportThunkList`](#importthunklist)

##### `impl<'data> Debug for ImportThunkList<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DelayLoadImportTable<'data>`

```rust
struct DelayLoadImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

Information for parsing a PE delay-load import table.

Returned by
[`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table).

#### Implementations

- `fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- `fn descriptors(self: &Self) -> Result<DelayLoadDescriptorIterator<'data>>` — [`Result`](../../index.md), [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)

- `fn name(self: &Self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn thunks(self: &Self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../index.md), [`ImportThunkList`](#importthunklist)

- `fn import<Pe: ImageNtHeaders>(self: &Self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](#imagentheaders), [`Result`](../../index.md), [`Import`](#import)

- `fn hint_name(self: &Self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for DelayLoadImportTable<'data>`

- `fn clone(self: &Self) -> DelayLoadImportTable<'data>` — [`DelayLoadImportTable`](#delayloadimporttable)

##### `impl<'data> Debug for DelayLoadImportTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DelayLoadDescriptorIterator<'data>`

```rust
struct DelayLoadDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

A fallible iterator for the descriptors in the delay-load data directory.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<&'data pe::ImageDelayloadDescriptor>>` — [`Result`](../../index.md), [`ImageDelayloadDescriptor`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data> Clone for DelayLoadDescriptorIterator<'data>`

- `fn clone(self: &Self) -> DelayLoadDescriptorIterator<'data>` — [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)

##### `impl<'data> Debug for DelayLoadDescriptorIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for DelayLoadDescriptorIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for DelayLoadDescriptorIterator<'data>`

- `type Item = Result<&'data ImageDelayloadDescriptor, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `RelocationBlockIterator<'data>`

```rust
struct RelocationBlockIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

An iterator over the relocation blocks in the `.reloc` section of a PE file.

Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks).

#### Implementations

- `fn new(data: &'data [u8]) -> Self`

- `fn next(self: &mut Self) -> Result<Option<RelocationIterator<'data>>>` — [`Result`](../../index.md), [`RelocationIterator`](#relocationiterator)

- `fn parse(self: &mut Self) -> Result<RelocationIterator<'data>>` — [`Result`](../../index.md), [`RelocationIterator`](#relocationiterator)

#### Trait Implementations

##### `impl<'data> Clone for RelocationBlockIterator<'data>`

- `fn clone(self: &Self) -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](#relocationblockiterator)

##### `impl<'data> Copy for RelocationBlockIterator<'data>`

##### `impl<'data> Debug for RelocationBlockIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for RelocationBlockIterator<'data>`

- `fn default() -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](#relocationblockiterator)

##### `impl<I> IntoIterator for RelocationBlockIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for RelocationBlockIterator<'data>`

- `type Item = Result<RelocationIterator<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `RelocationIterator<'data>`

```rust
struct RelocationIterator<'data> {
    virtual_address: u32,
    size: u32,
    relocs: slice::Iter<'data, crate::endian::U16<crate::endian::LittleEndian>>,
}
```

An iterator of the relocations in a block in the `.reloc` section of a PE file.

#### Implementations

- `fn virtual_address(self: &Self) -> u32`

- `fn size(self: &Self) -> u32`

#### Trait Implementations

##### `impl<'data> Clone for RelocationIterator<'data>`

- `fn clone(self: &Self) -> RelocationIterator<'data>` — [`RelocationIterator`](#relocationiterator)

##### `impl<'data> Debug for RelocationIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for RelocationIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for RelocationIterator<'data>`

- `type Item = Relocation`

- `fn next(self: &mut Self) -> Option<Relocation>` — [`Relocation`](#relocation)

### `Relocation`

```rust
struct Relocation {
    pub virtual_address: u32,
    pub typ: u16,
}
```

A relocation in the `.reloc` section of a PE file.

#### Fields

- **`virtual_address`**: `u32`

  The virtual address of the relocation.

- **`typ`**: `u16`

  One of the `pe::IMAGE_REL_BASED_*` constants.

#### Trait Implementations

##### `impl Clone for Relocation`

- `fn clone(self: &Self) -> Relocation` — [`Relocation`](#relocation)

##### `impl Copy for Relocation`

##### `impl Debug for Relocation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Relocation`

- `fn default() -> Relocation` — [`Relocation`](#relocation)

### `ResourceDirectory<'data>`

```rust
struct ResourceDirectory<'data> {
    data: &'data [u8],
}
```

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

#### Implementations

- `fn new(data: &'data [u8]) -> Self`

- `fn root(self: &Self) -> Result<ResourceDirectoryTable<'data>>` — [`Result`](../../index.md), [`ResourceDirectoryTable`](#resourcedirectorytable)

#### Trait Implementations

##### `impl<'data> Clone for ResourceDirectory<'data>`

- `fn clone(self: &Self) -> ResourceDirectory<'data>` — [`ResourceDirectory`](#resourcedirectory)

##### `impl<'data> Copy for ResourceDirectory<'data>`

##### `impl<'data> Debug for ResourceDirectory<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ResourceDirectoryTable<'data>`

```rust
struct ResourceDirectoryTable<'data> {
    pub header: &'data pe::ImageResourceDirectory,
    pub entries: &'data [pe::ImageResourceDirectoryEntry],
}
```

A table of resource entries.

#### Fields

- **`header`**: `&'data pe::ImageResourceDirectory`

  The table header.

- **`entries`**: `&'data [pe::ImageResourceDirectoryEntry]`

  The table entries.

#### Implementations

- `fn parse(data: &'data [u8], offset: u32) -> Result<Self>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ResourceDirectoryTable<'data>`

- `fn clone(self: &Self) -> ResourceDirectoryTable<'data>` — [`ResourceDirectoryTable`](#resourcedirectorytable)

##### `impl<'data> Debug for ResourceDirectoryTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ResourceName`

```rust
struct ResourceName {
    offset: u32,
}
```

A resource name.

#### Implementations

- `fn to_string_lossy(self: &Self, directory: ResourceDirectory<'_>) -> Result<String>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md)

- `fn data<'data>(self: &Self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md), [`U16Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- `fn raw_data<'data>(self: &Self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md)

#### Trait Implementations

##### `impl Clone for ResourceName`

- `fn clone(self: &Self) -> ResourceName` — [`ResourceName`](#resourcename)

##### `impl Copy for ResourceName`

##### `impl Debug for ResourceName`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RichHeaderInfo<'data>`

```rust
struct RichHeaderInfo<'data> {
    pub offset: usize,
    pub length: usize,
    pub xor_key: u32,
    masked_entries: &'data [pe::MaskedRichHeaderEntry],
}
```

Parsed information about a Rich Header.

#### Fields

- **`offset`**: `usize`

  The offset at which the rich header starts.

- **`length`**: `usize`

  The length (in bytes) of the rich header.
  
  This includes the payload, but also the 16-byte start sequence and the
  8-byte final "Rich" and XOR key.

- **`xor_key`**: `u32`

  The XOR key used to mask the rich header.
  
  Unless the file has been tampered with, it should be equal to a checksum
  of the file header.

#### Implementations

- `fn parse<R: ReadRef<'data>>(data: R, nt_header_offset: u64) -> Option<Self>`

- `fn unmasked_entries(self: &Self) -> impl Iterator<Item = RichHeaderEntry> + 'data` — [`RichHeaderEntry`](#richheaderentry)

#### Trait Implementations

##### `impl<'data> Clone for RichHeaderInfo<'data>`

- `fn clone(self: &Self) -> RichHeaderInfo<'data>` — [`RichHeaderInfo`](#richheaderinfo)

##### `impl<'data> Copy for RichHeaderInfo<'data>`

##### `impl<'data> Debug for RichHeaderInfo<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RichHeaderEntry`

```rust
struct RichHeaderEntry {
    pub comp_id: u32,
    pub count: u32,
}
```

A PE rich header entry after it has been unmasked.

See [`pe::MaskedRichHeaderEntry`](../../pe/index.md).

#### Fields

- **`comp_id`**: `u32`

  ID of the component.

- **`count`**: `u32`

  Number of times this component has been used when building this PE.

#### Trait Implementations

##### `impl Clone for RichHeaderEntry`

- `fn clone(self: &Self) -> RichHeaderEntry` — [`RichHeaderEntry`](#richheaderentry)

##### `impl Copy for RichHeaderEntry`

##### `impl Debug for RichHeaderEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `ExportTarget<'data>`

```rust
enum ExportTarget<'data> {
    Address(u32),
    ForwardByOrdinal(&'data [u8], u32),
    ForwardByName(&'data [u8], &'data [u8]),
}
```

Where an export is pointing to.

#### Variants

- **`Address`**

  The address of the export, relative to the image base.

- **`ForwardByOrdinal`**

  Forwarded to an export ordinal in another DLL.
  
  This gives the name of the DLL, and the ordinal.

- **`ForwardByName`**

  Forwarded to an export name in another DLL.
  
  This gives the name of the DLL, and the export name.

#### Implementations

- `fn is_address(self: &Self) -> bool`

- `fn is_forward(self: &Self) -> bool`

#### Trait Implementations

##### `impl<'data> Clone for ExportTarget<'data>`

- `fn clone(self: &Self) -> ExportTarget<'data>` — [`ExportTarget`](#exporttarget)

##### `impl<'data> Copy for ExportTarget<'data>`

##### `impl<'a> Debug for ExportTarget<'a>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

### `Import<'data>`

```rust
enum Import<'data> {
    Ordinal(u16),
    Name(u16, &'data [u8]),
}
```

A parsed import thunk.

#### Variants

- **`Ordinal`**

  Import by ordinal.

- **`Name`**

  Import by name.
  
  Includes a hint for the index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl<'data> Clone for Import<'data>`

- `fn clone(self: &Self) -> Import<'data>` — [`Import`](#import)

##### `impl<'data> Copy for Import<'data>`

##### `impl<'data> Debug for Import<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ResourceDirectoryEntryData<'data>`

```rust
enum ResourceDirectoryEntryData<'data> {
    Table(ResourceDirectoryTable<'data>),
    Data(&'data pe::ImageResourceDataEntry),
}
```

Data associated with a resource directory entry.

#### Variants

- **`Table`**

  A subtable entry.

- **`Data`**

  A resource data entry.

#### Implementations

- `fn table(self: Self) -> Option<ResourceDirectoryTable<'data>>` — [`ResourceDirectoryTable`](#resourcedirectorytable)

- `fn data(self: Self) -> Option<&'data pe::ImageResourceDataEntry>` — [`ImageResourceDataEntry`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data> Clone for ResourceDirectoryEntryData<'data>`

- `fn clone(self: &Self) -> ResourceDirectoryEntryData<'data>` — [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata)

##### `impl<'data> Debug for ResourceDirectoryEntryData<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ResourceNameOrId`

```rust
enum ResourceNameOrId {
    Name(ResourceName),
    Id(u16),
}
```

A resource name or ID.

Can be either a string or a numeric ID.

#### Variants

- **`Name`**

  A resource name.

- **`Id`**

  A resource ID.

#### Implementations

- `fn name(self: Self) -> Option<ResourceName>` — [`ResourceName`](#resourcename)

- `fn id(self: Self) -> Option<u16>`

#### Trait Implementations

##### `impl Debug for ResourceNameOrId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `ImageNtHeaders`

```rust
trait ImageNtHeaders: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageNtHeaders32`](../../pe/index.md) and [`pe::ImageNtHeaders64`](../../pe/index.md).

#### Required Methods

- `type ImageOptionalHeader: 1`

- `type ImageThunkData: 1`

- `fn is_type_64(self: &Self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_valid_optional_magic(self: &Self) -> bool`

  Return true if the magic field in the optional header is valid.

- `fn signature(self: &Self) -> u32`

  Return the signature

- `fn file_header(self: &Self) -> &pe::ImageFileHeader`

  Return the file header.

- `fn optional_header(self: &Self) -> &<Self as >::ImageOptionalHeader`

  Return the optional header.

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<(&'data Self, DataDirectories<'data>)>`

  Read the NT headers, including the data directories.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, data: R, offset: u64) -> read::Result<SectionTable<'data>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<SymbolTable<'data, R>>`

  Read the COFF symbol table and string table.

### `ImageOptionalHeader`

```rust
trait ImageOptionalHeader: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageOptionalHeader32`](../../pe/index.md) and [`pe::ImageOptionalHeader64`](../../pe/index.md).

#### Required Methods

- `fn magic(self: &Self) -> u16`

- `fn major_linker_version(self: &Self) -> u8`

- `fn minor_linker_version(self: &Self) -> u8`

- `fn size_of_code(self: &Self) -> u32`

- `fn size_of_initialized_data(self: &Self) -> u32`

- `fn size_of_uninitialized_data(self: &Self) -> u32`

- `fn address_of_entry_point(self: &Self) -> u32`

- `fn base_of_code(self: &Self) -> u32`

- `fn base_of_data(self: &Self) -> Option<u32>`

- `fn image_base(self: &Self) -> u64`

- `fn section_alignment(self: &Self) -> u32`

- `fn file_alignment(self: &Self) -> u32`

- `fn major_operating_system_version(self: &Self) -> u16`

- `fn minor_operating_system_version(self: &Self) -> u16`

- `fn major_image_version(self: &Self) -> u16`

- `fn minor_image_version(self: &Self) -> u16`

- `fn major_subsystem_version(self: &Self) -> u16`

- `fn minor_subsystem_version(self: &Self) -> u16`

- `fn win32_version_value(self: &Self) -> u32`

- `fn size_of_image(self: &Self) -> u32`

- `fn size_of_headers(self: &Self) -> u32`

- `fn check_sum(self: &Self) -> u32`

- `fn subsystem(self: &Self) -> u16`

- `fn dll_characteristics(self: &Self) -> u16`

- `fn size_of_stack_reserve(self: &Self) -> u64`

- `fn size_of_stack_commit(self: &Self) -> u64`

- `fn size_of_heap_reserve(self: &Self) -> u64`

- `fn size_of_heap_commit(self: &Self) -> u64`

- `fn loader_flags(self: &Self) -> u32`

- `fn number_of_rva_and_sizes(self: &Self) -> u32`

### `ImageThunkData`

```rust
trait ImageThunkData: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageThunkData32`](../../pe/index.md) and [`pe::ImageThunkData64`](../../pe/index.md).

#### Required Methods

- `fn raw(self: Self) -> u64`

  Return the raw thunk value.

- `fn is_ordinal(self: Self) -> bool`

  Returns true if the ordinal flag is set.

- `fn ordinal(self: Self) -> u16`

  Return the ordinal portion of the thunk.

- `fn address(self: Self) -> u32`

  Return the RVA portion of the thunk.

## Functions

### `optional_header_magic`

```rust
fn optional_header_magic<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<u16>
```

Find the optional header and read its `magic` field.

It can be useful to know this magic value before trying to
fully parse the NT headers.

### `parse_ordinal`

```rust
fn parse_ordinal(digits: &[u8]) -> Option<u32>
```

### `memmem`

```rust
fn memmem(data: &[u8], needle: &[u8], align: usize) -> Option<usize>
```

Find the offset of the first occurrence of needle in the data.

The offset must have the given alignment.

## Type Aliases

### `PeFile32<'data, R>`

```rust
type PeFile32<'data, R> = PeFile<'data, pe::ImageNtHeaders32, R>;
```

A PE32 (32-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders32`](../../pe/index.md), and corresponds
to [`crate::FileKind::Pe32`](../../index.md).

### `PeFile64<'data, R>`

```rust
type PeFile64<'data, R> = PeFile<'data, pe::ImageNtHeaders64, R>;
```

A PE32+ (64-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders64`](../../pe/index.md), and corresponds
to [`crate::FileKind::Pe64`](../../index.md).

### `PeComdatIterator32<'data, 'file, R>`

```rust
type PeComdatIterator32<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the COMDAT section groups in a [`PeFile32`](#pefile32).

### `PeComdatIterator64<'data, 'file, R>`

```rust
type PeComdatIterator64<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the COMDAT section groups in a [`PeFile64`](#pefile64).

### `PeComdat32<'data, 'file, R>`

```rust
type PeComdat32<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders32, R>;
```

A COMDAT section group in a [`PeFile32`](#pefile32).

### `PeComdat64<'data, 'file, R>`

```rust
type PeComdat64<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders64, R>;
```

A COMDAT section group in a [`PeFile64`](#pefile64).

### `PeComdatSectionIterator32<'data, 'file, R>`

```rust
type PeComdatSectionIterator32<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the sections in a COMDAT section group in a [`PeFile32`](#pefile32).

### `PeComdatSectionIterator64<'data, 'file, R>`

```rust
type PeComdatSectionIterator64<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the sections in a COMDAT section group in a [`PeFile64`](#pefile64).

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

