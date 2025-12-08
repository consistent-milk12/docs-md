*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [elf](index.md)*

---

# Module `elf`

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

- `fn parse(data: &'a [u8]) -> Option<Object<'a>>` — [`Object`](#object)

- `fn section(self: &Self, stash: &'a Stash, name: &str) -> Option<&'a [u8]>` — [`Stash`](../stash/index.md)

- `fn section_header(self: &Self, name: &str) -> Option<&<object::elf::FileHeader64<object::NativeEndian> as FileHeader>::SectionHeader>`

- `fn search_symtab(self: &Self, addr: u64) -> Option<&[u8]>`

- `fn search_object_map(self: &Self, _addr: u64) -> Option<(&Context<'_>, u64)>` — [`Context`](../index.md)

- `fn build_id(self: &Self) -> Option<&'a [u8]>`

- `fn gnu_debuglink_path(self: &Self, path: &Path) -> Option<(PathBuf, u32)>`

- `fn gnu_debugaltlink_path(self: &Self, path: &Path) -> Option<(PathBuf, &'a [u8])>`

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

