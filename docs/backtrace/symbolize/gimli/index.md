*[backtrace](../../index.md) / [symbolize](../index.md) / [gimli](index.md)*

---

# Module `gimli`

Support for symbolication using the `gimli` crate on crates.io

This is the default symbolication implementation for Rust.

## Modules

- [`mmap`](mmap/index.md) - 
- [`lru`](lru/index.md) - 
- [`stash`](stash/index.md) - 
- [`elf`](elf/index.md) - 
- [`libs_dl_iterate_phdr`](libs_dl_iterate_phdr/index.md) - 
- [`parse_running_mmaps`](parse_running_mmaps/index.md) - 

## Structs

### `Mapping`

```rust
struct Mapping {
    cx: Context<'static>,
    _map: self::mmap::Mmap,
    stash: self::stash::Stash,
}
```

#### Implementations

- `fn mk<F>(data: Mmap, mk: F) -> Option<Mapping>` — [`Mmap`](mmap/index.md), [`Mapping`](#mapping)

- `fn mk_or_other<F>(data: Mmap, mk: F) -> Option<Mapping>` — [`Mmap`](mmap/index.md), [`Mapping`](#mapping)

### `Context<'a>`

```rust
struct Context<'a> {
    dwarf: addr2line::Context<self::gimli::read::EndianSlice<'a, self::gimli::NativeEndian>>,
    object: self::elf::Object<'a>,
    package: Option<gimli::DwarfPackage<self::gimli::read::EndianSlice<'a, self::gimli::NativeEndian>>>,
}
```

#### Implementations

- `fn new(stash: &'data Stash, object: Object<'data>, sup: Option<Object<'data>>, dwp: Option<Object<'data>>) -> Option<Context<'data>>` — [`Stash`](stash/index.md), [`Object`](elf/index.md), [`Context`](#context)

- `fn find_frames(self: &Self, stash: &'data Stash, probe: u64) -> gimli::Result<addr2line::FrameIter<'_, EndianSlice<'data, Endian>>>` — [`Stash`](stash/index.md)

### `Cache`

```rust
struct Cache {
    libraries: Vec<Library>,
    mappings: lru::Lru<(usize, Mapping), MAPPINGS_CACHE_SIZE>,
}
```

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

- `fn new() -> Cache` — [`Cache`](#cache)

- `unsafe fn with_global(f: impl FnOnce(&mut Self))`

- `fn avma_to_svma(self: &Self, addr: *const u8) -> Option<(usize, *const u8)>`

- `fn mapping_for_lib<'a>(self: &'a mut Self, lib: usize) -> Option<(&'a mut Context<'a>, &'a Stash)>` — [`Context`](#context), [`Stash`](stash/index.md)

#### Trait Implementations

##### `impl Default for Cache`

- `fn default() -> Cache` — [`Cache`](#cache)

### `Library`

```rust
struct Library {
    name: mystd::ffi::OsString,
    segments: Vec<LibrarySegment>,
    bias: usize,
}
```

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

#### Variants

- **`Frame`**

  We were able to locate frame information for this symbol, and
  `addr2line`'s frame internally has all the nitty gritty details.

- **`Symtab`**

  Couldn't find debug information, but we found it in the symbol table of
  the elf executable.

#### Implementations

- `fn name(self: &Self) -> Option<SymbolName<'_>>` — [`SymbolName`](../../index.md)

- `fn addr(self: &Self) -> Option<*mut c_void>`

- `fn filename_raw(self: &Self) -> Option<BytesOrWideString<'_>>` — [`BytesOrWideString`](../../index.md)

- `fn filename(self: &Self) -> Option<&Path>`

- `fn lineno(self: &Self) -> Option<u32>`

- `fn colno(self: &Self) -> Option<u32>`

## Functions

### `mmap`

```rust
fn mmap(path: &mystd::path::Path) -> Option<self::mmap::Mmap>
```

### `create_mapping`

```rust
fn create_mapping(lib: &Library) -> Option<Mapping>
```

### `clear_symbol_cache`

```rust
unsafe fn clear_symbol_cache()
```

### `resolve`

```rust
unsafe fn resolve(what: super::ResolveWhat<'_>, cb: &mut dyn FnMut(&super::Symbol))
```

## Constants

### `MAPPINGS_CACHE_SIZE`

```rust
const MAPPINGS_CACHE_SIZE: usize = 4usize;
```

