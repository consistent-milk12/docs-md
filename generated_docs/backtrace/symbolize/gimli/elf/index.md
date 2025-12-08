*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [elf](index.md)*

---

# Module `elf`

## Contents

- [Structs](#structs)
  - [`ParsedSym`](#parsedsym)
  - [`Object`](#object)
- [Functions](#functions)
  - [`decompress_zlib`](#decompress_zlib)
  - [`debug_path_exists`](#debug_path_exists)
  - [`locate_build_id`](#locate_build_id)
  - [`locate_debuglink`](#locate_debuglink)
  - [`locate_debugaltlink`](#locate_debugaltlink)
  - [`handle_split_dwarf`](#handle_split_dwarf)
- [Type Aliases](#type-aliases)
  - [`Elf`](#elf)
- [Constants](#constants)
  - [`DEBUG_PATH`](#debug_path)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParsedSym`](#parsedsym) | struct |  |
| [`Object`](#object) | struct |  |
| [`decompress_zlib`](#decompress_zlib) | fn |  |
| [`debug_path_exists`](#debug_path_exists) | fn |  |
| [`locate_build_id`](#locate_build_id) | fn | Locate a debug file based on its build ID. |
| [`locate_debuglink`](#locate_debuglink) | fn | Locate a file specified in a `.gnu_debuglink` section. |
| [`locate_debugaltlink`](#locate_debugaltlink) | fn | Locate a file specified in a `.gnu_debugaltlink` section. |
| [`handle_split_dwarf`](#handle_split_dwarf) | fn |  |
| [`Elf`](#elf) | type |  |
| [`DEBUG_PATH`](#debug_path) | const |  |

## Structs

### `ParsedSym`

```rust
struct ParsedSym {
    address: u64,
    size: u64,
    name: u32,
}
```

### `Object<'a>`

```rust
struct Object<'a> {
    endian: object::NativeEndian,
    data: &'a [u8],
    sections: object::read::elf::SectionTable<'a, object::elf::FileHeader64<object::NativeEndian>>,
    strings: object::read::StringTable<'a>,
    syms: alloc::vec::Vec<ParsedSym>,
}
```

#### Fields

- **`endian`**: `object::NativeEndian`

  Zero-sized type representing the native endianness.
  
  We could use a literal instead, but this helps ensure correctness.

- **`data`**: `&'a [u8]`

  The entire file data.

- **`syms`**: `alloc::vec::Vec<ParsedSym>`

  List of pre-parsed and sorted symbols by base address.

#### Implementations

- <span id="object-parse"></span>`fn parse(data: &'a [u8]) -> Option<Object<'a>>` — [`Object`](#object)

- <span id="object-section"></span>`fn section(&self, stash: &'a Stash, name: &str) -> Option<&'a [u8]>` — [`Stash`](../stash/index.md)

- <span id="object-section-header"></span>`fn section_header(&self, name: &str) -> Option<&<object::elf::FileHeader64<object::NativeEndian> as FileHeader>::SectionHeader>`

- <span id="object-search-symtab"></span>`fn search_symtab(&self, addr: u64) -> Option<&[u8]>`

- <span id="object-search-object-map"></span>`fn search_object_map(&self, _addr: u64) -> Option<(&Context<'_>, u64)>` — [`Context`](../index.md)

- <span id="object-build-id"></span>`fn build_id(&self) -> Option<&'a [u8]>`

- <span id="object-gnu-debuglink-path"></span>`fn gnu_debuglink_path(&self, path: &Path) -> Option<(PathBuf, u32)>`

- <span id="object-gnu-debugaltlink-path"></span>`fn gnu_debugaltlink_path(&self, path: &Path) -> Option<(PathBuf, &'a [u8])>`

## Functions

### `decompress_zlib`

```rust
fn decompress_zlib(input: &[u8], output: &mut [u8]) -> Option<()>
```

### `debug_path_exists`

```rust
fn debug_path_exists() -> bool
```

### `locate_build_id`

```rust
fn locate_build_id(build_id: &[u8]) -> Option<super::mystd::path::PathBuf>
```

Locate a debug file based on its build ID.

The format of build id paths is documented at:
https://sourceware.org/gdb/onlinedocs/gdb/Separate-Debug-Files.html

### `locate_debuglink`

```rust
fn locate_debuglink(path: &super::mystd::path::Path, filename: &super::mystd::ffi::OsStr) -> Option<super::mystd::path::PathBuf>
```

Locate a file specified in a `.gnu_debuglink` section.

`path` is the file containing the section.
`filename` is from the contents of the section.

Search order is based on gdb, documented at:
https://sourceware.org/gdb/onlinedocs/gdb/Separate-Debug-Files.html

gdb also allows the user to customize the debug search path, but we don't.

gdb also supports debuginfod, but we don't yet.

### `locate_debugaltlink`

```rust
fn locate_debugaltlink(path: &super::mystd::path::Path, filename: &super::mystd::ffi::OsStr, build_id: &[u8]) -> Option<super::mystd::path::PathBuf>
```

Locate a file specified in a `.gnu_debugaltlink` section.

`path` is the file containing the section.
`filename` and `build_id` are the contents of the section.

Search order is based on gdb:
- filename, which is either absolute or relative to `path`
- the build ID path under `BUILD_ID_PATH`

gdb also allows the user to customize the debug search path, but we don't.

gdb also supports debuginfod, but we don't yet.

### `handle_split_dwarf`

```rust
fn handle_split_dwarf<'data>(package: Option<&gimli::DwarfPackage<self::gimli::read::EndianSlice<'data, self::gimli::NativeEndian>>>, stash: &'data self::stash::Stash, load: addr2line::SplitDwarfLoad<self::gimli::read::EndianSlice<'data, self::gimli::NativeEndian>>) -> Option<alloc::sync::Arc<gimli::Dwarf<self::gimli::read::EndianSlice<'data, self::gimli::NativeEndian>>>>
```

## Type Aliases

### `Elf`

```rust
type Elf = object::elf::FileHeader64<object::NativeEndian>;
```

## Constants

### `DEBUG_PATH`

```rust
const DEBUG_PATH: &str;
```

