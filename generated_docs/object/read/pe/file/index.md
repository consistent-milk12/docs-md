*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [file](index.md)*

---

# Module `file`

## Contents

- [Structs](#structs)
  - [`PeFile`](#pefile)
  - [`PeComdatIterator`](#pecomdatiterator)
  - [`PeComdat`](#pecomdat)
  - [`PeComdatSectionIterator`](#pecomdatsectioniterator)
- [Traits](#traits)
  - [`ImageNtHeaders`](#imagentheaders)
  - [`ImageOptionalHeader`](#imageoptionalheader)
- [Functions](#functions)
  - [`optional_header_magic`](#optional-header-magic)
- [Type Aliases](#type-aliases)
  - [`PeFile32`](#pefile32)
  - [`PeFile64`](#pefile64)
  - [`PeComdatIterator32`](#pecomdatiterator32)
  - [`PeComdatIterator64`](#pecomdatiterator64)
  - [`PeComdat32`](#pecomdat32)
  - [`PeComdat64`](#pecomdat64)
  - [`PeComdatSectionIterator32`](#pecomdatsectioniterator32)
  - [`PeComdatSectionIterator64`](#pecomdatsectioniterator64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PeFile`](#pefile) | struct | A PE image file. |
| [`PeComdatIterator`](#pecomdatiterator) | struct | An iterator for the COMDAT section groups in a [`PeFile`]. |
| [`PeComdat`](#pecomdat) | struct | A COMDAT section group in a [`PeFile`]. |
| [`PeComdatSectionIterator`](#pecomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`PeFile`]. |
| [`ImageNtHeaders`](#imagentheaders) | trait | A trait for generic access to [`pe::ImageNtHeaders32`] and [`pe::ImageNtHeaders64`]. |
| [`ImageOptionalHeader`](#imageoptionalheader) | trait | A trait for generic access to [`pe::ImageOptionalHeader32`] and [`pe::ImageOptionalHeader64`]. |
| [`optional_header_magic`](#optional-header-magic) | fn | Find the optional header and read its `magic` field. |
| [`PeFile32`](#pefile32) | type | A PE32 (32-bit) image file. |
| [`PeFile64`](#pefile64) | type | A PE32+ (64-bit) image file. |
| [`PeComdatIterator32`](#pecomdatiterator32) | type | An iterator for the COMDAT section groups in a [`PeFile32`]. |
| [`PeComdatIterator64`](#pecomdatiterator64) | type | An iterator for the COMDAT section groups in a [`PeFile64`]. |
| [`PeComdat32`](#pecomdat32) | type | A COMDAT section group in a [`PeFile32`]. |
| [`PeComdat64`](#pecomdat64) | type | A COMDAT section group in a [`PeFile64`]. |
| [`PeComdatSectionIterator32`](#pecomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`PeFile32`]. |
| [`PeComdatSectionIterator64`](#pecomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`PeFile64`]. |

## Structs

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:37-47`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L37-L47)*

A PE image file.

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- <span id="pefile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the raw PE file data.

- <span id="pefile-data"></span>`fn data(&self) -> R`

  Returns this binary data.

- <span id="pefile-dos-header"></span>`fn dos_header(&self) -> &'data pe::ImageDosHeader` — [`ImageDosHeader`](../../../pe/index.md#imagedosheader)

  Return the DOS header of this file.

- <span id="pefile-nt-headers"></span>`fn nt_headers(&self) -> &'data Pe`

  Return the NT Headers of this file.

- <span id="pefile-rich-header-info"></span>`fn rich_header_info(&self) -> Option<RichHeaderInfo<'_>>` — [`RichHeaderInfo`](../index.md#richheaderinfo)

  Returns information about the rich header of this file (if any).

- <span id="pefile-section-table"></span>`fn section_table(&self) -> SectionTable<'data>` — [`SectionTable`](../../coff/index.md#sectiontable)

  Returns the section table of this binary.

- <span id="pefile-data-directories"></span>`fn data_directories(&self) -> DataDirectories<'data>` — [`DataDirectories`](../index.md#datadirectories)

  Returns the data directories of this file.

- <span id="pefile-data-directory"></span>`fn data_directory(&self, id: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md#imagedatadirectory)

  Returns the data directory at the given index.

- <span id="pefile-export-table"></span>`fn export_table(&self) -> Result<Option<ExportTable<'data>>>` — [`Result`](../../../index.md#result), [`ExportTable`](../index.md#exporttable)

  Returns the export table of this file.

  

  The export table is located using the data directory.

- <span id="pefile-import-table"></span>`fn import_table(&self) -> Result<Option<ImportTable<'data>>>` — [`Result`](../../../index.md#result), [`ImportTable`](../index.md#importtable)

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

- <span id="pefile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

- <span id="pefile-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md#subarchitecture)

- <span id="pefile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="pefile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="pefile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../../index.md#objectkind)

- <span id="pefile-object-segments"></span>`fn segments(&self) -> PeSegmentIterator<'data, '_, Pe, R>` — [`PeSegmentIterator`](../index.md#pesegmentiterator)

- <span id="pefile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<PeSection<'data, 'file, Pe, R>>` — [`PeSection`](../index.md#pesection)

- <span id="pefile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<PeSection<'data, '_, Pe, R>>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`PeSection`](../index.md#pesection)

- <span id="pefile-object-sections"></span>`fn sections(&self) -> PeSectionIterator<'data, '_, Pe, R>` — [`PeSectionIterator`](../index.md#pesectioniterator)

- <span id="pefile-object-comdats"></span>`fn comdats(&self) -> PeComdatIterator<'data, '_, Pe, R>` — [`PeComdatIterator`](../index.md#pecomdatiterator)

- <span id="pefile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R>>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`CoffSymbol`](../../coff/index.md#coffsymbol)

- <span id="pefile-object-symbols"></span>`fn symbols(&self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../../coff/index.md#coffsymboliterator)

- <span id="pefile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../../coff/index.md#coffsymboltable)

- <span id="pefile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../../coff/index.md#coffsymboliterator)

- <span id="pefile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../../coff/index.md#coffsymboltable)

- <span id="pefile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../../index.md#nodynamicrelocationiterator)

- <span id="pefile-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../../index.md#result), [`Import`](../../../index.md#import)

- <span id="pefile-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md#result), [`Export`](../../../index.md#export)

- <span id="pefile-object-pdb-info"></span>`fn pdb_info(&self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../../index.md#result), [`CodeView`](../../../index.md#codeview)

- <span id="pefile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="pefile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="pefile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="pefile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../../index.md#fileflags)

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:432-439`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L432-L439)*

An iterator for the COMDAT section groups in a [`PeFile`](../index.md).

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:465-472`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L465-L472)*

A COMDAT section group in a [`PeFile`](../index.md).

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

- <span id="pecomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md#comdatkind)

- <span id="pecomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="pecomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="pecomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="pecomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md#objectcomdat)

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

*Defined in [`object-0.37.3/src/read/pe/file.rs:525-532`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L525-L532)*

An iterator for the sections in a COMDAT section group in a [`PeFile`](../index.md).

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

## Traits

### `ImageNtHeaders`

```rust
trait ImageNtHeaders: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:589-671`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L589-L671)*

A trait for generic access to [`pe::ImageNtHeaders32`](../../../pe/index.md) and [`pe::ImageNtHeaders64`](../../../pe/index.md).

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

- [`ImageNtHeaders32`](../../../pe/index.md#imagentheaders32)
- [`ImageNtHeaders64`](../../../pe/index.md#imagentheaders64)

### `ImageOptionalHeader`

```rust
trait ImageOptionalHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:675-709`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L675-L709)*

A trait for generic access to [`pe::ImageOptionalHeader32`](../../../pe/index.md) and [`pe::ImageOptionalHeader64`](../../../pe/index.md).

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

- [`ImageOptionalHeader32`](../../../pe/index.md#imageoptionalheader32)
- [`ImageOptionalHeader64`](../../../pe/index.md#imageoptionalheader64)

## Functions

### `optional_header_magic`

```rust
fn optional_header_magic<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<u16>
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:572-585`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L572-L585)*

Find the optional header and read its `magic` field.

It can be useful to know this magic value before trying to
fully parse the NT headers.

## Type Aliases

### `PeFile32<'data, R>`

```rust
type PeFile32<'data, R> = PeFile<'data, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:26`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L26)*

A PE32 (32-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders32`](../../../pe/index.md), and corresponds
to [`crate::FileKind::Pe32`](../../../index.md).

### `PeFile64<'data, R>`

```rust
type PeFile64<'data, R> = PeFile<'data, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:31`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L31)*

A PE32+ (64-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders64`](../../../pe/index.md), and corresponds
to [`crate::FileKind::Pe64`](../../../index.md).

### `PeComdatIterator32<'data, 'file, R>`

```rust
type PeComdatIterator32<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:422-423`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L422-L423)*

An iterator for the COMDAT section groups in a [`PeFile32`](../index.md).

### `PeComdatIterator64<'data, 'file, R>`

```rust
type PeComdatIterator64<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:425-426`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L425-L426)*

An iterator for the COMDAT section groups in a [`PeFile64`](../index.md).

### `PeComdat32<'data, 'file, R>`

```rust
type PeComdat32<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:455-456`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L455-L456)*

A COMDAT section group in a [`PeFile32`](../index.md).

### `PeComdat64<'data, 'file, R>`

```rust
type PeComdat64<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:458-459`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L458-L459)*

A COMDAT section group in a [`PeFile64`](../index.md).

### `PeComdatSectionIterator32<'data, 'file, R>`

```rust
type PeComdatSectionIterator32<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:515-516`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L515-L516)*

An iterator for the sections in a COMDAT section group in a [`PeFile32`](../index.md).

### `PeComdatSectionIterator64<'data, 'file, R>`

```rust
type PeComdatSectionIterator64<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/file.rs:518-519`](../../../../../.source_1765521767/object-0.37.3/src/read/pe/file.rs#L518-L519)*

An iterator for the sections in a COMDAT section group in a [`PeFile64`](../index.md).

