*[object](../index.md) / [read](index.md)*

---

# Module `read`

Interface for reading object files.

## Unified read API

The [`Object`](traits/index.md) trait provides a unified read API for accessing common features of
object files, such as sections and symbols. There is an implementation of this
trait for [`File`](any/index.md), which allows reading any file format, as well as implementations
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

##### `impl Clone`

- `fn clone(self: &Self) -> Error` — [`Error`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool` — [`Error`](../../read/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `SectionIndex`

```rust
struct SectionIndex(usize);
```

The index used to identify a section in a file.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> SectionIndex` — [`SectionIndex`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SectionIndex) -> bool` — [`SectionIndex`](../../read/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `SymbolIndex`

```rust
struct SymbolIndex(usize);
```

The index used to identify a symbol in a symbol table.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SymbolIndex) -> bool` — [`SymbolIndex`](../../read/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `SymbolMap<T: SymbolMapEntry>`

```rust
struct SymbolMap<T: SymbolMapEntry> {
    symbols: alloc::vec::Vec<T>,
}
```

A map from addresses to symbol information.

The symbol information depends on the chosen entry type, such as [`SymbolMapName`](#symbolmapname).

Returned by `Object::symbol_map`.

#### Implementations

- `fn new(symbols: Vec<T>) -> Self`

- `fn get(self: &Self, address: u64) -> Option<&T>`

- `fn symbols(self: &Self) -> &[T]`

#### Trait Implementations

##### `impl Clone<T: $crate::clone::Clone + SymbolMapEntry>`

- `fn clone(self: &Self) -> SymbolMap<T>` — [`SymbolMap`](../../read/index.md)

##### `impl Debug<T: $crate::fmt::Debug + SymbolMapEntry>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<T: $crate::default::Default + SymbolMapEntry>`

- `fn default() -> SymbolMap<T>` — [`SymbolMap`](../../read/index.md)

### `SymbolMapName<'data>`

```rust
struct SymbolMapName<'data> {
    address: u64,
    name: &'data str,
}
```

The type used for entries in a [`SymbolMap`](#symbolmap) that maps from addresses to names.

#### Implementations

- `fn new(address: u64, name: &'data str) -> Self`

- `fn address(self: &Self) -> u64`

- `fn name(self: &Self) -> &'data str`

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> SymbolMapName<'data>` — [`SymbolMapName`](../../read/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &SymbolMapName<'data>) -> bool` — [`SymbolMapName`](../../read/index.md)

##### `impl StructuralPartialEq<'data>`

##### `impl SymbolMapEntry<'data>`

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

- `fn get(self: &Self, address: u64) -> Option<&ObjectMapEntry<'data>>` — [`ObjectMapEntry`](../../read/index.md)

- `fn symbols(self: &Self) -> &[ObjectMapEntry<'data>]` — [`ObjectMapEntry`](../../read/index.md)

- `fn objects(self: &Self) -> &[ObjectMapFile<'data>]` — [`ObjectMapFile`](../../read/index.md)

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ObjectMap<'data>` — [`ObjectMap`](../../read/index.md)

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'data>`

- `fn default() -> ObjectMap<'data>` — [`ObjectMap`](../../read/index.md)

### `ObjectMapEntry<'data>`

```rust
struct ObjectMapEntry<'data> {
    address: u64,
    size: u64,
    name: &'data [u8],
    object: usize,
}
```

A symbol in an [`ObjectMap`](#objectmap).

#### Implementations

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn name(self: &Self) -> &'data [u8]`

- `fn object_index(self: &Self) -> usize`

- `fn object<'a>(self: &Self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>` — [`ObjectMap`](../../read/index.md), [`ObjectMapFile`](../../read/index.md)

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../../read/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'data>`

- `fn default() -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](../../read/index.md)

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &ObjectMapEntry<'data>) -> bool` — [`ObjectMapEntry`](../../read/index.md)

##### `impl StructuralPartialEq<'data>`

##### `impl SymbolMapEntry<'data>`

- `fn address(self: &Self) -> u64`

### `ObjectMapFile<'data>`

```rust
struct ObjectMapFile<'data> {
    path: &'data [u8],
    member: Option<&'data [u8]>,
}
```

An object file name in an [`ObjectMap`](#objectmap).

#### Implementations

- `fn new(path: &'data [u8], member: Option<&'data [u8]>) -> Self`

- `fn path(self: &Self) -> &'data [u8]`

- `fn member(self: &Self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ObjectMapFile<'data>` — [`ObjectMapFile`](../../read/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &ObjectMapFile<'data>) -> bool` — [`ObjectMapFile`](../../read/index.md)

##### `impl StructuralPartialEq<'data>`

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> Import<'data>` — [`Import`](../../read/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'data>`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &Import<'data>) -> bool` — [`Import`](../../read/index.md)

##### `impl StructuralPartialEq<'data>`

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> Export<'data>` — [`Export`](../../read/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'data>`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &Export<'data>) -> bool` — [`Export`](../../read/index.md)

##### `impl StructuralPartialEq<'data>`

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> CodeView<'data>` — [`CodeView`](../../read/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'data>`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &CodeView<'data>) -> bool` — [`CodeView`](../../read/index.md)

##### `impl StructuralPartialEq<'data>`

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

- `fn kind(self: &Self) -> RelocationKind` — [`RelocationKind`](../../common/index.md)

- `fn encoding(self: &Self) -> RelocationEncoding` — [`RelocationEncoding`](../../common/index.md)

- `fn size(self: &Self) -> u8`

- `fn target(self: &Self) -> RelocationTarget` — [`RelocationTarget`](../../read/index.md)

- `fn addend(self: &Self) -> i64`

- `fn set_addend(self: &mut Self, addend: i64)`

- `fn has_implicit_addend(self: &Self) -> bool`

- `fn flags(self: &Self) -> RelocationFlags` — [`RelocationFlags`](../../common/index.md)

#### Trait Implementations

##### `impl Debug`

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

- `fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>` — [`Object`](../../read/traits/index.md), [`Result`](../../read/index.md)

- `fn add<'data: 'file, 'file, T>(self: &mut Self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>` — [`Relocation`](../../read/index.md), [`Result`](../../read/index.md)

- `fn relocate(self: &Self, offset: u64, value: u64) -> u64`

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> RelocationMap` — [`RelocationMap`](../../read/index.md)

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

- `fn data<'data, R: ReadRef<'data>>(self: Self, file: R) -> Result<CompressedData<'data>>` — [`Result`](../../read/index.md), [`CompressedData`](../../read/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> CompressedFileRange` — [`CompressedFileRange`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CompressedFileRange) -> bool` — [`CompressedFileRange`](../../read/index.md)

##### `impl StructuralPartialEq`

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

- `fn decompress(self: Self) -> Result<Cow<'data, [u8]>>` — [`Result`](../../read/index.md)

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> CompressedData<'data>` — [`CompressedData`](../../read/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &CompressedData<'data>) -> bool` — [`CompressedData`](../../read/index.md)

##### `impl StructuralPartialEq<'data>`

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

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<FileKind>` — [`Result`](../../read/index.md), [`FileKind`](../../read/index.md)

- `fn parse_at<'data, R: ReadRef<'data>>(data: R, offset: u64) -> Result<FileKind>` — [`Result`](../../read/index.md), [`FileKind`](../../read/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> FileKind` — [`FileKind`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &FileKind) -> bool` — [`FileKind`](../../read/index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> ObjectKind` — [`ObjectKind`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ObjectKind) -> bool` — [`ObjectKind`](../../read/index.md)

##### `impl StructuralPartialEq`

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

The section where an [`ObjectSymbol`](traits/index.md) is defined.

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

- `fn index(self: Self) -> Option<SectionIndex>` — [`SectionIndex`](../../read/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> SymbolSection` — [`SymbolSection`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SymbolSection) -> bool` — [`SymbolSection`](../../read/index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> RelocationTarget` — [`RelocationTarget`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RelocationTarget) -> bool` — [`RelocationTarget`](../../read/index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> CompressionFormat` — [`CompressionFormat`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CompressionFormat) -> bool` — [`CompressionFormat`](../../read/index.md)

##### `impl StructuralPartialEq`

## Traits

### `SymbolMapEntry`

```rust
trait SymbolMapEntry { ... }
```

An entry in a [`SymbolMap`](#symbolmap).

#### Required Methods

- `fn address(self: &Self) -> u64`

  The symbol address.

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

