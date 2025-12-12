*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [parse_running_mmaps](index.md)*

---

# Module `parse_running_mmaps`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MapsEntry`](#mapsentry) | struct |  |
| [`parse_maps`](#parse-maps) | fn |  |

## Structs

### `MapsEntry`

```rust
struct MapsEntry {
    address: (usize, usize),
    perms: [char; 4],
    offset: u64,
    dev: (usize, usize),
    inode: usize,
    pathname: super::mystd::ffi::OsString,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/parse_running_mmaps_unix.rs:13-56`](../../../../../.source_1765210505/backtrace-0.3.76/src/symbolize/gimli/parse_running_mmaps_unix.rs#L13-L56)*

#### Fields

- **`address`**: `(usize, usize)`

  start (inclusive) and limit (exclusive) of address range.

- **`perms`**: `[char; 4]`

  The perms field are the permissions for the entry
  
  r = read
  w = write
  x = execute
  s = shared
  p = private (copy on write)

- **`offset`**: `u64`

  Offset into the file (or "whatever").

- **`dev`**: `(usize, usize)`

  device (major, minor)

- **`inode`**: `usize`

  inode on the device. 0 indicates that no inode is associated with the memory region (e.g. uninitalized data aka BSS).

- **`pathname`**: `super::mystd::ffi::OsString`

  Usually the file backing the mapping.
  
  Note: The man page for proc includes a note about "coordination" by
  using readelf to see the Offset field in ELF program headers. pnkfelix
  is not yet sure if that is intended to be a comment on pathname, or what
  form/purpose such coordination is meant to have.
  
  There are also some pseudo-paths:
  "[stack]": The initial process's (aka main thread's) stack.
  "[stack:<tid>]": a specific thread's stack. (This was only present for a limited range of Linux verisons; it was determined to be too expensive to provide.)
  "[vdso]": Virtual dynamically linked shared object
  "[`heap`](../../../../compact_str/repr/heap/index.md)": The process's heap
  
  The pathname can be blank, which means it is an anonymous mapping
  obtained via mmap.
  
  Newlines in pathname are replaced with an octal escape sequence.
  
  The pathname may have "(deleted)" appended onto it if the file-backed
  path has been deleted.
  
  Note that modifications like the latter two indicated above imply that
  in general the pathname may be ambiguous. (I.e. you cannot tell if the
  denoted filename actually ended with the text "(deleted)", or if that
  was added by the maps rendering.

#### Implementations

- <span id="mapsentry-pathname"></span>`fn pathname(&self) -> &OsString`

- <span id="mapsentry-ip-matches"></span>`fn ip_matches(&self, ip: usize) -> bool`

#### Trait Implementations

##### `impl Debug for MapsEntry`

- <span id="mapsentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MapsEntry`

##### `impl FromStr for MapsEntry`

- <span id="mapsentry-fromstr-type-err"></span>`type Err = &'static str`

- <span id="mapsentry-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl PartialEq for MapsEntry`

- <span id="mapsentry-eq"></span>`fn eq(&self, other: &MapsEntry) -> bool` — [`MapsEntry`](#mapsentry)

##### `impl StructuralPartialEq for MapsEntry`

## Functions

### `parse_maps`

```rust
fn parse_maps() -> Result<alloc::vec::Vec<MapsEntry>, &'static str>
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/parse_running_mmaps_unix.rs:58-71`](../../../../../.source_1765210505/backtrace-0.3.76/src/symbolize/gimli/parse_running_mmaps_unix.rs#L58-L71)*

