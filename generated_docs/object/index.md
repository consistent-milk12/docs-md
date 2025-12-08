# Crate `object`

The `object` crate provides a unified interface to working with object files
across platforms. It supports reading relocatable object files and executable files,
and writing relocatable object files and some executable files.

## Raw struct definitions

Raw structs are defined for: [ELF](elf), [Mach-O](macho), [PE/COFF](pe),
[XCOFF](xcoff), [archive](#archive).
Types and traits for zerocopy support are defined in the [`pod`](pod/index.md) and [`endian`](endian/index.md) modules.

## Unified read API

The [`read`](read/index.md) module provides a unified read API using the [`read::Object`](read/index.md) trait.
There is an implementation of this trait for [`read::File`](read/index.md), which allows reading any
file format, as well as implementations for each file format.

## Low level read API

The [`read#modules`](read/index.md) submodules define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

## Unified write API

The `mod@write` module provides a unified write API for relocatable object files
using `write::Object`. This does not support writing executable files.

## Low level write API

The `mod@write#modules` submodules define helpers for writing the raw structs.

## Build API

The `mod@build` submodules define helpers for building object files, either from
scratch or by modifying existing files.

## Shared definitions

The crate provides a number of definitions that are used by both the read and write
APIs. These are defined at the top level module, but none of these are the main entry
points of the crate.

## Modules

- [`common`](common/index.md) - 
- [`endian`](endian/index.md) - Types for compile-time and run-time endianness.
- [`pod`](pod/index.md) - Tools for converting file format structures to and from bytes.
- [`read`](read/index.md) - Interface for reading object files.
- [`archive`](archive/index.md) - Archive definitions.
- [`elf`](elf/index.md) - ELF definitions.
- [`macho`](macho/index.md) - Mach-O definitions.
- [`pe`](pe/index.md) - PE/COFF definitions.
- [`xcoff`](xcoff/index.md) - XCOFF definitions
- [`read_ref`](read_ref/index.md) - 
- [`read_cache`](read_cache/index.md) - 
- [`util`](util/index.md) - 
- [`gnu_compression`](gnu_compression/index.md) - 
- [`any`](any/index.md) - 
- [`archive`](archive/index.md) - Support for archive files.
- [`coff`](coff/index.md) - Support for reading Windows COFF files.
- [`elf`](elf/index.md) - Support for reading ELF files.
- [`macho`](macho/index.md) - Support for reading Mach-O files.
- [`pe`](pe/index.md) - Support for reading PE files.
- [`xcoff`](xcoff/index.md) - Support for reading AIX XCOFF files.
- [`traits`](traits/index.md) - 
- [`private`](private/index.md) - 

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

Compile-time little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- `fn clone(self: &Self) -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LittleEndian`

- `fn default() -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Endian for LittleEndian`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for LittleEndian`

- `fn eq(self: &Self, other: &LittleEndian) -> bool` — [`LittleEndian`](#littleendian)

##### `impl StructuralPartialEq for LittleEndian`

### `BigEndian`

```rust
struct BigEndian;
```

Compile-time big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- `fn clone(self: &Self) -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BigEndian`

- `fn default() -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Endian for BigEndian`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- `fn eq(self: &Self, other: &BigEndian) -> bool` — [`BigEndian`](#bigendian)

##### `impl StructuralPartialEq for BigEndian`

### `U16Bytes<E: Endian>`

```rust
struct U16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `u16` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 2]) -> Self`

- `fn new(e: E, n: u16) -> Self`

- `fn get(self: Self, e: E) -> u16`

- `fn set(self: &mut Self, e: E, n: u16)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for U16Bytes<E>`

- `fn clone(self: &Self) -> U16Bytes<E>` — [`U16Bytes`](#u16bytes)

##### `impl<E: $crate::marker::Copy + Endian> Copy for U16Bytes<E>`

##### `impl<E: Endian> Debug for U16Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for U16Bytes<E>`

- `fn default() -> U16Bytes<E>` — [`U16Bytes`](#u16bytes)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for U16Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for U16Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for U16Bytes<E>`

