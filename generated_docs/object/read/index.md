*[object](../index.md) / [read](index.md)*

---

# Module `read`

Interface for reading object files.

## Unified read API

The [`Object`](#object) trait provides a unified read API for accessing common features of
object files, such as sections and symbols. There is an implementation of this
trait for [`File`](#file), which allows reading any file format, as well as implementations
for each file format:
[`ElfFile`](elf::ElfFile), [`MachOFile`](macho::MachOFile), [`CoffFile`](coff::CoffFile),
[`PeFile`](pe::PeFile), [`WasmFile`](wasm::WasmFile), [`XcoffFile`](xcoff::XcoffFile).

## Low level read API

The submodules for each file format define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

See the [submodules](#modules) for examples of the low level read API.

## Naming Convention

Types that form part of the unified API for a file format are prefixed with the
name of the file format.

## Example for unified read API
 ```no_run
use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(all(feature = "read", feature = "std"))] {
    let data = fs::read("path/to/binary")?;
    let file = object::File::parse(&*data)?;
    for section in file.sections() {
        println!("{}", section.name()?);
    }
  }
    Ok(())
}
```

## Contents

- [Modules](#modules)
  - [`read_ref`](#read-ref)
  - [`read_cache`](#read-cache)
  - [`util`](#util)
  - [`gnu_compression`](#gnu-compression)
  - [`any`](#any)
  - [`archive`](#archive)
  - [`coff`](#coff)
  - [`elf`](#elf)
  - [`macho`](#macho)
  - [`pe`](#pe)
  - [`xcoff`](#xcoff)
  - [`traits`](#traits)
  - [`private`](#private)
- [Structs](#structs)
  - [`Error`](#error)
  - [`SectionIndex`](#sectionindex)
  - [`SymbolIndex`](#symbolindex)
  - [`SymbolMap`](#symbolmap)
  - [`SymbolMapName`](#symbolmapname)
  - [`ObjectMap`](#objectmap)
  - [`ObjectMapEntry`](#objectmapentry)
  - [`ObjectMapFile`](#objectmapfile)
  - [`Import`](#import)
  - [`Export`](#export)
  - [`CodeView`](#codeview)
  - [`Relocation`](#relocation)
  - [`RelocationMap`](#relocationmap)
  - [`RelocationMapEntry`](#relocationmapentry)
  - [`CompressedFileRange`](#compressedfilerange)
  - [`CompressedData`](#compresseddata)
  - [`ReadCache`](#readcache)
  - [`ReadCacheInternal`](#readcacheinternal)
  - [`ReadCacheRange`](#readcacherange)
  - [`Bytes`](#bytes)
  - [`DebugByte`](#debugbyte)
  - [`DebugLen`](#debuglen)
  - [`ByteString`](#bytestring)
  - [`StringTable`](#stringtable)
  - [`SegmentIterator`](#segmentiterator)
  - [`Segment`](#segment)
  - [`SectionIterator`](#sectioniterator)
  - [`Section`](#section)
  - [`ComdatIterator`](#comdatiterator)
  - [`Comdat`](#comdat)
  - [`ComdatSectionIterator`](#comdatsectioniterator)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`Symbol`](#symbol)
  - [`DynamicRelocationIterator`](#dynamicrelocationiterator)
  - [`SectionRelocationIterator`](#sectionrelocationiterator)
  - [`NoDynamicRelocationIterator`](#nodynamicrelocationiterator)
- [Enums](#enums)
  - [`FileKind`](#filekind)
  - [`ObjectKind`](#objectkind)
  - [`SymbolSection`](#symbolsection)
  - [`RelocationTarget`](#relocationtarget)
  - [`CompressionFormat`](#compressionformat)
  - [`Architecture`](#architecture)
  - [`SubArchitecture`](#subarchitecture)
  - [`AddressSize`](#addresssize)
  - [`BinaryFormat`](#binaryformat)
  - [`SectionKind`](#sectionkind)
  - [`ComdatKind`](#comdatkind)
  - [`SymbolKind`](#symbolkind)
  - [`SymbolScope`](#symbolscope)
  - [`RelocationKind`](#relocationkind)
  - [`RelocationEncoding`](#relocationencoding)
  - [`FileFlags`](#fileflags)
  - [`SegmentFlags`](#segmentflags)
  - [`SectionFlags`](#sectionflags)
  - [`SymbolFlags`](#symbolflags)
  - [`RelocationFlags`](#relocationflags)
  - [`File`](#file)
  - [`SegmentIteratorInternal`](#segmentiteratorinternal)
  - [`SegmentInternal`](#segmentinternal)
  - [`SectionIteratorInternal`](#sectioniteratorinternal)
  - [`SectionInternal`](#sectioninternal)
  - [`ComdatIteratorInternal`](#comdatiteratorinternal)
  - [`ComdatInternal`](#comdatinternal)
  - [`ComdatSectionIteratorInternal`](#comdatsectioniteratorinternal)
  - [`SymbolTableInternal`](#symboltableinternal)
  - [`SymbolIteratorInternal`](#symboliteratorinternal)
  - [`SymbolInternal`](#symbolinternal)
  - [`DynamicRelocationIteratorInternal`](#dynamicrelocationiteratorinternal)
  - [`SectionRelocationIteratorInternal`](#sectionrelocationiteratorinternal)
- [Traits](#traits)
  - [`ReadError`](#readerror)
  - [`SymbolMapEntry`](#symbolmapentry)
  - [`ReadRef`](#readref)
  - [`ReadCacheOps`](#readcacheops)
  - [`Object`](#object)
  - [`ObjectSegment`](#objectsegment)
  - [`ObjectSection`](#objectsection)
  - [`ObjectComdat`](#objectcomdat)
  - [`ObjectSymbolTable`](#objectsymboltable)
  - [`ObjectSymbol`](#objectsymbol)
- [Functions](#functions)
  - [`debug_list_bytes`](#debug-list-bytes)
  - [`align`](#align)
  - [`data_range`](#data-range)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
  - [`NativeFile`](#nativefile)
  - [`Result`](#result)
- [Macros](#macros)
  - [`with_inner!`](#with-inner)
  - [`with_inner_mut!`](#with-inner-mut)
  - [`map_inner!`](#map-inner)
  - [`map_inner_option!`](#map-inner-option)
  - [`map_inner_option_mut!`](#map-inner-option-mut)
  - [`next_inner!`](#next-inner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`read_ref`](#read-ref) | mod |  |
| [`read_cache`](#read-cache) | mod |  |
| [`util`](#util) | mod |  |
| [`gnu_compression`](#gnu-compression) | mod |  |
| [`any`](#any) | mod |  |
| [`archive`](#archive) | mod | Support for archive files. |
| [`coff`](#coff) | mod | Support for reading Windows COFF files. |
| [`elf`](#elf) | mod | Support for reading ELF files. |
| [`macho`](#macho) | mod | Support for reading Mach-O files. |
| [`pe`](#pe) | mod | Support for reading PE files. |
| [`xcoff`](#xcoff) | mod | Support for reading AIX XCOFF files. |
| [`traits`](#traits) | mod |  |
| [`private`](#private) | mod |  |
| [`Error`](#error) | struct | The error type used within the read module. |
| [`SectionIndex`](#sectionindex) | struct | The index used to identify a section in a file. |
| [`SymbolIndex`](#symbolindex) | struct | The index used to identify a symbol in a symbol table. |
| [`SymbolMap`](#symbolmap) | struct | A map from addresses to symbol information. |
| [`SymbolMapName`](#symbolmapname) | struct | The type used for entries in a [`SymbolMap`] that maps from addresses to names. |
| [`ObjectMap`](#objectmap) | struct | A map from addresses to symbol names and object files. |
| [`ObjectMapEntry`](#objectmapentry) | struct | A symbol in an [`ObjectMap`]. |
| [`ObjectMapFile`](#objectmapfile) | struct | An object file name in an [`ObjectMap`]. |
| [`Import`](#import) | struct | An imported symbol. |
| [`Export`](#export) | struct | An exported symbol. |
| [`CodeView`](#codeview) | struct | PDB information from the debug directory in a PE file. |
| [`Relocation`](#relocation) | struct | A relocation entry. |
| [`RelocationMap`](#relocationmap) | struct | A map from section offsets to relocation information. |
| [`RelocationMapEntry`](#relocationmapentry) | struct |  |
| [`CompressedFileRange`](#compressedfilerange) | struct | A range in a file that may be compressed. |
| [`CompressedData`](#compresseddata) | struct | Data that may be compressed. |
| [`ReadCache`](#readcache) | struct | An implementation of [`ReadRef`] for data in a stream that implements `Read + Seek`. |
| [`ReadCacheInternal`](#readcacheinternal) | struct |  |
| [`ReadCacheRange`](#readcacherange) | struct | An implementation of [`ReadRef`] for a range of data in a stream that implements `Read + Seek`. |
| [`Bytes`](#bytes) | struct | A newtype for byte slices. |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |
| [`ByteString`](#bytestring) | struct | A newtype for byte strings. |
| [`StringTable`](#stringtable) | struct | A table of zero-terminated strings. |
| [`SegmentIterator`](#segmentiterator) | struct | An iterator for the loadable segments in a [`File`]. |
| [`Segment`](#segment) | struct | A loadable segment in a [`File`]. |
| [`SectionIterator`](#sectioniterator) | struct | An iterator for the sections in a [`File`]. |
| [`Section`](#section) | struct | A section in a [`File`]. |
| [`ComdatIterator`](#comdatiterator) | struct | An iterator for the COMDAT section groups in a [`File`]. |
| [`Comdat`](#comdat) | struct | A COMDAT section group in a [`File`]. |
| [`ComdatSectionIterator`](#comdatsectioniterator) | struct | An iterator for the sections in a [`Comdat`]. |
| [`SymbolTable`](#symboltable) | struct | A symbol table in a [`File`]. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for the symbols in a [`SymbolTable`]. |
| [`Symbol`](#symbol) | struct | An symbol in a [`SymbolTable`]. |
| [`DynamicRelocationIterator`](#dynamicrelocationiterator) | struct | An iterator for the dynamic relocation entries in a [`File`]. |
| [`SectionRelocationIterator`](#sectionrelocationiterator) | struct | An iterator for the relocation entries in a [`Section`]. |
| [`NoDynamicRelocationIterator`](#nodynamicrelocationiterator) | struct | An iterator for files that don't have dynamic relocations. |
| [`FileKind`](#filekind) | enum | A file format kind. |
| [`ObjectKind`](#objectkind) | enum | An object kind. |
| [`SymbolSection`](#symbolsection) | enum | The section where an [`ObjectSymbol`] is defined. |
| [`RelocationTarget`](#relocationtarget) | enum | The target referenced by a [`Relocation`]. |
| [`CompressionFormat`](#compressionformat) | enum | A data compression format. |
| [`Architecture`](#architecture) | enum | A CPU architecture. |
| [`SubArchitecture`](#subarchitecture) | enum | A CPU sub-architecture. |
| [`AddressSize`](#addresssize) | enum | The size of an address value for an architecture. |
| [`BinaryFormat`](#binaryformat) | enum | A binary file format. |
| [`SectionKind`](#sectionkind) | enum | The kind of a section. |
| [`ComdatKind`](#comdatkind) | enum | The selection kind for a COMDAT section group. |
| [`SymbolKind`](#symbolkind) | enum | The kind of a symbol. |
| [`SymbolScope`](#symbolscope) | enum | A symbol scope. |
| [`RelocationKind`](#relocationkind) | enum | The operation used to calculate the result of the relocation. |
| [`RelocationEncoding`](#relocationencoding) | enum | Information about how the result of the relocation operation is encoded in the place. |
| [`FileFlags`](#fileflags) | enum | File flags that are specific to each file format. |
| [`SegmentFlags`](#segmentflags) | enum | Segment flags that are specific to each file format. |
| [`SectionFlags`](#sectionflags) | enum | Section flags that are specific to each file format. |
| [`SymbolFlags`](#symbolflags) | enum | Symbol flags that are specific to each file format. |
| [`RelocationFlags`](#relocationflags) | enum | Relocation fields that are specific to each file format and architecture. |
| [`File`](#file) | enum | An object file that can be any supported file format. |
| [`SegmentIteratorInternal`](#segmentiteratorinternal) | enum |  |
| [`SegmentInternal`](#segmentinternal) | enum |  |
| [`SectionIteratorInternal`](#sectioniteratorinternal) | enum |  |
| [`SectionInternal`](#sectioninternal) | enum |  |
| [`ComdatIteratorInternal`](#comdatiteratorinternal) | enum |  |
| [`ComdatInternal`](#comdatinternal) | enum |  |
| [`ComdatSectionIteratorInternal`](#comdatsectioniteratorinternal) | enum |  |
| [`SymbolTableInternal`](#symboltableinternal) | enum |  |
| [`SymbolIteratorInternal`](#symboliteratorinternal) | enum |  |
| [`SymbolInternal`](#symbolinternal) | enum |  |
| [`DynamicRelocationIteratorInternal`](#dynamicrelocationiteratorinternal) | enum |  |
| [`SectionRelocationIteratorInternal`](#sectionrelocationiteratorinternal) | enum |  |
| [`ReadError`](#readerror) | trait |  |
| [`SymbolMapEntry`](#symbolmapentry) | trait | An entry in a [`SymbolMap`]. |
| [`ReadRef`](#readref) | trait | A trait for reading references to [`Pod`] types from a block of data. |
| [`ReadCacheOps`](#readcacheops) | trait | Operations required to implement [`ReadCache`]. |
| [`Object`](#object) | trait | An object file. |
| [`ObjectSegment`](#objectsegment) | trait | A loadable segment in an [`Object`]. |
| [`ObjectSection`](#objectsection) | trait | A section in an [`Object`]. |
| [`ObjectComdat`](#objectcomdat) | trait | A COMDAT section group in an [`Object`]. |
| [`ObjectSymbolTable`](#objectsymboltable) | trait | A symbol table in an [`Object`]. |
| [`ObjectSymbol`](#objectsymbol) | trait | A symbol table entry in an [`Object`]. |
| [`debug_list_bytes`](#debug-list-bytes) | fn |  |
| [`align`](#align) | fn |  |
| [`data_range`](#data-range) | fn |  |
| [`Result`](#result) | type | The result type used within the read module. |
| [`NativeFile`](#nativefile) | type | The native executable file for the target platform. |
| [`Result`](#result) | type |  |
| [`with_inner!`](#with-inner) | macro | Evaluate an expression on the contents of a file format enum. |
| [`with_inner_mut!`](#with-inner-mut) | macro |  |
| [`map_inner!`](#map-inner) | macro | Like `with_inner!`, but wraps the result in another enum. |
| [`map_inner_option!`](#map-inner-option) | macro | Like `map_inner!`, but the result is a Result or Option. |
| [`map_inner_option_mut!`](#map-inner-option-mut) | macro |  |
| [`next_inner!`](#next-inner) | macro | Call `next` for a file format iterator. |

## Modules

- [`read_ref`](read_ref/index.md)
- [`read_cache`](read_cache/index.md)
- [`util`](util/index.md)
- [`gnu_compression`](gnu_compression/index.md)
- [`any`](any/index.md)
- [`archive`](archive/index.md) — Support for archive files.
- [`coff`](coff/index.md) — Support for reading Windows COFF files.
- [`elf`](elf/index.md) — Support for reading ELF files.
- [`macho`](macho/index.md) — Support for reading Mach-O files.
- [`pe`](pe/index.md) — Support for reading PE files.
- [`xcoff`](xcoff/index.md) — Support for reading AIX XCOFF files.
- [`traits`](traits/index.md)
- [`private`](private/index.md)

## Structs

### `Error`

```rust
struct Error(&'static str);
```

*Defined in [`object-0.37.3/src/read/mod.rs:116`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L116)*

The error type used within the read module.

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](../index.md#error)

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](../index.md#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToOwned for Error`

- <span id="error-toowned-type-owned"></span>`type Owned = T`

- <span id="error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionIndex`

```rust
struct SectionIndex(usize);
```

*Defined in [`object-0.37.3/src/read/mod.rs:389`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L389)*

The index used to identify a section in a file.

#### Trait Implementations

##### `impl Any for SectionIndex`

- <span id="sectionindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionIndex`

- <span id="sectionindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionIndex`

- <span id="sectionindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionIndex`

- <span id="sectionindex-clone"></span>`fn clone(&self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

##### `impl CloneToUninit for SectionIndex`

- <span id="sectionindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionIndex`

##### `impl Debug for SectionIndex`

- <span id="sectionindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SectionIndex`

- <span id="sectionindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionIndex`

##### `impl<T> From for SectionIndex`

- <span id="sectionindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SectionIndex`

- <span id="sectionindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SectionIndex`

- <span id="sectionindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SectionIndex`

- <span id="sectionindex-partialeq-eq"></span>`fn eq(&self, other: &SectionIndex) -> bool` — [`SectionIndex`](../index.md#sectionindex)

##### `impl StructuralPartialEq for SectionIndex`

##### `impl ToOwned for SectionIndex`

- <span id="sectionindex-toowned-type-owned"></span>`type Owned = T`

- <span id="sectionindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectionindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for SectionIndex`

- <span id="sectionindex-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for SectionIndex`

- <span id="sectionindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionIndex`

- <span id="sectionindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolIndex`

```rust
struct SymbolIndex(usize);
```

*Defined in [`object-0.37.3/src/read/mod.rs:399`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L399)*

The index used to identify a symbol in a symbol table.

#### Trait Implementations

##### `impl Any for SymbolIndex`

- <span id="symbolindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIndex`

- <span id="symbolindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIndex`

- <span id="symbolindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolIndex`

- <span id="symbolindex-clone"></span>`fn clone(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

##### `impl CloneToUninit for SymbolIndex`

- <span id="symbolindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SymbolIndex`

##### `impl Debug for SymbolIndex`

- <span id="symbolindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SymbolIndex`

- <span id="symbolindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolIndex`

##### `impl<T> From for SymbolIndex`

- <span id="symbolindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SymbolIndex`

- <span id="symbolindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolIndex`

- <span id="symbolindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SymbolIndex`

- <span id="symbolindex-partialeq-eq"></span>`fn eq(&self, other: &SymbolIndex) -> bool` — [`SymbolIndex`](../index.md#symbolindex)

##### `impl StructuralPartialEq for SymbolIndex`

##### `impl ToOwned for SymbolIndex`

- <span id="symbolindex-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for SymbolIndex`

- <span id="symbolindex-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for SymbolIndex`

- <span id="symbolindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIndex`

- <span id="symbolindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolMap<T: SymbolMapEntry>`

```rust
struct SymbolMap<T: SymbolMapEntry> {
    symbols: alloc::vec::Vec<T>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:451-453`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L451-L453)*

A map from addresses to symbol information.

The symbol information depends on the chosen entry type, such as [`SymbolMapName`](../index.md).

Returned by `Object::symbol_map`.

#### Implementations

- <span id="symbolmap-new"></span>`fn new(symbols: Vec<T>) -> Self`

  Construct a new symbol map.

  

  This function will sort the symbols by address.

- <span id="symbolmap-get"></span>`fn get(&self, address: u64) -> Option<&T>`

  Get the symbol before the given address.

- <span id="symbolmap-symbols"></span>`fn symbols(&self) -> &[T]`

  Get all symbols in the map.

#### Trait Implementations

##### `impl<T> Any for SymbolMap<T>`

- <span id="symbolmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolMap<T>`

- <span id="symbolmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolMap<T>`

- <span id="symbolmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + SymbolMapEntry> Clone for SymbolMap<T>`

- <span id="symbolmap-clone"></span>`fn clone(&self) -> SymbolMap<T>` — [`SymbolMap`](../index.md#symbolmap)

##### `impl<T> CloneToUninit for SymbolMap<T>`

- <span id="symbolmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug + SymbolMapEntry> Debug for SymbolMap<T>`

- <span id="symbolmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default + SymbolMapEntry> Default for SymbolMap<T>`

- <span id="symbolmap-default"></span>`fn default() -> SymbolMap<T>` — [`SymbolMap`](../index.md#symbolmap)

##### `impl<T> From for SymbolMap<T>`

- <span id="symbolmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for SymbolMap<T>`

- <span id="symbolmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToOwned for SymbolMap<T>`

- <span id="symbolmap-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for SymbolMap<T>`

- <span id="symbolmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SymbolMap<T>`

- <span id="symbolmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolMapName<'data>`

```rust
struct SymbolMapName<'data> {
    address: u64,
    name: &'data str,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:485-488`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L485-L488)*

The type used for entries in a [`SymbolMap`](../index.md) that maps from addresses to names.

#### Implementations

- <span id="symbolmapname-new"></span>`fn new(address: u64, name: &'data str) -> Self`

  Construct a `SymbolMapName`.

- <span id="symbolmapname-address"></span>`fn address(&self) -> u64`

  The symbol address.

- <span id="symbolmapname-name"></span>`fn name(&self) -> &'data str`

  The symbol name.

#### Trait Implementations

##### `impl Any for SymbolMapName<'data>`

- <span id="symbolmapname-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolMapName<'data>`

- <span id="symbolmapname-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolMapName<'data>`

- <span id="symbolmapname-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolMapName<'data>`

- <span id="symbolmapname-clone"></span>`fn clone(&self) -> SymbolMapName<'data>` — [`SymbolMapName`](../index.md#symbolmapname)

##### `impl CloneToUninit for SymbolMapName<'data>`

- <span id="symbolmapname-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SymbolMapName<'data>`

##### `impl Debug for SymbolMapName<'data>`

- <span id="symbolmapname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolMapName<'data>`

##### `impl<T> From for SymbolMapName<'data>`

- <span id="symbolmapname-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SymbolMapName<'data>`

- <span id="symbolmapname-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolMapName<'data>`

- <span id="symbolmapname-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SymbolMapName<'data>`

- <span id="symbolmapname-partialeq-eq"></span>`fn eq(&self, other: &SymbolMapName<'data>) -> bool` — [`SymbolMapName`](../index.md#symbolmapname)

##### `impl StructuralPartialEq for SymbolMapName<'data>`

##### `impl SymbolMapEntry for SymbolMapName<'data>`

- <span id="symbolmapname-symbolmapentry-address"></span>`fn address(&self) -> u64`

##### `impl ToOwned for SymbolMapName<'data>`

- <span id="symbolmapname-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolmapname-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolmapname-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolMapName<'data>`

- <span id="symbolmapname-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolmapname-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolMapName<'data>`

- <span id="symbolmapname-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolmapname-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ObjectMap<'data>`

```rust
struct ObjectMap<'data> {
    symbols: SymbolMap<ObjectMapEntry<'data>>,
    objects: alloc::vec::Vec<ObjectMapFile<'data>>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:522-525`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L522-L525)*

A map from addresses to symbol names and object files.

This is derived from STAB entries in Mach-O files.

Returned by `Object::object_map`.

#### Implementations

- <span id="objectmap-get"></span>`fn get(&self, address: u64) -> Option<&ObjectMapEntry<'data>>` — [`ObjectMapEntry`](../index.md#objectmapentry)

  Get the entry containing the given address.

- <span id="objectmap-symbols"></span>`fn symbols(&self) -> &[ObjectMapEntry<'data>]` — [`ObjectMapEntry`](../index.md#objectmapentry)

  Get all symbols in the map.

- <span id="objectmap-objects"></span>`fn objects(&self) -> &[ObjectMapFile<'data>]` — [`ObjectMapFile`](../index.md#objectmapfile)

  Get all objects in the map.

#### Trait Implementations

##### `impl Any for ObjectMap<'data>`

- <span id="objectmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ObjectMap<'data>`

- <span id="objectmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ObjectMap<'data>`

- <span id="objectmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ObjectMap<'data>`

- <span id="objectmap-clone"></span>`fn clone(&self) -> ObjectMap<'data>` — [`ObjectMap`](../index.md#objectmap)

##### `impl CloneToUninit for ObjectMap<'data>`

- <span id="objectmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ObjectMap<'data>`

- <span id="objectmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ObjectMap<'data>`

- <span id="objectmap-default"></span>`fn default() -> ObjectMap<'data>` — [`ObjectMap`](../index.md#objectmap)

##### `impl<T> From for ObjectMap<'data>`

- <span id="objectmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ObjectMap<'data>`

- <span id="objectmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ObjectMap<'data>`

- <span id="objectmap-toowned-type-owned"></span>`type Owned = T`

- <span id="objectmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="objectmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ObjectMap<'data>`

- <span id="objectmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="objectmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ObjectMap<'data>`

- <span id="objectmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="objectmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ObjectMapEntry<'data>`

```rust
struct ObjectMapEntry<'data> {
    address: u64,
    size: u64,
    name: &'data [u8],
    object: usize,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:550-555`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L550-L555)*

A symbol in an [`ObjectMap`](../index.md).

#### Implementations

- <span id="objectmapentry-address"></span>`fn address(&self) -> u64`

  Get the symbol address.

- <span id="objectmapentry-size"></span>`fn size(&self) -> u64`

  Get the symbol size.

  

  This may be 0 if the size is unknown.

- <span id="objectmapentry-name"></span>`fn name(&self) -> &'data [u8]`

  Get the symbol name.

- <span id="objectmapentry-object-index"></span>`fn object_index(&self) -> usize`

  Get the index of the object file name.

- <span id="objectmapentry-object"></span>`fn object<'a>(&self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>` — [`ObjectMap`](../index.md#objectmap), [`ObjectMapFile`](../index.md#objectmapfile)

  Get the object file name.

#### Trait Implementations

##### `impl Any for ObjectMapEntry<'data>`

- <span id="objectmapentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ObjectMapEntry<'data>`

- <span id="objectmapentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ObjectMapEntry<'data>`

- <span id="objectmapentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ObjectMapEntry<'data>`

- <span id="objectmapentry-clone"></span>`fn clone(&self) -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../index.md#objectmapentry)

##### `impl CloneToUninit for ObjectMapEntry<'data>`

- <span id="objectmapentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ObjectMapEntry<'data>`

##### `impl Debug for ObjectMapEntry<'data>`

- <span id="objectmapentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ObjectMapEntry<'data>`

- <span id="objectmapentry-default"></span>`fn default() -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../index.md#objectmapentry)

##### `impl Eq for ObjectMapEntry<'data>`

##### `impl<T> From for ObjectMapEntry<'data>`

- <span id="objectmapentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ObjectMapEntry<'data>`

- <span id="objectmapentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ObjectMapEntry<'data>`

- <span id="objectmapentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ObjectMapEntry<'data>`

- <span id="objectmapentry-partialeq-eq"></span>`fn eq(&self, other: &ObjectMapEntry<'data>) -> bool` — [`ObjectMapEntry`](../index.md#objectmapentry)

##### `impl StructuralPartialEq for ObjectMapEntry<'data>`

##### `impl SymbolMapEntry for ObjectMapEntry<'data>`

- <span id="objectmapentry-symbolmapentry-address"></span>`fn address(&self) -> u64`

##### `impl ToOwned for ObjectMapEntry<'data>`

- <span id="objectmapentry-toowned-type-owned"></span>`type Owned = T`

- <span id="objectmapentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="objectmapentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ObjectMapEntry<'data>`

- <span id="objectmapentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="objectmapentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ObjectMapEntry<'data>`

- <span id="objectmapentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="objectmapentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ObjectMapFile<'data>`

```rust
struct ObjectMapFile<'data> {
    path: &'data [u8],
    member: Option<&'data [u8]>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:600-603`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L600-L603)*

An object file name in an [`ObjectMap`](../index.md).

#### Implementations

- <span id="objectmapfile-new"></span>`fn new(path: &'data [u8], member: Option<&'data [u8]>) -> Self`

- <span id="objectmapfile-path"></span>`fn path(&self) -> &'data [u8]`

  Get the path to the file containing the object.

- <span id="objectmapfile-member"></span>`fn member(&self) -> Option<&'data [u8]>`

  If the file is an archive, get the name of the member containing the object.

#### Trait Implementations

##### `impl Any for ObjectMapFile<'data>`

- <span id="objectmapfile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ObjectMapFile<'data>`

- <span id="objectmapfile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ObjectMapFile<'data>`

- <span id="objectmapfile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ObjectMapFile<'data>`

- <span id="objectmapfile-clone"></span>`fn clone(&self) -> ObjectMapFile<'data>` — [`ObjectMapFile`](../index.md#objectmapfile)

##### `impl CloneToUninit for ObjectMapFile<'data>`

- <span id="objectmapfile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ObjectMapFile<'data>`

##### `impl Debug for ObjectMapFile<'data>`

- <span id="objectmapfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectMapFile<'data>`

##### `impl<T> From for ObjectMapFile<'data>`

- <span id="objectmapfile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ObjectMapFile<'data>`

- <span id="objectmapfile-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ObjectMapFile<'data>`

- <span id="objectmapfile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ObjectMapFile<'data>`

- <span id="objectmapfile-partialeq-eq"></span>`fn eq(&self, other: &ObjectMapFile<'data>) -> bool` — [`ObjectMapFile`](../index.md#objectmapfile)

##### `impl StructuralPartialEq for ObjectMapFile<'data>`

##### `impl ToOwned for ObjectMapFile<'data>`

- <span id="objectmapfile-toowned-type-owned"></span>`type Owned = T`

- <span id="objectmapfile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="objectmapfile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ObjectMapFile<'data>`

- <span id="objectmapfile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="objectmapfile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ObjectMapFile<'data>`

- <span id="objectmapfile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="objectmapfile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Import<'data>`

```rust
struct Import<'data> {
    library: ByteString<'data>,
    name: ByteString<'data>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:628-632`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L628-L632)*

An imported symbol.

Returned by `Object::imports`.

#### Implementations

- <span id="import-name"></span>`fn name(&self) -> &'data [u8]`

  The symbol name.

- <span id="import-library"></span>`fn library(&self) -> &'data [u8]`

  The name of the library to import the symbol from.

#### Trait Implementations

##### `impl Any for Import<'data>`

- <span id="import-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Import<'data>`

- <span id="import-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Import<'data>`

- <span id="import-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Import<'data>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'data>` — [`Import`](../index.md#import)

##### `impl CloneToUninit for Import<'data>`

- <span id="import-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Import<'data>`

##### `impl Debug for Import<'data>`

- <span id="import-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Import<'data>`

##### `impl<T> From for Import<'data>`

- <span id="import-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Import<'data>`

- <span id="import-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Import<'data>`

- <span id="import-partialeq-eq"></span>`fn eq(&self, other: &Import<'data>) -> bool` — [`Import`](../index.md#import)

##### `impl StructuralPartialEq for Import<'data>`

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

### `Export<'data>`

```rust
struct Export<'data> {
    name: ByteString<'data>,
    address: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:652-656`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L652-L656)*

An exported symbol.

Returned by `Object::exports`.

#### Implementations

- <span id="export-name"></span>`fn name(&self) -> &'data [u8]`

  The symbol name.

- <span id="export-address"></span>`fn address(&self) -> u64`

  The virtual address of the symbol.

#### Trait Implementations

##### `impl Any for Export<'data>`

- <span id="export-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Export<'data>`

- <span id="export-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Export<'data>`

- <span id="export-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Export<'data>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'data>` — [`Export`](../index.md#export)

##### `impl CloneToUninit for Export<'data>`

- <span id="export-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Export<'data>`

##### `impl Debug for Export<'data>`

- <span id="export-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Export<'data>`

##### `impl<T> From for Export<'data>`

- <span id="export-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Export<'data>`

- <span id="export-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Export<'data>`

- <span id="export-partialeq-eq"></span>`fn eq(&self, other: &Export<'data>) -> bool` — [`Export`](../index.md#export)

##### `impl StructuralPartialEq for Export<'data>`

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

### `CodeView<'data>`

```rust
struct CodeView<'data> {
    guid: [u8; 16],
    path: ByteString<'data>,
    age: u32,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:674-678`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L674-L678)*

PDB information from the debug directory in a PE file.

#### Implementations

- <span id="codeview-path"></span>`fn path(&self) -> &'data [u8]`

  The path to the PDB as stored in CodeView.

- <span id="codeview-age"></span>`fn age(&self) -> u32`

  The age of the PDB.

- <span id="codeview-guid"></span>`fn guid(&self) -> [u8; 16]`

  The GUID of the PDB.

#### Trait Implementations

##### `impl Any for CodeView<'data>`

- <span id="codeview-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CodeView<'data>`

- <span id="codeview-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CodeView<'data>`

- <span id="codeview-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CodeView<'data>`

- <span id="codeview-clone"></span>`fn clone(&self) -> CodeView<'data>` — [`CodeView`](../index.md#codeview)

##### `impl CloneToUninit for CodeView<'data>`

- <span id="codeview-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CodeView<'data>`

##### `impl Debug for CodeView<'data>`

- <span id="codeview-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CodeView<'data>`

##### `impl<T> From for CodeView<'data>`

- <span id="codeview-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CodeView<'data>`

- <span id="codeview-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CodeView<'data>`

- <span id="codeview-partialeq-eq"></span>`fn eq(&self, other: &CodeView<'data>) -> bool` — [`CodeView`](../index.md#codeview)

##### `impl StructuralPartialEq for CodeView<'data>`

##### `impl ToOwned for CodeView<'data>`

- <span id="codeview-toowned-type-owned"></span>`type Owned = T`

- <span id="codeview-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="codeview-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CodeView<'data>`

- <span id="codeview-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="codeview-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CodeView<'data>`

- <span id="codeview-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="codeview-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Relocation`

```rust
struct Relocation {
    kind: RelocationKind,
    encoding: RelocationEncoding,
    size: u8,
    target: RelocationTarget,
    addend: i64,
    implicit_addend: bool,
    flags: RelocationFlags,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:716-724`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L716-L724)*

A relocation entry.

Returned by `Object::dynamic_relocations` or `ObjectSection::relocations`.

#### Implementations

- <span id="relocation-kind"></span>`fn kind(&self) -> RelocationKind` — [`RelocationKind`](../index.md#relocationkind)

  The operation used to calculate the result of the relocation.

- <span id="relocation-encoding"></span>`fn encoding(&self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md#relocationencoding)

  Information about how the result of the relocation operation is encoded in the place.

- <span id="relocation-size"></span>`fn size(&self) -> u8`

  The size in bits of the place of the relocation.

  

  If 0, then the size is determined by the relocation kind.

- <span id="relocation-target"></span>`fn target(&self) -> RelocationTarget` — [`RelocationTarget`](../index.md#relocationtarget)

  The target of the relocation.

- <span id="relocation-addend"></span>`fn addend(&self) -> i64`

  The addend to use in the relocation calculation.

- <span id="relocation-set-addend"></span>`fn set_addend(&mut self, addend: i64)`

  Set the addend to use in the relocation calculation.

- <span id="relocation-has-implicit-addend"></span>`fn has_implicit_addend(&self) -> bool`

  Returns true if there is an implicit addend stored in the data at the offset

  to be relocated.

- <span id="relocation-flags"></span>`fn flags(&self) -> RelocationFlags` — [`RelocationFlags`](../index.md#relocationflags)

  Relocation flags that are specific to each file format.

  

  The values returned by `kind`, `encoding` and `size` are derived

  from these flags.

#### Trait Implementations

##### `impl Any for Relocation`

- <span id="relocation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Relocation`

- <span id="relocation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Relocation`

- <span id="relocation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Relocation`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Relocation`

- <span id="relocation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Relocation`

- <span id="relocation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Relocation`

- <span id="relocation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Relocation`

- <span id="relocation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationMap`

```rust
struct RelocationMap(alloc::collections::btree_map::BTreeMap<u64, RelocationMapEntry>);
```

*Defined in [`object-0.37.3/src/read/mod.rs:790`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L790)*

A map from section offsets to relocation information.

This can be used to apply relocations to a value at a given section offset.
This is intended for use with DWARF in relocatable object files, and only
supports relocations that are used in DWARF.

Returned by `ObjectSection::relocation_map`.

#### Implementations

- <span id="relocationmap-new"></span>`fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>` — [`Object`](#object), [`Result`](../index.md#result)

  Construct a new relocation map for a section.

  

  Fails if any relocation cannot be added to the map.

  You can manually use `add` if you need different error handling,

  such as to list all errors or to ignore them.

- <span id="relocationmap-add"></span>`fn add<'data: 'file, 'file, T>(&mut self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>` — [`Relocation`](../index.md#relocation), [`Result`](../index.md#result)

  Add a single relocation to the map.

- <span id="relocationmap-relocate"></span>`fn relocate(&self, offset: u64, value: u64) -> u64`

  Relocate a value that was read from the section at the given offset.

#### Trait Implementations

##### `impl Any for RelocationMap`

- <span id="relocationmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationMap`

- <span id="relocationmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationMap`

- <span id="relocationmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RelocationMap`

- <span id="relocationmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationMap`

- <span id="relocationmap-default"></span>`fn default() -> RelocationMap` — [`RelocationMap`](../index.md#relocationmap)

##### `impl<T> From for RelocationMap`

- <span id="relocationmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationMap`

- <span id="relocationmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RelocationMap`

- <span id="relocationmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationMap`

- <span id="relocationmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationMapEntry`

```rust
struct RelocationMapEntry {
    implicit_addend: bool,
    addend: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:871-874`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L871-L874)*

#### Trait Implementations

##### `impl Any for RelocationMapEntry`

- <span id="relocationmapentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationMapEntry`

- <span id="relocationmapentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationMapEntry`

- <span id="relocationmapentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationMapEntry`

- <span id="relocationmapentry-clone"></span>`fn clone(&self) -> RelocationMapEntry` — [`RelocationMapEntry`](#relocationmapentry)

##### `impl CloneToUninit for RelocationMapEntry`

- <span id="relocationmapentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationMapEntry`

##### `impl Debug for RelocationMapEntry`

- <span id="relocationmapentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationMapEntry`

##### `impl<T> From for RelocationMapEntry`

- <span id="relocationmapentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationMapEntry`

- <span id="relocationmapentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationMapEntry`

- <span id="relocationmapentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationMapEntry`

- <span id="relocationmapentry-partialeq-eq"></span>`fn eq(&self, other: &RelocationMapEntry) -> bool` — [`RelocationMapEntry`](#relocationmapentry)

##### `impl StructuralPartialEq for RelocationMapEntry`

##### `impl ToOwned for RelocationMapEntry`

- <span id="relocationmapentry-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationmapentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationmapentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationMapEntry`

- <span id="relocationmapentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationmapentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationMapEntry`

- <span id="relocationmapentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationmapentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CompressedFileRange`

```rust
struct CompressedFileRange {
    pub format: CompressionFormat,
    pub offset: u64,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:898-907`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L898-L907)*

A range in a file that may be compressed.

Returned by `ObjectSection::compressed_file_range`.

#### Fields

- **`format`**: `CompressionFormat`

  The data compression format.

- **`offset`**: `u64`

  The file offset of the compressed data.

- **`compressed_size`**: `u64`

  The compressed data size.

- **`uncompressed_size`**: `u64`

  The uncompressed data size.

#### Implementations

- <span id="compressedfilerange-none"></span>`fn none(range: Option<(u64, u64)>) -> Self`

  Data that is uncompressed.

- <span id="compressedfilerange-data"></span>`fn data<'data, R: ReadRef<'data>>(self, file: R) -> Result<CompressedData<'data>>` — [`Result`](../index.md#result), [`CompressedData`](../index.md#compresseddata)

  Convert to [`CompressedData`](../index.md) by reading from the file.

#### Trait Implementations

##### `impl Any for CompressedFileRange`

- <span id="compressedfilerange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CompressedFileRange`

- <span id="compressedfilerange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CompressedFileRange`

- <span id="compressedfilerange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CompressedFileRange`

- <span id="compressedfilerange-clone"></span>`fn clone(&self) -> CompressedFileRange` — [`CompressedFileRange`](../index.md#compressedfilerange)

##### `impl CloneToUninit for CompressedFileRange`

- <span id="compressedfilerange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CompressedFileRange`

##### `impl Debug for CompressedFileRange`

- <span id="compressedfilerange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressedFileRange`

##### `impl<T> From for CompressedFileRange`

- <span id="compressedfilerange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for CompressedFileRange`

- <span id="compressedfilerange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for CompressedFileRange`

- <span id="compressedfilerange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CompressedFileRange`

- <span id="compressedfilerange-partialeq-eq"></span>`fn eq(&self, other: &CompressedFileRange) -> bool` — [`CompressedFileRange`](../index.md#compressedfilerange)

##### `impl StructuralPartialEq for CompressedFileRange`

##### `impl ToOwned for CompressedFileRange`

- <span id="compressedfilerange-toowned-type-owned"></span>`type Owned = T`

- <span id="compressedfilerange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="compressedfilerange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CompressedFileRange`

- <span id="compressedfilerange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compressedfilerange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CompressedFileRange`

- <span id="compressedfilerange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compressedfilerange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CompressedData<'data>`

```rust
struct CompressedData<'data> {
    pub format: CompressionFormat,
    pub data: &'data [u8],
    pub uncompressed_size: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:947-954`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L947-L954)*

Data that may be compressed.

Returned by `ObjectSection::compressed_data`.

#### Fields

- **`format`**: `CompressionFormat`

  The data compression format.

- **`data`**: `&'data [u8]`

  The compressed data.

- **`uncompressed_size`**: `u64`

  The uncompressed data size.

#### Implementations

- <span id="compresseddata-none"></span>`fn none(data: &'data [u8]) -> Self`

  Data that is uncompressed.

- <span id="compresseddata-decompress"></span>`fn decompress(self) -> Result<Cow<'data, [u8]>>` — [`Result`](../index.md#result)

  Return the uncompressed data.

  

  Returns an error for invalid data or unsupported compression.

  This includes if the data is compressed but the `compression` feature

  for this crate is disabled.

#### Trait Implementations

##### `impl Any for CompressedData<'data>`

- <span id="compresseddata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CompressedData<'data>`

- <span id="compresseddata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CompressedData<'data>`

- <span id="compresseddata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CompressedData<'data>`

- <span id="compresseddata-clone"></span>`fn clone(&self) -> CompressedData<'data>` — [`CompressedData`](../index.md#compresseddata)

##### `impl CloneToUninit for CompressedData<'data>`

- <span id="compresseddata-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CompressedData<'data>`

##### `impl Debug for CompressedData<'data>`

- <span id="compresseddata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressedData<'data>`

##### `impl<T> From for CompressedData<'data>`

- <span id="compresseddata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for CompressedData<'data>`

- <span id="compresseddata-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for CompressedData<'data>`

- <span id="compresseddata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CompressedData<'data>`

- <span id="compresseddata-partialeq-eq"></span>`fn eq(&self, other: &CompressedData<'data>) -> bool` — [`CompressedData`](../index.md#compresseddata)

##### `impl StructuralPartialEq for CompressedData<'data>`

##### `impl ToOwned for CompressedData<'data>`

- <span id="compresseddata-toowned-type-owned"></span>`type Owned = T`

- <span id="compresseddata-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="compresseddata-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CompressedData<'data>`

- <span id="compresseddata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compresseddata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CompressedData<'data>`

- <span id="compresseddata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compresseddata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReadCache<R: ReadCacheOps>`

```rust
struct ReadCache<R: ReadCacheOps> {
    cache: core::cell::RefCell<ReadCacheInternal<R>>,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:31-33`](../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L31-L33)*

An implementation of [`ReadRef`](#readref) for data in a stream that implements
`Read + Seek`.

Contains a cache of read-only blocks of data, allowing references to
them to be returned. Entries in the cache are never removed.
Entries are keyed on the offset and size of the read.
Currently overlapping reads are considered separate reads.

This is primarily intended for environments where memory mapped files
are not available or not suitable, such as WebAssembly.

Note that malformed files can cause the cache to grow much larger than
the file size.

#### Implementations

- <span id="readcache-new"></span>`fn new(read: R) -> Self`

  Create an empty `ReadCache` for the given stream.

- <span id="readcache-range"></span>`fn range(&self, offset: u64, size: u64) -> ReadCacheRange<'_, R>` — [`ReadCacheRange`](#readcacherange)

  Return an implementation of `ReadRef` that restricts reads

  to the given range of the stream.

- <span id="readcache-clear"></span>`fn clear(&mut self)`

  Free buffers used by the cache.

- <span id="readcache-into-inner"></span>`fn into_inner(self) -> R`

  Unwrap this `ReadCache<R>`, returning the underlying reader.

#### Trait Implementations

##### `impl Any for ReadCache<R>`

- <span id="readcache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadCache<R>`

- <span id="readcache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadCache<R>`

- <span id="readcache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCache<R>`

- <span id="readcache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadCache<R>`

- <span id="readcache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadCache<R>`

- <span id="readcache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadCacheOps> ReadRef for &'a ReadCache<R>`

- <span id="a-readcache-readref-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="a-readcache-readref-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="a-readcache-readref-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

##### `impl<U> TryFrom for ReadCache<R>`

- <span id="readcache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readcache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadCache<R>`

- <span id="readcache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readcache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReadCacheInternal<R: ReadCacheOps>`

```rust
struct ReadCacheInternal<R: ReadCacheOps> {
    read: R,
    bufs: alloc::collections::btree_map::BTreeMap<(u64, u64), alloc::boxed::Box<[u8]>>,
    strings: alloc::collections::btree_map::BTreeMap<(u64, u8), alloc::boxed::Box<[u8]>>,
    len: Option<u64>,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:36-41`](../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L36-L41)*

#### Implementations

- <span id="readcacheinternal-range-in-bounds"></span>`fn range_in_bounds(&mut self, range: &Range<u64>) -> Result<(), ()>`

  Ensures this range is contained in the len of the file

- <span id="readcacheinternal-len"></span>`fn len(&mut self) -> Result<u64, ()>`

  The length of the underlying read, memoized

#### Trait Implementations

##### `impl Any for ReadCacheInternal<R>`

- <span id="readcacheinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadCacheInternal<R>`

- <span id="readcacheinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadCacheInternal<R>`

- <span id="readcacheinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCacheInternal<R>`

- <span id="readcacheinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadCacheInternal<R>`

- <span id="readcacheinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadCacheInternal<R>`

- <span id="readcacheinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReadCacheInternal<R>`

- <span id="readcacheinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readcacheinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadCacheInternal<R>`

- <span id="readcacheinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readcacheinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReadCacheRange<'a, R: ReadCacheOps>`

```rust
struct ReadCacheRange<'a, R: ReadCacheOps> {
    r: &'a ReadCache<R>,
    offset: u64,
    size: u64,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:172-176`](../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L172-L176)*

An implementation of [`ReadRef`](#readref) for a range of data in a stream that
implements `Read + Seek`.

Shares an underlying [`ReadCache`](#readcache) with a lifetime of `'a`.

#### Trait Implementations

##### `impl Any for ReadCacheRange<'a, R>`

- <span id="readcacherange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadCacheRange<'a, R>`

- <span id="readcacherange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadCacheRange<'a, R>`

- <span id="readcacherange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadCacheOps> Clone for ReadCacheRange<'a, R>`

- <span id="readcacherange-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ReadCacheRange<'a, R>`

- <span id="readcacherange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: ReadCacheOps> Copy for ReadCacheRange<'a, R>`

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCacheRange<'a, R>`

- <span id="readcacherange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadCacheRange<'a, R>`

- <span id="readcacherange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadCacheRange<'a, R>`

- <span id="readcacherange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadCacheOps> ReadRef for ReadCacheRange<'a, R>`

- <span id="readcacherange-readref-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="readcacherange-readref-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="readcacherange-readref-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

##### `impl ToOwned for ReadCacheRange<'a, R>`

- <span id="readcacherange-toowned-type-owned"></span>`type Owned = T`

- <span id="readcacherange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="readcacherange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReadCacheRange<'a, R>`

- <span id="readcacherange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readcacherange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadCacheRange<'a, R>`

- <span id="readcacherange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readcacherange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Bytes<'data>`

```rust
struct Bytes<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:16`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L16)*

A newtype for byte slices.

It has these important features:
- no methods that can panic, such as `Index`
- convenience methods for `Pod` types
- a useful `Debug` implementation

#### Implementations

- <span id="bytes-len"></span>`fn len(&self) -> usize`

  Return the length of the byte slice.

- <span id="bytes-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the byte slice is empty.

- <span id="bytes-skip"></span>`fn skip(&mut self, offset: usize) -> Result<(), ()>`

  Skip over the given number of bytes at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes.

- <span id="bytes-read-bytes"></span>`fn read_bytes(&mut self, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](#bytes)

  Return a reference to the given number of bytes at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes.

- <span id="bytes-read-bytes-at"></span>`fn read_bytes_at(self, offset: usize, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](#bytes)

  Return a reference to the given number of bytes at the given offset of the byte slice.

  

  Returns an error if the offset is invalid or there are too few bytes.

- <span id="bytes-read"></span>`fn read<T: Pod>(&mut self) -> Result<&'data T, ()>`

  Return a reference to a `Pod` struct at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes or the slice is incorrectly aligned.

- <span id="bytes-read-at"></span>`fn read_at<T: Pod>(self, offset: usize) -> Result<&'data T, ()>`

  Return a reference to a `Pod` struct at the given offset of the byte slice.

  

  Returns an error if there are too few bytes or the offset is incorrectly aligned.

- <span id="bytes-read-slice"></span>`fn read_slice<T: Pod>(&mut self, count: usize) -> Result<&'data [T], ()>`

  Return a reference to a slice of `Pod` structs at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes or the offset is incorrectly aligned.

- <span id="bytes-read-slice-at"></span>`fn read_slice_at<T: Pod>(self, offset: usize, count: usize) -> Result<&'data [T], ()>`

  Return a reference to a slice of `Pod` structs at the given offset of the byte slice.

  

  Returns an error if there are too few bytes or the offset is incorrectly aligned.

- <span id="bytes-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8], ()>`

  Read a null terminated string.

  

  Does not assume any encoding.

  Reads past the null byte, but doesn't return it.

- <span id="bytes-read-string-at"></span>`fn read_string_at(self, offset: usize) -> Result<&'data [u8], ()>`

  Read a null terminated string at an offset.

  

  Does not assume any encoding. Does not return the null byte.

- <span id="bytes-read-uleb128"></span>`fn read_uleb128(&mut self) -> Result<u64, ()>`

  Read an unsigned LEB128 number.

- <span id="bytes-read-sleb128"></span>`fn read_sleb128(&mut self) -> Result<i64, ()>`

  Read a signed LEB128 number.

#### Trait Implementations

##### `impl Any for Bytes<'data>`

- <span id="bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bytes<'data>`

- <span id="bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bytes<'data>`

- <span id="bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Bytes<'data>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'data>` — [`Bytes`](#bytes)

##### `impl CloneToUninit for Bytes<'data>`

- <span id="bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Bytes<'data>`

##### `impl Debug for Bytes<'data>`

- <span id="bytes-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bytes<'data>`

- <span id="bytes-default"></span>`fn default() -> Bytes<'data>` — [`Bytes`](#bytes)

##### `impl Eq for Bytes<'data>`

##### `impl<T> From for Bytes<'data>`

- <span id="bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bytes<'data>`

- <span id="bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Bytes<'data>`

- <span id="bytes-partialeq-eq"></span>`fn eq(&self, other: &Bytes<'data>) -> bool` — [`Bytes`](#bytes)

##### `impl StructuralPartialEq for Bytes<'data>`

##### `impl ToOwned for Bytes<'data>`

- <span id="bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Bytes<'data>`

- <span id="bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bytes<'data>`

- <span id="bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`object-0.37.3/src/read/util.rs:222`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L222)*

#### Trait Implementations

##### `impl Any for DebugByte`

- <span id="debugbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugByte`

- <span id="debugbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugByte`

- <span id="debugbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugByte`

- <span id="debugbyte-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugByte`

- <span id="debugbyte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugByte`

- <span id="debugbyte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugByte`

- <span id="debugbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugByte`

- <span id="debugbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLen`

```rust
struct DebugLen(usize);
```

*Defined in [`object-0.37.3/src/read/util.rs:230`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L230)*

#### Trait Implementations

##### `impl Any for DebugLen`

- <span id="debuglen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLen`

- <span id="debuglen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLen`

- <span id="debuglen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugLen`

- <span id="debuglen-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugLen`

- <span id="debuglen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLen`

- <span id="debuglen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugLen`

- <span id="debuglen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLen`

- <span id="debuglen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteString<'data>`

```rust
struct ByteString<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:244`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L244)*

A newtype for byte strings.

For byte slices that are strings of an unknown encoding.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

#### Trait Implementations

##### `impl Any for ByteString<'data>`

- <span id="bytestring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteString<'data>`

- <span id="bytestring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteString<'data>`

- <span id="bytestring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ByteString<'data>`

- <span id="bytestring-clone"></span>`fn clone(&self) -> ByteString<'data>` — [`ByteString`](util/index.md#bytestring)

##### `impl CloneToUninit for ByteString<'data>`

- <span id="bytestring-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ByteString<'data>`

##### `impl Debug for ByteString<'data>`

- <span id="bytestring-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteString<'data>`

- <span id="bytestring-default"></span>`fn default() -> ByteString<'data>` — [`ByteString`](util/index.md#bytestring)

##### `impl Eq for ByteString<'data>`

##### `impl<T> From for ByteString<'data>`

- <span id="bytestring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteString<'data>`

- <span id="bytestring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ByteString<'data>`

- <span id="bytestring-partialeq-eq"></span>`fn eq(&self, other: &ByteString<'data>) -> bool` — [`ByteString`](util/index.md#bytestring)

##### `impl StructuralPartialEq for ByteString<'data>`

##### `impl ToOwned for ByteString<'data>`

- <span id="bytestring-toowned-type-owned"></span>`type Owned = T`

- <span id="bytestring-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bytestring-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ByteString<'data>`

- <span id="bytestring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytestring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteString<'data>`

- <span id="bytestring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytestring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StringTable<'data, R>`

```rust
struct StringTable<'data, R>
where
    R: ReadRef<'data> {
    data: Option<R>,
    start: u64,
    end: u64,
    marker: core::marker::PhantomData<&'data ()>,
}
```

*Defined in [`object-0.37.3/src/read/util.rs:274-282`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L274-L282)*

A table of zero-terminated strings.

This is used by most file formats for strings such as section names and symbol names.

#### Implementations

- <span id="stringtable-new"></span>`fn new(data: R, start: u64, end: u64) -> Self`

  Interpret the given data as a string table.

- <span id="stringtable-get"></span>`fn get(&self, offset: u32) -> Result<&'data [u8], ()>`

  Return the string at the given offset.

#### Trait Implementations

##### `impl Any for StringTable<'data, R>`

- <span id="stringtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StringTable<'data, R>`

- <span id="stringtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StringTable<'data, R>`

- <span id="stringtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Clone for StringTable<'data, R>`

- <span id="stringtable-clone"></span>`fn clone(&self) -> StringTable<'data, R>` — [`StringTable`](#stringtable)

##### `impl CloneToUninit for StringTable<'data, R>`

- <span id="stringtable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R> Copy for StringTable<'data, R>`

##### `impl<R> Debug for StringTable<'data, R>`

- <span id="stringtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> Default for StringTable<'data, R>`

- <span id="stringtable-default"></span>`fn default() -> Self`

##### `impl<T> From for StringTable<'data, R>`

- <span id="stringtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StringTable<'data, R>`

- <span id="stringtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StringTable<'data, R>`

- <span id="stringtable-toowned-type-owned"></span>`type Owned = T`

- <span id="stringtable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stringtable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StringTable<'data, R>`

- <span id="stringtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stringtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StringTable<'data, R>`

- <span id="stringtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stringtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SegmentIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:532-534`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L532-L534)*

An iterator for the loadable segments in a [`File`](#file).

#### Trait Implementations

##### `impl Any for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="segmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="segmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-iterator-type-item"></span>`type Item = Segment<'data, 'file, R>`

- <span id="segmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Segment<'data, 'file, R: ReadRef<'data>>`

```rust
struct Segment<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:574-576`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L574-L576)*

A loadable segment in a [`File`](#file).

Most functionality is provided by the [`ObjectSegment`](#objectsegment) trait implementation.

#### Trait Implementations

##### `impl Any for Segment<'data, 'file, R>`

- <span id="segment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Segment<'data, 'file, R>`

- <span id="segment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Segment<'data, 'file, R>`

- <span id="segment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Segment<'data, 'file, R>`

- <span id="segment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Segment<'data, 'file, R>`

- <span id="segment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Segment<'data, 'file, R>`

- <span id="segment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSegment for Segment<'data, 'file, R>`

- <span id="segment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="segment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="segment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="segment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="segment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../index.md#result)

- <span id="segment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md#result)

- <span id="segment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../index.md#result)

- <span id="segment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../index.md#result)

- <span id="segment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl<R: ReadRef<'data>> Sealed for Segment<'data, 'file, R>`

##### `impl<U> TryFrom for Segment<'data, 'file, R>`

- <span id="segment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Segment<'data, 'file, R>`

- <span id="segment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:665-667`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L665-L667)*

An iterator for the sections in a [`File`](#file).

#### Trait Implementations

##### `impl Any for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-iterator-type-item"></span>`type Item = Section<'data, 'file, R>`

- <span id="sectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Section<'data, 'file, R: ReadRef<'data>>`

```rust
struct Section<'data, 'file, R: ReadRef<'data>> {
    inner: SectionInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:708-710`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L708-L710)*

A section in a [`File`](#file).

Most functionality is provided by the [`ObjectSection`](#objectsection) trait implementation.

#### Trait Implementations

##### `impl Any for Section<'data, 'file, R>`

- <span id="section-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Section<'data, 'file, R>`

- <span id="section-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Section<'data, 'file, R>`

- <span id="section-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Section<'data, 'file, R>`

- <span id="section-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Section<'data, 'file, R>`

- <span id="section-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Section<'data, 'file, R>`

- <span id="section-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSection for Section<'data, 'file, R>`

- <span id="section-objectsection-type-relocationiterator"></span>`type RelocationIterator = SectionRelocationIterator<'data, 'file, R>`

- <span id="section-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../index.md#sectionindex)

- <span id="section-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="section-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="section-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="section-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="section-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../index.md#result)

- <span id="section-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md#result)

- <span id="section-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../index.md#result), [`CompressedFileRange`](../index.md#compressedfilerange)

- <span id="section-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../index.md#result), [`CompressedData`](../index.md#compresseddata)

- <span id="section-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../index.md#result)

- <span id="section-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../index.md#result)

- <span id="section-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../index.md#result)

- <span id="section-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../index.md#result)

- <span id="section-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../index.md#sectionkind)

- <span id="section-objectsection-relocations"></span>`fn relocations(&self) -> SectionRelocationIterator<'data, 'file, R>` — [`SectionRelocationIterator`](#sectionrelocationiterator)

- <span id="section-objectsection-relocation-map"></span>`fn relocation_map(&self) -> Result<RelocationMap>` — [`Result`](../index.md#result), [`RelocationMap`](../index.md#relocationmap)

- <span id="section-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../index.md#sectionflags)

##### `impl<R: ReadRef<'data>> Sealed for Section<'data, 'file, R>`

##### `impl<U> TryFrom for Section<'data, 'file, R>`

- <span id="section-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="section-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Section<'data, 'file, R>`

- <span id="section-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="section-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:843-845`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L843-L845)*

An iterator for the COMDAT section groups in a [`File`](#file).

#### Trait Implementations

##### `impl Any for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-iterator-type-item"></span>`type Item = Comdat<'data, 'file, R>`

- <span id="comdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Comdat<'data, 'file, R: ReadRef<'data>>`

```rust
struct Comdat<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:885-887`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L885-L887)*

A COMDAT section group in a [`File`](#file).

Most functionality is provided by the [`ObjectComdat`](#objectcomdat) trait implementation.

#### Trait Implementations

##### `impl Any for Comdat<'data, 'file, R>`

- <span id="comdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Comdat<'data, 'file, R>`

- <span id="comdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Comdat<'data, 'file, R>`

- <span id="comdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Comdat<'data, 'file, R>`

- <span id="comdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Comdat<'data, 'file, R>`

- <span id="comdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Comdat<'data, 'file, R>`

- <span id="comdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectComdat for Comdat<'data, 'file, R>`

- <span id="comdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = ComdatSectionIterator<'data, 'file, R>`

- <span id="comdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../index.md#comdatkind)

- <span id="comdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

- <span id="comdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../index.md#result)

- <span id="comdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../index.md#result)

- <span id="comdat-objectcomdat-sections"></span>`fn sections(&self) -> ComdatSectionIterator<'data, 'file, R>` — [`ComdatSectionIterator`](#comdatsectioniterator)

##### `impl<R: ReadRef<'data>> Sealed for Comdat<'data, 'file, R>`

##### `impl<U> TryFrom for Comdat<'data, 'file, R>`

- <span id="comdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Comdat<'data, 'file, R>`

- <span id="comdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatSectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatSectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatSectionIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:959-961`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L959-L961)*

An iterator for the sections in a [`Comdat`](#comdat).

#### Trait Implementations

##### `impl Any for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="comdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolTable<'data, 'file, R>`

```rust
struct SymbolTable<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolTableInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1001-1006`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1001-L1006)*

A symbol table in a [`File`](#file).

Most functionality is provided by the [`ObjectSymbolTable`](#objectsymboltable) trait implementation.

#### Trait Implementations

##### `impl Any for SymbolTable<'data, 'file, R>`

- <span id="symboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTable<'data, 'file, R>`

- <span id="symboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTable<'data, 'file, R>`

- <span id="symboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolTable<'data, 'file, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolTable<'data, 'file, R>`

- <span id="symboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTable<'data, 'file, R>`

- <span id="symboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSymbolTable for SymbolTable<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](#objectsymboltable)

- <span id="symboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../index.md#symbolindex), [`Result`](../index.md#result), [`ObjectSymbolTable`](#objectsymboltable)

##### `impl<R: ReadRef<'data>> Sealed for SymbolTable<'data, 'file, R>`

##### `impl<U> TryFrom for SymbolTable<'data, 'file, R>`

- <span id="symboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTable<'data, 'file, R>`

- <span id="symboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolIterator<'data, 'file, R>`

```rust
struct SymbolIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1085-1090`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1085-L1090)*

An iterator for the symbols in a [`SymbolTable`](#symboltable).

#### Trait Implementations

##### `impl Any for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-iterator-type-item"></span>`type Item = Symbol<'data, 'file, R>`

- <span id="symboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Symbol<'data, 'file, R>`

```rust
struct Symbol<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1165-1170`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1165-L1170)*

An symbol in a [`SymbolTable`](#symboltable).

Most functionality is provided by the [`ObjectSymbol`](#objectsymbol) trait implementation.

#### Trait Implementations

##### `impl Any for Symbol<'data, 'file, R>`

- <span id="symbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Symbol<'data, 'file, R>`

- <span id="symbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Symbol<'data, 'file, R>`

- <span id="symbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Symbol<'data, 'file, R>`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Symbol<'data, 'file, R>`

- <span id="symbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Symbol<'data, 'file, R>`

- <span id="symbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSymbol for Symbol<'data, 'file, R>`

- <span id="symbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md#symbolindex)

- <span id="symbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../index.md#result)

- <span id="symbol-objectsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../index.md#result)

- <span id="symbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="symbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="symbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../index.md#symbolkind)

- <span id="symbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../index.md#symbolsection)

- <span id="symbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="symbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="symbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="symbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="symbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../index.md#symbolscope)

- <span id="symbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="symbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="symbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../index.md#symbolflags), [`SectionIndex`](../index.md#sectionindex), [`SymbolIndex`](../index.md#symbolindex)

##### `impl<R: ReadRef<'data>> Sealed for Symbol<'data, 'file, R>`

##### `impl<U> TryFrom for Symbol<'data, 'file, R>`

- <span id="symbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Symbol<'data, 'file, R>`

- <span id="symbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DynamicRelocationIterator<'data, 'file, R>`

```rust
struct DynamicRelocationIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: DynamicRelocationIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1301-1306`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1301-L1306)*

An iterator for the dynamic relocation entries in a [`File`](#file).

#### Trait Implementations

##### `impl Any for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dynamicrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="dynamicrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dynamicrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dynamicrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionRelocationIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionRelocationIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionRelocationIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1338-1340`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1338-L1340)*

An iterator for the relocation entries in a [`Section`](#section).

#### Trait Implementations

##### `impl Any for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="sectionrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NoDynamicRelocationIterator`

```rust
struct NoDynamicRelocationIterator;
```

*Defined in [`object-0.37.3/src/read/traits.rs:580`](../../../.source_1765521767/object-0.37.3/src/read/traits.rs#L580)*

An iterator for files that don't have dynamic relocations.

#### Trait Implementations

##### `impl Any for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nodynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nodynamicrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="nodynamicrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nodynamicrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nodynamicrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `FileKind`

```rust
enum FileKind {
    Archive,
    Coff,
    CoffBig,
    CoffImport,
    DyldCache,
    Elf32,
    Elf64,
    MachO32,
    MachO64,
    MachOFat32,
    MachOFat64,
    Pe32,
    Pe64,
    Xcoff32,
    Xcoff64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:198-281`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L198-L281)*

A file format kind.

#### Variants

- **`Archive`**

  A Unix archive.
  
  See [`archive::ArchiveFile`](archive/index.md).

- **`Coff`**

  A COFF object file.
  
  See [`coff::CoffFile`](coff/index.md).

- **`CoffBig`**

  A COFF bigobj object file.
  
  This supports a larger number of sections.
  
  See [`coff::CoffBigFile`](coff/index.md).

- **`CoffImport`**

  A Windows short import file.
  
  See [`coff::ImportFile`](coff/index.md).

- **`DyldCache`**

  A dyld cache file containing Mach-O images.
  
  See [`macho::DyldCache`](macho/index.md)

- **`Elf32`**

  A 32-bit ELF file.
  
  See [`elf::ElfFile32`](elf/index.md).

- **`Elf64`**

  A 64-bit ELF file.
  
  See [`elf::ElfFile64`](elf/index.md).

- **`MachO32`**

  A 32-bit Mach-O file.
  
  See [`macho::MachOFile32`](macho/index.md).

- **`MachO64`**

  A 64-bit Mach-O file.
  
  See [`macho::MachOFile64`](macho/index.md).

- **`MachOFat32`**

  A 32-bit Mach-O fat binary.
  
  See [`macho::MachOFatFile32`](macho/index.md).

- **`MachOFat64`**

  A 64-bit Mach-O fat binary.
  
  See [`macho::MachOFatFile64`](macho/index.md).

- **`Pe32`**

  A 32-bit PE file.
  
  See [`pe::PeFile32`](pe/index.md).

- **`Pe64`**

  A 64-bit PE file.
  
  See [`pe::PeFile64`](pe/index.md).

- **`Xcoff32`**

  A 32-bit XCOFF file.
  
  See [`xcoff::XcoffFile32`](xcoff/index.md).

- **`Xcoff64`**

  A 64-bit XCOFF file.
  
  See [`xcoff::XcoffFile64`](xcoff/index.md).

#### Implementations

- <span id="filekind-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<FileKind>` — [`Result`](../index.md#result), [`FileKind`](../index.md#filekind)

  Determine a file kind by parsing the start of the file.

- <span id="filekind-parse-at"></span>`fn parse_at<'data, R: ReadRef<'data>>(data: R, offset: u64) -> Result<FileKind>` — [`Result`](../index.md#result), [`FileKind`](../index.md#filekind)

  Determine a file kind by parsing at the given offset.

#### Trait Implementations

##### `impl Any for FileKind`

- <span id="filekind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FileKind`

- <span id="filekind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FileKind`

- <span id="filekind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FileKind`

- <span id="filekind-clone"></span>`fn clone(&self) -> FileKind` — [`FileKind`](../index.md#filekind)

##### `impl CloneToUninit for FileKind`

- <span id="filekind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FileKind`

##### `impl Debug for FileKind`

- <span id="filekind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileKind`

##### `impl<T> From for FileKind`

- <span id="filekind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FileKind`

- <span id="filekind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FileKind`

- <span id="filekind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FileKind`

- <span id="filekind-partialeq-eq"></span>`fn eq(&self, other: &FileKind) -> bool` — [`FileKind`](../index.md#filekind)

##### `impl StructuralPartialEq for FileKind`

##### `impl ToOwned for FileKind`

- <span id="filekind-toowned-type-owned"></span>`type Owned = T`

- <span id="filekind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filekind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FileKind`

- <span id="filekind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filekind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FileKind`

- <span id="filekind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filekind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ObjectKind`

```rust
enum ObjectKind {
    Unknown,
    Relocatable,
    Executable,
    Dynamic,
    Core,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:374-385`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L374-L385)*

An object kind.

Returned by `Object::kind`.

#### Variants

- **`Unknown`**

  The object kind is unknown.

- **`Relocatable`**

  Relocatable object.

- **`Executable`**

  Executable.

- **`Dynamic`**

  Dynamic shared object.

- **`Core`**

  Core.

#### Trait Implementations

##### `impl Any for ObjectKind`

- <span id="objectkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ObjectKind`

- <span id="objectkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ObjectKind`

- <span id="objectkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ObjectKind`

- <span id="objectkind-clone"></span>`fn clone(&self) -> ObjectKind` — [`ObjectKind`](../index.md#objectkind)

##### `impl CloneToUninit for ObjectKind`

- <span id="objectkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ObjectKind`

##### `impl Debug for ObjectKind`

- <span id="objectkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectKind`

##### `impl<T> From for ObjectKind`

- <span id="objectkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ObjectKind`

- <span id="objectkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ObjectKind`

- <span id="objectkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ObjectKind`

- <span id="objectkind-partialeq-eq"></span>`fn eq(&self, other: &ObjectKind) -> bool` — [`ObjectKind`](../index.md#objectkind)

##### `impl StructuralPartialEq for ObjectKind`

##### `impl ToOwned for ObjectKind`

- <span id="objectkind-toowned-type-owned"></span>`type Owned = T`

- <span id="objectkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="objectkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ObjectKind`

- <span id="objectkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="objectkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ObjectKind`

- <span id="objectkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="objectkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolSection`

```rust
enum SymbolSection {
    Unknown,
    None,
    Undefined,
    Absolute,
    Common,
    Section(SectionIndex),
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:410-423`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L410-L423)*

The section where an [`ObjectSymbol`](#objectsymbol) is defined.

#### Variants

- **`Unknown`**

  The section is unknown.

- **`None`**

  The section is not applicable for this symbol (such as file symbols).

- **`Undefined`**

  The symbol is undefined.

- **`Absolute`**

  The symbol has an absolute value.

- **`Common`**

  The symbol is a zero-initialized symbol that will be combined with duplicate definitions.

- **`Section`**

  The symbol is defined in the given section.

#### Implementations

- <span id="symbolsection-index"></span>`fn index(self) -> Option<SectionIndex>` — [`SectionIndex`](../index.md#sectionindex)

  Returns the section index for the section where the symbol is defined.

  

  May return `None` if the symbol is not defined in a section.

#### Trait Implementations

##### `impl Any for SymbolSection`

- <span id="symbolsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolSection`

- <span id="symbolsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolSection`

- <span id="symbolsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolSection`

- <span id="symbolsection-clone"></span>`fn clone(&self) -> SymbolSection` — [`SymbolSection`](../index.md#symbolsection)

##### `impl CloneToUninit for SymbolSection`

- <span id="symbolsection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SymbolSection`

##### `impl Debug for SymbolSection`

- <span id="symbolsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolSection`

##### `impl<T> From for SymbolSection`

- <span id="symbolsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SymbolSection`

- <span id="symbolsection-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolSection`

- <span id="symbolsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SymbolSection`

- <span id="symbolsection-partialeq-eq"></span>`fn eq(&self, other: &SymbolSection) -> bool` — [`SymbolSection`](../index.md#symbolsection)

##### `impl StructuralPartialEq for SymbolSection`

##### `impl ToOwned for SymbolSection`

- <span id="symbolsection-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolsection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolsection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolSection`

- <span id="symbolsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolSection`

- <span id="symbolsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationTarget`

```rust
enum RelocationTarget {
    Symbol(SymbolIndex),
    Section(SectionIndex),
    Absolute,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:703-710`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L703-L710)*

The target referenced by a [`Relocation`](../index.md).

#### Variants

- **`Symbol`**

  The target is a symbol.

- **`Section`**

  The target is a section.

- **`Absolute`**

  The offset is an absolute address.

#### Trait Implementations

##### `impl Any for RelocationTarget`

- <span id="relocationtarget-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationTarget`

- <span id="relocationtarget-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationTarget`

- <span id="relocationtarget-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationTarget`

- <span id="relocationtarget-clone"></span>`fn clone(&self) -> RelocationTarget` — [`RelocationTarget`](../index.md#relocationtarget)

##### `impl CloneToUninit for RelocationTarget`

- <span id="relocationtarget-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationTarget`

##### `impl Debug for RelocationTarget`

- <span id="relocationtarget-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationTarget`

##### `impl<T> From for RelocationTarget`

- <span id="relocationtarget-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationTarget`

- <span id="relocationtarget-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationTarget`

- <span id="relocationtarget-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationTarget`

- <span id="relocationtarget-partialeq-eq"></span>`fn eq(&self, other: &RelocationTarget) -> bool` — [`RelocationTarget`](../index.md#relocationtarget)

##### `impl StructuralPartialEq for RelocationTarget`

##### `impl ToOwned for RelocationTarget`

- <span id="relocationtarget-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationtarget-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationtarget-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationTarget`

- <span id="relocationtarget-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationtarget-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationTarget`

- <span id="relocationtarget-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationtarget-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CompressionFormat`

```rust
enum CompressionFormat {
    None,
    Unknown,
    Zlib,
    Zstandard,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:879-892`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L879-L892)*

A data compression format.

#### Variants

- **`None`**

  The data is uncompressed.

- **`Unknown`**

  The data is compressed, but the compression format is unknown.

- **`Zlib`**

  ZLIB/DEFLATE.
  
  Used for ELF compression and GNU compressed debug information.

- **`Zstandard`**

  Zstandard.
  
  Used for ELF compression.

#### Trait Implementations

##### `impl Any for CompressionFormat`

- <span id="compressionformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CompressionFormat`

- <span id="compressionformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CompressionFormat`

- <span id="compressionformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CompressionFormat`

- <span id="compressionformat-clone"></span>`fn clone(&self) -> CompressionFormat` — [`CompressionFormat`](../index.md#compressionformat)

##### `impl CloneToUninit for CompressionFormat`

- <span id="compressionformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CompressionFormat`

##### `impl Debug for CompressionFormat`

- <span id="compressionformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressionFormat`

##### `impl<T> From for CompressionFormat`

- <span id="compressionformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for CompressionFormat`

- <span id="compressionformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for CompressionFormat`

- <span id="compressionformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CompressionFormat`

- <span id="compressionformat-partialeq-eq"></span>`fn eq(&self, other: &CompressionFormat) -> bool` — [`CompressionFormat`](../index.md#compressionformat)

##### `impl StructuralPartialEq for CompressionFormat`

##### `impl ToOwned for CompressionFormat`

- <span id="compressionformat-toowned-type-owned"></span>`type Owned = T`

- <span id="compressionformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="compressionformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CompressionFormat`

- <span id="compressionformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compressionformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CompressionFormat`

- <span id="compressionformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compressionformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Architecture`

```rust
enum Architecture {
    Unknown,
    Aarch64,
    Aarch64_Ilp32,
    Alpha,
    Arm,
    Avr,
    Bpf,
    Csky,
    E2K32,
    E2K64,
    I386,
    X86_64,
    X86_64_X32,
    Hexagon,
    Hppa,
    LoongArch32,
    LoongArch64,
    M68k,
    Mips,
    Mips64,
    Mips64_N32,
    Msp430,
    PowerPc,
    PowerPc64,
    Riscv32,
    Riscv64,
    S390x,
    Sbf,
    Sharc,
    Sparc,
    Sparc32Plus,
    Sparc64,
    SuperH,
    Wasm32,
    Wasm64,
    Xtensa,
}
```

*Defined in [`object-0.37.3/src/common.rs:5-45`](../../../.source_1765521767/object-0.37.3/src/common.rs#L5-L45)*

A CPU architecture.

#### Implementations

- <span id="architecture-address-size"></span>`fn address_size(self) -> Option<AddressSize>` — [`AddressSize`](../index.md#addresssize)

  The size of an address value for this architecture.

  

  Returns `None` for unknown architectures.

#### Trait Implementations

##### `impl Any for Architecture`

- <span id="architecture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Architecture`

- <span id="architecture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Architecture`

- <span id="architecture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Architecture`

- <span id="architecture-clone"></span>`fn clone(&self) -> Architecture` — [`Architecture`](../index.md#architecture)

##### `impl CloneToUninit for Architecture`

- <span id="architecture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Architecture`

##### `impl Debug for Architecture`

- <span id="architecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Architecture`

##### `impl<T> From for Architecture`

- <span id="architecture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Architecture`

- <span id="architecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Architecture`

- <span id="architecture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Architecture`

- <span id="architecture-partialeq-eq"></span>`fn eq(&self, other: &Architecture) -> bool` — [`Architecture`](../index.md#architecture)

##### `impl StructuralPartialEq for Architecture`

##### `impl ToOwned for Architecture`

- <span id="architecture-toowned-type-owned"></span>`type Owned = T`

- <span id="architecture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="architecture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Architecture`

- <span id="architecture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="architecture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Architecture`

- <span id="architecture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="architecture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SubArchitecture`

```rust
enum SubArchitecture {
    Arm64E,
    Arm64EC,
}
```

*Defined in [`object-0.37.3/src/common.rs:51-54`](../../../.source_1765521767/object-0.37.3/src/common.rs#L51-L54)*

A CPU sub-architecture.

#### Trait Implementations

##### `impl Any for SubArchitecture`

- <span id="subarchitecture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubArchitecture`

- <span id="subarchitecture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubArchitecture`

- <span id="subarchitecture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SubArchitecture`

- <span id="subarchitecture-clone"></span>`fn clone(&self) -> SubArchitecture` — [`SubArchitecture`](../index.md#subarchitecture)

##### `impl CloneToUninit for SubArchitecture`

- <span id="subarchitecture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SubArchitecture`

##### `impl Debug for SubArchitecture`

- <span id="subarchitecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubArchitecture`

##### `impl<T> From for SubArchitecture`

- <span id="subarchitecture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SubArchitecture`

- <span id="subarchitecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SubArchitecture`

- <span id="subarchitecture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SubArchitecture`

- <span id="subarchitecture-partialeq-eq"></span>`fn eq(&self, other: &SubArchitecture) -> bool` — [`SubArchitecture`](../index.md#subarchitecture)

##### `impl StructuralPartialEq for SubArchitecture`

##### `impl ToOwned for SubArchitecture`

- <span id="subarchitecture-toowned-type-owned"></span>`type Owned = T`

- <span id="subarchitecture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="subarchitecture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SubArchitecture`

- <span id="subarchitecture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="subarchitecture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SubArchitecture`

- <span id="subarchitecture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="subarchitecture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddressSize`

```rust
enum AddressSize {
    U8,
    U16,
    U32,
    U64,
}
```

*Defined in [`object-0.37.3/src/common.rs:109-114`](../../../.source_1765521767/object-0.37.3/src/common.rs#L109-L114)*

The size of an address value for an architecture.

This may differ from the address size supported by the file format (such as for COFF).

#### Implementations

- <span id="addresssize-bytes"></span>`fn bytes(self) -> u8`

  The size in bytes of an address value.

#### Trait Implementations

##### `impl Any for AddressSize`

- <span id="addresssize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddressSize`

- <span id="addresssize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddressSize`

- <span id="addresssize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AddressSize`

- <span id="addresssize-clone"></span>`fn clone(&self) -> AddressSize` — [`AddressSize`](../index.md#addresssize)

##### `impl CloneToUninit for AddressSize`

- <span id="addresssize-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AddressSize`

##### `impl Debug for AddressSize`

- <span id="addresssize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddressSize`

##### `impl<T> From for AddressSize`

- <span id="addresssize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AddressSize`

- <span id="addresssize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for AddressSize`

- <span id="addresssize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AddressSize`

- <span id="addresssize-partialeq-eq"></span>`fn eq(&self, other: &AddressSize) -> bool` — [`AddressSize`](../index.md#addresssize)

##### `impl StructuralPartialEq for AddressSize`

##### `impl ToOwned for AddressSize`

- <span id="addresssize-toowned-type-owned"></span>`type Owned = T`

- <span id="addresssize-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addresssize-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddressSize`

- <span id="addresssize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addresssize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddressSize`

- <span id="addresssize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addresssize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BinaryFormat`

```rust
enum BinaryFormat {
    Coff,
    Elf,
    MachO,
    Pe,
    Wasm,
    Xcoff,
}
```

*Defined in [`object-0.37.3/src/common.rs:128-135`](../../../.source_1765521767/object-0.37.3/src/common.rs#L128-L135)*

A binary file format.

#### Implementations

- <span id="binaryformat-native-object"></span>`fn native_object() -> BinaryFormat` — [`BinaryFormat`](../index.md#binaryformat)

  The target's native binary format for relocatable object files.

  

  Defaults to `Elf` for unknown platforms.

#### Trait Implementations

##### `impl Any for BinaryFormat`

- <span id="binaryformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BinaryFormat`

- <span id="binaryformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BinaryFormat`

- <span id="binaryformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BinaryFormat`

- <span id="binaryformat-clone"></span>`fn clone(&self) -> BinaryFormat` — [`BinaryFormat`](../index.md#binaryformat)

##### `impl CloneToUninit for BinaryFormat`

- <span id="binaryformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BinaryFormat`

##### `impl Debug for BinaryFormat`

- <span id="binaryformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BinaryFormat`

##### `impl<T> From for BinaryFormat`

- <span id="binaryformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for BinaryFormat`

- <span id="binaryformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for BinaryFormat`

- <span id="binaryformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BinaryFormat`

- <span id="binaryformat-partialeq-eq"></span>`fn eq(&self, other: &BinaryFormat) -> bool` — [`BinaryFormat`](../index.md#binaryformat)

##### `impl StructuralPartialEq for BinaryFormat`

##### `impl ToOwned for BinaryFormat`

- <span id="binaryformat-toowned-type-owned"></span>`type Owned = T`

- <span id="binaryformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="binaryformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BinaryFormat`

- <span id="binaryformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="binaryformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BinaryFormat`

- <span id="binaryformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="binaryformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionKind`

```rust
enum SectionKind {
    Unknown,
    Text,
    Data,
    ReadOnlyData,
    ReadOnlyDataWithRel,
    ReadOnlyString,
    UninitializedData,
    Common,
    Tls,
    UninitializedTls,
    TlsVariables,
    OtherString,
    Other,
    Debug,
    DebugString,
    Linker,
    Note,
    Metadata,
    Elf(u32),
}
```

*Defined in [`object-0.37.3/src/common.rs:155-247`](../../../.source_1765521767/object-0.37.3/src/common.rs#L155-L247)*

The kind of a section.

#### Variants

- **`Unknown`**

  The section kind is unknown.

- **`Text`**

  An executable code section.
  
  Example ELF sections: `.text`
  
  Example Mach-O sections: `__TEXT/__text`

- **`Data`**

  A data section.
  
  Example ELF sections: `.data`
  
  Example Mach-O sections: `__DATA/__data`

- **`ReadOnlyData`**

  A read only data section.
  
  Example ELF sections: `.rodata`
  
  Example Mach-O sections: `__TEXT/__const`, `__DATA/__const`, `__TEXT/__literal4`

- **`ReadOnlyDataWithRel`**

  A read only data section with relocations.
  
  This is the same as either `Data` or `ReadOnlyData`, depending on the file format.
  This value is only used in the API for writing files. It is never returned when reading files.

- **`ReadOnlyString`**

  A loadable string section.
  
  Example ELF sections: `.rodata.str`
  
  Example Mach-O sections: `__TEXT/__cstring`

- **`UninitializedData`**

  An uninitialized data section.
  
  Example ELF sections: `.bss`
  
  Example Mach-O sections: `__DATA/__bss`

- **`Common`**

  An uninitialized common data section.
  
  Example Mach-O sections: `__DATA/__common`

- **`Tls`**

  A TLS data section.
  
  Example ELF sections: `.tdata`
  
  Example Mach-O sections: `__DATA/__thread_data`

- **`UninitializedTls`**

  An uninitialized TLS data section.
  
  Example ELF sections: `.tbss`
  
  Example Mach-O sections: `__DATA/__thread_bss`

- **`TlsVariables`**

  A TLS variables section.
  
  This contains TLS variable structures, rather than the variable initializers.
  
  Example Mach-O sections: `__DATA/__thread_vars`

- **`OtherString`**

  A non-loadable string section.
  
  Example ELF sections: `.comment`, `.debug_str`

- **`Other`**

  Some other non-loadable section.
  
  Example ELF sections: `.debug_info`

- **`Debug`**

  Debug information.
  
  Example Mach-O sections: `__DWARF/__debug_info`

- **`DebugString`**

  Debug strings.
  
  This is the same as either `Debug` or `OtherString`, depending on the file format.
  This value is only used in the API for writing files. It is never returned when reading files.

- **`Linker`**

  Information for the linker.
  
  Example COFF sections: `.drectve`

- **`Note`**

  ELF note section.

- **`Metadata`**

  Metadata such as symbols or relocations.
  
  Example ELF sections: `.symtab`, `.strtab`, `.group`

- **`Elf`**

  Some other ELF section type.
  
  This is the `sh_type` field in the section header.
  The meaning may be dependent on the architecture.

#### Implementations

- <span id="sectionkind-is-bss"></span>`fn is_bss(self) -> bool`

  Return true if this section contains zerofill data.

#### Trait Implementations

##### `impl Any for SectionKind`

- <span id="sectionkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionKind`

- <span id="sectionkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionKind`

- <span id="sectionkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionKind`

- <span id="sectionkind-clone"></span>`fn clone(&self) -> SectionKind` — [`SectionKind`](../index.md#sectionkind)

##### `impl CloneToUninit for SectionKind`

- <span id="sectionkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- <span id="sectionkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionKind`

##### `impl<T> From for SectionKind`

- <span id="sectionkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SectionKind`

- <span id="sectionkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SectionKind`

- <span id="sectionkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SectionKind`

- <span id="sectionkind-partialeq-eq"></span>`fn eq(&self, other: &SectionKind) -> bool` — [`SectionKind`](../index.md#sectionkind)

##### `impl StructuralPartialEq for SectionKind`

##### `impl ToOwned for SectionKind`

- <span id="sectionkind-toowned-type-owned"></span>`type Owned = T`

- <span id="sectionkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectionkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionKind`

- <span id="sectionkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionKind`

- <span id="sectionkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatKind`

```rust
enum ComdatKind {
    Unknown,
    Any,
    NoDuplicates,
    SameSize,
    ExactMatch,
    Largest,
    Newest,
}
```

*Defined in [`object-0.37.3/src/common.rs:264-291`](../../../.source_1765521767/object-0.37.3/src/common.rs#L264-L291)*

The selection kind for a COMDAT section group.

This determines the way in which the linker resolves multiple definitions of the COMDAT
sections.

#### Variants

- **`Unknown`**

  The selection kind is unknown.

- **`Any`**

  Multiple definitions are allowed.
  
  An arbitrary definition is selected, and the rest are removed.
  
  This is the only supported selection kind for ELF.

- **`NoDuplicates`**

  Multiple definitions are not allowed.
  
  This is used to group sections without allowing duplicates.

- **`SameSize`**

  Multiple definitions must have the same size.
  
  An arbitrary definition is selected, and the rest are removed.

- **`ExactMatch`**

  Multiple definitions must match exactly.
  
  An arbitrary definition is selected, and the rest are removed.

- **`Largest`**

  Multiple definitions are allowed, and the largest is selected.
  
  An arbitrary definition with the largest size is selected, and the rest are removed.

- **`Newest`**

  Multiple definitions are allowed, and the newest is selected.

#### Trait Implementations

##### `impl Any for ComdatKind`

- <span id="comdatkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatKind`

- <span id="comdatkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatKind`

- <span id="comdatkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ComdatKind`

- <span id="comdatkind-clone"></span>`fn clone(&self) -> ComdatKind` — [`ComdatKind`](../index.md#comdatkind)

##### `impl CloneToUninit for ComdatKind`

- <span id="comdatkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ComdatKind`

##### `impl Debug for ComdatKind`

- <span id="comdatkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatKind`

##### `impl<T> From for ComdatKind`

- <span id="comdatkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ComdatKind`

- <span id="comdatkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ComdatKind`

- <span id="comdatkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ComdatKind`

- <span id="comdatkind-partialeq-eq"></span>`fn eq(&self, other: &ComdatKind) -> bool` — [`ComdatKind`](../index.md#comdatkind)

##### `impl StructuralPartialEq for ComdatKind`

##### `impl ToOwned for ComdatKind`

- <span id="comdatkind-toowned-type-owned"></span>`type Owned = T`

- <span id="comdatkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="comdatkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ComdatKind`

- <span id="comdatkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatKind`

- <span id="comdatkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolKind`

```rust
enum SymbolKind {
    Unknown,
    Text,
    Data,
    Section,
    File,
    Label,
    Tls,
}
```

*Defined in [`object-0.37.3/src/common.rs:296-311`](../../../.source_1765521767/object-0.37.3/src/common.rs#L296-L311)*

The kind of a symbol.

#### Variants

- **`Unknown`**

  The symbol kind is unknown.

- **`Text`**

  The symbol is for executable code.

- **`Data`**

  The symbol is for a data object.

- **`Section`**

  The symbol is for a section.

- **`File`**

  The symbol is the name of a file. It precedes symbols within that file.

- **`Label`**

  The symbol is for a code label.

- **`Tls`**

  The symbol is for a thread local storage entity.

#### Trait Implementations

##### `impl Any for SymbolKind`

- <span id="symbolkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolKind`

- <span id="symbolkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolKind`

- <span id="symbolkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolKind`

- <span id="symbolkind-clone"></span>`fn clone(&self) -> SymbolKind` — [`SymbolKind`](../index.md#symbolkind)

##### `impl CloneToUninit for SymbolKind`

- <span id="symbolkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SymbolKind`

##### `impl Debug for SymbolKind`

- <span id="symbolkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolKind`

##### `impl<T> From for SymbolKind`

- <span id="symbolkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SymbolKind`

- <span id="symbolkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolKind`

- <span id="symbolkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SymbolKind`

- <span id="symbolkind-partialeq-eq"></span>`fn eq(&self, other: &SymbolKind) -> bool` — [`SymbolKind`](../index.md#symbolkind)

##### `impl StructuralPartialEq for SymbolKind`

##### `impl ToOwned for SymbolKind`

- <span id="symbolkind-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolKind`

- <span id="symbolkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolKind`

- <span id="symbolkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolScope`

```rust
enum SymbolScope {
    Unknown,
    Compilation,
    Linkage,
    Dynamic,
}
```

*Defined in [`object-0.37.3/src/common.rs:315-324`](../../../.source_1765521767/object-0.37.3/src/common.rs#L315-L324)*

A symbol scope.

#### Variants

- **`Unknown`**

  Unknown scope.

- **`Compilation`**

  Symbol is visible to the compilation unit.

- **`Linkage`**

  Symbol is visible to the static linkage unit.

- **`Dynamic`**

  Symbol is visible to dynamically linked objects.

#### Trait Implementations

##### `impl Any for SymbolScope`

- <span id="symbolscope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolScope`

- <span id="symbolscope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolScope`

- <span id="symbolscope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolScope`

- <span id="symbolscope-clone"></span>`fn clone(&self) -> SymbolScope` — [`SymbolScope`](../index.md#symbolscope)

##### `impl CloneToUninit for SymbolScope`

- <span id="symbolscope-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SymbolScope`

##### `impl Debug for SymbolScope`

- <span id="symbolscope-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolScope`

##### `impl<T> From for SymbolScope`

- <span id="symbolscope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SymbolScope`

- <span id="symbolscope-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolScope`

- <span id="symbolscope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SymbolScope`

- <span id="symbolscope-partialeq-eq"></span>`fn eq(&self, other: &SymbolScope) -> bool` — [`SymbolScope`](../index.md#symbolscope)

##### `impl StructuralPartialEq for SymbolScope`

##### `impl ToOwned for SymbolScope`

- <span id="symbolscope-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolscope-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolscope-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolScope`

- <span id="symbolscope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolscope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolScope`

- <span id="symbolscope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolscope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationKind`

```rust
enum RelocationKind {
    Unknown,
    Absolute,
    Relative,
    Got,
    GotRelative,
    GotBaseRelative,
    GotBaseOffset,
    PltRelative,
    ImageOffset,
    SectionOffset,
    SectionIndex,
}
```

*Defined in [`object-0.37.3/src/common.rs:343-366`](../../../.source_1765521767/object-0.37.3/src/common.rs#L343-L366)*

The operation used to calculate the result of the relocation.

The relocation descriptions use the following definitions. Note that
these definitions probably don't match any ELF ABI.

* A - The value of the addend.
* G - The address of the symbol's entry within the global offset table.
* L - The address of the symbol's entry within the procedure linkage table.
* P - The address of the place of the relocation.
* S - The address of the symbol.
* GotBase - The address of the global offset table.
* Image - The base address of the image.
* Section - The address of the section containing the symbol.

'XxxRelative' means 'Xxx + A - P'.  'XxxOffset' means 'S + A - Xxx'.

#### Variants

- **`Unknown`**

  The operation is unknown.

- **`Absolute`**

  S + A

- **`Relative`**

  S + A - P

- **`Got`**

  G + A - GotBase

- **`GotRelative`**

  G + A - P

- **`GotBaseRelative`**

  GotBase + A - P

- **`GotBaseOffset`**

  S + A - GotBase

- **`PltRelative`**

  L + A - P

- **`ImageOffset`**

  S + A - Image

- **`SectionOffset`**

  S + A - Section

- **`SectionIndex`**

  The index of the section containing the symbol.

#### Trait Implementations

##### `impl Any for RelocationKind`

- <span id="relocationkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationKind`

- <span id="relocationkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationKind`

- <span id="relocationkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationKind`

- <span id="relocationkind-clone"></span>`fn clone(&self) -> RelocationKind` — [`RelocationKind`](../index.md#relocationkind)

##### `impl CloneToUninit for RelocationKind`

- <span id="relocationkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationKind`

##### `impl Debug for RelocationKind`

- <span id="relocationkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationKind`

##### `impl<T> From for RelocationKind`

- <span id="relocationkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationKind`

- <span id="relocationkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationKind`

- <span id="relocationkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationKind`

- <span id="relocationkind-partialeq-eq"></span>`fn eq(&self, other: &RelocationKind) -> bool` — [`RelocationKind`](../index.md#relocationkind)

##### `impl StructuralPartialEq for RelocationKind`

##### `impl ToOwned for RelocationKind`

- <span id="relocationkind-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationKind`

- <span id="relocationkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationKind`

- <span id="relocationkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationEncoding`

```rust
enum RelocationEncoding {
    Unknown,
    Generic,
    X86Signed,
    X86RipRelative,
    X86RipRelativeMovq,
    X86Branch,
    S390xDbl,
    AArch64Call,
    LoongArchBranch,
    SharcTypeA,
    SharcTypeB,
    E2KLit,
    E2KDisp,
}
```

*Defined in [`object-0.37.3/src/common.rs:374-447`](../../../.source_1765521767/object-0.37.3/src/common.rs#L374-L447)*

Information about how the result of the relocation operation is encoded in the place.

This is usually architecture specific, such as specifying an addressing mode or
a specific instruction.

#### Variants

- **`Unknown`**

  The relocation encoding is unknown.

- **`Generic`**

  Generic encoding.

- **`X86Signed`**

  x86 sign extension at runtime.
  
  Used with `RelocationKind::Absolute`.

- **`X86RipRelative`**

  x86 rip-relative addressing.
  
  The `RelocationKind` must be PC relative.

- **`X86RipRelativeMovq`**

  x86 rip-relative addressing in movq instruction.
  
  The `RelocationKind` must be PC relative.

- **`X86Branch`**

  x86 branch instruction.
  
  The `RelocationKind` must be PC relative.

- **`S390xDbl`**

  s390x PC-relative offset shifted right by one bit.
  
  The `RelocationKind` must be PC relative.

- **`AArch64Call`**

  AArch64 call target.
  
  The `RelocationKind` must be PC relative.

- **`LoongArchBranch`**

  LoongArch branch offset with two trailing zeros.
  
  The `RelocationKind` must be PC relative.

- **`SharcTypeA`**

  SHARC+ 48-bit Type A instruction
  
  Represents these possible variants, each with a corresponding
  `R_SHARC_*` constant:
  
  * 24-bit absolute address
  * 32-bit absolute address
  * 6-bit relative address
  * 24-bit relative address
  * 6-bit absolute address in the immediate value field
  * 16-bit absolute address in the immediate value field

- **`SharcTypeB`**

  SHARC+ 32-bit Type B instruction
  
  Represents these possible variants, each with a corresponding
  `R_SHARC_*` constant:
  
  * 6-bit absolute address in the immediate value field
  * 7-bit absolute address in the immediate value field
  * 16-bit absolute address
  * 6-bit relative address

- **`E2KLit`**

  E2K 64-bit value stored in two LTS
  
  Memory representation:
  ```text
  0: LTS1 = value[63:32]
  4: LTS0 = value[31:0]
  ```

- **`E2KDisp`**

  E2K 28-bit value stored in CS0

#### Trait Implementations

##### `impl Any for RelocationEncoding`

- <span id="relocationencoding-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationEncoding`

- <span id="relocationencoding-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationEncoding`

- <span id="relocationencoding-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationEncoding`

- <span id="relocationencoding-clone"></span>`fn clone(&self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md#relocationencoding)

##### `impl CloneToUninit for RelocationEncoding`

- <span id="relocationencoding-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationEncoding`

##### `impl Debug for RelocationEncoding`

- <span id="relocationencoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationEncoding`

##### `impl<T> From for RelocationEncoding`

- <span id="relocationencoding-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationEncoding`

- <span id="relocationencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationEncoding`

- <span id="relocationencoding-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationEncoding`

- <span id="relocationencoding-partialeq-eq"></span>`fn eq(&self, other: &RelocationEncoding) -> bool` — [`RelocationEncoding`](../index.md#relocationencoding)

##### `impl StructuralPartialEq for RelocationEncoding`

##### `impl ToOwned for RelocationEncoding`

- <span id="relocationencoding-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationencoding-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationencoding-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationEncoding`

- <span id="relocationencoding-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationencoding-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationEncoding`

- <span id="relocationencoding-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationencoding-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FileFlags`

```rust
enum FileFlags {
    None,
    Elf {
        os_abi: u8,
        abi_version: u8,
        e_flags: u32,
    },
    MachO {
        flags: u32,
    },
    Coff {
        characteristics: u16,
    },
    Xcoff {
        f_flags: u16,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:452-479`](../../../.source_1765521767/object-0.37.3/src/common.rs#L452-L479)*

File flags that are specific to each file format.

#### Variants

- **`None`**

  No file flags.

- **`Elf`**

  ELF file flags.

- **`MachO`**

  Mach-O file flags.

- **`Coff`**

  COFF file flags.

- **`Xcoff`**

  XCOFF file flags.

#### Trait Implementations

##### `impl Any for FileFlags`

- <span id="fileflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FileFlags`

- <span id="fileflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FileFlags`

- <span id="fileflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FileFlags`

- <span id="fileflags-clone"></span>`fn clone(&self) -> FileFlags` — [`FileFlags`](../index.md#fileflags)

##### `impl CloneToUninit for FileFlags`

- <span id="fileflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FileFlags`

##### `impl Debug for FileFlags`

- <span id="fileflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileFlags`

##### `impl<T> From for FileFlags`

- <span id="fileflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FileFlags`

- <span id="fileflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FileFlags`

- <span id="fileflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FileFlags`

- <span id="fileflags-partialeq-eq"></span>`fn eq(&self, other: &FileFlags) -> bool` — [`FileFlags`](../index.md#fileflags)

##### `impl StructuralPartialEq for FileFlags`

##### `impl ToOwned for FileFlags`

- <span id="fileflags-toowned-type-owned"></span>`type Owned = T`

- <span id="fileflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fileflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FileFlags`

- <span id="fileflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fileflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FileFlags`

- <span id="fileflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fileflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentFlags`

```rust
enum SegmentFlags {
    None,
    Elf {
        p_flags: u32,
    },
    MachO {
        flags: u32,
        maxprot: u32,
        initprot: u32,
    },
    Coff {
        characteristics: u32,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:484-506`](../../../.source_1765521767/object-0.37.3/src/common.rs#L484-L506)*

Segment flags that are specific to each file format.

#### Variants

- **`None`**

  No segment flags.

- **`Elf`**

  ELF segment flags.

- **`MachO`**

  Mach-O segment flags.

- **`Coff`**

  COFF segment flags.

#### Trait Implementations

##### `impl Any for SegmentFlags`

- <span id="segmentflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentFlags`

- <span id="segmentflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentFlags`

- <span id="segmentflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SegmentFlags`

- <span id="segmentflags-clone"></span>`fn clone(&self) -> SegmentFlags` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl CloneToUninit for SegmentFlags`

- <span id="segmentflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- <span id="segmentflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SegmentFlags`

##### `impl<T> From for SegmentFlags`

- <span id="segmentflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SegmentFlags`

- <span id="segmentflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SegmentFlags`

- <span id="segmentflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SegmentFlags`

- <span id="segmentflags-partialeq-eq"></span>`fn eq(&self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl StructuralPartialEq for SegmentFlags`

##### `impl ToOwned for SegmentFlags`

- <span id="segmentflags-toowned-type-owned"></span>`type Owned = T`

- <span id="segmentflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="segmentflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SegmentFlags`

- <span id="segmentflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentFlags`

- <span id="segmentflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionFlags`

```rust
enum SectionFlags {
    None,
    Elf {
        sh_flags: u64,
    },
    MachO {
        flags: u32,
    },
    Coff {
        characteristics: u32,
    },
    Xcoff {
        s_flags: u32,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:511-534`](../../../.source_1765521767/object-0.37.3/src/common.rs#L511-L534)*

Section flags that are specific to each file format.

#### Variants

- **`None`**

  No section flags.

- **`Elf`**

  ELF section flags.

- **`MachO`**

  Mach-O section flags.

- **`Coff`**

  COFF section flags.

- **`Xcoff`**

  XCOFF section flags.

#### Trait Implementations

##### `impl Any for SectionFlags`

- <span id="sectionflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionFlags`

- <span id="sectionflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionFlags`

- <span id="sectionflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionFlags`

- <span id="sectionflags-clone"></span>`fn clone(&self) -> SectionFlags` — [`SectionFlags`](../index.md#sectionflags)

##### `impl CloneToUninit for SectionFlags`

- <span id="sectionflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionFlags`

##### `impl Debug for SectionFlags`

- <span id="sectionflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionFlags`

##### `impl<T> From for SectionFlags`

- <span id="sectionflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SectionFlags`

- <span id="sectionflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SectionFlags`

- <span id="sectionflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SectionFlags`

- <span id="sectionflags-partialeq-eq"></span>`fn eq(&self, other: &SectionFlags) -> bool` — [`SectionFlags`](../index.md#sectionflags)

##### `impl StructuralPartialEq for SectionFlags`

##### `impl ToOwned for SectionFlags`

- <span id="sectionflags-toowned-type-owned"></span>`type Owned = T`

- <span id="sectionflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectionflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionFlags`

- <span id="sectionflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionFlags`

- <span id="sectionflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolFlags<Section, Symbol>`

```rust
enum SymbolFlags<Section, Symbol> {
    None,
    Elf {
        st_info: u8,
        st_other: u8,
    },
    MachO {
        n_desc: u16,
    },
    CoffSection {
        selection: u8,
        associative_section: Option<Section>,
    },
    Xcoff {
        n_sclass: u8,
        x_smtyp: u8,
        x_smclas: u8,
        containing_csect: Option<Symbol>,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:539-578`](../../../.source_1765521767/object-0.37.3/src/common.rs#L539-L578)*

Symbol flags that are specific to each file format.

#### Variants

- **`None`**

  No symbol flags.

- **`Elf`**

  ELF symbol flags.

- **`MachO`**

  Mach-O symbol flags.

- **`CoffSection`**

  COFF flags for a section symbol.

- **`Xcoff`**

  XCOFF symbol flags.

#### Trait Implementations

##### `impl Any for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Section: clone::Clone, Symbol: clone::Clone> Clone for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-clone"></span>`fn clone(&self) -> SymbolFlags<Section, Symbol>` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl CloneToUninit for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Section: marker::Copy, Symbol: marker::Copy> Copy for SymbolFlags<Section, Symbol>`

##### `impl<Section: fmt::Debug, Symbol: fmt::Debug> Debug for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section: cmp::Eq, Symbol: cmp::Eq> Eq for SymbolFlags<Section, Symbol>`

##### `impl<T> From for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<Section: hash::Hash, Symbol: hash::Hash> Hash for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Section: cmp::PartialEq, Symbol: cmp::PartialEq> PartialEq for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-partialeq-eq"></span>`fn eq(&self, other: &SymbolFlags<Section, Symbol>) -> bool` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl<Section, Symbol> StructuralPartialEq for SymbolFlags<Section, Symbol>`

##### `impl ToOwned for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationFlags`

```rust
enum RelocationFlags {
    Generic {
        kind: RelocationKind,
        encoding: RelocationEncoding,
        size: u8,
    },
    Elf {
        r_type: u32,
    },
    MachO {
        r_type: u8,
        r_pcrel: bool,
        r_length: u8,
    },
    Coff {
        typ: u16,
    },
    Xcoff {
        r_rtype: u8,
        r_rsize: u8,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:583-619`](../../../.source_1765521767/object-0.37.3/src/common.rs#L583-L619)*

Relocation fields that are specific to each file format and architecture.

#### Variants

- **`Generic`**

  Format independent representation.

- **`Elf`**

  ELF relocation fields.

- **`MachO`**

  Mach-O relocation fields.

- **`Coff`**

  COFF relocation fields.

- **`Xcoff`**

  XCOFF relocation fields.

#### Trait Implementations

##### `impl Any for RelocationFlags`

- <span id="relocationflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationFlags`

- <span id="relocationflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationFlags`

- <span id="relocationflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationFlags`

- <span id="relocationflags-clone"></span>`fn clone(&self) -> RelocationFlags` — [`RelocationFlags`](../index.md#relocationflags)

##### `impl CloneToUninit for RelocationFlags`

- <span id="relocationflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationFlags`

##### `impl Debug for RelocationFlags`

- <span id="relocationflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationFlags`

##### `impl<T> From for RelocationFlags`

- <span id="relocationflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationFlags`

- <span id="relocationflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationFlags`

- <span id="relocationflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationFlags`

- <span id="relocationflags-partialeq-eq"></span>`fn eq(&self, other: &RelocationFlags) -> bool` — [`RelocationFlags`](../index.md#relocationflags)

##### `impl StructuralPartialEq for RelocationFlags`

##### `impl ToOwned for RelocationFlags`

- <span id="relocationflags-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationFlags`

- <span id="relocationflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationFlags`

- <span id="relocationflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `File<'data, R: ReadRef<'data>>`

```rust
enum File<'data, R: ReadRef<'data>> {
    Coff(coff::CoffFile<'data, R>),
    CoffBig(coff::CoffBigFile<'data, R>),
    Elf32(elf::ElfFile32<'data, crate::endian::Endianness, R>),
    Elf64(elf::ElfFile64<'data, crate::endian::Endianness, R>),
    MachO32(macho::MachOFile32<'data, crate::endian::Endianness, R>),
    MachO64(macho::MachOFile64<'data, crate::endian::Endianness, R>),
    Pe32(pe::PeFile32<'data, R>),
    Pe64(pe::PeFile64<'data, R>),
    Xcoff32(xcoff::XcoffFile32<'data, R>),
    Xcoff64(xcoff::XcoffFile64<'data, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:213-236`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L213-L236)*

An object file that can be any supported file format.

Most functionality is provided by the [`Object`](#object) trait implementation.

#### Implementations

- <span id="file-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../index.md#result)

  Parse the raw file data.

- <span id="file-parse-dyld-cache-image"></span>`fn parse_dyld_cache_image<'cache, E: crate::Endian>(image: &macho::DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](macho/index.md#dyldcacheimage), [`Result`](../index.md#result)

  Parse a Mach-O image from the dyld shared cache.

- <span id="file-format"></span>`fn format(&self) -> BinaryFormat` — [`BinaryFormat`](../index.md#binaryformat)

  Return the file format.

#### Trait Implementations

##### `impl Any for File<'data, R>`

- <span id="file-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for File<'data, R>`

- <span id="file-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for File<'data, R>`

- <span id="file-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for File<'data, R>`

- <span id="file-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for File<'data, R>`

- <span id="file-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for File<'data, R>`

- <span id="file-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Object for File<'data, R>`

- <span id="file-object-type-segment"></span>`type Segment = Segment<'data, 'file, R>`

- <span id="file-object-type-segmentiterator"></span>`type SegmentIterator = SegmentIterator<'data, 'file, R>`

- <span id="file-object-type-section"></span>`type Section = Section<'data, 'file, R>`

- <span id="file-object-type-sectioniterator"></span>`type SectionIterator = SectionIterator<'data, 'file, R>`

- <span id="file-object-type-comdat"></span>`type Comdat = Comdat<'data, 'file, R>`

- <span id="file-object-type-comdatiterator"></span>`type ComdatIterator = ComdatIterator<'data, 'file, R>`

- <span id="file-object-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="file-object-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="file-object-type-symboltable"></span>`type SymbolTable = SymbolTable<'data, 'file, R>`

- <span id="file-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = DynamicRelocationIterator<'data, 'file, R>`

- <span id="file-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../index.md#architecture)

- <span id="file-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../index.md#subarchitecture)

- <span id="file-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="file-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="file-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../index.md#objectkind)

- <span id="file-object-segments"></span>`fn segments(&self) -> SegmentIterator<'data, '_, R>` — [`SegmentIterator`](#segmentiterator)

- <span id="file-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<Section<'data, 'file, R>>` — [`Section`](#section)

- <span id="file-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<Section<'data, '_, R>>` — [`SectionIndex`](../index.md#sectionindex), [`Result`](../index.md#result), [`Section`](#section)

- <span id="file-object-sections"></span>`fn sections(&self) -> SectionIterator<'data, '_, R>` — [`SectionIterator`](#sectioniterator)

- <span id="file-object-comdats"></span>`fn comdats(&self) -> ComdatIterator<'data, '_, R>` — [`ComdatIterator`](#comdatiterator)

- <span id="file-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<Symbol<'data, '_, R>>` — [`SymbolIndex`](../index.md#symbolindex), [`Result`](../index.md#result), [`Symbol`](#symbol)

- <span id="file-object-symbols"></span>`fn symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](#symboliterator)

- <span id="file-object-symbol-table"></span>`fn symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](#symboltable)

- <span id="file-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](#symboliterator)

- <span id="file-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](#symboltable)

- <span id="file-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<DynamicRelocationIterator<'data, '_, R>>` — [`DynamicRelocationIterator`](#dynamicrelocationiterator)

- <span id="file-object-symbol-map"></span>`fn symbol_map(&self) -> SymbolMap<SymbolMapName<'data>>` — [`SymbolMap`](../index.md#symbolmap), [`SymbolMapName`](../index.md#symbolmapname)

- <span id="file-object-object-map"></span>`fn object_map(&self) -> ObjectMap<'data>` — [`ObjectMap`](../index.md#objectmap)

- <span id="file-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../index.md#result), [`Import`](../index.md#import)

- <span id="file-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../index.md#result), [`Export`](../index.md#export)

- <span id="file-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="file-object-mach-uuid"></span>`fn mach_uuid(&self) -> Result<Option<[u8; 16]>>` — [`Result`](../index.md#result)

- <span id="file-object-build-id"></span>`fn build_id(&self) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md#result)

- <span id="file-object-gnu-debuglink"></span>`fn gnu_debuglink(&self) -> Result<Option<(&'data [u8], u32)>>` — [`Result`](../index.md#result)

- <span id="file-object-gnu-debugaltlink"></span>`fn gnu_debugaltlink(&self) -> Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../index.md#result)

- <span id="file-object-pdb-info"></span>`fn pdb_info(&self) -> Result<Option<CodeView<'_>>>` — [`Result`](../index.md#result), [`CodeView`](../index.md#codeview)

- <span id="file-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="file-object-entry"></span>`fn entry(&self) -> u64`

- <span id="file-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../index.md#fileflags)

##### `impl<R: ReadRef<'data>> Sealed for File<'data, R>`

##### `impl<U> TryFrom for File<'data, R>`

- <span id="file-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for File<'data, R>`

- <span id="file-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegmentIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSegmentIterator<'data, 'file, R>),
    Elf32(elf::ElfSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegmentIterator32<'data, 'file, R>),
    Pe64(pe::PeSegmentIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegmentIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegmentIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:537-560`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L537-L560)*

#### Trait Implementations

##### `impl Any for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegment<'data, 'file, R>),
    CoffBig(coff::CoffBigSegment<'data, 'file, R>),
    Elf32(elf::ElfSegment32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegment64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegment32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegment64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegment32<'data, 'file, R>),
    Pe64(pe::PeSegment64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegment32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegment64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:579-602`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L579-L602)*

#### Trait Implementations

##### `impl Any for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeSectionIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSectionIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:671-694`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L671-L694)*

#### Trait Implementations

##### `impl Any for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectioniteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectioniteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSection<'data, 'file, R>),
    CoffBig(coff::CoffBigSection<'data, 'file, R>),
    Elf32(elf::ElfSection32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSection64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSection32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSection64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSection32<'data, 'file, R>),
    Pe64(pe::PeSection64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSection32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSection64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:712-735`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L712-L735)*

#### Trait Implementations

##### `impl Any for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectioninternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectioninternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:848-871`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L848-L871)*

#### Trait Implementations

##### `impl Any for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdat<'data, 'file, R>),
    CoffBig(coff::CoffBigComdat<'data, 'file, R>),
    Elf32(elf::ElfComdat32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdat64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdat32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdat64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdat32<'data, 'file, R>),
    Pe64(pe::PeComdat64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdat32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdat64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:889-912`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L889-L912)*

#### Trait Implementations

##### `impl Any for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatSectionIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatSectionIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:964-987`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L964-L987)*

#### Trait Implementations

##### `impl Any for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatsectioniteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatsectioniteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolTableInternal<'data, 'file, R>`

```rust
enum SymbolTableInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolTable32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolTable64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1009-1055`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1009-L1055)*

#### Trait Implementations

##### `impl Any for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltableinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltableinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolIteratorInternal<'data, 'file, R>`

```rust
enum SymbolIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolIterator32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolIterator64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1093-1149`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1093-L1149)*

#### Trait Implementations

##### `impl Any for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboliteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboliteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolInternal<'data, 'file, R>`

```rust
enum SymbolInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbol32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbol64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1172-1218`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1172-L1218)*

#### Trait Implementations

##### `impl Any for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DynamicRelocationIteratorInternal<'data, 'file, R>`

```rust
enum DynamicRelocationIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Elf32(elf::ElfDynamicRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfDynamicRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    None(core::marker::PhantomData<(&'data (), &'file (), R)>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1309-1320`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1309-L1320)*

#### Trait Implementations

##### `impl Any for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dynamicrelocationiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dynamicrelocationiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffRelocationIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigRelocationIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachORelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachORelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeRelocationIterator<'data, 'file, R>),
    Pe64(pe::PeRelocationIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffRelocationIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffRelocationIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1343-1366`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1343-L1366)*

#### Trait Implementations

##### `impl Any for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionrelocationiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionrelocationiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ReadError<T>`

```rust
trait ReadError<T> { ... }
```

*Defined in [`object-0.37.3/src/read/mod.rs:133-135`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L133-L135)*

#### Required Methods

- `fn read_error(self, error: &'static str) -> Result<T>`

#### Implementors

- `Option<T>`
- `result::Result<T, ()>`
- `result::Result<T, Error>`

### `SymbolMapEntry`

```rust
trait SymbolMapEntry { ... }
```

*Defined in [`object-0.37.3/src/read/mod.rs:440-443`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L440-L443)*

An entry in a [`SymbolMap`](../index.md).

#### Required Methods

- `fn address(&self) -> u64`

  The symbol address.

#### Implementors

- [`ObjectMapEntry`](../index.md#objectmapentry)
- [`SymbolMapName`](../index.md#symbolmapname)

### `ReadRef<'a>`

```rust
trait ReadRef<'a>: Clone + Copy { ... }
```

*Defined in [`object-0.37.3/src/read/read_ref.rs:49-124`](../../../.source_1765521767/object-0.37.3/src/read/read_ref.rs#L49-L124)*

A trait for reading references to [`Pod`](../index.md) types from a block of data.

This allows parsers to handle both of these cases:
- the block of data exists in memory, and it is desirable
  to use references to this block instead of copying it,
- the block of data exists in storage, and it is desirable
  to read on demand to minimize I/O and memory usage.

A block of data typically exists in memory as a result of using a memory
mapped file, and the crate was written with this use case in mind.
Reading the entire file into a `Vec` is also possible, but it often uses
more I/O and memory.
Both of these are handled by the `ReadRef` implementation for `&[u8]`.

For the second use case, the `ReadRef` trait is implemented for
[`&ReadCache`](super::ReadCache). This is useful for environments where
memory mapped files are not available or not suitable, such as WebAssembly.
This differs from reading into a `Vec` in that it only reads the portions
of the file that are needed for parsing.

The methods accept `self` by value because `Self` is expected to behave
similar to a reference: it may be a reference with a lifetime of `'a`,
or it may be a wrapper of a reference.

The `Clone` and `Copy` bounds are for convenience, and since `Self` is
expected to be similar to a reference, these are easily satisfied.

Object file parsers typically use offsets to locate the structures
in the block, and will most commonly use the `*_at` methods to
read a structure at a known offset.

Occasionally file parsers will need to treat the block as a stream,
and so convenience methods are provided that update an offset with
the size that was read.

#### Required Methods

- `fn len(self) -> result::Result<u64, ()>`

  The total size of the block of data.

- `fn read_bytes_at(self, offset: u64, size: u64) -> result::Result<&'a [u8], ()>`

  Get a reference to a `u8` slice at the given offset.

- `fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> result::Result<&'a [u8], ()>`

  Get a reference to a delimited `u8` slice which starts at range.start.

#### Provided Methods

- `fn read_bytes(self, offset: &mut u64, size: u64) -> result::Result<&'a [u8], ()>`

  Get a reference to a `u8` slice at the given offset, and update the offset.

- `fn read<T: Pod>(self, offset: &mut u64) -> result::Result<&'a T, ()>`

  Get a reference to a `Pod` type at the given offset, and update the offset.

- `fn read_at<T: Pod>(self, offset: u64) -> result::Result<&'a T, ()>`

  Get a reference to a `Pod` type at the given offset.

- `fn read_slice<T: Pod>(self, offset: &mut u64, count: usize) -> result::Result<&'a [T], ()>`

  Get a reference to a slice of a `Pod` type at the given offset, and update the offset.

- `fn read_slice_at<T: Pod>(self, offset: u64, count: usize) -> result::Result<&'a [T], ()>`

  Get a reference to a slice of a `Pod` type at the given offset.

#### Implementors

- [`ReadCacheRange`](#readcacherange)
- `&'a ReadCache<R>`
- `&'a [u8]`

### `ReadCacheOps`

```rust
trait ReadCacheOps { ... }
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:222-242`](../../../.source_1765521767/object-0.37.3/src/read/read_cache.rs#L222-L242)*

Operations required to implement [`ReadCache`](#readcache).

This is a subset of the `Read` and `Seek` traits.
A blanket implementation is provided for all types that implement
`Read + Seek`.

#### Required Methods

- `fn len(&mut self) -> Result<u64, ()>`

  Return the length of the stream.

- `fn seek(&mut self, pos: u64) -> Result<u64, ()>`

  Seek to the given position in the stream.

- `fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>`

  Read up to `buf.len()` bytes into `buf`.

- `fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ()>`

  Read exactly `buf.len()` bytes into `buf`.

### `Object<'data>`

```rust
trait Object<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:15-335`](../../../.source_1765521767/object-0.37.3/src/read/traits.rs#L15-L335)*

An object file.

This is the primary trait for the unified read API.

#### Associated Types

- `type Segment: 1`

- `type SegmentIterator: 1`

- `type Section: 1`

- `type SectionIterator: 1`

- `type Comdat: 1`

- `type ComdatIterator: 1`

- `type Symbol: 1`

- `type SymbolIterator: 1`

- `type SymbolTable: 1`

- `type DynamicRelocationIterator: 1`

#### Required Methods

- `fn architecture(&self) -> Architecture`

  Get the architecture type of the file.

- `fn is_little_endian(&self) -> bool`

  Return true if the file is little endian, false if it is big endian.

- `fn is_64(&self) -> bool`

  Return true if the file can contain 64-bit addresses.

- `fn kind(&self) -> ObjectKind`

  Return the kind of this object.

- `fn segments(&self) -> <Self as >::SegmentIterator`

  Get an iterator for the loadable segments in the file.

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<<Self as >::Section>`

  Like `Self::section_by_name`, but allows names that are not UTF-8.

- `fn section_by_index(&self, index: SectionIndex) -> Result<<Self as >::Section>`

  Get the section at the given index.

- `fn sections(&self) -> <Self as >::SectionIterator`

  Get an iterator for the sections in the file.

- `fn comdats(&self) -> <Self as >::ComdatIterator`

  Get an iterator for the COMDAT section groups in the file.

- `fn symbol_table(&self) -> Option<<Self as >::SymbolTable>`

  Get the debugging symbol table, if any.

- `fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the debugging symbol at the given index.

- `fn symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the debugging symbols in the file.

- `fn dynamic_symbol_table(&self) -> Option<<Self as >::SymbolTable>`

  Get the dynamic linking symbol table, if any.

- `fn dynamic_symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the dynamic linking symbols in the file.

- `fn dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>`

  Get the dynamic relocations for this file.

- `fn imports(&self) -> Result<Vec<Import<'data>>>`

  Get the imported symbols.

- `fn exports(&self) -> Result<Vec<Export<'data>>>`

  Get the exported symbols that expose both a name and an address.

- `fn has_debug_symbols(&self) -> bool`

  Return true if the file contains DWARF debug information sections, false if not.

- `fn relative_address_base(&self) -> u64`

  Get the base address used for relative virtual addresses.

- `fn entry(&self) -> u64`

  Get the virtual address of the entry point of the binary.

- `fn flags(&self) -> FileFlags`

  File flags that are specific to each file format.

#### Provided Methods

- `fn sub_architecture(&self) -> Option<SubArchitecture>`

  Get the sub-architecture type of the file if known.

- `fn endianness(&self) -> Endianness`

  Get the endianness of the file.

- `fn section_by_name(&self, section_name: &str) -> Option<<Self as >::Section>`

  Get the section named `section_name`, if such a section exists.

- `fn symbol_by_name<'file>(self: &'file Self, symbol_name: &str) -> Option<<Self as >::Symbol>`

  Get the symbol named `symbol_name`, if the symbol exists.

- `fn symbol_by_name_bytes<'file>(self: &'file Self, symbol_name: &[u8]) -> Option<<Self as >::Symbol>`

  Like `Self::symbol_by_name`, but allows names that are not UTF-8.

- `fn symbol_map(&self) -> SymbolMap<SymbolMapName<'data>>`

  Construct a map from addresses to symbol names.

- `fn object_map(&self) -> ObjectMap<'data>`

  Construct a map from addresses to symbol names and object file names.

- `fn mach_uuid(&self) -> Result<Option<[u8; 16]>>`

  The UUID from a Mach-O [`LC_UUID`](crate::macho::LC_UUID) load command.

- `fn build_id(&self) -> Result<Option<&'data [u8]>>`

  The build ID from an ELF [`NT_GNU_BUILD_ID`](crate::elf::NT_GNU_BUILD_ID) note.

- `fn gnu_debuglink(&self) -> Result<Option<(&'data [u8], u32)>>`

  The filename and CRC from a `.gnu_debuglink` section.

- `fn gnu_debugaltlink(&self) -> Result<Option<(&'data [u8], &'data [u8])>>`

  The filename and build ID from a `.gnu_debugaltlink` section.

- `fn pdb_info(&self) -> Result<Option<CodeView<'_>>>`

  The filename and GUID from the PE CodeView section.

#### Implementors

- [`CoffFile`](coff/index.md#cofffile)
- [`ElfFile`](elf/index.md#elffile)
- [`File`](#file)
- [`MachOFile`](macho/index.md#machofile)
- [`PeFile`](pe/index.md#pefile)
- [`XcoffFile`](xcoff/index.md#xcofffile)

### `ObjectSegment<'data>`

```rust
trait ObjectSegment<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:340-374`](../../../.source_1765521767/object-0.37.3/src/read/traits.rs#L340-L374)*

A loadable segment in an [`Object`](#object).

This trait is part of the unified read API.

#### Required Methods

- `fn address(&self) -> u64`

  Returns the virtual address of the segment.

- `fn size(&self) -> u64`

  Returns the size of the segment in memory.

- `fn align(&self) -> u64`

  Returns the alignment of the segment in memory.

- `fn file_range(&self) -> (u64, u64)`

  Returns the offset and size of the segment in the file.

- `fn data(&self) -> Result<&'data [u8]>`

  Returns a reference to the file contents of the segment.

- `fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the segment data in the given range.

- `fn name_bytes(&self) -> Result<Option<&[u8]>>`

  Returns the name of the segment.

- `fn name(&self) -> Result<Option<&str>>`

  Returns the name of the segment.

- `fn flags(&self) -> SegmentFlags`

  Return the flags of segment.

#### Implementors

- [`CoffSegment`](coff/index.md#coffsegment)
- [`ElfSegment`](elf/index.md#elfsegment)
- [`MachOSegment`](macho/index.md#machosegment)
- [`PeSegment`](pe/index.md#pesegment)
- [`Segment`](#segment)
- [`XcoffSegment`](xcoff/index.md#xcoffsegment)

### `ObjectSection<'data>`

```rust
trait ObjectSection<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:379-462`](../../../.source_1765521767/object-0.37.3/src/read/traits.rs#L379-L462)*

A section in an [`Object`](#object).

This trait is part of the unified read API.

#### Associated Types

- `type RelocationIterator: 1`

#### Required Methods

- `fn index(&self) -> SectionIndex`

  Returns the section index.

- `fn address(&self) -> u64`

  Returns the address of the section.

- `fn size(&self) -> u64`

  Returns the size of the section in memory.

- `fn align(&self) -> u64`

  Returns the alignment of the section in memory.

- `fn file_range(&self) -> Option<(u64, u64)>`

  Returns offset and size of on-disk segment (if any).

- `fn data(&self) -> Result<&'data [u8]>`

  Returns the raw contents of the section.

- `fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the raw contents of the section data in the given range.

- `fn compressed_file_range(&self) -> Result<CompressedFileRange>`

  Returns the potentially compressed file range of the section,

- `fn compressed_data(&self) -> Result<CompressedData<'data>>`

  Returns the potentially compressed contents of the section,

- `fn name_bytes(&self) -> Result<&'data [u8]>`

  Returns the name of the section.

- `fn name(&self) -> Result<&'data str>`

  Returns the name of the section.

- `fn segment_name_bytes(&self) -> Result<Option<&[u8]>>`

  Returns the name of the segment for this section.

- `fn segment_name(&self) -> Result<Option<&str>>`

  Returns the name of the segment for this section.

- `fn kind(&self) -> SectionKind`

  Return the kind of this section.

- `fn relocations(&self) -> <Self as >::RelocationIterator`

  Get the relocations for this section.

- `fn relocation_map(&self) -> Result<RelocationMap>`

  Construct a relocation map for this section.

- `fn flags(&self) -> SectionFlags`

  Section flags that are specific to each file format.

#### Provided Methods

- `fn uncompressed_data(&self) -> Result<Cow<'data, [u8]>>`

  Returns the uncompressed contents of the section.

#### Implementors

- [`CoffSection`](coff/index.md#coffsection)
- [`ElfSection`](elf/index.md#elfsection)
- [`MachOSection`](macho/index.md#machosection)
- [`PeSection`](pe/index.md#pesection)
- [`Section`](#section)
- [`XcoffSection`](xcoff/index.md#xcoffsection)

### `ObjectComdat<'data>`

```rust
trait ObjectComdat<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:467-487`](../../../.source_1765521767/object-0.37.3/src/read/traits.rs#L467-L487)*

A COMDAT section group in an [`Object`](#object).

This trait is part of the unified read API.

#### Associated Types

- `type SectionIterator: 1`

#### Required Methods

- `fn kind(&self) -> ComdatKind`

  Returns the COMDAT selection kind.

- `fn symbol(&self) -> SymbolIndex`

  Returns the index of the symbol used for the name of COMDAT section group.

- `fn name_bytes(&self) -> Result<&'data [u8]>`

  Returns the name of the COMDAT section group.

- `fn name(&self) -> Result<&'data str>`

  Returns the name of the COMDAT section group.

- `fn sections(&self) -> <Self as >::SectionIterator`

  Get the sections in this section group.

#### Implementors

- [`CoffComdat`](coff/index.md#coffcomdat)
- [`Comdat`](#comdat)
- [`ElfComdat`](elf/index.md#elfcomdat)
- [`MachOComdat`](macho/index.md#machocomdat)
- [`PeComdat`](pe/index.md#pecomdat)
- [`XcoffComdat`](xcoff/index.md#xcoffcomdat)

### `ObjectSymbolTable<'data>`

```rust
trait ObjectSymbolTable<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:492-510`](../../../.source_1765521767/object-0.37.3/src/read/traits.rs#L492-L510)*

A symbol table in an [`Object`](#object).

This trait is part of the unified read API.

#### Associated Types

- `type Symbol: 1`

- `type SymbolIterator: 1`

#### Required Methods

- `fn symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the symbols in the table.

- `fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the symbol at the given index.

#### Implementors

- [`CoffSymbolTable`](coff/index.md#coffsymboltable)
- [`ElfSymbolTable`](elf/index.md#elfsymboltable)
- [`MachOSymbolTable`](macho/index.md#machosymboltable)
- [`SymbolTable`](#symboltable)
- [`XcoffSymbolTable`](xcoff/index.md#xcoffsymboltable)

### `ObjectSymbol<'data>`

```rust
trait ObjectSymbol<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:515-576`](../../../.source_1765521767/object-0.37.3/src/read/traits.rs#L515-L576)*

A symbol table entry in an [`Object`](#object).

This trait is part of the unified read API.

#### Required Methods

- `fn index(&self) -> SymbolIndex`

  The index of the symbol.

- `fn name_bytes(&self) -> Result<&'data [u8]>`

  The name of the symbol.

- `fn name(&self) -> Result<&'data str>`

  The name of the symbol.

- `fn address(&self) -> u64`

  The address of the symbol. May be zero if the address is unknown.

- `fn size(&self) -> u64`

  The size of the symbol. May be zero if the size is unknown.

- `fn kind(&self) -> SymbolKind`

  Return the kind of this symbol.

- `fn section(&self) -> SymbolSection`

  Returns the section where the symbol is defined.

- `fn is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- `fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object

- `fn is_common(&self) -> bool`

  Return true if the symbol is common data.

- `fn is_weak(&self) -> bool`

  Return true if the symbol is weak.

- `fn scope(&self) -> SymbolScope`

  Returns the symbol scope.

- `fn is_global(&self) -> bool`

  Return true if the symbol visible outside of the compilation unit.

- `fn is_local(&self) -> bool`

  Return true if the symbol is only visible within the compilation unit.

- `fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>`

  Symbol flags that are specific to each file format.

#### Provided Methods

- `fn section_index(&self) -> Option<SectionIndex>`

  Returns the section index for the section containing this symbol.

#### Implementors

- [`CoffSymbol`](coff/index.md#coffsymbol)
- [`ElfSymbol`](elf/index.md#elfsymbol)
- [`MachOSymbol`](macho/index.md#machosymbol)
- [`Symbol`](#symbol)
- [`XcoffSymbol`](xcoff/index.md#xcoffsymbol)

## Functions

### `debug_list_bytes`

```rust
fn debug_list_bytes(bytes: &[u8], fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`object-0.37.3/src/read/util.rs:213-220`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L213-L220)*

### `align`

```rust
fn align(offset: usize, size: usize) -> usize
```

*Defined in [`object-0.37.3/src/read/util.rs:254-256`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L254-L256)*

### `data_range`

```rust
fn data_range(data: &[u8], data_address: u64, range_address: u64, size: u64) -> Option<&[u8]>
```

*Defined in [`object-0.37.3/src/read/util.rs:259-268`](../../../.source_1765521767/object-0.37.3/src/read/util.rs#L259-L268)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

*Defined in [`object-0.37.3/src/read/mod.rs:131`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L131)*

The result type used within the read module.

### `NativeFile<'data, R>`

```rust
type NativeFile<'data, R> = elf::ElfFile64<'data, crate::endian::Endianness, R>;
```

*Defined in [`object-0.37.3/src/read/mod.rs:171`](../../../.source_1765521767/object-0.37.3/src/read/mod.rs#L171)*

The native executable file for the target platform.

### `Result<T>`

```rust
type Result<T> = result::Result<T, ()>;
```

*Defined in [`object-0.37.3/src/read/read_ref.rs:9`](../../../.source_1765521767/object-0.37.3/src/read/read_ref.rs#L9)*

## Macros

### `with_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:30-57`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L30-L57)*

Evaluate an expression on the contents of a file format enum.

This is a hack to avoid virtual calls.

### `with_inner_mut!`

*Defined in [`object-0.37.3/src/read/any.rs:59-86`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L59-L86)*

### `map_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:89-116`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L89-L116)*

Like `with_inner!`, but wraps the result in another enum.

### `map_inner_option!`

*Defined in [`object-0.37.3/src/read/any.rs:119-146`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L119-L146)*

Like `map_inner!`, but the result is a Result or Option.

### `map_inner_option_mut!`

*Defined in [`object-0.37.3/src/read/any.rs:148-175`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L148-L175)*

### `next_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:178-205`](../../../.source_1765521767/object-0.37.3/src/read/any.rs#L178-L205)*

Call `next` for a file format iterator.

