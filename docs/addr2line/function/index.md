*[addr2line](../index.md) / [function](index.md)*

---

# Module `function`

## Structs

### `LazyFunctions<R: gimli::Reader>`

```rust
struct LazyFunctions<R: gimli::Reader>(core::cell::OnceCell<Result<Functions<R>, gimli::Error>>);
```

#### Implementations

- `fn new() -> Self`

- `fn borrow(self: &Self, unit: gimli::UnitRef<'_, R>) -> Result<&Functions<R>, gimli::Error>` — [`Functions`](#functions)

### `Functions<R: gimli::Reader>`

```rust
struct Functions<R: gimli::Reader> {
    functions: alloc::boxed::Box<[LazyFunction<R>]>,
    addresses: alloc::boxed::Box<[FunctionAddress]>,
}
```

#### Fields

- **`functions`**: `alloc::boxed::Box<[LazyFunction<R>]>`

  List of all `DW_TAG_subprogram` details in the unit.

- **`addresses`**: `alloc::boxed::Box<[FunctionAddress]>`

  List of `DW_TAG_subprogram` address ranges in the unit.

#### Implementations

- `fn parse(unit: gimli::UnitRef<'_, R>) -> Result<Functions<R>, gimli::Error>` — [`Functions`](#functions)

- `fn find_address(self: &Self, probe: u64) -> Option<usize>`

- `fn parse_inlined_functions(self: &Self, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<(), gimli::Error>` — [`DebugFile`](../index.md), [`Context`](../index.md)

### `FunctionAddress`

```rust
struct FunctionAddress {
    range: gimli::Range,
    function: usize,
}
```

A single address range for a function.

It is possible for a function to have multiple address ranges; this
is handled by having multiple `FunctionAddress` entries with the same
`function` field.

#### Fields

- **`function`**: `usize`

  An index into `Functions::functions`.

### `LazyFunction<R: gimli::Reader>`

```rust
struct LazyFunction<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    lazy: core::cell::OnceCell<Result<Function<R>, gimli::Error>>,
}
```

#### Implementations

- `fn new(dw_die_offset: gimli::UnitOffset<<R as >::Offset>) -> Self`

- `fn borrow(self: &Self, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<&Function<R>, gimli::Error>` — [`DebugFile`](../index.md), [`Context`](../index.md), [`Function`](#function)

### `Function<R: gimli::Reader>`

```rust
struct Function<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    name: Option<R>,
    inlined_functions: alloc::boxed::Box<[InlinedFunction<R>]>,
    inlined_addresses: alloc::boxed::Box<[InlinedFunctionAddress]>,
}
```

#### Fields

- **`inlined_functions`**: `alloc::boxed::Box<[InlinedFunction<R>]>`

  List of all `DW_TAG_inlined_subroutine` details in this function.

- **`inlined_addresses`**: `alloc::boxed::Box<[InlinedFunctionAddress]>`

  List of `DW_TAG_inlined_subroutine` address ranges in this function.

#### Implementations

- `fn parse(dw_die_offset: gimli::UnitOffset<<R as >::Offset>, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<Self, gimli::Error>` — [`DebugFile`](../index.md), [`Context`](../index.md)

- `fn parse_children(state: &mut InlinedState<'_, R>, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`InlinedState`](#inlinedstate)

- `fn skip(entries: &mut gimli::EntriesRaw<'_, '_, R>, abbrev: &gimli::Abbreviation, depth: isize) -> Result<(), gimli::Error>`

- `fn find_inlined_functions(self: &Self, probe: u64) -> alloc::vec::Vec<&InlinedFunction<R>>` — [`InlinedFunction`](#inlinedfunction)

### `InlinedFunctionAddress`

```rust
struct InlinedFunctionAddress {
    range: gimli::Range,
    call_depth: usize,
    function: usize,
}
```

#### Fields

- **`function`**: `usize`

  An index into `Function::inlined_functions`.

### `InlinedFunction<R: gimli::Reader>`

```rust
struct InlinedFunction<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    name: Option<R>,
    call_file: Option<u64>,
    call_line: u32,
    call_column: u32,
}
```

#### Implementations

- `fn parse(state: &mut InlinedState<'_, R>, dw_die_offset: gimli::UnitOffset<<R as >::Offset>, abbrev: &gimli::Abbreviation, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`InlinedState`](#inlinedstate)

### `InlinedState<'a, R: gimli::Reader>`

```rust
struct InlinedState<'a, R: gimli::Reader> {
    entries: gimli::EntriesRaw<'a, 'a, R>,
    functions: alloc::vec::Vec<InlinedFunction<R>>,
    addresses: alloc::vec::Vec<InlinedFunctionAddress>,
    file: crate::DebugFile,
    unit: gimli::UnitRef<'a, R>,
    ctx: &'a crate::Context<R>,
}
```

## Functions

### `name_attr`

```rust
fn name_attr<R>(attr: gimli::AttributeValue<R>, file: crate::DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &crate::Context<R>, recursion_limit: usize) -> Result<Option<R>, gimli::Error>
where
    R: gimli::Reader
```

### `name_entry`

```rust
fn name_entry<R>(file: crate::DebugFile, unit: gimli::UnitRef<'_, R>, offset: gimli::UnitOffset<<R as >::Offset>, ctx: &crate::Context<R>, recursion_limit: usize) -> Result<Option<R>, gimli::Error>
where
    R: gimli::Reader
```

