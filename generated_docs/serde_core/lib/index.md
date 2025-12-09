*[serde_core](../index.md) / [lib](index.md)*

---

# Module `lib`

A facade around all the types we need from the `std`, `core`, and `alloc`
crates. This avoids elaborate import wrangling having to happen in every
module.

## Contents

- [Modules](#modules)
  - [`core`](#core)
- [Structs](#structs)
  - [`BinaryHeap`](#binaryheap)
  - [`VecDeque`](#vecdeque)
  - [`CStr`](#cstr)
  - [`Write`](#write)
- [Enums](#enums)
  - [`BTreeSet`](#btreeset)
- [Functions](#functions)
  - [`Cell`](#cell)
  - [`RefCell`](#refcell)
  - [`Reverse`](#reverse)
  - [`Debug`](#debug)
  - [`Debug`](#debug)
  - [`FmtWrite`](#fmtwrite)
  - [`Bound`](#bound)
  - [`RangeTo`](#rangeto)
  - [`LinkedList`](#linkedlist)
  - [`CString`](#cstring)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`core`](#core) | mod |  |
| [`BinaryHeap`](#binaryheap) | struct |  |
| [`VecDeque`](#vecdeque) | struct |  |
| [`CStr`](#cstr) | struct |  |
| [`Write`](#write) | struct |  |
| [`BTreeSet`](#btreeset) | enum |  |
| [`Cell`](#cell) | fn |  |
| [`RefCell`](#refcell) | fn |  |
| [`Reverse`](#reverse) | fn |  |
| [`Debug`](#debug) | fn |  |
| [`Debug`](#debug) | fn |  |
| [`FmtWrite`](#fmtwrite) | fn |  |
| [`Bound`](#bound) | fn |  |
| [`RangeTo`](#rangeto) | fn |  |
| [`LinkedList`](#linkedlist) | fn |  |
| [`CString`](#cstring) | fn |  |

## Modules

- [`core`](core/index.md)

## Structs

### `BinaryHeap<'ctx, R>`

```rust
struct BinaryHeap<'ctx, R>(FrameIterState<'ctx, R>)
where
    R: gimli::Reader;
```

*Re-exported from `addr2line`*

An iterator over function frames.

#### Implementations

- <span id="frameiter-new-empty"></span>`fn new_empty() -> Self`

- <span id="frameiter-new-location"></span>`fn new_location(location: Location<'ctx>) -> Self` — [`f64`](#f64)

- <span id="frameiter-new-frames"></span>`fn new_frames(unit: &'ctx ResUnit<R>, sections: &'ctx gimli::Dwarf<R>, function: &'ctx Function<R>, inlined_functions: alloc::vec::Vec<&'ctx InlinedFunction<R>>, location: Option<Location<'ctx>>) -> Self` — [`VecDeque`](#vecdeque), [`CStr`](#cstr), [`CStr`](#cstr), [`crate_root`](../crate_root/index.md), [`CString`](#cstring), [`lib`](#lib), [`f64`](#f64)

- <span id="frameiter-next"></span>`fn next(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` — [`PhantomData`](#phantomdata), [`lib`](#lib), [`Cow`](#cow), [`net`](#net)

### `VecDeque<R: gimli::Reader>`

```rust
struct VecDeque<R: gimli::Reader> {
    offset: gimli::DebugInfoOffset<<R as >::Offset>,
    dw_unit: gimli::Unit<R>,
    lang: Option<gimli::DwLang>,
    lines: crate::line::LazyLines,
    functions: crate::function::LazyFunctions<R>,
    dwo: core::cell::OnceCell<Result<Option<alloc::boxed::Box<DwoUnit<R>>>, gimli::Error>>,
}
```

*Re-exported from `addr2line`*

#### Implementations

- <span id="resunit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, sections: &'a gimli::Dwarf<R>) -> gimli::UnitRef<'a, R>` — [`CStr`](#cstr)

- <span id="resunit-dwarf-and-unit"></span>`fn dwarf_and_unit<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<SimpleLookup<Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>, R, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>>>` — [`PhantomData`](#phantomdata), [`Visitor`](../de/index.md), [`net`](#net), [`lib`](#lib), [`I8Deserializer`](../de/value/index.md), [`CStr`](#cstr)

- <span id="resunit-parse-lines"></span>`fn parse_lines(&self, sections: &gimli::Dwarf<R>) -> Result<Option<&Lines>, gimli::Error>` — [`CStr`](#cstr), [`PhantomData`](#phantomdata), [`lib`](#lib), [`net`](#net)

- <span id="resunit-parse-functions"></span>`fn parse_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<&'unit Functions<R>, gimli::Error>, Buf = R>>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="resunit-parse-inlined-functions"></span>`fn parse_inlined_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(), gimli::Error>, Buf = R> + 'unit>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="resunit-find-location"></span>`fn find_location(&self, probe: u64, sections: &gimli::Dwarf<R>) -> Result<Option<Location<'_>>, gimli::Error>` — [`CStr`](#cstr), [`PhantomData`](#phantomdata), [`lib`](#lib), [`f64`](#f64), [`net`](#net)

- <span id="resunit-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64, sections: &gimli::Dwarf<R>) -> Result<Option<LineLocationRangeIter<'_>>, gimli::Error>` — [`CStr`](#cstr), [`PhantomData`](#phantomdata), [`lib`](#lib), [`net`](#net)

- <span id="resunit-find-function-or-location"></span>`fn find_function_or_location<'unit, 'ctx: 'unit>(self: &'unit Self, probe: u64, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(Option<&'unit Function<R>>, Option<Location<'unit>>), gimli::Error>, Buf = R>>` — [`PhantomData`](#phantomdata), [`lib`](#lib), [`CStr`](#cstr), [`f64`](#f64), [`net`](#net)

### `CStr<R: gimli::Reader>`

```rust
struct CStr<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    name: Option<R>,
    inlined_functions: alloc::boxed::Box<[InlinedFunction<R>]>,
    inlined_addresses: alloc::boxed::Box<[InlinedFunctionAddress]>,
}
```

*Re-exported from `addr2line`*

#### Fields

- **`inlined_functions`**: `alloc::boxed::Box<[InlinedFunction<R>]>`

  List of all `DW_TAG_inlined_subroutine` details in this function.

- **`inlined_addresses`**: `alloc::boxed::Box<[InlinedFunctionAddress]>`

  List of `DW_TAG_inlined_subroutine` address ranges in this function.

#### Implementations

- <span id="function-parse"></span>`fn parse(dw_die_offset: gimli::UnitOffset<<R as >::Offset>, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<Self, gimli::Error>` — [`RangeTo`](#rangeto), [`result`](#result), [`Visitor`](../de/index.md), [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="function-parse-children"></span>`fn parse_children(state: &mut InlinedState<'_, R>, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="function-skip"></span>`fn skip(entries: &mut gimli::EntriesRaw<'_, '_, R>, abbrev: &gimli::Abbreviation, depth: isize) -> Result<(), gimli::Error>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="function-find-inlined-functions"></span>`fn find_inlined_functions(&self, probe: u64) -> alloc::vec::Vec<&InlinedFunction<R>>` — [`crate_root`](../crate_root/index.md), [`CString`](#cstring)

### `Write<'ctx, R>`

```rust
struct Write<'ctx, R>
where
    R: gimli::Reader {
    unit: &'ctx crate::unit::ResUnit<R>,
    sections: &'ctx gimli::Dwarf<R>,
    function: &'ctx crate::function::Function<R>,
    inlined_functions: iter::Rev<alloc::vec::IntoIter<&'ctx crate::function::InlinedFunction<R>>>,
    next: Option<Location<'ctx>>,
}
```

*Re-exported from `addr2line`*

## Enums

### `BTreeSet<'ctx, R>`

```rust
enum BTreeSet<'ctx, R>
where
    R: gimli::Reader {
    Empty,
    Location(Option<Location<'ctx>>),
    Frames(FrameIterFrames<'ctx, R>),
}
```

*Re-exported from `addr2line`*

## Functions

