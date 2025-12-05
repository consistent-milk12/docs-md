*[regex_syntax](../../index.md) / [ast](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Ast`.

## Structs

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

- `fn new() -> Printer` — [`Printer`](../../../ast/print/index.md)

- `fn print<W: fmt::Write>(self: &mut Self, ast: &Ast, wtr: W) -> fmt::Result` — [`Ast`](../../../ast/index.md)

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

