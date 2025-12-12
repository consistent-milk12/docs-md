*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [dyld_cache](index.md)*

---

# Module `dyld_cache`

## Contents

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
- [Constants](#constants)
  - [`MIN_HEADER_SIZE_SUBCACHES_V1`](#min-header-size-subcaches-v1)
  - [`MIN_HEADER_SIZE_SUBCACHES_V2`](#min-header-size-subcaches-v2)
  - [`MIN_HEADER_SIZE_MAPPINGS_V2`](#min-header-size-mappings-v2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
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
| [`DyldSubCacheSlice`](#dyldsubcacheslice) | enum | A slice of structs describing each subcache. |
| [`DyldCacheMappingSlice`](#dyldcachemappingslice) | enum | The array of mappings for a single dyld cache file. |
| [`DyldCacheMappingVersionIterator`](#dyldcachemappingversioniterator) | enum |  |
| [`DyldCacheMappingVersion`](#dyldcachemappingversion) | enum |  |
| [`DyldCacheSlideInfo`](#dyldcacheslideinfo) | enum | The slide info for a dyld cache mapping, including variable length arrays. |
| [`DyldCacheRelocationIteratorVersion`](#dyldcacherelocationiteratorversion) | enum |  |
| [`RelocationStateV2`](#relocationstatev2) | enum |  |
| [`RelocationStateV3`](#relocationstatev3) | enum |  |
| [`RelocationStateV5`](#relocationstatev5) | enum |  |
| [`MIN_HEADER_SIZE_SUBCACHES_V1`](#min-header-size-subcaches-v1) | const |  |
| [`MIN_HEADER_SIZE_SUBCACHES_V2`](#min-header-size-subcaches-v2) | const |  |
| [`MIN_HEADER_SIZE_MAPPINGS_V2`](#min-header-size-mappings-v2) | const |  |

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:12-23`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L12-L23)*

A parsed representation of the dyld shared cache.

#### Fields

- **`files`**: `alloc::vec::Vec<DyldFile<'data, E, R>>`

  The first entry is the main cache file, and the rest are subcaches.

#### Implementations

- <span id="dyldcache-subcache-suffixes"></span>`fn subcache_suffixes(data: R) -> Result<Vec<String>>` — [`Result`](../../../index.md#result)

- <span id="dyldcache-parse"></span>`fn parse(data: R, subcache_data: &[R]) -> Result<Self>` — [`Result`](../../../index.md#result)

- <span id="dyldcache-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

- <span id="dyldcache-endianness"></span>`fn endianness(&self) -> Endianness` — [`Endianness`](../../../index.md#endianness)

- <span id="dyldcache-data"></span>`fn data(&self) -> R`

- <span id="dyldcache-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="dyldcache-images"></span>`fn images<'cache>(self: &'cache Self) -> DyldCacheImageIterator<'data, 'cache, E, R>` — [`DyldCacheImageIterator`](../index.md#dyldcacheimageiterator)

- <span id="dyldcache-mappings"></span>`fn mappings<'cache>(self: &'cache Self) -> impl Iterator<Item = DyldCacheMapping<'data, E, R>> + 'cache` — [`DyldCacheMapping`](../index.md#dyldcachemapping)

- <span id="dyldcache-data-and-offset-for-address"></span>`fn data_and_offset_for_address(&self, address: u64) -> Option<(R, u64)>`

#### Trait Implementations

##### `impl<E, R> Debug for DyldCache<'data, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:214-221`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L214-L221)*

The data for one file in the cache.

#### Implementations

- <span id="dyldfile-mappings"></span>`fn mappings(&self, endian: E) -> DyldCacheMappingIterator<'data, E, R>` — [`DyldCacheMappingIterator`](../index.md#dyldcachemappingiterator)

- <span id="dyldfile-address-to-file-offset"></span>`fn address_to_file_offset(&self, endian: E, address: u64) -> Option<u64>`

#### Trait Implementations

##### `impl<E, R> Debug for DyldFile<'data, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:256-263`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L256-L263)*

An iterator over all the images (dylibs) in the dyld shared cache.

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcacheimageiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcacheimageiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E, R> Iterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-iterator-type-item"></span>`type Item = DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-next"></span>`fn next(&mut self) -> Option<DyldCacheImage<'data, 'cache, E, R>>` — [`DyldCacheImage`](../index.md#dyldcacheimage)

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:283-290`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L283-L290)*

One image (dylib) from inside the dyld shared cache.

#### Implementations

- <span id="dyldcacheimage-info"></span>`fn info(&self) -> &'data macho::DyldCacheImageInfo<E>` — [`DyldCacheImageInfo`](../../../macho/index.md#dyldcacheimageinfo)

- <span id="dyldcacheimage-path"></span>`fn path(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="dyldcacheimage-image-data-and-offset"></span>`fn image_data_and_offset(&self) -> Result<(R, u64)>` — [`Result`](../../../index.md#result)

- <span id="dyldcacheimage-parse-object"></span>`fn parse_object(&self) -> Result<File<'data, R>>` — [`Result`](../../../index.md#result), [`File`](../../index.md#file)

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheImage<'data, 'cache, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:343-351`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L343-L351)*

An iterator over all the mappings for one subcache in a dyld shared cache.

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcachemappingiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcachemappingiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E, R> Iterator for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-iterator-type-item"></span>`type Item = DyldCacheMapping<'data, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:384-392`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L384-L392)*

Information about a mapping.

#### Implementations

- <span id="dyldcachemapping-address"></span>`fn address(&self) -> u64`

- <span id="dyldcachemapping-size"></span>`fn size(&self) -> u64`

- <span id="dyldcachemapping-file-offset"></span>`fn file_offset(&self) -> u64`

- <span id="dyldcachemapping-max-prot"></span>`fn max_prot(&self) -> u32`

- <span id="dyldcachemapping-init-prot"></span>`fn init_prot(&self) -> u32`

- <span id="dyldcachemapping-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="dyldcachemapping-relocations"></span>`fn relocations(&self) -> Result<DyldCacheRelocationIterator<'data, E, R>>` — [`Result`](../../../index.md#result), [`DyldCacheRelocationIterator`](../index.md#dyldcacherelocationiterator)

#### Trait Implementations

##### `impl<E, R> Clone for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-clone"></span>`fn clone(&self) -> DyldCacheMapping<'data, E, R>` — [`DyldCacheMapping`](../index.md#dyldcachemapping)

##### `impl<E, R> Copy for DyldCacheMapping<'data, E, R>`

##### `impl<E, R> Debug for DyldCacheMapping<'data, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:558-564`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L558-L564)*

An iterator over relocations in a mapping

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcacherelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcacherelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E, R> Iterator for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-iterator-type-item"></span>`type Item = Result<DyldRelocation, Error>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:605-629`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L605-L629)*

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

- <span id="dyldcacherelocationiteratorv2-next"></span>`fn next(&mut self) -> Result<Option<DyldRelocation>>` — [`Result`](../../../index.md#result), [`DyldRelocation`](../index.md#dyldrelocation)

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheRelocationIteratorV2<'data, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:713-730`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L713-L730)*

#### Fields

- **`start_index`**: `usize`

  Index of the page within the mapping.

- **`offset`**: `u64`

  The current offset within the mapping.

#### Implementations

- <span id="dyldcacherelocationiteratorv3-next"></span>`fn next(&mut self) -> Result<Option<DyldRelocation>>` — [`Result`](../../../index.md#result), [`DyldRelocation`](../index.md#dyldrelocation)

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheRelocationIteratorV3<'data, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:810-827`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L810-L827)*

#### Fields

- **`start_index`**: `usize`

  The next index within page_starts.

- **`offset`**: `u64`

  The current offset within the mapping.

#### Implementations

- <span id="dyldcacherelocationiteratorv5-next"></span>`fn next(&mut self) -> Result<Option<DyldRelocation>>` — [`Result`](../../../index.md#result), [`DyldRelocation`](../index.md#dyldrelocation)

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldRelocation`

```rust
struct DyldRelocation {
    pub offset: u64,
    pub value: u64,
    pub auth: Option<DyldRelocationAuth>,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:896-906`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L896-L906)*

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:921-928`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L921-L928)*

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

## Enums

### `DyldSubCacheSlice<'data, E: Endian>`

```rust
enum DyldSubCacheSlice<'data, E: Endian> {
    V1(&'data [macho::DyldSubCacheEntryV1<E>]),
    V2(&'data [macho::DyldSubCacheEntryV2<E>]),
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:31-36`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L31-L36)*

A slice of structs describing each subcache.

The struct gained an additional field (the file suffix) in dyld-1042.1 (macOS 13 / iOS 16),
so this is an enum of the two possible slice types.

#### Variants

- **`V1`**

  V1, used between dyld-940 and dyld-1042.1.

- **`V2`**

  V2, used since dyld-1042.1.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-clone"></span>`fn clone(&self) -> DyldSubCacheSlice<'data, E>` — [`DyldSubCacheSlice`](../index.md#dyldsubcacheslice)

##### `impl<E: marker::Copy + Endian> Copy for DyldSubCacheSlice<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DyldCacheMappingSlice<'data, E: Endian>`

```rust
enum DyldCacheMappingSlice<'data, E: Endian> {
    V1(&'data [macho::DyldCacheMappingInfo<E>]),
    V2(&'data [macho::DyldCacheMappingAndSlideInfo<E>]),
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:331-336`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L331-L336)*

The array of mappings for a single dyld cache file.

The mappings gained slide info in dyld-832.7 (macOS 11)
so this is an enum of the two possible slice types.

#### Variants

- **`V1`**

  V1, used before dyld-832.7.

- **`V2`**

  V2, used since dyld-832.7.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-clone"></span>`fn clone(&self) -> DyldCacheMappingSlice<'data, E>` — [`DyldCacheMappingSlice`](../index.md#dyldcachemappingslice)

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheMappingSlice<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheMappingSlice<'data, E>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:354-360`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L354-L360)*

#### Trait Implementations

##### `impl<E> Debug for DyldCacheMappingVersionIterator<'data, E>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:395-401`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L395-L401)*

#### Trait Implementations

##### `impl<E> Clone for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-clone"></span>`fn clone(&self) -> DyldCacheMappingVersion<'data, E>` — [`DyldCacheMappingVersion`](#dyldcachemappingversion)

##### `impl<E> Copy for DyldCacheMappingVersion<'data, E>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:539-554`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L539-L554)*

The slide info for a dyld cache mapping, including variable length arrays.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-clone"></span>`fn clone(&self) -> DyldCacheSlideInfo<'data, E>` — [`DyldCacheSlideInfo`](../index.md#dyldcacheslideinfo)

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheSlideInfo<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheSlideInfo<'data, E>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:585-594`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L585-L594)*

#### Trait Implementations

##### `impl<E, R> Debug for DyldCacheRelocationIteratorVersion<'data, E, R>`

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

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:597-602`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L597-L602)*

#### Trait Implementations

##### `impl Clone for RelocationStateV2`

- <span id="relocationstatev2-clone"></span>`fn clone(&self) -> RelocationStateV2` — [`RelocationStateV2`](#relocationstatev2)

##### `impl Copy for RelocationStateV2`

##### `impl Debug for RelocationStateV2`

- <span id="relocationstatev2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV2`

##### `impl PartialEq for RelocationStateV2`

- <span id="relocationstatev2-eq"></span>`fn eq(&self, other: &RelocationStateV2) -> bool` — [`RelocationStateV2`](#relocationstatev2)

##### `impl StructuralPartialEq for RelocationStateV2`

### `RelocationStateV3`

```rust
enum RelocationStateV3 {
    Start,
    Page,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:707-710`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L707-L710)*

#### Trait Implementations

##### `impl Clone for RelocationStateV3`

- <span id="relocationstatev3-clone"></span>`fn clone(&self) -> RelocationStateV3` — [`RelocationStateV3`](#relocationstatev3)

##### `impl Copy for RelocationStateV3`

##### `impl Debug for RelocationStateV3`

- <span id="relocationstatev3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV3`

##### `impl PartialEq for RelocationStateV3`

- <span id="relocationstatev3-eq"></span>`fn eq(&self, other: &RelocationStateV3) -> bool` — [`RelocationStateV3`](#relocationstatev3)

##### `impl StructuralPartialEq for RelocationStateV3`

### `RelocationStateV5`

```rust
enum RelocationStateV5 {
    Start,
    Page,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:804-807`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L804-L807)*

#### Trait Implementations

##### `impl Clone for RelocationStateV5`

- <span id="relocationstatev5-clone"></span>`fn clone(&self) -> RelocationStateV5` — [`RelocationStateV5`](#relocationstatev5)

##### `impl Copy for RelocationStateV5`

##### `impl Debug for RelocationStateV5`

- <span id="relocationstatev5-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV5`

##### `impl PartialEq for RelocationStateV5`

- <span id="relocationstatev5-eq"></span>`fn eq(&self, other: &RelocationStateV5) -> bool` — [`RelocationStateV5`](#relocationstatev5)

##### `impl StructuralPartialEq for RelocationStateV5`

## Constants

### `MIN_HEADER_SIZE_SUBCACHES_V1`
```rust
const MIN_HEADER_SIZE_SUBCACHES_V1: u32 = 456u32;
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:39`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L39)*

### `MIN_HEADER_SIZE_SUBCACHES_V2`
```rust
const MIN_HEADER_SIZE_SUBCACHES_V2: u32 = 464u32;
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:42`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L42)*

### `MIN_HEADER_SIZE_MAPPINGS_V2`
```rust
const MIN_HEADER_SIZE_MAPPINGS_V2: u32 = 320u32;
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:339`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L339)*

