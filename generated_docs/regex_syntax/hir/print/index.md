*[regex_syntax](../../index.md) / [hir](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Hir`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PrinterBuilder`](#printerbuilder) | struct | A builder for constructing a printer. |
| [`Printer`](#printer) | struct | A printer for a regular expression's high-level intermediate representation. |
| [`Writer`](#writer) | struct |  |

## Structs

### `PrinterBuilder`

```rust
struct PrinterBuilder {
    _priv: (),
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/print.rs:21-23`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/print.rs#L21-L23)*

A builder for constructing a printer.

Note that since a printer doesn't have any configuration knobs, this type
remains unexported.

#### Implementations

- <span id="printerbuilder-new"></span>`fn new() -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

- <span id="printerbuilder-build"></span>`fn build(&self) -> Printer` — [`Printer`](#printer)

#### Trait Implementations

##### `impl Clone for PrinterBuilder`

- <span id="printerbuilder-clone"></span>`fn clone(&self) -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

##### `impl Debug for PrinterBuilder`

- <span id="printerbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PrinterBuilder`

- <span id="printerbuilder-default"></span>`fn default() -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

### `Printer`

```rust
struct Printer {
    _priv: (),
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/print.rs:58-60`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/print.rs#L58-L60)*

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

- <span id="printer-new"></span>`fn new() -> Printer` — [`Printer`](#printer)

- <span id="printer-print"></span>`fn print<W: fmt::Write>(&mut self, hir: &Hir, wtr: W) -> fmt::Result` — [`Hir`](../index.md#hir)

#### Trait Implementations

##### `impl Debug for Printer`

- <span id="printer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Writer<W>`

```rust
struct Writer<W> {
    wtr: W,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/print.rs:78-80`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/print.rs#L78-L80)*

#### Implementations

- <span id="writer-write-literal-char"></span>`fn write_literal_char(&mut self, c: char) -> fmt::Result`

- <span id="writer-write-literal-byte"></span>`fn write_literal_byte(&mut self, b: u8) -> fmt::Result`

- <span id="writer-write-literal-class-byte"></span>`fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result`

#### Trait Implementations

##### `impl<W: fmt::Debug> Debug for Writer<W>`

- <span id="writer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: fmt::Write> Visitor for Writer<W>`

- <span id="writer-visitor-type-output"></span>`type Output = ()`

- <span id="writer-visitor-type-err"></span>`type Err = Error`

- <span id="writer-finish"></span>`fn finish(self) -> fmt::Result`

- <span id="writer-visit-pre"></span>`fn visit_pre(&mut self, hir: &Hir) -> fmt::Result` — [`Hir`](../index.md#hir)

- <span id="writer-visit-post"></span>`fn visit_post(&mut self, hir: &Hir) -> fmt::Result` — [`Hir`](../index.md#hir)

- <span id="writer-visit-alternation-in"></span>`fn visit_alternation_in(&mut self) -> fmt::Result`

