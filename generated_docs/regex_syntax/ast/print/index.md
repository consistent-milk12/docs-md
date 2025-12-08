*[regex_syntax](../../index.md) / [ast](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Ast`.

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

A printer for a regular expression abstract syntax tree.

A printer converts an abstract syntax tree (AST) to a regular expression
pattern string. This particular printer uses constant stack space and heap
space proportional to the size of the AST.

This printer will not necessarily preserve the original formatting of the
regular expression pattern string. For example, all whitespace and comments
are ignored.

#### Implementations

- `fn new() -> Printer` — [`Printer`](#printer)

- `fn print<W: fmt::Write>(self: &mut Self, ast: &Ast, wtr: W) -> fmt::Result` — [`Ast`](../index.md)

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

- `fn fmt_group_pre(self: &mut Self, ast: &ast::Group) -> fmt::Result` — [`Group`](../index.md)

- `fn fmt_group_post(self: &mut Self, _ast: &ast::Group) -> fmt::Result` — [`Group`](../index.md)

- `fn fmt_repetition(self: &mut Self, ast: &ast::Repetition) -> fmt::Result` — [`Repetition`](../index.md)

- `fn fmt_repetition_range(self: &mut Self, ast: &ast::RepetitionRange) -> fmt::Result` — [`RepetitionRange`](../index.md)

- `fn fmt_literal(self: &mut Self, ast: &ast::Literal) -> fmt::Result` — [`Literal`](../index.md)

- `fn fmt_assertion(self: &mut Self, ast: &ast::Assertion) -> fmt::Result` — [`Assertion`](../index.md)

- `fn fmt_set_flags(self: &mut Self, ast: &ast::SetFlags) -> fmt::Result` — [`SetFlags`](../index.md)

- `fn fmt_flags(self: &mut Self, ast: &ast::Flags) -> fmt::Result` — [`Flags`](../index.md)

- `fn fmt_class_bracketed_pre(self: &mut Self, ast: &ast::ClassBracketed) -> fmt::Result` — [`ClassBracketed`](../index.md)

- `fn fmt_class_bracketed_post(self: &mut Self, _ast: &ast::ClassBracketed) -> fmt::Result` — [`ClassBracketed`](../index.md)

- `fn fmt_class_set_binary_op_kind(self: &mut Self, ast: &ast::ClassSetBinaryOpKind) -> fmt::Result` — [`ClassSetBinaryOpKind`](../index.md)

- `fn fmt_class_perl(self: &mut Self, ast: &ast::ClassPerl) -> fmt::Result` — [`ClassPerl`](../index.md)

- `fn fmt_class_ascii(self: &mut Self, ast: &ast::ClassAscii) -> fmt::Result` — [`ClassAscii`](../index.md)

- `fn fmt_class_unicode(self: &mut Self, ast: &ast::ClassUnicode) -> fmt::Result` — [`ClassUnicode`](../index.md)

#### Trait Implementations

##### `impl<W: $crate::fmt::Debug> Debug for Writer<W>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<W: fmt::Write> Visitor for Writer<W>`

- `type Output = ()`

- `type Err = Error`

- `fn finish(self: Self) -> fmt::Result`

- `fn visit_pre(self: &mut Self, ast: &Ast) -> fmt::Result` — [`Ast`](../index.md)

- `fn visit_post(self: &mut Self, ast: &Ast) -> fmt::Result` — [`Ast`](../index.md)

- `fn visit_alternation_in(self: &mut Self) -> fmt::Result`

- `fn visit_class_set_item_pre(self: &mut Self, ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>` — [`ClassSetItem`](../index.md), [`Visitor`](../index.md)

- `fn visit_class_set_item_post(self: &mut Self, ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>` — [`ClassSetItem`](../index.md), [`Visitor`](../index.md)

- `fn visit_class_set_binary_op_in(self: &mut Self, ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>` — [`ClassSetBinaryOp`](../index.md), [`Visitor`](../index.md)

