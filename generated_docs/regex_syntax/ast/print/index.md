*[regex_syntax](../../index.md) / [ast](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Ast`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PrinterBuilder`](#printerbuilder) | struct | A builder for constructing a printer. |
| [`Printer`](#printer) | struct | A printer for a regular expression abstract syntax tree. |
| [`Writer`](#writer) | struct |  |

## Structs

### `PrinterBuilder`

```rust
struct PrinterBuilder {
    _priv: (),
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/print.rs:18-20`](../../../../.source_1765210505/regex-syntax-0.8.8/src/ast/print.rs#L18-L20)*

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

*Defined in [`regex-syntax-0.8.8/src/ast/print.rs:48-50`](../../../../.source_1765210505/regex-syntax-0.8.8/src/ast/print.rs#L48-L50)*

A printer for a regular expression abstract syntax tree.

A printer converts an abstract syntax tree (AST) to a regular expression
pattern string. This particular printer uses constant stack space and heap
space proportional to the size of the AST.

This printer will not necessarily preserve the original formatting of the
regular expression pattern string. For example, all whitespace and comments
are ignored.

#### Implementations

- <span id="printer-new"></span>`fn new() -> Printer` — [`Printer`](#printer)

- <span id="printer-print"></span>`fn print<W: fmt::Write>(&mut self, ast: &Ast, wtr: W) -> fmt::Result` — [`Ast`](../index.md)

#### Trait Implementations

##### `impl Debug for Printer`

- <span id="printer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Writer<W>`

```rust
struct Writer<W> {
    wtr: W,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/print.rs:68-70`](../../../../.source_1765210505/regex-syntax-0.8.8/src/ast/print.rs#L68-L70)*

#### Implementations

- <span id="writer-fmt-group-pre"></span>`fn fmt_group_pre(&mut self, ast: &ast::Group) -> fmt::Result` — [`Group`](../index.md)

- <span id="writer-fmt-group-post"></span>`fn fmt_group_post(&mut self, _ast: &ast::Group) -> fmt::Result` — [`Group`](../index.md)

- <span id="writer-fmt-repetition"></span>`fn fmt_repetition(&mut self, ast: &ast::Repetition) -> fmt::Result` — [`Repetition`](../index.md)

- <span id="writer-fmt-repetition-range"></span>`fn fmt_repetition_range(&mut self, ast: &ast::RepetitionRange) -> fmt::Result` — [`RepetitionRange`](../index.md)

- <span id="writer-fmt-literal"></span>`fn fmt_literal(&mut self, ast: &ast::Literal) -> fmt::Result` — [`Literal`](../index.md)

- <span id="writer-fmt-assertion"></span>`fn fmt_assertion(&mut self, ast: &ast::Assertion) -> fmt::Result` — [`Assertion`](../index.md)

- <span id="writer-fmt-set-flags"></span>`fn fmt_set_flags(&mut self, ast: &ast::SetFlags) -> fmt::Result` — [`SetFlags`](../index.md)

- <span id="writer-fmt-flags"></span>`fn fmt_flags(&mut self, ast: &ast::Flags) -> fmt::Result` — [`Flags`](../index.md)

- <span id="writer-fmt-class-bracketed-pre"></span>`fn fmt_class_bracketed_pre(&mut self, ast: &ast::ClassBracketed) -> fmt::Result` — [`ClassBracketed`](../index.md)

- <span id="writer-fmt-class-bracketed-post"></span>`fn fmt_class_bracketed_post(&mut self, _ast: &ast::ClassBracketed) -> fmt::Result` — [`ClassBracketed`](../index.md)

- <span id="writer-fmt-class-set-binary-op-kind"></span>`fn fmt_class_set_binary_op_kind(&mut self, ast: &ast::ClassSetBinaryOpKind) -> fmt::Result` — [`ClassSetBinaryOpKind`](../index.md)

- <span id="writer-fmt-class-perl"></span>`fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result` — [`ClassPerl`](../index.md)

- <span id="writer-fmt-class-ascii"></span>`fn fmt_class_ascii(&mut self, ast: &ast::ClassAscii) -> fmt::Result` — [`ClassAscii`](../index.md)

- <span id="writer-fmt-class-unicode"></span>`fn fmt_class_unicode(&mut self, ast: &ast::ClassUnicode) -> fmt::Result` — [`ClassUnicode`](../index.md)

#### Trait Implementations

##### `impl<W: fmt::Debug> Debug for Writer<W>`

- <span id="writer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: fmt::Write> Visitor for Writer<W>`

- <span id="writer-type-output"></span>`type Output = ()`

- <span id="writer-type-err"></span>`type Err = Error`

- <span id="writer-finish"></span>`fn finish(self) -> fmt::Result`

- <span id="writer-visit-pre"></span>`fn visit_pre(&mut self, ast: &Ast) -> fmt::Result` — [`Ast`](../index.md)

- <span id="writer-visit-post"></span>`fn visit_post(&mut self, ast: &Ast) -> fmt::Result` — [`Ast`](../index.md)

- <span id="writer-visit-alternation-in"></span>`fn visit_alternation_in(&mut self) -> fmt::Result`

- <span id="writer-visit-class-set-item-pre"></span>`fn visit_class_set_item_pre(&mut self, ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>` — [`ClassSetItem`](../index.md), [`Visitor`](../visitor/index.md)

- <span id="writer-visit-class-set-item-post"></span>`fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>` — [`ClassSetItem`](../index.md), [`Visitor`](../visitor/index.md)

- <span id="writer-visit-class-set-binary-op-in"></span>`fn visit_class_set_binary_op_in(&mut self, ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>` — [`ClassSetBinaryOp`](../index.md), [`Visitor`](../visitor/index.md)

