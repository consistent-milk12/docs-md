*[backtrace](../../index.md) / [symbolize](../index.md) / [gimli](index.md)*

---

# Module `gimli`

Support for symbolication using the `gimli` crate on crates.io

This is the default symbolication implementation for Rust.

## Contents

- [Modules](#modules)
  - [`mmap`](#mmap)
  - [`lru`](#lru)
  - [`stash`](#stash)
  - [`elf`](#elf)
  - [`libs_dl_iterate_phdr`](#libs-dl-iterate-phdr)
  - [`parse_running_mmaps`](#parse-running-mmaps)
- [Structs](#structs)
  - [`Mapping`](#mapping)
  - [`Context`](#context)
  - [`Cache`](#cache)
  - [`Library`](#library)
  - [`LibrarySegment`](#librarysegment)
- [Enums](#enums)
  - [`Either`](#either)
  - [`Symbol`](#symbol)
- [Functions](#functions)
  - [`mmap`](#mmap)
  - [`create_mapping`](#create-mapping)
  - [`clear_symbol_cache`](#clear-symbol-cache)
  - [`resolve`](#resolve)
- [Constants](#constants)
  - [`MAPPINGS_CACHE_SIZE`](#mappings-cache-size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`mmap`](#mmap) | mod |  |
| [`lru`](#lru) | mod |  |
| [`stash`](#stash) | mod |  |
| [`elf`](#elf) | mod |  |
| [`libs_dl_iterate_phdr`](#libs-dl-iterate-phdr) | mod |  |
| [`parse_running_mmaps`](#parse-running-mmaps) | mod |  |
| [`Mapping`](#mapping) | struct |  |
| [`Context`](#context) | struct |  |
| [`Cache`](#cache) | struct |  |
| [`Library`](#library) | struct |  |
| [`LibrarySegment`](#librarysegment) | struct |  |
| [`Either`](#either) | enum |  |
| [`Symbol`](#symbol) | enum |  |
| [`mmap`](#mmap) | fn |  |
| [`create_mapping`](#create-mapping) | fn |  |
| [`clear_symbol_cache`](#clear-symbol-cache) | fn |  |
| [`resolve`](#resolve) | fn |  |
| [`MAPPINGS_CACHE_SIZE`](#mappings-cache-size) | const |  |

## Modules

- [`mmap`](mmap/index.md)
- [`lru`](lru/index.md)
- [`stash`](stash/index.md)
- [`elf`](elf/index.md)
- [`libs_dl_iterate_phdr`](libs_dl_iterate_phdr/index.md)
- [`parse_running_mmaps`](parse_running_mmaps/index.md)

## Structs

### `Mapping`

```rust
struct Mapping {
    cx: Context<'static>,
    _map: self::mmap::Mmap,
    stash: self::stash::Stash,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:63-68`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L63-L68)*

#### Implementations

- <span id="supermapping-new"></span>`fn new(path: &Path) -> Option<Mapping>` — [`Mapping`](#mapping)

- <span id="supermapping-new-debug"></span>`fn new_debug(original_path: &Path, path: PathBuf, crc: Option<u32>) -> Option<Mapping>` — [`Mapping`](#mapping)

- <span id="supermapping-load-dwarf-package"></span>`fn load_dwarf_package<'data>(path: &Path, stash: &'data Stash) -> Option<Object<'data>>` — [`Stash`](stash/index.md#stash), [`Object`](elf/index.md#object)

### `Context<'a>`

```rust
struct Context<'a> {
    dwarf: addr2line::Context<self::gimli::read::EndianSlice<'a, self::gimli::NativeEndian>>,
    object: self::elf::Object<'a>,
    package: Option<gimli::DwarfPackage<self::gimli::read::EndianSlice<'a, self::gimli::NativeEndian>>>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:111-115`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L111-L115)*

#### Implementations

- <span id="context-new"></span>`fn new(stash: &'data Stash, object: Object<'data>, sup: Option<Object<'data>>, dwp: Option<Object<'data>>) -> Option<Context<'data>>` — [`Stash`](stash/index.md#stash), [`Object`](elf/index.md#object), [`Context`](#context)

- <span id="context-find-frames"></span>`fn find_frames(&self, stash: &'data Stash, probe: u64) -> gimli::Result<addr2line::FrameIter<'_, EndianSlice<'data, Endian>>>` — [`Stash`](stash/index.md#stash)

### `Cache`

```rust
struct Cache {
    libraries: Vec<Library>,
    mappings: lru::Lru<(usize, Mapping), MAPPINGS_CACHE_SIZE>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:259-273`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L259-L273)*

#### Fields

- **`libraries`**: `Vec<Library>`

  All known shared libraries that have been loaded.

- **`mappings`**: `lru::Lru<(usize, Mapping), MAPPINGS_CACHE_SIZE>`

  Mappings cache where we retain parsed dwarf information.
  
  This list has a fixed capacity for its entire lifetime which never
  increases. The `usize` element of each pair is an index into `libraries`
  above where `usize::max_value()` represents the current executable. The
  `Mapping` is corresponding parsed dwarf information.
  
  Note that this is basically an LRU cache and we'll be shifting things
  around in here as we symbolize addresses.

#### Implementations

- <span id="cache-new"></span>`fn new() -> Cache` — [`Cache`](#cache)

- <span id="cache-with-global"></span>`unsafe fn with_global(f: impl FnOnce(&mut Self))`

- <span id="cache-avma-to-svma"></span>`fn avma_to_svma(&self, addr: *const u8) -> Option<(usize, *const u8)>`

- <span id="cache-mapping-for-lib"></span>`fn mapping_for_lib<'a>(self: &'a mut Self, lib: usize) -> Option<(&'a mut Context<'a>, &'a Stash)>` — [`Context`](#context), [`Stash`](stash/index.md#stash)

#### Trait Implementations

##### `impl Default for Cache`

- <span id="cache-default"></span>`fn default() -> Cache` — [`Cache`](#cache)

### `Library`

```rust
struct Library {
    name: mystd::ffi::OsString,
    segments: Vec<LibrarySegment>,
    bias: usize,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:275-307`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L275-L307)*

#### Fields

- **`segments`**: `Vec<LibrarySegment>`

  Segments of this library loaded into memory, and where they're loaded.

- **`bias`**: `usize`

  The "bias" of this library, typically where it's loaded into memory.
  This value is added to each segment's stated address to get the actual
  virtual memory address that the segment is loaded into. Additionally
  this bias is subtracted from real virtual memory addresses to index into
  debuginfo and the symbol table.

### `LibrarySegment`

```rust
struct LibrarySegment {
    stated_virtual_memory_address: usize,
    len: usize,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:309-316`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L309-L316)*

#### Fields

- **`stated_virtual_memory_address`**: `usize`

  The stated address of this segment in the object file. This is not
  actually where the segment is loaded, but rather this address plus the
  containing library's `bias` is where to find it.

- **`len`**: `usize`

  The size of this segment in memory.

## Enums

### `Either<A, B>`

```rust
enum Either<A, B> {
    A(A),
    B(B),
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:70-74`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L70-L74)*

### `Symbol<'a>`

```rust
enum Symbol<'a> {
    Frame {
        addr: *mut libc::c_void,
        location: Option<addr2line::Location<'a>>,
        name: Option<&'a [u8]>,
    },
    Symtab {
        name: &'a [u8],
    },
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:503-514`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L503-L514)*

#### Variants

- **`Frame`**

  We were able to locate frame information for this symbol, and
  `addr2line`'s frame internally has all the nitty gritty details.

- **`Symtab`**

  Couldn't find debug information, but we found it in the symbol table of
  the elf executable.

#### Implementations

- <span id="symbol-name"></span>`fn name(&self) -> Option<SymbolName<'_>>` — [`SymbolName`](../index.md#symbolname)

- <span id="symbol-addr"></span>`fn addr(&self) -> Option<*mut c_void>`

- <span id="symbol-filename-raw"></span>`fn filename_raw(&self) -> Option<BytesOrWideString<'_>>` — [`BytesOrWideString`](../../types/index.md#bytesorwidestring)

- <span id="symbol-filename"></span>`fn filename(&self) -> Option<&Path>`

- <span id="symbol-lineno"></span>`fn lineno(&self) -> Option<u32>`

- <span id="symbol-colno"></span>`fn colno(&self) -> Option<u32>`

## Functions

### `mmap`

```rust
fn mmap(path: &mystd::path::Path) -> Option<self::mmap::Mmap>
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:192-196`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L192-L196)*

### `create_mapping`

```rust
fn create_mapping(lib: &Library) -> Option<Mapping>
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:318-328`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L318-L328)*

### `clear_symbol_cache`

```rust
unsafe fn clear_symbol_cache()
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:346-350`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L346-L350)*

### `resolve`

```rust
unsafe fn resolve(what: super::ResolveWhat<'_>, cb: &mut dyn FnMut(&super::Symbol))
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:441-501`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L441-L501)*

## Constants

### `MAPPINGS_CACHE_SIZE`
```rust
const MAPPINGS_CACHE_SIZE: usize = 4usize;
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:61`](../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli.rs#L61)*

