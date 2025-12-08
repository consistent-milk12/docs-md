*[object](../index.md) / [macho](index.md)*

---

# Module `macho`

Mach-O definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is based heavily on header files from MacOSX11.1.sdk.

## Structs

### `DyldCacheHeader<E: Endian>`

```rust
struct DyldCacheHeader<E: Endian> {
    pub magic: [u8; 16],
    pub mapping_offset: crate::endian::U32<E>,
    pub mapping_count: crate::endian::U32<E>,
    pub images_offset_old: crate::endian::U32<E>,
    pub images_count_old: crate::endian::U32<E>,
    pub dyld_base_address: crate::endian::U64<E>,
    pub code_signature_offset: crate::endian::U64<E>,
    pub code_signature_size: crate::endian::U64<E>,
    pub slide_info_offset_unused: crate::endian::U64<E>,
    pub slide_info_size_unused: crate::endian::U64<E>,
    pub local_symbols_offset: crate::endian::U64<E>,
    pub local_symbols_size: crate::endian::U64<E>,
    pub uuid: [u8; 16],
    pub cache_type: crate::endian::U64<E>,
    pub branch_pools_offset: crate::endian::U32<E>,
    pub branch_pools_count: crate::endian::U32<E>,
    pub dyld_in_cache_mh: crate::endian::U64<E>,
    pub dyld_in_cache_entry: crate::endian::U64<E>,
    pub images_text_offset: crate::endian::U64<E>,
    pub images_text_count: crate::endian::U64<E>,
    pub patch_info_addr: crate::endian::U64<E>,
    pub patch_info_size: crate::endian::U64<E>,
    pub other_image_group_addr_unused: crate::endian::U64<E>,
    pub other_image_group_size_unused: crate::endian::U64<E>,
    pub prog_closures_addr: crate::endian::U64<E>,
    pub prog_closures_size: crate::endian::U64<E>,
    pub prog_closures_trie_addr: crate::endian::U64<E>,
    pub prog_closures_trie_size: crate::endian::U64<E>,
    pub platform: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub shared_region_start: crate::endian::U64<E>,
    pub shared_region_size: crate::endian::U64<E>,
    pub max_slide: crate::endian::U64<E>,
    pub dylibs_image_array_addr: crate::endian::U64<E>,
    pub dylibs_image_array_size: crate::endian::U64<E>,
    pub dylibs_trie_addr: crate::endian::U64<E>,
    pub dylibs_trie_size: crate::endian::U64<E>,
    pub other_image_array_addr: crate::endian::U64<E>,
    pub other_image_array_size: crate::endian::U64<E>,
    pub other_trie_addr: crate::endian::U64<E>,
    pub other_trie_size: crate::endian::U64<E>,
    pub mapping_with_slide_offset: crate::endian::U32<E>,
    pub mapping_with_slide_count: crate::endian::U32<E>,
    pub dylibs_pbl_state_array_addr_unused: crate::endian::U64<E>,
    pub dylibs_pbl_set_addr: crate::endian::U64<E>,
    pub programs_pbl_set_pool_addr: crate::endian::U64<E>,
    pub programs_pbl_set_pool_size: crate::endian::U64<E>,
    pub program_trie_addr: crate::endian::U64<E>,
    pub os_version: crate::endian::U32<E>,
    pub alt_platform: crate::endian::U32<E>,
    pub alt_os_version: crate::endian::U32<E>,
    reserved1: [u8; 4],
    pub swift_opts_offset: crate::endian::U64<E>,
    pub swift_opts_size: crate::endian::U64<E>,
    pub sub_cache_array_offset: crate::endian::U32<E>,
    pub sub_cache_array_count: crate::endian::U32<E>,
    pub symbol_file_uuid: [u8; 16],
    pub rosetta_read_only_addr: crate::endian::U64<E>,
    pub rosetta_read_only_size: crate::endian::U64<E>,
    pub rosetta_read_write_addr: crate::endian::U64<E>,
    pub rosetta_read_write_size: crate::endian::U64<E>,
    pub images_offset: crate::endian::U32<E>,
    pub images_count: crate::endian::U32<E>,
    pub cache_sub_type: crate::endian::U32<E>,
    pub objc_opts_offset: crate::endian::U64<E>,
    pub objc_opts_size: crate::endian::U64<E>,
    pub cache_atlas_offset: crate::endian::U64<E>,
    pub cache_atlas_size: crate::endian::U64<E>,
    pub dynamic_data_offset: crate::endian::U64<E>,
    pub dynamic_data_max_size: crate::endian::U64<E>,
}
```

The dyld cache header.
Corresponds to struct dyld_cache_header from dyld_cache_format.h.
This header has grown over time. Only the fields up to and including dyld_base_address
are guaranteed to be present. For all other fields, check the header size before
accessing the field. The header size is stored in mapping_offset; the mappings start
right after the theader.

#### Fields

- **`magic`**: `[u8; 16]`

  e.g. "dyld_v0    i386"

- **`mapping_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_cache_mapping_info

- **`mapping_count`**: `crate::endian::U32<E>`

  number of dyld_cache_mapping_info entries

- **`images_offset_old`**: `crate::endian::U32<E>`

  UNUSED: moved to imagesOffset to prevent older dsc_extarctors from crashing

- **`images_count_old`**: `crate::endian::U32<E>`

  UNUSED: moved to imagesCount to prevent older dsc_extarctors from crashing

- **`dyld_base_address`**: `crate::endian::U64<E>`

  base address of dyld when cache was built

- **`code_signature_offset`**: `crate::endian::U64<E>`

  file offset of code signature blob

- **`code_signature_size`**: `crate::endian::U64<E>`

  size of code signature blob (zero means to end of file)

- **`slide_info_offset_unused`**: `crate::endian::U64<E>`

  unused.  Used to be file offset of kernel slid info

- **`slide_info_size_unused`**: `crate::endian::U64<E>`

  unused.  Used to be size of kernel slid info

- **`local_symbols_offset`**: `crate::endian::U64<E>`

  file offset of where local symbols are stored

- **`local_symbols_size`**: `crate::endian::U64<E>`

  size of local symbols information

- **`uuid`**: `[u8; 16]`

  unique value for each shared cache file

- **`cache_type`**: `crate::endian::U64<E>`

  0 for development, 1 for production, 2 for multi-cache

- **`branch_pools_offset`**: `crate::endian::U32<E>`

  file offset to table of uint64_t pool addresses

- **`branch_pools_count`**: `crate::endian::U32<E>`

  number of uint64_t entries

- **`dyld_in_cache_mh`**: `crate::endian::U64<E>`

  (unslid) address of mach_header of dyld in cache

- **`dyld_in_cache_entry`**: `crate::endian::U64<E>`

  (unslid) address of entry point (_dyld_start) of dyld in cache

- **`images_text_offset`**: `crate::endian::U64<E>`

  file offset to first dyld_cache_image_text_info

- **`images_text_count`**: `crate::endian::U64<E>`

  number of dyld_cache_image_text_info entries

- **`patch_info_addr`**: `crate::endian::U64<E>`

  (unslid) address of dyld_cache_patch_info

- **`patch_info_size`**: `crate::endian::U64<E>`

  Size of all of the patch information pointed to via the dyld_cache_patch_info

- **`other_image_group_addr_unused`**: `crate::endian::U64<E>`

  unused

- **`other_image_group_size_unused`**: `crate::endian::U64<E>`

  unused

- **`prog_closures_addr`**: `crate::endian::U64<E>`

  (unslid) address of list of program launch closures

- **`prog_closures_size`**: `crate::endian::U64<E>`

  size of list of program launch closures

- **`prog_closures_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie of indexes into program launch closures

- **`prog_closures_trie_size`**: `crate::endian::U64<E>`

  size of trie of indexes into program launch closures

- **`platform`**: `crate::endian::U32<E>`

  platform number (macOS=1, etc)

- **`shared_region_start`**: `crate::endian::U64<E>`

  base load address of cache if not slid

- **`shared_region_size`**: `crate::endian::U64<E>`

  overall size required to map the cache and all subCaches, if any

- **`max_slide`**: `crate::endian::U64<E>`

  runtime slide of cache can be between zero and this value

- **`dylibs_image_array_addr`**: `crate::endian::U64<E>`

  (unslid) address of ImageArray for dylibs in this cache

- **`dylibs_image_array_size`**: `crate::endian::U64<E>`

  size of ImageArray for dylibs in this cache

- **`dylibs_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie of indexes of all cached dylibs

- **`dylibs_trie_size`**: `crate::endian::U64<E>`

  size of trie of cached dylib paths

- **`other_image_array_addr`**: `crate::endian::U64<E>`

  (unslid) address of ImageArray for dylibs and bundles with dlopen closures

- **`other_image_array_size`**: `crate::endian::U64<E>`

  size of ImageArray for dylibs and bundles with dlopen closures

- **`other_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie of indexes of all dylibs and bundles with dlopen closures

- **`other_trie_size`**: `crate::endian::U64<E>`

  size of trie of dylibs and bundles with dlopen closures

- **`mapping_with_slide_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_cache_mapping_and_slide_info

- **`mapping_with_slide_count`**: `crate::endian::U32<E>`

  number of dyld_cache_mapping_and_slide_info entries

- **`dylibs_pbl_state_array_addr_unused`**: `crate::endian::U64<E>`

  unused

- **`dylibs_pbl_set_addr`**: `crate::endian::U64<E>`

  (unslid) address of PrebuiltLoaderSet of all cached dylibs

- **`programs_pbl_set_pool_addr`**: `crate::endian::U64<E>`

  (unslid) address of pool of PrebuiltLoaderSet for each program

- **`programs_pbl_set_pool_size`**: `crate::endian::U64<E>`

  size of pool of PrebuiltLoaderSet for each program

- **`program_trie_addr`**: `crate::endian::U64<E>`

  (unslid) address of trie mapping program path to PrebuiltLoaderSet

- **`os_version`**: `crate::endian::U32<E>`

  OS Version of dylibs in this cache for the main platform

- **`alt_platform`**: `crate::endian::U32<E>`

  e.g. iOSMac on macOS

- **`alt_os_version`**: `crate::endian::U32<E>`

  e.g. 14.0 for iOSMac

- **`swift_opts_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to Swift optimizations header

- **`swift_opts_size`**: `crate::endian::U64<E>`

  size of Swift optimizations header

- **`sub_cache_array_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_subcache_entry

- **`sub_cache_array_count`**: `crate::endian::U32<E>`

  number of subCache entries

- **`symbol_file_uuid`**: `[u8; 16]`

  unique value for the shared cache file containing unmapped local symbols

- **`rosetta_read_only_addr`**: `crate::endian::U64<E>`

  (unslid) address of the start of where Rosetta can add read-only/executable data

- **`rosetta_read_only_size`**: `crate::endian::U64<E>`

  maximum size of the Rosetta read-only/executable region

- **`rosetta_read_write_addr`**: `crate::endian::U64<E>`

  (unslid) address of the start of where Rosetta can add read-write data

- **`rosetta_read_write_size`**: `crate::endian::U64<E>`

  maximum size of the Rosetta read-write region

- **`images_offset`**: `crate::endian::U32<E>`

  file offset to first dyld_cache_image_info

- **`images_count`**: `crate::endian::U32<E>`

  number of dyld_cache_image_info entries

- **`cache_sub_type`**: `crate::endian::U32<E>`

  0 for development, 1 for production, when cacheType is multi-cache(2)

- **`objc_opts_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to ObjC optimizations header

- **`objc_opts_size`**: `crate::endian::U64<E>`

  size of ObjC optimizations header

- **`cache_atlas_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to embedded cache atlas for process introspection

- **`cache_atlas_size`**: `crate::endian::U64<E>`

  size of embedded cache atlas

- **`dynamic_data_offset`**: `crate::endian::U64<E>`

  VM offset from cache_header* to the location of dyld_cache_dynamic_data_header

- **`dynamic_data_max_size`**: `crate::endian::U64<E>`

  maximum size of space reserved from dynamic data

#### Implementations

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<&'data Self>` — [`Result`](../index.md)

- `fn parse_magic(self: &Self) -> Result<(Architecture, E)>` — [`Result`](../index.md), [`Architecture`](../index.md)

- `fn mappings<'data, R: ReadRef<'data>>(self: &Self, endian: E, data: R) -> Result<DyldCacheMappingSlice<'data, E>>` — [`Result`](../index.md), [`DyldCacheMappingSlice`](../read/macho/index.md)

- `fn subcaches<'data, R: ReadRef<'data>>(self: &Self, endian: E, data: R) -> Result<Option<DyldSubCacheSlice<'data, E>>>` — [`Result`](../index.md), [`DyldSubCacheSlice`](../read/macho/index.md)

- `fn symbols_subcache_uuid(self: &Self, endian: E) -> Option<[u8; 16]>`

