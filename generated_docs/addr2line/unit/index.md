*[addr2line](../index.md) / [unit](index.md)*

---

# Module `unit`

## Contents

- [Structs](#structs)
  - [`UnitRange`](#unitrange)
  - [`ResUnit`](#resunit)
  - [`ResUnits`](#resunits)
  - [`DwoUnit`](#dwounit)
  - [`SupUnit`](#supunit)
  - [`SupUnits`](#supunits)
  - [`LocationRangeIter`](#locationrangeiter)
- [Type Aliases](#type-aliases)
  - [`UnitRef`](#unitref)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UnitRange`](#unitrange) | struct |  |
| [`ResUnit`](#resunit) | struct |  |
| [`ResUnits`](#resunits) | struct |  |
| [`DwoUnit`](#dwounit) | struct | A DWO unit has its own DWARF sections. |
| [`SupUnit`](#supunit) | struct |  |
| [`SupUnits`](#supunits) | struct |  |
| [`LocationRangeIter`](#locationrangeiter) | struct | Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`. |
| [`UnitRef`](#unitref) | type |  |

## Structs

### `UnitRange`

```rust
struct UnitRange {
    unit_id: usize,
    min_begin: u64,
    range: gimli::Range,
}
```

*Defined in [`addr2line-0.25.1/src/unit.rs:12-16`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L12-L16)*

#### Trait Implementations

##### `impl Any for UnitRange`

- <span id="unitrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitRange`

- <span id="unitrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitRange`

- <span id="unitrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UnitRange`

- <span id="unitrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitRange`

- <span id="unitrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for UnitRange`

- <span id="unitrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitRange`

- <span id="unitrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResUnit<R: gimli::Reader>`

```rust
struct ResUnit<R: gimli::Reader> {
    offset: gimli::DebugInfoOffset<<R as >::Offset>,
    dw_unit: gimli::Unit<R>,
    lang: Option<gimli::DwLang>,
    lines: crate::line::LazyLines,
    functions: crate::function::LazyFunctions<R>,
    dwo: core::cell::OnceCell<Result<Option<alloc::boxed::Box<DwoUnit<R>>>, gimli::Error>>,
}
```

*Defined in [`addr2line-0.25.1/src/unit.rs:18-25`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L18-L25)*

#### Implementations

- <span id="resunit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, sections: &'a gimli::Dwarf<R>) -> gimli::UnitRef<'a, R>`

- <span id="resunit-dwarf-and-unit"></span>`fn dwarf_and_unit<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<SimpleLookup<Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>, R, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>>>` — [`Context`](../index.md#context), [`LookupResult`](../lookup/index.md#lookupresult), [`SimpleLookup`](../lookup/index.md#simplelookup), [`DebugFile`](../index.md#debugfile)

  Returns the DWARF sections and the unit.

  

  Loads the DWO unit if necessary.

- <span id="resunit-parse-lines"></span>`fn parse_lines(&self, sections: &gimli::Dwarf<R>) -> Result<Option<&Lines>, gimli::Error>` — [`Lines`](../line/index.md#lines)

- <span id="resunit-parse-functions"></span>`fn parse_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<&'unit Functions<R>, gimli::Error>, Buf = R>>` — [`Context`](../index.md#context), [`LookupResult`](../lookup/index.md#lookupresult), [`LookupContinuation`](../lookup/index.md#lookupcontinuation), [`Functions`](../function/index.md#functions)

- <span id="resunit-parse-inlined-functions"></span>`fn parse_inlined_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(), gimli::Error>, Buf = R> + 'unit>` — [`Context`](../index.md#context), [`LookupResult`](../lookup/index.md#lookupresult), [`LookupContinuation`](../lookup/index.md#lookupcontinuation)

- <span id="resunit-find-location"></span>`fn find_location(&self, probe: u64, sections: &gimli::Dwarf<R>) -> Result<Option<Location<'_>>, gimli::Error>` — [`Location`](../frame/index.md#location)

- <span id="resunit-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64, sections: &gimli::Dwarf<R>) -> Result<Option<LineLocationRangeIter<'_>>, gimli::Error>` — [`LineLocationRangeIter`](../line/index.md#linelocationrangeiter)

- <span id="resunit-find-function-or-location"></span>`fn find_function_or_location<'unit, 'ctx: 'unit>(self: &'unit Self, probe: u64, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(Option<&'unit Function<R>>, Option<Location<'unit>>), gimli::Error>, Buf = R>>` — [`Context`](../index.md#context), [`LookupResult`](../lookup/index.md#lookupresult), [`LookupContinuation`](../lookup/index.md#lookupcontinuation), [`Function`](../function/index.md#function), [`Location`](../frame/index.md#location)

#### Trait Implementations

##### `impl Any for ResUnit<R>`

- <span id="resunit-any-type-id"></span>`fn type_id(&self) -> TypeId`

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

- <span id="resunit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResUnit<R>`

- <span id="resunit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resunit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ResUnits<R: gimli::Reader>`

```rust
struct ResUnits<R: gimli::Reader> {
    ranges: alloc::boxed::Box<[UnitRange]>,
    units: alloc::boxed::Box<[ResUnit<R>]>,
}
```

*Defined in [`addr2line-0.25.1/src/unit.rs:196-199`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L196-L199)*

#### Implementations

- <span id="resunits-parse"></span>`fn parse(sections: &gimli::Dwarf<R>) -> Result<Self, gimli::Error>`

- <span id="resunits-iter"></span>`fn iter(&self) -> impl Iterator<Item = &ResUnit<R>>` — [`ResUnit`](#resunit)

- <span id="resunits-find-offset"></span>`fn find_offset(&self, offset: gimli::DebugInfoOffset<<R as >::Offset>) -> Result<&gimli::Unit<R>, gimli::Error>`

- <span id="resunits-find"></span>`fn find(&self, probe: u64) -> impl Iterator<Item = &ResUnit<R>>` — [`ResUnit`](#resunit)

  Finds the CUs for the function address given.

  

  There might be multiple CUs whose range contains this address.

  Weak symbols have shown up in the wild which cause this to happen

  but otherwise this can happen if the CU has non-contiguous functions

  but only reports a single range.

  

  Consequently we return an iterator for all CUs which may contain the

  address, and the caller must check if there is actually a function or

  location in the CU for that address.

- <span id="resunits-find-range"></span>`fn find_range(&self, probe_low: u64, probe_high: u64) -> impl Iterator<Item = (&ResUnit<R>, &gimli::Range)>` — [`ResUnit`](#resunit)

  Finds the CUs covering the range of addresses given.

  

  The range is [low, high) (ie, the upper bound is exclusive). This can return multiple

  ranges for the same unit.

- <span id="resunits-find-location-range"></span>`fn find_location_range<'a>(self: &'a Self, probe_low: u64, probe_high: u64, sections: &'a gimli::Dwarf<R>) -> Result<LocationRangeIter<'a, R>, gimli::Error>` — [`LocationRangeIter`](#locationrangeiter)

#### Trait Implementations

##### `impl Any for ResUnits<R>`

- <span id="resunits-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResUnits<R>`

- <span id="resunits-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResUnits<R>`

- <span id="resunits-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ResUnits<R>`

- <span id="resunits-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResUnits<R>`

- <span id="resunits-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ResUnits<R>`

- <span id="resunits-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resunits-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResUnits<R>`

- <span id="resunits-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resunits-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DwoUnit<R: gimli::Reader>`

```rust
struct DwoUnit<R: gimli::Reader> {
    sections: alloc::sync::Arc<gimli::Dwarf<R>>,
    dw_unit: gimli::Unit<R>,
}
```

*Defined in [`addr2line-0.25.1/src/unit.rs:475-478`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L475-L478)*

A DWO unit has its own DWARF sections.

#### Implementations

- <span id="dwounit-unit-ref"></span>`fn unit_ref(&self) -> gimli::UnitRef<'_, R>`

#### Trait Implementations

##### `impl Any for DwoUnit<R>`

- <span id="dwounit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DwoUnit<R>`

- <span id="dwounit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DwoUnit<R>`

- <span id="dwounit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DwoUnit<R>`

- <span id="dwounit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DwoUnit<R>`

- <span id="dwounit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DwoUnit<R>`

- <span id="dwounit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dwounit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DwoUnit<R>`

- <span id="dwounit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dwounit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SupUnit<R: gimli::Reader>`

```rust
struct SupUnit<R: gimli::Reader> {
    offset: gimli::DebugInfoOffset<<R as >::Offset>,
    dw_unit: gimli::Unit<R>,
}
```

*Defined in [`addr2line-0.25.1/src/unit.rs:486-489`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L486-L489)*

#### Trait Implementations

##### `impl Any for SupUnit<R>`

- <span id="supunit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SupUnit<R>`

- <span id="supunit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SupUnit<R>`

- <span id="supunit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SupUnit<R>`

- <span id="supunit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SupUnit<R>`

- <span id="supunit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SupUnit<R>`

- <span id="supunit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="supunit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SupUnit<R>`

- <span id="supunit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="supunit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SupUnits<R: gimli::Reader>`

```rust
struct SupUnits<R: gimli::Reader> {
    units: alloc::boxed::Box<[SupUnit<R>]>,
}
```

*Defined in [`addr2line-0.25.1/src/unit.rs:491-493`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L491-L493)*

#### Implementations

- <span id="supunits-parse"></span>`fn parse(sections: &gimli::Dwarf<R>) -> Result<Self, gimli::Error>`

- <span id="supunits-find-offset"></span>`fn find_offset(&self, offset: gimli::DebugInfoOffset<<R as >::Offset>) -> Result<&gimli::Unit<R>, gimli::Error>`

#### Trait Implementations

##### `impl Any for SupUnits<R>`

- <span id="supunits-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SupUnits<R>`

- <span id="supunits-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SupUnits<R>`

- <span id="supunits-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: gimli::Reader> Default for SupUnits<R>`

- <span id="supunits-default"></span>`fn default() -> Self`

##### `impl<T> From for SupUnits<R>`

- <span id="supunits-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SupUnits<R>`

- <span id="supunits-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SupUnits<R>`

- <span id="supunits-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="supunits-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SupUnits<R>`

- <span id="supunits-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="supunits-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`addr2line-0.25.1/src/unit.rs:539-546`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L539-L546)*

Iterator over `Location`s in a range of addresses, returned by `Context::find_location_range`.

#### Implementations

- <span id="locationrangeiter-next-loc"></span>`fn next_loc(&mut self) -> Result<Option<(u64, u64, Location<'ctx>)>, gimli::Error>` — [`Location`](../frame/index.md#location)

#### Trait Implementations

##### `impl Any for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="locationrangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="locationrangeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-iterator-type-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="locationrangeiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="locationrangeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="locationrangeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `UnitRef<'unit, R>`

```rust
type UnitRef<'unit, R> = (crate::DebugFile, gimli::UnitRef<'unit, R>);
```

*Defined in [`addr2line-0.25.1/src/unit.rs:27`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L27)*

