*[object](../../index.md) / [read](../index.md) / [coff](index.md)*

---

# Module `coff`

Support for reading Windows COFF files.

Traits are used to abstract over the difference between COFF object files
and COFF bigobj files. The primary trait for this is [`CoffHeader`](#coffheader).

## High level API

[`CoffFile`](#cofffile) implements the [`Object`](crate::read::Object) trait for
COFF files. [`CoffFile`](#cofffile) is parameterised by [`CoffHeader`](#coffheader).
The default parameter allows reading regular COFF object files,
while the type alias [`CoffBigFile`](#coffbigfile) allows reading COFF bigobj files.

[`ImportFile`](#importfile) allows reading COFF short imports that are used in import
libraries. Currently these are not integrated with the unified read API.

## Low level API

The [`CoffHeader`](#coffheader) trait can be directly used to parse both COFF
object files (which start with [`pe::ImageFileHeader`](../../pe/index.md)) and COFF bigobj
files (which start with [`pe::AnonObjectHeaderBigobj`](../../pe/index.md)).

### Example for low level API
 ```no_run
use object::pe;
use object::read::coff::{CoffHeader, ImageSymbol as _};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = pe::ImageFileHeader::parse(&*data, &mut offset)?;
    let sections = header.sections(&*data, offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
  }
    Ok(())
}
```

## Modules

- [`file`](file/index.md) - 
- [`section`](section/index.md) - 
- [`symbol`](symbol/index.md) - 
- [`relocation`](relocation/index.md) - 
- [`comdat`](comdat/index.md) - 
- [`import`](import/index.md) - Support for reading short import files.

## Structs

### `CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    sections: super::SectionTable<'data>,
    symbols: super::SymbolTable<'data, R, Coff>,
    image_base: u64,
}
```

The common parts of `PeFile` and `CoffFile`.

#### Trait Implementations

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffCommon<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    header: &'data Coff,
    common: CoffCommon<'data, R, Coff>,
    data: R,
}
```

A COFF object file.

This is a file that starts with [`pe::ImageFileHeader`](../../pe/index.md), and corresponds
to [`crate::FileKind::Coff`](../../index.md).

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn coff_header(self: &Self) -> &'data Coff`

- `fn coff_section_table(self: &Self) -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

- `fn coff_symbol_table(self: &Self) -> &SymbolTable<'data, R, Coff>` — [`SymbolTable`](#symboltable)

#### Trait Implementations

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffFile<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R, Coff> Object for CoffFile<'data, R, Coff>`

- `type Segment = CoffSegment<'data, 'file, R, Coff>`

- `type SegmentIterator = CoffSegmentIterator<'data, 'file, R, Coff>`

- `type Section = CoffSection<'data, 'file, R, Coff>`

- `type SectionIterator = CoffSectionIterator<'data, 'file, R, Coff>`

- `type Comdat = CoffComdat<'data, 'file, R, Coff>`

- `type ComdatIterator = CoffComdatIterator<'data, 'file, R, Coff>`

- `type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- `type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- `type SymbolTable = CoffSymbolTable<'data, 'file, R, Coff>`

- `type DynamicRelocationIterator = NoDynamicRelocationIterator`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- `fn segments(self: &Self) -> CoffSegmentIterator<'data, '_, R, Coff>` — [`CoffSegmentIterator`](#coffsegmentiterator)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<CoffSection<'data, 'file, R, Coff>>` — [`CoffSection`](#coffsection)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<CoffSection<'data, '_, R, Coff>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`CoffSection`](#coffsection)

- `fn sections(self: &Self) -> CoffSectionIterator<'data, '_, R, Coff>` — [`CoffSectionIterator`](#coffsectioniterator)

- `fn comdats(self: &Self) -> CoffComdatIterator<'data, '_, R, Coff>` — [`CoffComdatIterator`](#coffcomdatiterator)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R, Coff>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`CoffSymbol`](#coffsymbol)

- `fn symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](#coffsymboliterator)

- `fn symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](#coffsymboltable)

- `fn dynamic_symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](#coffsymboliterator)

- `fn dynamic_symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](#coffsymboltable)

- `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffFile<'data, R, Coff>`

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

- `fn pe_file_range_at(self: &Self, va: u32) -> Option<(u32, u32)>`

- `fn pe_data_at<R: ReadRef<'data>>(self: &Self, data: R, va: u32) -> Option<&'data [u8]>`

- `fn pe_data_containing<R: ReadRef<'data>>(self: &Self, data: R, va: u32) -> Option<(&'data [u8], u32)>`

- `fn section_containing(self: &Self, va: u32) -> Option<&'data ImageSectionHeader>` — [`ImageSectionHeader`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data> Clone for SectionTable<'data>`

- `fn clone(self: &Self) -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

##### `impl<'data> Copy for SectionTable<'data>`

##### `impl<'data> Debug for SectionTable<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for SectionTable<'data>`

- `fn default() -> SectionTable<'data>` — [`SectionTable`](#sectiontable)

### `CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

An iterator for the loadable sections in a [`CoffFile`](#cofffile).

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

A loadable section in a [`CoffFile`](#cofffile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- `fn coff_file(self: &Self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](#cofffile)

- `fn coff_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffSegment<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSegment for CoffSegment<'data, 'file, R, Coff>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSegment<'data, 'file, R, Coff>`

### `CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

An iterator for the sections in a [`CoffFile`](#cofffile).

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

A section in a [`CoffFile`](#cofffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- `fn coff_file(self: &Self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](#cofffile)

- `fn coff_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md)

- `fn coff_relocations(self: &Self) -> Result<&'data [pe::ImageRelocation]>` — [`Result`](../../index.md), [`ImageRelocation`](../../pe/index.md)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffSection<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSection for CoffSection<'data, 'file, R, Coff>`

- `type RelocationIterator = CoffRelocationIterator<'data, 'file, R, Coff>`

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

- `fn relocations(self: &Self) -> CoffRelocationIterator<'data, 'file, R, Coff>` — [`CoffRelocationIterator`](#coffrelocationiterator)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSection<'data, 'file, R, Coff>`

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

- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](#symboliterator)

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`CoffHeader`](#coffheader)

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

### `SymbolIterator<'data, 'table, R, Coff>`

```rust
struct SymbolIterator<'data, 'table, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    symbols: &'table SymbolTable<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for symbol entries in a COFF or PE file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<'data, 'table, R, Coff> Debug for SymbolIterator<'data, 'table, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'table, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'table, R: ReadRef<'data>, Coff: CoffHeader> Iterator for SymbolIterator<'data, 'table, R, Coff>`

- `type Item = (SymbolIndex, &'data <Coff as CoffHeader>::ImageSymbol)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffSymbolTable<'data, 'file, R, Coff>`

```rust
struct CoffSymbolTable<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
}
```

A symbol table in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

#### Trait Implementations

##### `impl<'data, 'file, R, Coff> Clone for CoffSymbolTable<'data, 'file, R, Coff>`

- `fn clone(self: &Self) -> CoffSymbolTable<'data, 'file, R, Coff>` — [`CoffSymbolTable`](#coffsymboltable)

##### `impl<'data, 'file, R, Coff> Copy for CoffSymbolTable<'data, 'file, R, Coff>`

##### `impl<'data, 'file, R, Coff> Debug for CoffSymbolTable<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbolTable for CoffSymbolTable<'data, 'file, R, Coff>`

- `type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- `type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbolTable<'data, 'file, R, Coff>`

### `CoffSymbolIterator<'data, 'file, R, Coff>`

```rust
struct CoffSymbolIterator<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

#### Implementations

- `fn new(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](file/index.md)

- `fn empty(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](file/index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffSymbolIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- `type Item = CoffSymbol<'data, 'file, R, Coff>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffSymbol<'data, 'file, R, Coff>`

```rust
struct CoffSymbol<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Coff as >::ImageSymbol,
}
```

A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- `fn raw_symbol(self: &Self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](#coffheader)

- `fn coff_symbol(self: &Self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](#coffheader)

#### Trait Implementations

##### `impl<'data, 'file, R, Coff> Clone for CoffSymbol<'data, 'file, R, Coff>`

- `fn clone(self: &Self) -> CoffSymbol<'data, 'file, R, Coff>` — [`CoffSymbol`](#coffsymbol)

##### `impl<'data, 'file, R, Coff> Copy for CoffSymbol<'data, 'file, R, Coff>`

##### `impl<'data, 'file, R, Coff> Debug for CoffSymbol<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbol for CoffSymbol<'data, 'file, R, Coff>`

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

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbol<'data, 'file, R, Coff>`

### `CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageRelocation>,
}
```

An iterator for the relocations in a [`CoffSection`](super::CoffSection).

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffRelocationIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffComdatIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdatIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the COMDAT section groups in a [`CoffFile`](#cofffile).

#### Implementations

- `fn new(file: &'file CoffFile<'data, R, Coff>) -> Self` — [`CoffFile`](#cofffile)

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffComdatIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for CoffComdatIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffComdatIterator<'data, 'file, R, Coff>`

- `type Item = CoffComdat<'data, 'file, R, Coff>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffComdat<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdat<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    symbol_index: crate::read::SymbolIndex,
    symbol: &'data <Coff as >::ImageSymbol,
    selection: u8,
}
```

A COMDAT section group in a [`CoffFile`](#cofffile).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Implementations

- `fn parse(file: &'file CoffFile<'data, R, Coff>, section_symbol: &'data <Coff as >::ImageSymbol, index: SymbolIndex) -> Option<CoffComdat<'data, 'file, R, Coff>>` — [`CoffFile`](#cofffile), [`CoffHeader`](#coffheader), [`SymbolIndex`](../../index.md), [`CoffComdat`](#coffcomdat)

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffComdat<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectComdat for CoffComdat<'data, 'file, R, Coff>`

- `type SectionIterator = CoffComdatSectionIterator<'data, 'file, R, Coff>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffComdat<'data, 'file, R, Coff>`

### `CoffComdatSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdatSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    section_number: i32,
    index: crate::read::SymbolIndex,
}
```

An iterator for the sections in a COMDAT section group in a [`CoffFile`](#cofffile).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ImportFile<'data>`

```rust
struct ImportFile<'data> {
    header: &'data pe::ImportObjectHeader,
    kind: ImportType,
    dll: crate::read::ByteString<'data>,
    symbol: crate::read::ByteString<'data>,
    import: Option<crate::read::ByteString<'data>>,
}
```

A Windows short form description of a symbol to import.

Used in Windows import libraries to provide a mapping from
a symbol name to a DLL export. This is not an object file.

This is a file that starts with [`pe::ImportObjectHeader`](../../pe/index.md), and corresponds
to [`crate::FileKind::CoffImport`](../../index.md).

#### Implementations

- `fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md)

- `fn symbol(self: &Self) -> &'data [u8]`

- `fn dll(self: &Self) -> &'data [u8]`

- `fn import(self: &Self) -> ImportName<'data>` — [`ImportName`](#importname)

- `fn import_type(self: &Self) -> ImportType` — [`ImportType`](#importtype)

#### Trait Implementations

##### `impl<'data> Clone for ImportFile<'data>`

- `fn clone(self: &Self) -> ImportFile<'data>` — [`ImportFile`](#importfile)

##### `impl<'data> Debug for ImportFile<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ImportObjectData<'data>`

```rust
struct ImportObjectData<'data> {
    symbol: crate::read::ByteString<'data>,
    dll: crate::read::ByteString<'data>,
    export: Option<crate::read::ByteString<'data>>,
}
```

The data following [`pe::ImportObjectHeader`](../../pe/index.md).

#### Implementations

- `fn symbol(self: &Self) -> &'data [u8]`

- `fn dll(self: &Self) -> &'data [u8]`

- `fn export(self: &Self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for ImportObjectData<'data>`

- `fn clone(self: &Self) -> ImportObjectData<'data>` — [`ImportObjectData`](#importobjectdata)

##### `impl<'data> Debug for ImportObjectData<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `ImportName<'data>`

```rust
enum ImportName<'data> {
    Ordinal(u16),
    Name(&'data [u8]),
}
```

The name or ordinal to import from a DLL.

#### Variants

- **`Ordinal`**

  Import by ordinal. Ordinarily this is a 1-based index.

- **`Name`**

  Import by name.

#### Trait Implementations

##### `impl<'data> Clone for ImportName<'data>`

- `fn clone(self: &Self) -> ImportName<'data>` — [`ImportName`](#importname)

##### `impl<'data> Copy for ImportName<'data>`

##### `impl<'data> Debug for ImportName<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for ImportName<'data>`

##### `impl<'data> PartialEq for ImportName<'data>`

- `fn eq(self: &Self, other: &ImportName<'data>) -> bool` — [`ImportName`](#importname)

##### `impl<'data> StructuralPartialEq for ImportName<'data>`

### `ImportType`

```rust
enum ImportType {
    Code,
    Data,
    Const,
}
```

The kind of import symbol.

#### Variants

- **`Code`**

  An executable code symbol.

- **`Data`**

  A data symbol.

- **`Const`**

  A constant value.

#### Trait Implementations

##### `impl Clone for ImportType`

- `fn clone(self: &Self) -> ImportType` — [`ImportType`](#importtype)

##### `impl Copy for ImportType`

##### `impl Debug for ImportType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ImportType`

##### `impl Hash for ImportType`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ImportType`

- `fn eq(self: &Self, other: &ImportType) -> bool` — [`ImportType`](#importtype)

##### `impl StructuralPartialEq for ImportType`

## Traits

### `CoffHeader`

```rust
trait CoffHeader: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageFileHeader`](../../pe/index.md) and [`pe::AnonObjectHeaderBigobj`](../../pe/index.md).

#### Required Methods

- `type ImageSymbol: 1`

- `type ImageSymbolBytes: 2`

- `fn is_type_bigobj() -> bool`

  Return true if this type is [`pe::AnonObjectHeaderBigobj`](../../pe/index.md).

- `fn machine(self: &Self) -> u16`

- `fn number_of_sections(self: &Self) -> u32`

- `fn pointer_to_symbol_table(self: &Self) -> u32`

- `fn number_of_symbols(self: &Self) -> u32`

- `fn characteristics(self: &Self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>`

  Read the file header.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, data: R, offset: u64) -> read::Result<SectionTable<'data>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<SymbolTable<'data, R, Self>>`

  Read the symbol table and string table.

### `ImageSymbol`

```rust
trait ImageSymbol: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageSymbol`](../../pe/index.md) and [`pe::ImageSymbolEx`](../../pe/index.md).

#### Required Methods

- `fn raw_name(self: &Self) -> &[u8; 8]`

- `fn value(self: &Self) -> u32`

- `fn section_number(self: &Self) -> i32`

- `fn typ(self: &Self) -> u16`

- `fn storage_class(self: &Self) -> u8`

- `fn number_of_aux_symbols(self: &Self) -> u8`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse a COFF symbol name.

- `fn address(self: &Self, image_base: u64, sections: &SectionTable<'_>) -> Result<Option<u64>>`

  Return the symbol address.

- `fn section(self: &Self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_definition(self: &Self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn has_aux_file_name(self: &Self) -> bool`

  Return true if the symbol has an auxiliary file name.

- `fn has_aux_function(self: &Self) -> bool`

  Return true if the symbol has an auxiliary function symbol.

- `fn has_aux_section(self: &Self) -> bool`

  Return true if the symbol has an auxiliary section symbol.

- `fn has_aux_weak_external(self: &Self) -> bool`

  Return true if the symbol has an auxiliary weak external symbol.

- `fn base_type(self: &Self) -> u16`

- `fn derived_type(self: &Self) -> u16`

## Functions

### `anon_object_class_id`

```rust
fn anon_object_class_id<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<pe::ClsId>
```

Read the `class_id` field from a [`pe::AnonObjectHeader`](../../pe/index.md).

This can be used to determine the format of the header.

## Type Aliases

### `CoffBigFile<'data, R>`

```rust
type CoffBigFile<'data, R> = CoffFile<'data, R, pe::AnonObjectHeaderBigobj>;
```

A COFF bigobj object file with 32-bit section numbers.

This is a file that starts with [`pe::AnonObjectHeaderBigobj`](../../pe/index.md), and corresponds
to [`crate::FileKind::CoffBig`](../../index.md).

Most functionality is provided by the [`Object`](../index.md) trait implementation.

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

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

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

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

### `CoffBigSymbolTable<'data, 'file, R>`

```rust
type CoffBigSymbolTable<'data, 'file, R> = CoffSymbolTable<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A symbol table in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSymbolIterator<'data, 'file, R>`

```rust
type CoffBigSymbolIterator<'data, 'file, R> = CoffSymbolIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSymbol<'data, 'file, R>`

```rust
type CoffBigSymbol<'data, 'file, R> = CoffSymbol<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A symbol in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

### `CoffBigRelocationIterator<'data, 'file, R>`

```rust
type CoffBigRelocationIterator<'data, 'file, R> = CoffRelocationIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection).

### `CoffBigComdatIterator<'data, 'file, R>`

```rust
type CoffBigComdatIterator<'data, 'file, R> = CoffComdatIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigComdat<'data, 'file, R>`

```rust
type CoffBigComdat<'data, 'file, R> = CoffComdat<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

### `CoffBigComdatSectionIterator<'data, 'file, R>`

```rust
type CoffBigComdatSectionIterator<'data, 'file, R> = CoffComdatSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

