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

## Contents

- [Modules](#modules)
  - [`file`](#file)
  - [`section`](#section)
  - [`data_directory`](#data_directory)
  - [`export`](#export)
  - [`import`](#import)
  - [`relocation`](#relocation)
  - [`resource`](#resource)
  - [`rich`](#rich)
- [Structs](#structs)
  - [`SectionTable`](#sectiontable)
  - [`SymbolTable`](#symboltable)
  - [`PeFile`](#pefile)
  - [`PeComdatIterator`](#pecomdatiterator)
  - [`PeComdat`](#pecomdat)
  - [`PeComdatSectionIterator`](#pecomdatsectioniterator)
  - [`PeSegmentIterator`](#pesegmentiterator)
  - [`PeSegment`](#pesegment)
  - [`PeSectionIterator`](#pesectioniterator)
  - [`PeSection`](#pesection)
  - [`PeRelocationIterator`](#perelocationiterator)
  - [`DataDirectories`](#datadirectories)
  - [`Export`](#export)
  - [`ExportTable`](#exporttable)
  - [`ImportTable`](#importtable)
  - [`ImportDescriptorIterator`](#importdescriptoriterator)
  - [`ImportThunkList`](#importthunklist)
  - [`DelayLoadImportTable`](#delayloadimporttable)
  - [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)
  - [`RelocationBlockIterator`](#relocationblockiterator)
  - [`RelocationIterator`](#relocationiterator)
  - [`Relocation`](#relocation)
  - [`ResourceDirectory`](#resourcedirectory)
  - [`ResourceDirectoryTable`](#resourcedirectorytable)
  - [`ResourceName`](#resourcename)
  - [`RichHeaderInfo`](#richheaderinfo)
  - [`RichHeaderEntry`](#richheaderentry)
- [Enums](#enums)
  - [`ExportTarget`](#exporttarget)
  - [`Import`](#import)
  - [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata)
  - [`ResourceNameOrId`](#resourcenameorid)
- [Traits](#traits)
  - [`ImageNtHeaders`](#imagentheaders)
  - [`ImageOptionalHeader`](#imageoptionalheader)
  - [`ImageThunkData`](#imagethunkdata)
- [Functions](#functions)
  - [`optional_header_magic`](#optional_header_magic)
  - [`parse_ordinal`](#parse_ordinal)
  - [`memmem`](#memmem)
- [Type Aliases](#type-aliases)
  - [`PeFile32`](#pefile32)
  - [`PeFile64`](#pefile64)
  - [`PeComdatIterator32`](#pecomdatiterator32)
  - [`PeComdatIterator64`](#pecomdatiterator64)
  - [`PeComdat32`](#pecomdat32)
  - [`PeComdat64`](#pecomdat64)
  - [`PeComdatSectionIterator32`](#pecomdatsectioniterator32)
  - [`PeComdatSectionIterator64`](#pecomdatsectioniterator64)
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
| [`file`](#file) | mod |  |
| [`section`](#section) | mod |  |
| [`data_directory`](#data_directory) | mod |  |
| [`export`](#export) | mod |  |
| [`import`](#import) | mod |  |
| [`relocation`](#relocation) | mod |  |
| [`resource`](#resource) | mod |  |
| [`rich`](#rich) | mod | PE rich header handling |
| [`SectionTable`](#sectiontable) | struct |  |
| [`SymbolTable`](#symboltable) | struct |  |
| [`PeFile`](#pefile) | struct | A PE image file. |
| [`PeComdatIterator`](#pecomdatiterator) | struct | An iterator for the COMDAT section groups in a [`PeFile`]. |
| [`PeComdat`](#pecomdat) | struct | A COMDAT section group in a [`PeFile`]. |
| [`PeComdatSectionIterator`](#pecomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`PeFile`]. |
| [`PeSegmentIterator`](#pesegmentiterator) | struct | An iterator for the loadable sections in a [`PeFile`]. |
| [`PeSegment`](#pesegment) | struct | A loadable section in a [`PeFile`]. |
| [`PeSectionIterator`](#pesectioniterator) | struct | An iterator for the sections in a [`PeFile`]. |
| [`PeSection`](#pesection) | struct | A section in a [`PeFile`]. |
| [`PeRelocationIterator`](#perelocationiterator) | struct | An iterator for the relocations in an [`PeSection`]. |
| [`DataDirectories`](#datadirectories) | struct | The table of data directories in a PE file. |
| [`Export`](#export) | struct | An export from a PE file. |
| [`ExportTable`](#exporttable) | struct | A partially parsed PE export table. |
| [`ImportTable`](#importtable) | struct | Information for parsing a PE import table. |
| [`ImportDescriptorIterator`](#importdescriptoriterator) | struct | A fallible iterator for the descriptors in the import data directory. |
| [`ImportThunkList`](#importthunklist) | struct | A list of import thunks. |
| [`DelayLoadImportTable`](#delayloadimporttable) | struct | Information for parsing a PE delay-load import table. |
| [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator) | struct | A fallible iterator for the descriptors in the delay-load data directory. |
| [`RelocationBlockIterator`](#relocationblockiterator) | struct | An iterator over the relocation blocks in the `.reloc` section of a PE file. |
| [`RelocationIterator`](#relocationiterator) | struct | An iterator of the relocations in a block in the `.reloc` section of a PE file. |
| [`Relocation`](#relocation) | struct | A relocation in the `.reloc` section of a PE file. |
| [`ResourceDirectory`](#resourcedirectory) | struct | The `.rsrc` section of a PE file. |
| [`ResourceDirectoryTable`](#resourcedirectorytable) | struct | A table of resource entries. |
| [`ResourceName`](#resourcename) | struct | A resource name. |
| [`RichHeaderInfo`](#richheaderinfo) | struct | Parsed information about a Rich Header. |
| [`RichHeaderEntry`](#richheaderentry) | struct | A PE rich header entry after it has been unmasked. |
| [`ExportTarget`](#exporttarget) | enum | Where an export is pointing to. |
| [`Import`](#import) | enum | A parsed import thunk. |
| [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata) | enum | Data associated with a resource directory entry. |
| [`ResourceNameOrId`](#resourcenameorid) | enum | A resource name or ID. |
| [`ImageNtHeaders`](#imagentheaders) | trait | A trait for generic access to [`pe::ImageNtHeaders32`] and [`pe::ImageNtHeaders64`]. |
| [`ImageOptionalHeader`](#imageoptionalheader) | trait | A trait for generic access to [`pe::ImageOptionalHeader32`] and [`pe::ImageOptionalHeader64`]. |
| [`ImageThunkData`](#imagethunkdata) | trait | A trait for generic access to [`pe::ImageThunkData32`] and [`pe::ImageThunkData64`]. |
| [`optional_header_magic`](#optional_header_magic) | fn | Find the optional header and read its `magic` field. |
| [`parse_ordinal`](#parse_ordinal) | fn |  |
| [`memmem`](#memmem) | fn | Find the offset of the first occurrence of needle in the data. |
| [`PeFile32`](#pefile32) | type | A PE32 (32-bit) image file. |
| [`PeFile64`](#pefile64) | type | A PE32+ (64-bit) image file. |
| [`PeComdatIterator32`](#pecomdatiterator32) | type | An iterator for the COMDAT section groups in a [`PeFile32`]. |
| [`PeComdatIterator64`](#pecomdatiterator64) | type | An iterator for the COMDAT section groups in a [`PeFile64`]. |
| [`PeComdat32`](#pecomdat32) | type | A COMDAT section group in a [`PeFile32`]. |
| [`PeComdat64`](#pecomdat64) | type | A COMDAT section group in a [`PeFile64`]. |
| [`PeComdatSectionIterator32`](#pecomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`PeFile32`]. |
| [`PeComdatSectionIterator64`](#pecomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`PeFile64`]. |
| [`PeSegmentIterator32`](#pesegmentiterator32) | type | An iterator for the loadable sections in a [`PeFile32`](super::PeFile32). |
| [`PeSegmentIterator64`](#pesegmentiterator64) | type | An iterator for the loadable sections in a [`PeFile64`](super::PeFile64). |
| [`PeSegment32`](#pesegment32) | type | A loadable section in a [`PeFile32`](super::PeFile32). |
| [`PeSegment64`](#pesegment64) | type | A loadable section in a [`PeFile64`](super::PeFile64). |
| [`PeSectionIterator32`](#pesectioniterator32) | type | An iterator for the sections in a [`PeFile32`](super::PeFile32). |
| [`PeSectionIterator64`](#pesectioniterator64) | type | An iterator for the sections in a [`PeFile64`](super::PeFile64). |
| [`PeSection32`](#pesection32) | type | A section in a [`PeFile32`](super::PeFile32). |
| [`PeSection64`](#pesection64) | type | A section in a [`PeFile64`](super::PeFile64). |

## Modules

- [`file`](file/index.md)
- [`section`](section/index.md)
- [`data_directory`](data_directory/index.md)
- [`export`](export/index.md)
- [`import`](import/index.md)
- [`relocation`](relocation/index.md)
- [`resource`](resource/index.md)
- [`rich`](rich/index.md) — PE rich header handling

## Structs

### `SectionTable<'data>`

```rust
struct SectionTable<'data> {
    sections: &'data [pe::ImageSectionHeader],
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:19-21`](../../../../.source_1765210505/object-0.37.3/src/read/coff/section.rs#L19-L21)*

The table of section headers in a COFF or PE file.

Returned by `CoffHeader::sections` and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../index.md)

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../pe/index.md)

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../index.md), [`ImageSectionHeader`](../../pe/index.md)

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`ImageSectionHeader`](../../pe/index.md)

- <span id="sectiontable-section-by-name"></span>`fn section_by_name<R: ReadRef<'data>>(&self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../index.md), [`SectionIndex`](../../index.md), [`ImageSectionHeader`](../../pe/index.md)

- <span id="sectiontable-max-section-file-offset"></span>`fn max_section_file_offset(&self) -> u64`

#### Trait Implementations

##### `impl Clone for SectionTable<'data>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data>` — [`SectionTable`](../coff/index.md)

##### `impl Copy for SectionTable<'data>`

##### `impl Debug for SectionTable<'data>`

- <span id="sectiontable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionTable<'data>`

- <span id="sectiontable-default"></span>`fn default() -> SectionTable<'data>` — [`SectionTable`](../coff/index.md)

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

*Defined in [`object-0.37.3/src/read/coff/symbol.rs:24-31`](../../../../.source_1765210505/object-0.37.3/src/read/coff/symbol.rs#L24-L31)*

A table of symbol entries in a COFF or PE file.

Also includes the string table used for the symbol names.

Returned by `CoffHeader::symbols` and
[`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols).

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../index.md)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](../coff/index.md)

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`CoffHeader`](../coff/index.md)

- <span id="symboltable-aux-function"></span>`fn aux_function(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ImageAuxSymbolFunction`](../../pe/index.md)

- <span id="symboltable-aux-section"></span>`fn aux_section(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ImageAuxSymbolSection`](../../pe/index.md)

- <span id="symboltable-aux-weak-external"></span>`fn aux_weak_external(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ImageAuxSymbolWeak`](../../pe/index.md)

- <span id="symboltable-aux-file-name"></span>`fn aux_file_name(&self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md)

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md)

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../index.md)

#### Trait Implementations

##### `impl<'data, R, Coff> Debug for SymbolTable<'data, R, Coff>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, R: ReadRef<'data>, Coff: CoffHeader> Default for SymbolTable<'data, R, Coff>`

- <span id="symboltable-default"></span>`fn default() -> Self`

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:37-47`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L37-L47)*

A PE image file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="pefile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- <span id="pefile-data"></span>`fn data(&self) -> R`

- <span id="pefile-dos-header"></span>`fn dos_header(&self) -> &'data pe::ImageDosHeader` — [`ImageDosHeader`](../../pe/index.md)

- <span id="pefile-nt-headers"></span>`fn nt_headers(&self) -> &'data Pe`

- <span id="pefile-rich-header-info"></span>`fn rich_header_info(&self) -> Option<RichHeaderInfo<'_>>` — [`RichHeaderInfo`](#richheaderinfo)

- <span id="pefile-section-table"></span>`fn section_table(&self) -> SectionTable<'data>` — [`SectionTable`](../coff/index.md)

- <span id="pefile-data-directories"></span>`fn data_directories(&self) -> DataDirectories<'data>` — [`DataDirectories`](#datadirectories)

- <span id="pefile-data-directory"></span>`fn data_directory(&self, id: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md)

- <span id="pefile-export-table"></span>`fn export_table(&self) -> Result<Option<ExportTable<'data>>>` — [`Result`](../../index.md), [`ExportTable`](#exporttable)

- <span id="pefile-import-table"></span>`fn import_table(&self) -> Result<Option<ImportTable<'data>>>` — [`Result`](../../index.md), [`ImportTable`](#importtable)

- <span id="pefile-section-alignment"></span>`fn section_alignment(&self) -> u64`

#### Trait Implementations

##### `impl<'data, Pe, R> Debug for PeFile<'data, Pe, R>`

- <span id="pefile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Pe, R> Object for PeFile<'data, Pe, R>`

- <span id="pefile-type-segment"></span>`type Segment = PeSegment<'data, 'file, Pe, R>`

- <span id="pefile-type-segmentiterator"></span>`type SegmentIterator = PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pefile-type-section"></span>`type Section = PeSection<'data, 'file, Pe, R>`

- <span id="pefile-type-sectioniterator"></span>`type SectionIterator = PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pefile-type-comdat"></span>`type Comdat = PeComdat<'data, 'file, Pe, R>`

- <span id="pefile-type-comdatiterator"></span>`type ComdatIterator = PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pefile-type-symbol"></span>`type Symbol = CoffSymbol<'data, 'file, R>`

- <span id="pefile-type-symboliterator"></span>`type SymbolIterator = CoffSymbolIterator<'data, 'file, R>`

- <span id="pefile-type-symboltable"></span>`type SymbolTable = CoffSymbolTable<'data, 'file, R>`

- <span id="pefile-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="pefile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md)

- <span id="pefile-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md)

- <span id="pefile-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="pefile-is-64"></span>`fn is_64(&self) -> bool`

- <span id="pefile-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- <span id="pefile-segments"></span>`fn segments(&self) -> PeSegmentIterator<'data, '_, Pe, R>` — [`PeSegmentIterator`](#pesegmentiterator)

- <span id="pefile-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<PeSection<'data, 'file, Pe, R>>` — [`PeSection`](#pesection)

- <span id="pefile-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<PeSection<'data, '_, Pe, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`PeSection`](#pesection)

- <span id="pefile-sections"></span>`fn sections(&self) -> PeSectionIterator<'data, '_, Pe, R>` — [`PeSectionIterator`](#pesectioniterator)

- <span id="pefile-comdats"></span>`fn comdats(&self) -> PeComdatIterator<'data, '_, Pe, R>` — [`PeComdatIterator`](#pecomdatiterator)

- <span id="pefile-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`CoffSymbol`](../coff/index.md)

- <span id="pefile-symbols"></span>`fn symbols(&self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../coff/index.md)

- <span id="pefile-symbol-table"></span>`fn symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../coff/index.md)

- <span id="pefile-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../coff/index.md)

- <span id="pefile-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../coff/index.md)

- <span id="pefile-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md)

- <span id="pefile-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- <span id="pefile-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- <span id="pefile-pdb-info"></span>`fn pdb_info(&self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../index.md), [`CodeView`](../../index.md)

- <span id="pefile-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="pefile-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="pefile-entry"></span>`fn entry(&self) -> u64`

- <span id="pefile-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md)

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:432-439`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L432-L439)*

An iterator for the COMDAT section groups in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pecomdatiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pecomdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-type-item"></span>`type Item = PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `PeComdat<'data, 'file, Pe, R>`

```rust
struct PeComdat<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:465-472`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L465-L472)*

A COMDAT section group in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectComdat for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-type-sectioniterator"></span>`type SectionIterator = PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- <span id="pecomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- <span id="pecomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="pecomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="pecomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:525-532`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L525-L532)*

An iterator for the sections in a COMDAT section group in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pecomdatsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pecomdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-type-item"></span>`type Item = SectionIndex`

- <span id="pecomdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:23-30`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L23-L30)*

An iterator for the loadable sections in a [`PeFile`](#pefile).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesegmentiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pesegmentiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-type-item"></span>`type Item = PeSegment<'data, 'file, Pe, R>`

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:58-65`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L58-L65)*

A loadable section in a [`PeFile`](#pefile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- <span id="pesegment-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](#pefile)

- <span id="pesegment-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSegment for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-address"></span>`fn address(&self) -> u64`

- <span id="pesegment-size"></span>`fn size(&self) -> u64`

- <span id="pesegment-align"></span>`fn align(&self) -> u64`

- <span id="pesegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="pesegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="pesegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="pesegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="pesegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="pesegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:162-169`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L162-L169)*

An iterator for the sections in a [`PeFile`](#pefile).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pesectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-type-item"></span>`type Item = PeSection<'data, 'file, Pe, R>`

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:198-206`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L198-L206)*

A section in a [`PeFile`](#pefile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="pesection-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](#pefile)

- <span id="pesection-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSection for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-type-relocationiterator"></span>`type RelocationIterator = PeRelocationIterator<'data, 'file, R>`

- <span id="pesection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- <span id="pesection-address"></span>`fn address(&self) -> u64`

- <span id="pesection-size"></span>`fn size(&self) -> u64`

- <span id="pesection-align"></span>`fn align(&self) -> u64`

- <span id="pesection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="pesection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="pesection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="pesection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- <span id="pesection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- <span id="pesection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="pesection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="pesection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="pesection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="pesection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md)

- <span id="pesection-relocations"></span>`fn relocations(&self) -> PeRelocationIterator<'data, 'file, R>` — [`PeRelocationIterator`](#perelocationiterator)

- <span id="pesection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- <span id="pesection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeSection<'data, 'file, Pe, R>`

### `PeRelocationIterator<'data, 'file, R>`

```rust
struct PeRelocationIterator<'data, 'file, R>(core::marker::PhantomData<(&'data (), &'file (), R)>);
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:466-468`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L466-L468)*

An iterator for the relocations in an [`PeSection`](#pesection).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug> Debug for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="perelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="perelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R> Iterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="perelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `DataDirectories<'data>`

```rust
struct DataDirectories<'data> {
    entries: &'data [pe::ImageDataDirectory],
}
```

*Defined in [`object-0.37.3/src/read/pe/data_directory.rs:16-18`](../../../../.source_1765210505/object-0.37.3/src/read/pe/data_directory.rs#L16-L18)*

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

#### Implementations

- <span id="datadirectories-parse"></span>`fn parse(data: &'data [u8], number: u32) -> Result<Self>` — [`Result`](../../index.md)

- <span id="datadirectories-len"></span>`fn len(&self) -> usize`

- <span id="datadirectories-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md)

- <span id="datadirectories-enumerate"></span>`fn enumerate(&self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` — [`ImageDataDirectory`](../../pe/index.md)

- <span id="datadirectories-get"></span>`fn get(&self, index: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md)

- <span id="datadirectories-export-directory"></span>`fn export_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` — [`SectionTable`](../coff/index.md), [`Result`](../../index.md), [`ImageExportDirectory`](../../pe/index.md)

- <span id="datadirectories-export-table"></span>`fn export_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` — [`SectionTable`](../coff/index.md), [`Result`](../../index.md), [`ExportTable`](#exporttable)

- <span id="datadirectories-import-table"></span>`fn import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` — [`SectionTable`](../coff/index.md), [`Result`](../../index.md), [`ImportTable`](#importtable)

- <span id="datadirectories-delay-load-import-table"></span>`fn delay_load_import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` — [`SectionTable`](../coff/index.md), [`Result`](../../index.md), [`DelayLoadImportTable`](#delayloadimporttable)

- <span id="datadirectories-relocation-blocks"></span>`fn relocation_blocks<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` — [`SectionTable`](../coff/index.md), [`Result`](../../index.md), [`RelocationBlockIterator`](#relocationblockiterator)

- <span id="datadirectories-resource-directory"></span>`fn resource_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` — [`SectionTable`](../coff/index.md), [`Result`](../../index.md), [`ResourceDirectory`](#resourcedirectory)

#### Trait Implementations

##### `impl Clone for DataDirectories<'data>`

- <span id="datadirectories-clone"></span>`fn clone(&self) -> DataDirectories<'data>` — [`DataDirectories`](#datadirectories)

##### `impl Copy for DataDirectories<'data>`

##### `impl Debug for DataDirectories<'data>`

- <span id="datadirectories-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Export<'data>`

```rust
struct Export<'data> {
    pub ordinal: u32,
    pub name: Option<&'data [u8]>,
    pub target: ExportTarget<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:42-51`](../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L42-L51)*

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

##### `impl Clone for Export<'data>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'data>` — [`Export`](#export)

##### `impl Copy for Export<'data>`

##### `impl Debug for Export<'a>`

- <span id="export-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/export.rs:87-94`](../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L87-L94)*

A partially parsed PE export table.

Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table).

#### Implementations

- <span id="exporttable-parse"></span>`fn parse(data: &'data [u8], virtual_address: u32) -> Result<Self>` — [`Result`](../../index.md)

- <span id="exporttable-parse-directory"></span>`fn parse_directory(data: &'data [u8]) -> Result<&'data pe::ImageExportDirectory>` — [`Result`](../../index.md), [`ImageExportDirectory`](../../pe/index.md)

- <span id="exporttable-directory"></span>`fn directory(&self) -> &'data pe::ImageExportDirectory` — [`ImageExportDirectory`](../../pe/index.md)

- <span id="exporttable-ordinal-base"></span>`fn ordinal_base(&self) -> u32`

- <span id="exporttable-addresses"></span>`fn addresses(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- <span id="exporttable-name-pointers"></span>`fn name_pointers(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- <span id="exporttable-name-ordinals"></span>`fn name_ordinals(&self) -> &'data [U16Bytes<LE>]` — [`U16Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- <span id="exporttable-name-iter"></span>`fn name_iter(&self) -> impl Iterator<Item = (u32, u16)> + 'data`

- <span id="exporttable-address-by-index"></span>`fn address_by_index(&self, index: u32) -> Result<u32>` — [`Result`](../../index.md)

- <span id="exporttable-address-by-ordinal"></span>`fn address_by_ordinal(&self, ordinal: u32) -> Result<u32>` — [`Result`](../../index.md)

- <span id="exporttable-target-by-index"></span>`fn target_by_index(&self, index: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md), [`ExportTarget`](#exporttarget)

- <span id="exporttable-target-by-ordinal"></span>`fn target_by_ordinal(&self, ordinal: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md), [`ExportTarget`](#exporttarget)

- <span id="exporttable-target-from-address"></span>`fn target_from_address(&self, address: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md), [`ExportTarget`](#exporttarget)

- <span id="exporttable-forward-offset"></span>`fn forward_offset(&self, address: u32) -> Option<usize>`

- <span id="exporttable-is-forward"></span>`fn is_forward(&self, address: u32) -> bool`

- <span id="exporttable-forward-string"></span>`fn forward_string(&self, address: u32) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="exporttable-name-from-pointer"></span>`fn name_from_pointer(&self, name_pointer: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="exporttable-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](#export)

#### Trait Implementations

##### `impl Clone for ExportTable<'data>`

- <span id="exporttable-clone"></span>`fn clone(&self) -> ExportTable<'data>` — [`ExportTable`](#exporttable)

##### `impl Debug for ExportTable<'data>`

- <span id="exporttable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportTable<'data>`

```rust
struct ImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:15-19`](../../../../.source_1765210505/object-0.37.3/src/read/pe/import.rs#L15-L19)*

Information for parsing a PE import table.

Returned by [`DataDirectories::import_table`](super::DataDirectories::import_table).

#### Implementations

- <span id="importtable-new"></span>`fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- <span id="importtable-descriptors"></span>`fn descriptors(&self) -> Result<ImportDescriptorIterator<'data>>` — [`Result`](../../index.md), [`ImportDescriptorIterator`](#importdescriptoriterator)

- <span id="importtable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="importtable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../index.md), [`ImportThunkList`](#importthunklist)

- <span id="importtable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](#imagentheaders), [`Result`](../../index.md), [`Import`](#import)

- <span id="importtable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl Clone for ImportTable<'data>`

- <span id="importtable-clone"></span>`fn clone(&self) -> ImportTable<'data>` — [`ImportTable`](#importtable)

##### `impl Debug for ImportTable<'data>`

- <span id="importtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportDescriptorIterator<'data>`

```rust
struct ImportDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:102-105`](../../../../.source_1765210505/object-0.37.3/src/read/pe/import.rs#L102-L105)*

A fallible iterator for the descriptors in the import data directory.

#### Implementations

- <span id="importdescriptoriterator-next"></span>`fn next(&mut self) -> Result<Option<&'data pe::ImageImportDescriptor>>` — [`Result`](../../index.md), [`ImageImportDescriptor`](../../pe/index.md)

#### Trait Implementations

##### `impl Clone for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-clone"></span>`fn clone(&self) -> ImportDescriptorIterator<'data>` — [`ImportDescriptorIterator`](#importdescriptoriterator)

##### `impl Debug for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="importdescriptoriterator-type-intoiter"></span>`type IntoIter = I`

- <span id="importdescriptoriterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-type-item"></span>`type Item = Result<&'data ImageImportDescriptor, Error>`

- <span id="importdescriptoriterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ImportThunkList<'data>`

```rust
struct ImportThunkList<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:148-150`](../../../../.source_1765210505/object-0.37.3/src/read/pe/import.rs#L148-L150)*

A list of import thunks.

These may be in the import lookup table, or the import address table.

#### Implementations

- <span id="importthunklist-get"></span>`fn get<Pe: ImageNtHeaders>(&self, index: usize) -> Result<<Pe as >::ImageThunkData>` — [`Result`](../../index.md), [`ImageNtHeaders`](#imagentheaders)

- <span id="importthunklist-next"></span>`fn next<Pe: ImageNtHeaders>(&mut self) -> Result<Option<<Pe as >::ImageThunkData>>` — [`Result`](../../index.md), [`ImageNtHeaders`](#imagentheaders)

#### Trait Implementations

##### `impl Clone for ImportThunkList<'data>`

- <span id="importthunklist-clone"></span>`fn clone(&self) -> ImportThunkList<'data>` — [`ImportThunkList`](#importthunklist)

##### `impl Debug for ImportThunkList<'data>`

- <span id="importthunklist-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DelayLoadImportTable<'data>`

```rust
struct DelayLoadImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:250-254`](../../../../.source_1765210505/object-0.37.3/src/read/pe/import.rs#L250-L254)*

Information for parsing a PE delay-load import table.

Returned by
[`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table).

#### Implementations

- <span id="delayloadimporttable-new"></span>`fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

- <span id="delayloadimporttable-descriptors"></span>`fn descriptors(&self) -> Result<DelayLoadDescriptorIterator<'data>>` — [`Result`](../../index.md), [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)

- <span id="delayloadimporttable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="delayloadimporttable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../index.md), [`ImportThunkList`](#importthunklist)

- <span id="delayloadimporttable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](#imagentheaders), [`Result`](../../index.md), [`Import`](#import)

- <span id="delayloadimporttable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl Clone for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-clone"></span>`fn clone(&self) -> DelayLoadImportTable<'data>` — [`DelayLoadImportTable`](#delayloadimporttable)

##### `impl Debug for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DelayLoadDescriptorIterator<'data>`

```rust
struct DelayLoadDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:341-344`](../../../../.source_1765210505/object-0.37.3/src/read/pe/import.rs#L341-L344)*

A fallible iterator for the descriptors in the delay-load data directory.

#### Implementations

- <span id="delayloaddescriptoriterator-next"></span>`fn next(&mut self) -> Result<Option<&'data pe::ImageDelayloadDescriptor>>` — [`Result`](../../index.md), [`ImageDelayloadDescriptor`](../../pe/index.md)

#### Trait Implementations

##### `impl Clone for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-clone"></span>`fn clone(&self) -> DelayLoadDescriptorIterator<'data>` — [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)

##### `impl Debug for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="delayloaddescriptoriterator-type-intoiter"></span>`type IntoIter = I`

- <span id="delayloaddescriptoriterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-type-item"></span>`type Item = Result<&'data ImageDelayloadDescriptor, Error>`

- <span id="delayloaddescriptoriterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RelocationBlockIterator<'data>`

```rust
struct RelocationBlockIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:11-13`](../../../../.source_1765210505/object-0.37.3/src/read/pe/relocation.rs#L11-L13)*

An iterator over the relocation blocks in the `.reloc` section of a PE file.

Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks).

#### Implementations

- <span id="relocationblockiterator-new"></span>`fn new(data: &'data [u8]) -> Self`

- <span id="relocationblockiterator-next"></span>`fn next(&mut self) -> Result<Option<RelocationIterator<'data>>>` — [`Result`](../../index.md), [`RelocationIterator`](#relocationiterator)

- <span id="relocationblockiterator-parse"></span>`fn parse(&mut self) -> Result<RelocationIterator<'data>>` — [`Result`](../../index.md), [`RelocationIterator`](#relocationiterator)

#### Trait Implementations

##### `impl Clone for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-clone"></span>`fn clone(&self) -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](#relocationblockiterator)

##### `impl Copy for RelocationBlockIterator<'data>`

##### `impl Debug for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-default"></span>`fn default() -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](#relocationblockiterator)

##### `impl IntoIterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationblockiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relocationblockiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-type-item"></span>`type Item = Result<RelocationIterator<'data>, Error>`

- <span id="relocationblockiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RelocationIterator<'data>`

```rust
struct RelocationIterator<'data> {
    virtual_address: u32,
    size: u32,
    relocs: slice::Iter<'data, crate::endian::U16<crate::endian::LittleEndian>>,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:68-72`](../../../../.source_1765210505/object-0.37.3/src/read/pe/relocation.rs#L68-L72)*

An iterator of the relocations in a block in the `.reloc` section of a PE file.

#### Implementations

- <span id="relocationiterator-virtual-address"></span>`fn virtual_address(&self) -> u32`

- <span id="relocationiterator-size"></span>`fn size(&self) -> u32`

#### Trait Implementations

##### `impl Clone for RelocationIterator<'data>`

- <span id="relocationiterator-clone"></span>`fn clone(&self) -> RelocationIterator<'data>` — [`RelocationIterator`](#relocationiterator)

##### `impl Debug for RelocationIterator<'data>`

- <span id="relocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RelocationIterator<'data>`

- <span id="relocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RelocationIterator<'data>`

- <span id="relocationiterator-type-item"></span>`type Item = Relocation`

- <span id="relocationiterator-next"></span>`fn next(&mut self) -> Option<Relocation>` — [`Relocation`](#relocation)

### `Relocation`

```rust
struct Relocation {
    pub virtual_address: u32,
    pub typ: u16,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:104-109`](../../../../.source_1765210505/object-0.37.3/src/read/pe/relocation.rs#L104-L109)*

A relocation in the `.reloc` section of a PE file.

#### Fields

- **`virtual_address`**: `u32`

  The virtual address of the relocation.

- **`typ`**: `u16`

  One of the `pe::IMAGE_REL_BASED_*` constants.

#### Trait Implementations

##### `impl Clone for Relocation`

- <span id="relocation-clone"></span>`fn clone(&self) -> Relocation` — [`Relocation`](#relocation)

##### `impl Copy for Relocation`

##### `impl Debug for Relocation`

- <span id="relocation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Relocation`

- <span id="relocation-default"></span>`fn default() -> Relocation` — [`Relocation`](#relocation)

### `ResourceDirectory<'data>`

```rust
struct ResourceDirectory<'data> {
    data: &'data [u8],
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:12-14`](../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L12-L14)*

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

#### Implementations

- <span id="resourcedirectory-new"></span>`fn new(data: &'data [u8]) -> Self`

- <span id="resourcedirectory-root"></span>`fn root(&self) -> Result<ResourceDirectoryTable<'data>>` — [`Result`](../../index.md), [`ResourceDirectoryTable`](#resourcedirectorytable)

#### Trait Implementations

##### `impl Clone for ResourceDirectory<'data>`

- <span id="resourcedirectory-clone"></span>`fn clone(&self) -> ResourceDirectory<'data>` — [`ResourceDirectory`](#resourcedirectory)

##### `impl Copy for ResourceDirectory<'data>`

##### `impl Debug for ResourceDirectory<'data>`

- <span id="resourcedirectory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceDirectoryTable<'data>`

```rust
struct ResourceDirectoryTable<'data> {
    pub header: &'data pe::ImageResourceDirectory,
    pub entries: &'data [pe::ImageResourceDirectoryEntry],
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:30-35`](../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L30-L35)*

A table of resource entries.

#### Fields

- **`header`**: `&'data pe::ImageResourceDirectory`

  The table header.

- **`entries`**: `&'data [pe::ImageResourceDirectoryEntry]`

  The table entries.

#### Implementations

- <span id="resourcedirectorytable-parse"></span>`fn parse(data: &'data [u8], offset: u32) -> Result<Self>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl Clone for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-clone"></span>`fn clone(&self) -> ResourceDirectoryTable<'data>` — [`ResourceDirectoryTable`](#resourcedirectorytable)

##### `impl Debug for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceName`

```rust
struct ResourceName {
    offset: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:143-145`](../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L143-L145)*

A resource name.

#### Implementations

- <span id="resourcename-to-string-lossy"></span>`fn to_string_lossy(&self, directory: ResourceDirectory<'_>) -> Result<String>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md)

- <span id="resourcename-data"></span>`fn data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md), [`U16Bytes`](../../index.md), [`LittleEndian`](../../index.md)

- <span id="resourcename-raw-data"></span>`fn raw_data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md)

#### Trait Implementations

##### `impl Clone for ResourceName`

- <span id="resourcename-clone"></span>`fn clone(&self) -> ResourceName` — [`ResourceName`](#resourcename)

##### `impl Copy for ResourceName`

##### `impl Debug for ResourceName`

- <span id="resourcename-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RichHeaderInfo<'data>`

```rust
struct RichHeaderInfo<'data> {
    pub offset: usize,
    pub length: usize,
    pub xor_key: u32,
    masked_entries: &'data [pe::MaskedRichHeaderEntry],
}
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:12-26`](../../../../.source_1765210505/object-0.37.3/src/read/pe/rich.rs#L12-L26)*

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

- <span id="richheaderinfo-parse"></span>`fn parse<R: ReadRef<'data>>(data: R, nt_header_offset: u64) -> Option<Self>`

- <span id="richheaderinfo-unmasked-entries"></span>`fn unmasked_entries(&self) -> impl Iterator<Item = RichHeaderEntry> + 'data` — [`RichHeaderEntry`](#richheaderentry)

#### Trait Implementations

##### `impl Clone for RichHeaderInfo<'data>`

- <span id="richheaderinfo-clone"></span>`fn clone(&self) -> RichHeaderInfo<'data>` — [`RichHeaderInfo`](#richheaderinfo)

##### `impl Copy for RichHeaderInfo<'data>`

##### `impl Debug for RichHeaderInfo<'data>`

- <span id="richheaderinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RichHeaderEntry`

```rust
struct RichHeaderEntry {
    pub comp_id: u32,
    pub count: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:33-38`](../../../../.source_1765210505/object-0.37.3/src/read/pe/rich.rs#L33-L38)*

A PE rich header entry after it has been unmasked.

See [`pe::MaskedRichHeaderEntry`](../../pe/index.md).

#### Fields

- **`comp_id`**: `u32`

  ID of the component.

- **`count`**: `u32`

  Number of times this component has been used when building this PE.

#### Trait Implementations

##### `impl Clone for RichHeaderEntry`

- <span id="richheaderentry-clone"></span>`fn clone(&self) -> RichHeaderEntry` — [`RichHeaderEntry`](#richheaderentry)

##### `impl Copy for RichHeaderEntry`

##### `impl Debug for RichHeaderEntry`

- <span id="richheaderentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ExportTarget<'data>`

```rust
enum ExportTarget<'data> {
    Address(u32),
    ForwardByOrdinal(&'data [u8], u32),
    ForwardByName(&'data [u8], &'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:10-21`](../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L10-L21)*

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

- <span id="exporttarget-is-address"></span>`fn is_address(&self) -> bool`

- <span id="exporttarget-is-forward"></span>`fn is_forward(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ExportTarget<'data>`

- <span id="exporttarget-clone"></span>`fn clone(&self) -> ExportTarget<'data>` — [`ExportTarget`](#exporttarget)

##### `impl Copy for ExportTarget<'data>`

##### `impl Debug for ExportTarget<'a>`

- <span id="exporttarget-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

### `Import<'data>`

```rust
enum Import<'data> {
    Ordinal(u16),
    Name(u16, &'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:180-187`](../../../../.source_1765210505/object-0.37.3/src/read/pe/import.rs#L180-L187)*

A parsed import thunk.

#### Variants

- **`Ordinal`**

  Import by ordinal.

- **`Name`**

  Import by name.
  
  Includes a hint for the index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl Clone for Import<'data>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'data>` — [`Import`](#import)

##### `impl Copy for Import<'data>`

##### `impl Debug for Import<'data>`

- <span id="import-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceDirectoryEntryData<'data>`

```rust
enum ResourceDirectoryEntryData<'data> {
    Table(ResourceDirectoryTable<'data>),
    Data(&'data pe::ImageResourceDataEntry),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:112-117`](../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L112-L117)*

Data associated with a resource directory entry.

#### Variants

- **`Table`**

  A subtable entry.

- **`Data`**

  A resource data entry.

#### Implementations

- <span id="resourcedirectoryentrydata-table"></span>`fn table(self) -> Option<ResourceDirectoryTable<'data>>` — [`ResourceDirectoryTable`](#resourcedirectorytable)

- <span id="resourcedirectoryentrydata-data"></span>`fn data(self) -> Option<&'data pe::ImageResourceDataEntry>` — [`ImageResourceDataEntry`](../../pe/index.md)

#### Trait Implementations

##### `impl Clone for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-clone"></span>`fn clone(&self) -> ResourceDirectoryEntryData<'data>` — [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata)

##### `impl Debug for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ResourceNameOrId`

```rust
enum ResourceNameOrId {
    Name(ResourceName),
    Id(u16),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:183-188`](../../../../.source_1765210505/object-0.37.3/src/read/pe/resource.rs#L183-L188)*

A resource name or ID.

Can be either a string or a numeric ID.

#### Variants

- **`Name`**

  A resource name.

- **`Id`**

  A resource ID.

#### Implementations

- <span id="resourcenameorid-name"></span>`fn name(self) -> Option<ResourceName>` — [`ResourceName`](#resourcename)

- <span id="resourcenameorid-id"></span>`fn id(self) -> Option<u16>`

#### Trait Implementations

##### `impl Debug for ResourceNameOrId`

- <span id="resourcenameorid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `ImageNtHeaders`

```rust
trait ImageNtHeaders: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:589-671`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L589-L671)*

A trait for generic access to [`pe::ImageNtHeaders32`](../../pe/index.md) and [`pe::ImageNtHeaders64`](../../pe/index.md).

#### Associated Types

- `type ImageOptionalHeader: 1`

- `type ImageThunkData: 1`

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_valid_optional_magic(&self) -> bool`

  Return true if the magic field in the optional header is valid.

- `fn signature(&self) -> u32`

  Return the signature

- `fn file_header(&self) -> &pe::ImageFileHeader`

  Return the file header.

- `fn optional_header(&self) -> &<Self as >::ImageOptionalHeader`

  Return the optional header.

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<(&'data Self, DataDirectories<'data>)>`

  Read the NT headers, including the data directories.

- `fn sections<'data, R: ReadRef<'data>>(&self, data: R, offset: u64) -> read::Result<SectionTable<'data>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<SymbolTable<'data, R>>`

  Read the COFF symbol table and string table.

#### Implementors

- [`ImageNtHeaders32`](../../pe/index.md)
- [`ImageNtHeaders64`](../../pe/index.md)

### `ImageOptionalHeader`

```rust
trait ImageOptionalHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:675-709`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L675-L709)*

A trait for generic access to [`pe::ImageOptionalHeader32`](../../pe/index.md) and [`pe::ImageOptionalHeader64`](../../pe/index.md).

#### Required Methods

- `fn magic(&self) -> u16`

- `fn major_linker_version(&self) -> u8`

- `fn minor_linker_version(&self) -> u8`

- `fn size_of_code(&self) -> u32`

- `fn size_of_initialized_data(&self) -> u32`

- `fn size_of_uninitialized_data(&self) -> u32`

- `fn address_of_entry_point(&self) -> u32`

- `fn base_of_code(&self) -> u32`

- `fn base_of_data(&self) -> Option<u32>`

- `fn image_base(&self) -> u64`

- `fn section_alignment(&self) -> u32`

- `fn file_alignment(&self) -> u32`

- `fn major_operating_system_version(&self) -> u16`

- `fn minor_operating_system_version(&self) -> u16`

- `fn major_image_version(&self) -> u16`

- `fn minor_image_version(&self) -> u16`

- `fn major_subsystem_version(&self) -> u16`

- `fn minor_subsystem_version(&self) -> u16`

- `fn win32_version_value(&self) -> u32`

- `fn size_of_image(&self) -> u32`

- `fn size_of_headers(&self) -> u32`

- `fn check_sum(&self) -> u32`

- `fn subsystem(&self) -> u16`

- `fn dll_characteristics(&self) -> u16`

- `fn size_of_stack_reserve(&self) -> u64`

- `fn size_of_stack_commit(&self) -> u64`

- `fn size_of_heap_reserve(&self) -> u64`

- `fn size_of_heap_commit(&self) -> u64`

- `fn loader_flags(&self) -> u32`

- `fn number_of_rva_and_sizes(&self) -> u32`

#### Implementors

- [`ImageOptionalHeader32`](../../pe/index.md)
- [`ImageOptionalHeader64`](../../pe/index.md)

### `ImageThunkData`

```rust
trait ImageThunkData: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:191-207`](../../../../.source_1765210505/object-0.37.3/src/read/pe/import.rs#L191-L207)*

A trait for generic access to [`pe::ImageThunkData32`](../../pe/index.md) and [`pe::ImageThunkData64`](../../pe/index.md).

#### Required Methods

- `fn raw(self) -> u64`

  Return the raw thunk value.

- `fn is_ordinal(self) -> bool`

  Returns true if the ordinal flag is set.

- `fn ordinal(self) -> u16`

  Return the ordinal portion of the thunk.

- `fn address(self) -> u32`

  Return the RVA portion of the thunk.

#### Implementors

- [`ImageThunkData32`](../../pe/index.md)
- [`ImageThunkData64`](../../pe/index.md)

## Functions

### `optional_header_magic`

```rust
fn optional_header_magic<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<u16>
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:572-585`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L572-L585)*

Find the optional header and read its `magic` field.

It can be useful to know this magic value before trying to
fully parse the NT headers.

### `parse_ordinal`

```rust
fn parse_ordinal(digits: &[u8]) -> Option<u32>
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:324-334`](../../../../.source_1765210505/object-0.37.3/src/read/pe/export.rs#L324-L334)*

### `memmem`

```rust
fn memmem(data: &[u8], needle: &[u8], align: usize) -> Option<usize>
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:84-92`](../../../../.source_1765210505/object-0.37.3/src/read/pe/rich.rs#L84-L92)*

Find the offset of the first occurrence of needle in the data.

The offset must have the given alignment.

## Type Aliases

### `PeFile32<'data, R>`

```rust
type PeFile32<'data, R> = PeFile<'data, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:26`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L26)*

A PE32 (32-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders32`](../../pe/index.md), and corresponds
to [`crate::FileKind::Pe32`](../../index.md).

### `PeFile64<'data, R>`

```rust
type PeFile64<'data, R> = PeFile<'data, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:31`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L31)*

A PE32+ (64-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders64`](../../pe/index.md), and corresponds
to [`crate::FileKind::Pe64`](../../index.md).

### `PeComdatIterator32<'data, 'file, R>`

```rust
type PeComdatIterator32<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:422-423`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L422-L423)*

An iterator for the COMDAT section groups in a [`PeFile32`](#pefile32).

### `PeComdatIterator64<'data, 'file, R>`

```rust
type PeComdatIterator64<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:425-426`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L425-L426)*

An iterator for the COMDAT section groups in a [`PeFile64`](#pefile64).

### `PeComdat32<'data, 'file, R>`

```rust
type PeComdat32<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:455-456`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L455-L456)*

A COMDAT section group in a [`PeFile32`](#pefile32).

### `PeComdat64<'data, 'file, R>`

```rust
type PeComdat64<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:458-459`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L458-L459)*

A COMDAT section group in a [`PeFile64`](#pefile64).

### `PeComdatSectionIterator32<'data, 'file, R>`

```rust
type PeComdatSectionIterator32<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:515-516`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L515-L516)*

An iterator for the sections in a COMDAT section group in a [`PeFile32`](#pefile32).

### `PeComdatSectionIterator64<'data, 'file, R>`

```rust
type PeComdatSectionIterator64<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:518-519`](../../../../.source_1765210505/object-0.37.3/src/read/pe/file.rs#L518-L519)*

An iterator for the sections in a COMDAT section group in a [`PeFile64`](#pefile64).

### `PeSegmentIterator32<'data, 'file, R>`

```rust
type PeSegmentIterator32<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:15-16`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L15-L16)*

An iterator for the loadable sections in a [`PeFile32`](super::PeFile32).

### `PeSegmentIterator64<'data, 'file, R>`

```rust
type PeSegmentIterator64<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:18-19`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L18-L19)*

An iterator for the loadable sections in a [`PeFile64`](super::PeFile64).

### `PeSegment32<'data, 'file, R>`

```rust
type PeSegment32<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:48-49`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L48-L49)*

A loadable section in a [`PeFile32`](super::PeFile32).

### `PeSegment64<'data, 'file, R>`

```rust
type PeSegment64<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:51-52`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L51-L52)*

A loadable section in a [`PeFile64`](super::PeFile64).

### `PeSectionIterator32<'data, 'file, R>`

```rust
type PeSectionIterator32<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:154-155`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L154-L155)*

An iterator for the sections in a [`PeFile32`](super::PeFile32).

### `PeSectionIterator64<'data, 'file, R>`

```rust
type PeSectionIterator64<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:157-158`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L157-L158)*

An iterator for the sections in a [`PeFile64`](super::PeFile64).

### `PeSection32<'data, 'file, R>`

```rust
type PeSection32<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:188-189`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L188-L189)*

A section in a [`PeFile32`](super::PeFile32).

### `PeSection64<'data, 'file, R>`

```rust
type PeSection64<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:191-192`](../../../../.source_1765210505/object-0.37.3/src/read/pe/section.rs#L191-L192)*

A section in a [`PeFile64`](super::PeFile64).

