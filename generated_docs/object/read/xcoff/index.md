*[object](../../index.md) / [read](../index.md) / [xcoff](index.md)*

---

# Module `xcoff`

Support for reading AIX XCOFF files.

Traits are used to abstract over the difference between 32-bit and 64-bit XCOFF.
The primary trait for this is [`FileHeader`](#fileheader).

## High level API

[`XcoffFile`](#xcofffile) implements the [`Object`](crate::read::Object) trait for XCOFF files.
[`XcoffFile`](#xcofffile) is parameterised by [`FileHeader`](#fileheader) to allow reading both 32-bit and
64-bit XCOFF. There are type aliases for these parameters ([`XcoffFile32`](#xcofffile32) and
[`XcoffFile64`](#xcofffile64)).

## Low level API

The [`FileHeader`](#fileheader) trait can be directly used to parse both [`xcoff::FileHeader32`](../../xcoff/index.md)
and [`xcoff::FileHeader64`](../../xcoff/index.md).

### Example for low level API
 ```no_run
use object::xcoff;
use object::read::xcoff::{FileHeader, SectionHeader, Symbol};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = xcoff::FileHeader64::parse(&*data, &mut offset)?;
    let aux_header = header.aux_header(&*data, &mut offset)?;
    let sections = header.sections(&*data, &mut offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name()));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
  }
    Ok(())
}
```

## Contents

- [Modules](#modules)
  - [`file`](#file)
  - [`section`](#section)
  - [`symbol`](#symbol)
  - [`relocation`](#relocation)
  - [`comdat`](#comdat)
  - [`segment`](#segment)
- [Structs](#structs)
  - [`XcoffFile`](#xcofffile)
  - [`XcoffSectionIterator`](#xcoffsectioniterator)
  - [`XcoffSection`](#xcoffsection)
  - [`SectionTable`](#sectiontable)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`XcoffSymbolTable`](#xcoffsymboltable)
  - [`XcoffSymbolIterator`](#xcoffsymboliterator)
  - [`XcoffSymbol`](#xcoffsymbol)
  - [`XcoffRelocationIterator`](#xcoffrelocationiterator)
  - [`XcoffComdatIterator`](#xcoffcomdatiterator)
  - [`XcoffComdat`](#xcoffcomdat)
  - [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator)
  - [`XcoffSegmentIterator`](#xcoffsegmentiterator)
  - [`XcoffSegment`](#xcoffsegment)
- [Traits](#traits)
  - [`FileHeader`](#fileheader)
  - [`AuxHeader`](#auxheader)
  - [`SectionHeader`](#sectionheader)
  - [`Symbol`](#symbol)
  - [`FileAux`](#fileaux)
  - [`CsectAux`](#csectaux)
  - [`Rel`](#rel)
- [Type Aliases](#type-aliases)
  - [`XcoffFile32`](#xcofffile32)
  - [`XcoffFile64`](#xcofffile64)
  - [`XcoffSectionIterator32`](#xcoffsectioniterator32)
  - [`XcoffSectionIterator64`](#xcoffsectioniterator64)
  - [`XcoffSection32`](#xcoffsection32)
  - [`XcoffSection64`](#xcoffsection64)
  - [`XcoffSymbolTable32`](#xcoffsymboltable32)
  - [`XcoffSymbolTable64`](#xcoffsymboltable64)
  - [`XcoffSymbolIterator32`](#xcoffsymboliterator32)
  - [`XcoffSymbolIterator64`](#xcoffsymboliterator64)
  - [`XcoffSymbol32`](#xcoffsymbol32)
  - [`XcoffSymbol64`](#xcoffsymbol64)
  - [`XcoffRelocationIterator32`](#xcoffrelocationiterator32)
  - [`XcoffRelocationIterator64`](#xcoffrelocationiterator64)
  - [`XcoffComdatIterator32`](#xcoffcomdatiterator32)
  - [`XcoffComdatIterator64`](#xcoffcomdatiterator64)
  - [`XcoffComdat32`](#xcoffcomdat32)
  - [`XcoffComdat64`](#xcoffcomdat64)
  - [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32)
  - [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64)
  - [`XcoffSegmentIterator32`](#xcoffsegmentiterator32)
  - [`XcoffSegmentIterator64`](#xcoffsegmentiterator64)
  - [`XcoffSegment32`](#xcoffsegment32)
  - [`XcoffSegment64`](#xcoffsegment64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`file`](#file) | mod |  |
| [`section`](#section) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`relocation`](#relocation) | mod |  |
| [`comdat`](#comdat) | mod | XCOFF doesn't support the COMDAT section. |
| [`segment`](#segment) | mod | TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready. |
| [`XcoffFile`](#xcofffile) | struct | A partially parsed XCOFF file. |
| [`XcoffSectionIterator`](#xcoffsectioniterator) | struct | An iterator for the sections in an [`XcoffFile`]. |
| [`XcoffSection`](#xcoffsection) | struct | A section in an [`XcoffFile`]. |
| [`SectionTable`](#sectiontable) | struct | The table of section headers in an XCOFF file. |
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in an XCOFF file. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for symbol entries in an XCOFF file. |
| [`XcoffSymbolTable`](#xcoffsymboltable) | struct | A symbol table in an [`XcoffFile`]. |
| [`XcoffSymbolIterator`](#xcoffsymboliterator) | struct | An iterator for the symbols in an [`XcoffFile`]. |
| [`XcoffSymbol`](#xcoffsymbol) | struct | A symbol in an [`XcoffFile`]. |
| [`XcoffRelocationIterator`](#xcoffrelocationiterator) | struct | An iterator for the relocations in an [`XcoffSection`](super::XcoffSection). |
| [`XcoffComdatIterator`](#xcoffcomdatiterator) | struct | An iterator for the COMDAT section groups in a [`XcoffFile`]. |
| [`XcoffComdat`](#xcoffcomdat) | struct | A COMDAT section group in a [`XcoffFile`]. |
| [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`XcoffFile`]. |
| [`XcoffSegmentIterator`](#xcoffsegmentiterator) | struct | An iterator for the segments in an [`XcoffFile`]. |
| [`XcoffSegment`](#xcoffsegment) | struct | A loadable section in an [`XcoffFile`]. |
| [`FileHeader`](#fileheader) | trait | A trait for generic access to [`xcoff::FileHeader32`] and [`xcoff::FileHeader64`]. |
| [`AuxHeader`](#auxheader) | trait | A trait for generic access to [`xcoff::AuxHeader32`] and [`xcoff::AuxHeader64`]. |
| [`SectionHeader`](#sectionheader) | trait | A trait for generic access to [`xcoff::SectionHeader32`] and [`xcoff::SectionHeader64`]. |
| [`Symbol`](#symbol) | trait | A trait for generic access to [`xcoff::Symbol32`] and [`xcoff::Symbol64`]. |
| [`FileAux`](#fileaux) | trait | A trait for generic access to [`xcoff::FileAux32`] and [`xcoff::FileAux64`]. |
| [`CsectAux`](#csectaux) | trait | A trait for generic access to [`xcoff::CsectAux32`] and [`xcoff::CsectAux64`]. |
| [`Rel`](#rel) | trait | A trait for generic access to [`xcoff::Rel32`] and [`xcoff::Rel64`]. |
| [`XcoffFile32`](#xcofffile32) | type | A 32-bit XCOFF object file. |
| [`XcoffFile64`](#xcofffile64) | type | A 64-bit XCOFF object file. |
| [`XcoffSectionIterator32`](#xcoffsectioniterator32) | type | An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSectionIterator64`](#xcoffsectioniterator64) | type | An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSection32`](#xcoffsection32) | type | A section in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSection64`](#xcoffsection64) | type | A section in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbolTable32`](#xcoffsymboltable32) | type | A symbol table in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolTable64`](#xcoffsymboltable64) | type | A symbol table in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbolIterator32`](#xcoffsymboliterator32) | type | An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolIterator64`](#xcoffsymboliterator64) | type | An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbol32`](#xcoffsymbol32) | type | A symbol in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbol64`](#xcoffsymbol64) | type | A symbol in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffRelocationIterator32`](#xcoffrelocationiterator32) | type | An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32). |
| [`XcoffRelocationIterator64`](#xcoffrelocationiterator64) | type | An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64). |
| [`XcoffComdatIterator32`](#xcoffcomdatiterator32) | type | An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatIterator64`](#xcoffcomdatiterator64) | type | An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdat32`](#xcoffcomdat32) | type | A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdat64`](#xcoffcomdat64) | type | A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegmentIterator32`](#xcoffsegmentiterator32) | type | An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegmentIterator64`](#xcoffsegmentiterator64) | type | An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegment32`](#xcoffsegment32) | type | A segment in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegment64`](#xcoffsegment64) | type | A segment in an [`XcoffFile64`](super::XcoffFile64). |

## Modules

- [`file`](file/index.md)
- [`section`](section/index.md)
- [`symbol`](symbol/index.md)
- [`relocation`](relocation/index.md)
- [`comdat`](comdat/index.md) — XCOFF doesn't support the COMDAT section.
- [`segment`](segment/index.md) — TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready.

## Structs

### `XcoffFile<'data, Xcoff, R>`

```rust
struct XcoffFile<'data, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    data: R,
    header: &'data Xcoff,
    aux_header: Option<&'data <Xcoff as >::AuxHeader>,
    sections: super::SectionTable<'data, Xcoff>,
    symbols: super::SymbolTable<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:35-45`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/file.rs#L35-L45)*

A partially parsed XCOFF file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="xcofffile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- <span id="xcofffile-data"></span>`fn data(&self) -> R`

- <span id="xcofffile-raw-header"></span>`fn raw_header(&self) -> &'data Xcoff`

- <span id="xcofffile-xcoff-header"></span>`fn xcoff_header(&self) -> &'data Xcoff`

- <span id="xcofffile-xcoff-aux-header"></span>`fn xcoff_aux_header(&self) -> Option<&'data <Xcoff as >::AuxHeader>` — [`FileHeader`](#fileheader)

- <span id="xcofffile-xcoff-section-table"></span>`fn xcoff_section_table(&self) -> &SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

- <span id="xcofffile-xcoff-symbol-table"></span>`fn xcoff_symbol_table(&self) -> &SymbolTable<'data, Xcoff, R>` — [`SymbolTable`](#symboltable)

#### Trait Implementations

##### `impl<'data, Xcoff, R> Debug for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Xcoff, R> Object for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-type-segment"></span>`type Segment = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-segmentiterator"></span>`type SegmentIterator = XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-section"></span>`type Section = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-sectioniterator"></span>`type SectionIterator = XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-comdat"></span>`type Comdat = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-comdatiterator"></span>`type ComdatIterator = XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-symboltable"></span>`type SymbolTable = XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcofffile-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="xcofffile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md)

- <span id="xcofffile-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="xcofffile-is-64"></span>`fn is_64(&self) -> bool`

- <span id="xcofffile-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- <span id="xcofffile-segments"></span>`fn segments(&self) -> XcoffSegmentIterator<'data, '_, Xcoff, R>` — [`XcoffSegmentIterator`](#xcoffsegmentiterator)

- <span id="xcofffile-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>` — [`XcoffSection`](#xcoffsection)

- <span id="xcofffile-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<XcoffSection<'data, '_, Xcoff, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`XcoffSection`](#xcoffsection)

- <span id="xcofffile-sections"></span>`fn sections(&self) -> XcoffSectionIterator<'data, '_, Xcoff, R>` — [`XcoffSectionIterator`](#xcoffsectioniterator)

- <span id="xcofffile-comdats"></span>`fn comdats(&self) -> XcoffComdatIterator<'data, '_, Xcoff, R>` — [`XcoffComdatIterator`](#xcoffcomdatiterator)

- <span id="xcofffile-symbol-table"></span>`fn symbol_table(&self) -> Option<XcoffSymbolTable<'data, '_, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- <span id="xcofffile-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<XcoffSymbol<'data, '_, Xcoff, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`XcoffSymbol`](#xcoffsymbol)

- <span id="xcofffile-symbols"></span>`fn symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- <span id="xcofffile-dynamic-symbol-table"></span>`fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- <span id="xcofffile-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- <span id="xcofffile-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>` — [`Object`](../index.md)

- <span id="xcofffile-imports"></span>`fn imports(&self) -> Result<alloc::vec::Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- <span id="xcofffile-exports"></span>`fn exports(&self) -> Result<alloc::vec::Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- <span id="xcofffile-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="xcofffile-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="xcofffile-entry"></span>`fn entry(&self) -> u64`

- <span id="xcofffile-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, Xcoff, R> Sealed for XcoffFile<'data, Xcoff, R>`

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

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:23-30`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L23-L30)*

An iterator for the sections in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-type-item"></span>`type Item = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:59-67`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L59-L67)*

A section in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="xcoffsection-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

- <span id="xcoffsection-xcoff-section"></span>`fn xcoff_section(&self) -> &'data <Xcoff as >::SectionHeader` — [`FileHeader`](#fileheader)

- <span id="xcoffsection-xcoff-relocations"></span>`fn xcoff_relocations(&self) -> Result<&'data [<Xcoff as >::Rel]>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="xcoffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectSection for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-type-relocationiterator"></span>`type RelocationIterator = XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- <span id="xcoffsection-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsection-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsection-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="xcoffsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="xcoffsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="xcoffsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- <span id="xcoffsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- <span id="xcoffsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="xcoffsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- <span id="xcoffsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="xcoffsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="xcoffsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md)

- <span id="xcoffsection-relocations"></span>`fn relocations(&self) -> <Self as >::RelocationIterator` — [`ObjectSection`](../index.md)

- <span id="xcoffsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- <span id="xcoffsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md)

- <span id="xcoffsection-uncompressed-data"></span>`fn uncompressed_data(&self) -> Result<alloc::borrow::Cow<'data, [u8]>>` — [`Result`](../../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffSection<'data, 'file, Xcoff, R>`

### `SectionTable<'data, Xcoff: FileHeader>`

```rust
struct SectionTable<'data, Xcoff: FileHeader> {
    sections: &'data [<Xcoff as >::SectionHeader],
}
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:228-230`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L228-L230)*

The table of section headers in an XCOFF file.

Returned by `FileHeader::sections`.

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<R: ReadRef<'data>>(header: &Xcoff, data: R, offset: &mut u64) -> Result<Self>` — [`Result`](../../index.md)

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` — [`FileHeader`](#fileheader)

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Xcoff: clone::Clone + FileHeader> Clone for SectionTable<'data, Xcoff>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

##### `impl<'data, Xcoff: marker::Copy + FileHeader> Copy for SectionTable<'data, Xcoff>`

##### `impl<'data, Xcoff: fmt::Debug + FileHeader> Debug for SectionTable<'data, Xcoff>`

- <span id="sectiontable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Xcoff> Default for SectionTable<'data, Xcoff>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

### `SymbolTable<'data, Xcoff, R>`

```rust
struct SymbolTable<'data, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'data [xcoff::SymbolBytes],
    strings: crate::read::StringTable<'data, R>,
    header: core::marker::PhantomData<Xcoff>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:23-31`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L23-L31)*

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by `FileHeader::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: Xcoff, data: R) -> Result<Self>` — [`Result`](../../index.md)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

- <span id="symboltable-iter-none"></span>`fn iter_none<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md)

- <span id="symboltable-symbol-unchecked"></span>`fn symbol_unchecked(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="symboltable-aux-file"></span>`fn aux_file(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="symboltable-aux-csect"></span>`fn aux_csect(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl<'data, Xcoff, R> Debug for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Xcoff, R> Default for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `SymbolIterator<'data, 'table, Xcoff, R>`

```rust
struct SymbolIterator<'data, 'table, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'table SymbolTable<'data, Xcoff, R>,
    index: usize,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:182-189`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L182-L189)*

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<'data, 'table, Xcoff, R> Debug for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'table, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-type-item"></span>`type Item = (SymbolIndex, &'data <Xcoff as FileHeader>::Symbol)`

- <span id="symboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSymbolTable<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolTable<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:217-224`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L217-L224)*

A symbol table in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-clone"></span>`fn clone(&self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>` — [`XcoffSymbolTable`](#xcoffsymboltable)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- <span id="xcoffsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbolTable<'data, 'file, Xcoff, R>`

### `XcoffSymbolIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: SymbolIterator<'data, 'file, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:263-270`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L263-L270)*

An iterator for the symbols in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Debug for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsymboliterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsymboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-type-item"></span>`type Item = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSymbol<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbol<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Xcoff as >::Symbol,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:307-316`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L307-L316)*

A symbol in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- <span id="xcoffsymbol-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

- <span id="xcoffsymbol-xcoff-symbol"></span>`fn xcoff_symbol(&self) -> &'data <Xcoff as >::Symbol` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-clone"></span>`fn clone(&self) -> XcoffSymbol<'data, 'file, Xcoff, R>` — [`XcoffSymbol`](#xcoffsymbol)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbol for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- <span id="xcoffsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="xcoffsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="xcoffsymbol-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsymbol-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md)

- <span id="xcoffsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md)

- <span id="xcoffsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="xcoffsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="xcoffsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="xcoffsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="xcoffsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md)

- <span id="xcoffsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="xcoffsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="xcoffsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md), [`SectionIndex`](../../index.md), [`SymbolIndex`](../../index.md)

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbol<'data, 'file, Xcoff, R>`

### `XcoffRelocationIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffRelocationIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    relocations: slice::Iter<'data, <<Xcoff as FileHeader>::SectionHeader as SectionHeader>::Rel>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:23-32`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/relocation.rs#L23-L32)*

An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffrelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="xcoffrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffComdatIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:21-28`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L21-L28)*

An iterator for the COMDAT section groups in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-type-item"></span>`type Item = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffComdat<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdat<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:55-62`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L55-L62)*

A COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectComdat for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-type-sectioniterator"></span>`type SectionIterator = XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- <span id="xcoffcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- <span id="xcoffcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="xcoffcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="xcoffcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffComdat<'data, 'file, Xcoff, R>`

### `XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:115-122`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L115-L122)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-type-item"></span>`type Item = SectionIndex`

- <span id="xcoffcomdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSegmentIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegmentIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:22-29`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/segment.rs#L22-L29)*

An iterator for the segments in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsegmentiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsegmentiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-type-item"></span>`type Item = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSegment<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegment<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:54-61`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/segment.rs#L54-L61)*

A loadable section in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectSegment for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsegment-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsegment-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="xcoffsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="xcoffsegment-data-range"></span>`fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="xcoffsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="xcoffsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="xcoffsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffSegment<'data, 'file, Xcoff, R>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:306-387`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/file.rs#L306-L387)*

A trait for generic access to [`xcoff::FileHeader32`](../../xcoff/index.md) and [`xcoff::FileHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

- `type AuxHeader: 1`

- `type SectionHeader: 1`

- `type Symbol: 1`

- `type FileAux: 1`

- `type CsectAux: 1`

- `type Rel: 1`

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn f_magic(&self) -> u16`

- `fn f_nscns(&self) -> u16`

- `fn f_timdat(&self) -> u32`

- `fn f_symptr(&self) -> <Self as >::Word`

- `fn f_nsyms(&self) -> u32`

- `fn f_opthdr(&self) -> u16`

- `fn f_flags(&self) -> u16`

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

- `fn aux_header<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<Option<&'data <Self as >::AuxHeader>>`

  Read the auxiliary file header.

- `fn sections<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<SectionTable<'data, Self>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(&self, data: R) -> Result<SymbolTable<'data, Self, R>>`

  Return the symbol table.

#### Implementors

- [`FileHeader32`](../../xcoff/index.md)
- [`FileHeader64`](../../xcoff/index.md)

### `AuxHeader`

```rust
trait AuxHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:475-508`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/file.rs#L475-L508)*

A trait for generic access to [`xcoff::AuxHeader32`](../../xcoff/index.md) and [`xcoff::AuxHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn o_mflag(&self) -> u16`

- `fn o_vstamp(&self) -> u16`

- `fn o_tsize(&self) -> <Self as >::Word`

- `fn o_dsize(&self) -> <Self as >::Word`

- `fn o_bsize(&self) -> <Self as >::Word`

- `fn o_entry(&self) -> <Self as >::Word`

- `fn o_text_start(&self) -> <Self as >::Word`

- `fn o_data_start(&self) -> <Self as >::Word`

- `fn o_toc(&self) -> <Self as >::Word`

- `fn o_snentry(&self) -> u16`

- `fn o_sntext(&self) -> u16`

- `fn o_sndata(&self) -> u16`

- `fn o_sntoc(&self) -> u16`

- `fn o_snloader(&self) -> u16`

- `fn o_snbss(&self) -> u16`

- `fn o_algntext(&self) -> u16`

- `fn o_algndata(&self) -> u16`

- `fn o_modtype(&self) -> u16`

- `fn o_cpuflag(&self) -> u8`

- `fn o_cputype(&self) -> u8`

- `fn o_maxstack(&self) -> <Self as >::Word`

- `fn o_maxdata(&self) -> <Self as >::Word`

- `fn o_debugger(&self) -> u32`

- `fn o_textpsize(&self) -> u8`

- `fn o_datapsize(&self) -> u8`

- `fn o_stackpsize(&self) -> u8`

- `fn o_flags(&self) -> u8`

- `fn o_sntdata(&self) -> u16`

- `fn o_sntbss(&self) -> u16`

- `fn o_x64flags(&self) -> Option<u16>`

#### Implementors

- [`AuxHeader32`](../../xcoff/index.md)
- [`AuxHeader64`](../../xcoff/index.md)

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:290-335`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L290-L335)*

A trait for generic access to [`xcoff::SectionHeader32`](../../xcoff/index.md) and [`xcoff::SectionHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

- `type HalfWord: 1`

- `type Xcoff: 1`

- `type Rel: 1`

#### Required Methods

- `fn s_name(&self) -> &[u8; 8]`

- `fn s_paddr(&self) -> <Self as >::Word`

- `fn s_vaddr(&self) -> <Self as >::Word`

- `fn s_size(&self) -> <Self as >::Word`

- `fn s_scnptr(&self) -> <Self as >::Word`

- `fn s_relptr(&self) -> <Self as >::Word`

- `fn s_lnnoptr(&self) -> <Self as >::Word`

- `fn s_nreloc(&self) -> <Self as >::HalfWord`

- `fn s_nlnno(&self) -> <Self as >::HalfWord`

- `fn s_flags(&self) -> u32`

- `fn relocations<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [<Self as >::Rel]>`

  Read the relocations.

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the section name.

- `fn file_range(&self) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

#### Implementors

- [`SectionHeader32`](../../xcoff/index.md)
- [`SectionHeader64`](../../xcoff/index.md)

### `Symbol`

```rust
trait Symbol: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:540-593`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L540-L593)*

A trait for generic access to [`xcoff::Symbol32`](../../xcoff/index.md) and [`xcoff::Symbol64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn n_value(&self) -> <Self as >::Word`

- `fn n_scnum(&self) -> i16`

- `fn n_type(&self) -> u16`

- `fn n_sclass(&self) -> u8`

- `fn n_numaux(&self) -> u8`

- `fn name_offset(&self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

#### Provided Methods

- `fn section(&self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_null(&self) -> bool`

  Return true if the symbol is a null placeholder.

- `fn is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- `fn has_aux_file(&self) -> bool`

  Return true if the symbol has file auxiliary entry.

- `fn has_aux_csect(&self) -> bool`

  Return true if the symbol has csect auxiliary entry.

#### Implementors

- [`Symbol32`](../../xcoff/index.md)
- [`Symbol64`](../../xcoff/index.md)

### `FileAux`

```rust
trait FileAux: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:687-720`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L687-L720)*

A trait for generic access to [`xcoff::FileAux32`](../../xcoff/index.md) and [`xcoff::FileAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_fname(&self) -> &[u8; 8]`

- `fn x_ftype(&self) -> u8`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn name_offset(&self) -> Option<u32>`

- `fn fname<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse the x_fname field, which may be an inline string or a string table offset.

#### Implementors

- [`FileAux32`](../../xcoff/index.md)
- [`FileAux64`](../../xcoff/index.md)

### `CsectAux`

```rust
trait CsectAux: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:752-768`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L752-L768)*

A trait for generic access to [`xcoff::CsectAux32`](../../xcoff/index.md) and [`xcoff::CsectAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_scnlen(&self) -> u64`

- `fn x_parmhash(&self) -> u32`

- `fn x_snhash(&self) -> u16`

- `fn x_smtyp(&self) -> u8`

- `fn x_smclas(&self) -> u8`

- `fn x_stab(&self) -> Option<u32>`

- `fn x_snstab(&self) -> Option<u16>`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn alignment(&self) -> u8`

- `fn sym_type(&self) -> u8`

#### Implementors

- [`CsectAux32`](../../xcoff/index.md)
- [`CsectAux64`](../../xcoff/index.md)

### `Rel`

```rust
trait Rel: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:88-98`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/relocation.rs#L88-L98)*

A trait for generic access to [`xcoff::Rel32`](../../xcoff/index.md) and [`xcoff::Rel64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn r_vaddr(&self) -> <Self as >::Word`

- `fn r_symndx(&self) -> u32`

- `fn r_rsize(&self) -> u8`

- `fn r_rtype(&self) -> u8`

#### Provided Methods

- `fn symbol(&self) -> SymbolIndex`

#### Implementors

- [`Rel32`](../../xcoff/index.md)
- [`Rel64`](../../xcoff/index.md)

## Type Aliases

### `XcoffFile32<'data, R>`

```rust
type XcoffFile32<'data, R> = XcoffFile<'data, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:24`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/file.rs#L24)*

A 32-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader32`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff32`](../../index.md).

### `XcoffFile64<'data, R>`

```rust
type XcoffFile64<'data, R> = XcoffFile<'data, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:29`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/file.rs#L29)*

A 64-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader64`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff64`](../../index.md).

### `XcoffSectionIterator32<'data, 'file, R>`

```rust
type XcoffSectionIterator32<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:15-16`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L15-L16)*

An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSectionIterator64<'data, 'file, R>`

```rust
type XcoffSectionIterator64<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:18-19`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L18-L19)*

An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSection32<'data, 'file, R>`

```rust
type XcoffSection32<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:49-50`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L49-L50)*

A section in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSection64<'data, 'file, R>`

```rust
type XcoffSection64<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:52-53`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/section.rs#L52-L53)*

A section in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolTable32<'data, 'file, R>`

```rust
type XcoffSymbolTable32<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:209-210`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L209-L210)*

A symbol table in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolTable64<'data, 'file, R>`

```rust
type XcoffSymbolTable64<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:212-213`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L212-L213)*

A symbol table in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolIterator32<'data, 'file, R>`

```rust
type XcoffSymbolIterator32<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:256-257`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L256-L257)*

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolIterator64<'data, 'file, R>`

```rust
type XcoffSymbolIterator64<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:259-260`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L259-L260)*

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbol32<'data, 'file, R>`

```rust
type XcoffSymbol32<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:297-298`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L297-L298)*

A symbol in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbol64<'data, 'file, R>`

```rust
type XcoffSymbol64<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:300-301`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L300-L301)*

A symbol in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffRelocationIterator32<'data, 'file, R>`

```rust
type XcoffRelocationIterator32<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:16-17`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/relocation.rs#L16-L17)*

An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).

### `XcoffRelocationIterator64<'data, 'file, R>`

```rust
type XcoffRelocationIterator64<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:19-20`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/relocation.rs#L19-L20)*

An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).

### `XcoffComdatIterator32<'data, 'file, R>`

```rust
type XcoffComdatIterator32<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:11-12`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L11-L12)*

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatIterator64<'data, 'file, R>`

```rust
type XcoffComdatIterator64<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:14-15`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L14-L15)*

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdat32<'data, 'file, R>`

```rust
type XcoffComdat32<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:44-45`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L44-L45)*

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdat64<'data, 'file, R>`

```rust
type XcoffComdat64<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:48-49`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L48-L49)*

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdatSectionIterator32<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator32<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:105-106`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L105-L106)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatSectionIterator64<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator64<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:108-109`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/comdat.rs#L108-L109)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegmentIterator32<'data, 'file, R>`

```rust
type XcoffSegmentIterator32<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:12-13`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/segment.rs#L12-L13)*

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegmentIterator64<'data, 'file, R>`

```rust
type XcoffSegmentIterator64<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:15-16`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/segment.rs#L15-L16)*

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegment32<'data, 'file, R>`

```rust
type XcoffSegment32<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:44-45`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/segment.rs#L44-L45)*

A segment in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegment64<'data, 'file, R>`

```rust
type XcoffSegment64<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:47-48`](../../../../.source_1765210505/object-0.37.3/src/read/xcoff/segment.rs#L47-L48)*

A segment in an [`XcoffFile64`](super::XcoffFile64).

