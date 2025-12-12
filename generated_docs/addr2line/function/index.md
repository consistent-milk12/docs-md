*[addr2line](../index.md) / [function](index.md)*

---

# Module `function`

## Contents

- [Structs](#structs)
  - [`LazyFunctions`](#lazyfunctions)
  - [`Functions`](#functions)
  - [`FunctionAddress`](#functionaddress)
  - [`LazyFunction`](#lazyfunction)
  - [`Function`](#function)
  - [`InlinedFunctionAddress`](#inlinedfunctionaddress)
  - [`InlinedFunction`](#inlinedfunction)
  - [`InlinedState`](#inlinedstate)
- [Functions](#functions)
  - [`name_attr`](#name-attr)
  - [`name_entry`](#name-entry)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LazyFunctions`](#lazyfunctions) | struct |  |
| [`Functions`](#functions) | struct |  |
| [`FunctionAddress`](#functionaddress) | struct | A single address range for a function. |
| [`LazyFunction`](#lazyfunction) | struct |  |
| [`Function`](#function) | struct |  |
| [`InlinedFunctionAddress`](#inlinedfunctionaddress) | struct |  |
| [`InlinedFunction`](#inlinedfunction) | struct |  |
| [`InlinedState`](#inlinedstate) | struct |  |
| [`name_attr`](#name-attr) | fn |  |
| [`name_entry`](#name-entry) | fn |  |

## Structs

### `LazyFunctions<R: gimli::Reader>`

```rust
struct LazyFunctions<R: gimli::Reader>(core::cell::OnceCell<Result<Functions<R>, gimli::Error>>);
```

*Defined in [`addr2line-0.25.1/src/function.rs:8`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L8)*

#### Implementations

- <span id="lazyfunctions-new"></span>`fn new() -> Self`

- <span id="lazyfunctions-borrow"></span>`fn borrow(&self, unit: gimli::UnitRef<'_, R>) -> Result<&Functions<R>, gimli::Error>` — [`Functions`](#functions)

### `Functions<R: gimli::Reader>`

```rust
struct Functions<R: gimli::Reader> {
    functions: alloc::boxed::Box<[LazyFunction<R>]>,
    addresses: alloc::boxed::Box<[FunctionAddress]>,
}
```

*Defined in [`addr2line-0.25.1/src/function.rs:23-28`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L23-L28)*

#### Fields

- **`functions`**: `alloc::boxed::Box<[LazyFunction<R>]>`

  List of all `DW_TAG_subprogram` details in the unit.

- **`addresses`**: `alloc::boxed::Box<[FunctionAddress]>`

  List of `DW_TAG_subprogram` address ranges in the unit.

#### Implementations

- <span id="functions-parse"></span>`fn parse(unit: gimli::UnitRef<'_, R>) -> Result<Functions<R>, gimli::Error>` — [`Functions`](#functions)

- <span id="functions-find-address"></span>`fn find_address(&self, probe: u64) -> Option<usize>`

- <span id="functions-parse-inlined-functions"></span>`fn parse_inlined_functions(&self, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<(), gimli::Error>` — [`DebugFile`](../index.md#debugfile), [`Context`](../index.md#context)

### `FunctionAddress`

```rust
struct FunctionAddress {
    range: gimli::Range,
    function: usize,
}
```

*Defined in [`addr2line-0.25.1/src/function.rs:35-39`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L35-L39)*

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

*Defined in [`addr2line-0.25.1/src/function.rs:41-44`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L41-L44)*

#### Implementations

- <span id="lazyfunction-new"></span>`fn new(dw_die_offset: gimli::UnitOffset<<R as >::Offset>) -> Self`

- <span id="lazyfunction-borrow"></span>`fn borrow(&self, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<&Function<R>, gimli::Error>` — [`DebugFile`](../index.md#debugfile), [`Context`](../index.md#context), [`Function`](#function)

### `Function<R: gimli::Reader>`

```rust
struct Function<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    name: Option<R>,
    inlined_functions: alloc::boxed::Box<[InlinedFunction<R>]>,
    inlined_addresses: alloc::boxed::Box<[InlinedFunctionAddress]>,
}
```

*Defined in [`addr2line-0.25.1/src/function.rs:67-74`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L67-L74)*

#### Fields

- **`inlined_functions`**: `alloc::boxed::Box<[InlinedFunction<R>]>`

  List of all `DW_TAG_inlined_subroutine` details in this function.

- **`inlined_addresses`**: `alloc::boxed::Box<[InlinedFunctionAddress]>`

  List of `DW_TAG_inlined_subroutine` address ranges in this function.

#### Implementations

- <span id="function-parse"></span>`fn parse(dw_die_offset: gimli::UnitOffset<<R as >::Offset>, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<Self, gimli::Error>` — [`DebugFile`](../index.md#debugfile), [`Context`](../index.md#context)

- <span id="function-parse-children"></span>`fn parse_children(state: &mut InlinedState<'_, R>, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`InlinedState`](#inlinedstate)

- <span id="function-skip"></span>`fn skip(entries: &mut gimli::EntriesRaw<'_, '_, R>, abbrev: &gimli::Abbreviation, depth: isize) -> Result<(), gimli::Error>`

- <span id="function-find-inlined-functions"></span>`fn find_inlined_functions(&self, probe: u64) -> alloc::vec::Vec<&InlinedFunction<R>>` — [`InlinedFunction`](#inlinedfunction)

### `InlinedFunctionAddress`

```rust
struct InlinedFunctionAddress {
    range: gimli::Range,
    call_depth: usize,
    function: usize,
}
```

*Defined in [`addr2line-0.25.1/src/function.rs:76-81`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L76-L81)*

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

*Defined in [`addr2line-0.25.1/src/function.rs:83-89`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L83-L89)*

#### Implementations

- <span id="inlinedfunction-parse"></span>`fn parse(state: &mut InlinedState<'_, R>, dw_die_offset: gimli::UnitOffset<<R as >::Offset>, abbrev: &gimli::Abbreviation, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`InlinedState`](#inlinedstate)

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

*Defined in [`addr2line-0.25.1/src/function.rs:463-473`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L463-L473)*

## Functions

### `name_attr`

```rust
fn name_attr<R>(attr: gimli::AttributeValue<R>, file: crate::DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &crate::Context<R>, recursion_limit: usize) -> Result<Option<R>, gimli::Error>
where
    R: gimli::Reader
```

*Defined in [`addr2line-0.25.1/src/function.rs:475-511`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L475-L511)*

### `name_entry`

```rust
fn name_entry<R>(file: crate::DebugFile, unit: gimli::UnitRef<'_, R>, offset: gimli::UnitOffset<<R as >::Offset>, ctx: &crate::Context<R>, recursion_limit: usize) -> Result<Option<R>, gimli::Error>
where
    R: gimli::Reader
```

*Defined in [`addr2line-0.25.1/src/function.rs:513-563`](../../../.source_1765521767/addr2line-0.25.1/src/function.rs#L513-L563)*

