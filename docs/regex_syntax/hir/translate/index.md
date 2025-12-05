*[regex_syntax](../../index.md) / [hir](../index.md) / [translate](index.md)*

---

# Module `translate`

Defines a translator that converts an `Ast` to an `Hir`.

## Structs

### `TranslatorBuilder`

```rust
struct TranslatorBuilder {
    utf8: bool,
    line_terminator: u8,
    flags: Flags,
}
```

A builder for constructing an AST->HIR translator.

#### Implementations

- `fn new() -> TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn build(self: &Self) -> Translator` — [`Translator`](../../../hir/translate/index.md)

- `fn utf8(self: &mut Self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn line_terminator(self: &mut Self, byte: u8) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn case_insensitive(self: &mut Self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn multi_line(self: &mut Self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn crlf(self: &mut Self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn swap_greed(self: &mut Self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

- `fn unicode(self: &mut Self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> TranslatorBuilder` — [`TranslatorBuilder`](../../../hir/translate/index.md)

### `Translator`

```rust
struct Translator {
    stack: core::cell::RefCell<alloc::vec::Vec<HirFrame>>,
    flags: core::cell::Cell<Flags>,
    utf8: bool,
    line_terminator: u8,
}
```

A translator maps abstract syntax to a high level intermediate
representation.

A translator may be benefit from reuse. That is, a translator can translate
many abstract syntax trees.

A `Translator` can be configured in more detail via a
[`TranslatorBuilder`](#translatorbuilder).

#### Fields

- **`stack`**: `core::cell::RefCell<alloc::vec::Vec<HirFrame>>`

  Our call stack, but on the heap.

- **`flags`**: `core::cell::Cell<Flags>`

  The current flag settings.

- **`utf8`**: `bool`

  Whether we're allowed to produce HIR that can match arbitrary bytes.

- **`line_terminator`**: `u8`

  The line terminator to use for `.`.

#### Implementations

- `fn new() -> Translator` — [`Translator`](../../../hir/translate/index.md)

- `fn translate(self: &mut Self, pattern: &str, ast: &Ast) -> core::result::Result<Hir, crate::hir::Error>` — [`Ast`](../../../ast/index.md), [`Hir`](../../../hir/index.md), [`Error`](../../../hir/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Translator` — [`Translator`](../../../hir/translate/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

