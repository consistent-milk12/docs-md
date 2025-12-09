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

The [`MachHeader`](#machheader) trait can be directly used to parse both [`macho::MachHeader32`](../../macho/index.md)
and [`macho::MachHeader64`](../../macho/index.md). Additionally, [`FatHeader`](../../macho/index.md) and the [`FatArch`](#fatarch) trait
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

## Contents

- [Modules](#modules)
  - [`dyld_cache`](#dyld_cache)
  - [`fat`](#fat)
  - [`file`](#file)
  - [`load_command`](#load_command)
  - [`segment`](#segment)
  - [`section`](#section)
  - [`symbol`](#symbol)
  - [`relocation`](#relocation)
- [Structs](#structs)
  - [`DyldCache`](#dyldcache)
  - [`DyldFile`](#dyldfile)
  - [`DyldCacheImageIterator`](#dyldcacheimageiterator)
  - [`DyldCacheImage`](#dyldcacheimage)
  - [`DyldCacheMappingIterator`](#dyldcachemappingiterator)
  - [`DyldCacheMapping`](#dyldcachemapping)
  - [`DyldCacheRelocationIterator`](#dyldcacherelocationiterator)
  - [`DyldCacheRelocationIteratorV2`](#dyldcacherelocationiteratorv2)
  - [`DyldCacheRelocationIteratorV3`](#dyldcacherelocationiteratorv3)
  - [`DyldCacheRelocationIteratorV5`](#dyldcacherelocationiteratorv5)
  - [`DyldRelocation`](#dyldrelocation)
  - [`DyldRelocationAuth`](#dyldrelocationauth)
  - [`MachOFatFile`](#machofatfile)
  - [`MachOFile`](#machofile)
  - [`MachOComdatIterator`](#machocomdatiterator)
  - [`MachOComdat`](#machocomdat)
  - [`MachOComdatSectionIterator`](#machocomdatsectioniterator)
  - [`LoadCommandIterator`](#loadcommanditerator)
  - [`LoadCommandData`](#loadcommanddata)
  - [`MachOSegmentIterator`](#machosegmentiterator)
  - [`MachOSegment`](#machosegment)
  - [`MachOSegmentInternal`](#machosegmentinternal)
  - [`MachOSectionIterator`](#machosectioniterator)
  - [`MachOSection`](#machosection)
  - [`MachOSectionInternal`](#machosectioninternal)
  - [`SymbolTable`](#symboltable)
  - [`MachOSymbolTable`](#machosymboltable)
  - [`MachOSymbolIterator`](#machosymboliterator)
  - [`MachOSymbol`](#machosymbol)
  - [`MachORelocationIterator`](#machorelocationiterator)
- [Enums](#enums)
  - [`DyldSubCacheSlice`](#dyldsubcacheslice)
  - [`DyldCacheMappingSlice`](#dyldcachemappingslice)
  - [`DyldCacheMappingVersionIterator`](#dyldcachemappingversioniterator)
  - [`DyldCacheMappingVersion`](#dyldcachemappingversion)
  - [`DyldCacheSlideInfo`](#dyldcacheslideinfo)
  - [`DyldCacheRelocationIteratorVersion`](#dyldcacherelocationiteratorversion)
  - [`RelocationStateV2`](#relocationstatev2)
  - [`RelocationStateV3`](#relocationstatev3)
  - [`RelocationStateV5`](#relocationstatev5)
  - [`LoadCommandVariant`](#loadcommandvariant)
- [Traits](#traits)
  - [`FatArch`](#fatarch)
  - [`MachHeader`](#machheader)
  - [`Segment`](#segment)
  - [`Section`](#section)
  - [`Nlist`](#nlist)
- [Type Aliases](#type-aliases)
  - [`MachOFatFile32`](#machofatfile32)
  - [`MachOFatFile64`](#machofatfile64)
  - [`MachOFile32`](#machofile32)
  - [`MachOFile64`](#machofile64)
  - [`MachOComdatIterator32`](#machocomdatiterator32)
  - [`MachOComdatIterator64`](#machocomdatiterator64)
  - [`MachOComdat32`](#machocomdat32)
  - [`MachOComdat64`](#machocomdat64)
  - [`MachOComdatSectionIterator32`](#machocomdatsectioniterator32)
  - [`MachOComdatSectionIterator64`](#machocomdatsectioniterator64)
  - [`MachOSegmentIterator32`](#machosegmentiterator32)
  - [`MachOSegmentIterator64`](#machosegmentiterator64)
  - [`MachOSegment32`](#machosegment32)
  - [`MachOSegment64`](#machosegment64)
  - [`MachOSectionIterator32`](#machosectioniterator32)
  - [`MachOSectionIterator64`](#machosectioniterator64)
  - [`MachOSection32`](#machosection32)
  - [`MachOSection64`](#machosection64)
  - [`MachOSymbolTable32`](#machosymboltable32)
  - [`MachOSymbolTable64`](#machosymboltable64)
  - [`MachOSymbolIterator32`](#machosymboliterator32)
  - [`MachOSymbolIterator64`](#machosymboliterator64)
  - [`MachOSymbol32`](#machosymbol32)
  - [`MachOSymbol64`](#machosymbol64)
  - [`MachORelocationIterator32`](#machorelocationiterator32)
  - [`MachORelocationIterator64`](#machorelocationiterator64)
- [Constants](#constants)
  - [`MIN_HEADER_SIZE_SUBCACHES_V1`](#min_header_size_subcaches_v1)
  - [`MIN_HEADER_SIZE_SUBCACHES_V2`](#min_header_size_subcaches_v2)
  - [`MIN_HEADER_SIZE_MAPPINGS_V2`](#min_header_size_mappings_v2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dyld_cache`](#dyld_cache) | mod |  |
| [`fat`](#fat) | mod |  |
| [`file`](#file) | mod |  |
| [`load_command`](#load_command) | mod |  |
| [`segment`](#segment) | mod |  |
| [`section`](#section) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`relocation`](#relocation) | mod |  |
| [`DyldCache`](#dyldcache) | struct | A parsed representation of the dyld shared cache. |
| [`DyldFile`](#dyldfile) | struct | The data for one file in the cache. |
| [`DyldCacheImageIterator`](#dyldcacheimageiterator) | struct | An iterator over all the images (dylibs) in the dyld shared cache. |
| [`DyldCacheImage`](#dyldcacheimage) | struct | One image (dylib) from inside the dyld shared cache. |
| [`DyldCacheMappingIterator`](#dyldcachemappingiterator) | struct | An iterator over all the mappings for one subcache in a dyld shared cache. |
| [`DyldCacheMapping`](#dyldcachemapping) | struct | Information about a mapping. |
| [`DyldCacheRelocationIterator`](#dyldcacherelocationiterator) | struct | An iterator over relocations in a mapping |
| [`DyldCacheRelocationIteratorV2`](#dyldcacherelocationiteratorv2) | struct |  |
| [`DyldCacheRelocationIteratorV3`](#dyldcacherelocationiteratorv3) | struct |  |
| [`DyldCacheRelocationIteratorV5`](#dyldcacherelocationiteratorv5) | struct |  |
| [`DyldRelocation`](#dyldrelocation) | struct | A cache mapping relocation. |
| [`DyldRelocationAuth`](#dyldrelocationauth) | struct | Pointer authentication data. |
| [`MachOFatFile`](#machofatfile) | struct | A Mach-O universal binary. |
| [`MachOFile`](#machofile) | struct | A partially parsed Mach-O file. |
| [`MachOComdatIterator`](#machocomdatiterator) | struct | An iterator for the COMDAT section groups in a [`MachOFile`]. |
| [`MachOComdat`](#machocomdat) | struct | A COMDAT section group in a [`MachOFile`]. |
| [`MachOComdatSectionIterator`](#machocomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`MachOFile`]. |
| [`LoadCommandIterator`](#loadcommanditerator) | struct | An iterator for the load commands from a [`MachHeader`]. |
| [`LoadCommandData`](#loadcommanddata) | struct | The data for a [`macho::LoadCommand`]. |
| [`MachOSegmentIterator`](#machosegmentiterator) | struct | An iterator for the segments in a [`MachOFile`]. |
| [`MachOSegment`](#machosegment) | struct | A segment in a [`MachOFile`]. |
| [`MachOSegmentInternal`](#machosegmentinternal) | struct |  |
| [`MachOSectionIterator`](#machosectioniterator) | struct | An iterator for the sections in a [`MachOFile`]. |
| [`MachOSection`](#machosection) | struct | A section in a [`MachOFile`]. |
| [`MachOSectionInternal`](#machosectioninternal) | struct |  |
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in a Mach-O file. |
| [`MachOSymbolTable`](#machosymboltable) | struct | A symbol table in a [`MachOFile`]. |
| [`MachOSymbolIterator`](#machosymboliterator) | struct | An iterator for the symbols in a [`MachOFile`]. |
| [`MachOSymbol`](#machosymbol) | struct | A symbol in a [`MachOFile`]. |
| [`MachORelocationIterator`](#machorelocationiterator) | struct | An iterator for the relocations in a [`MachOSection`](super::MachOSection). |
| [`DyldSubCacheSlice`](#dyldsubcacheslice) | enum | A slice of structs describing each subcache. |
| [`DyldCacheMappingSlice`](#dyldcachemappingslice) | enum | The array of mappings for a single dyld cache file. |
| [`DyldCacheMappingVersionIterator`](#dyldcachemappingversioniterator) | enum |  |
| [`DyldCacheMappingVersion`](#dyldcachemappingversion) | enum |  |
| [`DyldCacheSlideInfo`](#dyldcacheslideinfo) | enum | The slide info for a dyld cache mapping, including variable length arrays. |
| [`DyldCacheRelocationIteratorVersion`](#dyldcacherelocationiteratorversion) | enum |  |
| [`RelocationStateV2`](#relocationstatev2) | enum |  |
| [`RelocationStateV3`](#relocationstatev3) | enum |  |
| [`RelocationStateV5`](#relocationstatev5) | enum |  |
| [`LoadCommandVariant`](#loadcommandvariant) | enum | A [`macho::LoadCommand`] that has been interpreted according to its `cmd` field. |
| [`FatArch`](#fatarch) | trait | A trait for generic access to [`macho::FatArch32`] and [`macho::FatArch64`]. |
| [`MachHeader`](#machheader) | trait | A trait for generic access to [`macho::MachHeader32`] and [`macho::MachHeader64`]. |
| [`Segment`](#segment) | trait | A trait for generic access to [`macho::SegmentCommand32`] and [`macho::SegmentCommand64`]. |
| [`Section`](#section) | trait | A trait for generic access to [`macho::Section32`] and [`macho::Section64`]. |
| [`Nlist`](#nlist) | trait | A trait for generic access to [`macho::Nlist32`] and [`macho::Nlist64`]. |
| [`MachOFatFile32`](#machofatfile32) | type | A 32-bit Mach-O universal binary. |
| [`MachOFatFile64`](#machofatfile64) | type | A 64-bit Mach-O universal binary. |
| [`MachOFile32`](#machofile32) | type | A 32-bit Mach-O object file. |
| [`MachOFile64`](#machofile64) | type | A 64-bit Mach-O object file. |
| [`MachOComdatIterator32`](#machocomdatiterator32) | type | An iterator for the COMDAT section groups in a [`MachOFile64`]. |
| [`MachOComdatIterator64`](#machocomdatiterator64) | type | An iterator for the COMDAT section groups in a [`MachOFile64`]. |
| [`MachOComdat32`](#machocomdat32) | type | A COMDAT section group in a [`MachOFile32`]. |
| [`MachOComdat64`](#machocomdat64) | type | A COMDAT section group in a [`MachOFile64`]. |
| [`MachOComdatSectionIterator32`](#machocomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`MachOFile32`]. |
| [`MachOComdatSectionIterator64`](#machocomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`MachOFile64`]. |
| [`MachOSegmentIterator32`](#machosegmentiterator32) | type | An iterator for the segments in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSegmentIterator64`](#machosegmentiterator64) | type | An iterator for the segments in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSegment32`](#machosegment32) | type | A segment in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSegment64`](#machosegment64) | type | A segment in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSectionIterator32`](#machosectioniterator32) | type | An iterator for the sections in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSectionIterator64`](#machosectioniterator64) | type | An iterator for the sections in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSection32`](#machosection32) | type | A section in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSection64`](#machosection64) | type | A section in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSymbolTable32`](#machosymboltable32) | type | A symbol table in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSymbolTable64`](#machosymboltable64) | type | A symbol table in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSymbolIterator32`](#machosymboliterator32) | type | An iterator for the symbols in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSymbolIterator64`](#machosymboliterator64) | type | An iterator for the symbols in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSymbol32`](#machosymbol32) | type | A symbol in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSymbol64`](#machosymbol64) | type | A symbol in a [`MachOFile64`](super::MachOFile64). |
| [`MachORelocationIterator32`](#machorelocationiterator32) | type | An iterator for the relocations in a [`MachOSection32`](super::MachOSection32). |
| [`MachORelocationIterator64`](#machorelocationiterator64) | type | An iterator for the relocations in a [`MachOSection64`](super::MachOSection64). |
| [`MIN_HEADER_SIZE_SUBCACHES_V1`](#min_header_size_subcaches_v1) | const |  |
| [`MIN_HEADER_SIZE_SUBCACHES_V2`](#min_header_size_subcaches_v2) | const |  |
| [`MIN_HEADER_SIZE_MAPPINGS_V2`](#min_header_size_mappings_v2) | const |  |

## Modules

- [`dyld_cache`](dyld_cache/index.md)
- [`fat`](fat/index.md)
- [`file`](file/index.md)
- [`load_command`](load_command/index.md)
- [`segment`](segment/index.md)
- [`section`](section/index.md)
- [`symbol`](symbol/index.md)
- [`relocation`](relocation/index.md)

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:12-23`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L12-L23)*

A parsed representation of the dyld shared cache.

#### Fields

- **`files`**: `alloc::vec::Vec<DyldFile<'data, E, R>>`

  The first entry is the main cache file, and the rest are subcaches.

#### Implementations

- <span id="dyldcache-subcache-suffixes"></span>`fn subcache_suffixes(data: R) -> Result<Vec<String>>` — [`Result`](../../index.md)

- <span id="dyldcache-parse"></span>`fn parse(data: R, subcache_data: &[R]) -> Result<Self>` — [`Result`](../../index.md)

- <span id="dyldcache-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md)

- <span id="dyldcache-endianness"></span>`fn endianness(&self) -> Endianness` — [`Endianness`](../../index.md)

- <span id="dyldcache-data"></span>`fn data(&self) -> R`

- <span id="dyldcache-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="dyldcache-images"></span>`fn images<'cache>(self: &'cache Self) -> DyldCacheImageIterator<'data, 'cache, E, R>` — [`DyldCacheImageIterator`](#dyldcacheimageiterator)

- <span id="dyldcache-mappings"></span>`fn mappings<'cache>(self: &'cache Self) -> impl Iterator<Item = DyldCacheMapping<'data, E, R>> + 'cache` — [`DyldCacheMapping`](#dyldcachemapping)

- <span id="dyldcache-data-and-offset-for-address"></span>`fn data_and_offset_for_address(&self, address: u64) -> Option<(R, u64)>`

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCache<'data, E, R>`

- <span id="dyldcache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldFile<'data, E, R>`

```rust
struct DyldFile<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    data: R,
    mappings: DyldCacheMappingSlice<'data, E>,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:214-221`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L214-L221)*

The data for one file in the cache.

#### Implementations

- <span id="dyldfile-mappings"></span>`fn mappings(&self, endian: E) -> DyldCacheMappingIterator<'data, E, R>` — [`DyldCacheMappingIterator`](#dyldcachemappingiterator)

- <span id="dyldfile-address-to-file-offset"></span>`fn address_to_file_offset(&self, endian: E, address: u64) -> Option<u64>`

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldFile<'data, E, R>`

- <span id="dyldfile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:256-263`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L256-L263)*

An iterator over all the images (dylibs) in the dyld shared cache.

#### Trait Implementations

##### `impl<'data, 'cache, E, R> Debug for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcacheimageiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcacheimageiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'cache, E, R> Iterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-type-item"></span>`type Item = DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-next"></span>`fn next(&mut self) -> Option<DyldCacheImage<'data, 'cache, E, R>>` — [`DyldCacheImage`](#dyldcacheimage)

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:283-290`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L283-L290)*

One image (dylib) from inside the dyld shared cache.

#### Implementations

- <span id="dyldcacheimage-info"></span>`fn info(&self) -> &'data macho::DyldCacheImageInfo<E>` — [`DyldCacheImageInfo`](../../macho/index.md)

- <span id="dyldcacheimage-path"></span>`fn path(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="dyldcacheimage-image-data-and-offset"></span>`fn image_data_and_offset(&self) -> Result<(R, u64)>` — [`Result`](../../index.md)

- <span id="dyldcacheimage-parse-object"></span>`fn parse_object(&self) -> Result<File<'data, R>>` — [`Result`](../../index.md), [`File`](../index.md)

#### Trait Implementations

##### `impl<'data, 'cache, E, R> Debug for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:343-351`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L343-L351)*

An iterator over all the mappings for one subcache in a dyld shared cache.

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcachemappingiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcachemappingiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, E, R> Iterator for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-type-item"></span>`type Item = DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemappingiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:384-392`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L384-L392)*

Information about a mapping.

#### Implementations

- <span id="dyldcachemapping-address"></span>`fn address(&self) -> u64`

- <span id="dyldcachemapping-size"></span>`fn size(&self) -> u64`

- <span id="dyldcachemapping-file-offset"></span>`fn file_offset(&self) -> u64`

- <span id="dyldcachemapping-max-prot"></span>`fn max_prot(&self) -> u32`

- <span id="dyldcachemapping-init-prot"></span>`fn init_prot(&self) -> u32`

- <span id="dyldcachemapping-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="dyldcachemapping-relocations"></span>`fn relocations(&self) -> Result<DyldCacheRelocationIterator<'data, E, R>>` — [`Result`](../../index.md), [`DyldCacheRelocationIterator`](#dyldcacherelocationiterator)

#### Trait Implementations

##### `impl<'data, E, R> Clone for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-clone"></span>`fn clone(&self) -> DyldCacheMapping<'data, E, R>` — [`DyldCacheMapping`](#dyldcachemapping)

##### `impl<'data, E, R> Copy for DyldCacheMapping<'data, E, R>`

##### `impl<'data, E, R> Debug for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheRelocationIterator<'data, E, R>`

```rust
struct DyldCacheRelocationIterator<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    version: DyldCacheRelocationIteratorVersion<'data, E, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:558-564`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L558-L564)*

An iterator over relocations in a mapping

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcacherelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcacherelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, E, R> Iterator for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-type-item"></span>`type Item = Result<DyldRelocation, Error>`

- <span id="dyldcacherelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `DyldCacheRelocationIteratorV2<'data, E, R>`

```rust
struct DyldCacheRelocationIteratorV2<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    data: R,
    endian: E,
    mapping_file_offset: u64,
    page_size: u64,
    delta_mask: u64,
    delta_shift: u32,
    value_add: u64,
    page_starts: &'data [crate::endian::U16<E>],
    page_extras: &'data [crate::endian::U16<E>],
    state: RelocationStateV2,
    start_index: usize,
    extra_index: usize,
    page_offset: u64,
    offset: u64,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:605-629`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L605-L629)*

#### Fields

- **`start_index`**: `usize`

  The next index within page_starts.

- **`extra_index`**: `usize`

  The next index within page_extras.

- **`page_offset`**: `u64`

  The current page offset within the mapping.

- **`offset`**: `u64`

  The offset of the next linked list entry within the page.

#### Implementations

- <span id="dyldcacherelocationiteratorv2-next"></span>`fn next(&mut self) -> Result<Option<DyldRelocation>>` — [`Result`](../../index.md), [`DyldRelocation`](#dyldrelocation)

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheRelocationIteratorV3<'data, E, R>`

```rust
struct DyldCacheRelocationIteratorV3<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    data: R,
    endian: E,
    mapping_file_offset: u64,
    auth_value_add: u64,
    page_size: u64,
    page_starts: &'data [crate::endian::U16<E>],
    state: RelocationStateV3,
    start_index: usize,
    offset: u64,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:713-730`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L713-L730)*

#### Fields

- **`start_index`**: `usize`

  Index of the page within the mapping.

- **`offset`**: `u64`

  The current offset within the mapping.

#### Implementations

- <span id="dyldcacherelocationiteratorv3-next"></span>`fn next(&mut self) -> Result<Option<DyldRelocation>>` — [`Result`](../../index.md), [`DyldRelocation`](#dyldrelocation)

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheRelocationIteratorV5<'data, E, R>`

```rust
struct DyldCacheRelocationIteratorV5<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    data: R,
    endian: E,
    mapping_file_offset: u64,
    page_size: u64,
    value_add: u64,
    page_starts: &'data [crate::endian::U16<E>],
    state: RelocationStateV5,
    start_index: usize,
    offset: u64,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:810-827`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L810-L827)*

#### Fields

- **`start_index`**: `usize`

  The next index within page_starts.

- **`offset`**: `u64`

  The current offset within the mapping.

#### Implementations

- <span id="dyldcacherelocationiteratorv5-next"></span>`fn next(&mut self) -> Result<Option<DyldRelocation>>` — [`Result`](../../index.md), [`DyldRelocation`](#dyldrelocation)

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldRelocation`

```rust
struct DyldRelocation {
    pub offset: u64,
    pub value: u64,
    pub auth: Option<DyldRelocationAuth>,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:896-906`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L896-L906)*

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

- <span id="dyldrelocation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldRelocationAuth`

```rust
struct DyldRelocationAuth {
    pub key: macho::PtrauthKey,
    pub diversity: u16,
    pub addr_div: bool,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:921-928`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L921-L928)*

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

- <span id="dyldrelocationauth-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MachOFatFile<'data, Fat: FatArch>`

```rust
struct MachOFatFile<'data, Fat: FatArch> {
    header: &'data macho::FatHeader,
    arches: &'data [Fat],
}
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:25-28`](../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L25-L28)*

A Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat32`](../../index.md) or [`crate::FileKind::MachOFat64`](../../index.md).

#### Implementations

- <span id="machofatfile-parse"></span>`fn parse<R: ReadRef<'data>>(data: R) -> Result<Self>` — [`Result`](../../index.md)

- <span id="machofatfile-header"></span>`fn header(&self) -> &'data macho::FatHeader` — [`FatHeader`](../../macho/index.md)

- <span id="machofatfile-arches"></span>`fn arches(&self) -> &'data [Fat]`

#### Trait Implementations

##### `impl<'data, Fat: clone::Clone + FatArch> Clone for MachOFatFile<'data, Fat>`

- <span id="machofatfile-clone"></span>`fn clone(&self) -> MachOFatFile<'data, Fat>` — [`MachOFatFile`](#machofatfile)

##### `impl<'data, Fat: fmt::Debug + FatArch> Debug for MachOFatFile<'data, Fat>`

- <span id="machofatfile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/macho/file.rs:37-49`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L37-L49)*

A partially parsed Mach-O file.

Most of the functionality of this type is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="machofile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- <span id="machofile-parse-dyld-cache-image"></span>`fn parse_dyld_cache_image<'cache, E: Endian>(image: &DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](#dyldcacheimage), [`Result`](../../index.md)

- <span id="machofile-section-internal"></span>`fn section_internal(&self, index: SectionIndex) -> Result<&MachOSectionInternal<'data, Mach, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`MachOSectionInternal`](section/index.md)

- <span id="machofile-endian"></span>`fn endian(&self) -> <Mach as >::Endian` — [`MachHeader`](#machheader)

- <span id="machofile-data"></span>`fn data(&self) -> R`

- <span id="machofile-raw-header"></span>`fn raw_header(&self) -> &'data Mach`

- <span id="machofile-macho-header"></span>`fn macho_header(&self) -> &'data Mach`

- <span id="machofile-macho-load-commands"></span>`fn macho_load_commands(&self) -> Result<LoadCommandIterator<'data, <Mach as >::Endian>>` — [`Result`](../../index.md), [`LoadCommandIterator`](#loadcommanditerator), [`MachHeader`](#machheader)

- <span id="machofile-macho-symbol-table"></span>`fn macho_symbol_table(&self) -> &SymbolTable<'data, Mach, R>` — [`SymbolTable`](#symboltable)

- <span id="machofile-build-version"></span>`fn build_version(&self) -> Result<Option<&'data macho::BuildVersionCommand<<Mach as >::Endian>>>` — [`Result`](../../index.md), [`BuildVersionCommand`](../../macho/index.md), [`MachHeader`](#machheader)

#### Trait Implementations

##### `impl<'data, Mach, R> Debug for MachOFile<'data, Mach, R>`

- <span id="machofile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Mach, R> Object for MachOFile<'data, Mach, R>`

- <span id="machofile-type-segment"></span>`type Segment = MachOSegment<'data, 'file, Mach, R>`

- <span id="machofile-type-segmentiterator"></span>`type SegmentIterator = MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-section"></span>`type Section = MachOSection<'data, 'file, Mach, R>`

- <span id="machofile-type-sectioniterator"></span>`type SectionIterator = MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-comdat"></span>`type Comdat = MachOComdat<'data, 'file, Mach, R>`

- <span id="machofile-type-comdatiterator"></span>`type ComdatIterator = MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-symbol"></span>`type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machofile-type-symboliterator"></span>`type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-symboltable"></span>`type SymbolTable = MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machofile-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="machofile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md)

- <span id="machofile-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md)

- <span id="machofile-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="machofile-is-64"></span>`fn is_64(&self) -> bool`

- <span id="machofile-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- <span id="machofile-segments"></span>`fn segments(&self) -> MachOSegmentIterator<'data, '_, Mach, R>` — [`MachOSegmentIterator`](#machosegmentiterator)

- <span id="machofile-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<MachOSection<'data, 'file, Mach, R>>` — [`MachOSection`](#machosection)

- <span id="machofile-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<MachOSection<'data, '_, Mach, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`MachOSection`](#machosection)

- <span id="machofile-sections"></span>`fn sections(&self) -> MachOSectionIterator<'data, '_, Mach, R>` — [`MachOSectionIterator`](#machosectioniterator)

- <span id="machofile-comdats"></span>`fn comdats(&self) -> MachOComdatIterator<'data, '_, Mach, R>` — [`MachOComdatIterator`](#machocomdatiterator)

- <span id="machofile-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<MachOSymbol<'data, '_, Mach, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`MachOSymbol`](#machosymbol)

- <span id="machofile-symbols"></span>`fn symbols(&self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](#machosymboliterator)

- <span id="machofile-symbol-table"></span>`fn symbol_table(&self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](#machosymboltable)

- <span id="machofile-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](#machosymboliterator)

- <span id="machofile-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](#machosymboltable)

- <span id="machofile-object-map"></span>`fn object_map(&self) -> ObjectMap<'data>` — [`ObjectMap`](../../index.md)

- <span id="machofile-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- <span id="machofile-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- <span id="machofile-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../index.md)

- <span id="machofile-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="machofile-mach-uuid"></span>`fn mach_uuid(&self) -> Result<Option<[u8; 16]>>` — [`Result`](../../index.md)

- <span id="machofile-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="machofile-entry"></span>`fn entry(&self) -> u64`

- <span id="machofile-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md)

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

*Defined in [`object-0.37.3/src/read/macho/file.rs:527-534`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L527-L534)*

An iterator for the COMDAT section groups in a [`MachOFile`](#machofile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machocomdatiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machocomdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-type-item"></span>`type Item = MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `MachOComdat<'data, 'file, Mach, R>`

```rust
struct MachOComdat<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:561-568`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L561-L568)*

A COMDAT section group in a [`MachOFile`](#machofile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectComdat for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-type-sectioniterator"></span>`type SectionIterator = MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- <span id="machocomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- <span id="machocomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="machocomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="machocomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

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

*Defined in [`object-0.37.3/src/read/macho/file.rs:621-628`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L621-L628)*

An iterator for the sections in a COMDAT section group in a [`MachOFile`](#machofile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machocomdatsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machocomdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-type-item"></span>`type Item = SectionIndex`

- <span id="machocomdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LoadCommandIterator<'data, E: Endian>`

```rust
struct LoadCommandIterator<'data, E: Endian> {
    endian: E,
    data: crate::read::Bytes<'data>,
    ncmds: u32,
}
```

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:12-16`](../../../../.source_1765210505/object-0.37.3/src/read/macho/load_command.rs#L12-L16)*

An iterator for the load commands from a [`MachHeader`](#machheader).

#### Implementations

- <span id="loadcommanditerator-new"></span>`fn new(endian: E, data: &'data [u8], ncmds: u32) -> Self`

- <span id="loadcommanditerator-next"></span>`fn next(&mut self) -> Result<Option<LoadCommandData<'data, E>>>` — [`Result`](../../index.md), [`LoadCommandData`](#loadcommanddata)

- <span id="loadcommanditerator-parse"></span>`fn parse(&mut self) -> Result<LoadCommandData<'data, E>>` — [`Result`](../../index.md), [`LoadCommandData`](#loadcommanddata)

#### Trait Implementations

##### `impl<'data, E: clone::Clone + Endian> Clone for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-clone"></span>`fn clone(&self) -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](#loadcommanditerator)

##### `impl<'data, E: marker::Copy + Endian> Copy for LoadCommandIterator<'data, E>`

##### `impl<'data, E: fmt::Debug + Endian> Debug for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, E: default::Default + Endian> Default for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-default"></span>`fn default() -> LoadCommandIterator<'data, E>` — [`LoadCommandIterator`](#loadcommanditerator)

##### `impl<I> IntoIterator for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="loadcommanditerator-type-intoiter"></span>`type IntoIter = I`

- <span id="loadcommanditerator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, E: Endian> Iterator for LoadCommandIterator<'data, E>`

- <span id="loadcommanditerator-type-item"></span>`type Item = Result<LoadCommandData<'data, E>, Error>`

- <span id="loadcommanditerator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LoadCommandData<'data, E: Endian>`

```rust
struct LoadCommandData<'data, E: Endian> {
    cmd: u32,
    data: crate::read::Bytes<'data>,
    marker: core::marker::PhantomData<E>,
}
```

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:74-79`](../../../../.source_1765210505/object-0.37.3/src/read/macho/load_command.rs#L74-L79)*

The data for a [`macho::LoadCommand`](../../macho/index.md).

#### Implementations

- <span id="loadcommanddata-cmd"></span>`fn cmd(&self) -> u32`

- <span id="loadcommanddata-cmdsize"></span>`fn cmdsize(&self) -> u32`

- <span id="loadcommanddata-data"></span>`fn data<T: Pod>(&self) -> Result<&'data T>` — [`Result`](../../index.md)

- <span id="loadcommanddata-raw-data"></span>`fn raw_data(&self) -> &'data [u8]`

- <span id="loadcommanddata-string"></span>`fn string(&self, endian: E, s: macho::LcStr<E>) -> Result<&'data [u8]>` — [`LcStr`](../../macho/index.md), [`Result`](../../index.md)

- <span id="loadcommanddata-variant"></span>`fn variant(&self) -> Result<LoadCommandVariant<'data, E>>` — [`Result`](../../index.md), [`LoadCommandVariant`](#loadcommandvariant)

- <span id="loadcommanddata-segment-32"></span>`fn segment_32(self) -> Result<Option<(&'data macho::SegmentCommand32<E>, &'data [u8])>>` — [`Result`](../../index.md), [`SegmentCommand32`](../../macho/index.md)

- <span id="loadcommanddata-symtab"></span>`fn symtab(self) -> Result<Option<&'data macho::SymtabCommand<E>>>` — [`Result`](../../index.md), [`SymtabCommand`](../../macho/index.md)

- <span id="loadcommanddata-dysymtab"></span>`fn dysymtab(self) -> Result<Option<&'data macho::DysymtabCommand<E>>>` — [`Result`](../../index.md), [`DysymtabCommand`](../../macho/index.md)

- <span id="loadcommanddata-dylib"></span>`fn dylib(self) -> Result<Option<&'data macho::DylibCommand<E>>>` — [`Result`](../../index.md), [`DylibCommand`](../../macho/index.md)

- <span id="loadcommanddata-uuid"></span>`fn uuid(self) -> Result<Option<&'data macho::UuidCommand<E>>>` — [`Result`](../../index.md), [`UuidCommand`](../../macho/index.md)

- <span id="loadcommanddata-segment-64"></span>`fn segment_64(self) -> Result<Option<(&'data macho::SegmentCommand64<E>, &'data [u8])>>` — [`Result`](../../index.md), [`SegmentCommand64`](../../macho/index.md)

- <span id="loadcommanddata-dyld-info"></span>`fn dyld_info(self) -> Result<Option<&'data macho::DyldInfoCommand<E>>>` — [`Result`](../../index.md), [`DyldInfoCommand`](../../macho/index.md)

- <span id="loadcommanddata-entry-point"></span>`fn entry_point(self) -> Result<Option<&'data macho::EntryPointCommand<E>>>` — [`Result`](../../index.md), [`EntryPointCommand`](../../macho/index.md)

- <span id="loadcommanddata-build-version"></span>`fn build_version(self) -> Result<Option<&'data macho::BuildVersionCommand<E>>>` — [`Result`](../../index.md), [`BuildVersionCommand`](../../macho/index.md)

#### Trait Implementations

##### `impl<'data, E: clone::Clone + Endian> Clone for LoadCommandData<'data, E>`

- <span id="loadcommanddata-clone"></span>`fn clone(&self) -> LoadCommandData<'data, E>` — [`LoadCommandData`](#loadcommanddata)

##### `impl<'data, E: marker::Copy + Endian> Copy for LoadCommandData<'data, E>`

##### `impl<'data, E: fmt::Debug + Endian> Debug for LoadCommandData<'data, E>`

- <span id="loadcommanddata-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/macho/segment.rs:20-27`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L20-L27)*

An iterator for the segments in a [`MachOFile`](#machofile).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosegmentiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosegmentiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-type-item"></span>`type Item = MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/macho/segment.rs:55-62`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L55-L62)*

A segment in a [`MachOFile`](#machofile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- <span id="machosegment-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](#machofile)

- <span id="machosegment-macho-segment"></span>`fn macho_segment(&self) -> &'data <Mach as >::Segment` — [`MachHeader`](#machheader)

- <span id="machosegment-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSegment for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-address"></span>`fn address(&self) -> u64`

- <span id="machosegment-size"></span>`fn size(&self) -> u64`

- <span id="machosegment-align"></span>`fn align(&self) -> u64`

- <span id="machosegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="machosegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="machosegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="machosegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="machosegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="machosegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSegment<'data, 'file, Mach, R>`

### `MachOSegmentInternal<'data, Mach: MachHeader, R: ReadRef<'data>>`

```rust
struct MachOSegmentInternal<'data, Mach: MachHeader, R: ReadRef<'data>> {
    pub segment: &'data <Mach as >::Segment,
    pub data: R,
}
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:161-168`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L161-L168)*

#### Fields

- **`data`**: `R`

  The data for the file that contains the segment data.
  
  This is required for dyld caches, where this may be a different subcache
  from the file containing the Mach-O load commands.

#### Trait Implementations

##### `impl<'data, Mach: clone::Clone + MachHeader, R: clone::Clone + ReadRef<'data>> Clone for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-clone"></span>`fn clone(&self) -> MachOSegmentInternal<'data, Mach, R>` — [`MachOSegmentInternal`](segment/index.md)

##### `impl<'data, Mach: marker::Copy + MachHeader, R: marker::Copy + ReadRef<'data>> Copy for MachOSegmentInternal<'data, Mach, R>`

##### `impl<'data, Mach: fmt::Debug + MachHeader, R: fmt::Debug + ReadRef<'data>> Debug for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`object-0.37.3/src/read/macho/section.rs:22-29`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L22-L29)*

An iterator for the sections in a [`MachOFile`](#machofile).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-type-item"></span>`type Item = MachOSection<'data, 'file, Mach, R>`

- <span id="machosectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/macho/section.rs:68-75`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L68-L75)*

A section in a [`MachOFile`](#machofile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="machosection-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](#machofile)

- <span id="machosection-macho-section"></span>`fn macho_section(&self) -> &'data <Mach as >::Section` — [`MachHeader`](#machheader)

- <span id="machosection-macho-relocations"></span>`fn macho_relocations(&self) -> Result<&'data [macho::Relocation<<Mach as >::Endian>]>` — [`Result`](../../index.md), [`Relocation`](../../macho/index.md), [`MachHeader`](#machheader)

- <span id="machosection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="machosection-maybe-compressed-gnu"></span>`fn maybe_compressed_gnu(&self) -> Result<Option<CompressedFileRange>>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSection for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-type-relocationiterator"></span>`type RelocationIterator = MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machosection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- <span id="machosection-address"></span>`fn address(&self) -> u64`

- <span id="machosection-size"></span>`fn size(&self) -> u64`

- <span id="machosection-align"></span>`fn align(&self) -> u64`

- <span id="machosection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="machosection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="machosection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="machosection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- <span id="machosection-compressed-data"></span>`fn compressed_data(&self) -> read::Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- <span id="machosection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="machosection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="machosection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="machosection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="machosection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md)

- <span id="machosection-relocations"></span>`fn relocations(&self) -> MachORelocationIterator<'data, 'file, Mach, R>` — [`MachORelocationIterator`](#machorelocationiterator)

- <span id="machosection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- <span id="machosection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSection<'data, 'file, Mach, R>`

### `MachOSectionInternal<'data, Mach: MachHeader, R: ReadRef<'data>>`

```rust
struct MachOSectionInternal<'data, Mach: MachHeader, R: ReadRef<'data>> {
    pub index: crate::read::SectionIndex,
    pub kind: crate::read::SectionKind,
    pub section: &'data <Mach as >::Section,
    pub data: R,
}
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:241-250`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L241-L250)*

#### Fields

- **`data`**: `R`

  The data for the file that contains the section data.
  
  This is required for dyld caches, where this may be a different subcache
  from the file containing the Mach-O load commands.

#### Implementations

- <span id="machosectioninternal-parse"></span>`fn parse(index: SectionIndex, section: &'data <Mach as >::Section, data: R) -> Self` — [`SectionIndex`](../../index.md), [`MachHeader`](#machheader)

#### Trait Implementations

##### `impl<'data, Mach: clone::Clone + MachHeader, R: clone::Clone + ReadRef<'data>> Clone for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-clone"></span>`fn clone(&self) -> MachOSectionInternal<'data, Mach, R>` — [`MachOSectionInternal`](section/index.md)

##### `impl<'data, Mach: marker::Copy + MachHeader, R: marker::Copy + ReadRef<'data>> Copy for MachOSectionInternal<'data, Mach, R>`

##### `impl<'data, Mach: fmt::Debug + MachHeader, R: fmt::Debug + ReadRef<'data>> Debug for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SymbolTable<'data, Mach: MachHeader, R>`

```rust
struct SymbolTable<'data, Mach: MachHeader, R>
where
    R: ReadRef<'data> {
    symbols: &'data [<Mach as >::Nlist],
    strings: crate::read::util::StringTable<'data, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:23-29`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L23-L29)*

A table of symbol entries in a Mach-O file.

Also includes the string table used for the symbol names.

Returned by `macho::SymtabCommand::symbols`.

#### Implementations

- <span id="symboltable-new"></span>`fn new(symbols: &'data [<Mach as >::Nlist], strings: StringTable<'data, R>) -> Self` — [`MachHeader`](#machheader), [`StringTable`](../index.md)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- <span id="symboltable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Mach as >::Nlist>` — [`MachHeader`](#machheader)

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Mach as >::Nlist>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`MachHeader`](#machheader)

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Mach as >::Nlist) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../index.md)

- <span id="symboltable-object-map"></span>`fn object_map(&self, endian: <Mach as >::Endian) -> ObjectMap<'data>` — [`MachHeader`](#machheader), [`ObjectMap`](../../index.md)

#### Trait Implementations

##### `impl<'data, Mach: clone::Clone + MachHeader, R> Clone for SymbolTable<'data, Mach, R>`

- <span id="symboltable-clone"></span>`fn clone(&self) -> SymbolTable<'data, Mach, R>` — [`SymbolTable`](#symboltable)

##### `impl<'data, Mach: marker::Copy + MachHeader, R> Copy for SymbolTable<'data, Mach, R>`

##### `impl<'data, Mach: fmt::Debug + MachHeader, R> Debug for SymbolTable<'data, Mach, R>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Mach: MachHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Mach, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `MachOSymbolTable<'data, 'file, Mach, R>`

```rust
struct MachOSymbolTable<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:184-190`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L184-L190)*

A symbol table in a [`MachOFile`](#machofile).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-clone"></span>`fn clone(&self) -> MachOSymbolTable<'data, 'file, Mach, R>` — [`MachOSymbolTable`](#machosymboltable)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbolTable<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbolTable for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-type-symbol"></span>`type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymboltable-type-symboliterator"></span>`type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- <span id="machosymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

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

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:225-232`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L225-L232)*

An iterator for the symbols in a [`MachOFile`](#machofile).

#### Implementations

- <span id="machosymboliterator-new"></span>`fn new(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](#machofile)

- <span id="machosymboliterator-empty"></span>`fn empty(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](#machofile)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosymboliterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosymboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-type-item"></span>`type Item = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:294-302`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L294-L302)*

A symbol in a [`MachOFile`](#machofile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- <span id="machosymbol-new"></span>`fn new(file: &'file MachOFile<'data, Mach, R>, index: SymbolIndex, nlist: &'data <Mach as >::Nlist) -> Option<Self>` — [`MachOFile`](#machofile), [`SymbolIndex`](../../index.md), [`MachHeader`](#machheader)

- <span id="machosymbol-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](#machofile)

- <span id="machosymbol-macho-symbol"></span>`fn macho_symbol(&self) -> &'data <Mach as >::Nlist` — [`MachHeader`](#machheader)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-clone"></span>`fn clone(&self) -> MachOSymbol<'data, 'file, Mach, R>` — [`MachOSymbol`](#machosymbol)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbol<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbol for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- <span id="machosymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="machosymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md)

- <span id="machosymbol-address"></span>`fn address(&self) -> u64`

- <span id="machosymbol-size"></span>`fn size(&self) -> u64`

- <span id="machosymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md)

- <span id="machosymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md)

- <span id="machosymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="machosymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="machosymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="machosymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="machosymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md)

- <span id="machosymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="machosymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="machosymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md), [`SectionIndex`](../../index.md), [`SymbolIndex`](../../index.md)

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

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:20-27`](../../../../.source_1765210505/object-0.37.3/src/read/macho/relocation.rs#L20-L27)*

An iterator for the relocations in a [`MachOSection`](super::MachOSection).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machorelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machorelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="machorelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `DyldSubCacheSlice<'data, E: Endian>`

```rust
enum DyldSubCacheSlice<'data, E: Endian> {
    V1(&'data [macho::DyldSubCacheEntryV1<E>]),
    V2(&'data [macho::DyldSubCacheEntryV2<E>]),
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:31-36`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L31-L36)*

A slice of structs describing each subcache.

The struct gained an additional field (the file suffix) in dyld-1042.1 (macOS 13 / iOS 16),
so this is an enum of the two possible slice types.

#### Variants

- **`V1`**

  V1, used between dyld-940 and dyld-1042.1.

- **`V2`**

  V2, used since dyld-1042.1.

#### Trait Implementations

##### `impl<'data, E: clone::Clone + Endian> Clone for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-clone"></span>`fn clone(&self) -> DyldSubCacheSlice<'data, E>` — [`DyldSubCacheSlice`](#dyldsubcacheslice)

##### `impl<'data, E: marker::Copy + Endian> Copy for DyldSubCacheSlice<'data, E>`

##### `impl<'data, E: fmt::Debug + Endian> Debug for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheMappingSlice<'data, E: Endian>`

```rust
enum DyldCacheMappingSlice<'data, E: Endian> {
    V1(&'data [macho::DyldCacheMappingInfo<E>]),
    V2(&'data [macho::DyldCacheMappingAndSlideInfo<E>]),
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:331-336`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L331-L336)*

The array of mappings for a single dyld cache file.

The mappings gained slide info in dyld-832.7 (macOS 11)
so this is an enum of the two possible slice types.

#### Variants

- **`V1`**

  V1, used before dyld-832.7.

- **`V2`**

  V2, used since dyld-832.7.

#### Trait Implementations

##### `impl<'data, E: clone::Clone + Endian> Clone for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-clone"></span>`fn clone(&self) -> DyldCacheMappingSlice<'data, E>` — [`DyldCacheMappingSlice`](#dyldcachemappingslice)

##### `impl<'data, E: marker::Copy + Endian> Copy for DyldCacheMappingSlice<'data, E>`

##### `impl<'data, E: fmt::Debug + Endian> Debug for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheMappingVersionIterator<'data, E>`

```rust
enum DyldCacheMappingVersionIterator<'data, E>
where
    E: Endian {
    V1(slice::Iter<'data, macho::DyldCacheMappingInfo<E>>),
    V2(slice::Iter<'data, macho::DyldCacheMappingAndSlideInfo<E>>),
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:354-360`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L354-L360)*

#### Trait Implementations

##### `impl<'data, E> Debug for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheMappingVersion<'data, E>`

```rust
enum DyldCacheMappingVersion<'data, E>
where
    E: Endian {
    V1(&'data macho::DyldCacheMappingInfo<E>),
    V2(&'data macho::DyldCacheMappingAndSlideInfo<E>),
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:395-401`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L395-L401)*

#### Trait Implementations

##### `impl<'data, E> Clone for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-clone"></span>`fn clone(&self) -> DyldCacheMappingVersion<'data, E>` — [`DyldCacheMappingVersion`](dyld_cache/index.md)

##### `impl<'data, E> Copy for DyldCacheMappingVersion<'data, E>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:539-554`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L539-L554)*

The slide info for a dyld cache mapping, including variable length arrays.

#### Trait Implementations

##### `impl<'data, E: clone::Clone + Endian> Clone for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-clone"></span>`fn clone(&self) -> DyldCacheSlideInfo<'data, E>` — [`DyldCacheSlideInfo`](#dyldcacheslideinfo)

##### `impl<'data, E: marker::Copy + Endian> Copy for DyldCacheSlideInfo<'data, E>`

##### `impl<'data, E: fmt::Debug + Endian> Debug for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheRelocationIteratorVersion<'data, E, R>`

```rust
enum DyldCacheRelocationIteratorVersion<'data, E, R>
where
    E: Endian,
    R: ReadRef<'data> {
    None,
    V2(DyldCacheRelocationIteratorV2<'data, E, R>),
    V3(DyldCacheRelocationIteratorV3<'data, E, R>),
    V5(DyldCacheRelocationIteratorV5<'data, E, R>),
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:585-594`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L585-L594)*

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RelocationStateV2`

```rust
enum RelocationStateV2 {
    Start,
    Extra,
    Page,
    PageExtra,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:597-602`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L597-L602)*

#### Trait Implementations

##### `impl Clone for RelocationStateV2`

- <span id="relocationstatev2-clone"></span>`fn clone(&self) -> RelocationStateV2` — [`RelocationStateV2`](dyld_cache/index.md)

##### `impl Copy for RelocationStateV2`

##### `impl Debug for RelocationStateV2`

- <span id="relocationstatev2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV2`

##### `impl PartialEq for RelocationStateV2`

- <span id="relocationstatev2-eq"></span>`fn eq(&self, other: &RelocationStateV2) -> bool` — [`RelocationStateV2`](dyld_cache/index.md)

##### `impl StructuralPartialEq for RelocationStateV2`

### `RelocationStateV3`

```rust
enum RelocationStateV3 {
    Start,
    Page,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:707-710`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L707-L710)*

#### Trait Implementations

##### `impl Clone for RelocationStateV3`

- <span id="relocationstatev3-clone"></span>`fn clone(&self) -> RelocationStateV3` — [`RelocationStateV3`](dyld_cache/index.md)

##### `impl Copy for RelocationStateV3`

##### `impl Debug for RelocationStateV3`

- <span id="relocationstatev3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV3`

##### `impl PartialEq for RelocationStateV3`

- <span id="relocationstatev3-eq"></span>`fn eq(&self, other: &RelocationStateV3) -> bool` — [`RelocationStateV3`](dyld_cache/index.md)

##### `impl StructuralPartialEq for RelocationStateV3`

### `RelocationStateV5`

```rust
enum RelocationStateV5 {
    Start,
    Page,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:804-807`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L804-L807)*

#### Trait Implementations

##### `impl Clone for RelocationStateV5`

- <span id="relocationstatev5-clone"></span>`fn clone(&self) -> RelocationStateV5` — [`RelocationStateV5`](dyld_cache/index.md)

##### `impl Copy for RelocationStateV5`

##### `impl Debug for RelocationStateV5`

- <span id="relocationstatev5-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV5`

##### `impl PartialEq for RelocationStateV5`

- <span id="relocationstatev5-eq"></span>`fn eq(&self, other: &RelocationStateV5) -> bool` — [`RelocationStateV5`](dyld_cache/index.md)

##### `impl StructuralPartialEq for RelocationStateV5`

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

*Defined in [`object-0.37.3/src/read/macho/load_command.rs:280-360`](../../../../.source_1765210505/object-0.37.3/src/read/macho/load_command.rs#L280-L360)*

A [`macho::LoadCommand`](../../macho/index.md) that has been interpreted according to its `cmd` field.

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

##### `impl<'data, E: clone::Clone + Endian> Clone for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-clone"></span>`fn clone(&self) -> LoadCommandVariant<'data, E>` — [`LoadCommandVariant`](#loadcommandvariant)

##### `impl<'data, E: marker::Copy + Endian> Copy for LoadCommandVariant<'data, E>`

##### `impl<'data, E: fmt::Debug + Endian> Debug for LoadCommandVariant<'data, E>`

- <span id="loadcommandvariant-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `FatArch`

```rust
trait FatArch: Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:59-90`](../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L59-L90)*

A trait for generic access to [`macho::FatArch32`](../../macho/index.md) and [`macho::FatArch64`](../../macho/index.md).

#### Associated Types

- `type Word: 1`

#### Associated Constants

- `const MAGIC: u32`

#### Required Methods

- `fn cputype(&self) -> u32`

- `fn cpusubtype(&self) -> u32`

- `fn offset(&self) -> <Self as >::Word`

- `fn size(&self) -> <Self as >::Word`

- `fn align(&self) -> u32`

#### Provided Methods

- `fn architecture(&self) -> Architecture`

- `fn file_range(&self) -> (u64, u64)`

- `fn data<'data, R: ReadRef<'data>>(&self, file: R) -> Result<&'data [u8]>`

#### Implementors

- [`FatArch32`](../../macho/index.md)
- [`FatArch64`](../../macho/index.md)

### `MachHeader`

```rust
trait MachHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:644-723`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L644-L723)*

A trait for generic access to [`macho::MachHeader32`](../../macho/index.md) and [`macho::MachHeader64`](../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

- `type Segment: 1`

- `type Section: 1`

- `type Nlist: 1`

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_big_endian(&self) -> bool`

  Return true if the `magic` field signifies big-endian.

- `fn is_little_endian(&self) -> bool`

  Return true if the `magic` field signifies little-endian.

- `fn magic(&self) -> u32`

- `fn cputype(&self, endian: <Self as >::Endian) -> u32`

- `fn cpusubtype(&self, endian: <Self as >::Endian) -> u32`

- `fn filetype(&self, endian: <Self as >::Endian) -> u32`

- `fn ncmds(&self, endian: <Self as >::Endian) -> u32`

- `fn sizeofcmds(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

- `fn endian(&self) -> Result<<Self as >::Endian>`

- `fn load_commands<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<LoadCommandIterator<'data, <Self as >::Endian>>`

- `fn uuid<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<Option<[u8; 16]>>`

  Return the UUID from the `LC_UUID` load command, if one is present.

#### Implementors

- [`MachHeader32`](../../macho/index.md)
- [`MachHeader64`](../../macho/index.md)

### `Segment`

```rust
trait Segment: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:172-229`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L172-L229)*

A trait for generic access to [`macho::SegmentCommand32`](../../macho/index.md) and [`macho::SegmentCommand64`](../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

- `type Section: 1`

#### Required Methods

- `fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>`

- `fn cmd(&self, endian: <Self as >::Endian) -> u32`

- `fn cmdsize(&self, endian: <Self as >::Endian) -> u32`

- `fn segname(&self) -> &[u8; 16]`

- `fn vmaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn vmsize(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn fileoff(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn filesize(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn maxprot(&self, endian: <Self as >::Endian) -> u32`

- `fn initprot(&self, endian: <Self as >::Endian) -> u32`

- `fn nsects(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(&self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Get the segment data from the file data.

- `fn sections<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, section_data: R) -> Result<&'data [<Self as >::Section]>`

  Get the array of sections from the data following the segment command.

#### Implementors

- [`SegmentCommand32`](../../macho/index.md)
- [`SegmentCommand64`](../../macho/index.md)

### `Section`

```rust
trait Section: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:285-354`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L285-L354)*

A trait for generic access to [`macho::Section32`](../../macho/index.md) and [`macho::Section64`](../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn sectname(&self) -> &[u8; 16]`

- `fn segname(&self) -> &[u8; 16]`

- `fn addr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn offset(&self, endian: <Self as >::Endian) -> u32`

- `fn align(&self, endian: <Self as >::Endian) -> u32`

- `fn reloff(&self, endian: <Self as >::Endian) -> u32`

- `fn nreloc(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the `sectname` bytes up until the null terminator.

- `fn segment_name(&self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(&self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

- `fn relocations<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> Result<&'data [macho::Relocation<<Self as >::Endian>]>`

  Return the relocation array.

#### Implementors

- [`Section32`](../../macho/index.md)
- [`Section64`](../../macho/index.md)

### `Nlist`

```rust
trait Nlist: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:457-504`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L457-L504)*

A trait for generic access to [`macho::Nlist32`](../../macho/index.md) and [`macho::Nlist64`](../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn n_strx(&self, endian: <Self as >::Endian) -> u32`

- `fn n_type(&self) -> u8`

- `fn n_sect(&self) -> u8`

- `fn n_desc(&self, endian: <Self as >::Endian) -> u16`

- `fn n_value(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

- `fn is_stab(&self) -> bool`

  Return true if this is a STAB symbol.

- `fn is_undefined(&self) -> bool`

  Return true if this is an undefined symbol.

- `fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn library_ordinal(&self, endian: <Self as >::Endian) -> u8`

  Return the library ordinal.

#### Implementors

- [`Nlist32`](../../macho/index.md)
- [`Nlist64`](../../macho/index.md)

## Type Aliases

### `MachOFatFile32<'data>`

```rust
type MachOFatFile32<'data> = MachOFatFile<'data, macho::FatArch32>;
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:12`](../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L12)*

A 32-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat32`](../../index.md).

### `MachOFatFile64<'data>`

```rust
type MachOFatFile64<'data> = MachOFatFile<'data, macho::FatArch64>;
```

*Defined in [`object-0.37.3/src/read/macho/fat.rs:18`](../../../../.source_1765210505/object-0.37.3/src/read/macho/fat.rs#L18)*

A 64-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`](../../macho/index.md), and corresponds
to [`crate::FileKind::MachOFat64`](../../index.md).

### `MachOFile32<'data, Endian, R>`

```rust
type MachOFile32<'data, Endian, R> = MachOFile<'data, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:24-25`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L24-L25)*

A 32-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader32`](../../macho/index.md), and corresponds
to [`crate::FileKind::MachO32`](../../index.md).

### `MachOFile64<'data, Endian, R>`

```rust
type MachOFile64<'data, Endian, R> = MachOFile<'data, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:30-31`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L30-L31)*

A 64-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader64`](../../macho/index.md), and corresponds
to [`crate::FileKind::MachO64`](../../index.md).

### `MachOComdatIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator32<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:517-518`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L517-L518)*

An iterator for the COMDAT section groups in a [`MachOFile64`](#machofile64).

### `MachOComdatIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator64<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:520-521`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L520-L521)*

An iterator for the COMDAT section groups in a [`MachOFile64`](#machofile64).

### `MachOComdat32<'data, 'file, Endian, R>`

```rust
type MachOComdat32<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:550-551`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L550-L551)*

A COMDAT section group in a [`MachOFile32`](#machofile32).

### `MachOComdat64<'data, 'file, Endian, R>`

```rust
type MachOComdat64<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:554-555`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L554-L555)*

A COMDAT section group in a [`MachOFile64`](#machofile64).

### `MachOComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator32<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:611-612`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L611-L612)*

An iterator for the sections in a COMDAT section group in a [`MachOFile32`](#machofile32).

### `MachOComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator64<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:614-615`](../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L614-L615)*

An iterator for the sections in a COMDAT section group in a [`MachOFile64`](#machofile64).

### `MachOSegmentIterator32<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator32<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:12-13`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L12-L13)*

An iterator for the segments in a [`MachOFile32`](super::MachOFile32).

### `MachOSegmentIterator64<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator64<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:15-16`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L15-L16)*

An iterator for the segments in a [`MachOFile64`](super::MachOFile64).

### `MachOSegment32<'data, 'file, Endian, R>`

```rust
type MachOSegment32<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:45-46`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L45-L46)*

A segment in a [`MachOFile32`](super::MachOFile32).

### `MachOSegment64<'data, 'file, Endian, R>`

```rust
type MachOSegment64<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:48-49`](../../../../.source_1765210505/object-0.37.3/src/read/macho/segment.rs#L48-L49)*

A segment in a [`MachOFile64`](super::MachOFile64).

### `MachOSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator32<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:15-16`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L15-L16)*

An iterator for the sections in a [`MachOFile32`](super::MachOFile32).

### `MachOSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator64<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:18-19`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L18-L19)*

An iterator for the sections in a [`MachOFile64`](super::MachOFile64).

### `MachOSection32<'data, 'file, Endian, R>`

```rust
type MachOSection32<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:58-59`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L58-L59)*

A section in a [`MachOFile32`](super::MachOFile32).

### `MachOSection64<'data, 'file, Endian, R>`

```rust
type MachOSection64<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:61-62`](../../../../.source_1765210505/object-0.37.3/src/read/macho/section.rs#L61-L62)*

A section in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbolTable32<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable32<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:176-177`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L176-L177)*

A symbol table in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolTable64<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable64<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:179-180`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L179-L180)*

A symbol table in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbolIterator32<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator32<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:218-219`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L218-L219)*

An iterator for the symbols in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolIterator64<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator64<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:221-222`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L221-L222)*

An iterator for the symbols in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbol32<'data, 'file, Endian, R>`

```rust
type MachOSymbol32<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:284-285`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L284-L285)*

A symbol in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbol64<'data, 'file, Endian, R>`

```rust
type MachOSymbol64<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:287-288`](../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L287-L288)*

A symbol in a [`MachOFile64`](super::MachOFile64).

### `MachORelocationIterator32<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator32<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:13-14`](../../../../.source_1765210505/object-0.37.3/src/read/macho/relocation.rs#L13-L14)*

An iterator for the relocations in a [`MachOSection32`](super::MachOSection32).

### `MachORelocationIterator64<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator64<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:16-17`](../../../../.source_1765210505/object-0.37.3/src/read/macho/relocation.rs#L16-L17)*

An iterator for the relocations in a [`MachOSection64`](super::MachOSection64).

## Constants

### `MIN_HEADER_SIZE_SUBCACHES_V1`
```rust
const MIN_HEADER_SIZE_SUBCACHES_V1: u32 = 456u32;
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:39`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L39)*

### `MIN_HEADER_SIZE_SUBCACHES_V2`
```rust
const MIN_HEADER_SIZE_SUBCACHES_V2: u32 = 464u32;
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:42`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L42)*

### `MIN_HEADER_SIZE_MAPPINGS_V2`
```rust
const MIN_HEADER_SIZE_MAPPINGS_V2: u32 = 320u32;
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:339`](../../../../.source_1765210505/object-0.37.3/src/read/macho/dyld_cache.rs#L339)*

