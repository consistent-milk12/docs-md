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

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:63-68`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L63-L68)*

#### Implementations

- <span id="supermapping-new"></span>`fn new(path: &Path) -> Option<Mapping>` — [`Mapping`](#mapping)

- <span id="supermapping-new-debug"></span>`fn new_debug(original_path: &Path, path: PathBuf, crc: Option<u32>) -> Option<Mapping>` — [`Mapping`](#mapping)

  Load debuginfo from an external debug file.

- <span id="supermapping-load-dwarf-package"></span>`fn load_dwarf_package<'data>(path: &Path, stash: &'data Stash) -> Option<Object<'data>>` — [`Stash`](stash/index.md#stash), [`Object`](elf/index.md#object)

  Try to locate a DWARF package file.

#### Trait Implementations

##### `impl Any for Mapping`

- <span id="mapping-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mapping`

- <span id="mapping-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mapping`

- <span id="mapping-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Mapping`

- <span id="mapping-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Mapping`

- <span id="mapping-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Mapping`

- <span id="mapping-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapping-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Mapping`

- <span id="mapping-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapping-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Context<'a>`

```rust
struct Context<'a> {
    dwarf: addr2line::Context<self::gimli::read::EndianSlice<'a, self::gimli::NativeEndian>>,
    object: self::elf::Object<'a>,
    package: Option<gimli::DwarfPackage<self::gimli::read::EndianSlice<'a, self::gimli::NativeEndian>>>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:111-115`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L111-L115)*

#### Implementations

- <span id="context-new"></span>`fn new(stash: &'data Stash, object: Object<'data>, sup: Option<Object<'data>>, dwp: Option<Object<'data>>) -> Option<Context<'data>>` — [`Stash`](stash/index.md#stash), [`Object`](elf/index.md#object), [`Context`](#context)

- <span id="context-find-frames"></span>`fn find_frames(&self, stash: &'data Stash, probe: u64) -> gimli::Result<addr2line::FrameIter<'_, EndianSlice<'data, Endian>>>` — [`Stash`](stash/index.md#stash)

#### Trait Implementations

##### `impl Any for Context<'a>`

- <span id="context-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Context<'a>`

- <span id="context-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Context<'a>`

- <span id="context-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Context<'a>`

- <span id="context-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Context<'a>`

- <span id="context-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Context<'a>`

- <span id="context-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="context-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Context<'a>`

- <span id="context-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="context-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cache`

```rust
struct Cache {
    libraries: Vec<Library>,
    mappings: lru::Lru<(usize, Mapping), MAPPINGS_CACHE_SIZE>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:259-273`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L259-L273)*

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

##### `impl Any for Cache`

- <span id="cache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cache`

- <span id="cache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cache`

- <span id="cache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for Cache`

- <span id="cache-default"></span>`fn default() -> Cache` — [`Cache`](#cache)

##### `impl<T> From for Cache`

- <span id="cache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cache`

- <span id="cache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Cache`

- <span id="cache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cache`

- <span id="cache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Library`

```rust
struct Library {
    name: mystd::ffi::OsString,
    segments: Vec<LibrarySegment>,
    bias: usize,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:275-307`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L275-L307)*

#### Fields

- **`segments`**: `Vec<LibrarySegment>`

  Segments of this library loaded into memory, and where they're loaded.

- **`bias`**: `usize`

  The "bias" of this library, typically where it's loaded into memory.
  This value is added to each segment's stated address to get the actual
  virtual memory address that the segment is loaded into. Additionally
  this bias is subtracted from real virtual memory addresses to index into
  debuginfo and the symbol table.

#### Trait Implementations

##### `impl Any for Library`

- <span id="library-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Library`

- <span id="library-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Library`

- <span id="library-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Library`

- <span id="library-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Library`

- <span id="library-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Library`

- <span id="library-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="library-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Library`

- <span id="library-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="library-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LibrarySegment`

```rust
struct LibrarySegment {
    stated_virtual_memory_address: usize,
    len: usize,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:309-316`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L309-L316)*

#### Fields

- **`stated_virtual_memory_address`**: `usize`

  The stated address of this segment in the object file. This is not
  actually where the segment is loaded, but rather this address plus the
  containing library's `bias` is where to find it.

- **`len`**: `usize`

  The size of this segment in memory.

#### Trait Implementations

##### `impl Any for LibrarySegment`

- <span id="librarysegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LibrarySegment`

- <span id="librarysegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LibrarySegment`

- <span id="librarysegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LibrarySegment`

- <span id="librarysegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LibrarySegment`

- <span id="librarysegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LibrarySegment`

- <span id="librarysegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="librarysegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LibrarySegment`

- <span id="librarysegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="librarysegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Either<A, B>`

```rust
enum Either<A, B> {
    A(A),
    B(B),
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:70-74`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L70-L74)*

#### Trait Implementations

##### `impl Any for Either<A, B>`

- <span id="either-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Either<A, B>`

- <span id="either-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Either<A, B>`

- <span id="either-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Either<A, B>`

- <span id="either-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Either<A, B>`

- <span id="either-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Either<A, B>`

- <span id="either-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="either-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Either<A, B>`

- <span id="either-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="either-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:503-514`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L503-L514)*

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

#### Trait Implementations

##### `impl Any for Symbol<'a>`

- <span id="symbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Symbol<'a>`

- <span id="symbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Symbol<'a>`

- <span id="symbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Symbol<'a>`

- <span id="symbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Symbol<'a>`

- <span id="symbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Symbol<'a>`

- <span id="symbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Symbol<'a>`

- <span id="symbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `mmap`

```rust
fn mmap(path: &mystd::path::Path) -> Option<self::mmap::Mmap>
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:192-196`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L192-L196)*

### `create_mapping`

```rust
fn create_mapping(lib: &Library) -> Option<Mapping>
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:318-328`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L318-L328)*

### `clear_symbol_cache`

```rust
unsafe fn clear_symbol_cache()
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:346-350`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L346-L350)*

### `resolve`

```rust
unsafe fn resolve(what: super::ResolveWhat<'_>, cb: &mut dyn FnMut(&super::Symbol))
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:441-501`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L441-L501)*

## Constants

### `MAPPINGS_CACHE_SIZE`
```rust
const MAPPINGS_CACHE_SIZE: usize = 4usize;
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli.rs:61`](../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli.rs#L61)*

