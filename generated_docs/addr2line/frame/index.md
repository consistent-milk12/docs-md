*[addr2line](../index.md) / [frame](index.md)*

---

# Module `frame`

## Contents

- [Structs](#structs)
  - [`Location`](#location)
  - [`Frame`](#frame)
  - [`FrameIter`](#frameiter)
  - [`FrameIterFrames`](#frameiterframes)
  - [`FunctionName`](#functionname)
- [Enums](#enums)
  - [`FrameIterState`](#frameiterstate)
- [Functions](#functions)
  - [`demangle`](#demangle)
  - [`demangle_auto`](#demangle-auto)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Location`](#location) | struct | A source location. |
| [`Frame`](#frame) | struct | A function frame. |
| [`FrameIter`](#frameiter) | struct | An iterator over function frames. |
| [`FrameIterFrames`](#frameiterframes) | struct |  |
| [`FunctionName`](#functionname) | struct | A function name. |
| [`FrameIterState`](#frameiterstate) | enum |  |
| [`demangle`](#demangle) | fn | Demangle a symbol name using the demangling scheme for the given language. |
| [`demangle_auto`](#demangle-auto) | fn | Apply 'best effort' demangling of a symbol name. |

## Structs

### `Location<'a>`

```rust
struct Location<'a> {
    pub file: Option<&'a str>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:8-17`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L8-L17)*

A source location.

#### Fields

- **`file`**: `Option<&'a str>`

  The file name.

- **`line`**: `Option<u32>`

  The line number.

- **`column`**: `Option<u32>`

  The column number.
  
  A value of `Some(0)` indicates the left edge.

### `Frame<'ctx, R: gimli::Reader>`

```rust
struct Frame<'ctx, R: gimli::Reader> {
    pub dw_die_offset: Option<gimli::UnitOffset<<R as >::Offset>>,
    pub function: Option<FunctionName<R>>,
    pub location: Option<Location<'ctx>>,
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:20-27`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L20-L27)*

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

*Defined in [`addr2line-0.25.1/src/frame.rs:30-32`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L30-L32)*

An iterator over function frames.

#### Implementations

- <span id="frameiter-new-empty"></span>`fn new_empty() -> Self`

- <span id="frameiter-new-location"></span>`fn new_location(location: Location<'ctx>) -> Self` — [`Location`](#location)

- <span id="frameiter-new-frames"></span>`fn new_frames(unit: &'ctx ResUnit<R>, sections: &'ctx gimli::Dwarf<R>, function: &'ctx Function<R>, inlined_functions: alloc::vec::Vec<&'ctx InlinedFunction<R>>, location: Option<Location<'ctx>>) -> Self` — [`ResUnit`](../unit/index.md#resunit), [`Function`](../function/index.md#function), [`InlinedFunction`](../function/index.md#inlinedfunction), [`Location`](#location)

- <span id="frameiter-next"></span>`fn next(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` — [`Frame`](#frame)

### `FrameIterFrames<'ctx, R>`

```rust
struct FrameIterFrames<'ctx, R>
where
    R: gimli::Reader {
    unit: &'ctx crate::unit::ResUnit<R>,
    sections: &'ctx gimli::Dwarf<R>,
    function: &'ctx crate::function::Function<R>,
    inlined_functions: iter::Rev<alloc::vec::IntoIter<&'ctx crate::function::InlinedFunction<R>>>,
    next: Option<Location<'ctx>>,
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:43-52`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L43-L52)*

### `FunctionName<R: gimli::Reader>`

```rust
struct FunctionName<R: gimli::Reader> {
    pub name: R,
    pub language: Option<gimli::DwLang>,
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:163-168`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L163-L168)*

A function name.

#### Fields

- **`name`**: `R`

  The name of the function.

- **`language`**: `Option<gimli::DwLang>`

  The language of the compilation unit containing this function.

#### Implementations

- <span id="functionname-raw-name"></span>`fn raw_name(&self) -> Result<Cow<'_, str>, gimli::Error>`

- <span id="functionname-demangle"></span>`fn demangle(&self) -> Result<Cow<'_, str>, gimli::Error>`

## Enums

### `FrameIterState<'ctx, R>`

```rust
enum FrameIterState<'ctx, R>
where
    R: gimli::Reader {
    Empty,
    Location(Option<Location<'ctx>>),
    Frames(FrameIterFrames<'ctx, R>),
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:34-41`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L34-L41)*

## Functions

### `demangle`

```rust
fn demangle(name: &str, language: gimli::DwLang) -> Option<alloc::string::String>
```

*Defined in [`addr2line-0.25.1/src/frame.rs:186-202`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L186-L202)*

Demangle a symbol name using the demangling scheme for the given language.

Returns `None` if demangling failed or is not required.

### `demangle_auto`

```rust
fn demangle_auto(name: alloc::borrow::Cow<'_, str>, language: Option<gimli::DwLang>) -> alloc::borrow::Cow<'_, str>
```

*Defined in [`addr2line-0.25.1/src/frame.rs:213-221`](../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L213-L221)*

Apply 'best effort' demangling of a symbol name.

If `language` is given, then only the demangling scheme for that language
is used.

If `language` is `None`, then heuristics are used to determine how to
demangle the name. Currently, these heuristics are very basic.

If demangling fails or is not required, then `name` is returned unchanged.

