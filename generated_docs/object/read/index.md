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
  - [`read_ref`](#read_ref)
  - [`read_cache`](#read_cache)
  - [`util`](#util)
  - [`gnu_compression`](#gnu_compression)
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
  - [`debug_list_bytes`](#debug_list_bytes)
  - [`align`](#align)
  - [`data_range`](#data_range)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
  - [`NativeFile`](#nativefile)
  - [`Result`](#result)
- [Macros](#macros)
  - [`with_inner!`](#with_inner)
  - [`with_inner_mut!`](#with_inner_mut)
  - [`map_inner!`](#map_inner)
  - [`map_inner_option!`](#map_inner_option)
  - [`map_inner_option_mut!`](#map_inner_option_mut)
  - [`next_inner!`](#next_inner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`read_ref`](#read_ref) | mod |  |
| [`read_cache`](#read_cache) | mod |  |
| [`util`](#util) | mod |  |
| [`gnu_compression`](#gnu_compression) | mod |  |
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
| [`debug_list_bytes`](#debug_list_bytes) | fn |  |
| [`align`](#align) | fn |  |
| [`data_range`](#data_range) | fn |  |
| [`Result`](#result) | type | The result type used within the read module. |
| [`NativeFile`](#nativefile) | type | The native executable file for the target platform. |
| [`Result`](#result) | type |  |
| [`with_inner!`](#with_inner) | macro | Evaluate an expression on the contents of a file format enum. |
| [`with_inner_mut!`](#with_inner_mut) | macro |  |
| [`map_inner!`](#map_inner) | macro | Like `with_inner!`, but wraps the result in another enum. |
| [`map_inner_option!`](#map_inner_option) | macro | Like `map_inner!`, but the result is a Result or Option. |
| [`map_inner_option_mut!`](#map_inner_option_mut) | macro |  |
| [`next_inner!`](#next_inner) | macro | Call `next` for a file format iterator. |

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

*Defined in [`object-0.37.3/src/read/mod.rs:116`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L116)*

The error type used within the read module.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](../index.md)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](../index.md)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `SectionIndex`

```rust
struct SectionIndex(usize);
```

*Defined in [`object-0.37.3/src/read/mod.rs:389`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L389)*

The index used to identify a section in a file.

#### Trait Implementations

##### `impl Clone for SectionIndex`

- <span id="sectionindex-clone"></span>`fn clone(&self) -> SectionIndex` — [`SectionIndex`](../index.md)

##### `impl Copy for SectionIndex`

##### `impl Debug for SectionIndex`

- <span id="sectionindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SectionIndex`

- <span id="sectionindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionIndex`

##### `impl Hash for SectionIndex`

- <span id="sectionindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionIndex`

- <span id="sectionindex-eq"></span>`fn eq(&self, other: &SectionIndex) -> bool` — [`SectionIndex`](../index.md)

##### `impl StructuralPartialEq for SectionIndex`

##### `impl ToString for SectionIndex`

- <span id="sectionindex-to-string"></span>`fn to_string(&self) -> String`

### `SymbolIndex`

```rust
struct SymbolIndex(usize);
```

*Defined in [`object-0.37.3/src/read/mod.rs:399`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L399)*

The index used to identify a symbol in a symbol table.

#### Trait Implementations

##### `impl Clone for SymbolIndex`

- <span id="symbolindex-clone"></span>`fn clone(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

##### `impl Copy for SymbolIndex`

##### `impl Debug for SymbolIndex`

- <span id="symbolindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SymbolIndex`

- <span id="symbolindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolIndex`

##### `impl Hash for SymbolIndex`

- <span id="symbolindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolIndex`

- <span id="symbolindex-eq"></span>`fn eq(&self, other: &SymbolIndex) -> bool` — [`SymbolIndex`](../index.md)

##### `impl StructuralPartialEq for SymbolIndex`

##### `impl ToString for SymbolIndex`

- <span id="symbolindex-to-string"></span>`fn to_string(&self) -> String`

### `SymbolMap<T: SymbolMapEntry>`

```rust
struct SymbolMap<T: SymbolMapEntry> {
    symbols: alloc::vec::Vec<T>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:451-453`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L451-L453)*

A map from addresses to symbol information.

The symbol information depends on the chosen entry type, such as [`SymbolMapName`](../index.md).

Returned by `Object::symbol_map`.

#### Implementations

- <span id="symbolmap-new"></span>`fn new(symbols: Vec<T>) -> Self`

- <span id="symbolmap-get"></span>`fn get(&self, address: u64) -> Option<&T>`

- <span id="symbolmap-symbols"></span>`fn symbols(&self) -> &[T]`

#### Trait Implementations

##### `impl<T: clone::Clone + SymbolMapEntry> Clone for SymbolMap<T>`

- <span id="symbolmap-clone"></span>`fn clone(&self) -> SymbolMap<T>` — [`SymbolMap`](../index.md)

##### `impl<T: fmt::Debug + SymbolMapEntry> Debug for SymbolMap<T>`

- <span id="symbolmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default + SymbolMapEntry> Default for SymbolMap<T>`

- <span id="symbolmap-default"></span>`fn default() -> SymbolMap<T>` — [`SymbolMap`](../index.md)

### `SymbolMapName<'data>`

```rust
struct SymbolMapName<'data> {
    address: u64,
    name: &'data str,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:485-488`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L485-L488)*

The type used for entries in a [`SymbolMap`](../index.md) that maps from addresses to names.

#### Implementations

- <span id="symbolmapname-new"></span>`fn new(address: u64, name: &'data str) -> Self`

- <span id="symbolmapname-address"></span>`fn address(&self) -> u64`

- <span id="symbolmapname-name"></span>`fn name(&self) -> &'data str`

#### Trait Implementations

##### `impl Clone for SymbolMapName<'data>`

- <span id="symbolmapname-clone"></span>`fn clone(&self) -> SymbolMapName<'data>` — [`SymbolMapName`](../index.md)

##### `impl Copy for SymbolMapName<'data>`

##### `impl Debug for SymbolMapName<'data>`

- <span id="symbolmapname-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolMapName<'data>`

##### `impl Hash for SymbolMapName<'data>`

- <span id="symbolmapname-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolMapName<'data>`

- <span id="symbolmapname-eq"></span>`fn eq(&self, other: &SymbolMapName<'data>) -> bool` — [`SymbolMapName`](../index.md)

##### `impl StructuralPartialEq for SymbolMapName<'data>`

##### `impl SymbolMapEntry for SymbolMapName<'data>`

- <span id="symbolmapname-address"></span>`fn address(&self) -> u64`

### `ObjectMap<'data>`

```rust
struct ObjectMap<'data> {
    symbols: SymbolMap<ObjectMapEntry<'data>>,
    objects: alloc::vec::Vec<ObjectMapFile<'data>>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:522-525`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L522-L525)*

A map from addresses to symbol names and object files.

This is derived from STAB entries in Mach-O files.

Returned by `Object::object_map`.

#### Implementations

- <span id="objectmap-get"></span>`fn get(&self, address: u64) -> Option<&ObjectMapEntry<'data>>` — [`ObjectMapEntry`](../index.md)

- <span id="objectmap-symbols"></span>`fn symbols(&self) -> &[ObjectMapEntry<'data>]` — [`ObjectMapEntry`](../index.md)

- <span id="objectmap-objects"></span>`fn objects(&self) -> &[ObjectMapFile<'data>]` — [`ObjectMapFile`](../index.md)

#### Trait Implementations

##### `impl Clone for ObjectMap<'data>`

- <span id="objectmap-clone"></span>`fn clone(&self) -> ObjectMap<'data>` — [`ObjectMap`](../index.md)

##### `impl Debug for ObjectMap<'data>`

- <span id="objectmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ObjectMap<'data>`

- <span id="objectmap-default"></span>`fn default() -> ObjectMap<'data>` — [`ObjectMap`](../index.md)

### `ObjectMapEntry<'data>`

```rust
struct ObjectMapEntry<'data> {
    address: u64,
    size: u64,
    name: &'data [u8],
    object: usize,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:550-555`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L550-L555)*

A symbol in an [`ObjectMap`](../index.md).

#### Implementations

- <span id="objectmapentry-address"></span>`fn address(&self) -> u64`

- <span id="objectmapentry-size"></span>`fn size(&self) -> u64`

- <span id="objectmapentry-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="objectmapentry-object-index"></span>`fn object_index(&self) -> usize`

- <span id="objectmapentry-object"></span>`fn object<'a>(&self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>` — [`ObjectMap`](../index.md), [`ObjectMapFile`](../index.md)

#### Trait Implementations

##### `impl Clone for ObjectMapEntry<'data>`

- <span id="objectmapentry-clone"></span>`fn clone(&self) -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../index.md)

##### `impl Copy for ObjectMapEntry<'data>`

##### `impl Debug for ObjectMapEntry<'data>`

- <span id="objectmapentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ObjectMapEntry<'data>`

- <span id="objectmapentry-default"></span>`fn default() -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../index.md)

##### `impl Eq for ObjectMapEntry<'data>`

##### `impl Hash for ObjectMapEntry<'data>`

- <span id="objectmapentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ObjectMapEntry<'data>`

- <span id="objectmapentry-eq"></span>`fn eq(&self, other: &ObjectMapEntry<'data>) -> bool` — [`ObjectMapEntry`](../index.md)

##### `impl StructuralPartialEq for ObjectMapEntry<'data>`

##### `impl SymbolMapEntry for ObjectMapEntry<'data>`

- <span id="objectmapentry-address"></span>`fn address(&self) -> u64`

### `ObjectMapFile<'data>`

```rust
struct ObjectMapFile<'data> {
    path: &'data [u8],
    member: Option<&'data [u8]>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:600-603`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L600-L603)*

An object file name in an [`ObjectMap`](../index.md).

#### Implementations

- <span id="objectmapfile-new"></span>`fn new(path: &'data [u8], member: Option<&'data [u8]>) -> Self`

- <span id="objectmapfile-path"></span>`fn path(&self) -> &'data [u8]`

- <span id="objectmapfile-member"></span>`fn member(&self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl Clone for ObjectMapFile<'data>`

- <span id="objectmapfile-clone"></span>`fn clone(&self) -> ObjectMapFile<'data>` — [`ObjectMapFile`](../index.md)

##### `impl Copy for ObjectMapFile<'data>`

##### `impl Debug for ObjectMapFile<'data>`

- <span id="objectmapfile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectMapFile<'data>`

##### `impl Hash for ObjectMapFile<'data>`

- <span id="objectmapfile-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ObjectMapFile<'data>`

- <span id="objectmapfile-eq"></span>`fn eq(&self, other: &ObjectMapFile<'data>) -> bool` — [`ObjectMapFile`](../index.md)

##### `impl StructuralPartialEq for ObjectMapFile<'data>`

### `Import<'data>`

```rust
struct Import<'data> {
    library: ByteString<'data>,
    name: ByteString<'data>,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:628-632`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L628-L632)*

An imported symbol.

Returned by `Object::imports`.

#### Implementations

- <span id="import-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="import-library"></span>`fn library(&self) -> &'data [u8]`

#### Trait Implementations

##### `impl Clone for Import<'data>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'data>` — [`Import`](../index.md)

##### `impl Copy for Import<'data>`

##### `impl Debug for Import<'data>`

- <span id="import-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Import<'data>`

##### `impl PartialEq for Import<'data>`

- <span id="import-eq"></span>`fn eq(&self, other: &Import<'data>) -> bool` — [`Import`](../index.md)

##### `impl StructuralPartialEq for Import<'data>`

### `Export<'data>`

```rust
struct Export<'data> {
    name: ByteString<'data>,
    address: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:652-656`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L652-L656)*

An exported symbol.

Returned by `Object::exports`.

#### Implementations

- <span id="export-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="export-address"></span>`fn address(&self) -> u64`

#### Trait Implementations

##### `impl Clone for Export<'data>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'data>` — [`Export`](../index.md)

##### `impl Copy for Export<'data>`

##### `impl Debug for Export<'data>`

- <span id="export-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Export<'data>`

##### `impl PartialEq for Export<'data>`

- <span id="export-eq"></span>`fn eq(&self, other: &Export<'data>) -> bool` — [`Export`](../index.md)

##### `impl StructuralPartialEq for Export<'data>`

### `CodeView<'data>`

```rust
struct CodeView<'data> {
    guid: [u8; 16],
    path: ByteString<'data>,
    age: u32,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:674-678`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L674-L678)*

PDB information from the debug directory in a PE file.

#### Implementations

- <span id="codeview-path"></span>`fn path(&self) -> &'data [u8]`

- <span id="codeview-age"></span>`fn age(&self) -> u32`

- <span id="codeview-guid"></span>`fn guid(&self) -> [u8; 16]`

#### Trait Implementations

##### `impl Clone for CodeView<'data>`

- <span id="codeview-clone"></span>`fn clone(&self) -> CodeView<'data>` — [`CodeView`](../index.md)

##### `impl Copy for CodeView<'data>`

##### `impl Debug for CodeView<'data>`

- <span id="codeview-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CodeView<'data>`

##### `impl PartialEq for CodeView<'data>`

- <span id="codeview-eq"></span>`fn eq(&self, other: &CodeView<'data>) -> bool` — [`CodeView`](../index.md)

##### `impl StructuralPartialEq for CodeView<'data>`

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

*Defined in [`object-0.37.3/src/read/mod.rs:716-724`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L716-L724)*

A relocation entry.

Returned by `Object::dynamic_relocations` or `ObjectSection::relocations`.

#### Implementations

- <span id="relocation-kind"></span>`fn kind(&self) -> RelocationKind` — [`RelocationKind`](../index.md)

- <span id="relocation-encoding"></span>`fn encoding(&self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md)

- <span id="relocation-size"></span>`fn size(&self) -> u8`

- <span id="relocation-target"></span>`fn target(&self) -> RelocationTarget` — [`RelocationTarget`](../index.md)

- <span id="relocation-addend"></span>`fn addend(&self) -> i64`

- <span id="relocation-set-addend"></span>`fn set_addend(&mut self, addend: i64)`

- <span id="relocation-has-implicit-addend"></span>`fn has_implicit_addend(&self) -> bool`

- <span id="relocation-flags"></span>`fn flags(&self) -> RelocationFlags` — [`RelocationFlags`](../index.md)

#### Trait Implementations

##### `impl Debug for Relocation`

- <span id="relocation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RelocationMap`

```rust
struct RelocationMap(alloc::collections::btree_map::BTreeMap<u64, RelocationMapEntry>);
```

*Defined in [`object-0.37.3/src/read/mod.rs:790`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L790)*

A map from section offsets to relocation information.

This can be used to apply relocations to a value at a given section offset.
This is intended for use with DWARF in relocatable object files, and only
supports relocations that are used in DWARF.

Returned by `ObjectSection::relocation_map`.

#### Implementations

- <span id="relocationmap-new"></span>`fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>` — [`Object`](#object), [`Result`](../index.md)

- <span id="relocationmap-add"></span>`fn add<'data: 'file, 'file, T>(&mut self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>` — [`Relocation`](../index.md), [`Result`](../index.md)

- <span id="relocationmap-relocate"></span>`fn relocate(&self, offset: u64, value: u64) -> u64`

#### Trait Implementations

##### `impl Debug for RelocationMap`

- <span id="relocationmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationMap`

- <span id="relocationmap-default"></span>`fn default() -> RelocationMap` — [`RelocationMap`](../index.md)

### `RelocationMapEntry`

```rust
struct RelocationMapEntry {
    implicit_addend: bool,
    addend: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:871-874`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L871-L874)*

#### Trait Implementations

##### `impl Clone for RelocationMapEntry`

- <span id="relocationmapentry-clone"></span>`fn clone(&self) -> RelocationMapEntry` — [`RelocationMapEntry`](#relocationmapentry)

##### `impl Copy for RelocationMapEntry`

##### `impl Debug for RelocationMapEntry`

- <span id="relocationmapentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationMapEntry`

##### `impl Hash for RelocationMapEntry`

- <span id="relocationmapentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationMapEntry`

- <span id="relocationmapentry-eq"></span>`fn eq(&self, other: &RelocationMapEntry) -> bool` — [`RelocationMapEntry`](#relocationmapentry)

##### `impl StructuralPartialEq for RelocationMapEntry`

### `CompressedFileRange`

```rust
struct CompressedFileRange {
    pub format: CompressionFormat,
    pub offset: u64,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:898-907`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L898-L907)*

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

- <span id="compressedfilerange-data"></span>`fn data<'data, R: ReadRef<'data>>(self, file: R) -> Result<CompressedData<'data>>` — [`Result`](../index.md), [`CompressedData`](../index.md)

#### Trait Implementations

##### `impl Clone for CompressedFileRange`

- <span id="compressedfilerange-clone"></span>`fn clone(&self) -> CompressedFileRange` — [`CompressedFileRange`](../index.md)

##### `impl Copy for CompressedFileRange`

##### `impl Debug for CompressedFileRange`

- <span id="compressedfilerange-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressedFileRange`

##### `impl Hash for CompressedFileRange`

- <span id="compressedfilerange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressedFileRange`

- <span id="compressedfilerange-eq"></span>`fn eq(&self, other: &CompressedFileRange) -> bool` — [`CompressedFileRange`](../index.md)

##### `impl StructuralPartialEq for CompressedFileRange`

### `CompressedData<'data>`

```rust
struct CompressedData<'data> {
    pub format: CompressionFormat,
    pub data: &'data [u8],
    pub uncompressed_size: u64,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:947-954`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L947-L954)*

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

- <span id="compresseddata-decompress"></span>`fn decompress(self) -> Result<Cow<'data, [u8]>>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for CompressedData<'data>`

- <span id="compresseddata-clone"></span>`fn clone(&self) -> CompressedData<'data>` — [`CompressedData`](../index.md)

##### `impl Copy for CompressedData<'data>`

##### `impl Debug for CompressedData<'data>`

- <span id="compresseddata-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressedData<'data>`

##### `impl Hash for CompressedData<'data>`

- <span id="compresseddata-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressedData<'data>`

- <span id="compresseddata-eq"></span>`fn eq(&self, other: &CompressedData<'data>) -> bool` — [`CompressedData`](../index.md)

##### `impl StructuralPartialEq for CompressedData<'data>`

### `ReadCache<R: ReadCacheOps>`

```rust
struct ReadCache<R: ReadCacheOps> {
    cache: core::cell::RefCell<ReadCacheInternal<R>>,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:31-33`](../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L31-L33)*

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

- <span id="readcache-range"></span>`fn range(&self, offset: u64, size: u64) -> ReadCacheRange<'_, R>` — [`ReadCacheRange`](#readcacherange)

- <span id="readcache-clear"></span>`fn clear(&mut self)`

- <span id="readcache-into-inner"></span>`fn into_inner(self) -> R`

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCache<R>`

- <span id="readcache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, R: ReadCacheOps> ReadRef for &'a ReadCache<R>`

- <span id="a-readcache-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="a-readcache-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="a-readcache-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

### `ReadCacheInternal<R: ReadCacheOps>`

```rust
struct ReadCacheInternal<R: ReadCacheOps> {
    read: R,
    bufs: alloc::collections::btree_map::BTreeMap<(u64, u64), alloc::boxed::Box<[u8]>>,
    strings: alloc::collections::btree_map::BTreeMap<(u64, u8), alloc::boxed::Box<[u8]>>,
    len: Option<u64>,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:36-41`](../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L36-L41)*

#### Implementations

- <span id="readcacheinternal-range-in-bounds"></span>`fn range_in_bounds(&mut self, range: &Range<u64>) -> Result<(), ()>`

- <span id="readcacheinternal-len"></span>`fn len(&mut self) -> Result<u64, ()>`

#### Trait Implementations

##### `impl<R: fmt::Debug + ReadCacheOps> Debug for ReadCacheInternal<R>`

- <span id="readcacheinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReadCacheRange<'a, R: ReadCacheOps>`

```rust
struct ReadCacheRange<'a, R: ReadCacheOps> {
    r: &'a ReadCache<R>,
    offset: u64,
    size: u64,
}
```

*Defined in [`object-0.37.3/src/read/read_cache.rs:172-176`](../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L172-L176)*

An implementation of [`ReadRef`](#readref) for a range of data in a stream that
implements `Read + Seek`.

Shares an underlying [`ReadCache`](#readcache) with a lifetime of `'a`.

#### Trait Implementations

##### `impl<'a, R: ReadCacheOps> Clone for ReadCacheRange<'a, R>`

- <span id="readcacherange-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, R: ReadCacheOps> Copy for ReadCacheRange<'a, R>`

##### `impl<'a, R: fmt::Debug + ReadCacheOps> Debug for ReadCacheRange<'a, R>`

- <span id="readcacherange-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, R: ReadCacheOps> ReadRef for ReadCacheRange<'a, R>`

- <span id="readcacherange-len"></span>`fn len(self) -> Result<u64, ()>`

- <span id="readcacherange-read-bytes-at"></span>`fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- <span id="readcacherange-read-bytes-at-until"></span>`fn read_bytes_at_until(self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

### `Bytes<'data>`

```rust
struct Bytes<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:16`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L16)*

A newtype for byte slices.

It has these important features:
- no methods that can panic, such as `Index`
- convenience methods for `Pod` types
- a useful `Debug` implementation

#### Implementations

- <span id="bytes-len"></span>`fn len(&self) -> usize`

- <span id="bytes-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="bytes-skip"></span>`fn skip(&mut self, offset: usize) -> Result<(), ()>`

- <span id="bytes-read-bytes"></span>`fn read_bytes(&mut self, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](#bytes)

- <span id="bytes-read-bytes-at"></span>`fn read_bytes_at(self, offset: usize, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](#bytes)

- <span id="bytes-read"></span>`fn read<T: Pod>(&mut self) -> Result<&'data T, ()>`

- <span id="bytes-read-at"></span>`fn read_at<T: Pod>(self, offset: usize) -> Result<&'data T, ()>`

- <span id="bytes-read-slice"></span>`fn read_slice<T: Pod>(&mut self, count: usize) -> Result<&'data [T], ()>`

- <span id="bytes-read-slice-at"></span>`fn read_slice_at<T: Pod>(self, offset: usize, count: usize) -> Result<&'data [T], ()>`

- <span id="bytes-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8], ()>`

- <span id="bytes-read-string-at"></span>`fn read_string_at(self, offset: usize) -> Result<&'data [u8], ()>`

- <span id="bytes-read-uleb128"></span>`fn read_uleb128(&mut self) -> Result<u64, ()>`

- <span id="bytes-read-sleb128"></span>`fn read_sleb128(&mut self) -> Result<i64, ()>`

#### Trait Implementations

##### `impl Clone for Bytes<'data>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'data>` — [`Bytes`](#bytes)

##### `impl Copy for Bytes<'data>`

##### `impl Debug for Bytes<'data>`

- <span id="bytes-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bytes<'data>`

- <span id="bytes-default"></span>`fn default() -> Bytes<'data>` — [`Bytes`](#bytes)

##### `impl Eq for Bytes<'data>`

##### `impl PartialEq for Bytes<'data>`

- <span id="bytes-eq"></span>`fn eq(&self, other: &Bytes<'data>) -> bool` — [`Bytes`](#bytes)

##### `impl StructuralPartialEq for Bytes<'data>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`object-0.37.3/src/read/util.rs:222`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L222)*

#### Trait Implementations

##### `impl Debug for DebugByte`

- <span id="debugbyte-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

*Defined in [`object-0.37.3/src/read/util.rs:230`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L230)*

#### Trait Implementations

##### `impl Debug for DebugLen`

- <span id="debuglen-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ByteString<'data>`

```rust
struct ByteString<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:244`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L244)*

A newtype for byte strings.

For byte slices that are strings of an unknown encoding.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

#### Trait Implementations

##### `impl Clone for ByteString<'data>`

- <span id="bytestring-clone"></span>`fn clone(&self) -> ByteString<'data>` — [`ByteString`](util/index.md)

##### `impl Copy for ByteString<'data>`

##### `impl Debug for ByteString<'data>`

- <span id="bytestring-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteString<'data>`

- <span id="bytestring-default"></span>`fn default() -> ByteString<'data>` — [`ByteString`](util/index.md)

##### `impl Eq for ByteString<'data>`

##### `impl PartialEq for ByteString<'data>`

- <span id="bytestring-eq"></span>`fn eq(&self, other: &ByteString<'data>) -> bool` — [`ByteString`](util/index.md)

##### `impl StructuralPartialEq for ByteString<'data>`

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

*Defined in [`object-0.37.3/src/read/util.rs:274-282`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L274-L282)*

A table of zero-terminated strings.

This is used by most file formats for strings such as section names and symbol names.

#### Implementations

- <span id="stringtable-new"></span>`fn new(data: R, start: u64, end: u64) -> Self`

- <span id="stringtable-get"></span>`fn get(&self, offset: u32) -> Result<&'data [u8], ()>`

#### Trait Implementations

##### `impl<'data, R> Clone for StringTable<'data, R>`

- <span id="stringtable-clone"></span>`fn clone(&self) -> StringTable<'data, R>` — [`StringTable`](#stringtable)

##### `impl<'data, R> Copy for StringTable<'data, R>`

##### `impl<'data, R> Debug for StringTable<'data, R>`

- <span id="stringtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, R: ReadRef<'data>> Default for StringTable<'data, R>`

- <span id="stringtable-default"></span>`fn default() -> Self`

### `SegmentIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SegmentIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:532-534`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L532-L534)*

An iterator for the loadable segments in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="segmentiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="segmentiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-type-item"></span>`type Item = Segment<'data, 'file, R>`

- <span id="segmentiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Segment<'data, 'file, R: ReadRef<'data>>`

```rust
struct Segment<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:574-576`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L574-L576)*

A loadable segment in a [`File`](#file).

Most functionality is provided by the [`ObjectSegment`](#objectsegment) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Segment<'data, 'file, R>`

- <span id="segment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSegment for Segment<'data, 'file, R>`

- <span id="segment-address"></span>`fn address(&self) -> u64`

- <span id="segment-size"></span>`fn size(&self) -> u64`

- <span id="segment-align"></span>`fn align(&self) -> u64`

- <span id="segment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="segment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- <span id="segment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md)

- <span id="segment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../index.md)

- <span id="segment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../index.md)

- <span id="segment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Segment<'data, 'file, R>`

### `SectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:665-667`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L665-L667)*

An iterator for the sections in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-type-item"></span>`type Item = Section<'data, 'file, R>`

- <span id="sectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Section<'data, 'file, R: ReadRef<'data>>`

```rust
struct Section<'data, 'file, R: ReadRef<'data>> {
    inner: SectionInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:708-710`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L708-L710)*

A section in a [`File`](#file).

Most functionality is provided by the [`ObjectSection`](#objectsection) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Section<'data, 'file, R>`

- <span id="section-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSection for Section<'data, 'file, R>`

- <span id="section-type-relocationiterator"></span>`type RelocationIterator = SectionRelocationIterator<'data, 'file, R>`

- <span id="section-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../index.md)

- <span id="section-address"></span>`fn address(&self) -> u64`

- <span id="section-size"></span>`fn size(&self) -> u64`

- <span id="section-align"></span>`fn align(&self) -> u64`

- <span id="section-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="section-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- <span id="section-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md)

- <span id="section-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../index.md), [`CompressedFileRange`](../index.md)

- <span id="section-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../index.md), [`CompressedData`](../index.md)

- <span id="section-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- <span id="section-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../index.md)

- <span id="section-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../index.md)

- <span id="section-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../index.md)

- <span id="section-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../index.md)

- <span id="section-relocations"></span>`fn relocations(&self) -> SectionRelocationIterator<'data, 'file, R>` — [`SectionRelocationIterator`](#sectionrelocationiterator)

- <span id="section-relocation-map"></span>`fn relocation_map(&self) -> Result<RelocationMap>` — [`Result`](../index.md), [`RelocationMap`](../index.md)

- <span id="section-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Section<'data, 'file, R>`

### `ComdatIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:843-845`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L843-L845)*

An iterator for the COMDAT section groups in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-type-item"></span>`type Item = Comdat<'data, 'file, R>`

- <span id="comdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Comdat<'data, 'file, R: ReadRef<'data>>`

```rust
struct Comdat<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:885-887`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L885-L887)*

A COMDAT section group in a [`File`](#file).

Most functionality is provided by the [`ObjectComdat`](#objectcomdat) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Comdat<'data, 'file, R>`

- <span id="comdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectComdat for Comdat<'data, 'file, R>`

- <span id="comdat-type-sectioniterator"></span>`type SectionIterator = ComdatSectionIterator<'data, 'file, R>`

- <span id="comdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../index.md)

- <span id="comdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

- <span id="comdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- <span id="comdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../index.md)

- <span id="comdat-sections"></span>`fn sections(&self) -> ComdatSectionIterator<'data, 'file, R>` — [`ComdatSectionIterator`](#comdatsectioniterator)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Comdat<'data, 'file, R>`

### `ComdatSectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatSectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatSectionIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:959-961`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L959-L961)*

An iterator for the sections in a [`Comdat`](#comdat).

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-type-item"></span>`type Item = SectionIndex`

- <span id="comdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `SymbolTable<'data, 'file, R>`

```rust
struct SymbolTable<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolTableInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1001-1006`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1001-L1006)*

A symbol table in a [`File`](#file).

Most functionality is provided by the [`ObjectSymbolTable`](#objectsymboltable) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolTable<'data, 'file, R>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSymbolTable for SymbolTable<'data, 'file, R>`

- <span id="symboltable-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="symboltable-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="symboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](#objectsymboltable)

- <span id="symboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../index.md), [`Result`](../index.md), [`ObjectSymbolTable`](#objectsymboltable)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for SymbolTable<'data, 'file, R>`

### `SymbolIterator<'data, 'file, R>`

```rust
struct SymbolIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1085-1090`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1085-L1090)*

An iterator for the symbols in a [`SymbolTable`](#symboltable).

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-type-item"></span>`type Item = Symbol<'data, 'file, R>`

- <span id="symboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Symbol<'data, 'file, R>`

```rust
struct Symbol<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1165-1170`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1165-L1170)*

An symbol in a [`SymbolTable`](#symboltable).

Most functionality is provided by the [`ObjectSymbol`](#objectsymbol) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Symbol<'data, 'file, R>`

- <span id="symbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSymbol for Symbol<'data, 'file, R>`

- <span id="symbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

- <span id="symbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- <span id="symbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../index.md)

- <span id="symbol-address"></span>`fn address(&self) -> u64`

- <span id="symbol-size"></span>`fn size(&self) -> u64`

- <span id="symbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../index.md)

- <span id="symbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../index.md)

- <span id="symbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="symbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="symbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="symbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="symbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../index.md)

- <span id="symbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="symbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="symbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../index.md), [`SectionIndex`](../index.md), [`SymbolIndex`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Symbol<'data, 'file, R>`

### `DynamicRelocationIterator<'data, 'file, R>`

```rust
struct DynamicRelocationIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: DynamicRelocationIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1301-1306`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1301-L1306)*

An iterator for the dynamic relocation entries in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dynamicrelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dynamicrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="dynamicrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `SectionRelocationIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionRelocationIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionRelocationIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1338-1340`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1338-L1340)*

An iterator for the relocation entries in a [`Section`](#section).

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionrelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="sectionrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NoDynamicRelocationIterator`

```rust
struct NoDynamicRelocationIterator;
```

*Defined in [`object-0.37.3/src/read/traits.rs:580`](../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L580)*

An iterator for files that don't have dynamic relocations.

#### Trait Implementations

##### `impl Debug for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nodynamicrelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nodynamicrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="nodynamicrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/mod.rs:198-281`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L198-L281)*

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

- <span id="filekind-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<FileKind>` — [`Result`](../index.md), [`FileKind`](../index.md)

- <span id="filekind-parse-at"></span>`fn parse_at<'data, R: ReadRef<'data>>(data: R, offset: u64) -> Result<FileKind>` — [`Result`](../index.md), [`FileKind`](../index.md)

#### Trait Implementations

##### `impl Clone for FileKind`

- <span id="filekind-clone"></span>`fn clone(&self) -> FileKind` — [`FileKind`](../index.md)

##### `impl Copy for FileKind`

##### `impl Debug for FileKind`

- <span id="filekind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileKind`

##### `impl Hash for FileKind`

- <span id="filekind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FileKind`

- <span id="filekind-eq"></span>`fn eq(&self, other: &FileKind) -> bool` — [`FileKind`](../index.md)

##### `impl StructuralPartialEq for FileKind`

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

*Defined in [`object-0.37.3/src/read/mod.rs:374-385`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L374-L385)*

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

##### `impl Clone for ObjectKind`

- <span id="objectkind-clone"></span>`fn clone(&self) -> ObjectKind` — [`ObjectKind`](../index.md)

##### `impl Copy for ObjectKind`

##### `impl Debug for ObjectKind`

- <span id="objectkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectKind`

##### `impl Hash for ObjectKind`

- <span id="objectkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ObjectKind`

- <span id="objectkind-eq"></span>`fn eq(&self, other: &ObjectKind) -> bool` — [`ObjectKind`](../index.md)

##### `impl StructuralPartialEq for ObjectKind`

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

*Defined in [`object-0.37.3/src/read/mod.rs:410-423`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L410-L423)*

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

- <span id="symbolsection-index"></span>`fn index(self) -> Option<SectionIndex>` — [`SectionIndex`](../index.md)

#### Trait Implementations

##### `impl Clone for SymbolSection`

- <span id="symbolsection-clone"></span>`fn clone(&self) -> SymbolSection` — [`SymbolSection`](../index.md)

##### `impl Copy for SymbolSection`

##### `impl Debug for SymbolSection`

- <span id="symbolsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolSection`

##### `impl Hash for SymbolSection`

- <span id="symbolsection-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolSection`

- <span id="symbolsection-eq"></span>`fn eq(&self, other: &SymbolSection) -> bool` — [`SymbolSection`](../index.md)

##### `impl StructuralPartialEq for SymbolSection`

### `RelocationTarget`

```rust
enum RelocationTarget {
    Symbol(SymbolIndex),
    Section(SectionIndex),
    Absolute,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:703-710`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L703-L710)*

The target referenced by a [`Relocation`](../index.md).

#### Variants

- **`Symbol`**

  The target is a symbol.

- **`Section`**

  The target is a section.

- **`Absolute`**

  The offset is an absolute address.

#### Trait Implementations

##### `impl Clone for RelocationTarget`

- <span id="relocationtarget-clone"></span>`fn clone(&self) -> RelocationTarget` — [`RelocationTarget`](../index.md)

##### `impl Copy for RelocationTarget`

##### `impl Debug for RelocationTarget`

- <span id="relocationtarget-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationTarget`

##### `impl Hash for RelocationTarget`

- <span id="relocationtarget-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationTarget`

- <span id="relocationtarget-eq"></span>`fn eq(&self, other: &RelocationTarget) -> bool` — [`RelocationTarget`](../index.md)

##### `impl StructuralPartialEq for RelocationTarget`

### `CompressionFormat`

```rust
enum CompressionFormat {
    None,
    Unknown,
    Zlib,
    Zstandard,
}
```

*Defined in [`object-0.37.3/src/read/mod.rs:879-892`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L879-L892)*

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

##### `impl Clone for CompressionFormat`

- <span id="compressionformat-clone"></span>`fn clone(&self) -> CompressionFormat` — [`CompressionFormat`](../index.md)

##### `impl Copy for CompressionFormat`

##### `impl Debug for CompressionFormat`

- <span id="compressionformat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressionFormat`

##### `impl Hash for CompressionFormat`

- <span id="compressionformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressionFormat`

- <span id="compressionformat-eq"></span>`fn eq(&self, other: &CompressionFormat) -> bool` — [`CompressionFormat`](../index.md)

##### `impl StructuralPartialEq for CompressionFormat`

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

*Defined in [`object-0.37.3/src/common.rs:5-45`](../../../.source_1765210505/object-0.37.3/src/common.rs#L5-L45)*

A CPU architecture.

#### Implementations

- <span id="architecture-address-size"></span>`fn address_size(self) -> Option<AddressSize>` — [`AddressSize`](../index.md)

#### Trait Implementations

##### `impl Clone for Architecture`

- <span id="architecture-clone"></span>`fn clone(&self) -> Architecture` — [`Architecture`](../index.md)

##### `impl Copy for Architecture`

##### `impl Debug for Architecture`

- <span id="architecture-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Architecture`

##### `impl Hash for Architecture`

- <span id="architecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Architecture`

- <span id="architecture-eq"></span>`fn eq(&self, other: &Architecture) -> bool` — [`Architecture`](../index.md)

##### `impl StructuralPartialEq for Architecture`

### `SubArchitecture`

```rust
enum SubArchitecture {
    Arm64E,
    Arm64EC,
}
```

*Defined in [`object-0.37.3/src/common.rs:51-54`](../../../.source_1765210505/object-0.37.3/src/common.rs#L51-L54)*

A CPU sub-architecture.

#### Trait Implementations

##### `impl Clone for SubArchitecture`

- <span id="subarchitecture-clone"></span>`fn clone(&self) -> SubArchitecture` — [`SubArchitecture`](../index.md)

##### `impl Copy for SubArchitecture`

##### `impl Debug for SubArchitecture`

- <span id="subarchitecture-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubArchitecture`

##### `impl Hash for SubArchitecture`

- <span id="subarchitecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SubArchitecture`

- <span id="subarchitecture-eq"></span>`fn eq(&self, other: &SubArchitecture) -> bool` — [`SubArchitecture`](../index.md)

##### `impl StructuralPartialEq for SubArchitecture`

### `AddressSize`

```rust
enum AddressSize {
    U8,
    U16,
    U32,
    U64,
}
```

*Defined in [`object-0.37.3/src/common.rs:109-114`](../../../.source_1765210505/object-0.37.3/src/common.rs#L109-L114)*

The size of an address value for an architecture.

This may differ from the address size supported by the file format (such as for COFF).

#### Implementations

- <span id="addresssize-bytes"></span>`fn bytes(self) -> u8`

#### Trait Implementations

##### `impl Clone for AddressSize`

- <span id="addresssize-clone"></span>`fn clone(&self) -> AddressSize` — [`AddressSize`](../index.md)

##### `impl Copy for AddressSize`

##### `impl Debug for AddressSize`

- <span id="addresssize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddressSize`

##### `impl Hash for AddressSize`

- <span id="addresssize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for AddressSize`

- <span id="addresssize-eq"></span>`fn eq(&self, other: &AddressSize) -> bool` — [`AddressSize`](../index.md)

##### `impl StructuralPartialEq for AddressSize`

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

*Defined in [`object-0.37.3/src/common.rs:128-135`](../../../.source_1765210505/object-0.37.3/src/common.rs#L128-L135)*

A binary file format.

#### Implementations

- <span id="binaryformat-native-object"></span>`fn native_object() -> BinaryFormat` — [`BinaryFormat`](../index.md)

#### Trait Implementations

##### `impl Clone for BinaryFormat`

- <span id="binaryformat-clone"></span>`fn clone(&self) -> BinaryFormat` — [`BinaryFormat`](../index.md)

##### `impl Copy for BinaryFormat`

##### `impl Debug for BinaryFormat`

- <span id="binaryformat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BinaryFormat`

##### `impl Hash for BinaryFormat`

- <span id="binaryformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BinaryFormat`

- <span id="binaryformat-eq"></span>`fn eq(&self, other: &BinaryFormat) -> bool` — [`BinaryFormat`](../index.md)

##### `impl StructuralPartialEq for BinaryFormat`

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

*Defined in [`object-0.37.3/src/common.rs:155-247`](../../../.source_1765210505/object-0.37.3/src/common.rs#L155-L247)*

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

#### Trait Implementations

##### `impl Clone for SectionKind`

- <span id="sectionkind-clone"></span>`fn clone(&self) -> SectionKind` — [`SectionKind`](../index.md)

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- <span id="sectionkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionKind`

##### `impl Hash for SectionKind`

- <span id="sectionkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionKind`

- <span id="sectionkind-eq"></span>`fn eq(&self, other: &SectionKind) -> bool` — [`SectionKind`](../index.md)

##### `impl StructuralPartialEq for SectionKind`

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

*Defined in [`object-0.37.3/src/common.rs:264-291`](../../../.source_1765210505/object-0.37.3/src/common.rs#L264-L291)*

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

##### `impl Clone for ComdatKind`

- <span id="comdatkind-clone"></span>`fn clone(&self) -> ComdatKind` — [`ComdatKind`](../index.md)

##### `impl Copy for ComdatKind`

##### `impl Debug for ComdatKind`

- <span id="comdatkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatKind`

##### `impl Hash for ComdatKind`

- <span id="comdatkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ComdatKind`

- <span id="comdatkind-eq"></span>`fn eq(&self, other: &ComdatKind) -> bool` — [`ComdatKind`](../index.md)

##### `impl StructuralPartialEq for ComdatKind`

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

*Defined in [`object-0.37.3/src/common.rs:296-311`](../../../.source_1765210505/object-0.37.3/src/common.rs#L296-L311)*

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

##### `impl Clone for SymbolKind`

- <span id="symbolkind-clone"></span>`fn clone(&self) -> SymbolKind` — [`SymbolKind`](../index.md)

##### `impl Copy for SymbolKind`

##### `impl Debug for SymbolKind`

- <span id="symbolkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolKind`

##### `impl Hash for SymbolKind`

- <span id="symbolkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolKind`

- <span id="symbolkind-eq"></span>`fn eq(&self, other: &SymbolKind) -> bool` — [`SymbolKind`](../index.md)

##### `impl StructuralPartialEq for SymbolKind`

### `SymbolScope`

```rust
enum SymbolScope {
    Unknown,
    Compilation,
    Linkage,
    Dynamic,
}
```

*Defined in [`object-0.37.3/src/common.rs:315-324`](../../../.source_1765210505/object-0.37.3/src/common.rs#L315-L324)*

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

##### `impl Clone for SymbolScope`

- <span id="symbolscope-clone"></span>`fn clone(&self) -> SymbolScope` — [`SymbolScope`](../index.md)

##### `impl Copy for SymbolScope`

##### `impl Debug for SymbolScope`

- <span id="symbolscope-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolScope`

##### `impl Hash for SymbolScope`

- <span id="symbolscope-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolScope`

- <span id="symbolscope-eq"></span>`fn eq(&self, other: &SymbolScope) -> bool` — [`SymbolScope`](../index.md)

##### `impl StructuralPartialEq for SymbolScope`

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

*Defined in [`object-0.37.3/src/common.rs:343-366`](../../../.source_1765210505/object-0.37.3/src/common.rs#L343-L366)*

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

##### `impl Clone for RelocationKind`

- <span id="relocationkind-clone"></span>`fn clone(&self) -> RelocationKind` — [`RelocationKind`](../index.md)

##### `impl Copy for RelocationKind`

##### `impl Debug for RelocationKind`

- <span id="relocationkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationKind`

##### `impl Hash for RelocationKind`

- <span id="relocationkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationKind`

- <span id="relocationkind-eq"></span>`fn eq(&self, other: &RelocationKind) -> bool` — [`RelocationKind`](../index.md)

##### `impl StructuralPartialEq for RelocationKind`

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

*Defined in [`object-0.37.3/src/common.rs:374-447`](../../../.source_1765210505/object-0.37.3/src/common.rs#L374-L447)*

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

##### `impl Clone for RelocationEncoding`

- <span id="relocationencoding-clone"></span>`fn clone(&self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md)

##### `impl Copy for RelocationEncoding`

##### `impl Debug for RelocationEncoding`

- <span id="relocationencoding-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationEncoding`

##### `impl Hash for RelocationEncoding`

- <span id="relocationencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationEncoding`

- <span id="relocationencoding-eq"></span>`fn eq(&self, other: &RelocationEncoding) -> bool` — [`RelocationEncoding`](../index.md)

##### `impl StructuralPartialEq for RelocationEncoding`

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

*Defined in [`object-0.37.3/src/common.rs:452-479`](../../../.source_1765210505/object-0.37.3/src/common.rs#L452-L479)*

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

##### `impl Clone for FileFlags`

- <span id="fileflags-clone"></span>`fn clone(&self) -> FileFlags` — [`FileFlags`](../index.md)

##### `impl Copy for FileFlags`

##### `impl Debug for FileFlags`

- <span id="fileflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileFlags`

##### `impl Hash for FileFlags`

- <span id="fileflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FileFlags`

- <span id="fileflags-eq"></span>`fn eq(&self, other: &FileFlags) -> bool` — [`FileFlags`](../index.md)

##### `impl StructuralPartialEq for FileFlags`

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

*Defined in [`object-0.37.3/src/common.rs:484-506`](../../../.source_1765210505/object-0.37.3/src/common.rs#L484-L506)*

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

##### `impl Clone for SegmentFlags`

- <span id="segmentflags-clone"></span>`fn clone(&self) -> SegmentFlags` — [`SegmentFlags`](../index.md)

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- <span id="segmentflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SegmentFlags`

##### `impl Hash for SegmentFlags`

- <span id="segmentflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SegmentFlags`

- <span id="segmentflags-eq"></span>`fn eq(&self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](../index.md)

##### `impl StructuralPartialEq for SegmentFlags`

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

*Defined in [`object-0.37.3/src/common.rs:511-534`](../../../.source_1765210505/object-0.37.3/src/common.rs#L511-L534)*

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

##### `impl Clone for SectionFlags`

- <span id="sectionflags-clone"></span>`fn clone(&self) -> SectionFlags` — [`SectionFlags`](../index.md)

##### `impl Copy for SectionFlags`

##### `impl Debug for SectionFlags`

- <span id="sectionflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionFlags`

##### `impl Hash for SectionFlags`

- <span id="sectionflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionFlags`

- <span id="sectionflags-eq"></span>`fn eq(&self, other: &SectionFlags) -> bool` — [`SectionFlags`](../index.md)

##### `impl StructuralPartialEq for SectionFlags`

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

*Defined in [`object-0.37.3/src/common.rs:539-578`](../../../.source_1765210505/object-0.37.3/src/common.rs#L539-L578)*

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

##### `impl<Section: clone::Clone, Symbol: clone::Clone> Clone for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-clone"></span>`fn clone(&self) -> SymbolFlags<Section, Symbol>` — [`SymbolFlags`](../index.md)

##### `impl<Section: marker::Copy, Symbol: marker::Copy> Copy for SymbolFlags<Section, Symbol>`

##### `impl<Section: fmt::Debug, Symbol: fmt::Debug> Debug for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section: cmp::Eq, Symbol: cmp::Eq> Eq for SymbolFlags<Section, Symbol>`

##### `impl<Section: hash::Hash, Symbol: hash::Hash> Hash for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<Section: cmp::PartialEq, Symbol: cmp::PartialEq> PartialEq for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-eq"></span>`fn eq(&self, other: &SymbolFlags<Section, Symbol>) -> bool` — [`SymbolFlags`](../index.md)

##### `impl<Section, Symbol> StructuralPartialEq for SymbolFlags<Section, Symbol>`

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

*Defined in [`object-0.37.3/src/common.rs:583-619`](../../../.source_1765210505/object-0.37.3/src/common.rs#L583-L619)*

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

##### `impl Clone for RelocationFlags`

- <span id="relocationflags-clone"></span>`fn clone(&self) -> RelocationFlags` — [`RelocationFlags`](../index.md)

##### `impl Copy for RelocationFlags`

##### `impl Debug for RelocationFlags`

- <span id="relocationflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationFlags`

##### `impl Hash for RelocationFlags`

- <span id="relocationflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationFlags`

- <span id="relocationflags-eq"></span>`fn eq(&self, other: &RelocationFlags) -> bool` — [`RelocationFlags`](../index.md)

##### `impl StructuralPartialEq for RelocationFlags`

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

*Defined in [`object-0.37.3/src/read/any.rs:213-236`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L213-L236)*

An object file that can be any supported file format.

Most functionality is provided by the [`Object`](#object) trait implementation.

#### Implementations

- <span id="file-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../index.md)

- <span id="file-parse-dyld-cache-image"></span>`fn parse_dyld_cache_image<'cache, E: crate::Endian>(image: &macho::DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](macho/index.md), [`Result`](../index.md)

- <span id="file-format"></span>`fn format(&self) -> BinaryFormat` — [`BinaryFormat`](../index.md)

#### Trait Implementations

##### `impl<'data, R: fmt::Debug + ReadRef<'data>> Debug for File<'data, R>`

- <span id="file-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, R> Object for File<'data, R>`

- <span id="file-type-segment"></span>`type Segment = Segment<'data, 'file, R>`

- <span id="file-type-segmentiterator"></span>`type SegmentIterator = SegmentIterator<'data, 'file, R>`

- <span id="file-type-section"></span>`type Section = Section<'data, 'file, R>`

- <span id="file-type-sectioniterator"></span>`type SectionIterator = SectionIterator<'data, 'file, R>`

- <span id="file-type-comdat"></span>`type Comdat = Comdat<'data, 'file, R>`

- <span id="file-type-comdatiterator"></span>`type ComdatIterator = ComdatIterator<'data, 'file, R>`

- <span id="file-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="file-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="file-type-symboltable"></span>`type SymbolTable = SymbolTable<'data, 'file, R>`

- <span id="file-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = DynamicRelocationIterator<'data, 'file, R>`

- <span id="file-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../index.md)

- <span id="file-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../index.md)

- <span id="file-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="file-is-64"></span>`fn is_64(&self) -> bool`

- <span id="file-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../index.md)

- <span id="file-segments"></span>`fn segments(&self) -> SegmentIterator<'data, '_, R>` — [`SegmentIterator`](#segmentiterator)

- <span id="file-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<Section<'data, 'file, R>>` — [`Section`](#section)

- <span id="file-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<Section<'data, '_, R>>` — [`SectionIndex`](../index.md), [`Result`](../index.md), [`Section`](#section)

- <span id="file-sections"></span>`fn sections(&self) -> SectionIterator<'data, '_, R>` — [`SectionIterator`](#sectioniterator)

- <span id="file-comdats"></span>`fn comdats(&self) -> ComdatIterator<'data, '_, R>` — [`ComdatIterator`](#comdatiterator)

- <span id="file-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<Symbol<'data, '_, R>>` — [`SymbolIndex`](../index.md), [`Result`](../index.md), [`Symbol`](#symbol)

- <span id="file-symbols"></span>`fn symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](#symboliterator)

- <span id="file-symbol-table"></span>`fn symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](#symboltable)

- <span id="file-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](#symboliterator)

- <span id="file-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](#symboltable)

- <span id="file-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<DynamicRelocationIterator<'data, '_, R>>` — [`DynamicRelocationIterator`](#dynamicrelocationiterator)

- <span id="file-symbol-map"></span>`fn symbol_map(&self) -> SymbolMap<SymbolMapName<'data>>` — [`SymbolMap`](../index.md), [`SymbolMapName`](../index.md)

- <span id="file-object-map"></span>`fn object_map(&self) -> ObjectMap<'data>` — [`ObjectMap`](../index.md)

- <span id="file-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../index.md), [`Import`](../index.md)

- <span id="file-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../index.md), [`Export`](../index.md)

- <span id="file-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="file-mach-uuid"></span>`fn mach_uuid(&self) -> Result<Option<[u8; 16]>>` — [`Result`](../index.md)

- <span id="file-build-id"></span>`fn build_id(&self) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md)

- <span id="file-gnu-debuglink"></span>`fn gnu_debuglink(&self) -> Result<Option<(&'data [u8], u32)>>` — [`Result`](../index.md)

- <span id="file-gnu-debugaltlink"></span>`fn gnu_debugaltlink(&self) -> Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../index.md)

- <span id="file-pdb-info"></span>`fn pdb_info(&self) -> Result<Option<CodeView<'_>>>` — [`Result`](../index.md), [`CodeView`](../index.md)

- <span id="file-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="file-entry"></span>`fn entry(&self) -> u64`

- <span id="file-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../index.md)

##### `impl<'data, R: ReadRef<'data>> Sealed for File<'data, R>`

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

*Defined in [`object-0.37.3/src/read/any.rs:537-560`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L537-L560)*

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:579-602`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L579-L602)*

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:671-694`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L671-L694)*

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:712-735`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L712-L735)*

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

*Defined in [`object-0.37.3/src/read/any.rs:848-871`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L848-L871)*

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:889-912`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L889-L912)*

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

*Defined in [`object-0.37.3/src/read/any.rs:964-987`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L964-L987)*

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:1009-1055`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1009-L1055)*

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:1093-1149`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1093-L1149)*

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:1172-1218`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1172-L1218)*

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

*Defined in [`object-0.37.3/src/read/any.rs:1309-1320`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1309-L1320)*

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/any.rs:1343-1366`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L1343-L1366)*

#### Trait Implementations

##### `impl<'data, 'file, R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `ReadError<T>`

```rust
trait ReadError<T> { ... }
```

*Defined in [`object-0.37.3/src/read/mod.rs:133-135`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L133-L135)*

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

*Defined in [`object-0.37.3/src/read/mod.rs:440-443`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L440-L443)*

An entry in a [`SymbolMap`](../index.md).

#### Required Methods

- `fn address(&self) -> u64`

  The symbol address.

#### Implementors

- [`ObjectMapEntry`](../index.md)
- [`SymbolMapName`](../index.md)

### `ReadRef<'a>`

```rust
trait ReadRef<'a>: Clone + Copy { ... }
```

*Defined in [`object-0.37.3/src/read/read_ref.rs:49-124`](../../../.source_1765210505/object-0.37.3/src/read/read_ref.rs#L49-L124)*

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

*Defined in [`object-0.37.3/src/read/read_cache.rs:222-242`](../../../.source_1765210505/object-0.37.3/src/read/read_cache.rs#L222-L242)*

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

*Defined in [`object-0.37.3/src/read/traits.rs:15-335`](../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L15-L335)*

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

- [`CoffFile`](coff/index.md)
- [`ElfFile`](elf/index.md)
- [`File`](#file)
- [`MachOFile`](macho/index.md)
- [`PeFile`](pe/index.md)
- [`XcoffFile`](xcoff/index.md)

### `ObjectSegment<'data>`

```rust
trait ObjectSegment<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:340-374`](../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L340-L374)*

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

- [`CoffSegment`](coff/index.md)
- [`ElfSegment`](elf/index.md)
- [`MachOSegment`](macho/index.md)
- [`PeSegment`](pe/index.md)
- [`Segment`](#segment)
- [`XcoffSegment`](xcoff/index.md)

### `ObjectSection<'data>`

```rust
trait ObjectSection<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:379-462`](../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L379-L462)*

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

- [`CoffSection`](coff/index.md)
- [`ElfSection`](elf/index.md)
- [`MachOSection`](macho/index.md)
- [`PeSection`](pe/index.md)
- [`Section`](#section)
- [`XcoffSection`](xcoff/index.md)

### `ObjectComdat<'data>`

```rust
trait ObjectComdat<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:467-487`](../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L467-L487)*

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

- [`CoffComdat`](coff/index.md)
- [`Comdat`](#comdat)
- [`ElfComdat`](elf/index.md)
- [`MachOComdat`](macho/index.md)
- [`PeComdat`](pe/index.md)
- [`XcoffComdat`](xcoff/index.md)

### `ObjectSymbolTable<'data>`

```rust
trait ObjectSymbolTable<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:492-510`](../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L492-L510)*

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

- [`CoffSymbolTable`](coff/index.md)
- [`ElfSymbolTable`](elf/index.md)
- [`MachOSymbolTable`](macho/index.md)
- [`SymbolTable`](#symboltable)
- [`XcoffSymbolTable`](xcoff/index.md)

### `ObjectSymbol<'data>`

```rust
trait ObjectSymbol<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:515-576`](../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L515-L576)*

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

- [`CoffSymbol`](coff/index.md)
- [`ElfSymbol`](elf/index.md)
- [`MachOSymbol`](macho/index.md)
- [`Symbol`](#symbol)
- [`XcoffSymbol`](xcoff/index.md)

## Functions

### `debug_list_bytes`

```rust
fn debug_list_bytes(bytes: &[u8], fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`object-0.37.3/src/read/util.rs:213-220`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L213-L220)*

### `align`

```rust
fn align(offset: usize, size: usize) -> usize
```

*Defined in [`object-0.37.3/src/read/util.rs:254-256`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L254-L256)*

### `data_range`

```rust
fn data_range(data: &[u8], data_address: u64, range_address: u64, size: u64) -> Option<&[u8]>
```

*Defined in [`object-0.37.3/src/read/util.rs:259-268`](../../../.source_1765210505/object-0.37.3/src/read/util.rs#L259-L268)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

*Defined in [`object-0.37.3/src/read/mod.rs:131`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L131)*

The result type used within the read module.

### `NativeFile<'data, R>`

```rust
type NativeFile<'data, R> = elf::ElfFile64<'data, crate::endian::Endianness, R>;
```

*Defined in [`object-0.37.3/src/read/mod.rs:171`](../../../.source_1765210505/object-0.37.3/src/read/mod.rs#L171)*

The native executable file for the target platform.

### `Result<T>`

```rust
type Result<T> = result::Result<T, ()>;
```

*Defined in [`object-0.37.3/src/read/read_ref.rs:9`](../../../.source_1765210505/object-0.37.3/src/read/read_ref.rs#L9)*

## Macros

### `with_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:30-57`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L30-L57)*

Evaluate an expression on the contents of a file format enum.

This is a hack to avoid virtual calls.

### `with_inner_mut!`

*Defined in [`object-0.37.3/src/read/any.rs:59-86`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L59-L86)*

### `map_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:89-116`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L89-L116)*

Like `with_inner!`, but wraps the result in another enum.

### `map_inner_option!`

*Defined in [`object-0.37.3/src/read/any.rs:119-146`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L119-L146)*

Like `map_inner!`, but the result is a Result or Option.

### `map_inner_option_mut!`

*Defined in [`object-0.37.3/src/read/any.rs:148-175`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L148-L175)*

### `next_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:178-205`](../../../.source_1765210505/object-0.37.3/src/read/any.rs#L178-L205)*

Call `next` for a file format iterator.

