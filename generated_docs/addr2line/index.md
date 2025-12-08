# Crate `addr2line`

`addr2line` provides a cross-platform library for retrieving per-address debug information
from files with DWARF debug information. Given an address, it can return the file name,
line number, and function name associated with that address, as well as the inline call
stack leading to that address.

At the lowest level, the library uses a [`Context`](#context) to cache parsed information so that
multiple lookups are efficient. To create a `Context`, you first need to open and parse the
file using an object file parser such as [`object`](https://github.com/gimli-rs/object),
create a [`gimli::Dwarf`](../gimli/read/index.md), and finally call `Context::from_dwarf`.

Location information is obtained with `Context::find_location` or
`Context::find_location_range`. Function information is obtained with
`Context::find_frames`, which returns a frame for each inline function. Each frame
contains both name and location.

The library also provides a `Loader` which internally memory maps the files,
uses the `object` crate to do the parsing, and creates a `Context`.
The `Context` is not exposed, but the `Loader` provides the same functionality
via `Loader::find_location`, `Loader::find_location_range`, and
`Loader::find_frames`. The `Loader` also provides `Loader::find_symbol`
to use the symbol table instead of DWARF debugging information.
The `Loader` will load Mach-O dSYM files and split DWARF files as needed.

The crate has a CLI wrapper around the library which provides some of
the functionality of the `addr2line` command line tool distributed with
[GNU binutils](https://sourceware.org/binutils/docs/binutils/addr2line.html).

## Modules

- [`maybe_small`](maybe_small/index.md) - 
- [`frame`](frame/index.md) - 
- [`function`](function/index.md) - 
- [`line`](line/index.md) - 
- [`lookup`](lookup/index.md) - 
- [`unit`](unit/index.md) - 

## Structs

### `Frame<'ctx, R: gimli::Reader>`

```rust
struct Frame<'ctx, R: gimli::Reader> {
    pub dw_die_offset: Option<gimli::UnitOffset<<R as >::Offset>>,
    pub function: Option<FunctionName<R>>,
    pub location: Option<Location<'ctx>>,
}
```

A function frame.

#### Fields

- **`dw_die_offset`**: `Option<gimli::UnitOffset<<R as >::Offset>>`

  The DWARF unit offset corresponding to the DIE of the function.

- **`function`**: `Option<FunctionName<R>>`

  The name of the function.

- **`location`**: `Option<Location<'ctx>>`

  The source location corresponding to this frame.

### `FrameIter<'ctx, R>`

```rust
struct FrameIter<'ctx, R>(FrameIterState<'ctx, R>)
where
    R: gimli::Reader;
```

An iterator over function frames.

#### Implementations

- `fn new_empty() -> Self`

- `fn new_location(location: Location<'ctx>) -> Self` — [`Location`](#location)

- `fn new_frames(unit: &'ctx ResUnit<R>, sections: &'ctx gimli::Dwarf<R>, function: &'ctx Function<R>, inlined_functions: alloc::vec::Vec<&'ctx InlinedFunction<R>>, location: Option<Location<'ctx>>) -> Self` — [`ResUnit`](unit/index.md), [`Function`](function/index.md), [`InlinedFunction`](function/index.md), [`Location`](#location)

- `fn next(self: &mut Self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` — [`Frame`](#frame)

### `FunctionName<R: gimli::Reader>`

```rust
struct FunctionName<R: gimli::Reader> {
    pub name: R,
    pub language: Option<gimli::DwLang>,
}
```

A function name.

#### Fields

- **`name`**: `R`

  The name of the function.

- **`language`**: `Option<gimli::DwLang>`

  The language of the compilation unit containing this function.

#### Implementations

- `fn raw_name(self: &Self) -> Result<Cow<'_, str>, gimli::Error>`

- `fn demangle(self: &Self) -> Result<Cow<'_, str>, gimli::Error>`

### `Location<'a>`

```rust
struct Location<'a> {
    pub file: Option<&'a str>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}
```

A source location.

#### Fields

- **`file`**: `Option<&'a str>`

  The file name.

- **`line`**: `Option<u32>`

  The line number.

- **`column`**: `Option<u32>`

  The column number.
  
  A value of `Some(0)` indicates the left edge.

### `SplitDwarfLoad<R>`

```rust
struct SplitDwarfLoad<R> {
    pub dwo_id: gimli::DwoId,
    pub comp_dir: Option<R>,
    pub path: Option<R>,
    pub parent: alloc::sync::Arc<gimli::Dwarf<R>>,
}
```

This struct contains the information needed to find split DWARF data
and to produce a `gimli::Dwarf<R>` for it.

#### Fields

- **`dwo_id`**: `gimli::DwoId`

  The dwo id, for looking up in a DWARF package, or for
  verifying an unpacked dwo found on the file system

- **`comp_dir`**: `Option<R>`

  The compilation directory `path` is relative to.

- **`path`**: `Option<R>`

  A path on the filesystem, relative to `comp_dir` to find this dwo.

- **`parent`**: `alloc::sync::Arc<gimli::Dwarf<R>>`

  Once the split DWARF data is loaded, the loader is expected
  to call [make_dwo(parent)](gimli::read::Dwarf::make_dwo) before
  returning the data.

### `LocationRangeIter<'ctx, R: gimli::Reader>`

```rust
struct LocationRangeIter<'ctx, R: gimli::Reader> {
    unit_iter: alloc::boxed::Box<dyn Iterator<Item = (&'ctx ResUnit<R>, &'ctx gimli::Range)>>,
    iter: Option<crate::line::LineLocationRangeIter<'ctx>>,
    probe_low: u64,
    probe_high: u64,
    sections: &'ctx gimli::Dwarf<R>,
}
```

Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`.

#### Implementations

- `fn next_loc(self: &mut Self) -> Result<Option<(u64, u64, Location<'ctx>)>, gimli::Error>` — [`Location`](#location)

#### Trait Implementations

##### `impl<I> IntoIterator for LocationRangeIter<'ctx, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'ctx, R> Iterator for LocationRangeIter<'ctx, R>`

- `type Item = (u64, u64, Location<'ctx>)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Context<R: gimli::Reader>`

```rust
struct Context<R: gimli::Reader> {
    sections: alloc::sync::Arc<gimli::Dwarf<R>>,
    units: crate::unit::ResUnits<R>,
    sup_units: crate::unit::SupUnits<R>,
}
```

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

#### Implementations

- `fn find_unit(self: &Self, offset: gimli::DebugInfoOffset<<R as >::Offset>, file: DebugFile) -> Result<(&gimli::Unit<R>, gimli::UnitOffset<<R as >::Offset>), gimli::Error>` — [`DebugFile`](#debugfile)

### `RangeAttributes<R: gimli::Reader>`

```rust
struct RangeAttributes<R: gimli::Reader> {
    low_pc: Option<u64>,
    high_pc: Option<u64>,
    size: Option<u64>,
    ranges_offset: Option<gimli::RangeListsOffset<<R as gimli::Reader>::Offset>>,
}
```

#### Implementations

- `fn for_each_range<F: FnMut(gimli::Range)>(self: &Self, unit: gimli::UnitRef<'_, R>, f: F) -> Result<bool, gimli::Error>`

#### Trait Implementations

##### `impl<R: gimli::Reader> Default for RangeAttributes<R>`

- `fn default() -> Self`

## Enums

### `LookupResult<L: LookupContinuation>`

```rust
enum LookupResult<L: LookupContinuation> {
    Load {
        load: SplitDwarfLoad<<L as LookupContinuation>::Buf>,
        continuation: L,
    },
    Output(<L as LookupContinuation>::Output),
}
```

Operations that consult debug information may require additional files
to be loaded if split DWARF is being used. This enum returns the result
of the operation in the `Output` variant, or information about the split
DWARF that is required and a continuation to invoke once it is available
in the `Load` variant.

This enum is intended to be used in a loop like so:
```no_run
  use addr2line::*;
  use std::sync::Arc;
  let ctx: Context<gimli::EndianSlice<gimli::RunTimeEndian>> = todo!();
  let do_split_dwarf_load = |load: SplitDwarfLoad<gimli::EndianSlice<gimli::RunTimeEndian>>| -> Option<Arc<gimli::Dwarf<gimli::EndianSlice<gimli::RunTimeEndian>>>> { None };
  const ADDRESS: u64 = 0xdeadbeef;
  let mut r = ctx.find_frames(ADDRESS);
  let result = loop {
    match r {
      LookupResult::Output(result) => break result,
      LookupResult::Load { load, continuation } => {
        let dwo = do_split_dwarf_load(load);
        r = continuation.resume(dwo);
      }
    }
  };
```

#### Variants

- **`Load`**

  The lookup requires split DWARF data to be loaded.

- **`Output`**

  The lookup has completed and produced an output.

#### Implementations

- `fn skip_all_loads(self: Self) -> <L as >::Output` — [`LookupContinuation`](#lookupcontinuation)

- `fn map<T, F: FnOnce(<L as >::Output) -> T>(self: Self, f: F) -> LookupResult<MappedLookup<T, L, F>>` — [`LookupResult`](#lookupresult), [`MappedLookup`](lookup/index.md)

- `fn unwrap(self: Self) -> <L as >::Output` — [`LookupContinuation`](#lookupcontinuation)

### `DebugFile`

```rust
enum DebugFile {
    Primary,
    Supplementary,
    Dwo,
}
```

#### Trait Implementations

##### `impl Clone for DebugFile`

- `fn clone(self: &Self) -> DebugFile` — [`DebugFile`](#debugfile)

##### `impl Copy for DebugFile`

##### `impl Debug for DebugFile`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for DebugFile`

##### `impl PartialEq for DebugFile`

- `fn eq(self: &Self, other: &DebugFile) -> bool` — [`DebugFile`](#debugfile)

##### `impl StructuralPartialEq for DebugFile`

## Traits

## Functions

## Type Aliases

### `Error`

```rust
type Error = gimli::Error;
```

### `LazyResult<T>`

```rust
type LazyResult<T> = core::cell::OnceCell<Result<T, gimli::Error>>;
```

