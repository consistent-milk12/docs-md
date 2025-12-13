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

*Defined in [`addr2line-0.25.1/src/frame.rs:30-32`](../../../.source_1765633015/addr2line-0.25.1/src/frame.rs#L30-L32)*

*Re-exported from `addr2line`*

An iterator over function frames.

#### Implementations

- <span id="frameiter-new-empty"></span>`fn new_empty() -> Self`

- <span id="frameiter-new-location"></span>`fn new_location(location: Location<'ctx>) -> Self` — [`f64`](#f64)

- <span id="frameiter-new-frames"></span>`fn new_frames(unit: &'ctx ResUnit<R>, sections: &'ctx gimli::Dwarf<R>, function: &'ctx Function<R>, inlined_functions: alloc::vec::Vec<&'ctx InlinedFunction<R>>, location: Option<Location<'ctx>>) -> Self` — [`VecDeque`](#vecdeque), [`CStr`](#cstr), [`CStr`](#cstr), [`crate_root`](../crate_root/index.md#crate-root), [`CString`](#cstring), [`lib`](#lib), [`f64`](#f64)

- <span id="frameiter-next"></span>`fn next(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` — [`PhantomData`](#phantomdata), [`lib`](#lib), [`Cow`](#cow), [`net`](#net)

  Advances the iterator and returns the next frame.

#### Trait Implementations

##### `impl Any for FrameIter<'ctx, R>`

- <span id="frameiter-any-type-id"></span>`fn type_id(&self) -> TypeId` — [`RangeFrom`](#rangefrom)

##### `impl<T> Borrow for FrameIter<'ctx, R>`

- <span id="frameiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FrameIter<'ctx, R>`

- <span id="frameiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FrameIter<'ctx, R>`

- <span id="frameiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FrameIter<'ctx, R>`

- <span id="frameiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FrameIter<'ctx, R>`

- <span id="frameiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frameiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

##### `impl<U> TryInto for FrameIter<'ctx, R>`

- <span id="frameiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frameiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

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

*Defined in [`addr2line-0.25.1/src/unit.rs:18-25`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L18-L25)*

*Re-exported from `addr2line`*

#### Implementations

- <span id="resunit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, sections: &'a gimli::Dwarf<R>) -> gimli::UnitRef<'a, R>` — [`CStr`](#cstr)

- <span id="resunit-dwarf-and-unit"></span>`fn dwarf_and_unit<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<SimpleLookup<Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>, R, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>>>` — [`PhantomData`](#phantomdata), [`Visitor`](../de/index.md#visitor), [`net`](#net), [`lib`](#lib), [`I8Deserializer`](../de/value/index.md#i8deserializer), [`CStr`](#cstr)

  Returns the DWARF sections and the unit.

  

  Loads the DWO unit if necessary.

- <span id="resunit-parse-lines"></span>`fn parse_lines(&self, sections: &gimli::Dwarf<R>) -> Result<Option<&Lines>, gimli::Error>` — [`CStr`](#cstr), [`PhantomData`](#phantomdata), [`lib`](#lib), [`net`](#net)

- <span id="resunit-parse-functions"></span>`fn parse_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<&'unit Functions<R>, gimli::Error>, Buf = R>>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="resunit-parse-inlined-functions"></span>`fn parse_inlined_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(), gimli::Error>, Buf = R> + 'unit>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="resunit-find-location"></span>`fn find_location(&self, probe: u64, sections: &gimli::Dwarf<R>) -> Result<Option<Location<'_>>, gimli::Error>` — [`CStr`](#cstr), [`PhantomData`](#phantomdata), [`lib`](#lib), [`f64`](#f64), [`net`](#net)

- <span id="resunit-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64, sections: &gimli::Dwarf<R>) -> Result<Option<LineLocationRangeIter<'_>>, gimli::Error>` — [`CStr`](#cstr), [`PhantomData`](#phantomdata), [`lib`](#lib), [`net`](#net)

- <span id="resunit-find-function-or-location"></span>`fn find_function_or_location<'unit, 'ctx: 'unit>(self: &'unit Self, probe: u64, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(Option<&'unit Function<R>>, Option<Location<'unit>>), gimli::Error>, Buf = R>>` — [`PhantomData`](#phantomdata), [`lib`](#lib), [`CStr`](#cstr), [`f64`](#f64), [`net`](#net)

#### Trait Implementations

##### `impl Any for ResUnit<R>`

- <span id="resunit-any-type-id"></span>`fn type_id(&self) -> TypeId` — [`RangeFrom`](#rangefrom)

##### `impl<T> Borrow for ResUnit<R>`

- <span id="resunit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResUnit<R>`

- <span id="resunit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ResUnit<R>`

- <span id="resunit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResUnit<R>`

- <span id="resunit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ResUnit<R>`

- <span id="resunit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resunit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

##### `impl<U> TryInto for ResUnit<R>`

- <span id="resunit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resunit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

### `CStr<R: gimli::Reader>`

```rust
struct CStr<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    name: Option<R>,
    inlined_functions: alloc::boxed::Box<[InlinedFunction<R>]>,
    inlined_addresses: alloc::boxed::Box<[InlinedFunctionAddress]>,
}
```

*Defined in [`addr2line-0.25.1/src/function.rs:67-74`](../../../.source_1765633015/addr2line-0.25.1/src/function.rs#L67-L74)*

*Re-exported from `addr2line`*

#### Fields

- **`inlined_functions`**: `alloc::boxed::Box<[InlinedFunction<R>]>`

  List of all `DW_TAG_inlined_subroutine` details in this function.

- **`inlined_addresses`**: `alloc::boxed::Box<[InlinedFunctionAddress]>`

  List of `DW_TAG_inlined_subroutine` address ranges in this function.

#### Implementations

- <span id="function-parse"></span>`fn parse(dw_die_offset: gimli::UnitOffset<<R as >::Offset>, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<Self, gimli::Error>` — [`RangeTo`](#rangeto), [`result`](#result), [`Visitor`](../de/index.md#visitor), [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="function-parse-children"></span>`fn parse_children(state: &mut InlinedState<'_, R>, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="function-skip"></span>`fn skip(entries: &mut gimli::EntriesRaw<'_, '_, R>, abbrev: &gimli::Abbreviation, depth: isize) -> Result<(), gimli::Error>` — [`PhantomData`](#phantomdata), [`net`](#net)

- <span id="function-find-inlined-functions"></span>`fn find_inlined_functions(&self, probe: u64) -> alloc::vec::Vec<&InlinedFunction<R>>` — [`crate_root`](../crate_root/index.md#crate-root), [`CString`](#cstring)

  Build the list of inlined functions that contain `probe`.

#### Trait Implementations

##### `impl Any for Function<R>`

- <span id="function-any-type-id"></span>`fn type_id(&self) -> TypeId` — [`RangeFrom`](#rangefrom)

##### `impl<T> Borrow for Function<R>`

- <span id="function-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Function<R>`

- <span id="function-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Function<R>`

- <span id="function-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Function<R>`

- <span id="function-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Function<R>`

- <span id="function-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="function-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

##### `impl<U> TryInto for Function<R>`

- <span id="function-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="function-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

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

*Defined in [`addr2line-0.25.1/src/frame.rs:43-52`](../../../.source_1765633015/addr2line-0.25.1/src/frame.rs#L43-L52)*

*Re-exported from `addr2line`*

#### Trait Implementations

##### `impl Any for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-any-type-id"></span>`fn type_id(&self) -> TypeId` — [`RangeFrom`](#rangefrom)

##### `impl<T> Borrow for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frameiterframes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

##### `impl<U> TryInto for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frameiterframes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

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

*Defined in [`addr2line-0.25.1/src/frame.rs:34-41`](../../../.source_1765633015/addr2line-0.25.1/src/frame.rs#L34-L41)*

*Re-exported from `addr2line`*

#### Trait Implementations

##### `impl Any for FrameIterState<'ctx, R>`

- <span id="frameiterstate-any-type-id"></span>`fn type_id(&self) -> TypeId` — [`RangeFrom`](#rangefrom)

##### `impl<T> Borrow for FrameIterState<'ctx, R>`

- <span id="frameiterstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FrameIterState<'ctx, R>`

- <span id="frameiterstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FrameIterState<'ctx, R>`

- <span id="frameiterstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FrameIterState<'ctx, R>`

- <span id="frameiterstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FrameIterState<'ctx, R>`

- <span id="frameiterstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frameiterstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

##### `impl<U> TryInto for FrameIterState<'ctx, R>`

- <span id="frameiterstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frameiterstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`PhantomData`](#phantomdata), [`FmtWrite`](#fmtwrite)

## Functions

### `Cell`

```rust
fn Cell(&self) -> &T
```

### `RefCell`

```rust
unsafe fn RefCell(&self, dest: *mut u8)
```

### `Reverse`

```rust
fn Reverse(&mut self) -> &mut T
```

### `Debug`

```rust
fn Debug(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
<code>[From]&lt;T&gt; for U</code> chooses to do.

### `Debug`

```rust
fn Debug(t: T) -> T
```

Returns the argument unchanged.

### `FmtWrite`

```rust
fn FmtWrite(self) -> Result<U, <U as TryFrom>::Error>
```

### `Bound`

```rust
fn Bound(value: U) -> Result<T, <T as TryFrom>::Error>
```

### `RangeTo`

```rust
fn RangeTo(&self) -> U32X4
```

*Defined in [`adler2-2.0.1/src/algo.rs:111`](../../../.source_1765633015/adler2-2.0.1/src/algo.rs#L111)*

### `LinkedList`

```rust
fn LinkedList(location: Location<'ctx>) -> Self
```

*Defined in [`addr2line-0.25.1/src/frame.rs:62-64`](../../../.source_1765633015/addr2line-0.25.1/src/frame.rs#L62-L64)*

### `CString`

```rust
fn CString(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>
```

*Defined in [`addr2line-0.25.1/src/frame.rs:84-145`](../../../.source_1765633015/addr2line-0.25.1/src/frame.rs#L84-L145)*

Advances the iterator and returns the next frame.

