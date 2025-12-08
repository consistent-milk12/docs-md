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
  - [`demangle_auto`](#demangle_auto)

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
| [`demangle_auto`](#demangle_auto) | fn | Apply 'best effort' demangling of a symbol name. |

## Structs

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

- <span id="frameiter-new-empty"></span>`fn new_empty() -> Self`

- <span id="frameiter-new-location"></span>`fn new_location(location: Location<'ctx>) -> Self` — [`Location`](../index.md)

- <span id="frameiter-new-frames"></span>`fn new_frames(unit: &'ctx ResUnit<R>, sections: &'ctx gimli::Dwarf<R>, function: &'ctx Function<R>, inlined_functions: alloc::vec::Vec<&'ctx InlinedFunction<R>>, location: Option<Location<'ctx>>) -> Self` — [`ResUnit`](../unit/index.md), [`Function`](../function/index.md), [`InlinedFunction`](../function/index.md), [`Location`](../index.md)

- <span id="frameiter-next"></span>`fn next(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>` — [`Frame`](../index.md)

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

## Functions

### `demangle`

```rust
fn demangle(name: &str, language: gimli::DwLang) -> Option<alloc::string::String>
```

Demangle a symbol name using the demangling scheme for the given language.

Returns `None` if demangling failed or is not required.

### `demangle_auto`

```rust
fn demangle_auto(name: alloc::borrow::Cow<'_, str>, language: Option<gimli::DwLang>) -> alloc::borrow::Cow<'_, str>
```

Apply 'best effort' demangling of a symbol name.

If `language` is given, then only the demangling scheme for that language
is used.

If `language` is `None`, then heuristics are used to determine how to
demangle the name. Currently, these heuristics are very basic.

If demangling fails or is not required, then `name` is returned unchanged.

