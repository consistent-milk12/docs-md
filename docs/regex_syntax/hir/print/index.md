*[regex_syntax](../../index.md) / [hir](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Hir`.

## Structs

### `PrinterBuilder`

```rust
struct PrinterBuilder {
    _priv: (),
}
```

A builder for constructing a printer.

Note that since a printer doesn't have any configuration knobs, this type
remains unexported.

#### Implementations

- `fn new() -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

- `fn build(self: &Self) -> Printer` — [`Printer`](#printer)

#### Trait Implementations

##### `impl Clone for PrinterBuilder`

- `fn clone(self: &Self) -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

##### `impl Debug for PrinterBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PrinterBuilder`

- `fn default() -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

### `Printer`

```rust
struct Printer {
    _priv: (),
}
```

A printer for a regular expression's high-level intermediate
representation.

A printer converts a high-level intermediate representation (HIR) to a
regular expression pattern string. This particular printer uses constant
stack space and heap space proportional to the size of the HIR.

Since this printer is only using the HIR, the pattern it prints will likely
not resemble the original pattern at all. For example, a pattern like
`\pL` will have its entire class written out.

The purpose of this printer is to provide a means to mutate an HIR and then
build a regular expression from the result of that mutation. (A regex
library could provide a constructor from this HIR explicitly, but that
creates an unnecessary public coupling between the regex library and this
specific HIR representation.)

#### Implementations

- `fn new() -> Printer` — [`Printer`](#printer)

- `fn print<W: fmt::Write>(self: &mut Self, hir: &Hir, wtr: W) -> fmt::Result` — [`Hir`](../index.md)

#### Trait Implementations

##### `impl Debug for Printer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Writer<W>`

```rust
struct Writer<W> {
    wtr: W,
}
```

#### Implementations

- `fn write_literal_char(self: &mut Self, c: char) -> fmt::Result`

- `fn write_literal_byte(self: &mut Self, b: u8) -> fmt::Result`

- `fn write_literal_class_byte(self: &mut Self, b: u8) -> fmt::Result`

#### Trait Implementations

##### `impl<W: $crate::fmt::Debug> Debug for Writer<W>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<W: fmt::Write> Visitor for Writer<W>`

- `type Output = ()`

- `type Err = Error`

- `fn finish(self: Self) -> fmt::Result`

- `fn visit_pre(self: &mut Self, hir: &Hir) -> fmt::Result` — [`Hir`](../index.md)

- `fn visit_post(self: &mut Self, hir: &Hir) -> fmt::Result` — [`Hir`](../index.md)

- `fn visit_alternation_in(self: &mut Self) -> fmt::Result`

