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

  Return the suffixes of the subcache files given the data of the main cache file.

  

  Each of these should be appended to the path of the main cache file.

- <span id="dyldcache-parse"></span>`fn parse(data: R, subcache_data: &[R]) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the raw dyld shared cache data.

  

  For shared caches from macOS 12 / iOS 15 and above, the subcache files need to be

  supplied as well, in the correct order. Use `Self::subcache_suffixes` to obtain

  the suffixes for the path of the files.

- <span id="dyldcache-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

  Get the architecture type of the file.

- <span id="dyldcache-endianness"></span>`fn endianness(&self) -> Endianness` — [`Endianness`](../../../index.md#endianness)

  Get the endianness of the file.

- <span id="dyldcache-data"></span>`fn data(&self) -> R`

  Get the data of the main cache file.

- <span id="dyldcache-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

  Return true if the file is little endian, false if it is big endian.

- <span id="dyldcache-images"></span>`fn images<'cache>(self: &'cache Self) -> DyldCacheImageIterator<'data, 'cache, E, R>` — [`DyldCacheImageIterator`](../index.md#dyldcacheimageiterator)

  Iterate over the images in this cache.

- <span id="dyldcache-mappings"></span>`fn mappings<'cache>(self: &'cache Self) -> impl Iterator<Item = DyldCacheMapping<'data, E, R>> + 'cache` — [`DyldCacheMapping`](../index.md#dyldcachemapping)

  Return all the mappings in this cache.

- <span id="dyldcache-data-and-offset-for-address"></span>`fn data_and_offset_for_address(&self, address: u64) -> Option<(R, u64)>`

  Find the address in a mapping and return the cache or subcache data it was found in,

  together with the translated file offset.

#### Trait Implementations

##### `impl Any for DyldCache<'data, E, R>`

- <span id="dyldcache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCache<'data, E, R>`

- <span id="dyldcache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCache<'data, E, R>`

- <span id="dyldcache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCache<'data, E, R>`

- <span id="dyldcache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCache<'data, E, R>`

- <span id="dyldcache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCache<'data, E, R>`

- <span id="dyldcache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldCache<'data, E, R>`

- <span id="dyldcache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCache<'data, E, R>`

- <span id="dyldcache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Return an iterator for the mappings.

- <span id="dyldfile-address-to-file-offset"></span>`fn address_to_file_offset(&self, endian: E, address: u64) -> Option<u64>`

  Find the file offset an address in the mappings.

#### Trait Implementations

##### `impl Any for DyldFile<'data, E, R>`

- <span id="dyldfile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldFile<'data, E, R>`

- <span id="dyldfile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldFile<'data, E, R>`

- <span id="dyldfile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldFile<'data, E, R>`

- <span id="dyldfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldFile<'data, E, R>`

- <span id="dyldfile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldFile<'data, E, R>`

- <span id="dyldfile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldFile<'data, E, R>`

- <span id="dyldfile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldfile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldFile<'data, E, R>`

