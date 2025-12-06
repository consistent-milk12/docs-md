*[object](../../index.md) / [read](../index.md) / [macho](index.md)*

---

# Module `macho`

Support for reading Mach-O files.

Traits are used to abstract over the difference between 32-bit and 64-bit Mach-O
files. The primary trait for this is [`MachHeader`](#machheader).

## High level API

[`MachOFile`](#machofile) implements the [`Object`](crate::read::Object) trait for Mach-O files.
[`MachOFile`](#machofile) is parameterised by [`MachHeader`](#machheader) to allow reading both 32-bit and
64-bit Mach-O files. There are type aliases for these parameters ([`MachOFile32`](#machofile32) and
[`MachOFile64`](#machofile64)).

## Low level API

The [`MachHeader`](#machheader) trait can be directly used to parse both `macho::MachHeader32`
and `macho::MachHeader64`. Additionally, [`FatHeader`](../../macho/index.md) and the [`FatArch`](#fatarch) trait
can be used to iterate images in multi-architecture binaries, and [`DyldCache`](#dyldcache) can
be used to locate images in a dyld shared cache.

### Example for low level API
 ```no_run
use object::macho;
use object::read::macho::{MachHeader, Nlist};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let header = macho::MachHeader64::<object::Endianness>::parse(&*data, 0)?;
    let endian = header.endian()?;
    let mut commands = header.load_commands(endian, &*data, 0)?;
    while let Some(command) = commands.next()? {
        if let Some(symtab_command) = command.symtab()? {
            let symbols = symtab_command.symbols::<macho::MachHeader64<_>, _>(endian, &*data)?;
            for symbol in symbols.iter() {
                let name = symbol.name(endian, symbols.strings())?;
                println!("{}", String::from_utf8_lossy(name));
            }
        }
    }
  }
    Ok(())
}
```

## Structs

### `DyldCache<'data, E, R>`

```rust
struct DyldCache<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    endian: E,
    data: R,
    files: alloc::vec::Vec<DyldFile<'data, E, R>>,
    images: &'data [macho::DyldCacheImageInfo<E>],
    arch: crate::read::Architecture,
}
```

A parsed representation of the dyld shared cache.

#### Fields

- **`files`**: `alloc::vec::Vec<DyldFile<'data, E, R>>`

  The first entry is the main cache file, and the rest are subcaches.

#### Implementations

- `fn subcache_suffixes(data: R) -> Result<Vec<String>>` — [`Result`](../../index.md)

- `fn parse(data: R, subcache_data: &[R]) -> Result<Self>` — [`Result`](../../index.md)

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn endianness(self: &Self) -> Endianness` — [`Endianness`](../../index.md)

- `fn data(self: &Self) -> R`

- `fn is_little_endian(self: &Self) -> bool`

- `fn images<'cache>(self: &'cache Self) -> DyldCacheImageIterator<'data, 'cache, E, R>` — [`DyldCacheImageIterator`](#dyldcacheimageiterator)

- `fn mappings<'cache>(self: &'cache Self) -> impl Iterator<Item = DyldCacheMapping<'data, E, R>> + 'cache` — [`DyldCacheMapping`](#dyldcachemapping)

- `fn data_and_offset_for_address(self: &Self, address: u64) -> Option<(R, u64)>`

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCache<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldCacheImageIterator<'data, 'cache, E, R>`

```rust
struct DyldCacheImageIterator<'data, 'cache, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    cache: &'cache DyldCache<'data, E, R>,
    iter: slice::Iter<'data, macho::DyldCacheImageInfo<E>>,
}
```

An iterator over all the images (dylibs) in the dyld shared cache.

#### Trait Implementations

##### `impl<'data, 'cache, E, R> Debug for DyldCacheImageIterator<'data, 'cache, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'cache, E, R> Iterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- `type Item = DyldCacheImage<'data, 'cache, E, R>`

- `fn next(self: &mut Self) -> Option<DyldCacheImage<'data, 'cache, E, R>>` — [`DyldCacheImage`](#dyldcacheimage)

### `DyldCacheImage<'data, 'cache, E, R>`

```rust
struct DyldCacheImage<'data, 'cache, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    cache: &'cache DyldCache<'data, E, R>,
    image_info: &'data macho::DyldCacheImageInfo<E>,
}
```

One image (dylib) from inside the dyld shared cache.

#### Implementations

- `fn info(self: &Self) -> &'data macho::DyldCacheImageInfo<E>` — [`DyldCacheImageInfo`](../../macho/index.md)

- `fn path(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn image_data_and_offset(self: &Self) -> Result<(R, u64)>` — [`Result`](../../index.md)

- `fn parse_object(self: &Self) -> Result<File<'data, R>>` — [`Result`](../../index.md), [`File`](../index.md)

#### Trait Implementations

##### `impl<'data, 'cache, E, R> Debug for DyldCacheImage<'data, 'cache, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldCacheMappingIterator<'data, E, R>`

```rust
struct DyldCacheMappingIterator<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    endian: E,
    data: R,
    iter: DyldCacheMappingVersionIterator<'data, E>,
}
```

An iterator over all the mappings for one subcache in a dyld shared cache.

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheMappingIterator<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for DyldCacheMappingIterator<'data, E, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, E, R> Iterator for DyldCacheMappingIterator<'data, E, R>`

- `type Item = DyldCacheMapping<'data, E, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `DyldCacheMapping<'data, E, R>`

```rust
struct DyldCacheMapping<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    endian: E,
    data: R,
    info: DyldCacheMappingVersion<'data, E>,
}
```

Information about a mapping.

#### Implementations

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn file_offset(self: &Self) -> u64`

- `fn max_prot(self: &Self) -> u32`

- `fn init_prot(self: &Self) -> u32`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn relocations(self: &Self) -> Result<DyldCacheRelocationIterator<'data, E, R>>` — [`Result`](../../index.md), [`DyldCacheRelocationIterator`](#dyldcacherelocationiterator)

#### Trait Implementations

##### `impl<'data, E, R> Clone for DyldCacheMapping<'data, E, R>`

- `fn clone(self: &Self) -> DyldCacheMapping<'data, E, R>` — [`DyldCacheMapping`](#dyldcachemapping)

##### `impl<'data, E, R> Copy for DyldCacheMapping<'data, E, R>`

##### `impl<'data, E, R> Debug for DyldCacheMapping<'data, E, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheRelocationIterator<'data, E, R>`

```rust
struct DyldCacheRelocationIterator<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    version: DyldCacheRelocationIteratorVersion<'data, E, R>,
}
```

An iterator over relocations in a mapping

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIterator<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for DyldCacheRelocationIterator<'data, E, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, E, R> Iterator for DyldCacheRelocationIterator<'data, E, R>`

- `type Item = Result<DyldRelocation, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `DyldRelocation`

```rust
struct DyldRelocation {
    pub offset: u64,
    pub value: u64,
    pub auth: Option<DyldRelocationAuth>,
}
```

A cache mapping relocation.

#### Fields

- **`offset`**: `u64`

  The offset of the relocation within the mapping.
  
  This can be added to either the mapping file offset or the
  mapping address.

- **`value`**: `u64`

  The value to be relocated.

- **`auth`**: `Option<DyldRelocationAuth>`

  The pointer authentication data, if present.

#### Trait Implementations

##### `impl Debug for DyldRelocation`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldRelocationAuth`

```rust
struct DyldRelocationAuth {
    pub key: macho::PtrauthKey,
    pub diversity: u16,
    pub addr_div: bool,
}
```

Pointer authentication data.

This is used for signing pointers for the arm64e ABI.

#### Fields

- **`key`**: `macho::PtrauthKey`

  The key used to generate the signed value.

- **`diversity`**: `u16`

  The integer diversity value.

- **`addr_div`**: `bool`

  Whether the address should be blended with the diversity value.

#### Trait Implementations

##### `impl Debug for DyldRelocationAuth`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MachOFatFile<'data, Fat: FatArch>`

```rust
struct MachOFatFile<'data, Fat: FatArch> {
    header: &'data macho::FatHeader,
    arches: &'data [Fat],
}
```

A Mach-O universal binary.

This is a file that starts with `macho::FatHeader`, and corresponds
to [`crate::FileKind::MachOFat32`](../../index.md#machofat32) or [`crate::FileKind::MachOFat64`](../../index.md#machofat64).

#### Implementations

- `fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn header(self: &Self) -> &'data macho::FatHeader` — [`FatHeader`](../../macho/index.md)

- `fn arches(self: &Self) -> &'data [Fat]`

#### Trait Implementations

##### `impl<'data, Fat: $crate::clone::Clone + FatArch> Clone for MachOFatFile<'data, Fat>`

- `fn clone(self: &Self) -> MachOFatFile<'data, Fat>` — [`MachOFatFile`](#machofatfile)

##### `impl<'data, Fat: $crate::fmt::Debug + FatArch> Debug for MachOFatFile<'data, Fat>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MachOFile<'data, Mach, R>`

```rust
struct MachOFile<'data, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    endian: <Mach as >::Endian,
    data: R,
    header_offset: u64,
    header: &'data Mach,
    segments: alloc::vec::Vec<super::MachOSegmentInternal<'data, Mach, R>>,
    sections: alloc::vec::Vec<super::MachOSectionInternal<'data, Mach, R>>,
    symbols: super::SymbolTable<'data, Mach, R>,
}
```

A partially parsed Mach-O file.

Most of the functionality of this type is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn parse_dyld_cache_image<'cache, E: Endian>(image: &DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](#dyldcacheimage), [`Result`](../../index.md)

- `fn section_internal(self: &Self, index: SectionIndex) -> Result<&MachOSectionInternal<'data, Mach, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`MachOSectionInternal`](section/index.md)

- `fn endian(self: &Self) -> <Mach as >::Endian` — [`MachHeader`](#machheader)

- `fn data(self: &Self) -> R`

- `fn raw_header(self: &Self) -> &'data Mach`

- `fn macho_header(self: &Self) -> &'data Mach`

- `fn macho_load_commands(self: &Self) -> Result<LoadCommandIterator<'data, <Mach as >::Endian>>` — [`Result`](../../index.md), [`LoadCommandIterator`](#loadcommanditerator), [`MachHeader`](#machheader)

- `fn macho_symbol_table(self: &Self) -> &SymbolTable<'data, Mach, R>` — [`SymbolTable`](#symboltable)

- `fn build_version(self: &Self) -> Result<Option<&'data macho::BuildVersionCommand<<Mach as >::Endian>>>` — [`Result`](../../index.md), [`BuildVersionCommand`](../../macho/index.md), [`MachHeader`](#machheader)

#### Trait Implementations

##### `impl<'data, Mach, R> Debug for MachOFile<'data, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Mach, R> Object for MachOFile<'data, Mach, R>`

- `type Segment = MachOSegment<'data, 'file, Mach, R>`

- `type SegmentIterator = MachOSegmentIterator<'data, 'file, Mach, R>`

- `type Section = MachOSection<'data, 'file, Mach, R>`

- `type SectionIterator = MachOSectionIterator<'data, 'file, Mach, R>`

- `type Comdat = MachOComdat<'data, 'file, Mach, R>`

- `type ComdatIterator = MachOComdatIterator<'data, 'file, Mach, R>`

- `type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- `type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- `type SymbolTable = MachOSymbolTable<'data, 'file, Mach, R>`

- `type DynamicRelocationIterator = NoDynamicRelocationIterator`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- `fn segments(self: &Self) -> MachOSegmentIterator<'data, '_, Mach, R>` — [`MachOSegmentIterator`](#machosegmentiterator)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<MachOSection<'data, 'file, Mach, R>>` — [`MachOSection`](#machosection)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<MachOSection<'data, '_, Mach, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`MachOSection`](#machosection)

- `fn sections(self: &Self) -> MachOSectionIterator<'data, '_, Mach, R>` — [`MachOSectionIterator`](#machosectioniterator)

- `fn comdats(self: &Self) -> MachOComdatIterator<'data, '_, Mach, R>` — [`MachOComdatIterator`](#machocomdatiterator)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<MachOSymbol<'data, '_, Mach, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`MachOSymbol`](#machosymbol)

- `fn symbols(self: &Self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](#machosymboliterator)

- `fn symbol_table(self: &Self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](#machosymboltable)

- `fn dynamic_symbols(self: &Self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](#machosymboliterator)

- `fn dynamic_symbol_table(self: &Self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](#machosymboltable)

- `fn object_map(self: &Self) -> ObjectMap<'data>` — [`ObjectMap`](../../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>` — [`Result`](../../index.md)

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, Mach, R> Sealed for MachOFile<'data, Mach, R>`

### `MachOComdatIterator<'data, 'file, Mach, R>`

```rust
struct MachOComdatIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

An iterator for the COMDAT section groups in a [`MachOFile`](#machofile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for MachOComdatIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatIterator<'data, 'file, Mach, R>`

- `type Item = MachOComdat<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `MachOComdat<'data, 'file, Mach, R>`

```rust
struct MachOComdat<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

A COMDAT section group in a [`MachOFile`](#machofile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdat<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectComdat for MachOComdat<'data, 'file, Mach, R>`

- `type SectionIterator = MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOComdat<'data, 'file, Mach, R>`

### `MachOComdatSectionIterator<'data, 'file, Mach, R>`

```rust
struct MachOComdatSectionIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`MachOFile`](#machofile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `LoadCommandIterator<'data, E: Endian>`

```rust
struct LoadCommandIterator<'data, E: Endian> {
    endian: E,
    data: crate::read::Bytes<'data>,
    ncmds: u32,
}
```

An iterator for the load commands from a [`MachHeader`](#machheader).

#### Implementations

- `fn new(endian: E, data: &'data [u8], ncmds: u32) -> Self`

- `fn next(self: &mut Self) -> Result<Option<LoadCommandData<'data, E>>>` — [`Result`](../../index.md), [`LoadCommandData`](#loadcommanddata)

- `fn parse(self: &mut Self) -> Result<LoadCommandData<'data, E>>` — [`Result`](../../index.md), [`LoadCommandData`](#loadcommanddata)

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for LoadCommandIterator<'data, E>`

- `fn clone(self: &Self) -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](#loadcommanditerator)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for LoadCommandIterator<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for LoadCommandIterator<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, E: $crate::default::Default + Endian> Default for LoadCommandIterator<'data, E>`

- `fn default() -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](#loadcommanditerator)

##### `impl<I> IntoIterator for LoadCommandIterator<'data, E>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, E: Endian> Iterator for LoadCommandIterator<'data, E>`

- `type Item = Result<LoadCommandData<'data, E>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `LoadCommandData<'data, E: Endian>`

```rust
struct LoadCommandData<'data, E: Endian> {
    cmd: u32,
    data: crate::read::Bytes<'data>,
    marker: core::marker::PhantomData<E>,
}
```

The data for a `macho::LoadCommand`.

#### Implementations

- `fn cmd(self: &Self) -> u32`

- `fn cmdsize(self: &Self) -> u32`

- `fn data<T: Pod>(self: &Self) -> Result<&'data T>` — [`Result`](../../index.md)

- `fn raw_data(self: &Self) -> &'data [u8]`

- `fn string(self: &Self, endian: E, s: macho::LcStr<E>) -> Result<&'data [u8]>` — [`LcStr`](../../macho/index.md), [`Result`](../../index.md)

- `fn variant(self: &Self) -> Result<LoadCommandVariant<'data, E>>` — [`Result`](../../index.md), [`LoadCommandVariant`](#loadcommandvariant)

- `fn segment_32(self: Self) -> Result<Option<(&'data macho::SegmentCommand32<E>, &'data [u8])>>` — [`Result`](../../index.md), [`SegmentCommand32`](../../macho/index.md)

- `fn symtab(self: Self) -> Result<Option<&'data macho::SymtabCommand<E>>>` — [`Result`](../../index.md), [`SymtabCommand`](../../macho/index.md)

- `fn dysymtab(self: Self) -> Result<Option<&'data macho::DysymtabCommand<E>>>` — [`Result`](../../index.md), [`DysymtabCommand`](../../macho/index.md)

- `fn dylib(self: Self) -> Result<Option<&'data macho::DylibCommand<E>>>` — [`Result`](../../index.md), [`DylibCommand`](../../macho/index.md)

- `fn uuid(self: Self) -> Result<Option<&'data macho::UuidCommand<E>>>` — [`Result`](../../index.md), [`UuidCommand`](../../macho/index.md)

- `fn segment_64(self: Self) -> Result<Option<(&'data macho::SegmentCommand64<E>, &'data [u8])>>` — [`Result`](../../index.md), [`SegmentCommand64`](../../macho/index.md)

- `fn dyld_info(self: Self) -> Result<Option<&'data macho::DyldInfoCommand<E>>>` — [`Result`](../../index.md), [`DyldInfoCommand`](../../macho/index.md)

- `fn entry_point(self: Self) -> Result<Option<&'data macho::EntryPointCommand<E>>>` — [`Result`](../../index.md), [`EntryPointCommand`](../../macho/index.md)

- `fn build_version(self: Self) -> Result<Option<&'data macho::BuildVersionCommand<E>>>` — [`Result`](../../index.md), [`BuildVersionCommand`](../../macho/index.md)

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for LoadCommandData<'data, E>`

- `fn clone(self: &Self) -> LoadCommandData<'data, E>` — [`LoadCommandData`](#loadcommanddata)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for LoadCommandData<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for LoadCommandData<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MachOSegmentIterator<'data, 'file, Mach, R>`

```rust
struct MachOSegmentIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    iter: slice::Iter<'file, MachOSegmentInternal<'data, Mach, R>>,
}
```

An iterator for the segments in a [`MachOFile`](#machofile).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSegmentIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- `type Item = MachOSegment<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `MachOSegment<'data, 'file, Mach, R>`

```rust
struct MachOSegment<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    internal: &'file MachOSegmentInternal<'data, Mach, R>,
}
```

A segment in a [`MachOFile`](#machofile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](#machofile)

- `fn macho_segment(self: &Self) -> &'data <Mach as >::Segment` — [`MachHeader`](#machheader)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSegment<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSegment for MachOSegment<'data, 'file, Mach, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSegment<'data, 'file, Mach, R>`

### `MachOSectionIterator<'data, 'file, Mach, R>`

```rust
struct MachOSectionIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    iter: slice::Iter<'file, MachOSectionInternal<'data, Mach, R>>,
}
```

An iterator for the sections in a [`MachOFile`](#machofile).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSectionIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSectionIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSectionIterator<'data, 'file, Mach, R>`

- `type Item = MachOSection<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `MachOSection<'data, 'file, Mach, R>`

```rust
struct MachOSection<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    internal: MachOSectionInternal<'data, Mach, R>,
}
```

A section in a [`MachOFile`](#machofile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](#machofile)

- `fn macho_section(self: &Self) -> &'data <Mach as >::Section` — [`MachHeader`](#machheader)

- `fn macho_relocations(self: &Self) -> Result<&'data [macho::Relocation<<Mach as >::Endian>]>` — [`Result`](../../index.md), [`Relocation`](../../macho/index.md), [`MachHeader`](#machheader)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn maybe_compressed_gnu(self: &Self) -> Result<Option<CompressedFileRange>>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSection<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSection for MachOSection<'data, 'file, Mach, R>`

- `type RelocationIterator = MachORelocationIterator<'data, 'file, Mach, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- `fn compressed_data(self: &Self) -> read::Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../index.md)

- `fn relocations(self: &Self) -> MachORelocationIterator<'data, 'file, Mach, R>` — [`MachORelocationIterator`](#machorelocationiterator)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSection<'data, 'file, Mach, R>`

### `SymbolTable<'data, Mach: MachHeader, R>`

```rust
struct SymbolTable<'data, Mach: MachHeader, R>
where
    R: ReadRef<'data> {
    symbols: &'data [<Mach as >::Nlist],
    strings: crate::read::util::StringTable<'data, R>,
}
```

A table of symbol entries in a Mach-O file.

Also includes the string table used for the symbol names.

Returned by `macho::SymtabCommand::symbols`.

#### Implementations

- `fn new(symbols: &'data [<Mach as >::Nlist], strings: StringTable<'data, R>) -> Self` — [`MachHeader`](#machheader), [`StringTable`](../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, <Mach as >::Nlist>` — [`MachHeader`](#machheader)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Mach as >::Nlist>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`MachHeader`](#machheader)

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Mach as >::Nlist) -> Option<Entry>>(self: &Self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../index.md)

- `fn object_map(self: &Self, endian: <Mach as >::Endian) -> ObjectMap<'data>` — [`MachHeader`](#machheader), [`ObjectMap`](../../index.md)

#### Trait Implementations

##### `impl<'data, Mach: $crate::clone::Clone + MachHeader, R> Clone for SymbolTable<'data, Mach, R>`

- `fn clone(self: &Self) -> SymbolTable<'data, Mach, R>` — [`SymbolTable`](#symboltable)

##### `impl<'data, Mach: $crate::marker::Copy + MachHeader, R> Copy for SymbolTable<'data, Mach, R>`

##### `impl<'data, Mach: $crate::fmt::Debug + MachHeader, R> Debug for SymbolTable<'data, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Mach: MachHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Mach, R>`

- `fn default() -> Self`

### `MachOSymbolTable<'data, 'file, Mach, R>`

```rust
struct MachOSymbolTable<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
}
```

A symbol table in a [`MachOFile`](#machofile).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbolTable<'data, 'file, Mach, R>`

- `fn clone(self: &Self) -> MachOSymbolTable<'data, 'file, Mach, R>` — [`MachOSymbolTable`](#machosymboltable)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbolTable<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolTable<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbolTable for MachOSymbolTable<'data, 'file, Mach, R>`

- `type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- `type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSymbolTable<'data, 'file, Mach, R>`

### `MachOSymbolIterator<'data, 'file, Mach, R>`

```rust
struct MachOSymbolIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in a [`MachOFile`](#machofile).

#### Implementations

- `fn new(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](#machofile)

- `fn empty(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](#machofile)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- `type Item = MachOSymbol<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `MachOSymbol<'data, 'file, Mach, R>`

```rust
struct MachOSymbol<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    index: crate::read::SymbolIndex,
    nlist: &'data <Mach as >::Nlist,
}
```

A symbol in a [`MachOFile`](#machofile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- `fn new(file: &'file MachOFile<'data, Mach, R>, index: SymbolIndex, nlist: &'data <Mach as >::Nlist) -> Option<Self>` — [`MachOFile`](#machofile), [`SymbolIndex`](../../index.md), [`MachHeader`](#machheader)

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](#machofile)

- `fn macho_symbol(self: &Self) -> &'data <Mach as >::Nlist` — [`MachHeader`](#machheader)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbol<'data, 'file, Mach, R>`

- `fn clone(self: &Self) -> MachOSymbol<'data, 'file, Mach, R>` — [`MachOSymbol`](#machosymbol)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbol<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbol<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbol for MachOSymbol<'data, 'file, Mach, R>`

- `fn index(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

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

##### `impl<'data, 'file, Mach, R> Sealed for MachOSymbol<'data, 'file, Mach, R>`

### `MachORelocationIterator<'data, 'file, Mach, R>`

```rust
struct MachORelocationIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    relocations: slice::Iter<'data, macho::Relocation<<Mach as >::Endian>>,
}
```

An iterator for the relocations in a [`MachOSection`](super::MachOSection).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachORelocationIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachORelocationIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachORelocationIterator<'data, 'file, Mach, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Enums

### `DyldSubCacheSlice<'data, E: Endian>`

```rust
enum DyldSubCacheSlice<'data, E: Endian> {
    V1(&'data [macho::DyldSubCacheEntryV1<E>]),
    V2(&'data [macho::DyldSubCacheEntryV2<E>]),
}
```

A slice of structs describing each subcache.

The struct gained an additional field (the file suffix) in dyld-1042.1 (macOS 13 / iOS 16),
so this is an enum of the two possible slice types.

#### Variants

- **`V1`**

  V1, used between dyld-940 and dyld-1042.1.

- **`V2`**

  V2, used since dyld-1042.1.

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for DyldSubCacheSlice<'data, E>`

- `fn clone(self: &Self) -> DyldSubCacheSlice<'data, E>` — [`DyldSubCacheSlice`](#dyldsubcacheslice)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for DyldSubCacheSlice<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for DyldSubCacheSlice<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldCacheMappingSlice<'data, E: Endian>`

```rust
enum DyldCacheMappingSlice<'data, E: Endian> {
    V1(&'data [macho::DyldCacheMappingInfo<E>]),
    V2(&'data [macho::DyldCacheMappingAndSlideInfo<E>]),
}
```

The array of mappings for a single dyld cache file.

The mappings gained slide info in dyld-832.7 (macOS 11)
so this is an enum of the two possible slice types.

#### Variants

- **`V1`**

  V1, used before dyld-832.7.

- **`V2`**

  V2, used since dyld-832.7.

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for DyldCacheMappingSlice<'data, E>`

- `fn clone(self: &Self) -> DyldCacheMappingSlice<'data, E>` — [`DyldCacheMappingSlice`](#dyldcachemappingslice)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for DyldCacheMappingSlice<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for DyldCacheMappingSlice<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldCacheSlideInfo<'data, E: Endian>`

```rust
enum DyldCacheSlideInfo<'data, E: Endian> {
    None,
    V2 {
        slide: &'data macho::DyldCacheSlideInfo2<E>,
        page_starts: &'data [crate::endian::U16<E>],
        page_extras: &'data [crate::endian::U16<E>],
    },
    V3 {
        slide: &'data macho::DyldCacheSlideInfo3<E>,
        page_starts: &'data [crate::endian::U16<E>],
    },
    V5 {
        slide: &'data macho::DyldCacheSlideInfo5<E>,
        page_starts: &'data [crate::endian::U16<E>],
    },
}
```

The slide info for a dyld cache mapping, including variable length arrays.

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for DyldCacheSlideInfo<'data, E>`

- `fn clone(self: &Self) -> DyldCacheSlideInfo<'data, E>` — [`DyldCacheSlideInfo`](#dyldcacheslideinfo)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for DyldCacheSlideInfo<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for DyldCacheSlideInfo<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LoadCommandVariant<'data, E: Endian>`

```rust
enum LoadCommandVariant<'data, E: Endian> {
    Segment32(&'data macho::SegmentCommand32<E>, &'data [u8]),
    Symtab(&'data macho::SymtabCommand<E>),
    Thread(&'data macho::ThreadCommand<E>, &'data [u8]),
    Dysymtab(&'data macho::DysymtabCommand<E>),
    Dylib(&'data macho::DylibCommand<E>),
    IdDylib(&'data macho::DylibCommand<E>),
    LoadDylinker(&'data macho::DylinkerCommand<E>),
    IdDylinker(&'data macho::DylinkerCommand<E>),
    PreboundDylib(&'data macho::PreboundDylibCommand<E>),
    Routines32(&'data macho::RoutinesCommand32<E>),
    SubFramework(&'data macho::SubFrameworkCommand<E>),
    SubUmbrella(&'data macho::SubUmbrellaCommand<E>),
    SubClient(&'data macho::SubClientCommand<E>),
    SubLibrary(&'data macho::SubLibraryCommand<E>),
    TwolevelHints(&'data macho::TwolevelHintsCommand<E>),
    PrebindCksum(&'data macho::PrebindCksumCommand<E>),
    Segment64(&'data macho::SegmentCommand64<E>, &'data [u8]),
    Routines64(&'data macho::RoutinesCommand64<E>),
    Uuid(&'data macho::UuidCommand<E>),
    Rpath(&'data macho::RpathCommand<E>),
    LinkeditData(&'data macho::LinkeditDataCommand<E>),
    EncryptionInfo32(&'data macho::EncryptionInfoCommand32<E>),
    DyldInfo(&'data macho::DyldInfoCommand<E>),
    VersionMin(&'data macho::VersionMinCommand<E>),
    DyldEnvironment(&'data macho::DylinkerCommand<E>),
    EntryPoint(&'data macho::EntryPointCommand<E>),
    SourceVersion(&'data macho::SourceVersionCommand<E>),
    EncryptionInfo64(&'data macho::EncryptionInfoCommand64<E>),
    LinkerOption(&'data macho::LinkerOptionCommand<E>),
    Note(&'data macho::NoteCommand<E>),
    BuildVersion(&'data macho::BuildVersionCommand<E>),
    FilesetEntry(&'data macho::FilesetEntryCommand<E>),
    Other,
}
```

A `macho::LoadCommand` that has been interpreted according to its `cmd` field.

#### Variants

- **`Segment32`**

  `LC_SEGMENT`

- **`Symtab`**

  `LC_SYMTAB`

- **`Thread`**

  `LC_THREAD` or `LC_UNIXTHREAD`

- **`Dysymtab`**

  `LC_DYSYMTAB`

- **`Dylib`**

  `LC_LOAD_DYLIB`, `LC_LOAD_WEAK_DYLIB`, `LC_REEXPORT_DYLIB`,
  `LC_LAZY_LOAD_DYLIB`, or `LC_LOAD_UPWARD_DYLIB`

- **`IdDylib`**

  `LC_ID_DYLIB`

- **`LoadDylinker`**

  `LC_LOAD_DYLINKER`

- **`IdDylinker`**

  `LC_ID_DYLINKER`

- **`PreboundDylib`**

  `LC_PREBOUND_DYLIB`

- **`Routines32`**

  `LC_ROUTINES`

- **`SubFramework`**

  `LC_SUB_FRAMEWORK`

- **`SubUmbrella`**

  `LC_SUB_UMBRELLA`

- **`SubClient`**

  `LC_SUB_CLIENT`

- **`SubLibrary`**

  `LC_SUB_LIBRARY`

- **`TwolevelHints`**

  `LC_TWOLEVEL_HINTS`

- **`PrebindCksum`**

  `LC_PREBIND_CKSUM`

- **`Segment64`**

  `LC_SEGMENT_64`

- **`Routines64`**

  `LC_ROUTINES_64`

- **`Uuid`**

  `LC_UUID`

- **`Rpath`**

  `LC_RPATH`

- **`LinkeditData`**

  `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,
  `LC_DATA_IN_CODE`, `LC_DYLIB_CODE_SIGN_DRS`, `LC_LINKER_OPTIMIZATION_HINT`,
  `LC_DYLD_EXPORTS_TRIE`, or `LC_DYLD_CHAINED_FIXUPS`.

- **`EncryptionInfo32`**

  `LC_ENCRYPTION_INFO`

- **`DyldInfo`**

  `LC_DYLD_INFO` or `LC_DYLD_INFO_ONLY`

- **`VersionMin`**

  `LC_VERSION_MIN_MACOSX`, `LC_VERSION_MIN_IPHONEOS`, `LC_VERSION_MIN_WATCHOS`,
  or `LC_VERSION_MIN_TVOS`

- **`DyldEnvironment`**

  `LC_DYLD_ENVIRONMENT`

- **`EntryPoint`**

  `LC_MAIN`

- **`SourceVersion`**

  `LC_SOURCE_VERSION`

- **`EncryptionInfo64`**

  `LC_ENCRYPTION_INFO_64`

- **`LinkerOption`**

  `LC_LINKER_OPTION`

- **`Note`**

  `LC_NOTE`

- **`BuildVersion`**

  `LC_BUILD_VERSION`

- **`FilesetEntry`**

  `LC_FILESET_ENTRY`

- **`Other`**

  An unrecognized or obsolete load command.

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for LoadCommandVariant<'data, E>`

- `fn clone(self: &Self) -> LoadCommandVariant<'data, E>` — [`LoadCommandVariant`](#loadcommandvariant)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for LoadCommandVariant<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for LoadCommandVariant<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `FatArch`

```rust
trait FatArch: Pod { ... }
```

A trait for generic access to `macho::FatArch32` and `macho::FatArch64`.

#### Required Methods

- `type Word: 1`

- `const MAGIC: u32`

- `fn cputype(self: &Self) -> u32`

- `fn cpusubtype(self: &Self) -> u32`

- `fn offset(self: &Self) -> <Self as >::Word`

- `fn size(self: &Self) -> <Self as >::Word`

- `fn align(self: &Self) -> u32`

- `fn architecture(self: &Self) -> Architecture`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data<'data, R: ReadRef<'data>>(self: &Self, file: R) -> Result<&'data [u8]>`

### `MachHeader`

```rust
trait MachHeader: Debug + Pod { ... }
```

A trait for generic access to `macho::MachHeader32` and `macho::MachHeader64`.

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `type Segment: 1`

- `type Section: 1`

- `type Nlist: 1`

- `fn is_type_64(self: &Self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_big_endian(self: &Self) -> bool`

  Return true if the `magic` field signifies big-endian.

- `fn is_little_endian(self: &Self) -> bool`

  Return true if the `magic` field signifies little-endian.

- `fn magic(self: &Self) -> u32`

- `fn cputype(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn cpusubtype(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn filetype(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn ncmds(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sizeofcmds(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(self: &Self) -> bool`

- `fn endian(self: &Self) -> Result<<Self as >::Endian>`

- `fn load_commands<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<LoadCommandIterator<'data, <Self as >::Endian>>`

- `fn uuid<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<Option<[u8; 16]>>`

  Return the UUID from the `LC_UUID` load command, if one is present.

### `Segment`

```rust
trait Segment: Debug + Pod { ... }
```

A trait for generic access to `macho::SegmentCommand32` and `macho::SegmentCommand64`.

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `type Section: 1`

- `fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>`

- `fn cmd(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn cmdsize(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn vmaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn vmsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn fileoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn filesize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn maxprot(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn initprot(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn nsects(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn name(self: &Self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(self: &Self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Get the segment data from the file data.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, section_data: R) -> Result<&'data [<Self as >::Section]>`

  Get the array of sections from the data following the segment command.

### `Section`

```rust
trait Section: Debug + Pod { ... }
```

A trait for generic access to `macho::Section32` and `macho::Section64`.

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn sectname(self: &Self) -> &[u8; 16]`

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn offset(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn align(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn reloff(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn nreloc(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn name(self: &Self) -> &[u8]`

  Return the `sectname` bytes up until the null terminator.

- `fn segment_name(self: &Self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(self: &Self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

- `fn relocations<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> Result<&'data [macho::Relocation<<Self as >::Endian>]>`

  Return the relocation array.

### `Nlist`

```rust
trait Nlist: Debug + Pod { ... }
```

A trait for generic access to `macho::Nlist32` and `macho::Nlist64`.

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn n_strx(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_type(self: &Self) -> u8`

- `fn n_sect(self: &Self) -> u8`

- `fn n_desc(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn n_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn name<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

- `fn is_stab(self: &Self) -> bool`

  Return true if this is a STAB symbol.

- `fn is_undefined(self: &Self) -> bool`

  Return true if this is an undefined symbol.

- `fn is_definition(self: &Self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn library_ordinal(self: &Self, endian: <Self as >::Endian) -> u8`

  Return the library ordinal.

## Type Aliases

### `MachOFatFile32<'data>`

```rust
type MachOFatFile32<'data> = MachOFatFile<'data, macho::FatArch32>;
```

A 32-bit Mach-O universal binary.

This is a file that starts with `macho::FatHeader`, and corresponds
to [`crate::FileKind::MachOFat32`](../../index.md#machofat32).

### `MachOFatFile64<'data>`

```rust
type MachOFatFile64<'data> = MachOFatFile<'data, macho::FatArch64>;
```

A 64-bit Mach-O universal binary.

This is a file that starts with `macho::FatHeader`, and corresponds
to [`crate::FileKind::MachOFat64`](../../index.md#machofat64).

### `MachOFile32<'data, Endian, R>`

```rust
type MachOFile32<'data, Endian, R> = MachOFile<'data, macho::MachHeader32<Endian>, R>;
```

A 32-bit Mach-O object file.

This is a file that starts with `macho::MachHeader32`, and corresponds
to [`crate::FileKind::MachO32`](../../index.md#macho32).

### `MachOFile64<'data, Endian, R>`

```rust
type MachOFile64<'data, Endian, R> = MachOFile<'data, macho::MachHeader64<Endian>, R>;
```

A 64-bit Mach-O object file.

This is a file that starts with `macho::MachHeader64`, and corresponds
to [`crate::FileKind::MachO64`](../../index.md#macho64).

### `MachOComdatIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator32<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the COMDAT section groups in a [`MachOFile64`](#machofile64).

### `MachOComdatIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator64<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the COMDAT section groups in a [`MachOFile64`](#machofile64).

### `MachOComdat32<'data, 'file, Endian, R>`

```rust
type MachOComdat32<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A COMDAT section group in a [`MachOFile32`](#machofile32).

### `MachOComdat64<'data, 'file, Endian, R>`

```rust
type MachOComdat64<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A COMDAT section group in a [`MachOFile64`](#machofile64).

### `MachOComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator32<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in a [`MachOFile32`](#machofile32).

### `MachOComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator64<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in a [`MachOFile64`](#machofile64).

### `MachOSegmentIterator32<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator32<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the segments in a [`MachOFile32`](super::MachOFile32).

### `MachOSegmentIterator64<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator64<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the segments in a [`MachOFile64`](super::MachOFile64).

### `MachOSegment32<'data, 'file, Endian, R>`

```rust
type MachOSegment32<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A segment in a [`MachOFile32`](super::MachOFile32).

### `MachOSegment64<'data, 'file, Endian, R>`

```rust
type MachOSegment64<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A segment in a [`MachOFile64`](super::MachOFile64).

### `MachOSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator32<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the sections in a [`MachOFile32`](super::MachOFile32).

### `MachOSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator64<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the sections in a [`MachOFile64`](super::MachOFile64).

### `MachOSection32<'data, 'file, Endian, R>`

```rust
type MachOSection32<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A section in a [`MachOFile32`](super::MachOFile32).

### `MachOSection64<'data, 'file, Endian, R>`

```rust
type MachOSection64<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A section in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbolTable32<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable32<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A symbol table in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolTable64<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable64<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A symbol table in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbolIterator32<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator32<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the symbols in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolIterator64<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator64<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the symbols in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbol32<'data, 'file, Endian, R>`

```rust
type MachOSymbol32<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A symbol in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbol64<'data, 'file, Endian, R>`

```rust
type MachOSymbol64<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A symbol in a [`MachOFile64`](super::MachOFile64).

### `MachORelocationIterator32<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator32<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the relocations in a [`MachOSection32`](super::MachOSection32).

### `MachORelocationIterator64<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator64<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the relocations in a [`MachOSection64`](super::MachOSection64).

