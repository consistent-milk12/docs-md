*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [dyld_cache](index.md)*

---

# Module `dyld_cache`

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

- `fn subcache_suffixes(data: R) -> Result<Vec<String>>` — [`Result`](../../../index.md)

- `fn parse(data: R, subcache_data: &[R]) -> Result<Self>` — [`Result`](../../../index.md)

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../../index.md)

- `fn endianness(self: &Self) -> Endianness` — [`Endianness`](../../../index.md)

- `fn data(self: &Self) -> R`

- `fn is_little_endian(self: &Self) -> bool`

- `fn images<'cache>(self: &'cache Self) -> DyldCacheImageIterator<'data, 'cache, E, R>` — [`DyldCacheImageIterator`](../index.md)

- `fn mappings<'cache>(self: &'cache Self) -> impl Iterator<Item = DyldCacheMapping<'data, E, R>> + 'cache` — [`DyldCacheMapping`](../index.md)

- `fn data_and_offset_for_address(self: &Self, address: u64) -> Option<(R, u64)>`

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCache<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

The data for one file in the cache.

#### Implementations

- `fn mappings(self: &Self, endian: E) -> DyldCacheMappingIterator<'data, E, R>` — [`DyldCacheMappingIterator`](../index.md)

- `fn address_to_file_offset(self: &Self, endian: E, address: u64) -> Option<u64>`

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldFile<'data, E, R>`

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

- `fn next(self: &mut Self) -> Option<DyldCacheImage<'data, 'cache, E, R>>` — [`DyldCacheImage`](../index.md)

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

- `fn info(self: &Self) -> &'data macho::DyldCacheImageInfo<E>` — [`DyldCacheImageInfo`](../../../macho/index.md)

- `fn path(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

- `fn image_data_and_offset(self: &Self) -> Result<(R, u64)>` — [`Result`](../../../index.md)

- `fn parse_object(self: &Self) -> Result<File<'data, R>>` — [`Result`](../../../index.md), [`File`](../../index.md)

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

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn relocations(self: &Self) -> Result<DyldCacheRelocationIterator<'data, E, R>>` — [`Result`](../../../index.md), [`DyldCacheRelocationIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, E, R> Clone for DyldCacheMapping<'data, E, R>`

- `fn clone(self: &Self) -> DyldCacheMapping<'data, E, R>` — [`DyldCacheMapping`](../index.md)

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

- `fn next(self: &mut Self) -> Result<Option<DyldRelocation>>` — [`Result`](../../../index.md), [`DyldRelocation`](../index.md)

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorV2<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

#### Fields

- **`start_index`**: `usize`

  Index of the page within the mapping.

- **`offset`**: `u64`

  The current offset within the mapping.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<DyldRelocation>>` — [`Result`](../../../index.md), [`DyldRelocation`](../index.md)

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorV3<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

#### Fields

- **`start_index`**: `usize`

  The next index within page_starts.

- **`offset`**: `u64`

  The current offset within the mapping.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<DyldRelocation>>` — [`Result`](../../../index.md), [`DyldRelocation`](../index.md)

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorV5<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> DyldSubCacheSlice<'data, E>` — [`DyldSubCacheSlice`](../index.md)

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

- `fn clone(self: &Self) -> DyldCacheMappingSlice<'data, E>` — [`DyldCacheMappingSlice`](../index.md)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for DyldCacheMappingSlice<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for DyldCacheMappingSlice<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldCacheMappingVersionIterator<'data, E>`

```rust
enum DyldCacheMappingVersionIterator<'data, E>
where
    E: Endian {
    V1(slice::Iter<'data, macho::DyldCacheMappingInfo<E>>),
    V2(slice::Iter<'data, macho::DyldCacheMappingAndSlideInfo<E>>),
}
```

#### Trait Implementations

##### `impl<'data, E> Debug for DyldCacheMappingVersionIterator<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldCacheMappingVersion<'data, E>`

