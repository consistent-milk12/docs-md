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
  - [`data_directory`](#data-directory)
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
  - [`optional_header_magic`](#optional-header-magic)
  - [`parse_ordinal`](#parse-ordinal)
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
| [`data_directory`](#data-directory) | mod |  |
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
| [`optional_header_magic`](#optional-header-magic) | fn | Find the optional header and read its `magic` field. |
| [`parse_ordinal`](#parse-ordinal) | fn |  |
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

*Defined in [`object-0.37.3/src/read/coff/section.rs:19-21`](../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L19-L21)*

The table of section headers in a COFF or PE file.

Returned by `CoffHeader::sections` and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the section table.

  

  `data` must be the entire file data.

  `offset` must be after the optional file header.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Iterate over the section headers.

  

  Warning: section indices start at 1.

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../index.md#sectionindex), [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Iterate over the section headers and their indices.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Return the section header at the given index.

  

  The index is 1-based.

- <span id="sectiontable-section-by-name"></span>`fn section_by_name<R: ReadRef<'data>>(&self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../index.md#stringtable), [`SectionIndex`](../../index.md#sectionindex), [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Return the section header with the given name.

  

  The returned index is 1-based.

  

  Ignores sections with invalid names.

- <span id="sectiontable-max-section-file-offset"></span>`fn max_section_file_offset(&self) -> u64`

  Compute the maximum file offset used by sections.

  

  This will usually match the end of file, unless the PE file has a

  [data overlay](https://security.stackexchange.com/questions/77336/how-is-the-file-overlay-read-by-an-exe-virus)

#### Trait Implementations

##### `impl Any for SectionTable<'data>`

- <span id="sectiontable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionTable<'data>`

- <span id="sectiontable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionTable<'data>`

- <span id="sectiontable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionTable<'data>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data>` — [`SectionTable`](../coff/index.md#sectiontable)

##### `impl CloneToUninit for SectionTable<'data>`

- <span id="sectiontable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionTable<'data>`

##### `impl Debug for SectionTable<'data>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionTable<'data>`

- <span id="sectiontable-default"></span>`fn default() -> SectionTable<'data>` — [`SectionTable`](../coff/index.md#sectiontable)

##### `impl<T> From for SectionTable<'data>`

- <span id="sectiontable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionTable<'data>`

- <span id="sectiontable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SectionTable<'data>`

- <span id="sectiontable-toowned-type-owned"></span>`type Owned = T`

- <span id="sectiontable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectiontable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionTable<'data>`

- <span id="sectiontable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectiontable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionTable<'data>`

- <span id="sectiontable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectiontable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/coff/symbol.rs:24-31`](../../../../.source_1765633015/object-0.37.3/src/read/coff/symbol.rs#L24-L31)*

A table of symbol entries in a COFF or PE file.

Also includes the string table used for the symbol names.

Returned by `CoffHeader::symbols` and
[`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols).

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Read the symbol table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbol table entries.

  

  This includes auxiliary symbol table entries.

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](../coff/index.md#symboliterator)

  Iterate over the symbols.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`CoffHeader`](../coff/index.md#coffheader)

  Return the symbol table entry at the given index.

- <span id="symboltable-aux-function"></span>`fn aux_function(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ImageAuxSymbolFunction`](../../pe/index.md#imageauxsymbolfunction)

  Return the auxiliary function symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-section"></span>`fn aux_section(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ImageAuxSymbolSection`](../../pe/index.md#imageauxsymbolsection)

  Return the auxiliary section symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-weak-external"></span>`fn aux_weak_external(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ImageAuxSymbolWeak`](../../pe/index.md#imageauxsymbolweak)

  Return the auxiliary weak external symbol for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-aux-file-name"></span>`fn aux_file_name(&self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result)

  Return the auxiliary file name for the symbol table entry at the given index.

  

  Note that the index is of the symbol, not the first auxiliary record.

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result)

  Return the symbol table entry or auxiliary record at the given index and offset.

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../index.md#symbolmap)

  Construct a map from addresses to a user-defined map entry.

#### Trait Implementations

##### `impl Any for SymbolTable<'data, R, Coff>`

- <span id="symboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTable<'data, R, Coff>`

- <span id="symboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTable<'data, R, Coff>`

- <span id="symboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Coff> Debug for SymbolTable<'data, R, Coff>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Default for SymbolTable<'data, R, Coff>`

- <span id="symboltable-default"></span>`fn default() -> Self`

##### `impl<T> From for SymbolTable<'data, R, Coff>`

- <span id="symboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTable<'data, R, Coff>`

- <span id="symboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolTable<'data, R, Coff>`

- <span id="symboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTable<'data, R, Coff>`

- <span id="symboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:37-47`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L37-L47)*

A PE image file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="pefile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw PE file data.

- <span id="pefile-data"></span>`fn data(&self) -> R`

  Returns this binary data.

- <span id="pefile-dos-header"></span>`fn dos_header(&self) -> &'data pe::ImageDosHeader` — [`ImageDosHeader`](../../pe/index.md#imagedosheader)

  Return the DOS header of this file.

- <span id="pefile-nt-headers"></span>`fn nt_headers(&self) -> &'data Pe`

  Return the NT Headers of this file.

- <span id="pefile-rich-header-info"></span>`fn rich_header_info(&self) -> Option<RichHeaderInfo<'_>>` — [`RichHeaderInfo`](#richheaderinfo)

  Returns information about the rich header of this file (if any).

- <span id="pefile-section-table"></span>`fn section_table(&self) -> SectionTable<'data>` — [`SectionTable`](../coff/index.md#sectiontable)

  Returns the section table of this binary.

- <span id="pefile-data-directories"></span>`fn data_directories(&self) -> DataDirectories<'data>` — [`DataDirectories`](#datadirectories)

  Returns the data directories of this file.

- <span id="pefile-data-directory"></span>`fn data_directory(&self, id: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md#imagedatadirectory)

  Returns the data directory at the given index.

- <span id="pefile-export-table"></span>`fn export_table(&self) -> Result<Option<ExportTable<'data>>>` — [`Result`](../../index.md#result), [`ExportTable`](#exporttable)

  Returns the export table of this file.

  

  The export table is located using the data directory.

- <span id="pefile-import-table"></span>`fn import_table(&self) -> Result<Option<ImportTable<'data>>>` — [`Result`](../../index.md#result), [`ImportTable`](#importtable)

  Returns the import table of this file.

  

  The import table is located using the data directory.

- <span id="pefile-section-alignment"></span>`fn section_alignment(&self) -> u64`

#### Trait Implementations

##### `impl Any for PeFile<'data, Pe, R>`

- <span id="pefile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeFile<'data, Pe, R>`

- <span id="pefile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeFile<'data, Pe, R>`

- <span id="pefile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeFile<'data, Pe, R>`

- <span id="pefile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeFile<'data, Pe, R>`

- <span id="pefile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeFile<'data, Pe, R>`

- <span id="pefile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Pe, R> Object for PeFile<'data, Pe, R>`

- <span id="pefile-object-type-segment"></span>`type Segment = PeSegment<'data, 'file, Pe, R>`

- <span id="pefile-object-type-segmentiterator"></span>`type SegmentIterator = PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pefile-object-type-section"></span>`type Section = PeSection<'data, 'file, Pe, R>`

- <span id="pefile-object-type-sectioniterator"></span>`type SectionIterator = PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pefile-object-type-comdat"></span>`type Comdat = PeComdat<'data, 'file, Pe, R>`

- <span id="pefile-object-type-comdatiterator"></span>`type ComdatIterator = PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pefile-object-type-symbol"></span>`type Symbol = CoffSymbol<'data, 'file, R>`

- <span id="pefile-object-type-symboliterator"></span>`type SymbolIterator = CoffSymbolIterator<'data, 'file, R>`

- <span id="pefile-object-type-symboltable"></span>`type SymbolTable = CoffSymbolTable<'data, 'file, R>`

- <span id="pefile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="pefile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="pefile-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md#subarchitecture)

- <span id="pefile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="pefile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="pefile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="pefile-object-segments"></span>`fn segments(&self) -> PeSegmentIterator<'data, '_, Pe, R>` — [`PeSegmentIterator`](#pesegmentiterator)

- <span id="pefile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<PeSection<'data, 'file, Pe, R>>` — [`PeSection`](#pesection)

- <span id="pefile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<PeSection<'data, '_, Pe, R>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`PeSection`](#pesection)

- <span id="pefile-object-sections"></span>`fn sections(&self) -> PeSectionIterator<'data, '_, Pe, R>` — [`PeSectionIterator`](#pesectioniterator)

- <span id="pefile-object-comdats"></span>`fn comdats(&self) -> PeComdatIterator<'data, '_, Pe, R>` — [`PeComdatIterator`](#pecomdatiterator)

- <span id="pefile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`CoffSymbol`](../coff/index.md#coffsymbol)

- <span id="pefile-object-symbols"></span>`fn symbols(&self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../coff/index.md#coffsymboliterator)

- <span id="pefile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../coff/index.md#coffsymboltable)

- <span id="pefile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../coff/index.md#coffsymboliterator)

- <span id="pefile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../coff/index.md#coffsymboltable)

- <span id="pefile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md#nodynamicrelocationiterator)

- <span id="pefile-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="pefile-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="pefile-object-pdb-info"></span>`fn pdb_info(&self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../index.md#result), [`CodeView`](../../index.md#codeview)

- <span id="pefile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="pefile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="pefile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="pefile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<Pe, R> Sealed for PeFile<'data, Pe, R>`

##### `impl<U> TryFrom for PeFile<'data, Pe, R>`

- <span id="pefile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pefile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeFile<'data, Pe, R>`

- <span id="pefile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pefile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeComdatIterator<'data, 'file, Pe, R>`

```rust
struct PeComdatIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:432-439`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L432-L439)*

An iterator for the COMDAT section groups in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pecomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pecomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Pe, R> Iterator for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-iterator-type-item"></span>`type Item = PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pecomdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeComdatIterator<'data, 'file, Pe, R>`

- <span id="pecomdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pecomdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeComdat<'data, 'file, Pe, R>`

```rust
struct PeComdat<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:465-472`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L465-L472)*

A COMDAT section group in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Pe, R> ObjectComdat for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="pecomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="pecomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="pecomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="pecomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md#objectcomdat)

##### `impl<Pe, R> Sealed for PeComdat<'data, 'file, Pe, R>`

##### `impl<U> TryFrom for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pecomdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeComdat<'data, 'file, Pe, R>`

- <span id="pecomdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pecomdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeComdatSectionIterator<'data, 'file, Pe, R>`

```rust
struct PeComdatSectionIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:525-532`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L525-L532)*

An iterator for the sections in a COMDAT section group in a [`PeFile`](#pefile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pecomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pecomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Pe, R> Iterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="pecomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pecomdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeComdatSectionIterator<'data, 'file, Pe, R>`

- <span id="pecomdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pecomdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:23-30`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L23-L30)*

An iterator for the loadable sections in a [`PeFile`](#pefile).

#### Trait Implementations

##### `impl Any for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pesegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Pe, R> Iterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-iterator-type-item"></span>`type Item = PeSegment<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesegmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesegmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:58-65`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L58-L65)*

A loadable section in a [`PeFile`](#pefile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- <span id="pesegment-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](#pefile)

  Get the PE file containing this segment.

- <span id="pesegment-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Get the raw PE section header.

#### Trait Implementations

##### `impl Any for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Pe, R> ObjectSegment for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="pesegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="pesegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="pesegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="pesegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="pesegment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="pesegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="pesegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="pesegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<Pe, R> Sealed for PeSegment<'data, 'file, Pe, R>`

##### `impl<U> TryFrom for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:162-169`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L162-L169)*

An iterator for the sections in a [`PeFile`](#pefile).

#### Trait Implementations

##### `impl Any for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pesectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Pe, R> Iterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-iterator-type-item"></span>`type Item = PeSection<'data, 'file, Pe, R>`

- <span id="pesectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/section.rs:198-206`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L198-L206)*

A section in a [`PeFile`](#pefile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="pesection-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](#pefile)

  Get the PE file containing this segment.

- <span id="pesection-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../pe/index.md#imagesectionheader)

  Get the raw PE section header.

#### Trait Implementations

##### `impl Any for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Pe, R> ObjectSection for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-objectsection-type-relocationiterator"></span>`type RelocationIterator = PeRelocationIterator<'data, 'file, R>`

- <span id="pesection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="pesection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="pesection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="pesection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="pesection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="pesection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="pesection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="pesection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="pesection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="pesection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="pesection-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="pesection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="pesection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="pesection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="pesection-objectsection-relocations"></span>`fn relocations(&self) -> PeRelocationIterator<'data, 'file, R>` — [`PeRelocationIterator`](#perelocationiterator)

- <span id="pesection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="pesection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

##### `impl<Pe, R> Sealed for PeSection<'data, 'file, Pe, R>`

##### `impl<U> TryFrom for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeRelocationIterator<'data, 'file, R>`

```rust
struct PeRelocationIterator<'data, 'file, R>(core::marker::PhantomData<(&'data (), &'file (), R)>);
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:466-468`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L466-L468)*

An iterator for the relocations in an [`PeSection`](#pesection).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug> Debug for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="perelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="perelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="perelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="perelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="perelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DataDirectories<'data>`

```rust
struct DataDirectories<'data> {
    entries: &'data [pe::ImageDataDirectory],
}
```

*Defined in [`object-0.37.3/src/read/pe/data_directory.rs:16-18`](../../../../.source_1765633015/object-0.37.3/src/read/pe/data_directory.rs#L16-L18)*

The table of data directories in a PE file.

Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse).

#### Implementations

- <span id="datadirectories-parse"></span>`fn parse(data: &'data [u8], number: u32) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the data directory table.

  

  `data` must be the remaining optional data following the

  [optional header](pe::ImageOptionalHeader64).  `number` must be from the

  [`number_of_rva_and_sizes`](pe::ImageOptionalHeader64::number_of_rva_and_sizes)

  field of the optional header.

- <span id="datadirectories-len"></span>`fn len(&self) -> usize`

  The number of data directories.

- <span id="datadirectories-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md#imagedatadirectory)

  Iterator over the data directories.

- <span id="datadirectories-enumerate"></span>`fn enumerate(&self) -> core::iter::Enumerate<slice::Iter<'data, pe::ImageDataDirectory>>` — [`ImageDataDirectory`](../../pe/index.md#imagedatadirectory)

  Iterator which gives the directories as well as their index (one of the IMAGE_DIRECTORY_ENTRY_* constants).

- <span id="datadirectories-get"></span>`fn get(&self, index: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../pe/index.md#imagedatadirectory)

  Returns the data directory at the given index.

  

  Index should be one of the `IMAGE_DIRECTORY_ENTRY_*` constants.

  

  Returns `None` if the index is larger than the table size,

  or if the entry at the index has a zero virtual address.

- <span id="datadirectories-export-directory"></span>`fn export_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<&'data pe::ImageExportDirectory>>` — [`SectionTable`](../coff/index.md#sectiontable), [`Result`](../../index.md#result), [`ImageExportDirectory`](../../pe/index.md#imageexportdirectory)

  Returns the unparsed export directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-export-table"></span>`fn export_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ExportTable<'data>>>` — [`SectionTable`](../coff/index.md#sectiontable), [`Result`](../../index.md#result), [`ExportTable`](#exporttable)

  Returns the partially parsed export directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-import-table"></span>`fn import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ImportTable<'data>>>` — [`SectionTable`](../coff/index.md#sectiontable), [`Result`](../../index.md#result), [`ImportTable`](#importtable)

  Returns the partially parsed import directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-delay-load-import-table"></span>`fn delay_load_import_table<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<DelayLoadImportTable<'data>>>` — [`SectionTable`](../coff/index.md#sectiontable), [`Result`](../../index.md#result), [`DelayLoadImportTable`](#delayloadimporttable)

  Returns the partially parsed delay-load import directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-relocation-blocks"></span>`fn relocation_blocks<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<RelocationBlockIterator<'data>>>` — [`SectionTable`](../coff/index.md#sectiontable), [`Result`](../../index.md#result), [`RelocationBlockIterator`](#relocationblockiterator)

  Returns the blocks in the base relocation directory.

  

  `data` must be the entire file data.

- <span id="datadirectories-resource-directory"></span>`fn resource_directory<R: ReadRef<'data>>(&self, data: R, sections: &SectionTable<'data>) -> Result<Option<ResourceDirectory<'data>>>` — [`SectionTable`](../coff/index.md#sectiontable), [`Result`](../../index.md#result), [`ResourceDirectory`](#resourcedirectory)

  Returns the resource directory.

  

  `data` must be the entire file data.

#### Trait Implementations

##### `impl Any for DataDirectories<'data>`

- <span id="datadirectories-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DataDirectories<'data>`

- <span id="datadirectories-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DataDirectories<'data>`

- <span id="datadirectories-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DataDirectories<'data>`

- <span id="datadirectories-clone"></span>`fn clone(&self) -> DataDirectories<'data>` — [`DataDirectories`](#datadirectories)

##### `impl CloneToUninit for DataDirectories<'data>`

- <span id="datadirectories-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DataDirectories<'data>`

##### `impl Debug for DataDirectories<'data>`

- <span id="datadirectories-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DataDirectories<'data>`

- <span id="datadirectories-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DataDirectories<'data>`

- <span id="datadirectories-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DataDirectories<'data>`

- <span id="datadirectories-toowned-type-owned"></span>`type Owned = T`

- <span id="datadirectories-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="datadirectories-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DataDirectories<'data>`

- <span id="datadirectories-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="datadirectories-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DataDirectories<'data>`

- <span id="datadirectories-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="datadirectories-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Export<'data>`

```rust
struct Export<'data> {
    pub ordinal: u32,
    pub name: Option<&'data [u8]>,
    pub target: ExportTarget<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:42-51`](../../../../.source_1765633015/object-0.37.3/src/read/pe/export.rs#L42-L51)*

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

##### `impl Any for Export<'data>`

- <span id="export-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Export<'data>`

- <span id="export-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Export<'data>`

- <span id="export-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Export<'data>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'data>` — [`Export`](#export)

##### `impl CloneToUninit for Export<'data>`

- <span id="export-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Export<'data>`

##### `impl Debug for Export<'a>`

- <span id="export-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl<T> From for Export<'data>`

- <span id="export-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Export<'data>`

- <span id="export-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Export<'data>`

- <span id="export-toowned-type-owned"></span>`type Owned = T`

- <span id="export-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="export-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Export<'data>`

- <span id="export-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="export-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Export<'data>`

- <span id="export-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="export-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/pe/export.rs:87-94`](../../../../.source_1765633015/object-0.37.3/src/read/pe/export.rs#L87-L94)*

A partially parsed PE export table.

Returned by [`DataDirectories::export_table`](super::DataDirectories::export_table).

#### Implementations

- <span id="exporttable-parse"></span>`fn parse(data: &'data [u8], virtual_address: u32) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the export table given its section data and address.

- <span id="exporttable-parse-directory"></span>`fn parse_directory(data: &'data [u8]) -> Result<&'data pe::ImageExportDirectory>` — [`Result`](../../index.md#result), [`ImageExportDirectory`](../../pe/index.md#imageexportdirectory)

  Parse the export directory given its section data.

- <span id="exporttable-directory"></span>`fn directory(&self) -> &'data pe::ImageExportDirectory` — [`ImageExportDirectory`](../../pe/index.md#imageexportdirectory)

  Returns the header of the export table.

- <span id="exporttable-ordinal-base"></span>`fn ordinal_base(&self) -> u32`

  Returns the base value of ordinals.

  

  Adding this to an address index will give an ordinal.

- <span id="exporttable-addresses"></span>`fn addresses(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../index.md#u32bytes), [`LittleEndian`](../../index.md#littleendian)

  Returns the unparsed address table.

  

  An address table entry may be a local address, or the address of a forwarded export entry.

  See `Self::is_forward` and `Self::target_from_address`.

- <span id="exporttable-name-pointers"></span>`fn name_pointers(&self) -> &'data [U32Bytes<LE>]` — [`U32Bytes`](../../index.md#u32bytes), [`LittleEndian`](../../index.md#littleendian)

  Returns the unparsed name pointer table.

  

  A name pointer table entry can be used with `Self::name_from_pointer`.

- <span id="exporttable-name-ordinals"></span>`fn name_ordinals(&self) -> &'data [U16Bytes<LE>]` — [`U16Bytes`](../../index.md#u16bytes), [`LittleEndian`](../../index.md#littleendian)

  Returns the unparsed ordinal table.

  

  An ordinal table entry is a 0-based index into the address table.

  See `Self::address_by_index` and `Self::target_by_index`.

- <span id="exporttable-name-iter"></span>`fn name_iter(&self) -> impl Iterator<Item = (u32, u16)> + 'data`

  Returns an iterator for the entries in the name pointer table and ordinal table.

  

  A name pointer table entry can be used with `Self::name_from_pointer`.

  

  An ordinal table entry is a 0-based index into the address table.

  See `Self::address_by_index` and `Self::target_by_index`.

- <span id="exporttable-address-by-index"></span>`fn address_by_index(&self, index: u32) -> Result<u32>` — [`Result`](../../index.md#result)

  Returns the export address table entry at the given address index.

  

  This may be a local address, or the address of a forwarded export entry.

  See `Self::is_forward` and `Self::target_from_address`.

  

  `index` is a 0-based index into the export address table.

- <span id="exporttable-address-by-ordinal"></span>`fn address_by_ordinal(&self, ordinal: u32) -> Result<u32>` — [`Result`](../../index.md#result)

  Returns the export address table entry at the given ordinal.

  

  This may be a local address, or the address of a forwarded export entry.

  See `Self::is_forward` and `Self::target_from_address`.

- <span id="exporttable-target-by-index"></span>`fn target_by_index(&self, index: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md#result), [`ExportTarget`](#exporttarget)

  Returns the target of the export at the given address index.

  

  `index` is a 0-based index into the export address table.

- <span id="exporttable-target-by-ordinal"></span>`fn target_by_ordinal(&self, ordinal: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md#result), [`ExportTarget`](#exporttarget)

  Returns the target of the export at the given ordinal.

- <span id="exporttable-target-from-address"></span>`fn target_from_address(&self, address: u32) -> Result<ExportTarget<'data>>` — [`Result`](../../index.md#result), [`ExportTarget`](#exporttarget)

  Convert an export address table entry into a target.

- <span id="exporttable-forward-offset"></span>`fn forward_offset(&self, address: u32) -> Option<usize>`

- <span id="exporttable-is-forward"></span>`fn is_forward(&self, address: u32) -> bool`

  Return true if the export address table entry is a forward.

- <span id="exporttable-forward-string"></span>`fn forward_string(&self, address: u32) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

  Return the forward string if the export address table entry is a forward.

- <span id="exporttable-name-from-pointer"></span>`fn name_from_pointer(&self, name_pointer: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

  Convert an export name pointer table entry into a name.

- <span id="exporttable-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](#export)

  Returns the parsed exports in this table.

#### Trait Implementations

##### `impl Any for ExportTable<'data>`

- <span id="exporttable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExportTable<'data>`

- <span id="exporttable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExportTable<'data>`

- <span id="exporttable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ExportTable<'data>`

- <span id="exporttable-clone"></span>`fn clone(&self) -> ExportTable<'data>` — [`ExportTable`](#exporttable)

##### `impl CloneToUninit for ExportTable<'data>`

- <span id="exporttable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ExportTable<'data>`

- <span id="exporttable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ExportTable<'data>`

- <span id="exporttable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExportTable<'data>`

- <span id="exporttable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ExportTable<'data>`

- <span id="exporttable-toowned-type-owned"></span>`type Owned = T`

- <span id="exporttable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exporttable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExportTable<'data>`

- <span id="exporttable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exporttable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExportTable<'data>`

- <span id="exporttable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exporttable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImportTable<'data>`

```rust
struct ImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:15-19`](../../../../.source_1765633015/object-0.37.3/src/read/pe/import.rs#L15-L19)*

Information for parsing a PE import table.

Returned by [`DataDirectories::import_table`](super::DataDirectories::import_table).

#### Implementations

- <span id="importtable-new"></span>`fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

  Create a new import table parser.

  

  The import descriptors start at `import_address`.

  The size declared in the `IMAGE_DIRECTORY_ENTRY_IMPORT` data directory is

  ignored by the Windows loader, and so descriptors will be parsed until a null entry.

  

  `section_data` should be from the section containing `import_address`, and

  `section_address` should be the address of that section. Pointers within the

  descriptors and thunks may point to anywhere within the section data.

- <span id="importtable-descriptors"></span>`fn descriptors(&self) -> Result<ImportDescriptorIterator<'data>>` — [`Result`](../../index.md#result), [`ImportDescriptorIterator`](#importdescriptoriterator)

  Return an iterator for the import descriptors.

- <span id="importtable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

  Return a library name given its address.

  

  This address may be from `pe::ImageImportDescriptor::name`.

- <span id="importtable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../index.md#result), [`ImportThunkList`](#importthunklist)

  Return a list of thunks given its address.

  

  This address may be from `pe::ImageImportDescriptor::original_first_thunk`

  or `pe::ImageImportDescriptor::first_thunk`.

- <span id="importtable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](#imagentheaders), [`Result`](../../index.md#result), [`Import`](#import)

  Parse a thunk.

- <span id="importtable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../index.md#result)

  Return the hint and name at the given address.

  

  This address may be from [`pe::ImageThunkData32`](../../pe/index.md) or [`pe::ImageThunkData64`](../../pe/index.md).

  

  The hint is an index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl Any for ImportTable<'data>`

- <span id="importtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportTable<'data>`

- <span id="importtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportTable<'data>`

- <span id="importtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportTable<'data>`

- <span id="importtable-clone"></span>`fn clone(&self) -> ImportTable<'data>` — [`ImportTable`](#importtable)

##### `impl CloneToUninit for ImportTable<'data>`

- <span id="importtable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportTable<'data>`

- <span id="importtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportTable<'data>`

- <span id="importtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportTable<'data>`

- <span id="importtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ImportTable<'data>`

- <span id="importtable-toowned-type-owned"></span>`type Owned = T`

- <span id="importtable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importtable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportTable<'data>`

- <span id="importtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportTable<'data>`

- <span id="importtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImportDescriptorIterator<'data>`

```rust
struct ImportDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:102-105`](../../../../.source_1765633015/object-0.37.3/src/read/pe/import.rs#L102-L105)*

A fallible iterator for the descriptors in the import data directory.

#### Implementations

- <span id="importdescriptoriterator-next"></span>`fn next(&mut self) -> Result<Option<&'data pe::ImageImportDescriptor>>` — [`Result`](../../index.md#result), [`ImageImportDescriptor`](../../pe/index.md#imageimportdescriptor)

  Return the next descriptor.

  

  Returns `Ok(None)` when a null descriptor is found.

#### Trait Implementations

##### `impl Any for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-clone"></span>`fn clone(&self) -> ImportDescriptorIterator<'data>` — [`ImportDescriptorIterator`](#importdescriptoriterator)

##### `impl CloneToUninit for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="importdescriptoriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="importdescriptoriterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-iterator-type-item"></span>`type Item = Result<&'data ImageImportDescriptor, Error>`

- <span id="importdescriptoriterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-toowned-type-owned"></span>`type Owned = T`

- <span id="importdescriptoriterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importdescriptoriterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importdescriptoriterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportDescriptorIterator<'data>`

- <span id="importdescriptoriterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importdescriptoriterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImportThunkList<'data>`

```rust
struct ImportThunkList<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:148-150`](../../../../.source_1765633015/object-0.37.3/src/read/pe/import.rs#L148-L150)*

A list of import thunks.

These may be in the import lookup table, or the import address table.

#### Implementations

- <span id="importthunklist-get"></span>`fn get<Pe: ImageNtHeaders>(&self, index: usize) -> Result<<Pe as >::ImageThunkData>` — [`Result`](../../index.md#result), [`ImageNtHeaders`](#imagentheaders)

  Get the thunk at the given index.

- <span id="importthunklist-next"></span>`fn next<Pe: ImageNtHeaders>(&mut self) -> Result<Option<<Pe as >::ImageThunkData>>` — [`Result`](../../index.md#result), [`ImageNtHeaders`](#imagentheaders)

  Return the first thunk in the list, and update `self` to point after it.

  

  Returns `Ok(None)` when a null thunk is found.

#### Trait Implementations

##### `impl Any for ImportThunkList<'data>`

- <span id="importthunklist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImportThunkList<'data>`

- <span id="importthunklist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImportThunkList<'data>`

- <span id="importthunklist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImportThunkList<'data>`

- <span id="importthunklist-clone"></span>`fn clone(&self) -> ImportThunkList<'data>` — [`ImportThunkList`](#importthunklist)

##### `impl CloneToUninit for ImportThunkList<'data>`

- <span id="importthunklist-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImportThunkList<'data>`

- <span id="importthunklist-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImportThunkList<'data>`

- <span id="importthunklist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ImportThunkList<'data>`

- <span id="importthunklist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ImportThunkList<'data>`

- <span id="importthunklist-toowned-type-owned"></span>`type Owned = T`

- <span id="importthunklist-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="importthunklist-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImportThunkList<'data>`

- <span id="importthunklist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="importthunklist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImportThunkList<'data>`

- <span id="importthunklist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="importthunklist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DelayLoadImportTable<'data>`

```rust
struct DelayLoadImportTable<'data> {
    section_data: crate::read::Bytes<'data>,
    section_address: u32,
    import_address: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:250-254`](../../../../.source_1765633015/object-0.37.3/src/read/pe/import.rs#L250-L254)*

Information for parsing a PE delay-load import table.

Returned by
[`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table).

#### Implementations

- <span id="delayloadimporttable-new"></span>`fn new(section_data: &'data [u8], section_address: u32, import_address: u32) -> Self`

  Create a new delay load import table parser.

  

  The import descriptors start at `import_address`.

  This table works in the same way the import table does: descriptors will be

  parsed until a null entry.

  

  `section_data` should be from the section containing `import_address`, and

  `section_address` should be the address of that section. Pointers within the

  descriptors and thunks may point to anywhere within the section data.

- <span id="delayloadimporttable-descriptors"></span>`fn descriptors(&self) -> Result<DelayLoadDescriptorIterator<'data>>` — [`Result`](../../index.md#result), [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)

  Return an iterator for the import descriptors.

- <span id="delayloadimporttable-name"></span>`fn name(&self, address: u32) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

  Return a library name given its address.

  

  This address may be from `pe::ImageDelayloadDescriptor::dll_name_rva`.

- <span id="delayloadimporttable-thunks"></span>`fn thunks(&self, address: u32) -> Result<ImportThunkList<'data>>` — [`Result`](../../index.md#result), [`ImportThunkList`](#importthunklist)

  Return a list of thunks given its address.

  

  This address may be from the INT, i.e. from

  `pe::ImageDelayloadDescriptor::import_name_table_rva`.

  

  Please note that others RVA values from [`pe::ImageDelayloadDescriptor`](../../pe/index.md) are used

  by the delay loader at runtime to store values, and thus do not point inside the same

  section as the INT. Calling this function on those addresses will fail.

- <span id="delayloadimporttable-import"></span>`fn import<Pe: ImageNtHeaders>(&self, thunk: <Pe as >::ImageThunkData) -> Result<Import<'data>>` — [`ImageNtHeaders`](#imagentheaders), [`Result`](../../index.md#result), [`Import`](#import)

  Parse a thunk.

- <span id="delayloadimporttable-hint-name"></span>`fn hint_name(&self, address: u32) -> Result<(u16, &'data [u8])>` — [`Result`](../../index.md#result)

  Return the hint and name at the given address.

  

  This address may be from [`pe::ImageThunkData32`](../../pe/index.md) or [`pe::ImageThunkData64`](../../pe/index.md).

  

  The hint is an index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl Any for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-clone"></span>`fn clone(&self) -> DelayLoadImportTable<'data>` — [`DelayLoadImportTable`](#delayloadimporttable)

##### `impl CloneToUninit for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-toowned-type-owned"></span>`type Owned = T`

- <span id="delayloadimporttable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="delayloadimporttable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="delayloadimporttable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DelayLoadImportTable<'data>`

- <span id="delayloadimporttable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="delayloadimporttable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DelayLoadDescriptorIterator<'data>`

```rust
struct DelayLoadDescriptorIterator<'data> {
    data: crate::read::Bytes<'data>,
    null: bool,
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:341-344`](../../../../.source_1765633015/object-0.37.3/src/read/pe/import.rs#L341-L344)*

A fallible iterator for the descriptors in the delay-load data directory.

#### Implementations

- <span id="delayloaddescriptoriterator-next"></span>`fn next(&mut self) -> Result<Option<&'data pe::ImageDelayloadDescriptor>>` — [`Result`](../../index.md#result), [`ImageDelayloadDescriptor`](../../pe/index.md#imagedelayloaddescriptor)

  Return the next descriptor.

  

  Returns `Ok(None)` when a null descriptor is found.

#### Trait Implementations

##### `impl Any for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-clone"></span>`fn clone(&self) -> DelayLoadDescriptorIterator<'data>` — [`DelayLoadDescriptorIterator`](#delayloaddescriptoriterator)

##### `impl CloneToUninit for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="delayloaddescriptoriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="delayloaddescriptoriterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-iterator-type-item"></span>`type Item = Result<&'data ImageDelayloadDescriptor, Error>`

- <span id="delayloaddescriptoriterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-toowned-type-owned"></span>`type Owned = T`

- <span id="delayloaddescriptoriterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="delayloaddescriptoriterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="delayloaddescriptoriterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DelayLoadDescriptorIterator<'data>`

- <span id="delayloaddescriptoriterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="delayloaddescriptoriterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationBlockIterator<'data>`

```rust
struct RelocationBlockIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:11-13`](../../../../.source_1765633015/object-0.37.3/src/read/pe/relocation.rs#L11-L13)*

An iterator over the relocation blocks in the `.reloc` section of a PE file.

Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks).

#### Implementations

- <span id="relocationblockiterator-new"></span>`fn new(data: &'data [u8]) -> Self`

  Construct a new iterator from the data of the `.reloc` section.

- <span id="relocationblockiterator-next"></span>`fn next(&mut self) -> Result<Option<RelocationIterator<'data>>>` — [`Result`](../../index.md#result), [`RelocationIterator`](#relocationiterator)

  Read the next relocation page.

- <span id="relocationblockiterator-parse"></span>`fn parse(&mut self) -> Result<RelocationIterator<'data>>` — [`Result`](../../index.md#result), [`RelocationIterator`](#relocationiterator)

#### Trait Implementations

##### `impl Any for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-clone"></span>`fn clone(&self) -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](#relocationblockiterator)

##### `impl CloneToUninit for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationBlockIterator<'data>`

##### `impl Debug for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-default"></span>`fn default() -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](#relocationblockiterator)

##### `impl<T> From for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationblockiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relocationblockiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-iterator-type-item"></span>`type Item = Result<RelocationIterator<'data>, Error>`

- <span id="relocationblockiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationblockiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationblockiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationblockiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationblockiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationIterator<'data>`

```rust
struct RelocationIterator<'data> {
    virtual_address: u32,
    size: u32,
    relocs: slice::Iter<'data, crate::endian::U16<crate::endian::LittleEndian>>,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:68-72`](../../../../.source_1765633015/object-0.37.3/src/read/pe/relocation.rs#L68-L72)*

An iterator of the relocations in a block in the `.reloc` section of a PE file.

#### Implementations

- <span id="relocationiterator-virtual-address"></span>`fn virtual_address(&self) -> u32`

  Return the virtual address of the page that this block of relocations applies to.

- <span id="relocationiterator-size"></span>`fn size(&self) -> u32`

  Return the size in bytes of this block of relocations.

#### Trait Implementations

##### `impl Any for RelocationIterator<'data>`

- <span id="relocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationIterator<'data>`

- <span id="relocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationIterator<'data>`

- <span id="relocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationIterator<'data>`

- <span id="relocationiterator-clone"></span>`fn clone(&self) -> RelocationIterator<'data>` — [`RelocationIterator`](#relocationiterator)

##### `impl CloneToUninit for RelocationIterator<'data>`

- <span id="relocationiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RelocationIterator<'data>`

- <span id="relocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RelocationIterator<'data>`

- <span id="relocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationIterator<'data>`

- <span id="relocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RelocationIterator<'data>`

- <span id="relocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RelocationIterator<'data>`

- <span id="relocationiterator-iterator-type-item"></span>`type Item = Relocation`

- <span id="relocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<Relocation>` — [`Relocation`](#relocation)

##### `impl ToOwned for RelocationIterator<'data>`

- <span id="relocationiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationIterator<'data>`

- <span id="relocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationIterator<'data>`

- <span id="relocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Relocation`

```rust
struct Relocation {
    pub virtual_address: u32,
    pub typ: u16,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:104-109`](../../../../.source_1765633015/object-0.37.3/src/read/pe/relocation.rs#L104-L109)*

A relocation in the `.reloc` section of a PE file.

#### Fields

- **`virtual_address`**: `u32`

  The virtual address of the relocation.

- **`typ`**: `u16`

  One of the `pe::IMAGE_REL_BASED_*` constants.

#### Trait Implementations

##### `impl Any for Relocation`

- <span id="relocation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Relocation`

- <span id="relocation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Relocation`

- <span id="relocation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Relocation`

- <span id="relocation-clone"></span>`fn clone(&self) -> Relocation` — [`Relocation`](#relocation)

##### `impl CloneToUninit for Relocation`

- <span id="relocation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Relocation`

##### `impl Debug for Relocation`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Relocation`

- <span id="relocation-default"></span>`fn default() -> Relocation` — [`Relocation`](#relocation)

##### `impl<T> From for Relocation`

- <span id="relocation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Relocation`

- <span id="relocation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Relocation`

- <span id="relocation-toowned-type-owned"></span>`type Owned = T`

- <span id="relocation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Relocation`

- <span id="relocation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Relocation`

- <span id="relocation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceDirectory<'data>`

```rust
struct ResourceDirectory<'data> {
    data: &'data [u8],
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:12-14`](../../../../.source_1765633015/object-0.37.3/src/read/pe/resource.rs#L12-L14)*

The `.rsrc` section of a PE file.

Returned by [`DataDirectories::resource_directory`](super::DataDirectories::resource_directory).

#### Implementations

- <span id="resourcedirectory-new"></span>`fn new(data: &'data [u8]) -> Self`

  Construct from the data of the `.rsrc` section.

- <span id="resourcedirectory-root"></span>`fn root(&self) -> Result<ResourceDirectoryTable<'data>>` — [`Result`](../../index.md#result), [`ResourceDirectoryTable`](#resourcedirectorytable)

  Parses the root resource directory.

#### Trait Implementations

##### `impl Any for ResourceDirectory<'data>`

- <span id="resourcedirectory-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceDirectory<'data>`

- <span id="resourcedirectory-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceDirectory<'data>`

- <span id="resourcedirectory-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceDirectory<'data>`

- <span id="resourcedirectory-clone"></span>`fn clone(&self) -> ResourceDirectory<'data>` — [`ResourceDirectory`](#resourcedirectory)

##### `impl CloneToUninit for ResourceDirectory<'data>`

- <span id="resourcedirectory-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ResourceDirectory<'data>`

##### `impl Debug for ResourceDirectory<'data>`

- <span id="resourcedirectory-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceDirectory<'data>`

- <span id="resourcedirectory-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceDirectory<'data>`

- <span id="resourcedirectory-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceDirectory<'data>`

- <span id="resourcedirectory-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcedirectory-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcedirectory-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceDirectory<'data>`

- <span id="resourcedirectory-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcedirectory-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceDirectory<'data>`

- <span id="resourcedirectory-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcedirectory-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceDirectoryTable<'data>`

```rust
struct ResourceDirectoryTable<'data> {
    pub header: &'data pe::ImageResourceDirectory,
    pub entries: &'data [pe::ImageResourceDirectoryEntry],
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:30-35`](../../../../.source_1765633015/object-0.37.3/src/read/pe/resource.rs#L30-L35)*

A table of resource entries.

#### Fields

- **`header`**: `&'data pe::ImageResourceDirectory`

  The table header.

- **`entries`**: `&'data [pe::ImageResourceDirectoryEntry]`

  The table entries.

#### Implementations

- <span id="resourcedirectorytable-parse"></span>`fn parse(data: &'data [u8], offset: u32) -> Result<Self>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl Any for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-clone"></span>`fn clone(&self) -> ResourceDirectoryTable<'data>` — [`ResourceDirectoryTable`](#resourcedirectorytable)

##### `impl CloneToUninit for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcedirectorytable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcedirectorytable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcedirectorytable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceDirectoryTable<'data>`

- <span id="resourcedirectorytable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcedirectorytable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceName`

```rust
struct ResourceName {
    offset: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:143-145`](../../../../.source_1765633015/object-0.37.3/src/read/pe/resource.rs#L143-L145)*

A resource name.

#### Implementations

- <span id="resourcename-to-string-lossy"></span>`fn to_string_lossy(&self, directory: ResourceDirectory<'_>) -> Result<String>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md#result)

  Converts to a `String`.

- <span id="resourcename-data"></span>`fn data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [U16Bytes<LE>]>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md#result), [`U16Bytes`](../../index.md#u16bytes), [`LittleEndian`](../../index.md#littleendian)

  Returns the string unicode buffer.

- <span id="resourcename-raw-data"></span>`fn raw_data<'data>(&self, directory: ResourceDirectory<'data>) -> Result<&'data [u8]>` — [`ResourceDirectory`](#resourcedirectory), [`Result`](../../index.md#result)

  Returns the string buffer as raw bytes.

#### Trait Implementations

##### `impl Any for ResourceName`

- <span id="resourcename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceName`

- <span id="resourcename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceName`

- <span id="resourcename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceName`

- <span id="resourcename-clone"></span>`fn clone(&self) -> ResourceName` — [`ResourceName`](#resourcename)

##### `impl CloneToUninit for ResourceName`

- <span id="resourcename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ResourceName`

##### `impl Debug for ResourceName`

- <span id="resourcename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceName`

- <span id="resourcename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceName`

- <span id="resourcename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceName`

- <span id="resourcename-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceName`

- <span id="resourcename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceName`

- <span id="resourcename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RichHeaderInfo<'data>`

```rust
struct RichHeaderInfo<'data> {
    pub offset: usize,
    pub length: usize,
    pub xor_key: u32,
    masked_entries: &'data [pe::MaskedRichHeaderEntry],
}
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:12-26`](../../../../.source_1765633015/object-0.37.3/src/read/pe/rich.rs#L12-L26)*

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

  Try to locate a rich header and its entries in the current PE file.

- <span id="richheaderinfo-unmasked-entries"></span>`fn unmasked_entries(&self) -> impl Iterator<Item = RichHeaderEntry> + 'data` — [`RichHeaderEntry`](#richheaderentry)

  Returns an iterator over the unmasked entries.

#### Trait Implementations

##### `impl Any for RichHeaderInfo<'data>`

- <span id="richheaderinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RichHeaderInfo<'data>`

- <span id="richheaderinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RichHeaderInfo<'data>`

- <span id="richheaderinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RichHeaderInfo<'data>`

- <span id="richheaderinfo-clone"></span>`fn clone(&self) -> RichHeaderInfo<'data>` — [`RichHeaderInfo`](#richheaderinfo)

##### `impl CloneToUninit for RichHeaderInfo<'data>`

- <span id="richheaderinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RichHeaderInfo<'data>`

##### `impl Debug for RichHeaderInfo<'data>`

- <span id="richheaderinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RichHeaderInfo<'data>`

- <span id="richheaderinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RichHeaderInfo<'data>`

- <span id="richheaderinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RichHeaderInfo<'data>`

- <span id="richheaderinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="richheaderinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="richheaderinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RichHeaderInfo<'data>`

- <span id="richheaderinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="richheaderinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RichHeaderInfo<'data>`

- <span id="richheaderinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="richheaderinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RichHeaderEntry`

```rust
struct RichHeaderEntry {
    pub comp_id: u32,
    pub count: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:33-38`](../../../../.source_1765633015/object-0.37.3/src/read/pe/rich.rs#L33-L38)*

A PE rich header entry after it has been unmasked.

See [`pe::MaskedRichHeaderEntry`](../../pe/index.md).

#### Fields

- **`comp_id`**: `u32`

  ID of the component.

- **`count`**: `u32`

  Number of times this component has been used when building this PE.

#### Trait Implementations

##### `impl Any for RichHeaderEntry`

- <span id="richheaderentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RichHeaderEntry`

- <span id="richheaderentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RichHeaderEntry`

- <span id="richheaderentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RichHeaderEntry`

- <span id="richheaderentry-clone"></span>`fn clone(&self) -> RichHeaderEntry` — [`RichHeaderEntry`](#richheaderentry)

##### `impl CloneToUninit for RichHeaderEntry`

- <span id="richheaderentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RichHeaderEntry`

##### `impl Debug for RichHeaderEntry`

- <span id="richheaderentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RichHeaderEntry`

- <span id="richheaderentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RichHeaderEntry`

- <span id="richheaderentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RichHeaderEntry`

- <span id="richheaderentry-toowned-type-owned"></span>`type Owned = T`

- <span id="richheaderentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="richheaderentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RichHeaderEntry`

- <span id="richheaderentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="richheaderentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RichHeaderEntry`

- <span id="richheaderentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="richheaderentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ExportTarget<'data>`

```rust
enum ExportTarget<'data> {
    Address(u32),
    ForwardByOrdinal(&'data [u8], u32),
    ForwardByName(&'data [u8], &'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:10-21`](../../../../.source_1765633015/object-0.37.3/src/read/pe/export.rs#L10-L21)*

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

  Returns true if the target is an address.

- <span id="exporttarget-is-forward"></span>`fn is_forward(&self) -> bool`

  Returns true if the export is forwarded to another DLL.

#### Trait Implementations

##### `impl Any for ExportTarget<'data>`

- <span id="exporttarget-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExportTarget<'data>`

- <span id="exporttarget-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExportTarget<'data>`

- <span id="exporttarget-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ExportTarget<'data>`

- <span id="exporttarget-clone"></span>`fn clone(&self) -> ExportTarget<'data>` — [`ExportTarget`](#exporttarget)

##### `impl CloneToUninit for ExportTarget<'data>`

- <span id="exporttarget-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ExportTarget<'data>`

##### `impl Debug for ExportTarget<'a>`

- <span id="exporttarget-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl<T> From for ExportTarget<'data>`

- <span id="exporttarget-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExportTarget<'data>`

- <span id="exporttarget-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ExportTarget<'data>`

- <span id="exporttarget-toowned-type-owned"></span>`type Owned = T`

- <span id="exporttarget-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exporttarget-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExportTarget<'data>`

- <span id="exporttarget-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exporttarget-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExportTarget<'data>`

- <span id="exporttarget-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exporttarget-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Import<'data>`

```rust
enum Import<'data> {
    Ordinal(u16),
    Name(u16, &'data [u8]),
}
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:180-187`](../../../../.source_1765633015/object-0.37.3/src/read/pe/import.rs#L180-L187)*

A parsed import thunk.

#### Variants

- **`Ordinal`**

  Import by ordinal.

- **`Name`**

  Import by name.
  
  Includes a hint for the index into the export name pointer table in the target library.

#### Trait Implementations

##### `impl Any for Import<'data>`

- <span id="import-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Import<'data>`

- <span id="import-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Import<'data>`

- <span id="import-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Import<'data>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'data>` — [`Import`](#import)

##### `impl CloneToUninit for Import<'data>`

- <span id="import-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Import<'data>`

##### `impl Debug for Import<'data>`

- <span id="import-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Import<'data>`

- <span id="import-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Import<'data>`

- <span id="import-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Import<'data>`

- <span id="import-toowned-type-owned"></span>`type Owned = T`

- <span id="import-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="import-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Import<'data>`

- <span id="import-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="import-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Import<'data>`

- <span id="import-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="import-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceDirectoryEntryData<'data>`

```rust
enum ResourceDirectoryEntryData<'data> {
    Table(ResourceDirectoryTable<'data>),
    Data(&'data pe::ImageResourceDataEntry),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:112-117`](../../../../.source_1765633015/object-0.37.3/src/read/pe/resource.rs#L112-L117)*

Data associated with a resource directory entry.

#### Variants

- **`Table`**

  A subtable entry.

- **`Data`**

  A resource data entry.

#### Implementations

- <span id="resourcedirectoryentrydata-table"></span>`fn table(self) -> Option<ResourceDirectoryTable<'data>>` — [`ResourceDirectoryTable`](#resourcedirectorytable)

  Converts to an option of table.

  

  Helper for iterator filtering.

- <span id="resourcedirectoryentrydata-data"></span>`fn data(self) -> Option<&'data pe::ImageResourceDataEntry>` — [`ImageResourceDataEntry`](../../pe/index.md#imageresourcedataentry)

  Converts to an option of data entry.

  

  Helper for iterator filtering.

#### Trait Implementations

##### `impl Any for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-clone"></span>`fn clone(&self) -> ResourceDirectoryEntryData<'data>` — [`ResourceDirectoryEntryData`](#resourcedirectoryentrydata)

##### `impl CloneToUninit for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-toowned-type-owned"></span>`type Owned = T`

- <span id="resourcedirectoryentrydata-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resourcedirectoryentrydata-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcedirectoryentrydata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceDirectoryEntryData<'data>`

- <span id="resourcedirectoryentrydata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcedirectoryentrydata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResourceNameOrId`

```rust
enum ResourceNameOrId {
    Name(ResourceName),
    Id(u16),
}
```

*Defined in [`object-0.37.3/src/read/pe/resource.rs:183-188`](../../../../.source_1765633015/object-0.37.3/src/read/pe/resource.rs#L183-L188)*

A resource name or ID.

Can be either a string or a numeric ID.

#### Variants

- **`Name`**

  A resource name.

- **`Id`**

  A resource ID.

#### Implementations

- <span id="resourcenameorid-name"></span>`fn name(self) -> Option<ResourceName>` — [`ResourceName`](#resourcename)

  Converts to an option of name.

  

  Helper for iterator filtering.

- <span id="resourcenameorid-id"></span>`fn id(self) -> Option<u16>`

  Converts to an option of ID.

  

  Helper for iterator filtering.

#### Trait Implementations

##### `impl Any for ResourceNameOrId`

- <span id="resourcenameorid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResourceNameOrId`

- <span id="resourcenameorid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResourceNameOrId`

- <span id="resourcenameorid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ResourceNameOrId`

- <span id="resourcenameorid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ResourceNameOrId`

- <span id="resourcenameorid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResourceNameOrId`

- <span id="resourcenameorid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ResourceNameOrId`

- <span id="resourcenameorid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resourcenameorid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResourceNameOrId`

- <span id="resourcenameorid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resourcenameorid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ImageNtHeaders`

```rust
trait ImageNtHeaders: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:589-671`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L589-L671)*

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

- [`ImageNtHeaders32`](../../pe/index.md#imagentheaders32)
- [`ImageNtHeaders64`](../../pe/index.md#imagentheaders64)

### `ImageOptionalHeader`

```rust
trait ImageOptionalHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:675-709`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L675-L709)*

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

- [`ImageOptionalHeader32`](../../pe/index.md#imageoptionalheader32)
- [`ImageOptionalHeader64`](../../pe/index.md#imageoptionalheader64)

### `ImageThunkData`

```rust
trait ImageThunkData: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/import.rs:191-207`](../../../../.source_1765633015/object-0.37.3/src/read/pe/import.rs#L191-L207)*

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

- [`ImageThunkData32`](../../pe/index.md#imagethunkdata32)
- [`ImageThunkData64`](../../pe/index.md#imagethunkdata64)

## Functions

### `optional_header_magic`

```rust
fn optional_header_magic<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<u16>
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:572-585`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L572-L585)*

Find the optional header and read its `magic` field.

It can be useful to know this magic value before trying to
fully parse the NT headers.

### `parse_ordinal`

```rust
fn parse_ordinal(digits: &[u8]) -> Option<u32>
```

*Defined in [`object-0.37.3/src/read/pe/export.rs:324-334`](../../../../.source_1765633015/object-0.37.3/src/read/pe/export.rs#L324-L334)*

### `memmem`

```rust
fn memmem(data: &[u8], needle: &[u8], align: usize) -> Option<usize>
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:84-92`](../../../../.source_1765633015/object-0.37.3/src/read/pe/rich.rs#L84-L92)*

Find the offset of the first occurrence of needle in the data.

The offset must have the given alignment.

## Type Aliases

### `PeFile32<'data, R>`

```rust
type PeFile32<'data, R> = PeFile<'data, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:26`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L26)*

A PE32 (32-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders32`](../../pe/index.md), and corresponds
to [`crate::FileKind::Pe32`](../../index.md).

### `PeFile64<'data, R>`

```rust
type PeFile64<'data, R> = PeFile<'data, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:31`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L31)*

A PE32+ (64-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders64`](../../pe/index.md), and corresponds
to [`crate::FileKind::Pe64`](../../index.md).

### `PeComdatIterator32<'data, 'file, R>`

```rust
type PeComdatIterator32<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:422-423`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L422-L423)*

An iterator for the COMDAT section groups in a [`PeFile32`](#pefile32).

### `PeComdatIterator64<'data, 'file, R>`

```rust
type PeComdatIterator64<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:425-426`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L425-L426)*

An iterator for the COMDAT section groups in a [`PeFile64`](#pefile64).

### `PeComdat32<'data, 'file, R>`

```rust
type PeComdat32<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:455-456`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L455-L456)*

A COMDAT section group in a [`PeFile32`](#pefile32).

### `PeComdat64<'data, 'file, R>`

```rust
type PeComdat64<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:458-459`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L458-L459)*

A COMDAT section group in a [`PeFile64`](#pefile64).

### `PeComdatSectionIterator32<'data, 'file, R>`

```rust
type PeComdatSectionIterator32<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:515-516`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L515-L516)*

An iterator for the sections in a COMDAT section group in a [`PeFile32`](#pefile32).

### `PeComdatSectionIterator64<'data, 'file, R>`

```rust
type PeComdatSectionIterator64<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:518-519`](../../../../.source_1765633015/object-0.37.3/src/read/pe/file.rs#L518-L519)*

An iterator for the sections in a COMDAT section group in a [`PeFile64`](#pefile64).

### `PeSegmentIterator32<'data, 'file, R>`

```rust
type PeSegmentIterator32<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:15-16`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L15-L16)*

An iterator for the loadable sections in a [`PeFile32`](super::PeFile32).

### `PeSegmentIterator64<'data, 'file, R>`

```rust
type PeSegmentIterator64<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:18-19`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L18-L19)*

An iterator for the loadable sections in a [`PeFile64`](super::PeFile64).

### `PeSegment32<'data, 'file, R>`

```rust
type PeSegment32<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:48-49`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L48-L49)*

A loadable section in a [`PeFile32`](super::PeFile32).

### `PeSegment64<'data, 'file, R>`

```rust
type PeSegment64<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:51-52`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L51-L52)*

A loadable section in a [`PeFile64`](super::PeFile64).

### `PeSectionIterator32<'data, 'file, R>`

```rust
type PeSectionIterator32<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:154-155`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L154-L155)*

An iterator for the sections in a [`PeFile32`](super::PeFile32).

### `PeSectionIterator64<'data, 'file, R>`

```rust
type PeSectionIterator64<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:157-158`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L157-L158)*

An iterator for the sections in a [`PeFile64`](super::PeFile64).

### `PeSection32<'data, 'file, R>`

```rust
type PeSection32<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:188-189`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L188-L189)*

A section in a [`PeFile32`](super::PeFile32).

### `PeSection64<'data, 'file, R>`

```rust
type PeSection64<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:191-192`](../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L191-L192)*

A section in a [`PeFile64`](super::PeFile64).

