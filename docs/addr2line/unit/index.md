*[addr2line](../index.md) / [unit](index.md)*

---

# Module `unit`

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

- `fn unit_ref<'a>(self: &'a Self, sections: &'a gimli::Dwarf<R>) -> gimli::UnitRef<'a, R>`

- `fn dwarf_and_unit<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<SimpleLookup<Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>, R, impl FnOnce(Option<Arc<gimli::Dwarf<R>>>) -> Result<(crate::DebugFile, gimli::UnitRef<'unit, R>), gimli::Error>>>` — [`Context`](../index.md), [`LookupResult`](../lookup/index.md), [`SimpleLookup`](../lookup/index.md), [`DebugFile`](../index.md)

- `fn parse_lines(self: &Self, sections: &gimli::Dwarf<R>) -> Result<Option<&Lines>, gimli::Error>` — [`Lines`](../line/index.md)

- `fn parse_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<&'unit Functions<R>, gimli::Error>, Buf = R>>` — [`Context`](../index.md), [`LookupResult`](../lookup/index.md), [`LookupContinuation`](../lookup/index.md), [`Functions`](../function/index.md)

- `fn parse_inlined_functions<'unit, 'ctx: 'unit>(self: &'unit Self, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(), gimli::Error>, Buf = R> + 'unit>` — [`Context`](../index.md), [`LookupResult`](../lookup/index.md), [`LookupContinuation`](../lookup/index.md)

- `fn find_location(self: &Self, probe: u64, sections: &gimli::Dwarf<R>) -> Result<Option<Location<'_>>, gimli::Error>` — [`Location`](../frame/index.md)

- `fn find_location_range(self: &Self, probe_low: u64, probe_high: u64, sections: &gimli::Dwarf<R>) -> Result<Option<LineLocationRangeIter<'_>>, gimli::Error>` — [`LineLocationRangeIter`](../line/index.md)

- `fn find_function_or_location<'unit, 'ctx: 'unit>(self: &'unit Self, probe: u64, ctx: &'ctx Context<R>) -> LookupResult<impl LookupContinuation<Output = Result<(Option<&'unit Function<R>>, Option<Location<'unit>>), gimli::Error>, Buf = R>>` — [`Context`](../index.md), [`LookupResult`](../lookup/index.md), [`LookupContinuation`](../lookup/index.md), [`Function`](../function/index.md), [`Location`](../frame/index.md)

### `ResUnits<R: gimli::Reader>`

```rust
struct ResUnits<R: gimli::Reader> {
    ranges: alloc::boxed::Box<[UnitRange]>,
    units: alloc::boxed::Box<[ResUnit<R>]>,
}
```

#### Implementations

- `fn parse(sections: &gimli::Dwarf<R>) -> Result<Self, gimli::Error>`

- `fn iter(self: &Self) -> impl Iterator<Item = &ResUnit<R>>` — [`ResUnit`](#resunit)

- `fn find_offset(self: &Self, offset: gimli::DebugInfoOffset<<R as >::Offset>) -> Result<&gimli::Unit<R>, gimli::Error>`

- `fn find(self: &Self, probe: u64) -> impl Iterator<Item = &ResUnit<R>>` — [`ResUnit`](#resunit)

- `fn find_range(self: &Self, probe_low: u64, probe_high: u64) -> impl Iterator<Item = (&ResUnit<R>, &gimli::Range)>` — [`ResUnit`](#resunit)

- `fn find_location_range<'a>(self: &'a Self, probe_low: u64, probe_high: u64, sections: &'a gimli::Dwarf<R>) -> Result<LocationRangeIter<'a, R>, gimli::Error>` — [`LocationRangeIter`](#locationrangeiter)

### `DwoUnit<R: gimli::Reader>`

```rust
struct DwoUnit<R: gimli::Reader> {
    sections: alloc::sync::Arc<gimli::Dwarf<R>>,
    dw_unit: gimli::Unit<R>,
}
```

A DWO unit has its own DWARF sections.

#### Implementations

- `fn unit_ref(self: &Self) -> gimli::UnitRef<'_, R>`

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

- `fn parse(sections: &gimli::Dwarf<R>) -> Result<Self, gimli::Error>`

- `fn find_offset(self: &Self, offset: gimli::DebugInfoOffset<<R as >::Offset>) -> Result<&gimli::Unit<R>, gimli::Error>`

#### Trait Implementations

##### `impl<R: gimli::Reader> Default for SupUnits<R>`

- `fn default() -> Self`

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

- `fn next_loc(self: &mut Self) -> Result<Option<(u64, u64, Location<'ctx>)>, gimli::Error>` — [`Location`](../frame/index.md)

#### Trait Implementations

##### `impl<I> IntoIterator for LocationRangeIter<'ctx, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'ctx, R> Iterator for LocationRangeIter<'ctx, R>`

- `type Item = (u64, u64, Location<'ctx>)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Type Aliases

### `UnitRef<'unit, R>`

```rust
type UnitRef<'unit, R> = (crate::DebugFile, gimli::UnitRef<'unit, R>);
```