- `fn images<'data, R: ReadRef<'data>>(self: &Self, endian: E, data: R) -> Result<&'data [macho::DyldCacheImageInfo<E>]>` — [`Result`](../index.md), [`DyldCacheImageInfo`](#dyldcacheimageinfo)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldCacheHeader<E>`

- `fn clone(self: &Self) -> DyldCacheHeader<E>` — [`DyldCacheHeader`](#dyldcacheheader)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldCacheHeader<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldCacheHeader<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldCacheHeader<E>`

### `DyldCacheMappingInfo<E: Endian>`

```rust
struct DyldCacheMappingInfo<E: Endian> {
    pub address: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
    pub file_offset: crate::endian::U64<E>,
    pub max_prot: crate::endian::U32<E>,
    pub init_prot: crate::endian::U32<E>,
}
```

Corresponds to struct dyld_cache_mapping_info from dyld_cache_format.h.

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldCacheMappingInfo<E>`

- `fn clone(self: &Self) -> DyldCacheMappingInfo<E>` — [`DyldCacheMappingInfo`](#dyldcachemappinginfo)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldCacheMappingInfo<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldCacheMappingInfo<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldCacheMappingInfo<E>`

### `DyldCacheMappingAndSlideInfo<E: Endian>`

```rust
struct DyldCacheMappingAndSlideInfo<E: Endian> {
    pub address: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
    pub file_offset: crate::endian::U64<E>,
    pub slide_info_file_offset: crate::endian::U64<E>,
    pub slide_info_file_size: crate::endian::U64<E>,
    pub flags: crate::endian::U64<E>,
    pub max_prot: crate::endian::U32<E>,
    pub init_prot: crate::endian::U32<E>,
}
```

Corresponds to struct dyld_cache_mapping_and_slide_info from dyld_cache_format.h.

#### Implementations

- `fn slide<'data, R: ReadRef<'data>>(self: &Self, endian: E, data: R) -> Result<DyldCacheSlideInfo<'data, E>>` — [`Result`](../index.md), [`DyldCacheSlideInfo`](../read/macho/index.md)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldCacheMappingAndSlideInfo<E>`

- `fn clone(self: &Self) -> DyldCacheMappingAndSlideInfo<E>` — [`DyldCacheMappingAndSlideInfo`](#dyldcachemappingandslideinfo)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldCacheMappingAndSlideInfo<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldCacheMappingAndSlideInfo<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldCacheMappingAndSlideInfo<E>`

### `DyldCacheImageInfo<E: Endian>`

```rust
struct DyldCacheImageInfo<E: Endian> {
    pub address: crate::endian::U64<E>,
    pub mod_time: crate::endian::U64<E>,
    pub inode: crate::endian::U64<E>,
    pub path_file_offset: crate::endian::U32<E>,
    pub pad: crate::endian::U32<E>,
}
```

Corresponds to struct dyld_cache_image_info from dyld_cache_format.h.

#### Implementations

- `fn path<'data, R: ReadRef<'data>>(self: &Self, endian: E, data: R) -> Result<&'data [u8]>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldCacheImageInfo<E>`

- `fn clone(self: &Self) -> DyldCacheImageInfo<E>` — [`DyldCacheImageInfo`](#dyldcacheimageinfo)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldCacheImageInfo<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldCacheImageInfo<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldCacheImageInfo<E>`

### `DyldCacheSlideInfo2<E: Endian>`

```rust
struct DyldCacheSlideInfo2<E: Endian> {
    pub version: crate::endian::U32<E>,
    pub page_size: crate::endian::U32<E>,
    pub page_starts_offset: crate::endian::U32<E>,
    pub page_starts_count: crate::endian::U32<E>,
    pub page_extras_offset: crate::endian::U32<E>,
    pub page_extras_count: crate::endian::U32<E>,
    pub delta_mask: crate::endian::U64<E>,
    pub value_add: crate::endian::U64<E>,
}
```

Corresponds to struct dyld_cache_slide_info2 from dyld_cache_format.h.

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldCacheSlideInfo2<E>`

- `fn clone(self: &Self) -> DyldCacheSlideInfo2<E>` — [`DyldCacheSlideInfo2`](#dyldcacheslideinfo2)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldCacheSlideInfo2<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldCacheSlideInfo2<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldCacheSlideInfo2<E>`

### `DyldCacheSlideInfo3<E: Endian>`

```rust
struct DyldCacheSlideInfo3<E: Endian> {
    pub version: crate::endian::U32<E>,
    pub page_size: crate::endian::U32<E>,
    pub page_starts_count: crate::endian::U32<E>,
    reserved1: [u8; 4],
    pub auth_value_add: crate::endian::U64<E>,
}
```

Corresponds to struct dyld_cache_slide_info3 from dyld_cache_format.h.

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldCacheSlideInfo3<E>`

- `fn clone(self: &Self) -> DyldCacheSlideInfo3<E>` — [`DyldCacheSlideInfo3`](#dyldcacheslideinfo3)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldCacheSlideInfo3<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldCacheSlideInfo3<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldCacheSlideInfo3<E>`

### `DyldCacheSlidePointer3`

```rust
struct DyldCacheSlidePointer3(u64);
```

Corresponds to union dyld_cache_slide_pointer3 from dyld_cache_format.h.

#### Implementations

- `fn is_auth(self: &Self) -> bool`

- `fn target(self: &Self) -> u64`

- `fn high8(self: &Self) -> u64`

- `fn runtime_offset(self: &Self) -> u64`

- `fn diversity(self: &Self) -> u16`

- `fn addr_div(self: &Self) -> bool`

- `fn key(self: &Self) -> u8`

- `fn next(self: &Self) -> u64`

#### Trait Implementations

##### `impl Clone for DyldCacheSlidePointer3`

- `fn clone(self: &Self) -> DyldCacheSlidePointer3` — [`DyldCacheSlidePointer3`](#dyldcacheslidepointer3)

##### `impl Copy for DyldCacheSlidePointer3`

##### `impl Debug for DyldCacheSlidePointer3`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldCacheSlideInfo5<E: Endian>`

```rust
struct DyldCacheSlideInfo5<E: Endian> {
    pub version: crate::endian::U32<E>,
    pub page_size: crate::endian::U32<E>,
    pub page_starts_count: crate::endian::U32<E>,
    reserved1: [u8; 4],
    pub value_add: crate::endian::U64<E>,
}
```

Corresponds to struct dyld_cache_slide_info5 from dyld_cache_format.h.

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldCacheSlideInfo5<E>`

- `fn clone(self: &Self) -> DyldCacheSlideInfo5<E>` — [`DyldCacheSlideInfo5`](#dyldcacheslideinfo5)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldCacheSlideInfo5<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldCacheSlideInfo5<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldCacheSlideInfo5<E>`

### `DyldCacheSlidePointer5`

```rust
struct DyldCacheSlidePointer5(u64);
```

Corresponds to struct dyld_cache_slide_pointer5 from dyld_cache_format.h.

#### Implementations

- `fn is_auth(self: &Self) -> bool`

- `fn runtime_offset(self: &Self) -> u64`

- `fn high8(self: &Self) -> u64`

- `fn diversity(self: &Self) -> u16`

- `fn addr_div(self: &Self) -> bool`

- `fn key_is_data(self: &Self) -> bool`

- `fn next(self: &Self) -> u64`

#### Trait Implementations

##### `impl Clone for DyldCacheSlidePointer5`

- `fn clone(self: &Self) -> DyldCacheSlidePointer5` — [`DyldCacheSlidePointer5`](#dyldcacheslidepointer5)

##### `impl Copy for DyldCacheSlidePointer5`

##### `impl Debug for DyldCacheSlidePointer5`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DyldSubCacheEntryV1<E: Endian>`

```rust
struct DyldSubCacheEntryV1<E: Endian> {
    pub uuid: [u8; 16],
    pub cache_vm_offset: crate::endian::U64<E>,
}
```

Added in dyld-940, which shipped with macOS 12 / iOS 15.
Originally called `dyld_subcache_entry`, renamed to `dyld_subcache_entry_v1`
in dyld-1042.1.

#### Fields

- **`uuid`**: `[u8; 16]`

  The UUID of this subcache.

- **`cache_vm_offset`**: `crate::endian::U64<E>`

  The offset of this subcache from the main cache base address.

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldSubCacheEntryV1<E>`

- `fn clone(self: &Self) -> DyldSubCacheEntryV1<E>` — [`DyldSubCacheEntryV1`](#dyldsubcacheentryv1)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldSubCacheEntryV1<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldSubCacheEntryV1<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldSubCacheEntryV1<E>`

### `DyldSubCacheEntryV2<E: Endian>`

```rust
struct DyldSubCacheEntryV2<E: Endian> {
    pub uuid: [u8; 16],
    pub cache_vm_offset: crate::endian::U64<E>,
    pub file_suffix: [u8; 32],
}
```

Added in dyld-1042.1, which shipped with macOS 13 / iOS 16.
Called `dyld_subcache_entry` as of dyld-1042.1.

#### Fields

- **`uuid`**: `[u8; 16]`

  The UUID of this subcache.

- **`cache_vm_offset`**: `crate::endian::U64<E>`

  The offset of this subcache from the main cache base address.

- **`file_suffix`**: `[u8; 32]`

  The file name suffix of the subCache file, e.g. ".25.data" or ".03.development".

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldSubCacheEntryV2<E>`

- `fn clone(self: &Self) -> DyldSubCacheEntryV2<E>` — [`DyldSubCacheEntryV2`](#dyldsubcacheentryv2)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldSubCacheEntryV2<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldSubCacheEntryV2<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldSubCacheEntryV2<E>`

### `FatHeader`

```rust
struct FatHeader {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub nfat_arch: crate::endian::U32<crate::endian::BigEndian>,
}
```

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  FAT_MAGIC or FAT_MAGIC_64

- **`nfat_arch`**: `crate::endian::U32<crate::endian::BigEndian>`

  number of structs that follow

#### Trait Implementations

##### `impl Clone for FatHeader`

- `fn clone(self: &Self) -> FatHeader` — [`FatHeader`](#fatheader)

##### `impl Copy for FatHeader`

##### `impl Debug for FatHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Pod for FatHeader`

### `FatArch32`

```rust
struct FatArch32 {
    pub cputype: crate::endian::U32<crate::endian::BigEndian>,
    pub cpusubtype: crate::endian::U32<crate::endian::BigEndian>,
    pub offset: crate::endian::U32<crate::endian::BigEndian>,
    pub size: crate::endian::U32<crate::endian::BigEndian>,
    pub align: crate::endian::U32<crate::endian::BigEndian>,
}
```

#### Fields

- **`cputype`**: `crate::endian::U32<crate::endian::BigEndian>`

  cpu specifier (int)

- **`cpusubtype`**: `crate::endian::U32<crate::endian::BigEndian>`

  machine specifier (int)

- **`offset`**: `crate::endian::U32<crate::endian::BigEndian>`

  file offset to this object file

- **`size`**: `crate::endian::U32<crate::endian::BigEndian>`

  size of this object file

- **`align`**: `crate::endian::U32<crate::endian::BigEndian>`

  alignment as a power of 2

#### Trait Implementations

##### `impl Clone for FatArch32`

- `fn clone(self: &Self) -> FatArch32` — [`FatArch32`](#fatarch32)

##### `impl Copy for FatArch32`

##### `impl Debug for FatArch32`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FatArch for FatArch32`

- `type Word = u32`

- `const MAGIC: u32`

- `fn cputype(self: &Self) -> u32`

- `fn cpusubtype(self: &Self) -> u32`

- `fn offset(self: &Self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md)

- `fn size(self: &Self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md)

- `fn align(self: &Self) -> u32`

##### `impl Pod for FatArch32`

### `FatArch64`

```rust
struct FatArch64 {
    pub cputype: crate::endian::U32<crate::endian::BigEndian>,
    pub cpusubtype: crate::endian::U32<crate::endian::BigEndian>,
    pub offset: crate::endian::U64<crate::endian::BigEndian>,
    pub size: crate::endian::U64<crate::endian::BigEndian>,
    pub align: crate::endian::U32<crate::endian::BigEndian>,
    pub reserved: crate::endian::U32<crate::endian::BigEndian>,
}
```

#### Fields

- **`cputype`**: `crate::endian::U32<crate::endian::BigEndian>`

  cpu specifier (int)

- **`cpusubtype`**: `crate::endian::U32<crate::endian::BigEndian>`

  machine specifier (int)

- **`offset`**: `crate::endian::U64<crate::endian::BigEndian>`

  file offset to this object file

- **`size`**: `crate::endian::U64<crate::endian::BigEndian>`

  size of this object file

- **`align`**: `crate::endian::U32<crate::endian::BigEndian>`

  alignment as a power of 2

- **`reserved`**: `crate::endian::U32<crate::endian::BigEndian>`

  reserved

#### Trait Implementations

##### `impl Clone for FatArch64`

- `fn clone(self: &Self) -> FatArch64` — [`FatArch64`](#fatarch64)

##### `impl Copy for FatArch64`

##### `impl Debug for FatArch64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FatArch for FatArch64`

- `type Word = u64`

- `const MAGIC: u32`

- `fn cputype(self: &Self) -> u32`

- `fn cpusubtype(self: &Self) -> u32`

- `fn offset(self: &Self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md)

- `fn size(self: &Self) -> <Self as >::Word` — [`FatArch`](../read/macho/index.md)

- `fn align(self: &Self) -> u32`

##### `impl Pod for FatArch64`

### `MachHeader32<E: Endian>`

```rust
struct MachHeader32<E: Endian> {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub cputype: crate::endian::U32<E>,
    pub cpusubtype: crate::endian::U32<E>,
    pub filetype: crate::endian::U32<E>,
    pub ncmds: crate::endian::U32<E>,
    pub sizeofcmds: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
}
```

The 32-bit mach header.

Appears at the very beginning of the object file for 32-bit architectures.

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  mach magic number identifier

- **`cputype`**: `crate::endian::U32<E>`

  cpu specifier

- **`cpusubtype`**: `crate::endian::U32<E>`

  machine specifier

- **`filetype`**: `crate::endian::U32<E>`

  type of file

- **`ncmds`**: `crate::endian::U32<E>`

  number of load commands

- **`sizeofcmds`**: `crate::endian::U32<E>`

  the size of all the load commands

- **`flags`**: `crate::endian::U32<E>`

  flags

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for MachHeader32<E>`

- `fn clone(self: &Self) -> MachHeader32<E>` — [`MachHeader32`](#machheader32)

##### `impl<E: $crate::marker::Copy + Endian> Copy for MachHeader32<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for MachHeader32<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Endian: endian::Endian> MachHeader for macho::MachHeader32<Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `type Segment = SegmentCommand32<Endian>`

- `type Section = Section32<Endian>`

- `type Nlist = Nlist32<Endian>`

- `fn is_type_64(self: &Self) -> bool`

- `fn is_big_endian(self: &Self) -> bool`

- `fn is_little_endian(self: &Self) -> bool`

- `fn magic(self: &Self) -> u32`

- `fn cputype(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn cpusubtype(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn filetype(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn ncmds(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn sizeofcmds(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

##### `impl<E: Endian> Pod for MachHeader32<E>`

### `MachHeader64<E: Endian>`

```rust
struct MachHeader64<E: Endian> {
    pub magic: crate::endian::U32<crate::endian::BigEndian>,
    pub cputype: crate::endian::U32<E>,
    pub cpusubtype: crate::endian::U32<E>,
    pub filetype: crate::endian::U32<E>,
    pub ncmds: crate::endian::U32<E>,
    pub sizeofcmds: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub reserved: crate::endian::U32<E>,
}
```

The 64-bit mach header.

Appears at the very beginning of object files for 64-bit architectures.

#### Fields

- **`magic`**: `crate::endian::U32<crate::endian::BigEndian>`

  mach magic number identifier

- **`cputype`**: `crate::endian::U32<E>`

  cpu specifier

- **`cpusubtype`**: `crate::endian::U32<E>`

  machine specifier

- **`filetype`**: `crate::endian::U32<E>`

  type of file

- **`ncmds`**: `crate::endian::U32<E>`

  number of load commands

- **`sizeofcmds`**: `crate::endian::U32<E>`

  the size of all the load commands

- **`flags`**: `crate::endian::U32<E>`

  flags

- **`reserved`**: `crate::endian::U32<E>`

  reserved

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for MachHeader64<E>`

- `fn clone(self: &Self) -> MachHeader64<E>` — [`MachHeader64`](#machheader64)

##### `impl<E: $crate::marker::Copy + Endian> Copy for MachHeader64<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for MachHeader64<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Endian: endian::Endian> MachHeader for macho::MachHeader64<Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `type Segment = SegmentCommand64<Endian>`

- `type Section = Section64<Endian>`

- `type Nlist = Nlist64<Endian>`

- `fn is_type_64(self: &Self) -> bool`

- `fn is_big_endian(self: &Self) -> bool`

- `fn is_little_endian(self: &Self) -> bool`

- `fn magic(self: &Self) -> u32`

- `fn cputype(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn cpusubtype(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn filetype(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn ncmds(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn sizeofcmds(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32` — [`MachHeader`](../read/macho/index.md)

##### `impl<E: Endian> Pod for MachHeader64<E>`

### `LoadCommand<E: Endian>`

```rust
struct LoadCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
}
```

Common fields at the start of every load command.

The load commands directly follow the mach_header.  The total size of all
of the commands is given by the sizeofcmds field in the mach_header.  All
load commands must have as their first two fields `cmd` and `cmdsize`.  The `cmd`
field is filled in with a constant for that command type.  Each command type
has a structure specifically for it.  The `cmdsize` field is the size in bytes
of the particular load command structure plus anything that follows it that
is a part of the load command (i.e. section structures, strings, etc.).  To
advance to the next load command the `cmdsize` can be added to the offset or
pointer of the current load command.  The `cmdsize` for 32-bit architectures
MUST be a multiple of 4 bytes and for 64-bit architectures MUST be a multiple
of 8 bytes (these are forever the maximum alignment of any load commands).
The padded bytes must be zero.  All tables in the object file must also
follow these rules so the file can be memory mapped.  Otherwise the pointers
to these tables will not work well or at all on some machines.  With all
padding zeroed like objects will compare byte for byte.

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  Type of load command.
  
  One of the `LC_*` constants.

- **`cmdsize`**: `crate::endian::U32<E>`

  Total size of command in bytes.

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for LoadCommand<E>`

- `fn clone(self: &Self) -> LoadCommand<E>` — [`LoadCommand`](#loadcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for LoadCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for LoadCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for LoadCommand<E>`

### `LcStr<E: Endian>`

```rust
struct LcStr<E: Endian> {
    pub offset: crate::endian::U32<E>,
}
```

A variable length string in a load command.

The strings are stored just after the load command structure and
the offset is from the start of the load command structure.  The size
of the string is reflected in the `cmdsize` field of the load command.
Once again any padded bytes to bring the `cmdsize` field to a multiple
of 4 bytes must be zero.

#### Fields

- **`offset`**: `crate::endian::U32<E>`

  offset to the string

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for LcStr<E>`

- `fn clone(self: &Self) -> LcStr<E>` — [`LcStr`](#lcstr)

##### `impl<E: $crate::marker::Copy + Endian> Copy for LcStr<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for LcStr<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for LcStr<E>`

### `SegmentCommand32<E: Endian>`

```rust
struct SegmentCommand32<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub segname: [u8; 16],
    pub vmaddr: crate::endian::U32<E>,
    pub vmsize: crate::endian::U32<E>,
    pub fileoff: crate::endian::U32<E>,
    pub filesize: crate::endian::U32<E>,
    pub maxprot: crate::endian::U32<E>,
    pub initprot: crate::endian::U32<E>,
    pub nsects: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
}
```

32-bit segment load command.

The segment load command indicates that a part of this file is to be
mapped into the task's address space.  The size of this segment in memory,
vmsize, maybe equal to or larger than the amount to map from this file,
filesize.  The file is mapped starting at fileoff to the beginning of
the segment in memory, vmaddr.  The rest of the memory of the segment,
if any, is allocated zero fill on demand.  The segment's maximum virtual
memory protection and initial virtual memory protection are specified
by the maxprot and initprot fields.  If the segment has sections then the
`Section32` structures directly follow the segment command and their size is
reflected in `cmdsize`.

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SEGMENT

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sizeof section structs

- **`segname`**: `[u8; 16]`

  segment name

- **`vmaddr`**: `crate::endian::U32<E>`

  memory address of this segment

- **`vmsize`**: `crate::endian::U32<E>`

  memory size of this segment

- **`fileoff`**: `crate::endian::U32<E>`

  file offset of this segment

- **`filesize`**: `crate::endian::U32<E>`

  amount to map from the file

- **`maxprot`**: `crate::endian::U32<E>`

  maximum VM protection

- **`initprot`**: `crate::endian::U32<E>`

  initial VM protection

- **`nsects`**: `crate::endian::U32<E>`

  number of sections in segment

- **`flags`**: `crate::endian::U32<E>`

  flags

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SegmentCommand32<E>`

- `fn clone(self: &Self) -> SegmentCommand32<E>` — [`SegmentCommand32`](#segmentcommand32)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SegmentCommand32<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SegmentCommand32<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SegmentCommand32<E>`

##### `impl<Endian: endian::Endian> Segment for macho::SegmentCommand32<Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `type Section = Section32<<SegmentCommand32<Endian> as Segment>::Endian>`

- `fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>` — [`LoadCommandData`](../read/macho/index.md), [`Segment`](../read/macho/index.md), [`Result`](../index.md)

- `fn cmd(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn cmdsize(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn vmaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn vmsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn fileoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn filesize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn maxprot(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn initprot(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn nsects(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

### `SegmentCommand64<E: Endian>`

```rust
struct SegmentCommand64<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub segname: [u8; 16],
    pub vmaddr: crate::endian::U64<E>,
    pub vmsize: crate::endian::U64<E>,
    pub fileoff: crate::endian::U64<E>,
    pub filesize: crate::endian::U64<E>,
    pub maxprot: crate::endian::U32<E>,
    pub initprot: crate::endian::U32<E>,
    pub nsects: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
}
```

64-bit segment load command.

The 64-bit segment load command indicates that a part of this file is to be
mapped into a 64-bit task's address space.  If the 64-bit segment has
sections then `Section64` structures directly follow the 64-bit segment
command and their size is reflected in `cmdsize`.

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SEGMENT_64

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sizeof section_64 structs

- **`segname`**: `[u8; 16]`

  segment name

- **`vmaddr`**: `crate::endian::U64<E>`

  memory address of this segment

- **`vmsize`**: `crate::endian::U64<E>`

  memory size of this segment

- **`fileoff`**: `crate::endian::U64<E>`

  file offset of this segment

- **`filesize`**: `crate::endian::U64<E>`

  amount to map from the file

- **`maxprot`**: `crate::endian::U32<E>`

  maximum VM protection

- **`initprot`**: `crate::endian::U32<E>`

  initial VM protection

- **`nsects`**: `crate::endian::U32<E>`

  number of sections in segment

- **`flags`**: `crate::endian::U32<E>`

  flags

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SegmentCommand64<E>`

- `fn clone(self: &Self) -> SegmentCommand64<E>` — [`SegmentCommand64`](#segmentcommand64)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SegmentCommand64<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SegmentCommand64<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SegmentCommand64<E>`

##### `impl<Endian: endian::Endian> Segment for macho::SegmentCommand64<Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `type Section = Section64<<SegmentCommand64<Endian> as Segment>::Endian>`

- `fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>` — [`LoadCommandData`](../read/macho/index.md), [`Segment`](../read/macho/index.md), [`Result`](../index.md)

- `fn cmd(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn cmdsize(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn vmaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn vmsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn fileoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn filesize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Segment`](../read/macho/index.md)

- `fn maxprot(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn initprot(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn nsects(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32` — [`Segment`](../read/macho/index.md)

### `Section32<E: Endian>`

```rust
struct Section32<E: Endian> {
    pub sectname: [u8; 16],
    pub segname: [u8; 16],
    pub addr: crate::endian::U32<E>,
    pub size: crate::endian::U32<E>,
    pub offset: crate::endian::U32<E>,
    pub align: crate::endian::U32<E>,
    pub reloff: crate::endian::U32<E>,
    pub nreloc: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub reserved1: crate::endian::U32<E>,
    pub reserved2: crate::endian::U32<E>,
}
```

32-bit section.

#### Fields

- **`sectname`**: `[u8; 16]`

  name of this section

- **`segname`**: `[u8; 16]`

  segment this section goes in

- **`addr`**: `crate::endian::U32<E>`

  memory address of this section

- **`size`**: `crate::endian::U32<E>`

  size in bytes of this section

- **`offset`**: `crate::endian::U32<E>`

  file offset of this section

- **`align`**: `crate::endian::U32<E>`

  section alignment (power of 2)

- **`reloff`**: `crate::endian::U32<E>`

  file offset of relocation entries

- **`nreloc`**: `crate::endian::U32<E>`

  number of relocation entries

- **`flags`**: `crate::endian::U32<E>`

  flags (section type and attributes)

- **`reserved1`**: `crate::endian::U32<E>`

  reserved (for offset or index)

- **`reserved2`**: `crate::endian::U32<E>`

  reserved (for count or sizeof)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for Section32<E>`

- `fn clone(self: &Self) -> Section32<E>` — [`Section32`](#section32)

##### `impl<E: $crate::marker::Copy + Endian> Copy for Section32<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for Section32<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for Section32<E>`

##### `impl<Endian: endian::Endian> Section for macho::Section32<Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `fn sectname(self: &Self) -> &[u8; 16]`

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md)

- `fn size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md)

- `fn offset(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn align(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn reloff(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn nreloc(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

### `Section64<E: Endian>`

```rust
struct Section64<E: Endian> {
    pub sectname: [u8; 16],
    pub segname: [u8; 16],
    pub addr: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
    pub offset: crate::endian::U32<E>,
    pub align: crate::endian::U32<E>,
    pub reloff: crate::endian::U32<E>,
    pub nreloc: crate::endian::U32<E>,
    pub flags: crate::endian::U32<E>,
    pub reserved1: crate::endian::U32<E>,
    pub reserved2: crate::endian::U32<E>,
    pub reserved3: crate::endian::U32<E>,
}
```

64-bit section.

#### Fields

- **`sectname`**: `[u8; 16]`

  name of this section

- **`segname`**: `[u8; 16]`

  segment this section goes in

- **`addr`**: `crate::endian::U64<E>`

  memory address of this section

- **`size`**: `crate::endian::U64<E>`

  size in bytes of this section

- **`offset`**: `crate::endian::U32<E>`

  file offset of this section

- **`align`**: `crate::endian::U32<E>`

  section alignment (power of 2)

- **`reloff`**: `crate::endian::U32<E>`

  file offset of relocation entries

- **`nreloc`**: `crate::endian::U32<E>`

  number of relocation entries

- **`flags`**: `crate::endian::U32<E>`

  flags (section type and attributes)

- **`reserved1`**: `crate::endian::U32<E>`

  reserved (for offset or index)

- **`reserved2`**: `crate::endian::U32<E>`

  reserved (for count or sizeof)

- **`reserved3`**: `crate::endian::U32<E>`

  reserved

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for Section64<E>`

- `fn clone(self: &Self) -> Section64<E>` — [`Section64`](#section64)

##### `impl<E: $crate::marker::Copy + Endian> Copy for Section64<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for Section64<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for Section64<E>`

##### `impl<Endian: endian::Endian> Section for macho::Section64<Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `fn sectname(self: &Self) -> &[u8; 16]`

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn addr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md)

- `fn size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Section`](../read/macho/index.md)

- `fn offset(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn align(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn reloff(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn nreloc(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32` — [`Section`](../read/macho/index.md)

### `Fvmlib<E: Endian>`

```rust
struct Fvmlib<E: Endian> {
    pub name: LcStr<E>,
    pub minor_version: crate::endian::U32<E>,
    pub header_addr: crate::endian::U32<E>,
}
```

#### Fields

- **`name`**: `LcStr<E>`

  library's target pathname

- **`minor_version`**: `crate::endian::U32<E>`

  library's minor version number

- **`header_addr`**: `crate::endian::U32<E>`

  library's header address

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for Fvmlib<E>`

- `fn clone(self: &Self) -> Fvmlib<E>` — [`Fvmlib`](#fvmlib)

##### `impl<E: $crate::marker::Copy + Endian> Copy for Fvmlib<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for Fvmlib<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for Fvmlib<E>`

### `FvmlibCommand<E: Endian>`

```rust
struct FvmlibCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub fvmlib: Fvmlib<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_IDFVMLIB or LC_LOADFVMLIB

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`fvmlib`**: `Fvmlib<E>`

  the library identification

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for FvmlibCommand<E>`

- `fn clone(self: &Self) -> FvmlibCommand<E>` — [`FvmlibCommand`](#fvmlibcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for FvmlibCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for FvmlibCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for FvmlibCommand<E>`

### `Dylib<E: Endian>`

```rust
struct Dylib<E: Endian> {
    pub name: LcStr<E>,
    pub timestamp: crate::endian::U32<E>,
    pub current_version: crate::endian::U32<E>,
    pub compatibility_version: crate::endian::U32<E>,
}
```

#### Fields

- **`name`**: `LcStr<E>`

  library's path name

- **`timestamp`**: `crate::endian::U32<E>`

  library's build time stamp

- **`current_version`**: `crate::endian::U32<E>`

  library's current version number

- **`compatibility_version`**: `crate::endian::U32<E>`

  library's compatibility vers number

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for Dylib<E>`

- `fn clone(self: &Self) -> Dylib<E>` — [`Dylib`](#dylib)

##### `impl<E: $crate::marker::Copy + Endian> Copy for Dylib<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for Dylib<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for Dylib<E>`

### `DylibCommand<E: Endian>`

```rust
struct DylibCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub dylib: Dylib<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ID_DYLIB, LC_LOAD_{,WEAK_}DYLIB, LC_REEXPORT_DYLIB

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`dylib`**: `Dylib<E>`

  the library identification

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DylibCommand<E>`

- `fn clone(self: &Self) -> DylibCommand<E>` — [`DylibCommand`](#dylibcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DylibCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DylibCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DylibCommand<E>`

### `SubFrameworkCommand<E: Endian>`

```rust
struct SubFrameworkCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub umbrella: LcStr<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_FRAMEWORK

- **`cmdsize`**: `crate::endian::U32<E>`

  includes umbrella string

- **`umbrella`**: `LcStr<E>`

  the umbrella framework name

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SubFrameworkCommand<E>`

- `fn clone(self: &Self) -> SubFrameworkCommand<E>` — [`SubFrameworkCommand`](#subframeworkcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SubFrameworkCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SubFrameworkCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SubFrameworkCommand<E>`

### `SubClientCommand<E: Endian>`

```rust
struct SubClientCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub client: LcStr<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_CLIENT

- **`cmdsize`**: `crate::endian::U32<E>`

  includes client string

- **`client`**: `LcStr<E>`

  the client name

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SubClientCommand<E>`

- `fn clone(self: &Self) -> SubClientCommand<E>` — [`SubClientCommand`](#subclientcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SubClientCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SubClientCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SubClientCommand<E>`

### `SubUmbrellaCommand<E: Endian>`

```rust
struct SubUmbrellaCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub sub_umbrella: LcStr<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_UMBRELLA

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sub_umbrella string

- **`sub_umbrella`**: `LcStr<E>`

  the sub_umbrella framework name

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SubUmbrellaCommand<E>`

- `fn clone(self: &Self) -> SubUmbrellaCommand<E>` — [`SubUmbrellaCommand`](#subumbrellacommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SubUmbrellaCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SubUmbrellaCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SubUmbrellaCommand<E>`

### `SubLibraryCommand<E: Endian>`

```rust
struct SubLibraryCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub sub_library: LcStr<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SUB_LIBRARY

- **`cmdsize`**: `crate::endian::U32<E>`

  includes sub_library string

- **`sub_library`**: `LcStr<E>`

  the sub_library name

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SubLibraryCommand<E>`

- `fn clone(self: &Self) -> SubLibraryCommand<E>` — [`SubLibraryCommand`](#sublibrarycommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SubLibraryCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SubLibraryCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SubLibraryCommand<E>`

### `PreboundDylibCommand<E: Endian>`

```rust
struct PreboundDylibCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub name: LcStr<E>,
    pub nmodules: crate::endian::U32<E>,
    pub linked_modules: LcStr<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_PREBOUND_DYLIB

- **`cmdsize`**: `crate::endian::U32<E>`

  includes strings

- **`name`**: `LcStr<E>`

  library's path name

- **`nmodules`**: `crate::endian::U32<E>`

  number of modules in library

- **`linked_modules`**: `LcStr<E>`

  bit vector of linked modules

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for PreboundDylibCommand<E>`

- `fn clone(self: &Self) -> PreboundDylibCommand<E>` — [`PreboundDylibCommand`](#prebounddylibcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for PreboundDylibCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for PreboundDylibCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for PreboundDylibCommand<E>`

### `DylinkerCommand<E: Endian>`

```rust
struct DylinkerCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub name: LcStr<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ID_DYLINKER, LC_LOAD_DYLINKER or LC_DYLD_ENVIRONMENT

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`name`**: `LcStr<E>`

  dynamic linker's path name

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DylinkerCommand<E>`

- `fn clone(self: &Self) -> DylinkerCommand<E>` — [`DylinkerCommand`](#dylinkercommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DylinkerCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DylinkerCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DylinkerCommand<E>`

### `ThreadCommand<E: Endian>`

```rust
struct ThreadCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_THREAD or  LC_UNIXTHREAD

- **`cmdsize`**: `crate::endian::U32<E>`

  total size of this command

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for ThreadCommand<E>`

- `fn clone(self: &Self) -> ThreadCommand<E>` — [`ThreadCommand`](#threadcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for ThreadCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for ThreadCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for ThreadCommand<E>`

### `RoutinesCommand32<E: Endian>`

```rust
struct RoutinesCommand32<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub init_address: crate::endian::U32<E>,
    pub init_module: crate::endian::U32<E>,
    pub reserved1: crate::endian::U32<E>,
    pub reserved2: crate::endian::U32<E>,
    pub reserved3: crate::endian::U32<E>,
    pub reserved4: crate::endian::U32<E>,
    pub reserved5: crate::endian::U32<E>,
    pub reserved6: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ROUTINES

- **`cmdsize`**: `crate::endian::U32<E>`

  total size of this command

- **`init_address`**: `crate::endian::U32<E>`

  address of initialization routine

- **`init_module`**: `crate::endian::U32<E>`

  index into the module table that the init routine is defined in

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for RoutinesCommand32<E>`

- `fn clone(self: &Self) -> RoutinesCommand32<E>` — [`RoutinesCommand32`](#routinescommand32)

##### `impl<E: $crate::marker::Copy + Endian> Copy for RoutinesCommand32<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for RoutinesCommand32<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for RoutinesCommand32<E>`

### `RoutinesCommand64<E: Endian>`

```rust
struct RoutinesCommand64<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub init_address: crate::endian::U64<E>,
    pub init_module: crate::endian::U64<E>,
    pub reserved1: crate::endian::U64<E>,
    pub reserved2: crate::endian::U64<E>,
    pub reserved3: crate::endian::U64<E>,
    pub reserved4: crate::endian::U64<E>,
    pub reserved5: crate::endian::U64<E>,
    pub reserved6: crate::endian::U64<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ROUTINES_64

- **`cmdsize`**: `crate::endian::U32<E>`

  total size of this command

- **`init_address`**: `crate::endian::U64<E>`

  address of initialization routine

- **`init_module`**: `crate::endian::U64<E>`

  index into the module table that the init routine is defined in

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for RoutinesCommand64<E>`

- `fn clone(self: &Self) -> RoutinesCommand64<E>` — [`RoutinesCommand64`](#routinescommand64)

##### `impl<E: $crate::marker::Copy + Endian> Copy for RoutinesCommand64<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for RoutinesCommand64<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for RoutinesCommand64<E>`

### `SymtabCommand<E: Endian>`

```rust
struct SymtabCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub symoff: crate::endian::U32<E>,
    pub nsyms: crate::endian::U32<E>,
    pub stroff: crate::endian::U32<E>,
    pub strsize: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SYMTAB

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct SymtabCommand)

- **`symoff`**: `crate::endian::U32<E>`

  symbol table offset

- **`nsyms`**: `crate::endian::U32<E>`

  number of symbol table entries

- **`stroff`**: `crate::endian::U32<E>`

  string table offset

- **`strsize`**: `crate::endian::U32<E>`

  string table size in bytes

#### Implementations

- `fn symbols<'data, Mach: MachHeader<Endian = E>, R: ReadRef<'data>>(self: &Self, endian: E, data: R) -> Result<SymbolTable<'data, Mach, R>>` — [`Result`](../index.md), [`SymbolTable`](../read/macho/index.md)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SymtabCommand<E>`

- `fn clone(self: &Self) -> SymtabCommand<E>` — [`SymtabCommand`](#symtabcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SymtabCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SymtabCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SymtabCommand<E>`

### `DysymtabCommand<E: Endian>`

```rust
struct DysymtabCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub ilocalsym: crate::endian::U32<E>,
    pub nlocalsym: crate::endian::U32<E>,
    pub iextdefsym: crate::endian::U32<E>,
    pub nextdefsym: crate::endian::U32<E>,
    pub iundefsym: crate::endian::U32<E>,
    pub nundefsym: crate::endian::U32<E>,
    pub tocoff: crate::endian::U32<E>,
    pub ntoc: crate::endian::U32<E>,
    pub modtaboff: crate::endian::U32<E>,
    pub nmodtab: crate::endian::U32<E>,
    pub extrefsymoff: crate::endian::U32<E>,
    pub nextrefsyms: crate::endian::U32<E>,
    pub indirectsymoff: crate::endian::U32<E>,
    pub nindirectsyms: crate::endian::U32<E>,
    pub extreloff: crate::endian::U32<E>,
    pub nextrel: crate::endian::U32<E>,
    pub locreloff: crate::endian::U32<E>,
    pub nlocrel: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_DYSYMTAB

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct DysymtabCommand)

- **`ilocalsym`**: `crate::endian::U32<E>`

  index to local symbols

- **`nlocalsym`**: `crate::endian::U32<E>`

  number of local symbols

- **`iextdefsym`**: `crate::endian::U32<E>`

  index to externally defined symbols

- **`nextdefsym`**: `crate::endian::U32<E>`

  number of externally defined symbols

- **`iundefsym`**: `crate::endian::U32<E>`

  index to undefined symbols

- **`nundefsym`**: `crate::endian::U32<E>`

  number of undefined symbols

- **`tocoff`**: `crate::endian::U32<E>`

  file offset to table of contents

- **`ntoc`**: `crate::endian::U32<E>`

  number of entries in table of contents

- **`modtaboff`**: `crate::endian::U32<E>`

  file offset to module table

- **`nmodtab`**: `crate::endian::U32<E>`

  number of module table entries

- **`extrefsymoff`**: `crate::endian::U32<E>`

  offset to referenced symbol table

- **`nextrefsyms`**: `crate::endian::U32<E>`

  number of referenced symbol table entries

- **`indirectsymoff`**: `crate::endian::U32<E>`

  file offset to the indirect symbol table

- **`nindirectsyms`**: `crate::endian::U32<E>`

  number of indirect symbol table entries

- **`extreloff`**: `crate::endian::U32<E>`

  offset to external relocation entries

- **`nextrel`**: `crate::endian::U32<E>`

  number of external relocation entries

- **`locreloff`**: `crate::endian::U32<E>`

  offset to local relocation entries

- **`nlocrel`**: `crate::endian::U32<E>`

  number of local relocation entries

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DysymtabCommand<E>`

- `fn clone(self: &Self) -> DysymtabCommand<E>` — [`DysymtabCommand`](#dysymtabcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DysymtabCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DysymtabCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DysymtabCommand<E>`

### `DylibTableOfContents<E: Endian>`

```rust
struct DylibTableOfContents<E: Endian> {
    pub symbol_index: crate::endian::U32<E>,
    pub module_index: crate::endian::U32<E>,
}
```

#### Fields

- **`symbol_index`**: `crate::endian::U32<E>`

  the defined external symbol (index into the symbol table)

- **`module_index`**: `crate::endian::U32<E>`

  index into the module table this symbol is defined in

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DylibTableOfContents<E>`

- `fn clone(self: &Self) -> DylibTableOfContents<E>` — [`DylibTableOfContents`](#dylibtableofcontents)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DylibTableOfContents<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DylibTableOfContents<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DylibTableOfContents<E>`

### `DylibModule32<E: Endian>`

```rust
struct DylibModule32<E: Endian> {
    pub module_name: crate::endian::U32<E>,
    pub iextdefsym: crate::endian::U32<E>,
    pub nextdefsym: crate::endian::U32<E>,
    pub irefsym: crate::endian::U32<E>,
    pub nrefsym: crate::endian::U32<E>,
    pub ilocalsym: crate::endian::U32<E>,
    pub nlocalsym: crate::endian::U32<E>,
    pub iextrel: crate::endian::U32<E>,
    pub nextrel: crate::endian::U32<E>,
    pub iinit_iterm: crate::endian::U32<E>,
    pub ninit_nterm: crate::endian::U32<E>,
    pub objc_module_info_addr: crate::endian::U32<E>,
    pub objc_module_info_size: crate::endian::U32<E>,
}
```

#### Fields

- **`module_name`**: `crate::endian::U32<E>`

  the module name (index into string table)

- **`iextdefsym`**: `crate::endian::U32<E>`

  index into externally defined symbols

- **`nextdefsym`**: `crate::endian::U32<E>`

  number of externally defined symbols

- **`irefsym`**: `crate::endian::U32<E>`

  index into reference symbol table

- **`nrefsym`**: `crate::endian::U32<E>`

  number of reference symbol table entries

- **`ilocalsym`**: `crate::endian::U32<E>`

  index into symbols for local symbols

- **`nlocalsym`**: `crate::endian::U32<E>`

  number of local symbols

- **`iextrel`**: `crate::endian::U32<E>`

  index into external relocation entries

- **`nextrel`**: `crate::endian::U32<E>`

  number of external relocation entries

- **`iinit_iterm`**: `crate::endian::U32<E>`

  low 16 bits are the index into the init section, high 16 bits are the index into the term section

- **`ninit_nterm`**: `crate::endian::U32<E>`

  low 16 bits are the number of init section entries, high 16 bits are the number of term section entries

- **`objc_module_info_addr`**: `crate::endian::U32<E>`

  for this module address of the start of the (__OBJC,__module_info) section

- **`objc_module_info_size`**: `crate::endian::U32<E>`

  for this module size of the (__OBJC,__module_info) section

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DylibModule32<E>`

- `fn clone(self: &Self) -> DylibModule32<E>` — [`DylibModule32`](#dylibmodule32)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DylibModule32<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DylibModule32<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DylibModule32<E>`

### `DylibModule64<E: Endian>`

```rust
struct DylibModule64<E: Endian> {
    pub module_name: crate::endian::U32<E>,
    pub iextdefsym: crate::endian::U32<E>,
    pub nextdefsym: crate::endian::U32<E>,
    pub irefsym: crate::endian::U32<E>,
    pub nrefsym: crate::endian::U32<E>,
    pub ilocalsym: crate::endian::U32<E>,
    pub nlocalsym: crate::endian::U32<E>,
    pub iextrel: crate::endian::U32<E>,
    pub nextrel: crate::endian::U32<E>,
    pub iinit_iterm: crate::endian::U32<E>,
    pub ninit_nterm: crate::endian::U32<E>,
    pub objc_module_info_size: crate::endian::U32<E>,
    pub objc_module_info_addr: crate::endian::U64<E>,
}
```

#### Fields

- **`module_name`**: `crate::endian::U32<E>`

  the module name (index into string table)

- **`iextdefsym`**: `crate::endian::U32<E>`

  index into externally defined symbols

- **`nextdefsym`**: `crate::endian::U32<E>`

  number of externally defined symbols

- **`irefsym`**: `crate::endian::U32<E>`

  index into reference symbol table

- **`nrefsym`**: `crate::endian::U32<E>`

  number of reference symbol table entries

- **`ilocalsym`**: `crate::endian::U32<E>`

  index into symbols for local symbols

- **`nlocalsym`**: `crate::endian::U32<E>`

  number of local symbols

- **`iextrel`**: `crate::endian::U32<E>`

  index into external relocation entries

- **`nextrel`**: `crate::endian::U32<E>`

  number of external relocation entries

- **`iinit_iterm`**: `crate::endian::U32<E>`

  low 16 bits are the index into the init section, high 16 bits are the index into the term section

- **`ninit_nterm`**: `crate::endian::U32<E>`

  low 16 bits are the number of init section entries, high 16 bits are the number of term section entries

- **`objc_module_info_size`**: `crate::endian::U32<E>`

  for this module size of the (__OBJC,__module_info) section

- **`objc_module_info_addr`**: `crate::endian::U64<E>`

  for this module address of the start of the (__OBJC,__module_info) section

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DylibModule64<E>`

- `fn clone(self: &Self) -> DylibModule64<E>` — [`DylibModule64`](#dylibmodule64)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DylibModule64<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DylibModule64<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DylibModule64<E>`

### `DylibReference<E: Endian>`

```rust
struct DylibReference<E: Endian> {
    pub bitfield: crate::endian::U32<E>,
}
```

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DylibReference<E>`

- `fn clone(self: &Self) -> DylibReference<E>` — [`DylibReference`](#dylibreference)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DylibReference<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DylibReference<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DylibReference<E>`

### `TwolevelHintsCommand<E: Endian>`

```rust
struct TwolevelHintsCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub offset: crate::endian::U32<E>,
    pub nhints: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_TWOLEVEL_HINTS

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct TwolevelHintsCommand)

- **`offset`**: `crate::endian::U32<E>`

  offset to the hint table

- **`nhints`**: `crate::endian::U32<E>`

  number of hints in the hint table

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for TwolevelHintsCommand<E>`

- `fn clone(self: &Self) -> TwolevelHintsCommand<E>` — [`TwolevelHintsCommand`](#twolevelhintscommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for TwolevelHintsCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for TwolevelHintsCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for TwolevelHintsCommand<E>`

### `TwolevelHint<E: Endian>`

```rust
struct TwolevelHint<E: Endian> {
    pub bitfield: crate::endian::U32<E>,
}
```

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for TwolevelHint<E>`

- `fn clone(self: &Self) -> TwolevelHint<E>` — [`TwolevelHint`](#twolevelhint)

##### `impl<E: $crate::marker::Copy + Endian> Copy for TwolevelHint<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for TwolevelHint<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for TwolevelHint<E>`

### `PrebindCksumCommand<E: Endian>`

```rust
struct PrebindCksumCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub cksum: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_PREBIND_CKSUM

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct PrebindCksumCommand)

- **`cksum`**: `crate::endian::U32<E>`

  the check sum or zero

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for PrebindCksumCommand<E>`

- `fn clone(self: &Self) -> PrebindCksumCommand<E>` — [`PrebindCksumCommand`](#prebindcksumcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for PrebindCksumCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for PrebindCksumCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for PrebindCksumCommand<E>`

### `UuidCommand<E: Endian>`

```rust
struct UuidCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub uuid: [u8; 16],
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_UUID

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct UuidCommand)

- **`uuid`**: `[u8; 16]`

  the 128-bit uuid

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for UuidCommand<E>`

- `fn clone(self: &Self) -> UuidCommand<E>` — [`UuidCommand`](#uuidcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for UuidCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for UuidCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for UuidCommand<E>`

### `RpathCommand<E: Endian>`

```rust
struct RpathCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub path: LcStr<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_RPATH

- **`cmdsize`**: `crate::endian::U32<E>`

  includes string

- **`path`**: `LcStr<E>`

  path to add to run path

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for RpathCommand<E>`

- `fn clone(self: &Self) -> RpathCommand<E>` — [`RpathCommand`](#rpathcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for RpathCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for RpathCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for RpathCommand<E>`

### `LinkeditDataCommand<E: Endian>`

```rust
struct LinkeditDataCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub dataoff: crate::endian::U32<E>,
    pub datasize: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,
  `LC_DATA_IN_CODE`, `LC_DYLIB_CODE_SIGN_DRS`, `LC_LINKER_OPTIMIZATION_HINT`,
  `LC_DYLD_EXPORTS_TRIE`, or `LC_DYLD_CHAINED_FIXUPS`.

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct LinkeditDataCommand)

- **`dataoff`**: `crate::endian::U32<E>`

  file offset of data in __LINKEDIT segment

- **`datasize`**: `crate::endian::U32<E>`

  file size of data in __LINKEDIT segment

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for LinkeditDataCommand<E>`

- `fn clone(self: &Self) -> LinkeditDataCommand<E>` — [`LinkeditDataCommand`](#linkeditdatacommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for LinkeditDataCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for LinkeditDataCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for LinkeditDataCommand<E>`

### `FilesetEntryCommand<E: Endian>`

```rust
struct FilesetEntryCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub vmaddr: crate::endian::U64<E>,
    pub fileoff: crate::endian::U64<E>,
    pub entry_id: LcStr<E>,
    pub reserved: crate::endian::U32<E>,
}
```

#### Fields

- **`cmdsize`**: `crate::endian::U32<E>`

  includes id string

- **`vmaddr`**: `crate::endian::U64<E>`

  memory address of the dylib

- **`fileoff`**: `crate::endian::U64<E>`

  file offset of the dylib

- **`entry_id`**: `LcStr<E>`

  contained entry id

- **`reserved`**: `crate::endian::U32<E>`

  entry_id is 32-bits long, so this is the reserved padding

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for FilesetEntryCommand<E>`

- `fn clone(self: &Self) -> FilesetEntryCommand<E>` — [`FilesetEntryCommand`](#filesetentrycommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for FilesetEntryCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for FilesetEntryCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for FilesetEntryCommand<E>`

### `EncryptionInfoCommand32<E: Endian>`

```rust
struct EncryptionInfoCommand32<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub cryptoff: crate::endian::U32<E>,
    pub cryptsize: crate::endian::U32<E>,
    pub cryptid: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ENCRYPTION_INFO

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct EncryptionInfoCommand32)

- **`cryptoff`**: `crate::endian::U32<E>`

  file offset of encrypted range

- **`cryptsize`**: `crate::endian::U32<E>`

  file size of encrypted range

- **`cryptid`**: `crate::endian::U32<E>`

  which enryption system, 0 means not-encrypted yet

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for EncryptionInfoCommand32<E>`

- `fn clone(self: &Self) -> EncryptionInfoCommand32<E>` — [`EncryptionInfoCommand32`](#encryptioninfocommand32)

##### `impl<E: $crate::marker::Copy + Endian> Copy for EncryptionInfoCommand32<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for EncryptionInfoCommand32<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for EncryptionInfoCommand32<E>`

### `EncryptionInfoCommand64<E: Endian>`

```rust
struct EncryptionInfoCommand64<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub cryptoff: crate::endian::U32<E>,
    pub cryptsize: crate::endian::U32<E>,
    pub cryptid: crate::endian::U32<E>,
    pub pad: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_ENCRYPTION_INFO_64

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct EncryptionInfoCommand64)

- **`cryptoff`**: `crate::endian::U32<E>`

  file offset of encrypted range

- **`cryptsize`**: `crate::endian::U32<E>`

  file size of encrypted range

- **`cryptid`**: `crate::endian::U32<E>`

  which enryption system, 0 means not-encrypted yet

- **`pad`**: `crate::endian::U32<E>`

  padding to make this struct's size a multiple of 8 bytes

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for EncryptionInfoCommand64<E>`

- `fn clone(self: &Self) -> EncryptionInfoCommand64<E>` — [`EncryptionInfoCommand64`](#encryptioninfocommand64)

##### `impl<E: $crate::marker::Copy + Endian> Copy for EncryptionInfoCommand64<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for EncryptionInfoCommand64<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for EncryptionInfoCommand64<E>`

### `VersionMinCommand<E: Endian>`

```rust
struct VersionMinCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub version: crate::endian::U32<E>,
    pub sdk: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_VERSION_MIN_MACOSX or LC_VERSION_MIN_IPHONEOS or LC_VERSION_MIN_WATCHOS or LC_VERSION_MIN_TVOS

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct VersionMinCommand)

- **`version`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

- **`sdk`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for VersionMinCommand<E>`

- `fn clone(self: &Self) -> VersionMinCommand<E>` — [`VersionMinCommand`](#versionmincommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for VersionMinCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for VersionMinCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for VersionMinCommand<E>`

### `BuildVersionCommand<E: Endian>`

```rust
struct BuildVersionCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub platform: crate::endian::U32<E>,
    pub minos: crate::endian::U32<E>,
    pub sdk: crate::endian::U32<E>,
    pub ntools: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_BUILD_VERSION

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct BuildVersionCommand) plus ntools * sizeof(struct BuildToolVersion)

- **`platform`**: `crate::endian::U32<E>`

  platform

- **`minos`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

- **`sdk`**: `crate::endian::U32<E>`

  X.Y.Z is encoded in nibbles xxxx.yy.zz

- **`ntools`**: `crate::endian::U32<E>`

  number of tool entries following this

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for BuildVersionCommand<E>`

- `fn clone(self: &Self) -> BuildVersionCommand<E>` — [`BuildVersionCommand`](#buildversioncommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for BuildVersionCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for BuildVersionCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for BuildVersionCommand<E>`

### `BuildToolVersion<E: Endian>`

```rust
struct BuildToolVersion<E: Endian> {
    pub tool: crate::endian::U32<E>,
    pub version: crate::endian::U32<E>,
}
```

#### Fields

- **`tool`**: `crate::endian::U32<E>`

  enum for the tool

- **`version`**: `crate::endian::U32<E>`

  version number of the tool

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for BuildToolVersion<E>`

- `fn clone(self: &Self) -> BuildToolVersion<E>` — [`BuildToolVersion`](#buildtoolversion)

##### `impl<E: $crate::marker::Copy + Endian> Copy for BuildToolVersion<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for BuildToolVersion<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for BuildToolVersion<E>`

### `DyldInfoCommand<E: Endian>`

```rust
struct DyldInfoCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub rebase_off: crate::endian::U32<E>,
    pub rebase_size: crate::endian::U32<E>,
    pub bind_off: crate::endian::U32<E>,
    pub bind_size: crate::endian::U32<E>,
    pub weak_bind_off: crate::endian::U32<E>,
    pub weak_bind_size: crate::endian::U32<E>,
    pub lazy_bind_off: crate::endian::U32<E>,
    pub lazy_bind_size: crate::endian::U32<E>,
    pub export_off: crate::endian::U32<E>,
    pub export_size: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_DYLD_INFO or LC_DYLD_INFO_ONLY

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct DyldInfoCommand)

- **`rebase_off`**: `crate::endian::U32<E>`

  file offset to rebase info

- **`rebase_size`**: `crate::endian::U32<E>`

  size of rebase info

- **`bind_off`**: `crate::endian::U32<E>`

  file offset to binding info

- **`bind_size`**: `crate::endian::U32<E>`

  size of binding info

- **`weak_bind_off`**: `crate::endian::U32<E>`

  file offset to weak binding info

- **`weak_bind_size`**: `crate::endian::U32<E>`

  size of weak binding info

- **`lazy_bind_off`**: `crate::endian::U32<E>`

  file offset to lazy binding info

- **`lazy_bind_size`**: `crate::endian::U32<E>`

  size of lazy binding infs

- **`export_off`**: `crate::endian::U32<E>`

  file offset to lazy binding info

- **`export_size`**: `crate::endian::U32<E>`

  size of lazy binding infs

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DyldInfoCommand<E>`

- `fn clone(self: &Self) -> DyldInfoCommand<E>` — [`DyldInfoCommand`](#dyldinfocommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DyldInfoCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DyldInfoCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DyldInfoCommand<E>`

### `LinkerOptionCommand<E: Endian>`

```rust
struct LinkerOptionCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub count: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_LINKER_OPTION only used in MH_OBJECT filetypes

- **`count`**: `crate::endian::U32<E>`

  number of strings

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for LinkerOptionCommand<E>`

- `fn clone(self: &Self) -> LinkerOptionCommand<E>` — [`LinkerOptionCommand`](#linkeroptioncommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for LinkerOptionCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for LinkerOptionCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for LinkerOptionCommand<E>`

### `SymsegCommand<E: Endian>`

```rust
struct SymsegCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub offset: crate::endian::U32<E>,
    pub size: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SYMSEG

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct SymsegCommand)

- **`offset`**: `crate::endian::U32<E>`

  symbol segment offset

- **`size`**: `crate::endian::U32<E>`

  symbol segment size in bytes

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SymsegCommand<E>`

- `fn clone(self: &Self) -> SymsegCommand<E>` — [`SymsegCommand`](#symsegcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SymsegCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SymsegCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SymsegCommand<E>`

### `IdentCommand<E: Endian>`

```rust
struct IdentCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_IDENT

- **`cmdsize`**: `crate::endian::U32<E>`

  strings that follow this command

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for IdentCommand<E>`

- `fn clone(self: &Self) -> IdentCommand<E>` — [`IdentCommand`](#identcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for IdentCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for IdentCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for IdentCommand<E>`

### `FvmfileCommand<E: Endian>`

```rust
struct FvmfileCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub name: LcStr<E>,
    pub header_addr: crate::endian::U32<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_FVMFILE

- **`cmdsize`**: `crate::endian::U32<E>`

  includes pathname string

- **`name`**: `LcStr<E>`

  files pathname

- **`header_addr`**: `crate::endian::U32<E>`

  files virtual address

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for FvmfileCommand<E>`

- `fn clone(self: &Self) -> FvmfileCommand<E>` — [`FvmfileCommand`](#fvmfilecommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for FvmfileCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for FvmfileCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for FvmfileCommand<E>`

### `EntryPointCommand<E: Endian>`

```rust
struct EntryPointCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub entryoff: crate::endian::U64<E>,
    pub stacksize: crate::endian::U64<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_MAIN only used in MH_EXECUTE filetypes

- **`cmdsize`**: `crate::endian::U32<E>`

  24

- **`entryoff`**: `crate::endian::U64<E>`

  file (__TEXT) offset of main()

- **`stacksize`**: `crate::endian::U64<E>`

  if not zero, initial stack size

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for EntryPointCommand<E>`

- `fn clone(self: &Self) -> EntryPointCommand<E>` — [`EntryPointCommand`](#entrypointcommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for EntryPointCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for EntryPointCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for EntryPointCommand<E>`

### `SourceVersionCommand<E: Endian>`

```rust
struct SourceVersionCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub version: crate::endian::U64<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_SOURCE_VERSION

- **`cmdsize`**: `crate::endian::U32<E>`

  16

- **`version`**: `crate::endian::U64<E>`

  A.B.C.D.E packed as a24.b10.c10.d10.e10

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for SourceVersionCommand<E>`

- `fn clone(self: &Self) -> SourceVersionCommand<E>` — [`SourceVersionCommand`](#sourceversioncommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for SourceVersionCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for SourceVersionCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for SourceVersionCommand<E>`

### `DataInCodeEntry<E: Endian>`

```rust
struct DataInCodeEntry<E: Endian> {
    pub offset: crate::endian::U32<E>,
    pub length: crate::endian::U16<E>,
    pub kind: crate::endian::U16<E>,
}
```

#### Fields

- **`offset`**: `crate::endian::U32<E>`

  from mach_header to start of data range

- **`length`**: `crate::endian::U16<E>`

  number of bytes in data range

- **`kind`**: `crate::endian::U16<E>`

  a DICE_KIND_* value

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for DataInCodeEntry<E>`

- `fn clone(self: &Self) -> DataInCodeEntry<E>` — [`DataInCodeEntry`](#dataincodeentry)

##### `impl<E: $crate::marker::Copy + Endian> Copy for DataInCodeEntry<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for DataInCodeEntry<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for DataInCodeEntry<E>`

### `NoteCommand<E: Endian>`

```rust
struct NoteCommand<E: Endian> {
    pub cmd: crate::endian::U32<E>,
    pub cmdsize: crate::endian::U32<E>,
    pub data_owner: [u8; 16],
    pub offset: crate::endian::U64<E>,
    pub size: crate::endian::U64<E>,
}
```

#### Fields

- **`cmd`**: `crate::endian::U32<E>`

  LC_NOTE

- **`cmdsize`**: `crate::endian::U32<E>`

  sizeof(struct NoteCommand)

- **`data_owner`**: `[u8; 16]`

  owner name for this LC_NOTE

- **`offset`**: `crate::endian::U64<E>`

  file offset of this data

- **`size`**: `crate::endian::U64<E>`

  length of data region

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for NoteCommand<E>`

- `fn clone(self: &Self) -> NoteCommand<E>` — [`NoteCommand`](#notecommand)

##### `impl<E: $crate::marker::Copy + Endian> Copy for NoteCommand<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for NoteCommand<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for NoteCommand<E>`

### `Nlist32<E: Endian>`

```rust
struct Nlist32<E: Endian> {
    pub n_strx: crate::endian::U32<E>,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: crate::endian::U16<E>,
    pub n_value: crate::endian::U32<E>,
}
```

#### Fields

- **`n_strx`**: `crate::endian::U32<E>`

  index into the string table

- **`n_type`**: `u8`

  type flag, see below

- **`n_sect`**: `u8`

  section number or NO_SECT

- **`n_desc`**: `crate::endian::U16<E>`

  see <mach-o/stab.h>

- **`n_value`**: `crate::endian::U32<E>`

  value of this symbol (or stab offset)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for Nlist32<E>`

- `fn clone(self: &Self) -> Nlist32<E>` — [`Nlist32`](#nlist32)

##### `impl<E: $crate::marker::Copy + Endian> Copy for Nlist32<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for Nlist32<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Endian: endian::Endian> Nlist for macho::Nlist32<Endian>`

- `type Word = u32`

- `type Endian = Endian`

- `fn n_strx(self: &Self, endian: <Self as >::Endian) -> u32` — [`Nlist`](../read/macho/index.md)

- `fn n_type(self: &Self) -> u8`

- `fn n_sect(self: &Self) -> u8`

- `fn n_desc(self: &Self, endian: <Self as >::Endian) -> u16` — [`Nlist`](../read/macho/index.md)

- `fn n_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Nlist`](../read/macho/index.md)

##### `impl<E: Endian> Pod for Nlist32<E>`

### `Nlist64<E: Endian>`

```rust
struct Nlist64<E: Endian> {
    pub n_strx: crate::endian::U32<E>,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: crate::endian::U16<E>,
    pub n_value: crate::endian::U64Bytes<E>,
}
```

#### Fields

- **`n_strx`**: `crate::endian::U32<E>`

  index into the string table

- **`n_type`**: `u8`

  type flag, see below

- **`n_sect`**: `u8`

  section number or NO_SECT

- **`n_desc`**: `crate::endian::U16<E>`

  see <mach-o/stab.h>

- **`n_value`**: `crate::endian::U64Bytes<E>`

  value of this symbol (or stab offset)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for Nlist64<E>`

- `fn clone(self: &Self) -> Nlist64<E>` — [`Nlist64`](#nlist64)

##### `impl<E: $crate::marker::Copy + Endian> Copy for Nlist64<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for Nlist64<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Endian: endian::Endian> Nlist for macho::Nlist64<Endian>`

- `type Word = u64`

- `type Endian = Endian`

- `fn n_strx(self: &Self, endian: <Self as >::Endian) -> u32` — [`Nlist`](../read/macho/index.md)

- `fn n_type(self: &Self) -> u8`

- `fn n_sect(self: &Self) -> u8`

- `fn n_desc(self: &Self, endian: <Self as >::Endian) -> u16` — [`Nlist`](../read/macho/index.md)

- `fn n_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word` — [`Nlist`](../read/macho/index.md)

##### `impl<E: Endian> Pod for Nlist64<E>`

### `Relocation<E: Endian>`

```rust
struct Relocation<E: Endian> {
    pub r_word0: crate::endian::U32<E>,
    pub r_word1: crate::endian::U32<E>,
}
```

A relocation entry.

Mach-O relocations have plain and scattered variants, with the
meaning of the fields depending on the variant.

This type provides functions for determining whether the relocation
is scattered, and for accessing the fields of each variant.

#### Implementations

- `fn r_scattered(self: Self, endian: E, cputype: u32) -> bool`

- `fn info(self: Self, endian: E) -> RelocationInfo` — [`RelocationInfo`](#relocationinfo)

- `fn scattered_info(self: Self, endian: E) -> ScatteredRelocationInfo` — [`ScatteredRelocationInfo`](#scatteredrelocationinfo)

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for Relocation<E>`

- `fn clone(self: &Self) -> Relocation<E>` — [`Relocation`](#relocation)

##### `impl<E: $crate::marker::Copy + Endian> Copy for Relocation<E>`

##### `impl<E: $crate::fmt::Debug + Endian> Debug for Relocation<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: Endian> Pod for Relocation<E>`

### `RelocationInfo`

```rust
struct RelocationInfo {
    pub r_address: u32,
    pub r_symbolnum: u32,
    pub r_pcrel: bool,
    pub r_length: u8,
    pub r_extern: bool,
    pub r_type: u8,
}
```

#### Fields

- **`r_address`**: `u32`

  offset in the section to what is being relocated

- **`r_symbolnum`**: `u32`

  symbol index if r_extern == 1 or section ordinal if r_extern == 0

- **`r_pcrel`**: `bool`

  was relocated pc relative already

- **`r_length`**: `u8`

  0=byte, 1=word, 2=long, 3=quad

- **`r_extern`**: `bool`

  does not include value of sym referenced

- **`r_type`**: `u8`

  if not 0, machine specific relocation type

#### Implementations

- `fn relocation<E: Endian>(self: Self, endian: E) -> Relocation<E>` — [`Relocation`](#relocation)

#### Trait Implementations

##### `impl Clone for RelocationInfo`

- `fn clone(self: &Self) -> RelocationInfo` — [`RelocationInfo`](#relocationinfo)

##### `impl Copy for RelocationInfo`

##### `impl Debug for RelocationInfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ScatteredRelocationInfo`

```rust
struct ScatteredRelocationInfo {
    pub r_address: u32,
    pub r_type: u8,
    pub r_length: u8,
    pub r_pcrel: bool,
    pub r_value: u32,
}
```

#### Fields

- **`r_address`**: `u32`

  offset in the section to what is being relocated

- **`r_type`**: `u8`

  if not 0, machine specific relocation type

- **`r_length`**: `u8`

  0=byte, 1=word, 2=long, 3=quad

- **`r_pcrel`**: `bool`

  was relocated pc relative already

- **`r_value`**: `u32`

  the value the item to be relocated is referring to (without any offset added)

#### Implementations

- `fn relocation<E: Endian>(self: Self, endian: E) -> Relocation<E>` — [`Relocation`](#relocation)

#### Trait Implementations

##### `impl Clone for ScatteredRelocationInfo`

- `fn clone(self: &Self) -> ScatteredRelocationInfo` — [`ScatteredRelocationInfo`](#scatteredrelocationinfo)

##### `impl Copy for ScatteredRelocationInfo`

##### `impl Debug for ScatteredRelocationInfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `PtrauthKey`

```rust
enum PtrauthKey {
    IA,
    IB,
    DA,
    DB,
}
```

The key used to sign a pointer for authentication.

The variant values correspond to the values used in the
`ptrauth_key` enum in `ptrauth.h`.

#### Variants

- **`IA`**

  Instruction key A.

- **`IB`**

  Instruction key B.

- **`DA`**

  Data key A.

- **`DB`**

  Data key B.

#### Trait Implementations

##### `impl Clone for PtrauthKey`

- `fn clone(self: &Self) -> PtrauthKey` — [`PtrauthKey`](#ptrauthkey)

##### `impl Copy for PtrauthKey`

##### `impl Debug for PtrauthKey`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for PtrauthKey`

##### `impl PartialEq for PtrauthKey`

- `fn eq(self: &Self, other: &PtrauthKey) -> bool` — [`PtrauthKey`](#ptrauthkey)

##### `impl StructuralPartialEq for PtrauthKey`

## Functions

### `cpu_subtype_intel`

```rust
const fn cpu_subtype_intel(f: u32, m: u32) -> u32
```

### `cpu_subtype_intel_family`

```rust
const fn cpu_subtype_intel_family(x: u32) -> u32
```

### `cpu_subtype_intel_model`

```rust
const fn cpu_subtype_intel_model(x: u32) -> u32
```

## Constants

### `CPU_ARCH_MASK`

```rust
const CPU_ARCH_MASK: u32 = 4_278_190_080u32;
```

mask for architecture bits

### `CPU_ARCH_ABI64`

```rust
const CPU_ARCH_ABI64: u32 = 16_777_216u32;
```

64 bit ABI

### `CPU_ARCH_ABI64_32`

```rust
const CPU_ARCH_ABI64_32: u32 = 33_554_432u32;
```

ABI for 64-bit hardware with 32-bit types; LP32

### `CPU_TYPE_ANY`

```rust
const CPU_TYPE_ANY: u32 = 4_294_967_295u32;
```

### `CPU_TYPE_VAX`

```rust
const CPU_TYPE_VAX: u32 = 1u32;
```

### `CPU_TYPE_MC680X0`

```rust
const CPU_TYPE_MC680X0: u32 = 6u32;
```

### `CPU_TYPE_X86`

```rust
const CPU_TYPE_X86: u32 = 7u32;
```

### `CPU_TYPE_X86_64`

```rust
const CPU_TYPE_X86_64: u32 = 16_777_223u32;
```

### `CPU_TYPE_MIPS`

```rust
const CPU_TYPE_MIPS: u32 = 8u32;
```

### `CPU_TYPE_MC98000`

```rust
const CPU_TYPE_MC98000: u32 = 10u32;
```

### `CPU_TYPE_HPPA`

```rust
const CPU_TYPE_HPPA: u32 = 11u32;
```

### `CPU_TYPE_ARM`

```rust
const CPU_TYPE_ARM: u32 = 12u32;
```

### `CPU_TYPE_ARM64`

```rust
const CPU_TYPE_ARM64: u32 = 16_777_228u32;
```

### `CPU_TYPE_ARM64_32`

```rust
const CPU_TYPE_ARM64_32: u32 = 33_554_444u32;
```

### `CPU_TYPE_MC88000`

```rust
const CPU_TYPE_MC88000: u32 = 13u32;
```

### `CPU_TYPE_SPARC`

```rust
const CPU_TYPE_SPARC: u32 = 14u32;
```

### `CPU_TYPE_I860`

```rust
const CPU_TYPE_I860: u32 = 15u32;
```

### `CPU_TYPE_ALPHA`

```rust
const CPU_TYPE_ALPHA: u32 = 16u32;
```

### `CPU_TYPE_POWERPC`

```rust
const CPU_TYPE_POWERPC: u32 = 18u32;
```

### `CPU_TYPE_POWERPC64`

```rust
const CPU_TYPE_POWERPC64: u32 = 16_777_234u32;
```

### `CPU_SUBTYPE_MASK`

```rust
const CPU_SUBTYPE_MASK: u32 = 4_278_190_080u32;
```

mask for feature flags

### `CPU_SUBTYPE_LIB64`

```rust
const CPU_SUBTYPE_LIB64: u32 = 2_147_483_648u32;
```

64 bit libraries

### `CPU_SUBTYPE_PTRAUTH_ABI`

```rust
const CPU_SUBTYPE_PTRAUTH_ABI: u32 = 2_147_483_648u32;
```

pointer authentication with versioned ABI

### `CPU_SUBTYPE_ANY`

```rust
const CPU_SUBTYPE_ANY: u32 = 4_294_967_295u32;
```

When selecting a slice, ANY will pick the slice with the best
grading for the selected cpu_type_t, unlike the "ALL" subtypes,
which are the slices that can run on any hardware for that cpu type.

### `CPU_SUBTYPE_MULTIPLE`

```rust
const CPU_SUBTYPE_MULTIPLE: u32 = 4_294_967_295u32;
```

### `CPU_SUBTYPE_LITTLE_ENDIAN`

```rust
const CPU_SUBTYPE_LITTLE_ENDIAN: u32 = 0u32;
```

### `CPU_SUBTYPE_BIG_ENDIAN`

```rust
const CPU_SUBTYPE_BIG_ENDIAN: u32 = 1u32;
```

### `CPU_SUBTYPE_VAX_ALL`

```rust
const CPU_SUBTYPE_VAX_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_VAX780`

```rust
const CPU_SUBTYPE_VAX780: u32 = 1u32;
```

### `CPU_SUBTYPE_VAX785`

```rust
const CPU_SUBTYPE_VAX785: u32 = 2u32;
```

### `CPU_SUBTYPE_VAX750`

```rust
const CPU_SUBTYPE_VAX750: u32 = 3u32;
```

### `CPU_SUBTYPE_VAX730`

```rust
const CPU_SUBTYPE_VAX730: u32 = 4u32;
```

### `CPU_SUBTYPE_UVAXI`

```rust
const CPU_SUBTYPE_UVAXI: u32 = 5u32;
```

### `CPU_SUBTYPE_UVAXII`

```rust
const CPU_SUBTYPE_UVAXII: u32 = 6u32;
```

### `CPU_SUBTYPE_VAX8200`

```rust
const CPU_SUBTYPE_VAX8200: u32 = 7u32;
```

### `CPU_SUBTYPE_VAX8500`

```rust
const CPU_SUBTYPE_VAX8500: u32 = 8u32;
```

### `CPU_SUBTYPE_VAX8600`

```rust
const CPU_SUBTYPE_VAX8600: u32 = 9u32;
```

### `CPU_SUBTYPE_VAX8650`

```rust
const CPU_SUBTYPE_VAX8650: u32 = 10u32;
```

### `CPU_SUBTYPE_VAX8800`

```rust
const CPU_SUBTYPE_VAX8800: u32 = 11u32;
```

### `CPU_SUBTYPE_UVAXIII`

```rust
const CPU_SUBTYPE_UVAXIII: u32 = 12u32;
```

### `CPU_SUBTYPE_MC680X0_ALL`

```rust
const CPU_SUBTYPE_MC680X0_ALL: u32 = 1u32;
```

### `CPU_SUBTYPE_MC68030`

```rust
const CPU_SUBTYPE_MC68030: u32 = 1u32;
```

### `CPU_SUBTYPE_MC68040`

```rust
const CPU_SUBTYPE_MC68040: u32 = 2u32;
```

### `CPU_SUBTYPE_MC68030_ONLY`

```rust
const CPU_SUBTYPE_MC68030_ONLY: u32 = 3u32;
```

### `CPU_SUBTYPE_I386_ALL`

```rust
const CPU_SUBTYPE_I386_ALL: u32 = 3u32;
```

### `CPU_SUBTYPE_386`

```rust
const CPU_SUBTYPE_386: u32 = 3u32;
```

### `CPU_SUBTYPE_486`

```rust
const CPU_SUBTYPE_486: u32 = 4u32;
```

### `CPU_SUBTYPE_486SX`

```rust
const CPU_SUBTYPE_486SX: u32 = 132u32;
```

### `CPU_SUBTYPE_586`

```rust
const CPU_SUBTYPE_586: u32 = 5u32;
```

### `CPU_SUBTYPE_PENT`

```rust
const CPU_SUBTYPE_PENT: u32 = 5u32;
```

### `CPU_SUBTYPE_PENTPRO`

```rust
const CPU_SUBTYPE_PENTPRO: u32 = 22u32;
```

### `CPU_SUBTYPE_PENTII_M3`

```rust
const CPU_SUBTYPE_PENTII_M3: u32 = 54u32;
```

### `CPU_SUBTYPE_PENTII_M5`

```rust
const CPU_SUBTYPE_PENTII_M5: u32 = 86u32;
```

### `CPU_SUBTYPE_CELERON`

```rust
const CPU_SUBTYPE_CELERON: u32 = 103u32;
```

### `CPU_SUBTYPE_CELERON_MOBILE`

```rust
const CPU_SUBTYPE_CELERON_MOBILE: u32 = 119u32;
```

### `CPU_SUBTYPE_PENTIUM_3`

```rust
const CPU_SUBTYPE_PENTIUM_3: u32 = 8u32;
```

### `CPU_SUBTYPE_PENTIUM_3_M`

```rust
const CPU_SUBTYPE_PENTIUM_3_M: u32 = 24u32;
```

### `CPU_SUBTYPE_PENTIUM_3_XEON`

```rust
const CPU_SUBTYPE_PENTIUM_3_XEON: u32 = 40u32;
```

### `CPU_SUBTYPE_PENTIUM_M`

```rust
const CPU_SUBTYPE_PENTIUM_M: u32 = 9u32;
```

### `CPU_SUBTYPE_PENTIUM_4`

```rust
const CPU_SUBTYPE_PENTIUM_4: u32 = 10u32;
```

### `CPU_SUBTYPE_PENTIUM_4_M`

```rust
const CPU_SUBTYPE_PENTIUM_4_M: u32 = 26u32;
```

### `CPU_SUBTYPE_ITANIUM`

```rust
const CPU_SUBTYPE_ITANIUM: u32 = 11u32;
```

### `CPU_SUBTYPE_ITANIUM_2`

```rust
const CPU_SUBTYPE_ITANIUM_2: u32 = 27u32;
```

### `CPU_SUBTYPE_XEON`

```rust
const CPU_SUBTYPE_XEON: u32 = 12u32;
```

### `CPU_SUBTYPE_XEON_MP`

```rust
const CPU_SUBTYPE_XEON_MP: u32 = 28u32;
```

### `CPU_SUBTYPE_INTEL_FAMILY_MAX`

```rust
const CPU_SUBTYPE_INTEL_FAMILY_MAX: u32 = 15u32;
```

### `CPU_SUBTYPE_INTEL_MODEL_ALL`

```rust
const CPU_SUBTYPE_INTEL_MODEL_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_X86_ALL`

```rust
const CPU_SUBTYPE_X86_ALL: u32 = 3u32;
```

### `CPU_SUBTYPE_X86_64_ALL`

```rust
const CPU_SUBTYPE_X86_64_ALL: u32 = 3u32;
```

### `CPU_SUBTYPE_X86_ARCH1`

```rust
const CPU_SUBTYPE_X86_ARCH1: u32 = 4u32;
```

### `CPU_SUBTYPE_X86_64_H`

```rust
const CPU_SUBTYPE_X86_64_H: u32 = 8u32;
```

Haswell feature subset

### `CPU_SUBTYPE_MIPS_ALL`

```rust
const CPU_SUBTYPE_MIPS_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_MIPS_R2300`

```rust
const CPU_SUBTYPE_MIPS_R2300: u32 = 1u32;
```

### `CPU_SUBTYPE_MIPS_R2600`

```rust
const CPU_SUBTYPE_MIPS_R2600: u32 = 2u32;
```

### `CPU_SUBTYPE_MIPS_R2800`

```rust
const CPU_SUBTYPE_MIPS_R2800: u32 = 3u32;
```

### `CPU_SUBTYPE_MIPS_R2000A`

```rust
const CPU_SUBTYPE_MIPS_R2000A: u32 = 4u32;
```

pmax

### `CPU_SUBTYPE_MIPS_R2000`

```rust
const CPU_SUBTYPE_MIPS_R2000: u32 = 5u32;
```

### `CPU_SUBTYPE_MIPS_R3000A`

```rust
const CPU_SUBTYPE_MIPS_R3000A: u32 = 6u32;
```

3max

### `CPU_SUBTYPE_MIPS_R3000`

```rust
const CPU_SUBTYPE_MIPS_R3000: u32 = 7u32;
```

### `CPU_SUBTYPE_MC98000_ALL`

```rust
const CPU_SUBTYPE_MC98000_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_MC98601`

```rust
const CPU_SUBTYPE_MC98601: u32 = 1u32;
```

### `CPU_SUBTYPE_HPPA_ALL`

```rust
const CPU_SUBTYPE_HPPA_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_HPPA_7100LC`

```rust
const CPU_SUBTYPE_HPPA_7100LC: u32 = 1u32;
```

### `CPU_SUBTYPE_MC88000_ALL`

```rust
const CPU_SUBTYPE_MC88000_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_MC88100`

```rust
const CPU_SUBTYPE_MC88100: u32 = 1u32;
```

### `CPU_SUBTYPE_MC88110`

```rust
const CPU_SUBTYPE_MC88110: u32 = 2u32;
```

### `CPU_SUBTYPE_SPARC_ALL`

```rust
const CPU_SUBTYPE_SPARC_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_I860_ALL`

```rust
const CPU_SUBTYPE_I860_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_I860_860`

```rust
const CPU_SUBTYPE_I860_860: u32 = 1u32;
```

### `CPU_SUBTYPE_POWERPC_ALL`

```rust
const CPU_SUBTYPE_POWERPC_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_POWERPC_601`

```rust
const CPU_SUBTYPE_POWERPC_601: u32 = 1u32;
```

### `CPU_SUBTYPE_POWERPC_602`

```rust
const CPU_SUBTYPE_POWERPC_602: u32 = 2u32;
```

### `CPU_SUBTYPE_POWERPC_603`

```rust
const CPU_SUBTYPE_POWERPC_603: u32 = 3u32;
```

### `CPU_SUBTYPE_POWERPC_603E`

```rust
const CPU_SUBTYPE_POWERPC_603E: u32 = 4u32;
```

### `CPU_SUBTYPE_POWERPC_603EV`

```rust
const CPU_SUBTYPE_POWERPC_603EV: u32 = 5u32;
```

### `CPU_SUBTYPE_POWERPC_604`

```rust
const CPU_SUBTYPE_POWERPC_604: u32 = 6u32;
```

### `CPU_SUBTYPE_POWERPC_604E`

```rust
const CPU_SUBTYPE_POWERPC_604E: u32 = 7u32;
```

### `CPU_SUBTYPE_POWERPC_620`

```rust
const CPU_SUBTYPE_POWERPC_620: u32 = 8u32;
```

### `CPU_SUBTYPE_POWERPC_750`

```rust
const CPU_SUBTYPE_POWERPC_750: u32 = 9u32;
```

### `CPU_SUBTYPE_POWERPC_7400`

```rust
const CPU_SUBTYPE_POWERPC_7400: u32 = 10u32;
```

### `CPU_SUBTYPE_POWERPC_7450`

```rust
const CPU_SUBTYPE_POWERPC_7450: u32 = 11u32;
```

### `CPU_SUBTYPE_POWERPC_970`

```rust
const CPU_SUBTYPE_POWERPC_970: u32 = 100u32;
```

### `CPU_SUBTYPE_ARM_ALL`

```rust
const CPU_SUBTYPE_ARM_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_ARM_V4T`

```rust
const CPU_SUBTYPE_ARM_V4T: u32 = 5u32;
```

### `CPU_SUBTYPE_ARM_V6`

```rust
const CPU_SUBTYPE_ARM_V6: u32 = 6u32;
```

### `CPU_SUBTYPE_ARM_V5TEJ`

```rust
const CPU_SUBTYPE_ARM_V5TEJ: u32 = 7u32;
```

### `CPU_SUBTYPE_ARM_XSCALE`

```rust
const CPU_SUBTYPE_ARM_XSCALE: u32 = 8u32;
```

### `CPU_SUBTYPE_ARM_V7`

```rust
const CPU_SUBTYPE_ARM_V7: u32 = 9u32;
```

ARMv7-A and ARMv7-R

### `CPU_SUBTYPE_ARM_V7F`

```rust
const CPU_SUBTYPE_ARM_V7F: u32 = 10u32;
```

Cortex A9

### `CPU_SUBTYPE_ARM_V7S`

```rust
const CPU_SUBTYPE_ARM_V7S: u32 = 11u32;
```

Swift

### `CPU_SUBTYPE_ARM_V7K`

```rust
const CPU_SUBTYPE_ARM_V7K: u32 = 12u32;
```

### `CPU_SUBTYPE_ARM_V8`

```rust
const CPU_SUBTYPE_ARM_V8: u32 = 13u32;
```

### `CPU_SUBTYPE_ARM_V6M`

```rust
const CPU_SUBTYPE_ARM_V6M: u32 = 14u32;
```

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM_V7M`

```rust
const CPU_SUBTYPE_ARM_V7M: u32 = 15u32;
```

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM_V7EM`

```rust
const CPU_SUBTYPE_ARM_V7EM: u32 = 16u32;
```

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM_V8M`

```rust
const CPU_SUBTYPE_ARM_V8M: u32 = 17u32;
```

Not meant to be run under xnu

### `CPU_SUBTYPE_ARM64_ALL`

```rust
const CPU_SUBTYPE_ARM64_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_ARM64_V8`

```rust
const CPU_SUBTYPE_ARM64_V8: u32 = 1u32;
```

### `CPU_SUBTYPE_ARM64E`

```rust
const CPU_SUBTYPE_ARM64E: u32 = 2u32;
```

### `CPU_SUBTYPE_ARM64_32_ALL`

```rust
const CPU_SUBTYPE_ARM64_32_ALL: u32 = 0u32;
```

### `CPU_SUBTYPE_ARM64_32_V8`

```rust
const CPU_SUBTYPE_ARM64_32_V8: u32 = 1u32;
```

### `VM_PROT_READ`

```rust
const VM_PROT_READ: u32 = 1u32;
```

read permission

### `VM_PROT_WRITE`

```rust
const VM_PROT_WRITE: u32 = 2u32;
```

write permission

### `VM_PROT_EXECUTE`

```rust
const VM_PROT_EXECUTE: u32 = 4u32;
```

execute permission

### `DYLD_CACHE_MAPPING_AUTH_DATA`

```rust
const DYLD_CACHE_MAPPING_AUTH_DATA: u64 = 1u64;
```

### `DYLD_CACHE_MAPPING_DIRTY_DATA`

```rust
const DYLD_CACHE_MAPPING_DIRTY_DATA: u64 = 2u64;
```

### `DYLD_CACHE_MAPPING_CONST_DATA`

```rust
const DYLD_CACHE_MAPPING_CONST_DATA: u64 = 4u64;
```

### `DYLD_CACHE_MAPPING_TEXT_STUBS`

```rust
const DYLD_CACHE_MAPPING_TEXT_STUBS: u64 = 8u64;
```

### `DYLD_CACHE_DYNAMIC_CONFIG_DATA`

```rust
const DYLD_CACHE_DYNAMIC_CONFIG_DATA: u64 = 16u64;
```

### `DYLD_CACHE_SLIDE_PAGE_ATTRS`

```rust
const DYLD_CACHE_SLIDE_PAGE_ATTRS: u16 = 49_152u16;
```

### `DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA`

```rust
const DYLD_CACHE_SLIDE_PAGE_ATTR_EXTRA: u16 = 32_768u16;
```

### `DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE`

```rust
const DYLD_CACHE_SLIDE_PAGE_ATTR_NO_REBASE: u16 = 16_384u16;
```

### `DYLD_CACHE_SLIDE_PAGE_ATTR_END`

```rust
const DYLD_CACHE_SLIDE_PAGE_ATTR_END: u16 = 32_768u16;
```

### `DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE`

```rust
const DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE: u16 = 65_535u16;
```

Page has no rebasing.

### `DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE`

```rust
const DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE: u16 = 65_535u16;
```

Page has no rebasing.

### `FAT_MAGIC`

```rust
const FAT_MAGIC: u32 = 3_405_691_582u32;
```

### `FAT_CIGAM`

```rust
const FAT_CIGAM: u32 = 3_199_925_962u32;
```

NXSwapLong(FAT_MAGIC)

### `FAT_MAGIC_64`

```rust
const FAT_MAGIC_64: u32 = 3_405_691_583u32;
```

### `FAT_CIGAM_64`

```rust
const FAT_CIGAM_64: u32 = 3_216_703_178u32;
```

NXSwapLong(FAT_MAGIC_64)

### `MH_MAGIC`

```rust
const MH_MAGIC: u32 = 4_277_009_102u32;
```

the mach magic number

### `MH_CIGAM`

```rust
const MH_CIGAM: u32 = 3_472_551_422u32;
```

NXSwapInt(MH_MAGIC)

### `MH_MAGIC_64`

```rust
const MH_MAGIC_64: u32 = 4_277_009_103u32;
```

the 64-bit mach magic number

### `MH_CIGAM_64`

```rust
const MH_CIGAM_64: u32 = 3_489_328_638u32;
```

NXSwapInt(MH_MAGIC_64)

### `MH_OBJECT`

```rust
const MH_OBJECT: u32 = 1u32;
```

relocatable object file

### `MH_EXECUTE`

```rust
const MH_EXECUTE: u32 = 2u32;
```

demand paged executable file

### `MH_FVMLIB`

```rust
const MH_FVMLIB: u32 = 3u32;
```

fixed VM shared library file

### `MH_CORE`

```rust
const MH_CORE: u32 = 4u32;
```

core file

### `MH_PRELOAD`

```rust
const MH_PRELOAD: u32 = 5u32;
```

preloaded executable file

### `MH_DYLIB`

```rust
const MH_DYLIB: u32 = 6u32;
```

dynamically bound shared library

### `MH_DYLINKER`

```rust
const MH_DYLINKER: u32 = 7u32;
```

dynamic link editor

### `MH_BUNDLE`

```rust
const MH_BUNDLE: u32 = 8u32;
```

dynamically bound bundle file

### `MH_DYLIB_STUB`

```rust
const MH_DYLIB_STUB: u32 = 9u32;
```

shared library stub for static linking only, no section contents

### `MH_DSYM`

```rust
const MH_DSYM: u32 = 10u32;
```

companion file with only debug sections

### `MH_KEXT_BUNDLE`

```rust
const MH_KEXT_BUNDLE: u32 = 11u32;
```

x86_64 kexts

### `MH_FILESET`

```rust
const MH_FILESET: u32 = 12u32;
```

set of mach-o's

### `MH_NOUNDEFS`

```rust
const MH_NOUNDEFS: u32 = 1u32;
```

the object file has no undefined references

### `MH_INCRLINK`

```rust
const MH_INCRLINK: u32 = 2u32;
```

the object file is the output of an incremental link against a base file and can't be link edited again

### `MH_DYLDLINK`

```rust
const MH_DYLDLINK: u32 = 4u32;
```

the object file is input for the dynamic linker and can't be statically link edited again

### `MH_BINDATLOAD`

```rust
const MH_BINDATLOAD: u32 = 8u32;
```

the object file's undefined references are bound by the dynamic linker when loaded.

### `MH_PREBOUND`

```rust
const MH_PREBOUND: u32 = 16u32;
```

the file has its dynamic undefined references prebound.

### `MH_SPLIT_SEGS`

```rust
const MH_SPLIT_SEGS: u32 = 32u32;
```

the file has its read-only and read-write segments split

### `MH_LAZY_INIT`

```rust
const MH_LAZY_INIT: u32 = 64u32;
```

the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)

### `MH_TWOLEVEL`

```rust
const MH_TWOLEVEL: u32 = 128u32;
```

the image is using two-level name space bindings

### `MH_FORCE_FLAT`

```rust
const MH_FORCE_FLAT: u32 = 256u32;
```

the executable is forcing all images to use flat name space bindings

### `MH_NOMULTIDEFS`

```rust
const MH_NOMULTIDEFS: u32 = 512u32;
```

this umbrella guarantees no multiple definitions of symbols in its sub-images so the two-level namespace hints can always be used.

### `MH_NOFIXPREBINDING`

```rust
const MH_NOFIXPREBINDING: u32 = 1_024u32;
```

do not have dyld notify the prebinding agent about this executable

### `MH_PREBINDABLE`

```rust
const MH_PREBINDABLE: u32 = 2_048u32;
```

the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.

### `MH_ALLMODSBOUND`

```rust
const MH_ALLMODSBOUND: u32 = 4_096u32;
```

indicates that this binary binds to all two-level namespace modules of its dependent libraries. only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.

### `MH_SUBSECTIONS_VIA_SYMBOLS`

```rust
const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 8_192u32;
```

safe to divide up the sections into sub-sections via symbols for dead code stripping

### `MH_CANONICAL`

```rust
const MH_CANONICAL: u32 = 16_384u32;
```

the binary has been canonicalized via the unprebind operation

### `MH_WEAK_DEFINES`

```rust
const MH_WEAK_DEFINES: u32 = 32_768u32;
```

the final linked image contains external weak symbols

### `MH_BINDS_TO_WEAK`

```rust
const MH_BINDS_TO_WEAK: u32 = 65_536u32;
```

the final linked image uses weak symbols

### `MH_ALLOW_STACK_EXECUTION`

```rust
const MH_ALLOW_STACK_EXECUTION: u32 = 131_072u32;
```

When this bit is set, all stacks in the task will be given stack execution privilege.  Only used in MH_EXECUTE filetypes.

### `MH_ROOT_SAFE`

```rust
const MH_ROOT_SAFE: u32 = 262_144u32;
```

When this bit is set, the binary declares it is safe for use in processes with uid zero

### `MH_SETUID_SAFE`

```rust
const MH_SETUID_SAFE: u32 = 524_288u32;
```

When this bit is set, the binary declares it is safe for use in processes when issetugid() is true

### `MH_NO_REEXPORTED_DYLIBS`

```rust
const MH_NO_REEXPORTED_DYLIBS: u32 = 1_048_576u32;
```

When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported

### `MH_PIE`

```rust
const MH_PIE: u32 = 2_097_152u32;
```

When this bit is set, the OS will load the main executable at a random address.  Only used in MH_EXECUTE filetypes.

### `MH_DEAD_STRIPPABLE_DYLIB`

```rust
const MH_DEAD_STRIPPABLE_DYLIB: u32 = 4_194_304u32;
```

Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib.

### `MH_HAS_TLV_DESCRIPTORS`

```rust
const MH_HAS_TLV_DESCRIPTORS: u32 = 8_388_608u32;
```

Contains a section of type S_THREAD_LOCAL_VARIABLES

### `MH_NO_HEAP_EXECUTION`

```rust
const MH_NO_HEAP_EXECUTION: u32 = 16_777_216u32;
```

When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.

### `MH_APP_EXTENSION_SAFE`

```rust
const MH_APP_EXTENSION_SAFE: u32 = 33_554_432u32;
```

The code was linked for use in an application extension.

### `MH_NLIST_OUTOFSYNC_WITH_DYLDINFO`

```rust
const MH_NLIST_OUTOFSYNC_WITH_DYLDINFO: u32 = 67_108_864u32;
```

The external symbols listed in the nlist symbol table do not include all the symbols listed in the dyld info.

### `MH_SIM_SUPPORT`

```rust
const MH_SIM_SUPPORT: u32 = 134_217_728u32;
```

Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with
the platforms macOS, iOSMac, iOSSimulator, tvOSSimulator and watchOSSimulator.

### `MH_DYLIB_IN_CACHE`

```rust
const MH_DYLIB_IN_CACHE: u32 = 2_147_483_648u32;
```

Only for use on dylibs. When this bit is set, the dylib is part of the dyld
shared cache, rather than loose in the filesystem.

### `LC_REQ_DYLD`

```rust
const LC_REQ_DYLD: u32 = 2_147_483_648u32;
```

### `LC_SEGMENT`

```rust
const LC_SEGMENT: u32 = 1u32;
```

segment of this file to be mapped

### `LC_SYMTAB`

```rust
const LC_SYMTAB: u32 = 2u32;
```

link-edit stab symbol table info

### `LC_SYMSEG`

```rust
const LC_SYMSEG: u32 = 3u32;
```

link-edit gdb symbol table info (obsolete)

### `LC_THREAD`

```rust
const LC_THREAD: u32 = 4u32;
```

thread

### `LC_UNIXTHREAD`

```rust
const LC_UNIXTHREAD: u32 = 5u32;
```

unix thread (includes a stack)

### `LC_LOADFVMLIB`

```rust
const LC_LOADFVMLIB: u32 = 6u32;
```

load a specified fixed VM shared library

### `LC_IDFVMLIB`

```rust
const LC_IDFVMLIB: u32 = 7u32;
```

fixed VM shared library identification

### `LC_IDENT`

```rust
const LC_IDENT: u32 = 8u32;
```

object identification info (obsolete)

### `LC_FVMFILE`

```rust
const LC_FVMFILE: u32 = 9u32;
```

fixed VM file inclusion (internal use)

### `LC_PREPAGE`

```rust
const LC_PREPAGE: u32 = 10u32;
```

prepage command (internal use)

### `LC_DYSYMTAB`

```rust
const LC_DYSYMTAB: u32 = 11u32;
```

dynamic link-edit symbol table info

### `LC_LOAD_DYLIB`

```rust
const LC_LOAD_DYLIB: u32 = 12u32;
```

load a dynamically linked shared library

### `LC_ID_DYLIB`

```rust
const LC_ID_DYLIB: u32 = 13u32;
```

dynamically linked shared lib ident

### `LC_LOAD_DYLINKER`

```rust
const LC_LOAD_DYLINKER: u32 = 14u32;
```

load a dynamic linker

### `LC_ID_DYLINKER`

```rust
const LC_ID_DYLINKER: u32 = 15u32;
```

dynamic linker identification

### `LC_PREBOUND_DYLIB`

```rust
const LC_PREBOUND_DYLIB: u32 = 16u32;
```

modules prebound for a dynamically linked shared library

### `LC_ROUTINES`

```rust
const LC_ROUTINES: u32 = 17u32;
```

image routines

### `LC_SUB_FRAMEWORK`

```rust
const LC_SUB_FRAMEWORK: u32 = 18u32;
```

sub framework

### `LC_SUB_UMBRELLA`

```rust
const LC_SUB_UMBRELLA: u32 = 19u32;
```

sub umbrella

### `LC_SUB_CLIENT`

```rust
const LC_SUB_CLIENT: u32 = 20u32;
```

sub client

### `LC_SUB_LIBRARY`

```rust
const LC_SUB_LIBRARY: u32 = 21u32;
```

sub library

### `LC_TWOLEVEL_HINTS`

```rust
const LC_TWOLEVEL_HINTS: u32 = 22u32;
```

two-level namespace lookup hints

### `LC_PREBIND_CKSUM`

```rust
const LC_PREBIND_CKSUM: u32 = 23u32;
```

prebind checksum

### `LC_LOAD_WEAK_DYLIB`

```rust
const LC_LOAD_WEAK_DYLIB: u32 = 2_147_483_672u32;
```

load a dynamically linked shared library that is allowed to be missing
(all symbols are weak imported).

### `LC_SEGMENT_64`

```rust
const LC_SEGMENT_64: u32 = 25u32;
```

64-bit segment of this file to be mapped

### `LC_ROUTINES_64`

```rust
const LC_ROUTINES_64: u32 = 26u32;
```

64-bit image routines

### `LC_UUID`

```rust
const LC_UUID: u32 = 27u32;
```

the uuid

### `LC_RPATH`

```rust
const LC_RPATH: u32 = 2_147_483_676u32;
```

runpath additions

### `LC_CODE_SIGNATURE`

```rust
const LC_CODE_SIGNATURE: u32 = 29u32;
```

local of code signature

### `LC_SEGMENT_SPLIT_INFO`

```rust
const LC_SEGMENT_SPLIT_INFO: u32 = 30u32;
```

local of info to split segments

### `LC_REEXPORT_DYLIB`

```rust
const LC_REEXPORT_DYLIB: u32 = 2_147_483_679u32;
```

load and re-export dylib

### `LC_LAZY_LOAD_DYLIB`

```rust
const LC_LAZY_LOAD_DYLIB: u32 = 32u32;
```

delay load of dylib until first use

### `LC_ENCRYPTION_INFO`

```rust
const LC_ENCRYPTION_INFO: u32 = 33u32;
```

encrypted segment information

### `LC_DYLD_INFO`

```rust
const LC_DYLD_INFO: u32 = 34u32;
```

compressed dyld information

### `LC_DYLD_INFO_ONLY`

```rust
const LC_DYLD_INFO_ONLY: u32 = 2_147_483_682u32;
```

compressed dyld information only

### `LC_LOAD_UPWARD_DYLIB`

```rust
const LC_LOAD_UPWARD_DYLIB: u32 = 2_147_483_683u32;
```

load upward dylib

### `LC_VERSION_MIN_MACOSX`

```rust
const LC_VERSION_MIN_MACOSX: u32 = 36u32;
```

build for MacOSX min OS version

### `LC_VERSION_MIN_IPHONEOS`

```rust
const LC_VERSION_MIN_IPHONEOS: u32 = 37u32;
```

build for iPhoneOS min OS version

### `LC_FUNCTION_STARTS`

```rust
const LC_FUNCTION_STARTS: u32 = 38u32;
```

compressed table of function start addresses

### `LC_DYLD_ENVIRONMENT`

```rust
const LC_DYLD_ENVIRONMENT: u32 = 39u32;
```

string for dyld to treat like environment variable

### `LC_MAIN`

```rust
const LC_MAIN: u32 = 2_147_483_688u32;
```

replacement for LC_UNIXTHREAD

### `LC_DATA_IN_CODE`

```rust
const LC_DATA_IN_CODE: u32 = 41u32;
```

table of non-instructions in __text

### `LC_SOURCE_VERSION`

```rust
const LC_SOURCE_VERSION: u32 = 42u32;
```

source version used to build binary

### `LC_DYLIB_CODE_SIGN_DRS`

```rust
const LC_DYLIB_CODE_SIGN_DRS: u32 = 43u32;
```

Code signing DRs copied from linked dylibs

### `LC_ENCRYPTION_INFO_64`

```rust
const LC_ENCRYPTION_INFO_64: u32 = 44u32;
```

64-bit encrypted segment information

### `LC_LINKER_OPTION`

```rust
const LC_LINKER_OPTION: u32 = 45u32;
```

linker options in MH_OBJECT files

### `LC_LINKER_OPTIMIZATION_HINT`

```rust
const LC_LINKER_OPTIMIZATION_HINT: u32 = 46u32;
```

optimization hints in MH_OBJECT files

### `LC_VERSION_MIN_TVOS`

```rust
const LC_VERSION_MIN_TVOS: u32 = 47u32;
```

build for AppleTV min OS version

### `LC_VERSION_MIN_WATCHOS`

```rust
const LC_VERSION_MIN_WATCHOS: u32 = 48u32;
```

build for Watch min OS version

### `LC_NOTE`

```rust
const LC_NOTE: u32 = 49u32;
```

arbitrary data included within a Mach-O file

### `LC_BUILD_VERSION`

```rust
const LC_BUILD_VERSION: u32 = 50u32;
```

build for platform min OS version

### `LC_DYLD_EXPORTS_TRIE`

```rust
const LC_DYLD_EXPORTS_TRIE: u32 = 2_147_483_699u32;
```

used with `LinkeditDataCommand`, payload is trie

### `LC_DYLD_CHAINED_FIXUPS`

```rust
const LC_DYLD_CHAINED_FIXUPS: u32 = 2_147_483_700u32;
```

used with `LinkeditDataCommand`

### `LC_FILESET_ENTRY`

```rust
const LC_FILESET_ENTRY: u32 = 2_147_483_701u32;
```

used with `FilesetEntryCommand`

### `SG_HIGHVM`

```rust
const SG_HIGHVM: u32 = 1u32;
```

the file contents for this segment is for the high part of the VM space, the low part is zero filled (for stacks in core files)

### `SG_FVMLIB`

```rust
const SG_FVMLIB: u32 = 2u32;
```

this segment is the VM that is allocated by a fixed VM library, for overlap checking in the link editor

### `SG_NORELOC`

```rust
const SG_NORELOC: u32 = 4u32;
```

this segment has nothing that was relocated in it and nothing relocated to it, that is it maybe safely replaced without relocation

### `SG_PROTECTED_VERSION_1`

```rust
const SG_PROTECTED_VERSION_1: u32 = 8u32;
```

This segment is protected.  If the segment starts at file offset 0, the first page of the segment is not protected.  All other pages of the segment are protected.

### `SG_READ_ONLY`

```rust
const SG_READ_ONLY: u32 = 16u32;
```

This segment is made read-only after fixups

### `SECTION_TYPE`

```rust
const SECTION_TYPE: u32 = 255u32;
```

256 section types

### `SECTION_ATTRIBUTES`

```rust
const SECTION_ATTRIBUTES: u32 = 4_294_967_040u32;
```

24 section attributes

### `S_REGULAR`

```rust
const S_REGULAR: u32 = 0u32;
```

regular section

### `S_ZEROFILL`

```rust
const S_ZEROFILL: u32 = 1u32;
```

zero fill on demand section

### `S_CSTRING_LITERALS`

```rust
const S_CSTRING_LITERALS: u32 = 2u32;
```

section with only literal C strings

### `S_4BYTE_LITERALS`

```rust
const S_4BYTE_LITERALS: u32 = 3u32;
```

section with only 4 byte literals

### `S_8BYTE_LITERALS`

```rust
const S_8BYTE_LITERALS: u32 = 4u32;
```

section with only 8 byte literals

### `S_LITERAL_POINTERS`

```rust
const S_LITERAL_POINTERS: u32 = 5u32;
```

section with only pointers to literals

### `S_NON_LAZY_SYMBOL_POINTERS`

```rust
const S_NON_LAZY_SYMBOL_POINTERS: u32 = 6u32;
```

section with only non-lazy symbol pointers

### `S_LAZY_SYMBOL_POINTERS`

```rust
const S_LAZY_SYMBOL_POINTERS: u32 = 7u32;
```

section with only lazy symbol pointers

### `S_SYMBOL_STUBS`

```rust
const S_SYMBOL_STUBS: u32 = 8u32;
```

section with only symbol stubs, byte size of stub in the reserved2 field

### `S_MOD_INIT_FUNC_POINTERS`

```rust
const S_MOD_INIT_FUNC_POINTERS: u32 = 9u32;
```

section with only function pointers for initialization

### `S_MOD_TERM_FUNC_POINTERS`

```rust
const S_MOD_TERM_FUNC_POINTERS: u32 = 10u32;
```

section with only function pointers for termination

### `S_COALESCED`

```rust
const S_COALESCED: u32 = 11u32;
```

section contains symbols that are to be coalesced

### `S_GB_ZEROFILL`

```rust
const S_GB_ZEROFILL: u32 = 12u32;
```

zero fill on demand section (that can be larger than 4 gigabytes)

### `S_INTERPOSING`

```rust
const S_INTERPOSING: u32 = 13u32;
```

section with only pairs of function pointers for interposing

### `S_16BYTE_LITERALS`

```rust
const S_16BYTE_LITERALS: u32 = 14u32;
```

section with only 16 byte literals

### `S_DTRACE_DOF`

```rust
const S_DTRACE_DOF: u32 = 15u32;
```

section contains DTrace Object Format

### `S_LAZY_DYLIB_SYMBOL_POINTERS`

```rust
const S_LAZY_DYLIB_SYMBOL_POINTERS: u32 = 16u32;
```

section with only lazy symbol pointers to lazy loaded dylibs

### `S_THREAD_LOCAL_REGULAR`

```rust
const S_THREAD_LOCAL_REGULAR: u32 = 17u32;
```

template of initial values for TLVs

### `S_THREAD_LOCAL_ZEROFILL`

```rust
const S_THREAD_LOCAL_ZEROFILL: u32 = 18u32;
```

template of initial values for TLVs

### `S_THREAD_LOCAL_VARIABLES`

```rust
const S_THREAD_LOCAL_VARIABLES: u32 = 19u32;
```

TLV descriptors

### `S_THREAD_LOCAL_VARIABLE_POINTERS`

```rust
const S_THREAD_LOCAL_VARIABLE_POINTERS: u32 = 20u32;
```

pointers to TLV descriptors

### `S_THREAD_LOCAL_INIT_FUNCTION_POINTERS`

```rust
const S_THREAD_LOCAL_INIT_FUNCTION_POINTERS: u32 = 21u32;
```

functions to call to initialize TLV values

### `S_INIT_FUNC_OFFSETS`

```rust
const S_INIT_FUNC_OFFSETS: u32 = 22u32;
```

32-bit offsets to initializers

### `SECTION_ATTRIBUTES_USR`

```rust
const SECTION_ATTRIBUTES_USR: u32 = 4_278_190_080u32;
```

User setable attributes

### `S_ATTR_PURE_INSTRUCTIONS`

```rust
const S_ATTR_PURE_INSTRUCTIONS: u32 = 2_147_483_648u32;
```

section contains only true machine instructions

### `S_ATTR_NO_TOC`

```rust
const S_ATTR_NO_TOC: u32 = 1_073_741_824u32;
```

section contains coalesced symbols that are not to be in a ranlib table of contents

### `S_ATTR_STRIP_STATIC_SYMS`

```rust
const S_ATTR_STRIP_STATIC_SYMS: u32 = 536_870_912u32;
```

ok to strip static symbols in this section in files with the MH_DYLDLINK flag

### `S_ATTR_NO_DEAD_STRIP`

```rust
const S_ATTR_NO_DEAD_STRIP: u32 = 268_435_456u32;
```

no dead stripping

### `S_ATTR_LIVE_SUPPORT`

```rust
const S_ATTR_LIVE_SUPPORT: u32 = 134_217_728u32;
```

blocks are live if they reference live blocks

### `S_ATTR_SELF_MODIFYING_CODE`

```rust
const S_ATTR_SELF_MODIFYING_CODE: u32 = 67_108_864u32;
```

Used with i386 code stubs written on by dyld

### `S_ATTR_DEBUG`

```rust
const S_ATTR_DEBUG: u32 = 33_554_432u32;
```

a debug section

### `SECTION_ATTRIBUTES_SYS`

```rust
const SECTION_ATTRIBUTES_SYS: u32 = 16_776_960u32;
```

system setable attributes

### `S_ATTR_SOME_INSTRUCTIONS`

```rust
const S_ATTR_SOME_INSTRUCTIONS: u32 = 1_024u32;
```

section contains some machine instructions

### `S_ATTR_EXT_RELOC`

```rust
const S_ATTR_EXT_RELOC: u32 = 512u32;
```

section has external relocation entries

### `S_ATTR_LOC_RELOC`

```rust
const S_ATTR_LOC_RELOC: u32 = 256u32;
```

section has local relocation entries

### `SEG_PAGEZERO`

```rust
const SEG_PAGEZERO: &str;
```

the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files

### `SEG_TEXT`

```rust
const SEG_TEXT: &str;
```

the tradition UNIX text segment

### `SECT_TEXT`

```rust
const SECT_TEXT: &str;
```

the real text part of the text section no headers, and no padding

### `SECT_FVMLIB_INIT0`

```rust
const SECT_FVMLIB_INIT0: &str;
```

the fvmlib initialization section

### `SECT_FVMLIB_INIT1`

```rust
const SECT_FVMLIB_INIT1: &str;
```

the section following the fvmlib initialization section

### `SEG_DATA`

```rust
const SEG_DATA: &str;
```

the tradition UNIX data segment

### `SECT_DATA`

```rust
const SECT_DATA: &str;
```

the real initialized data section no padding, no bss overlap

### `SECT_BSS`

```rust
const SECT_BSS: &str;
```

the real uninitialized data section no padding

### `SECT_COMMON`

```rust
const SECT_COMMON: &str;
```

the section common symbols are allocated in by the link editor

### `SEG_OBJC`

```rust
const SEG_OBJC: &str;
```

objective-C runtime segment

### `SECT_OBJC_SYMBOLS`

```rust
const SECT_OBJC_SYMBOLS: &str;
```

symbol table

### `SECT_OBJC_MODULES`

```rust
const SECT_OBJC_MODULES: &str;
```

module information

### `SECT_OBJC_STRINGS`

```rust
const SECT_OBJC_STRINGS: &str;
```

string table

### `SECT_OBJC_REFS`

```rust
const SECT_OBJC_REFS: &str;
```

string table

### `SEG_ICON`

```rust
const SEG_ICON: &str;
```

the icon segment

### `SECT_ICON_HEADER`

```rust
const SECT_ICON_HEADER: &str;
```

the icon headers

### `SECT_ICON_TIFF`

```rust
const SECT_ICON_TIFF: &str;
```

the icons in tiff format

### `SEG_LINKEDIT`

```rust
const SEG_LINKEDIT: &str;
```

the segment containing all structs created and maintained by the link editor.  Created with -seglinkedit option to ld(1) for MH_EXECUTE and FVMLIB file types only

### `SEG_LINKINFO`

```rust
const SEG_LINKINFO: &str;
```

the segment overlapping with linkedit containing linking information

### `SEG_UNIXSTACK`

```rust
const SEG_UNIXSTACK: &str;
```

the unix stack segment

### `SEG_IMPORT`

```rust
const SEG_IMPORT: &str;
```

the segment for the self (dyld) modifying code stubs that has read, write and execute permissions

### `INDIRECT_SYMBOL_LOCAL`

```rust
const INDIRECT_SYMBOL_LOCAL: u32 = 2_147_483_648u32;
```

### `INDIRECT_SYMBOL_ABS`

```rust
const INDIRECT_SYMBOL_ABS: u32 = 1_073_741_824u32;
```

### `PLATFORM_MACOS`

```rust
const PLATFORM_MACOS: u32 = 1u32;
```

### `PLATFORM_IOS`

```rust
const PLATFORM_IOS: u32 = 2u32;
```

### `PLATFORM_TVOS`

```rust
const PLATFORM_TVOS: u32 = 3u32;
```

### `PLATFORM_WATCHOS`

```rust
const PLATFORM_WATCHOS: u32 = 4u32;
```

### `PLATFORM_BRIDGEOS`

```rust
const PLATFORM_BRIDGEOS: u32 = 5u32;
```

### `PLATFORM_MACCATALYST`

```rust
const PLATFORM_MACCATALYST: u32 = 6u32;
```

### `PLATFORM_IOSSIMULATOR`

```rust
const PLATFORM_IOSSIMULATOR: u32 = 7u32;
```

### `PLATFORM_TVOSSIMULATOR`

```rust
const PLATFORM_TVOSSIMULATOR: u32 = 8u32;
```

### `PLATFORM_WATCHOSSIMULATOR`

```rust
const PLATFORM_WATCHOSSIMULATOR: u32 = 9u32;
```

### `PLATFORM_DRIVERKIT`

```rust
const PLATFORM_DRIVERKIT: u32 = 10u32;
```

### `PLATFORM_XROS`

```rust
const PLATFORM_XROS: u32 = 11u32;
```

### `PLATFORM_XROSSIMULATOR`

```rust
const PLATFORM_XROSSIMULATOR: u32 = 12u32;
```

### `TOOL_CLANG`

```rust
const TOOL_CLANG: u32 = 1u32;
```

### `TOOL_SWIFT`

```rust
const TOOL_SWIFT: u32 = 2u32;
```

### `TOOL_LD`

```rust
const TOOL_LD: u32 = 3u32;
```

### `REBASE_TYPE_POINTER`

```rust
const REBASE_TYPE_POINTER: u8 = 1u8;
```

### `REBASE_TYPE_TEXT_ABSOLUTE32`

```rust
const REBASE_TYPE_TEXT_ABSOLUTE32: u8 = 2u8;
```

### `REBASE_TYPE_TEXT_PCREL32`

```rust
const REBASE_TYPE_TEXT_PCREL32: u8 = 3u8;
```

### `REBASE_OPCODE_MASK`

```rust
const REBASE_OPCODE_MASK: u8 = 240u8;
```

### `REBASE_IMMEDIATE_MASK`

```rust
const REBASE_IMMEDIATE_MASK: u8 = 15u8;
```

### `REBASE_OPCODE_DONE`

```rust
const REBASE_OPCODE_DONE: u8 = 0u8;
```

### `REBASE_OPCODE_SET_TYPE_IMM`

```rust
const REBASE_OPCODE_SET_TYPE_IMM: u8 = 16u8;
```

### `REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`

```rust
const REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB: u8 = 32u8;
```

### `REBASE_OPCODE_ADD_ADDR_ULEB`

```rust
const REBASE_OPCODE_ADD_ADDR_ULEB: u8 = 48u8;
```

### `REBASE_OPCODE_ADD_ADDR_IMM_SCALED`

```rust
const REBASE_OPCODE_ADD_ADDR_IMM_SCALED: u8 = 64u8;
```

### `REBASE_OPCODE_DO_REBASE_IMM_TIMES`

```rust
const REBASE_OPCODE_DO_REBASE_IMM_TIMES: u8 = 80u8;
```

### `REBASE_OPCODE_DO_REBASE_ULEB_TIMES`

```rust
const REBASE_OPCODE_DO_REBASE_ULEB_TIMES: u8 = 96u8;
```

### `REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB`

```rust
const REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB: u8 = 112u8;
```

### `REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB`

```rust
const REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB: u8 = 128u8;
```

### `BIND_TYPE_POINTER`

```rust
const BIND_TYPE_POINTER: u8 = 1u8;
```

### `BIND_TYPE_TEXT_ABSOLUTE32`

```rust
const BIND_TYPE_TEXT_ABSOLUTE32: u8 = 2u8;
```

### `BIND_TYPE_TEXT_PCREL32`

```rust
const BIND_TYPE_TEXT_PCREL32: u8 = 3u8;
```

### `BIND_SPECIAL_DYLIB_SELF`

```rust
const BIND_SPECIAL_DYLIB_SELF: i8 = 0i8;
```

### `BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE`

```rust
const BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE: i8 = -1i8;
```

### `BIND_SPECIAL_DYLIB_FLAT_LOOKUP`

```rust
const BIND_SPECIAL_DYLIB_FLAT_LOOKUP: i8 = -2i8;
```

### `BIND_SPECIAL_DYLIB_WEAK_LOOKUP`

```rust
const BIND_SPECIAL_DYLIB_WEAK_LOOKUP: i8 = -3i8;
```

### `BIND_SYMBOL_FLAGS_WEAK_IMPORT`

```rust
const BIND_SYMBOL_FLAGS_WEAK_IMPORT: u8 = 1u8;
```

### `BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION`

```rust
const BIND_SYMBOL_FLAGS_NON_WEAK_DEFINITION: u8 = 8u8;
```

### `BIND_OPCODE_MASK`

```rust
const BIND_OPCODE_MASK: u8 = 240u8;
```

### `BIND_IMMEDIATE_MASK`

```rust
const BIND_IMMEDIATE_MASK: u8 = 15u8;
```

### `BIND_OPCODE_DONE`

```rust
const BIND_OPCODE_DONE: u8 = 0u8;
```

### `BIND_OPCODE_SET_DYLIB_ORDINAL_IMM`

```rust
const BIND_OPCODE_SET_DYLIB_ORDINAL_IMM: u8 = 16u8;
```

### `BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB`

```rust
const BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB: u8 = 32u8;
```

### `BIND_OPCODE_SET_DYLIB_SPECIAL_IMM`

```rust
const BIND_OPCODE_SET_DYLIB_SPECIAL_IMM: u8 = 48u8;
```

### `BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM`

```rust
const BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM: u8 = 64u8;
```

### `BIND_OPCODE_SET_TYPE_IMM`

```rust
const BIND_OPCODE_SET_TYPE_IMM: u8 = 80u8;
```

### `BIND_OPCODE_SET_ADDEND_SLEB`

```rust
const BIND_OPCODE_SET_ADDEND_SLEB: u8 = 96u8;
```

### `BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB`

```rust
const BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB: u8 = 112u8;
```

### `BIND_OPCODE_ADD_ADDR_ULEB`

```rust
const BIND_OPCODE_ADD_ADDR_ULEB: u8 = 128u8;
```

### `BIND_OPCODE_DO_BIND`

```rust
const BIND_OPCODE_DO_BIND: u8 = 144u8;
```

### `BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB`

```rust
const BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB: u8 = 160u8;
```

### `BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED`

```rust
const BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED: u8 = 176u8;
```

### `BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB`

```rust
const BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB: u8 = 192u8;
```

### `BIND_OPCODE_THREADED`

```rust
const BIND_OPCODE_THREADED: u8 = 208u8;
```

### `BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB`

```rust
const BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB: u8 = 0u8;
```

### `BIND_SUBOPCODE_THREADED_APPLY`

```rust
const BIND_SUBOPCODE_THREADED_APPLY: u8 = 1u8;
```

### `EXPORT_SYMBOL_FLAGS_KIND_MASK`

```rust
const EXPORT_SYMBOL_FLAGS_KIND_MASK: u32 = 3u32;
```

### `EXPORT_SYMBOL_FLAGS_KIND_REGULAR`

```rust
const EXPORT_SYMBOL_FLAGS_KIND_REGULAR: u32 = 0u32;
```

### `EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL`

```rust
const EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL: u32 = 1u32;
```

### `EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE`

```rust
const EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE: u32 = 2u32;
```

### `EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION`

```rust
const EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION: u32 = 4u32;
```

### `EXPORT_SYMBOL_FLAGS_REEXPORT`

```rust
const EXPORT_SYMBOL_FLAGS_REEXPORT: u32 = 8u32;
```

### `EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER`

```rust
const EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER: u32 = 16u32;
```

### `DICE_KIND_DATA`

```rust
const DICE_KIND_DATA: u32 = 1u32;
```

### `DICE_KIND_JUMP_TABLE8`

```rust
const DICE_KIND_JUMP_TABLE8: u32 = 2u32;
```

### `DICE_KIND_JUMP_TABLE16`

```rust
const DICE_KIND_JUMP_TABLE16: u32 = 3u32;
```

### `DICE_KIND_JUMP_TABLE32`

```rust
const DICE_KIND_JUMP_TABLE32: u32 = 4u32;
```

### `DICE_KIND_ABS_JUMP_TABLE32`

```rust
const DICE_KIND_ABS_JUMP_TABLE32: u32 = 5u32;
```

### `N_STAB`

```rust
const N_STAB: u8 = 224u8;
```

if any of these bits set, a symbolic debugging entry

### `N_PEXT`

```rust
const N_PEXT: u8 = 16u8;
```

private external symbol bit

### `N_TYPE`

```rust
const N_TYPE: u8 = 14u8;
```

mask for the type bits

### `N_EXT`

```rust
const N_EXT: u8 = 1u8;
```

external symbol bit, set for external symbols

### `N_UNDF`

```rust
const N_UNDF: u8 = 0u8;
```

undefined, n_sect == NO_SECT

### `N_ABS`

```rust
const N_ABS: u8 = 2u8;
```

absolute, n_sect == NO_SECT

### `N_SECT`

```rust
const N_SECT: u8 = 14u8;
```

defined in section number n_sect

### `N_PBUD`

```rust
const N_PBUD: u8 = 12u8;
```

prebound undefined (defined in a dylib)

### `N_INDR`

```rust
const N_INDR: u8 = 10u8;
```

indirect

### `NO_SECT`

```rust
const NO_SECT: u8 = 0u8;
```

symbol is not in any section

### `MAX_SECT`

```rust
const MAX_SECT: u8 = 255u8;
```

1 thru 255 inclusive

### `REFERENCE_TYPE`

```rust
const REFERENCE_TYPE: u16 = 7u16;
```

### `REFERENCE_FLAG_UNDEFINED_NON_LAZY`

```rust
const REFERENCE_FLAG_UNDEFINED_NON_LAZY: u16 = 0u16;
```

### `REFERENCE_FLAG_UNDEFINED_LAZY`

```rust
const REFERENCE_FLAG_UNDEFINED_LAZY: u16 = 1u16;
```

### `REFERENCE_FLAG_DEFINED`

```rust
const REFERENCE_FLAG_DEFINED: u16 = 2u16;
```

### `REFERENCE_FLAG_PRIVATE_DEFINED`

```rust
const REFERENCE_FLAG_PRIVATE_DEFINED: u16 = 3u16;
```

### `REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY`

```rust
const REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY: u16 = 4u16;
```

### `REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY`

```rust
const REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY: u16 = 5u16;
```

### `REFERENCED_DYNAMICALLY`

```rust
const REFERENCED_DYNAMICALLY: u16 = 16u16;
```

### `SELF_LIBRARY_ORDINAL`

```rust
const SELF_LIBRARY_ORDINAL: u8 = 0u8;
```

### `MAX_LIBRARY_ORDINAL`

```rust
const MAX_LIBRARY_ORDINAL: u8 = 253u8;
```

### `DYNAMIC_LOOKUP_ORDINAL`

```rust
const DYNAMIC_LOOKUP_ORDINAL: u8 = 254u8;
```

### `EXECUTABLE_ORDINAL`

```rust
const EXECUTABLE_ORDINAL: u8 = 255u8;
```

### `N_NO_DEAD_STRIP`

```rust
const N_NO_DEAD_STRIP: u16 = 32u16;
```

symbol is not to be dead stripped

### `N_DESC_DISCARDED`

```rust
const N_DESC_DISCARDED: u16 = 32u16;
```

symbol is discarded

### `N_WEAK_REF`

```rust
const N_WEAK_REF: u16 = 64u16;
```

symbol is weak referenced

### `N_WEAK_DEF`

```rust
const N_WEAK_DEF: u16 = 128u16;
```

coalesced symbol is a weak definition

### `N_REF_TO_WEAK`

```rust
const N_REF_TO_WEAK: u16 = 128u16;
```

reference to a weak symbol

### `N_ARM_THUMB_DEF`

```rust
const N_ARM_THUMB_DEF: u16 = 8u16;
```

symbol is a Thumb function (ARM)

### `N_SYMBOL_RESOLVER`

```rust
const N_SYMBOL_RESOLVER: u16 = 256u16;
```

### `N_ALT_ENTRY`

```rust
const N_ALT_ENTRY: u16 = 512u16;
```

### `N_GSYM`

```rust
const N_GSYM: u8 = 32u8;
```

global symbol: name,,NO_SECT,type,0

### `N_FNAME`

```rust
const N_FNAME: u8 = 34u8;
```

procedure name (f77 kludge): name,,NO_SECT,0,0

### `N_FUN`

```rust
const N_FUN: u8 = 36u8;
```

procedure: name,,n_sect,linenumber,address

### `N_STSYM`

```rust
const N_STSYM: u8 = 38u8;
```

static symbol: name,,n_sect,type,address

### `N_LCSYM`

```rust
const N_LCSYM: u8 = 40u8;
```

.lcomm symbol: name,,n_sect,type,address

### `N_BNSYM`

```rust
const N_BNSYM: u8 = 46u8;
```

begin nsect sym: 0,,n_sect,0,address

### `N_AST`

```rust
const N_AST: u8 = 50u8;
```

AST file path: name,,NO_SECT,0,0

### `N_OPT`

```rust
const N_OPT: u8 = 60u8;
```

emitted with gcc2_compiled and in gcc source

### `N_RSYM`

```rust
const N_RSYM: u8 = 64u8;
```

register sym: name,,NO_SECT,type,register

### `N_SLINE`

```rust
const N_SLINE: u8 = 68u8;
```

src line: 0,,n_sect,linenumber,address

### `N_ENSYM`

```rust
const N_ENSYM: u8 = 78u8;
```

end nsect sym: 0,,n_sect,0,address

### `N_SSYM`

```rust
const N_SSYM: u8 = 96u8;
```

structure elt: name,,NO_SECT,type,struct_offset

### `N_SO`

```rust
const N_SO: u8 = 100u8;
```

source file name: name,,n_sect,0,address

### `N_OSO`

```rust
const N_OSO: u8 = 102u8;
```

object file name: name,,0,0,st_mtime

### `N_LSYM`

```rust
const N_LSYM: u8 = 128u8;
```

local sym: name,,NO_SECT,type,offset

### `N_BINCL`

```rust
const N_BINCL: u8 = 130u8;
```

include file beginning: name,,NO_SECT,0,sum

### `N_SOL`

```rust
const N_SOL: u8 = 132u8;
```

#included file name: name,,n_sect,0,address

### `N_PARAMS`

```rust
const N_PARAMS: u8 = 134u8;
```

compiler parameters: name,,NO_SECT,0,0

### `N_VERSION`

```rust
const N_VERSION: u8 = 136u8;
```

compiler version: name,,NO_SECT,0,0

### `N_OLEVEL`

```rust
const N_OLEVEL: u8 = 138u8;
```

compiler -O level: name,,NO_SECT,0,0

### `N_PSYM`

```rust
const N_PSYM: u8 = 160u8;
```

parameter: name,,NO_SECT,type,offset

### `N_EINCL`

```rust
const N_EINCL: u8 = 162u8;
```

include file end: name,,NO_SECT,0,0

### `N_ENTRY`

```rust
const N_ENTRY: u8 = 164u8;
```

alternate entry: name,,n_sect,linenumber,address

### `N_LBRAC`

```rust
const N_LBRAC: u8 = 192u8;
```

left bracket: 0,,NO_SECT,nesting level,address

### `N_EXCL`

```rust
const N_EXCL: u8 = 194u8;
```

deleted include file: name,,NO_SECT,0,sum

### `N_RBRAC`

```rust
const N_RBRAC: u8 = 224u8;
```

right bracket: 0,,NO_SECT,nesting level,address

### `N_BCOMM`

```rust
const N_BCOMM: u8 = 226u8;
```

begin common: name,,NO_SECT,0,0

### `N_ECOMM`

```rust
const N_ECOMM: u8 = 228u8;
```

end common: name,,n_sect,0,0

### `N_ECOML`

```rust
const N_ECOML: u8 = 232u8;
```

end common (local name): 0,,n_sect,0,address

### `N_LENG`

```rust
const N_LENG: u8 = 254u8;
```

second stab entry with length information

### `N_PC`

```rust
const N_PC: u8 = 48u8;
```

global pascal symbol: name,,NO_SECT,subtype,line

### `R_ABS`

```rust
const R_ABS: u8 = 0u8;
```

absolute relocation type for Mach-O files

### `R_SCATTERED`

```rust
const R_SCATTERED: u32 = 2_147_483_648u32;
```

Bit set in `Relocation::r_word0` for scattered relocations.

### `GENERIC_RELOC_VANILLA`

```rust
const GENERIC_RELOC_VANILLA: u8 = 0u8;
```

generic relocation as described above

### `GENERIC_RELOC_PAIR`

```rust
const GENERIC_RELOC_PAIR: u8 = 1u8;
```

Only follows a GENERIC_RELOC_SECTDIFF

### `GENERIC_RELOC_SECTDIFF`

```rust
const GENERIC_RELOC_SECTDIFF: u8 = 2u8;
```

### `GENERIC_RELOC_PB_LA_PTR`

```rust
const GENERIC_RELOC_PB_LA_PTR: u8 = 3u8;
```

prebound lazy pointer

### `GENERIC_RELOC_LOCAL_SECTDIFF`

```rust
const GENERIC_RELOC_LOCAL_SECTDIFF: u8 = 4u8;
```

### `GENERIC_RELOC_TLV`

```rust
const GENERIC_RELOC_TLV: u8 = 5u8;
```

thread local variables

### `ARM_RELOC_VANILLA`

```rust
const ARM_RELOC_VANILLA: u8 = 0u8;
```

generic relocation as described above

### `ARM_RELOC_PAIR`

```rust
const ARM_RELOC_PAIR: u8 = 1u8;
```

the second relocation entry of a pair

### `ARM_RELOC_SECTDIFF`

```rust
const ARM_RELOC_SECTDIFF: u8 = 2u8;
```

a PAIR follows with subtract symbol value

### `ARM_RELOC_LOCAL_SECTDIFF`

```rust
const ARM_RELOC_LOCAL_SECTDIFF: u8 = 3u8;
```

like ARM_RELOC_SECTDIFF, but the symbol referenced was local.

### `ARM_RELOC_PB_LA_PTR`

```rust
const ARM_RELOC_PB_LA_PTR: u8 = 4u8;
```

prebound lazy pointer

### `ARM_RELOC_BR24`

```rust
const ARM_RELOC_BR24: u8 = 5u8;
```

24 bit branch displacement (to a word address)

### `ARM_THUMB_RELOC_BR22`

```rust
const ARM_THUMB_RELOC_BR22: u8 = 6u8;
```

22 bit branch displacement (to a half-word address)

### `ARM_THUMB_32BIT_BRANCH`

```rust
const ARM_THUMB_32BIT_BRANCH: u8 = 7u8;
```

obsolete - a thumb 32-bit branch instruction possibly needing page-spanning branch workaround

### `ARM_RELOC_HALF`

```rust
const ARM_RELOC_HALF: u8 = 8u8;
```

### `ARM_RELOC_HALF_SECTDIFF`

```rust
const ARM_RELOC_HALF_SECTDIFF: u8 = 9u8;
```

### `ARM64_RELOC_UNSIGNED`

```rust
const ARM64_RELOC_UNSIGNED: u8 = 0u8;
```

for pointers

### `ARM64_RELOC_SUBTRACTOR`

```rust
const ARM64_RELOC_SUBTRACTOR: u8 = 1u8;
```

must be followed by a ARM64_RELOC_UNSIGNED

### `ARM64_RELOC_BRANCH26`

```rust
const ARM64_RELOC_BRANCH26: u8 = 2u8;
```

a B/BL instruction with 26-bit displacement

### `ARM64_RELOC_PAGE21`

```rust
const ARM64_RELOC_PAGE21: u8 = 3u8;
```

pc-rel distance to page of target

### `ARM64_RELOC_PAGEOFF12`

```rust
const ARM64_RELOC_PAGEOFF12: u8 = 4u8;
```

offset within page, scaled by r_length

### `ARM64_RELOC_GOT_LOAD_PAGE21`

```rust
const ARM64_RELOC_GOT_LOAD_PAGE21: u8 = 5u8;
```

pc-rel distance to page of GOT slot

### `ARM64_RELOC_GOT_LOAD_PAGEOFF12`

```rust
const ARM64_RELOC_GOT_LOAD_PAGEOFF12: u8 = 6u8;
```

offset within page of GOT slot, scaled by r_length

### `ARM64_RELOC_POINTER_TO_GOT`

```rust
const ARM64_RELOC_POINTER_TO_GOT: u8 = 7u8;
```

for pointers to GOT slots

### `ARM64_RELOC_TLVP_LOAD_PAGE21`

```rust
const ARM64_RELOC_TLVP_LOAD_PAGE21: u8 = 8u8;
```

pc-rel distance to page of TLVP slot

### `ARM64_RELOC_TLVP_LOAD_PAGEOFF12`

```rust
const ARM64_RELOC_TLVP_LOAD_PAGEOFF12: u8 = 9u8;
```

offset within page of TLVP slot, scaled by r_length

### `ARM64_RELOC_ADDEND`

```rust
const ARM64_RELOC_ADDEND: u8 = 10u8;
```

must be followed by PAGE21 or PAGEOFF12

### `ARM64_RELOC_AUTHENTICATED_POINTER`

```rust
const ARM64_RELOC_AUTHENTICATED_POINTER: u8 = 11u8;
```

### `PPC_RELOC_VANILLA`

```rust
const PPC_RELOC_VANILLA: u8 = 0u8;
```

generic relocation as described above

### `PPC_RELOC_PAIR`

```rust
const PPC_RELOC_PAIR: u8 = 1u8;
```

the second relocation entry of a pair

### `PPC_RELOC_BR14`

```rust
const PPC_RELOC_BR14: u8 = 2u8;
```

14 bit branch displacement (to a word address)

### `PPC_RELOC_BR24`

```rust
const PPC_RELOC_BR24: u8 = 3u8;
```

24 bit branch displacement (to a word address)

### `PPC_RELOC_HI16`

```rust
const PPC_RELOC_HI16: u8 = 4u8;
```

a PAIR follows with the low half

### `PPC_RELOC_LO16`

```rust
const PPC_RELOC_LO16: u8 = 5u8;
```

a PAIR follows with the high half

### `PPC_RELOC_HA16`

```rust
const PPC_RELOC_HA16: u8 = 6u8;
```

Same as the RELOC_HI16 except the low 16 bits and the high 16 bits are added together
with the low 16 bits sign extended first.  This means if bit 15 of the low 16 bits is
set the high 16 bits stored in the instruction will be adjusted.

### `PPC_RELOC_LO14`

```rust
const PPC_RELOC_LO14: u8 = 7u8;
```

Same as the LO16 except that the low 2 bits are not stored in the instruction and are
always zero.  This is used in double word load/store instructions.

### `PPC_RELOC_SECTDIFF`

```rust
const PPC_RELOC_SECTDIFF: u8 = 8u8;
```

a PAIR follows with subtract symbol value

### `PPC_RELOC_PB_LA_PTR`

```rust
const PPC_RELOC_PB_LA_PTR: u8 = 9u8;
```

prebound lazy pointer

### `PPC_RELOC_HI16_SECTDIFF`

```rust
const PPC_RELOC_HI16_SECTDIFF: u8 = 10u8;
```

section difference forms of above.  a PAIR

### `PPC_RELOC_LO16_SECTDIFF`

```rust
const PPC_RELOC_LO16_SECTDIFF: u8 = 11u8;
```

follows these with subtract symbol value

### `PPC_RELOC_HA16_SECTDIFF`

```rust
const PPC_RELOC_HA16_SECTDIFF: u8 = 12u8;
```

### `PPC_RELOC_JBSR`

```rust
const PPC_RELOC_JBSR: u8 = 13u8;
```

### `PPC_RELOC_LO14_SECTDIFF`

```rust
const PPC_RELOC_LO14_SECTDIFF: u8 = 14u8;
```

### `PPC_RELOC_LOCAL_SECTDIFF`

```rust
const PPC_RELOC_LOCAL_SECTDIFF: u8 = 15u8;
```

like PPC_RELOC_SECTDIFF, but the symbol referenced was local.

### `X86_64_RELOC_UNSIGNED`

```rust
const X86_64_RELOC_UNSIGNED: u8 = 0u8;
```

for absolute addresses

### `X86_64_RELOC_SIGNED`

```rust
const X86_64_RELOC_SIGNED: u8 = 1u8;
```

for signed 32-bit displacement

### `X86_64_RELOC_BRANCH`

```rust
const X86_64_RELOC_BRANCH: u8 = 2u8;
```

a CALL/JMP instruction with 32-bit displacement

### `X86_64_RELOC_GOT_LOAD`

```rust
const X86_64_RELOC_GOT_LOAD: u8 = 3u8;
```

a MOVQ load of a GOT entry

### `X86_64_RELOC_GOT`

```rust
const X86_64_RELOC_GOT: u8 = 4u8;
```

other GOT references

### `X86_64_RELOC_SUBTRACTOR`

```rust
const X86_64_RELOC_SUBTRACTOR: u8 = 5u8;
```

must be followed by a X86_64_RELOC_UNSIGNED

### `X86_64_RELOC_SIGNED_1`

```rust
const X86_64_RELOC_SIGNED_1: u8 = 6u8;
```

for signed 32-bit displacement with a -1 addend

### `X86_64_RELOC_SIGNED_2`

```rust
const X86_64_RELOC_SIGNED_2: u8 = 7u8;
```

for signed 32-bit displacement with a -2 addend

### `X86_64_RELOC_SIGNED_4`

```rust
const X86_64_RELOC_SIGNED_4: u8 = 8u8;
```

for signed 32-bit displacement with a -4 addend

### `X86_64_RELOC_TLV`

```rust
const X86_64_RELOC_TLV: u8 = 9u8;
```

for thread local variables

