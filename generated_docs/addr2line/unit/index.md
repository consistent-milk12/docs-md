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

#### Implementations

- <span id="resunit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, sections: &'a gimli::Dwarf<R>) -> gimli::UnitRef<'a, R>`

- <span id="resunit-dwarf-and-unit"></span>`fn dwarf_and_unit<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<SimpleLookup<Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>, R, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>>>` — [`Context`](../index.md), [`LookupResult`](../index.md), [`SimpleLookup`](../lookup/index.md), [`DebugFile`](../index.md)

- <span id="resunit-parse-lines"></span>`fn parse_lines(&self, sections: &gimli::Dwarf<R>) -> Result<Option<&Lines>, gimli::Error>` — [`Lines`](../line/index.md)

- <span id="resunit-parse-functions"></span>`fn parse_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<&'unit Functions<R>, gimli::Error>, Buf = R>>` — [`Context`](../index.md), [`LookupResult`](../index.md), [`LookupContinuation`](../index.md), [`Functions`](../function/index.md)

- <span id="resunit-parse-inlined-functions"></span>`fn parse_inlined_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(), gimli::Error>, Buf = R> + 'unit>` — [`Context`](../index.md), [`LookupResult`](../index.md), [`LookupContinuation`](../index.md)

- <span id="resunit-find-location"></span>`fn find_location(&self, probe: u64, sections: &gimli::Dwarf<R>) -> Result<Option<Location<'_>>, gimli::Error>` — [`Location`](../index.md)

- <span id="resunit-find-location-range"></span>`fn find_location_range(&self, probe_low: u64, probe_high: u64, sections: &gimli::Dwarf<R>) -> Result<Option<LineLocationRangeIter<'_>>, gimli::Error>` — [`LineLocationRangeIter`](../line/index.md)

- <span id="resunit-find-function-or-location"></span>`fn find_function_or_location<'unit, 'ctx: 'unit>(self: &'unit Self, probe: u64, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(Option<&'unit Function<R>>, Option<Location<'unit>>), gimli::Error>, Buf = R>>` — [`Context`](../index.md), [`LookupResult`](../index.md), [`LookupContinuation`](../index.md), [`Function`](../function/index.md), [`Location`](../index.md)

### `ResUnits<R: gimli::Reader>`

```rust
struct ResUnits<R: gimli::Reader> {
    ranges: alloc::boxed::Box<[UnitRange]>,
    units: alloc::boxed::Box<[ResUnit<R>]>,
}
```

#### Implementations

- <span id="resunits-parse"></span>`fn parse(sections: &gimli::Dwarf<R>) -> Result<Self, gimli::Error>`

- <span id="resunits-iter"></span>`fn iter(&self) -> impl Iterator<Item = &ResUnit<R>>` — [`ResUnit`](#resunit)

- <span id="resunits-find-offset"></span>`fn find_offset(&self, offset: gimli::DebugInfoOffset<<R as >::Offset>) -> Result<&gimli::Unit<R>, gimli::Error>`

- <span id="resunits-find"></span>`fn find(&self, probe: u64) -> impl Iterator<Item = &ResUnit<R>>` — [`ResUnit`](#resunit)

- <span id="resunits-find-range"></span>`fn find_range(&self, probe_low: u64, probe_high: u64) -> impl Iterator<Item = (&ResUnit<R>, &gimli::Range)>` — [`ResUnit`](#resunit)

- <span id="resunits-find-location-range"></span>`fn find_location_range<'a>(self: &'a Self, probe_low: u64, probe_high: u64, sections: &'a gimli::Dwarf<R>) -> Result<LocationRangeIter<'a, R>, gimli::Error>` — [`LocationRangeIter`](../index.md)

### `DwoUnit<R: gimli::Reader>`

```rust
struct DwoUnit<R: gimli::Reader> {
    sections: alloc::sync::Arc<gimli::Dwarf<R>>,
    dw_unit: gimli::Unit<R>,
}
```

A DWO unit has its own DWARF sections.

#### Implementations

- <span id="dwounit-unit-ref"></span>`fn unit_ref(&self) -> gimli::UnitRef<'_, R>`

### `SupUnit<R: gimli::Reader>`

```rust
struct SupUnit<R: gimli::Reader> {
    offset: gimli::DebugInfoOffset<<R as >::Offset>,
    dw_unit: gimli::Unit<R>,
}
```

### `SupUnits<R: gimli::Reader>`

```rust
struct SupUnits<R: gimli::Reader> {
    units: alloc::boxed::Box<[SupUnit<R>]>,
}
```

#### Implementations

- <span id="supunits-parse"></span>`fn parse(sections: &gimli::Dwarf<R>) -> Result<Self, gimli::Error>`

- <span id="supunits-find-offset"></span>`fn find_offset(&self, offset: gimli::DebugInfoOffset<<R as >::Offset>) -> Result<&gimli::Unit<R>, gimli::Error>`

#### Trait Implementations

##### `impl<R: gimli::Reader> Default for SupUnits<R>`

- <span id="supunits-default"></span>`fn default() -> Self`

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

- <span id="locationrangeiter-next-loc"></span>`fn next_loc(&mut self) -> Result<Option<(u64, u64, Location<'ctx>)>, gimli::Error>` — [`Location`](../index.md)

#### Trait Implementations

##### `impl<I> IntoIterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="locationrangeiter-intoiter"></span>`type IntoIter = I`

- <span id="locationrangeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'ctx, R> Iterator for LocationRangeIter<'ctx, R>`

- <span id="locationrangeiter-item"></span>`type Item = (u64, u64, Location<'ctx>)`

- <span id="locationrangeiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `UnitRef<'unit, R>`

```rust
type UnitRef<'unit, R> = (crate::DebugFile, gimli::UnitRef<'unit, R>);
```