```rust
enum DyldCacheMappingVersion<'data, E>
where
    E: Endian {
    V1(&'data macho::DyldCacheMappingInfo<E>),
    V2(&'data macho::DyldCacheMappingAndSlideInfo<E>),
}
```

#### Trait Implementations

##### `impl<'data, E> Clone for DyldCacheMappingVersion<'data, E>`

- `fn clone(self: &Self) -> DyldCacheMappingVersion<'data, E>` — [`DyldCacheMappingVersion`](#dyldcachemappingversion)

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

The slide info for a dyld cache mapping, including variable length arrays.

#### Trait Implementations

##### `impl<'data, E: $crate::clone::Clone + Endian> Clone for DyldCacheSlideInfo<'data, E>`

- `fn clone(self: &Self) -> DyldCacheSlideInfo<'data, E>` — [`DyldCacheSlideInfo`](../index.md)

##### `impl<'data, E: $crate::marker::Copy + Endian> Copy for DyldCacheSlideInfo<'data, E>`

##### `impl<'data, E: $crate::fmt::Debug + Endian> Debug for DyldCacheSlideInfo<'data, E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

#### Trait Implementations

##### `impl<'data, E, R> Debug for DyldCacheRelocationIteratorVersion<'data, E, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RelocationStateV2`

```rust
enum RelocationStateV2 {
    Start,
    Extra,
    Page,
    PageExtra,
}
```

#### Trait Implementations

##### `impl Clone for RelocationStateV2`

- `fn clone(self: &Self) -> RelocationStateV2` — [`RelocationStateV2`](#relocationstatev2)

##### `impl Copy for RelocationStateV2`

##### `impl Debug for RelocationStateV2`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationStateV2`

##### `impl PartialEq for RelocationStateV2`

- `fn eq(self: &Self, other: &RelocationStateV2) -> bool` — [`RelocationStateV2`](#relocationstatev2)

##### `impl StructuralPartialEq for RelocationStateV2`

### `RelocationStateV3`

```rust
enum RelocationStateV3 {
    Start,
    Page,
}
```

#### Trait Implementations

##### `impl Clone for RelocationStateV3`

- `fn clone(self: &Self) -> RelocationStateV3` — [`RelocationStateV3`](#relocationstatev3)

##### `impl Copy for RelocationStateV3`

##### `impl Debug for RelocationStateV3`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationStateV3`

##### `impl PartialEq for RelocationStateV3`

- `fn eq(self: &Self, other: &RelocationStateV3) -> bool` — [`RelocationStateV3`](#relocationstatev3)

##### `impl StructuralPartialEq for RelocationStateV3`

### `RelocationStateV5`

```rust
enum RelocationStateV5 {
    Start,
    Page,
}
```

#### Trait Implementations

##### `impl Clone for RelocationStateV5`

- `fn clone(self: &Self) -> RelocationStateV5` — [`RelocationStateV5`](#relocationstatev5)

##### `impl Copy for RelocationStateV5`

##### `impl Debug for RelocationStateV5`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RelocationStateV5`

##### `impl PartialEq for RelocationStateV5`

- `fn eq(self: &Self, other: &RelocationStateV5) -> bool` — [`RelocationStateV5`](#relocationstatev5)

##### `impl StructuralPartialEq for RelocationStateV5`

## Constants

### `MIN_HEADER_SIZE_SUBCACHES_V1`

```rust
const MIN_HEADER_SIZE_SUBCACHES_V1: u32 = 456u32;
```

### `MIN_HEADER_SIZE_SUBCACHES_V2`

```rust
const MIN_HEADER_SIZE_SUBCACHES_V2: u32 = 464u32;
```

### `MIN_HEADER_SIZE_MAPPINGS_V2`

```rust
const MIN_HEADER_SIZE_MAPPINGS_V2: u32 = 320u32;
```

