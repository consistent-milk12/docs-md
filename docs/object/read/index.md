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

## Modules

- [`archive`](archive/index.md) - Support for archive files.
- [`coff`](coff/index.md) - Support for reading Windows COFF files.
- [`elf`](elf/index.md) - Support for reading ELF files.
- [`macho`](macho/index.md) - Support for reading Mach-O files.
- [`pe`](pe/index.md) - Support for reading PE files.
- [`xcoff`](xcoff/index.md) - Support for reading AIX XCOFF files.

## Structs

### `Error`

```rust
struct Error(&'static str);
```

The error type used within the read module.

#### Trait Implementations

##### `impl Clone for Error`

- `fn clone(self: &Self) -> Error` — [`Error`](../index.md)

##### `impl Copy for Error`

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- `fn eq(self: &Self, other: &Error) -> bool` — [`Error`](../index.md)

##### `impl StructuralPartialEq for Error`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

### `SectionIndex`

```rust
struct SectionIndex(usize);
```

The index used to identify a section in a file.

#### Trait Implementations

##### `impl Clone for SectionIndex`

- `fn clone(self: &Self) -> SectionIndex` — [`SectionIndex`](../index.md)

##### `impl Copy for SectionIndex`

##### `impl Debug for SectionIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SectionIndex`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionIndex`

##### `impl Hash for SectionIndex`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SectionIndex`

- `fn eq(self: &Self, other: &SectionIndex) -> bool` — [`SectionIndex`](../index.md)

##### `impl StructuralPartialEq for SectionIndex`

##### `impl<T> ToString for SectionIndex`

- `fn to_string(self: &Self) -> String`

### `SymbolIndex`

```rust
struct SymbolIndex(usize);
```

The index used to identify a symbol in a symbol table.

#### Trait Implementations

##### `impl Clone for SymbolIndex`

- `fn clone(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

##### `impl Copy for SymbolIndex`

##### `impl Debug for SymbolIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SymbolIndex`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolIndex`

##### `impl Hash for SymbolIndex`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolIndex`

- `fn eq(self: &Self, other: &SymbolIndex) -> bool` — [`SymbolIndex`](../index.md)

##### `impl StructuralPartialEq for SymbolIndex`

##### `impl<T> ToString for SymbolIndex`

- `fn to_string(self: &Self) -> String`

### `SymbolMap<T: SymbolMapEntry>`

```rust
struct SymbolMap<T: SymbolMapEntry> {
    symbols: alloc::vec::Vec<T>,
}
```

A map from addresses to symbol information.

The symbol information depends on the chosen entry type, such as [`SymbolMapName`](../index.md).

Returned by `Object::symbol_map`.

#### Implementations

- `fn new(symbols: Vec<T>) -> Self`

- `fn get(self: &Self, address: u64) -> Option<&T>`

- `fn symbols(self: &Self) -> &[T]`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + SymbolMapEntry> Clone for SymbolMap<T>`

- `fn clone(self: &Self) -> SymbolMap<T>` — [`SymbolMap`](../index.md)

##### `impl<T: $crate::fmt::Debug + SymbolMapEntry> Debug for SymbolMap<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::default::Default + SymbolMapEntry> Default for SymbolMap<T>`

- `fn default() -> SymbolMap<T>` — [`SymbolMap`](../index.md)

### `SymbolMapName<'data>`

```rust
struct SymbolMapName<'data> {
    address: u64,
    name: &'data str,
}
```

The type used for entries in a [`SymbolMap`](../index.md) that maps from addresses to names.

#### Implementations

- `fn new(address: u64, name: &'data str) -> Self`

- `fn address(self: &Self) -> u64`

- `fn name(self: &Self) -> &'data str`

#### Trait Implementations

##### `impl<'data> Clone for SymbolMapName<'data>`

- `fn clone(self: &Self) -> SymbolMapName<'data>` — [`SymbolMapName`](../index.md)

##### `impl<'data> Copy for SymbolMapName<'data>`

##### `impl<'data> Debug for SymbolMapName<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for SymbolMapName<'data>`

##### `impl<'data> Hash for SymbolMapName<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for SymbolMapName<'data>`

- `fn eq(self: &Self, other: &SymbolMapName<'data>) -> bool` — [`SymbolMapName`](../index.md)

##### `impl<'data> StructuralPartialEq for SymbolMapName<'data>`

##### `impl<'data> SymbolMapEntry for SymbolMapName<'data>`

- `fn address(self: &Self) -> u64`

### `ObjectMap<'data>`

```rust
struct ObjectMap<'data> {
    symbols: SymbolMap<ObjectMapEntry<'data>>,
    objects: alloc::vec::Vec<ObjectMapFile<'data>>,
}
```

A map from addresses to symbol names and object files.

This is derived from STAB entries in Mach-O files.

Returned by `Object::object_map`.

#### Implementations

- `fn get(self: &Self, address: u64) -> Option<&ObjectMapEntry<'data>>` — [`ObjectMapEntry`](../index.md)

- `fn symbols(self: &Self) -> &[ObjectMapEntry<'data>]` — [`ObjectMapEntry`](../index.md)