- `fn cmp(self: &Self, other: &U16Bytes<E>) -> $crate::cmp::Ordering` — [`U16Bytes`](#u16bytes)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for U16Bytes<E>`

- `fn eq(self: &Self, other: &U16Bytes<E>) -> bool` — [`U16Bytes`](#u16bytes)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for U16Bytes<E>`

- `fn partial_cmp(self: &Self, other: &U16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U16Bytes`](#u16bytes)

##### `impl<E: Endian> Pod for U16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U16Bytes<E>`

### `U32Bytes<E: Endian>`

```rust
struct U32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `u32` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 4]) -> Self`

- `fn new(e: E, n: u32) -> Self`

- `fn get(self: Self, e: E) -> u32`

- `fn set(self: &mut Self, e: E, n: u32)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for U32Bytes<E>`

- `fn clone(self: &Self) -> U32Bytes<E>` — [`U32Bytes`](#u32bytes)

##### `impl<E: $crate::marker::Copy + Endian> Copy for U32Bytes<E>`

##### `impl<E: Endian> Debug for U32Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for U32Bytes<E>`

- `fn default() -> U32Bytes<E>` — [`U32Bytes`](#u32bytes)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for U32Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for U32Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for U32Bytes<E>`

- `fn cmp(self: &Self, other: &U32Bytes<E>) -> $crate::cmp::Ordering` — [`U32Bytes`](#u32bytes)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for U32Bytes<E>`

- `fn eq(self: &Self, other: &U32Bytes<E>) -> bool` — [`U32Bytes`](#u32bytes)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for U32Bytes<E>`

- `fn partial_cmp(self: &Self, other: &U32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U32Bytes`](#u32bytes)

##### `impl<E: Endian> Pod for U32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U32Bytes<E>`

### `U64Bytes<E: Endian>`

```rust
struct U64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `u64` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 8]) -> Self`

- `fn new(e: E, n: u64) -> Self`

- `fn get(self: Self, e: E) -> u64`

- `fn set(self: &mut Self, e: E, n: u64)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for U64Bytes<E>`

- `fn clone(self: &Self) -> U64Bytes<E>` — [`U64Bytes`](#u64bytes)

##### `impl<E: $crate::marker::Copy + Endian> Copy for U64Bytes<E>`

##### `impl<E: Endian> Debug for U64Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for U64Bytes<E>`

- `fn default() -> U64Bytes<E>` — [`U64Bytes`](#u64bytes)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for U64Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for U64Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for U64Bytes<E>`

- `fn cmp(self: &Self, other: &U64Bytes<E>) -> $crate::cmp::Ordering` — [`U64Bytes`](#u64bytes)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for U64Bytes<E>`

- `fn eq(self: &Self, other: &U64Bytes<E>) -> bool` — [`U64Bytes`](#u64bytes)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for U64Bytes<E>`

- `fn partial_cmp(self: &Self, other: &U64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U64Bytes`](#u64bytes)

##### `impl<E: Endian> Pod for U64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U64Bytes<E>`

### `I16Bytes<E: Endian>`

```rust
struct I16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `i16` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 2]) -> Self`

- `fn new(e: E, n: i16) -> Self`

- `fn get(self: Self, e: E) -> i16`

- `fn set(self: &mut Self, e: E, n: i16)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for I16Bytes<E>`

- `fn clone(self: &Self) -> I16Bytes<E>` — [`I16Bytes`](#i16bytes)

##### `impl<E: $crate::marker::Copy + Endian> Copy for I16Bytes<E>`

##### `impl<E: Endian> Debug for I16Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for I16Bytes<E>`

- `fn default() -> I16Bytes<E>` — [`I16Bytes`](#i16bytes)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for I16Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for I16Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for I16Bytes<E>`

- `fn cmp(self: &Self, other: &I16Bytes<E>) -> $crate::cmp::Ordering` — [`I16Bytes`](#i16bytes)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for I16Bytes<E>`

- `fn eq(self: &Self, other: &I16Bytes<E>) -> bool` — [`I16Bytes`](#i16bytes)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for I16Bytes<E>`

- `fn partial_cmp(self: &Self, other: &I16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I16Bytes`](#i16bytes)

##### `impl<E: Endian> Pod for I16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I16Bytes<E>`

### `I32Bytes<E: Endian>`

```rust
struct I32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `i32` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 4]) -> Self`

- `fn new(e: E, n: i32) -> Self`

- `fn get(self: Self, e: E) -> i32`

- `fn set(self: &mut Self, e: E, n: i32)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for I32Bytes<E>`

- `fn clone(self: &Self) -> I32Bytes<E>` — [`I32Bytes`](#i32bytes)

##### `impl<E: $crate::marker::Copy + Endian> Copy for I32Bytes<E>`

##### `impl<E: Endian> Debug for I32Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for I32Bytes<E>`

- `fn default() -> I32Bytes<E>` — [`I32Bytes`](#i32bytes)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for I32Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for I32Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for I32Bytes<E>`

- `fn cmp(self: &Self, other: &I32Bytes<E>) -> $crate::cmp::Ordering` — [`I32Bytes`](#i32bytes)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for I32Bytes<E>`

- `fn eq(self: &Self, other: &I32Bytes<E>) -> bool` — [`I32Bytes`](#i32bytes)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for I32Bytes<E>`

- `fn partial_cmp(self: &Self, other: &I32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I32Bytes`](#i32bytes)

##### `impl<E: Endian> Pod for I32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I32Bytes<E>`

### `I64Bytes<E: Endian>`

```rust
struct I64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `i64` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 8]) -> Self`

- `fn new(e: E, n: i64) -> Self`

- `fn get(self: Self, e: E) -> i64`

- `fn set(self: &mut Self, e: E, n: i64)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for I64Bytes<E>`

- `fn clone(self: &Self) -> I64Bytes<E>` — [`I64Bytes`](#i64bytes)

##### `impl<E: $crate::marker::Copy + Endian> Copy for I64Bytes<E>`

##### `impl<E: Endian> Debug for I64Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for I64Bytes<E>`

- `fn default() -> I64Bytes<E>` — [`I64Bytes`](#i64bytes)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for I64Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for I64Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for I64Bytes<E>`

- `fn cmp(self: &Self, other: &I64Bytes<E>) -> $crate::cmp::Ordering` — [`I64Bytes`](#i64bytes)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for I64Bytes<E>`

- `fn eq(self: &Self, other: &I64Bytes<E>) -> bool` — [`I64Bytes`](#i64bytes)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for I64Bytes<E>`

- `fn partial_cmp(self: &Self, other: &I64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I64Bytes`](#i64bytes)

##### `impl<E: Endian> Pod for I64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I64Bytes<E>`

### `Error`

```rust
struct Error(&'static str);
```

The error type used within the read module.

#### Trait Implementations

##### `impl Clone for Error`

- `fn clone(self: &Self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- `fn eq(self: &Self, other: &Error) -> bool` — [`Error`](#error)

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

- `fn clone(self: &Self) -> SectionIndex` — [`SectionIndex`](#sectionindex)

##### `impl Copy for SectionIndex`

##### `impl Debug for SectionIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SectionIndex`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionIndex`

##### `impl Hash for SectionIndex`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SectionIndex`

- `fn eq(self: &Self, other: &SectionIndex) -> bool` — [`SectionIndex`](#sectionindex)

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

- `fn clone(self: &Self) -> SymbolIndex` — [`SymbolIndex`](#symbolindex)

##### `impl Copy for SymbolIndex`

##### `impl Debug for SymbolIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SymbolIndex`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolIndex`

##### `impl Hash for SymbolIndex`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolIndex`

- `fn eq(self: &Self, other: &SymbolIndex) -> bool` — [`SymbolIndex`](#symbolindex)

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

The symbol information depends on the chosen entry type, such as [`SymbolMapName`](#symbolmapname).

Returned by `Object::symbol_map`.

#### Implementations

- `fn new(symbols: Vec<T>) -> Self`

- `fn get(self: &Self, address: u64) -> Option<&T>`

- `fn symbols(self: &Self) -> &[T]`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + SymbolMapEntry> Clone for SymbolMap<T>`

- `fn clone(self: &Self) -> SymbolMap<T>` — [`SymbolMap`](#symbolmap)

##### `impl<T: $crate::fmt::Debug + SymbolMapEntry> Debug for SymbolMap<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::default::Default + SymbolMapEntry> Default for SymbolMap<T>`

- `fn default() -> SymbolMap<T>` — [`SymbolMap`](#symbolmap)

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

##### `impl<'data> Clone for SymbolMapName<'data>`

- `fn clone(self: &Self) -> SymbolMapName<'data>` — [`SymbolMapName`](#symbolmapname)

##### `impl<'data> Copy for SymbolMapName<'data>`

##### `impl<'data> Debug for SymbolMapName<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for SymbolMapName<'data>`

##### `impl<'data> Hash for SymbolMapName<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for SymbolMapName<'data>`

- `fn eq(self: &Self, other: &SymbolMapName<'data>) -> bool` — [`SymbolMapName`](#symbolmapname)

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

- `fn get(self: &Self, address: u64) -> Option<&ObjectMapEntry<'data>>` — [`ObjectMapEntry`](#objectmapentry)

- `fn symbols(self: &Self) -> &[ObjectMapEntry<'data>]` — [`ObjectMapEntry`](#objectmapentry)

- `fn objects(self: &Self) -> &[ObjectMapFile<'data>]` — [`ObjectMapFile`](#objectmapfile)

#### Trait Implementations

##### `impl<'data> Clone for ObjectMap<'data>`

- `fn clone(self: &Self) -> ObjectMap<'data>` — [`ObjectMap`](#objectmap)

##### `impl<'data> Debug for ObjectMap<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for ObjectMap<'data>`

- `fn default() -> ObjectMap<'data>` — [`ObjectMap`](#objectmap)

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

- `fn object<'a>(self: &Self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>` — [`ObjectMap`](#objectmap), [`ObjectMapFile`](#objectmapfile)

#### Trait Implementations

##### `impl<'data> Clone for ObjectMapEntry<'data>`

- `fn clone(self: &Self) -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](#objectmapentry)

##### `impl<'data> Copy for ObjectMapEntry<'data>`

##### `impl<'data> Debug for ObjectMapEntry<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for ObjectMapEntry<'data>`

- `fn default() -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](#objectmapentry)

##### `impl<'data> Eq for ObjectMapEntry<'data>`

##### `impl<'data> Hash for ObjectMapEntry<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for ObjectMapEntry<'data>`

- `fn eq(self: &Self, other: &ObjectMapEntry<'data>) -> bool` — [`ObjectMapEntry`](#objectmapentry)

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

An object file name in an [`ObjectMap`](#objectmap).

#### Implementations

- `fn new(path: &'data [u8], member: Option<&'data [u8]>) -> Self`

- `fn path(self: &Self) -> &'data [u8]`

- `fn member(self: &Self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for ObjectMapFile<'data>`

- `fn clone(self: &Self) -> ObjectMapFile<'data>` — [`ObjectMapFile`](#objectmapfile)

##### `impl<'data> Copy for ObjectMapFile<'data>`

##### `impl<'data> Debug for ObjectMapFile<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for ObjectMapFile<'data>`

##### `impl<'data> Hash for ObjectMapFile<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for ObjectMapFile<'data>`

- `fn eq(self: &Self, other: &ObjectMapFile<'data>) -> bool` — [`ObjectMapFile`](#objectmapfile)

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

- `fn clone(self: &Self) -> Import<'data>` — [`Import`](#import)

##### `impl<'data> Copy for Import<'data>`

##### `impl<'data> Debug for Import<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for Import<'data>`

##### `impl<'data> PartialEq for Import<'data>`

- `fn eq(self: &Self, other: &Import<'data>) -> bool` — [`Import`](#import)

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

- `fn clone(self: &Self) -> Export<'data>` — [`Export`](#export)

##### `impl<'data> Copy for Export<'data>`

##### `impl<'data> Debug for Export<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for Export<'data>`

##### `impl<'data> PartialEq for Export<'data>`

- `fn eq(self: &Self, other: &Export<'data>) -> bool` — [`Export`](#export)

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

- `fn clone(self: &Self) -> CodeView<'data>` — [`CodeView`](#codeview)

##### `impl<'data> Copy for CodeView<'data>`

##### `impl<'data> Debug for CodeView<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for CodeView<'data>`

##### `impl<'data> PartialEq for CodeView<'data>`

- `fn eq(self: &Self, other: &CodeView<'data>) -> bool` — [`CodeView`](#codeview)

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

- `fn kind(self: &Self) -> RelocationKind` — [`RelocationKind`](#relocationkind)

- `fn encoding(self: &Self) -> RelocationEncoding` — [`RelocationEncoding`](#relocationencoding)

- `fn size(self: &Self) -> u8`

- `fn target(self: &Self) -> RelocationTarget` — [`RelocationTarget`](#relocationtarget)

- `fn addend(self: &Self) -> i64`

- `fn set_addend(self: &mut Self, addend: i64)`

- `fn has_implicit_addend(self: &Self) -> bool`

- `fn flags(self: &Self) -> RelocationFlags` — [`RelocationFlags`](#relocationflags)

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

- `fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>` — [`Object`](read/index.md), [`Result`](#result)

- `fn add<'data: 'file, 'file, T>(self: &mut Self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>` — [`Relocation`](#relocation), [`Result`](#result)

- `fn relocate(self: &Self, offset: u64, value: u64) -> u64`

#### Trait Implementations

##### `impl Debug for RelocationMap`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RelocationMap`

- `fn default() -> RelocationMap` — [`RelocationMap`](#relocationmap)

### `RelocationMapEntry`

```rust
struct RelocationMapEntry {
    implicit_addend: bool,
    addend: u64,
}
```

#### Trait Implementations

##### `impl Clone for RelocationMapEntry`

- `fn clone(self: &Self) -> RelocationMapEntry` — [`RelocationMapEntry`](read/index.md)

##### `impl Copy for RelocationMapEntry`

##### `impl Debug for RelocationMapEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationMapEntry`

##### `impl Hash for RelocationMapEntry`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationMapEntry`

- `fn eq(self: &Self, other: &RelocationMapEntry) -> bool` — [`RelocationMapEntry`](read/index.md)

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

- `fn data<'data, R: ReadRef<'data>>(self: Self, file: R) -> Result<CompressedData<'data>>` — [`Result`](#result), [`CompressedData`](#compresseddata)

#### Trait Implementations

##### `impl Clone for CompressedFileRange`

- `fn clone(self: &Self) -> CompressedFileRange` — [`CompressedFileRange`](#compressedfilerange)

##### `impl Copy for CompressedFileRange`

##### `impl Debug for CompressedFileRange`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for CompressedFileRange`

##### `impl Hash for CompressedFileRange`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for CompressedFileRange`

- `fn eq(self: &Self, other: &CompressedFileRange) -> bool` — [`CompressedFileRange`](#compressedfilerange)

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

- `fn decompress(self: Self) -> Result<Cow<'data, [u8]>>` — [`Result`](#result)

#### Trait Implementations

##### `impl<'data> Clone for CompressedData<'data>`

- `fn clone(self: &Self) -> CompressedData<'data>` — [`CompressedData`](#compresseddata)

##### `impl<'data> Copy for CompressedData<'data>`

##### `impl<'data> Debug for CompressedData<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Eq for CompressedData<'data>`

##### `impl<'data> Hash for CompressedData<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'data> PartialEq for CompressedData<'data>`

- `fn eq(self: &Self, other: &CompressedData<'data>) -> bool` — [`CompressedData`](#compresseddata)

##### `impl<'data> StructuralPartialEq for CompressedData<'data>`

## Enums

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

- `fn address_size(self: Self) -> Option<AddressSize>` — [`AddressSize`](#addresssize)

#### Trait Implementations

##### `impl Clone for Architecture`

- `fn clone(self: &Self) -> Architecture` — [`Architecture`](#architecture)

##### `impl Copy for Architecture`

##### `impl Debug for Architecture`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Architecture`

##### `impl Hash for Architecture`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Architecture`

- `fn eq(self: &Self, other: &Architecture) -> bool` — [`Architecture`](#architecture)

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

- `fn clone(self: &Self) -> SubArchitecture` — [`SubArchitecture`](#subarchitecture)

##### `impl Copy for SubArchitecture`

##### `impl Debug for SubArchitecture`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SubArchitecture`

##### `impl Hash for SubArchitecture`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SubArchitecture`

- `fn eq(self: &Self, other: &SubArchitecture) -> bool` — [`SubArchitecture`](#subarchitecture)

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

- `fn clone(self: &Self) -> AddressSize` — [`AddressSize`](#addresssize)

##### `impl Copy for AddressSize`

##### `impl Debug for AddressSize`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AddressSize`

##### `impl Hash for AddressSize`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for AddressSize`

- `fn eq(self: &Self, other: &AddressSize) -> bool` — [`AddressSize`](#addresssize)

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

- `fn native_object() -> BinaryFormat` — [`BinaryFormat`](#binaryformat)

#### Trait Implementations

##### `impl Clone for BinaryFormat`

- `fn clone(self: &Self) -> BinaryFormat` — [`BinaryFormat`](#binaryformat)

##### `impl Copy for BinaryFormat`

##### `impl Debug for BinaryFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for BinaryFormat`

##### `impl Hash for BinaryFormat`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for BinaryFormat`

- `fn eq(self: &Self, other: &BinaryFormat) -> bool` — [`BinaryFormat`](#binaryformat)

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

- `fn clone(self: &Self) -> SectionKind` — [`SectionKind`](#sectionkind)

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SectionKind`

##### `impl Hash for SectionKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SectionKind`

- `fn eq(self: &Self, other: &SectionKind) -> bool` — [`SectionKind`](#sectionkind)

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

- `fn clone(self: &Self) -> ComdatKind` — [`ComdatKind`](#comdatkind)

##### `impl Copy for ComdatKind`

##### `impl Debug for ComdatKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ComdatKind`

##### `impl Hash for ComdatKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ComdatKind`

- `fn eq(self: &Self, other: &ComdatKind) -> bool` — [`ComdatKind`](#comdatkind)

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

- `fn clone(self: &Self) -> SymbolKind` — [`SymbolKind`](#symbolkind)

##### `impl Copy for SymbolKind`

##### `impl Debug for SymbolKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SymbolKind`

##### `impl Hash for SymbolKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolKind`

- `fn eq(self: &Self, other: &SymbolKind) -> bool` — [`SymbolKind`](#symbolkind)

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

- `fn clone(self: &Self) -> SymbolScope` — [`SymbolScope`](#symbolscope)

##### `impl Copy for SymbolScope`

##### `impl Debug for SymbolScope`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SymbolScope`

##### `impl Hash for SymbolScope`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolScope`

- `fn eq(self: &Self, other: &SymbolScope) -> bool` — [`SymbolScope`](#symbolscope)

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

- `fn clone(self: &Self) -> RelocationKind` — [`RelocationKind`](#relocationkind)

##### `impl Copy for RelocationKind`

##### `impl Debug for RelocationKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationKind`

##### `impl Hash for RelocationKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationKind`

- `fn eq(self: &Self, other: &RelocationKind) -> bool` — [`RelocationKind`](#relocationkind)

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

- `fn clone(self: &Self) -> RelocationEncoding` — [`RelocationEncoding`](#relocationencoding)

##### `impl Copy for RelocationEncoding`

##### `impl Debug for RelocationEncoding`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationEncoding`

##### `impl Hash for RelocationEncoding`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationEncoding`

- `fn eq(self: &Self, other: &RelocationEncoding) -> bool` — [`RelocationEncoding`](#relocationencoding)

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

- `fn clone(self: &Self) -> FileFlags` — [`FileFlags`](#fileflags)

##### `impl Copy for FileFlags`

##### `impl Debug for FileFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FileFlags`

##### `impl Hash for FileFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for FileFlags`

- `fn eq(self: &Self, other: &FileFlags) -> bool` — [`FileFlags`](#fileflags)

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

- `fn clone(self: &Self) -> SegmentFlags` — [`SegmentFlags`](#segmentflags)

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SegmentFlags`

##### `impl Hash for SegmentFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SegmentFlags`

- `fn eq(self: &Self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](#segmentflags)

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

- `fn clone(self: &Self) -> SectionFlags` — [`SectionFlags`](#sectionflags)

##### `impl Copy for SectionFlags`

##### `impl Debug for SectionFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SectionFlags`

##### `impl Hash for SectionFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SectionFlags`

- `fn eq(self: &Self, other: &SectionFlags) -> bool` — [`SectionFlags`](#sectionflags)

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

- `fn clone(self: &Self) -> SymbolFlags<Section, Symbol>` — [`SymbolFlags`](#symbolflags)

##### `impl<Section: $crate::marker::Copy, Symbol: $crate::marker::Copy> Copy for SymbolFlags<Section, Symbol>`

##### `impl<Section: $crate::fmt::Debug, Symbol: $crate::fmt::Debug> Debug for SymbolFlags<Section, Symbol>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Section: $crate::cmp::Eq, Symbol: $crate::cmp::Eq> Eq for SymbolFlags<Section, Symbol>`

##### `impl<Section: $crate::hash::Hash, Symbol: $crate::hash::Hash> Hash for SymbolFlags<Section, Symbol>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<Section: $crate::cmp::PartialEq, Symbol: $crate::cmp::PartialEq> PartialEq for SymbolFlags<Section, Symbol>`

- `fn eq(self: &Self, other: &SymbolFlags<Section, Symbol>) -> bool` — [`SymbolFlags`](#symbolflags)

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

- `fn clone(self: &Self) -> RelocationFlags` — [`RelocationFlags`](#relocationflags)

##### `impl Copy for RelocationFlags`

##### `impl Debug for RelocationFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationFlags`

##### `impl Hash for RelocationFlags`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationFlags`

- `fn eq(self: &Self, other: &RelocationFlags) -> bool` — [`RelocationFlags`](#relocationflags)

##### `impl StructuralPartialEq for RelocationFlags`

### `Endianness`

```rust
enum Endianness {
    Little,
    Big,
}
```

An endianness that is selectable at run-time.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Clone for Endianness`

- `fn clone(self: &Self) -> Endianness` — [`Endianness`](#endianness)

##### `impl Copy for Endianness`

##### `impl Debug for Endianness`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Endianness`

- `fn default() -> Endianness` — [`Endianness`](#endianness)

##### `impl Endian for Endianness`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for Endianness`

##### `impl Hash for Endianness`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Endianness`

- `fn eq(self: &Self, other: &Endianness) -> bool` — [`Endianness`](#endianness)

##### `impl StructuralPartialEq for Endianness`

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
  
  See [`archive::ArchiveFile`](read/archive/index.md).

- **`Coff`**

  A COFF object file.
  
  See [`coff::CoffFile`](read/coff/index.md).

- **`CoffBig`**

  A COFF bigobj object file.
  
  This supports a larger number of sections.
  
  See [`coff::CoffBigFile`](read/coff/index.md).

- **`CoffImport`**

  A Windows short import file.
  
  See [`coff::ImportFile`](read/coff/index.md).

- **`DyldCache`**

  A dyld cache file containing Mach-O images.
  
  See [`macho::DyldCache`](read/macho/index.md)

- **`Elf32`**

  A 32-bit ELF file.
  
  See [`elf::ElfFile32`](read/elf/index.md).

- **`Elf64`**

  A 64-bit ELF file.
  
  See [`elf::ElfFile64`](read/elf/index.md).

- **`MachO32`**

  A 32-bit Mach-O file.
  
  See [`macho::MachOFile32`](read/macho/index.md).

- **`MachO64`**

  A 64-bit Mach-O file.
  
  See [`macho::MachOFile64`](read/macho/index.md).

- **`MachOFat32`**

  A 32-bit Mach-O fat binary.
  
  See [`macho::MachOFatFile32`](read/macho/index.md).

- **`MachOFat64`**

  A 64-bit Mach-O fat binary.
  
  See [`macho::MachOFatFile64`](read/macho/index.md).

- **`Pe32`**

  A 32-bit PE file.
  
  See [`pe::PeFile32`](read/pe/index.md).

- **`Pe64`**

  A 64-bit PE file.
  
  See [`pe::PeFile64`](read/pe/index.md).

- **`Xcoff32`**

  A 32-bit XCOFF file.
  
  See [`xcoff::XcoffFile32`](read/xcoff/index.md).

- **`Xcoff64`**

  A 64-bit XCOFF file.
  
  See [`xcoff::XcoffFile64`](read/xcoff/index.md).

#### Implementations

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<FileKind>` — [`Result`](#result), [`FileKind`](#filekind)

- `fn parse_at<'data, R: ReadRef<'data>>(data: R, offset: u64) -> Result<FileKind>` — [`Result`](#result), [`FileKind`](#filekind)

#### Trait Implementations

##### `impl Clone for FileKind`

- `fn clone(self: &Self) -> FileKind` — [`FileKind`](#filekind)

##### `impl Copy for FileKind`

##### `impl Debug for FileKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FileKind`

##### `impl Hash for FileKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for FileKind`

- `fn eq(self: &Self, other: &FileKind) -> bool` — [`FileKind`](#filekind)

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

- `fn clone(self: &Self) -> ObjectKind` — [`ObjectKind`](#objectkind)

##### `impl Copy for ObjectKind`

##### `impl Debug for ObjectKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ObjectKind`

##### `impl Hash for ObjectKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ObjectKind`

- `fn eq(self: &Self, other: &ObjectKind) -> bool` — [`ObjectKind`](#objectkind)

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

The section where an [`ObjectSymbol`](read/index.md) is defined.

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

- `fn index(self: Self) -> Option<SectionIndex>` — [`SectionIndex`](#sectionindex)

#### Trait Implementations

##### `impl Clone for SymbolSection`

- `fn clone(self: &Self) -> SymbolSection` — [`SymbolSection`](#symbolsection)

##### `impl Copy for SymbolSection`

##### `impl Debug for SymbolSection`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SymbolSection`

##### `impl Hash for SymbolSection`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SymbolSection`

- `fn eq(self: &Self, other: &SymbolSection) -> bool` — [`SymbolSection`](#symbolsection)

##### `impl StructuralPartialEq for SymbolSection`

### `RelocationTarget`

```rust
enum RelocationTarget {
    Symbol(SymbolIndex),
    Section(SectionIndex),
    Absolute,
}
```

The target referenced by a [`Relocation`](#relocation).

#### Variants

- **`Symbol`**

  The target is a symbol.

- **`Section`**

  The target is a section.

- **`Absolute`**

  The offset is an absolute address.

#### Trait Implementations

##### `impl Clone for RelocationTarget`

- `fn clone(self: &Self) -> RelocationTarget` — [`RelocationTarget`](#relocationtarget)

##### `impl Copy for RelocationTarget`

##### `impl Debug for RelocationTarget`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationTarget`

##### `impl Hash for RelocationTarget`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RelocationTarget`

- `fn eq(self: &Self, other: &RelocationTarget) -> bool` — [`RelocationTarget`](#relocationtarget)

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

- `fn clone(self: &Self) -> CompressionFormat` — [`CompressionFormat`](#compressionformat)

##### `impl Copy for CompressionFormat`

##### `impl Debug for CompressionFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for CompressionFormat`

##### `impl Hash for CompressionFormat`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for CompressionFormat`

- `fn eq(self: &Self, other: &CompressionFormat) -> bool` — [`CompressionFormat`](#compressionformat)

##### `impl StructuralPartialEq for CompressionFormat`

## Traits

### `Endian`

```rust
trait Endian: Debug + Default + Clone + Copy + PartialEq + Eq + 'static { ... }
```

A trait for using an endianness specification.

Provides methods for converting between the specified endianness and
the native endianness of the target machine.

This trait does not require that the endianness is known at compile time.

#### Required Methods

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn from_little_endian(little_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn is_big_endian(self: Self) -> bool`

  Return true for big endian byte order.

- `fn is_little_endian(self: Self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self: Self, n: u16) -> u16`

  Converts an unsigned 16 bit integer to native endian.

- `fn read_u32(self: Self, n: u32) -> u32`

  Converts an unsigned 32 bit integer to native endian.

- `fn read_u64(self: Self, n: u64) -> u64`

  Converts an unsigned 64 bit integer to native endian.

- `fn read_i16(self: Self, n: i16) -> i16`

  Converts a signed 16 bit integer to native endian.

- `fn read_i32(self: Self, n: i32) -> i32`

  Converts a signed 32 bit integer to native endian.

- `fn read_i64(self: Self, n: i64) -> i64`

  Converts a signed 64 bit integer to native endian.

- `fn read_u16_bytes(self: Self, n: [u8; 2]) -> u16`

  Converts an unaligned unsigned 16 bit integer to native endian.

- `fn read_u32_bytes(self: Self, n: [u8; 4]) -> u32`

  Converts an unaligned unsigned 32 bit integer to native endian.

- `fn read_u64_bytes(self: Self, n: [u8; 8]) -> u64`

  Converts an unaligned unsigned 64 bit integer to native endian.

- `fn read_i16_bytes(self: Self, n: [u8; 2]) -> i16`

  Converts an unaligned signed 16 bit integer to native endian.

- `fn read_i32_bytes(self: Self, n: [u8; 4]) -> i32`

  Converts an unaligned signed 32 bit integer to native endian.

- `fn read_i64_bytes(self: Self, n: [u8; 8]) -> i64`

  Converts an unaligned signed 64 bit integer to native endian.

- `fn write_u16(self: Self, n: u16) -> u16`

  Converts an unsigned 16 bit integer from native endian.

- `fn write_u32(self: Self, n: u32) -> u32`

  Converts an unsigned 32 bit integer from native endian.

- `fn write_u64(self: Self, n: u64) -> u64`

  Converts an unsigned 64 bit integer from native endian.

- `fn write_i16(self: Self, n: i16) -> i16`

  Converts a signed 16 bit integer from native endian.

- `fn write_i32(self: Self, n: i32) -> i32`

  Converts a signed 32 bit integer from native endian.

- `fn write_i64(self: Self, n: i64) -> i64`

  Converts a signed 64 bit integer from native endian.

- `fn write_u16_bytes(self: Self, n: u16) -> [u8; 2]`

  Converts an unaligned unsigned 16 bit integer from native endian.

- `fn write_u32_bytes(self: Self, n: u32) -> [u8; 4]`

  Converts an unaligned unsigned 32 bit integer from native endian.

- `fn write_u64_bytes(self: Self, n: u64) -> [u8; 8]`

  Converts an unaligned unsigned 64 bit integer from native endian.

- `fn write_i16_bytes(self: Self, n: i16) -> [u8; 2]`

  Converts an unaligned signed 16 bit integer from native endian.

- `fn write_i32_bytes(self: Self, n: i32) -> [u8; 4]`

  Converts an unaligned signed 32 bit integer from native endian.

- `fn write_i64_bytes(self: Self, n: i64) -> [u8; 8]`

  Converts an unaligned signed 64 bit integer from native endian.

### `Pod`

```rust
trait Pod: Copy + 'static { ... }
```

A trait for types that can safely be converted from and to byte slices.

# Safety
A type that is `Pod` must:
- be `#[repr(C)]` or `#[repr(transparent)]`
- have no invalid byte values
- have no padding

### `ReadError<T>`

```rust
trait ReadError<T> { ... }
```

#### Required Methods

- `fn read_error(self: Self, error: &'static str) -> Result<T>`

### `SymbolMapEntry`

```rust
trait SymbolMapEntry { ... }
```

An entry in a [`SymbolMap`](#symbolmap).

#### Required Methods

- `fn address(self: &Self) -> u64`

  The symbol address.

## Functions

### `from_bytes`

```rust
fn from_bytes<T: Pod>(data: &[u8]) -> result::Result<(&T, &[u8]), ()>
```

Cast the head of a byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `from_bytes_mut`

```rust
fn from_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<(&mut T, &mut [u8]), ()>
```

Cast the head of a mutable byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes`

```rust
fn slice_from_bytes<T: Pod>(data: &[u8], count: usize) -> result::Result<(&[T], &[u8]), ()>
```

Cast the head of a byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes_mut`

```rust
fn slice_from_bytes_mut<T: Pod>(data: &mut [u8], count: usize) -> result::Result<(&mut [T], &mut [u8]), ()>
```

Cast the head of a mutable byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_all_bytes`

```rust
fn slice_from_all_bytes<T: Pod>(data: &[u8]) -> result::Result<&[T], ()>
```

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `slice_from_all_bytes_mut`

```rust
fn slice_from_all_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<&mut [T], ()>
```

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `bytes_of`

```rust
fn bytes_of<T: Pod>(val: &T) -> &[u8]
```

Cast a `Pod` type to a byte slice.

### `bytes_of_mut`

```rust
fn bytes_of_mut<T: Pod>(val: &mut T) -> &mut [u8]
```

Cast a `Pod` type to a mutable byte slice.

### `bytes_of_slice`

```rust
fn bytes_of_slice<T: Pod>(val: &[T]) -> &[u8]
```

Cast a slice of a `Pod` type to a byte slice.

### `bytes_of_slice_mut`

```rust
fn bytes_of_slice_mut<T: Pod>(val: &mut [T]) -> &mut [u8]
```

Cast a slice of a `Pod` type to a mutable byte slice.

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The native endianness for the target platform.

### `U16<E>`

```rust
type U16<E> = U16Bytes<E>;
```

A `u16` value with an externally specified endianness of type `E`.

### `U32<E>`

```rust
type U32<E> = U32Bytes<E>;
```

A `u32` value with an externally specified endianness of type `E`.

### `U64<E>`

```rust
type U64<E> = U64Bytes<E>;
```

A `u64` value with an externally specified endianness of type `E`.

### `I16<E>`

```rust
type I16<E> = I16Bytes<E>;
```

An `i16` value with an externally specified endianness of type `E`.

### `I32<E>`

```rust
type I32<E> = I32Bytes<E>;
```

An `i32` value with an externally specified endianness of type `E`.

### `I64<E>`

```rust
type I64<E> = I64Bytes<E>;
```

An `i64` value with an externally specified endianness of type `E`.

### `Result<T>`

```rust
type Result<T> = result::Result<T, ()>;
```

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

## Macros

### `unsafe_impl_endian_pod!`

### `unsafe_impl_pod!`