- <span id="dyldfile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldfile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcacheimageiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcacheimageiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E, R> Iterator for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-iterator-type-item"></span>`type Item = DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-iterator-next"></span>`fn next(&mut self) -> Option<DyldCacheImage<'data, 'cache, E, R>>` — [`DyldCacheImage`](../index.md#dyldcacheimage)

##### `impl<U> TryFrom for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheimageiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheImageIterator<'data, 'cache, E, R>`

- <span id="dyldcacheimageiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheimageiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Return the raw data structure for this image.

- <span id="dyldcacheimage-path"></span>`fn path(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

  The file system path of this image.

- <span id="dyldcacheimage-image-data-and-offset"></span>`fn image_data_and_offset(&self) -> Result<(R, u64)>` — [`Result`](../../../index.md#result)

  The subcache data which contains the Mach-O header for this image,

  together with the file offset at which this image starts.

- <span id="dyldcacheimage-parse-object"></span>`fn parse_object(&self) -> Result<File<'data, R>>` — [`Result`](../../../index.md#result), [`File`](../../index.md#file)

  Parse this image into an Object.

#### Trait Implementations

##### `impl Any for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheimage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheImage<'data, 'cache, E, R>`

- <span id="dyldcacheimage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheimage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcachemappingiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcachemappingiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E, R> Iterator for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-iterator-type-item"></span>`type Item = DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemappingiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcachemappingiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheMappingIterator<'data, E, R>`

- <span id="dyldcachemappingiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcachemappingiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  The mapping address

- <span id="dyldcachemapping-size"></span>`fn size(&self) -> u64`

  The mapping size

- <span id="dyldcachemapping-file-offset"></span>`fn file_offset(&self) -> u64`

  The mapping file offset

- <span id="dyldcachemapping-max-prot"></span>`fn max_prot(&self) -> u32`

  The mapping maximum protection

- <span id="dyldcachemapping-init-prot"></span>`fn init_prot(&self) -> u32`

  The mapping initial protection

- <span id="dyldcachemapping-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

  The mapping data

- <span id="dyldcachemapping-relocations"></span>`fn relocations(&self) -> Result<DyldCacheRelocationIterator<'data, E, R>>` — [`Result`](../../../index.md#result), [`DyldCacheRelocationIterator`](../index.md#dyldcacherelocationiterator)

  Relocations for the mapping

#### Trait Implementations

##### `impl Any for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Clone for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-clone"></span>`fn clone(&self) -> DyldCacheMapping<'data, E, R>` — [`DyldCacheMapping`](../index.md#dyldcachemapping)

##### `impl CloneToUninit for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E, R> Copy for DyldCacheMapping<'data, E, R>`

##### `impl<E, R> Debug for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcachemapping-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcachemapping-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcachemapping-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheMapping<'data, E, R>`

- <span id="dyldcachemapping-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcachemapping-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dyldcacherelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dyldcacherelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<E, R> Iterator for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-iterator-type-item"></span>`type Item = Result<DyldRelocation, Error>`

- <span id="dyldcacherelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacherelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheRelocationIterator<'data, E, R>`

- <span id="dyldcacherelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacherelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacherelocationiteratorv2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheRelocationIteratorV2<'data, E, R>`

- <span id="dyldcacherelocationiteratorv2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacherelocationiteratorv2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacherelocationiteratorv3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheRelocationIteratorV3<'data, E, R>`

- <span id="dyldcacherelocationiteratorv3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacherelocationiteratorv3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacherelocationiteratorv5-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheRelocationIteratorV5<'data, E, R>`

- <span id="dyldcacherelocationiteratorv5-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacherelocationiteratorv5-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldRelocation`

- <span id="dyldrelocation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldRelocation`

- <span id="dyldrelocation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldRelocation`

- <span id="dyldrelocation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DyldRelocation`

- <span id="dyldrelocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldRelocation`

- <span id="dyldrelocation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldRelocation`

- <span id="dyldrelocation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldRelocation`

- <span id="dyldrelocation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldrelocation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldRelocation`

- <span id="dyldrelocation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldrelocation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldRelocationAuth`

- <span id="dyldrelocationauth-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldRelocationAuth`

- <span id="dyldrelocationauth-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldRelocationAuth`

- <span id="dyldrelocationauth-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DyldRelocationAuth`

- <span id="dyldrelocationauth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldRelocationAuth`

- <span id="dyldrelocationauth-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldRelocationAuth`

- <span id="dyldrelocationauth-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldRelocationAuth`

- <span id="dyldrelocationauth-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldrelocationauth-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldRelocationAuth`

- <span id="dyldrelocationauth-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldrelocationauth-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-clone"></span>`fn clone(&self) -> DyldSubCacheSlice<'data, E>` — [`DyldSubCacheSlice`](../index.md#dyldsubcacheslice)

##### `impl CloneToUninit for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldSubCacheSlice<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldsubcacheslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldsubcacheslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldsubcacheslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldSubCacheSlice<'data, E>`

- <span id="dyldsubcacheslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldsubcacheslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-clone"></span>`fn clone(&self) -> DyldCacheMappingSlice<'data, E>` — [`DyldCacheMappingSlice`](../index.md#dyldcachemappingslice)

##### `impl CloneToUninit for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheMappingSlice<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcachemappingslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcachemappingslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcachemappingslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheMappingSlice<'data, E>`

- <span id="dyldcachemappingslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcachemappingslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E> Debug for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcachemappingversioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheMappingVersionIterator<'data, E>`

- <span id="dyldcachemappingversioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcachemappingversioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E> Clone for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-clone"></span>`fn clone(&self) -> DyldCacheMappingVersion<'data, E>` — [`DyldCacheMappingVersion`](#dyldcachemappingversion)

##### `impl CloneToUninit for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E> Copy for DyldCacheMappingVersion<'data, E>`

##### `impl<T> From for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcachemappingversion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcachemappingversion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcachemappingversion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheMappingVersion<'data, E>`

- <span id="dyldcachemappingversion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcachemappingversion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-clone"></span>`fn clone(&self) -> DyldCacheSlideInfo<'data, E>` — [`DyldCacheSlideInfo`](../index.md#dyldcacheslideinfo)

##### `impl CloneToUninit for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for DyldCacheSlideInfo<'data, E>`

##### `impl<E: fmt::Debug + Endian> Debug for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="dyldcacheslideinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyldcacheslideinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacheslideinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheSlideInfo<'data, E>`

- <span id="dyldcacheslideinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacheslideinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E, R> Debug for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyldcacherelocationiteratorversion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DyldCacheRelocationIteratorVersion<'data, E, R>`

- <span id="dyldcacherelocationiteratorversion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyldcacherelocationiteratorversion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for RelocationStateV2`

- <span id="relocationstatev2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationStateV2`

- <span id="relocationstatev2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationStateV2`

- <span id="relocationstatev2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationStateV2`

- <span id="relocationstatev2-clone"></span>`fn clone(&self) -> RelocationStateV2` — [`RelocationStateV2`](#relocationstatev2)

##### `impl CloneToUninit for RelocationStateV2`

- <span id="relocationstatev2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationStateV2`

##### `impl Debug for RelocationStateV2`

- <span id="relocationstatev2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV2`

##### `impl<T> From for RelocationStateV2`

- <span id="relocationstatev2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationStateV2`

- <span id="relocationstatev2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationStateV2`

- <span id="relocationstatev2-partialeq-eq"></span>`fn eq(&self, other: &RelocationStateV2) -> bool` — [`RelocationStateV2`](#relocationstatev2)

##### `impl StructuralPartialEq for RelocationStateV2`

##### `impl ToOwned for RelocationStateV2`

- <span id="relocationstatev2-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationstatev2-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationstatev2-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationStateV2`

- <span id="relocationstatev2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationstatev2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationStateV2`

- <span id="relocationstatev2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationstatev2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationStateV3`

```rust
enum RelocationStateV3 {
    Start,
    Page,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:707-710`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L707-L710)*

#### Trait Implementations

##### `impl Any for RelocationStateV3`

- <span id="relocationstatev3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationStateV3`

- <span id="relocationstatev3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationStateV3`

- <span id="relocationstatev3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationStateV3`

- <span id="relocationstatev3-clone"></span>`fn clone(&self) -> RelocationStateV3` — [`RelocationStateV3`](#relocationstatev3)

##### `impl CloneToUninit for RelocationStateV3`

- <span id="relocationstatev3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationStateV3`

##### `impl Debug for RelocationStateV3`

- <span id="relocationstatev3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV3`

##### `impl<T> From for RelocationStateV3`

- <span id="relocationstatev3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationStateV3`

- <span id="relocationstatev3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationStateV3`

- <span id="relocationstatev3-partialeq-eq"></span>`fn eq(&self, other: &RelocationStateV3) -> bool` — [`RelocationStateV3`](#relocationstatev3)

##### `impl StructuralPartialEq for RelocationStateV3`

##### `impl ToOwned for RelocationStateV3`

- <span id="relocationstatev3-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationstatev3-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationstatev3-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationStateV3`

- <span id="relocationstatev3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationstatev3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationStateV3`

- <span id="relocationstatev3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationstatev3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationStateV5`

```rust
enum RelocationStateV5 {
    Start,
    Page,
}
```

*Defined in [`object-0.37.3/src/read/macho/dyld_cache.rs:804-807`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/dyld_cache.rs#L804-L807)*

#### Trait Implementations

##### `impl Any for RelocationStateV5`

- <span id="relocationstatev5-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationStateV5`

- <span id="relocationstatev5-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationStateV5`

- <span id="relocationstatev5-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationStateV5`

- <span id="relocationstatev5-clone"></span>`fn clone(&self) -> RelocationStateV5` — [`RelocationStateV5`](#relocationstatev5)

##### `impl CloneToUninit for RelocationStateV5`

- <span id="relocationstatev5-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationStateV5`

##### `impl Debug for RelocationStateV5`

- <span id="relocationstatev5-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationStateV5`

##### `impl<T> From for RelocationStateV5`

- <span id="relocationstatev5-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationStateV5`

- <span id="relocationstatev5-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationStateV5`

- <span id="relocationstatev5-partialeq-eq"></span>`fn eq(&self, other: &RelocationStateV5) -> bool` — [`RelocationStateV5`](#relocationstatev5)

##### `impl StructuralPartialEq for RelocationStateV5`

##### `impl ToOwned for RelocationStateV5`

- <span id="relocationstatev5-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationstatev5-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationstatev5-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationStateV5`

- <span id="relocationstatev5-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationstatev5-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationStateV5`

- <span id="relocationstatev5-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationstatev5-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