- `fn objects(self: &Self) -> &[ObjectMapFile<'data>]` — [`ObjectMapFile`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ObjectMap<'data>`

- `fn clone(self: &Self) -> ObjectMap<'data>` — [`ObjectMap`](../index.md)

##### `impl<'data> Debug for ObjectMap<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for ObjectMap<'data>`

- `fn default() -> ObjectMap<'data>` — [`ObjectMap`](../index.md)

### `ObjectMapEntry<'data>`

```rust
struct ObjectMapEntry<'data> {
    address: u64,
    size: u64,
    name: &'data [u8],
    object: usize,
}
```

A symbol in an [`ObjectMap`](../index.md).

#### Implementations

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn name(self: &Self) -> &'data [u8]`

- `fn object_index(self: &Self) -> usize`

- `fn object<'a>(self: &Self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>` — [`ObjectMap`](../index.md), [`ObjectMapFile`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for ObjectMapEntry<'data>`

- `fn clone(self: &Self) -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../index.md)

##### `impl<'data> Copy for ObjectMapEntry<'data>`

##### `impl<'data> Debug for ObjectMapEntry<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for ObjectMapEntry<'data>`

- `fn default() -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../index.md)

##### `impl<'data> Eq for ObjectMapEntry<'data>`

##### `impl<'data> Hash for ObjectMapEntry<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for ObjectMapEntry<'data>`

- `fn eq(self: &Self, other: &ObjectMapEntry<'data>) -> bool` — [`ObjectMapEntry`](../index.md)

##### `impl<'data> StructuralPartialEq for ObjectMapEntry<'data>`

##### `impl<'data> SymbolMapEntry for ObjectMapEntry<'data>`

- `fn address(self: &Self) -> u64`

### `ObjectMapFile<'data>`

```rust
struct ObjectMapFile<'data> {
    path: &'data [u8],
    member: Option<&'data [u8]>,
}
```

An object file name in an [`ObjectMap`](../index.md).

#### Implementations

- `fn new(path: &'data [u8], member: Option<&'data [u8]>) -> Self`

- `fn path(self: &Self) -> &'data [u8]`

- `fn member(self: &Self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for ObjectMapFile<'data>`

- `fn clone(self: &Self) -> ObjectMapFile<'data>` — [`ObjectMapFile`](../index.md)

##### `impl<'data> Copy for ObjectMapFile<'data>`

##### `impl<'data> Debug for ObjectMapFile<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for ObjectMapFile<'data>`

##### `impl<'data> Hash for ObjectMapFile<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for ObjectMapFile<'data>`

- `fn eq(self: &Self, other: &ObjectMapFile<'data>) -> bool` — [`ObjectMapFile`](../index.md)

##### `impl<'data> StructuralPartialEq for ObjectMapFile<'data>`

### `Import<'data>`

```rust
struct Import<'data> {
    library: ByteString<'data>,
    name: ByteString<'data>,
}
```

An imported symbol.

Returned by `Object::imports`.

#### Implementations

- `fn name(self: &Self) -> &'data [u8]`

- `fn library(self: &Self) -> &'data [u8]`

#### Trait Implementations

##### `impl<'data> Clone for Import<'data>`

- `fn clone(self: &Self) -> Import<'data>` — [`Import`](../index.md)

##### `impl<'data> Copy for Import<'data>`

##### `impl<'data> Debug for Import<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for Import<'data>`

##### `impl<'data> PartialEq for Import<'data>`

- `fn eq(self: &Self, other: &Import<'data>) -> bool` — [`Import`](../index.md)

##### `impl<'data> StructuralPartialEq for Import<'data>`

### `Export<'data>`

```rust
struct Export<'data> {
    name: ByteString<'data>,
    address: u64,
}
```

An exported symbol.

Returned by `Object::exports`.

#### Implementations

- `fn name(self: &Self) -> &'data [u8]`

- `fn address(self: &Self) -> u64`

#### Trait Implementations

##### `impl<'data> Clone for Export<'data>`

- `fn clone(self: &Self) -> Export<'data>` — [`Export`](../index.md)

##### `impl<'data> Copy for Export<'data>`

##### `impl<'data> Debug for Export<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for Export<'data>`

##### `impl<'data> PartialEq for Export<'data>`

- `fn eq(self: &Self, other: &Export<'data>) -> bool` — [`Export`](../index.md)

##### `impl<'data> StructuralPartialEq for Export<'data>`

### `CodeView<'data>`

```rust
struct CodeView<'data> {
    guid: [u8; 16],
    path: ByteString<'data>,
    age: u32,
}
```

PDB information from the debug directory in a PE file.

#### Implementations

- `fn path(self: &Self) -> &'data [u8]`

- `fn age(self: &Self) -> u32`

- `fn guid(self: &Self) -> [u8; 16]`

#### Trait Implementations

##### `impl<'data> Clone for CodeView<'data>`

- `fn clone(self: &Self) -> CodeView<'data>` — [`CodeView`](../index.md)

##### `impl<'data> Copy for CodeView<'data>`

##### `impl<'data> Debug for CodeView<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for CodeView<'data>`

##### `impl<'data> PartialEq for CodeView<'data>`

- `fn eq(self: &Self, other: &CodeView<'data>) -> bool` — [`CodeView`](../index.md)

##### `impl<'data> StructuralPartialEq for CodeView<'data>`

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

A relocation entry.

Returned by `Object::dynamic_relocations` or `ObjectSection::relocations`.

#### Implementations

- `fn kind(self: &Self) -> RelocationKind` — [`RelocationKind`](../index.md)

- `fn encoding(self: &Self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md)

- `fn size(self: &Self) -> u8`

- `fn target(self: &Self) -> RelocationTarget` — [`RelocationTarget`](../index.md)

- `fn addend(self: &Self) -> i64`

- `fn set_addend(self: &mut Self, addend: i64)`

- `fn has_implicit_addend(self: &Self) -> bool`

- `fn flags(self: &Self) -> RelocationFlags` — [`RelocationFlags`](../index.md)

#### Trait Implementations

##### `impl Debug for Relocation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RelocationMap`

```rust
struct RelocationMap(alloc::collections::btree_map::BTreeMap<u64, RelocationMapEntry>);
```

A map from section offsets to relocation information.

This can be used to apply relocations to a value at a given section offset.
This is intended for use with DWARF in relocatable object files, and only
supports relocations that are used in DWARF.

Returned by `ObjectSection::relocation_map`.

#### Implementations

- `fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>` — [`Object`](#object), [`Result`](../index.md)

- `fn add<'data: 'file, 'file, T>(self: &mut Self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>` — [`Relocation`](../index.md), [`Result`](../index.md)

- `fn relocate(self: &Self, offset: u64, value: u64) -> u64`

#### Trait Implementations

##### `impl Debug for RelocationMap`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RelocationMap`

- `fn default() -> RelocationMap` — [`RelocationMap`](../index.md)

### `CompressedFileRange`

```rust
struct CompressedFileRange {
    pub format: CompressionFormat,
    pub offset: u64,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
}
```

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

- `fn none(range: Option<(u64, u64)>) -> Self`

- `fn data<'data, R: ReadRef<'data>>(self: Self, file: R) -> Result<CompressedData<'data>>` — [`Result`](../index.md), [`CompressedData`](../index.md)

#### Trait Implementations

##### `impl Clone for CompressedFileRange`

- `fn clone(self: &Self) -> CompressedFileRange` — [`CompressedFileRange`](../index.md)

##### `impl Copy for CompressedFileRange`

##### `impl Debug for CompressedFileRange`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for CompressedFileRange`

##### `impl Hash for CompressedFileRange`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for CompressedFileRange`

- `fn eq(self: &Self, other: &CompressedFileRange) -> bool` — [`CompressedFileRange`](../index.md)

##### `impl StructuralPartialEq for CompressedFileRange`

### `CompressedData<'data>`

```rust
struct CompressedData<'data> {
    pub format: CompressionFormat,
    pub data: &'data [u8],
    pub uncompressed_size: u64,
}
```

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

- `fn none(data: &'data [u8]) -> Self`

- `fn decompress(self: Self) -> Result<Cow<'data, [u8]>>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for CompressedData<'data>`

- `fn clone(self: &Self) -> CompressedData<'data>` — [`CompressedData`](../index.md)

##### `impl<'data> Copy for CompressedData<'data>`

##### `impl<'data> Debug for CompressedData<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for CompressedData<'data>`

##### `impl<'data> Hash for CompressedData<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for CompressedData<'data>`

- `fn eq(self: &Self, other: &CompressedData<'data>) -> bool` — [`CompressedData`](../index.md)

##### `impl<'data> StructuralPartialEq for CompressedData<'data>`

### `ReadCache<R: ReadCacheOps>`

```rust
struct ReadCache<R: ReadCacheOps> {
    cache: core::cell::RefCell<ReadCacheInternal<R>>,
}
```

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

- `fn new(read: R) -> Self`

- `fn range(self: &Self, offset: u64, size: u64) -> ReadCacheRange<'_, R>` — [`ReadCacheRange`](#readcacherange)

- `fn clear(self: &mut Self)`

- `fn into_inner(self: Self) -> R`

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + ReadCacheOps> Debug for ReadCache<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReadCacheRange<'a, R: ReadCacheOps>`

```rust
struct ReadCacheRange<'a, R: ReadCacheOps> {
    r: &'a ReadCache<R>,
    offset: u64,
    size: u64,
}
```

An implementation of [`ReadRef`](#readref) for a range of data in a stream that
implements `Read + Seek`.

Shares an underlying [`ReadCache`](#readcache) with a lifetime of `'a`.

#### Trait Implementations

##### `impl<'a, R: ReadCacheOps> Clone for ReadCacheRange<'a, R>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, R: ReadCacheOps> Copy for ReadCacheRange<'a, R>`

##### `impl<'a, R: $crate::fmt::Debug + ReadCacheOps> Debug for ReadCacheRange<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, R: ReadCacheOps> ReadRef for ReadCacheRange<'a, R>`

- `fn len(self: Self) -> Result<u64, ()>`

- `fn read_bytes_at(self: Self, offset: u64, size: u64) -> Result<&'a [u8], ()>`

- `fn read_bytes_at_until(self: Self, range: Range<u64>, delimiter: u8) -> Result<&'a [u8], ()>`

### `Bytes<'data>`

```rust
struct Bytes<'data>(&'data [u8]);
```

A newtype for byte slices.

It has these important features:
- no methods that can panic, such as `Index`
- convenience methods for `Pod` types
- a useful `Debug` implementation

#### Implementations

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn skip(self: &mut Self, offset: usize) -> Result<(), ()>`

- `fn read_bytes(self: &mut Self, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](#bytes)

- `fn read_bytes_at(self: Self, offset: usize, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](#bytes)

- `fn read<T: Pod>(self: &mut Self) -> Result<&'data T, ()>`

- `fn read_at<T: Pod>(self: Self, offset: usize) -> Result<&'data T, ()>`

- `fn read_slice<T: Pod>(self: &mut Self, count: usize) -> Result<&'data [T], ()>`

- `fn read_slice_at<T: Pod>(self: Self, offset: usize, count: usize) -> Result<&'data [T], ()>`

- `fn read_string(self: &mut Self) -> Result<&'data [u8], ()>`

- `fn read_string_at(self: Self, offset: usize) -> Result<&'data [u8], ()>`

- `fn read_uleb128(self: &mut Self) -> Result<u64, ()>`

- `fn read_sleb128(self: &mut Self) -> Result<i64, ()>`

#### Trait Implementations

##### `impl<'data> Clone for Bytes<'data>`

- `fn clone(self: &Self) -> Bytes<'data>` — [`Bytes`](#bytes)

##### `impl<'data> Copy for Bytes<'data>`

##### `impl<'data> Debug for Bytes<'data>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data> Default for Bytes<'data>`

- `fn default() -> Bytes<'data>` — [`Bytes`](#bytes)

##### `impl<'data> Eq for Bytes<'data>`

##### `impl<'data> PartialEq for Bytes<'data>`

- `fn eq(self: &Self, other: &Bytes<'data>) -> bool` — [`Bytes`](#bytes)

##### `impl<'data> StructuralPartialEq for Bytes<'data>`

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

A table of zero-terminated strings.

This is used by most file formats for strings such as section names and symbol names.

#### Implementations

- `fn new(data: R, start: u64, end: u64) -> Self`

- `fn get(self: &Self, offset: u32) -> Result<&'data [u8], ()>`

#### Trait Implementations

##### `impl<'data, R> Clone for StringTable<'data, R>`

- `fn clone(self: &Self) -> StringTable<'data, R>` — [`StringTable`](#stringtable)

##### `impl<'data, R> Copy for StringTable<'data, R>`

##### `impl<'data, R> Debug for StringTable<'data, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R: ReadRef<'data>> Default for StringTable<'data, R>`

- `fn default() -> Self`

### `SegmentIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SegmentIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentIteratorInternal<'data, 'file, R>,
}
```

An iterator for the loadable segments in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SegmentIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SegmentIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SegmentIterator<'data, 'file, R>`

- `type Item = Segment<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Segment<'data, 'file, R: ReadRef<'data>>`

```rust
struct Segment<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentInternal<'data, 'file, R>,
}
```

A loadable segment in a [`File`](#file).

Most functionality is provided by the [`ObjectSegment`](#objectsegment) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Segment<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSegment for Segment<'data, 'file, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Segment<'data, 'file, R>`

### `SectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionIteratorInternal<'data, 'file, R>,
}
```

An iterator for the sections in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SectionIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SectionIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SectionIterator<'data, 'file, R>`

- `type Item = Section<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Section<'data, 'file, R: ReadRef<'data>>`

```rust
struct Section<'data, 'file, R: ReadRef<'data>> {
    inner: SectionInternal<'data, 'file, R>,
}
```

A section in a [`File`](#file).

Most functionality is provided by the [`ObjectSection`](#objectsection) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Section<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSection for Section<'data, 'file, R>`

- `type RelocationIterator = SectionRelocationIterator<'data, 'file, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../index.md), [`CompressedFileRange`](../index.md)

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>` — [`Result`](../index.md), [`CompressedData`](../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../index.md)

- `fn relocations(self: &Self) -> SectionRelocationIterator<'data, 'file, R>` — [`SectionRelocationIterator`](#sectionrelocationiterator)

- `fn relocation_map(self: &Self) -> Result<RelocationMap>` — [`Result`](../index.md), [`RelocationMap`](../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Section<'data, 'file, R>`

### `ComdatIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatIteratorInternal<'data, 'file, R>,
}
```

An iterator for the COMDAT section groups in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ComdatIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ComdatIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for ComdatIterator<'data, 'file, R>`

- `type Item = Comdat<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Comdat<'data, 'file, R: ReadRef<'data>>`

```rust
struct Comdat<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatInternal<'data, 'file, R>,
}
```

A COMDAT section group in a [`File`](#file).

Most functionality is provided by the [`ObjectComdat`](#objectcomdat) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Comdat<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectComdat for Comdat<'data, 'file, R>`

- `type SectionIterator = ComdatSectionIterator<'data, 'file, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../index.md)

- `fn sections(self: &Self) -> ComdatSectionIterator<'data, 'file, R>` — [`ComdatSectionIterator`](#comdatsectioniterator)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Comdat<'data, 'file, R>`

### `ComdatSectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatSectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatSectionIteratorInternal<'data, 'file, R>,
}
```

An iterator for the sections in a [`Comdat`](#comdat).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ComdatSectionIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for ComdatSectionIterator<'data, 'file, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `SymbolTable<'data, 'file, R>`

```rust
struct SymbolTable<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolTableInternal<'data, 'file, R>,
}
```

A symbol table in a [`File`](#file).

Most functionality is provided by the [`ObjectSymbolTable`](#objectsymboltable) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolTable<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSymbolTable for SymbolTable<'data, 'file, R>`

- `type Symbol = Symbol<'data, 'file, R>`

- `type SymbolIterator = SymbolIterator<'data, 'file, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](#objectsymboltable)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../index.md), [`Result`](../index.md), [`ObjectSymbolTable`](#objectsymboltable)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for SymbolTable<'data, 'file, R>`

### `SymbolIterator<'data, 'file, R>`

```rust
struct SymbolIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolIteratorInternal<'data, 'file, R>,
}
```

An iterator for the symbols in a [`SymbolTable`](coff/index.md).

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'file, R>`

- `type Item = Symbol<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Symbol<'data, 'file, R>`

```rust
struct Symbol<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolInternal<'data, 'file, R>,
}
```

An symbol in a [`SymbolTable`](coff/index.md).

Most functionality is provided by the [`ObjectSymbol`](#objectsymbol) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Symbol<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSymbol for Symbol<'data, 'file, R>`

- `fn index(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn kind(self: &Self) -> SymbolKind` — [`SymbolKind`](../index.md)

- `fn section(self: &Self) -> SymbolSection` — [`SymbolSection`](../index.md)

- `fn is_undefined(self: &Self) -> bool`

- `fn is_definition(self: &Self) -> bool`

- `fn is_common(self: &Self) -> bool`

- `fn is_weak(self: &Self) -> bool`

- `fn scope(self: &Self) -> SymbolScope` — [`SymbolScope`](../index.md)

- `fn is_global(self: &Self) -> bool`

- `fn is_local(self: &Self) -> bool`

- `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../index.md), [`SectionIndex`](../index.md), [`SymbolIndex`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Symbol<'data, 'file, R>`

### `DynamicRelocationIterator<'data, 'file, R>`

```rust
struct DynamicRelocationIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: DynamicRelocationIteratorInternal<'data, 'file, R>,
}
```

An iterator for the dynamic relocation entries in a [`File`](#file).

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for DynamicRelocationIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for DynamicRelocationIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for DynamicRelocationIterator<'data, 'file, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `SectionRelocationIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionRelocationIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionRelocationIteratorInternal<'data, 'file, R>,
}
```

An iterator for the relocation entries in a [`Section`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SectionRelocationIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SectionRelocationIterator<'data, 'file, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `NoDynamicRelocationIterator`

```rust
struct NoDynamicRelocationIterator;
```

An iterator for files that don't have dynamic relocations.

#### Trait Implementations

##### `impl Debug for NoDynamicRelocationIterator`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for NoDynamicRelocationIterator`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for NoDynamicRelocationIterator`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A file format kind.

#### Variants

- **`Archive`**

  A Unix archive.
  
  See `archive::ArchiveFile`.

- **`Coff`**

  A COFF object file.
  
  See `coff::CoffFile`.

- **`CoffBig`**

  A COFF bigobj object file.
  
  This supports a larger number of sections.
  
  See `coff::CoffBigFile`.

- **`CoffImport`**

  A Windows short import file.
  
  See `coff::ImportFile`.

- **`DyldCache`**

  A dyld cache file containing Mach-O images.
  
  See `macho::DyldCache`

- **`Elf32`**

  A 32-bit ELF file.
  
  See `elf::ElfFile32`.

- **`Elf64`**

  A 64-bit ELF file.
  
  See `elf::ElfFile64`.

- **`MachO32`**

  A 32-bit Mach-O file.
  
  See `macho::MachOFile32`.

- **`MachO64`**

  A 64-bit Mach-O file.
  
  See `macho::MachOFile64`.

- **`MachOFat32`**

  A 32-bit Mach-O fat binary.
  
  See `macho::MachOFatFile32`.

- **`MachOFat64`**

  A 64-bit Mach-O fat binary.
  
  See `macho::MachOFatFile64`.

- **`Pe32`**

  A 32-bit PE file.
  
  See `pe::PeFile32`.

- **`Pe64`**

  A 64-bit PE file.
  
  See `pe::PeFile64`.

- **`Xcoff32`**

  A 32-bit XCOFF file.
  
  See `xcoff::XcoffFile32`.

- **`Xcoff64`**

  A 64-bit XCOFF file.
  
  See `xcoff::XcoffFile64`.

#### Implementations

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<FileKind>` — [`Result`](../index.md), [`FileKind`](../index.md)

- `fn parse_at<'data, R: ReadRef<'data>>(data: R, offset: u64) -> Result<FileKind>` — [`Result`](../index.md), [`FileKind`](../index.md)

#### Trait Implementations

##### `impl Clone for FileKind`

- `fn clone(self: &Self) -> FileKind` — [`FileKind`](../index.md)

##### `impl Copy for FileKind`

##### `impl Debug for FileKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FileKind`

##### `impl Hash for FileKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for FileKind`

- `fn eq(self: &Self, other: &FileKind) -> bool` — [`FileKind`](../index.md)

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

- `fn clone(self: &Self) -> ObjectKind` — [`ObjectKind`](../index.md)

##### `impl Copy for ObjectKind`

##### `impl Debug for ObjectKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ObjectKind`

##### `impl Hash for ObjectKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ObjectKind`

- `fn eq(self: &Self, other: &ObjectKind) -> bool` — [`ObjectKind`](../index.md)

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

- `fn index(self: Self) -> Option<SectionIndex>` — [`SectionIndex`](../index.md)

#### Trait Implementations

##### `impl Clone for SymbolSection`

- `fn clone(self: &Self) -> SymbolSection` — [`SymbolSection`](../index.md)

##### `impl Copy for SymbolSection`

##### `impl Debug for SymbolSection`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SymbolSection`

##### `impl Hash for SymbolSection`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolSection`

- `fn eq(self: &Self, other: &SymbolSection) -> bool` — [`SymbolSection`](../index.md)

##### `impl StructuralPartialEq for SymbolSection`

### `RelocationTarget`

```rust
enum RelocationTarget {
    Symbol(SymbolIndex),
    Section(SectionIndex),
    Absolute,
}
```

The target referenced by a [`Relocation`](../macho/index.md).

#### Variants

- **`Symbol`**

  The target is a symbol.

- **`Section`**

  The target is a section.

- **`Absolute`**

  The offset is an absolute address.

#### Trait Implementations

##### `impl Clone for RelocationTarget`

- `fn clone(self: &Self) -> RelocationTarget` — [`RelocationTarget`](../index.md)

##### `impl Copy for RelocationTarget`

##### `impl Debug for RelocationTarget`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationTarget`

##### `impl Hash for RelocationTarget`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationTarget`

- `fn eq(self: &Self, other: &RelocationTarget) -> bool` — [`RelocationTarget`](../index.md)

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

- `fn clone(self: &Self) -> CompressionFormat` — [`CompressionFormat`](../index.md)

##### `impl Copy for CompressionFormat`

##### `impl Debug for CompressionFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for CompressionFormat`

##### `impl Hash for CompressionFormat`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for CompressionFormat`

- `fn eq(self: &Self, other: &CompressionFormat) -> bool` — [`CompressionFormat`](../index.md)

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

A CPU architecture.

#### Implementations

- `fn address_size(self: Self) -> Option<AddressSize>` — [`AddressSize`](../index.md)

#### Trait Implementations

##### `impl Clone for Architecture`

- `fn clone(self: &Self) -> Architecture` — [`Architecture`](../index.md)

##### `impl Copy for Architecture`

##### `impl Debug for Architecture`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Architecture`

##### `impl Hash for Architecture`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Architecture`

- `fn eq(self: &Self, other: &Architecture) -> bool` — [`Architecture`](../index.md)

##### `impl StructuralPartialEq for Architecture`

### `SubArchitecture`

```rust
enum SubArchitecture {
    Arm64E,
    Arm64EC,
}
```

A CPU sub-architecture.

#### Trait Implementations

##### `impl Clone for SubArchitecture`

- `fn clone(self: &Self) -> SubArchitecture` — [`SubArchitecture`](../index.md)

##### `impl Copy for SubArchitecture`

##### `impl Debug for SubArchitecture`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SubArchitecture`

##### `impl Hash for SubArchitecture`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SubArchitecture`

- `fn eq(self: &Self, other: &SubArchitecture) -> bool` — [`SubArchitecture`](../index.md)

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

The size of an address value for an architecture.

This may differ from the address size supported by the file format (such as for COFF).

#### Implementations

- `fn bytes(self: Self) -> u8`

#### Trait Implementations

##### `impl Clone for AddressSize`

- `fn clone(self: &Self) -> AddressSize` — [`AddressSize`](../index.md)

##### `impl Copy for AddressSize`

##### `impl Debug for AddressSize`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AddressSize`

##### `impl Hash for AddressSize`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for AddressSize`

- `fn eq(self: &Self, other: &AddressSize) -> bool` — [`AddressSize`](../index.md)

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

A binary file format.

#### Implementations

- `fn native_object() -> BinaryFormat` — [`BinaryFormat`](../index.md)

#### Trait Implementations

##### `impl Clone for BinaryFormat`

- `fn clone(self: &Self) -> BinaryFormat` — [`BinaryFormat`](../index.md)

##### `impl Copy for BinaryFormat`

##### `impl Debug for BinaryFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for BinaryFormat`

##### `impl Hash for BinaryFormat`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for BinaryFormat`

- `fn eq(self: &Self, other: &BinaryFormat) -> bool` — [`BinaryFormat`](../index.md)

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

- `fn is_bss(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for SectionKind`

- `fn clone(self: &Self) -> SectionKind` — [`SectionKind`](../index.md)

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SectionKind`

##### `impl Hash for SectionKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SectionKind`

- `fn eq(self: &Self, other: &SectionKind) -> bool` — [`SectionKind`](../index.md)

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

- `fn clone(self: &Self) -> ComdatKind` — [`ComdatKind`](../index.md)

##### `impl Copy for ComdatKind`

##### `impl Debug for ComdatKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ComdatKind`

##### `impl Hash for ComdatKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ComdatKind`

- `fn eq(self: &Self, other: &ComdatKind) -> bool` — [`ComdatKind`](../index.md)

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

- `fn clone(self: &Self) -> SymbolKind` — [`SymbolKind`](../index.md)

##### `impl Copy for SymbolKind`

##### `impl Debug for SymbolKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SymbolKind`

##### `impl Hash for SymbolKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolKind`

- `fn eq(self: &Self, other: &SymbolKind) -> bool` — [`SymbolKind`](../index.md)

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

- `fn clone(self: &Self) -> SymbolScope` — [`SymbolScope`](../index.md)

##### `impl Copy for SymbolScope`

##### `impl Debug for SymbolScope`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SymbolScope`

##### `impl Hash for SymbolScope`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolScope`

- `fn eq(self: &Self, other: &SymbolScope) -> bool` — [`SymbolScope`](../index.md)

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

- `fn clone(self: &Self) -> RelocationKind` — [`RelocationKind`](../index.md)

##### `impl Copy for RelocationKind`

##### `impl Debug for RelocationKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationKind`

##### `impl Hash for RelocationKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationKind`

- `fn eq(self: &Self, other: &RelocationKind) -> bool` — [`RelocationKind`](../index.md)

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

- `fn clone(self: &Self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md)

##### `impl Copy for RelocationEncoding`

##### `impl Debug for RelocationEncoding`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationEncoding`

##### `impl Hash for RelocationEncoding`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationEncoding`

- `fn eq(self: &Self, other: &RelocationEncoding) -> bool` — [`RelocationEncoding`](../index.md)

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

- `fn clone(self: &Self) -> FileFlags` — [`FileFlags`](../index.md)

##### `impl Copy for FileFlags`

##### `impl Debug for FileFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FileFlags`

##### `impl Hash for FileFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for FileFlags`

- `fn eq(self: &Self, other: &FileFlags) -> bool` — [`FileFlags`](../index.md)

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

- `fn clone(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../index.md)

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SegmentFlags`

##### `impl Hash for SegmentFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SegmentFlags`

- `fn eq(self: &Self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](../index.md)

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

- `fn clone(self: &Self) -> SectionFlags` — [`SectionFlags`](../index.md)

##### `impl Copy for SectionFlags`

##### `impl Debug for SectionFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SectionFlags`

##### `impl Hash for SectionFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SectionFlags`

- `fn eq(self: &Self, other: &SectionFlags) -> bool` — [`SectionFlags`](../index.md)

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

##### `impl<Section: $crate::clone::Clone, Symbol: $crate::clone::Clone> Clone for SymbolFlags<Section, Symbol>`

- `fn clone(self: &Self) -> SymbolFlags<Section, Symbol>` — [`SymbolFlags`](../index.md)

##### `impl<Section: $crate::marker::Copy, Symbol: $crate::marker::Copy> Copy for SymbolFlags<Section, Symbol>`

##### `impl<Section: $crate::fmt::Debug, Symbol: $crate::fmt::Debug> Debug for SymbolFlags<Section, Symbol>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Section: $crate::cmp::Eq, Symbol: $crate::cmp::Eq> Eq for SymbolFlags<Section, Symbol>`

##### `impl<Section: $crate::hash::Hash, Symbol: $crate::hash::Hash> Hash for SymbolFlags<Section, Symbol>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<Section: $crate::cmp::PartialEq, Symbol: $crate::cmp::PartialEq> PartialEq for SymbolFlags<Section, Symbol>`

- `fn eq(self: &Self, other: &SymbolFlags<Section, Symbol>) -> bool` — [`SymbolFlags`](../index.md)

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

- `fn clone(self: &Self) -> RelocationFlags` — [`RelocationFlags`](../index.md)

##### `impl Copy for RelocationFlags`

##### `impl Debug for RelocationFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationFlags`

##### `impl Hash for RelocationFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationFlags`

- `fn eq(self: &Self, other: &RelocationFlags) -> bool` — [`RelocationFlags`](../index.md)

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

An object file that can be any supported file format.

Most functionality is provided by the [`Object`](#object) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../index.md)

- `fn parse_dyld_cache_image<'cache, E: crate::Endian>(image: &macho::DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](macho/index.md), [`Result`](../index.md)

- `fn format(self: &Self) -> BinaryFormat` — [`BinaryFormat`](../index.md)

#### Trait Implementations

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>> Debug for File<'data, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R> Object for File<'data, R>`

- `type Segment = Segment<'data, 'file, R>`

- `type SegmentIterator = SegmentIterator<'data, 'file, R>`

- `type Section = Section<'data, 'file, R>`

- `type SectionIterator = SectionIterator<'data, 'file, R>`

- `type Comdat = Comdat<'data, 'file, R>`

- `type ComdatIterator = ComdatIterator<'data, 'file, R>`

- `type Symbol = Symbol<'data, 'file, R>`

- `type SymbolIterator = SymbolIterator<'data, 'file, R>`

- `type SymbolTable = SymbolTable<'data, 'file, R>`

- `type DynamicRelocationIterator = DynamicRelocationIterator<'data, 'file, R>`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../index.md)

- `fn segments(self: &Self) -> SegmentIterator<'data, '_, R>` — [`SegmentIterator`](#segmentiterator)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<Section<'data, 'file, R>>` — [`Section`](#section)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<Section<'data, '_, R>>` — [`SectionIndex`](../index.md), [`Result`](../index.md), [`Section`](#section)

- `fn sections(self: &Self) -> SectionIterator<'data, '_, R>` — [`SectionIterator`](#sectioniterator)

- `fn comdats(self: &Self) -> ComdatIterator<'data, '_, R>` — [`ComdatIterator`](#comdatiterator)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<Symbol<'data, '_, R>>` — [`SymbolIndex`](../index.md), [`Result`](../index.md), [`Symbol`](#symbol)

- `fn symbols(self: &Self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](#symboliterator)

- `fn symbol_table(self: &Self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](#symboltable)

- `fn dynamic_symbols(self: &Self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](#symboliterator)

- `fn dynamic_symbol_table(self: &Self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](#symboltable)

- `fn dynamic_relocations(self: &Self) -> Option<DynamicRelocationIterator<'data, '_, R>>` — [`DynamicRelocationIterator`](#dynamicrelocationiterator)

- `fn symbol_map(self: &Self) -> SymbolMap<SymbolMapName<'data>>` — [`SymbolMap`](../index.md), [`SymbolMapName`](../index.md)

- `fn object_map(self: &Self) -> ObjectMap<'data>` — [`ObjectMap`](../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../index.md), [`Import`](../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../index.md), [`Export`](../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>` — [`Result`](../index.md)

- `fn build_id(self: &Self) -> Result<Option<&'data [u8]>>` — [`Result`](../index.md)

- `fn gnu_debuglink(self: &Self) -> Result<Option<(&'data [u8], u32)>>` — [`Result`](../index.md)

- `fn gnu_debugaltlink(self: &Self) -> Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../index.md)

- `fn pdb_info(self: &Self) -> Result<Option<CodeView<'_>>>` — [`Result`](../index.md), [`CodeView`](../index.md)

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../index.md)

##### `impl<'data, R: ReadRef<'data>> Sealed for File<'data, R>`

## Traits

### `SymbolMapEntry`

```rust
trait SymbolMapEntry { ... }
```

An entry in a [`SymbolMap`](../index.md).

#### Required Methods

- `fn address(self: &Self) -> u64`

  The symbol address.

### `ReadRef<'a>`

```rust
trait ReadRef<'a>: Clone + Copy { ... }
```

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

- `fn len(self: Self) -> result::Result<u64, ()>`

  The total size of the block of data.

- `fn read_bytes_at(self: Self, offset: u64, size: u64) -> result::Result<&'a [u8], ()>`

  Get a reference to a `u8` slice at the given offset.

- `fn read_bytes_at_until(self: Self, range: Range<u64>, delimiter: u8) -> result::Result<&'a [u8], ()>`

  Get a reference to a delimited `u8` slice which starts at range.start.

- `fn read_bytes(self: Self, offset: &mut u64, size: u64) -> result::Result<&'a [u8], ()>`

  Get a reference to a `u8` slice at the given offset, and update the offset.

- `fn read<T: Pod>(self: Self, offset: &mut u64) -> result::Result<&'a T, ()>`

  Get a reference to a `Pod` type at the given offset, and update the offset.

- `fn read_at<T: Pod>(self: Self, offset: u64) -> result::Result<&'a T, ()>`

  Get a reference to a `Pod` type at the given offset.

- `fn read_slice<T: Pod>(self: Self, offset: &mut u64, count: usize) -> result::Result<&'a [T], ()>`

  Get a reference to a slice of a `Pod` type at the given offset, and update the offset.

- `fn read_slice_at<T: Pod>(self: Self, offset: u64, count: usize) -> result::Result<&'a [T], ()>`

  Get a reference to a slice of a `Pod` type at the given offset.

### `ReadCacheOps`

```rust
trait ReadCacheOps { ... }
```

Operations required to implement [`ReadCache`](#readcache).

This is a subset of the `Read` and `Seek` traits.
A blanket implementation is provided for all types that implement
`Read + Seek`.

#### Required Methods

- `fn len(self: &mut Self) -> Result<u64, ()>`

  Return the length of the stream.

- `fn seek(self: &mut Self, pos: u64) -> Result<u64, ()>`

  Seek to the given position in the stream.

- `fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize, ()>`

  Read up to `buf.len()` bytes into `buf`.

- `fn read_exact(self: &mut Self, buf: &mut [u8]) -> Result<(), ()>`

  Read exactly `buf.len()` bytes into `buf`.

### `Object<'data>`

```rust
trait Object<'data>: read::private::Sealed { ... }
```

An object file.

This is the primary trait for the unified read API.

#### Required Methods

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

- `fn architecture(self: &Self) -> Architecture`

  Get the architecture type of the file.

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>`

  Get the sub-architecture type of the file if known.

- `fn endianness(self: &Self) -> Endianness`

  Get the endianness of the file.

- `fn is_little_endian(self: &Self) -> bool`

  Return true if the file is little endian, false if it is big endian.

- `fn is_64(self: &Self) -> bool`

  Return true if the file can contain 64-bit addresses.

- `fn kind(self: &Self) -> ObjectKind`

  Return the kind of this object.

- `fn segments(self: &Self) -> <Self as >::SegmentIterator`

  Get an iterator for the loadable segments in the file.

- `fn section_by_name(self: &Self, section_name: &str) -> Option<<Self as >::Section>`

  Get the section named `section_name`, if such a section exists.

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<<Self as >::Section>`

  Like `Self::section_by_name`, but allows names that are not UTF-8.

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<<Self as >::Section>`

  Get the section at the given index.

- `fn sections(self: &Self) -> <Self as >::SectionIterator`

  Get an iterator for the sections in the file.

- `fn comdats(self: &Self) -> <Self as >::ComdatIterator`

  Get an iterator for the COMDAT section groups in the file.

- `fn symbol_table(self: &Self) -> Option<<Self as >::SymbolTable>`

  Get the debugging symbol table, if any.

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the debugging symbol at the given index.

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator`

  Get an iterator for the debugging symbols in the file.

- `fn symbol_by_name<'file>(self: &'file Self, symbol_name: &str) -> Option<<Self as >::Symbol>`

  Get the symbol named `symbol_name`, if the symbol exists.

- `fn symbol_by_name_bytes<'file>(self: &'file Self, symbol_name: &[u8]) -> Option<<Self as >::Symbol>`

  Like `Self::symbol_by_name`, but allows names that are not UTF-8.

- `fn dynamic_symbol_table(self: &Self) -> Option<<Self as >::SymbolTable>`

  Get the dynamic linking symbol table, if any.

- `fn dynamic_symbols(self: &Self) -> <Self as >::SymbolIterator`

  Get an iterator for the dynamic linking symbols in the file.

- `fn dynamic_relocations(self: &Self) -> Option<<Self as >::DynamicRelocationIterator>`

  Get the dynamic relocations for this file.

- `fn symbol_map(self: &Self) -> SymbolMap<SymbolMapName<'data>>`

  Construct a map from addresses to symbol names.

- `fn object_map(self: &Self) -> ObjectMap<'data>`

  Construct a map from addresses to symbol names and object file names.

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>`

  Get the imported symbols.

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>`

  Get the exported symbols that expose both a name and an address.

- `fn has_debug_symbols(self: &Self) -> bool`

  Return true if the file contains DWARF debug information sections, false if not.

- `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>`

  The UUID from a Mach-O [`LC_UUID`](crate::macho::LC_UUID) load command.

- `fn build_id(self: &Self) -> Result<Option<&'data [u8]>>`

  The build ID from an ELF [`NT_GNU_BUILD_ID`](crate::elf::NT_GNU_BUILD_ID) note.

- `fn gnu_debuglink(self: &Self) -> Result<Option<(&'data [u8], u32)>>`

  The filename and CRC from a `.gnu_debuglink` section.

- `fn gnu_debugaltlink(self: &Self) -> Result<Option<(&'data [u8], &'data [u8])>>`

  The filename and build ID from a `.gnu_debugaltlink` section.

- `fn pdb_info(self: &Self) -> Result<Option<CodeView<'_>>>`

  The filename and GUID from the PE CodeView section.

- `fn relative_address_base(self: &Self) -> u64`

  Get the base address used for relative virtual addresses.

- `fn entry(self: &Self) -> u64`

  Get the virtual address of the entry point of the binary.

- `fn flags(self: &Self) -> FileFlags`

  File flags that are specific to each file format.

### `ObjectSegment<'data>`

```rust
trait ObjectSegment<'data>: read::private::Sealed { ... }
```

A loadable segment in an [`Object`](#object).

This trait is part of the unified read API.

#### Required Methods

- `fn address(self: &Self) -> u64`

  Returns the virtual address of the segment.

- `fn size(self: &Self) -> u64`

  Returns the size of the segment in memory.

- `fn align(self: &Self) -> u64`

  Returns the alignment of the segment in memory.

- `fn file_range(self: &Self) -> (u64, u64)`

  Returns the offset and size of the segment in the file.

- `fn data(self: &Self) -> Result<&'data [u8]>`

  Returns a reference to the file contents of the segment.

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the segment data in the given range.

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>`

  Returns the name of the segment.

- `fn name(self: &Self) -> Result<Option<&str>>`

  Returns the name of the segment.

- `fn flags(self: &Self) -> SegmentFlags`

  Return the flags of segment.

### `ObjectSection<'data>`

```rust
trait ObjectSection<'data>: read::private::Sealed { ... }
```

A section in an [`Object`](#object).

This trait is part of the unified read API.

#### Required Methods

- `type RelocationIterator: 1`

- `fn index(self: &Self) -> SectionIndex`

  Returns the section index.

- `fn address(self: &Self) -> u64`

  Returns the address of the section.

- `fn size(self: &Self) -> u64`

  Returns the size of the section in memory.

- `fn align(self: &Self) -> u64`

  Returns the alignment of the section in memory.

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

  Returns offset and size of on-disk segment (if any).

- `fn data(self: &Self) -> Result<&'data [u8]>`

  Returns the raw contents of the section.

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the raw contents of the section data in the given range.

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>`

  Returns the potentially compressed file range of the section,

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>`

  Returns the potentially compressed contents of the section,

- `fn uncompressed_data(self: &Self) -> Result<Cow<'data, [u8]>>`

  Returns the uncompressed contents of the section.

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>`

  Returns the name of the section.

- `fn name(self: &Self) -> Result<&'data str>`

  Returns the name of the section.

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>`

  Returns the name of the segment for this section.

- `fn segment_name(self: &Self) -> Result<Option<&str>>`

  Returns the name of the segment for this section.

- `fn kind(self: &Self) -> SectionKind`

  Return the kind of this section.

- `fn relocations(self: &Self) -> <Self as >::RelocationIterator`

  Get the relocations for this section.

- `fn relocation_map(self: &Self) -> Result<RelocationMap>`

  Construct a relocation map for this section.

- `fn flags(self: &Self) -> SectionFlags`

  Section flags that are specific to each file format.

### `ObjectComdat<'data>`

```rust
trait ObjectComdat<'data>: read::private::Sealed { ... }
```

A COMDAT section group in an [`Object`](#object).

This trait is part of the unified read API.

#### Required Methods

- `type SectionIterator: 1`

- `fn kind(self: &Self) -> ComdatKind`

  Returns the COMDAT selection kind.

- `fn symbol(self: &Self) -> SymbolIndex`

  Returns the index of the symbol used for the name of COMDAT section group.

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>`

  Returns the name of the COMDAT section group.

- `fn name(self: &Self) -> Result<&'data str>`

  Returns the name of the COMDAT section group.

- `fn sections(self: &Self) -> <Self as >::SectionIterator`

  Get the sections in this section group.

### `ObjectSymbolTable<'data>`

```rust
trait ObjectSymbolTable<'data>: read::private::Sealed { ... }
```

A symbol table in an [`Object`](#object).

This trait is part of the unified read API.

#### Required Methods

- `type Symbol: 1`

- `type SymbolIterator: 1`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator`

  Get an iterator for the symbols in the table.

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the symbol at the given index.

### `ObjectSymbol<'data>`

```rust
trait ObjectSymbol<'data>: read::private::Sealed { ... }
```

A symbol table entry in an [`Object`](#object).

This trait is part of the unified read API.

#### Required Methods

- `fn index(self: &Self) -> SymbolIndex`

  The index of the symbol.

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>`

  The name of the symbol.

- `fn name(self: &Self) -> Result<&'data str>`

  The name of the symbol.

- `fn address(self: &Self) -> u64`

  The address of the symbol. May be zero if the address is unknown.

- `fn size(self: &Self) -> u64`

  The size of the symbol. May be zero if the size is unknown.

- `fn kind(self: &Self) -> SymbolKind`

  Return the kind of this symbol.

- `fn section(self: &Self) -> SymbolSection`

  Returns the section where the symbol is defined.

- `fn section_index(self: &Self) -> Option<SectionIndex>`

  Returns the section index for the section containing this symbol.

- `fn is_undefined(self: &Self) -> bool`

  Return true if the symbol is undefined.

- `fn is_definition(self: &Self) -> bool`

  Return true if the symbol is a definition of a function or data object

- `fn is_common(self: &Self) -> bool`

  Return true if the symbol is common data.

- `fn is_weak(self: &Self) -> bool`

  Return true if the symbol is weak.

- `fn scope(self: &Self) -> SymbolScope`

  Returns the symbol scope.

- `fn is_global(self: &Self) -> bool`

  Return true if the symbol visible outside of the compilation unit.

- `fn is_local(self: &Self) -> bool`

  Return true if the symbol is only visible within the compilation unit.

- `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>`

  Symbol flags that are specific to each file format.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result type used within the read module.

### `NativeFile<'data, R>`

```rust
type NativeFile<'data, R> = elf::ElfFile64<'data, crate::endian::Endianness, R>;
```

The native executable file for the target platform.

