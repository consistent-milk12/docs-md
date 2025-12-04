# Crate `addr2line`

`addr2line` provides a cross-platform library for retrieving per-address debug information
from files with DWARF debug information. Given an address, it can return the file name,
line number, and function name associated with that address, as well as the inline call
stack leading to that address.

At the lowest level, the library uses a [`Context`](#context) to cache parsed information so that
multiple lookups are efficient. To create a `Context`, you first need to open and parse the
file using an object file parser such as [`object`](https://github.com/gimli-rs/object),
create a `gimli::Dwarf`, and finally call `Context::from_dwarf`.

Location information is obtained with `Context::find_location` or
`Context::find_location_range`. Function information is obtained with
`Context::find_frames`, which returns a frame for each inline function. Each frame
contains both name and location.

The library also provides a [`Loader`](#loader) which internally memory maps the files,
uses the `object` crate to do the parsing, and creates a `Context`.
The `Context` is not exposed, but the `Loader` provides the same functionality
via `Loader::find_location`, `Loader::find_location_range`, and
`Loader::find_frames`. The `Loader` also provides `Loader::find_symbol`
to use the symbol table instead of DWARF debugging information.
The `Loader` will load Mach-O dSYM files and split DWARF files as needed.

The crate has a CLI wrapper around the library which provides some of
the functionality of the `addr2line` command line tool distributed with
[GNU binutils](https://sourceware.org/binutils/docs/binutils/addr2line.html).

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

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `FrameIter<'ctx, R>`

```rust
struct FrameIter<'ctx, R>()
where
    R: gimli::Reader;
```

An iterator over function frames.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>`
  Advances the iterator and returns the next frame.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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
  The raw name of this function before demangling.

- `fn demangle(self: &Self) -> Result<Cow<'_, str>, gimli::Error>`
  The name of this function after demangling (if applicable).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `LocationRangeIter<'ctx, R: gimli::Reader>`

```rust
struct LocationRangeIter<'ctx, R: gimli::Reader> {
    // [REDACTED: Private Fields]
}
```

Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'ctx, R>`

- `type Item = (u64, u64, Location<'ctx>)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Context<R: gimli::Reader>`

```rust
struct Context<R: gimli::Reader> {
    // [REDACTED: Private Fields]
}
```

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

#### Implementations

- `fn find_dwarf_and_unit(self: &Self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Option<gimli::UnitRef<'_, R>>, Buf = R>>`
  Find the DWARF unit corresponding to the given virtual memory address.

- `fn find_location(self: &Self, probe: u64) -> Result<Option<Location<'_>>, gimli::Error>`
  Find the source file and line corresponding to the given virtual memory address.

- `fn find_location_range(self: &Self, probe_low: u64, probe_high: u64) -> Result<LocationRangeIter<'_, R>, gimli::Error>`
  Return source file and lines for a range of addresses. For each location it also

- `fn find_frames(self: &Self, probe: u64) -> LookupResult<impl LookupContinuation<Output = Result<FrameIter<'_, R>, gimli::Error>, Buf = R>>`
  Return an iterator for the function frames corresponding to the given virtual

- `fn preload_units(self: &Self, probe: u64) -> impl Iterator<Item = (SplitDwarfLoad<R>, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(), gimli::Error> + '_)>`
  Preload units for `probe`.

- `fn from_sections(debug_abbrev: gimli::DebugAbbrev<R>, debug_addr: gimli::DebugAddr<R>, debug_aranges: gimli::DebugAranges<R>, debug_info: gimli::DebugInfo<R>, debug_line: gimli::DebugLine<R>, debug_line_str: gimli::DebugLineStr<R>, debug_ranges: gimli::DebugRanges<R>, debug_rnglists: gimli::DebugRngLists<R>, debug_str: gimli::DebugStr<R>, debug_str_offsets: gimli::DebugStrOffsets<R>, default_section: R) -> Result<Self, gimli::Error>`
  Construct a new `Context` from DWARF sections.

- `fn from_dwarf(sections: gimli::Dwarf<R>) -> Result<Context<R>, gimli::Error>`
  Construct a new `Context` from an existing [`gimli::Dwarf`] object.

- `fn from_arc_dwarf(sections: Arc<gimli::Dwarf<R>>) -> Result<Context<R>, gimli::Error>`
  Construct a new `Context` from an existing [`gimli::Dwarf`] object.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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
  # use addr2line::*;
  # use std::sync::Arc;
  # let ctx: Context<gimli::EndianSlice<gimli::RunTimeEndian>> = todo!();
  # let do_split_dwarf_load = |load: SplitDwarfLoad<gimli::EndianSlice<gimli::RunTimeEndian>>| -> Option<Arc<gimli::Dwarf<gimli::EndianSlice<gimli::RunTimeEndian>>>> { None };
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

- `fn skip_all_loads(self: Self) -> <L as >::Output`
  Callers that do not handle split DWARF can call `skip_all_loads`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Traits

## Functions

