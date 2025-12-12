*[regex_syntax](../../index.md) / [hir](../index.md) / [translate](index.md)*

---

# Module `translate`

Defines a translator that converts an `Ast` to an `Hir`.

## Contents

- [Structs](#structs)
  - [`TranslatorBuilder`](#translatorbuilder)
  - [`Translator`](#translator)
  - [`TranslatorI`](#translatori)
  - [`Flags`](#flags)
- [Enums](#enums)
  - [`HirFrame`](#hirframe)
- [Functions](#functions)
  - [`hir_ascii_class_bytes`](#hir-ascii-class-bytes)
  - [`ascii_class`](#ascii-class)
  - [`ascii_class_as_chars`](#ascii-class-as-chars)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TranslatorBuilder`](#translatorbuilder) | struct | A builder for constructing an AST->HIR translator. |
| [`Translator`](#translator) | struct | A translator maps abstract syntax to a high level intermediate representation. |
| [`TranslatorI`](#translatori) | struct | The internal implementation of a translator. |
| [`Flags`](#flags) | struct | A translator's representation of a regular expression's flags at any given moment in time. |
| [`HirFrame`](#hirframe) | enum | An HirFrame is a single stack frame, represented explicitly, which is created for each item in the Ast that we traverse. |
| [`hir_ascii_class_bytes`](#hir-ascii-class-bytes) | fn |  |
| [`ascii_class`](#ascii-class) | fn |  |
| [`ascii_class_as_chars`](#ascii-class-as-chars) | fn |  |
| [`Result`](#result) | type |  |

## Structs

### `TranslatorBuilder`

```rust
struct TranslatorBuilder {
    utf8: bool,
    line_terminator: u8,
    flags: Flags,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:20-24`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L20-L24)*

A builder for constructing an AST->HIR translator.

#### Implementations

- <span id="translatorbuilder-new"></span>`fn new() -> TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-build"></span>`fn build(&self) -> Translator` — [`Translator`](#translator)

- <span id="translatorbuilder-utf8"></span>`fn utf8(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

- <span id="translatorbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

#### Trait Implementations

##### `impl Clone for TranslatorBuilder`

- <span id="translatorbuilder-clone"></span>`fn clone(&self) -> TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

##### `impl Debug for TranslatorBuilder`

- <span id="translatorbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TranslatorBuilder`

- <span id="translatorbuilder-default"></span>`fn default() -> TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

### `Translator`

```rust
struct Translator {
    stack: core::cell::RefCell<alloc::vec::Vec<HirFrame>>,
    flags: core::cell::Cell<Flags>,
    utf8: bool,
    line_terminator: u8,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:147-156`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L147-L156)*

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

- <span id="translator-new"></span>`fn new() -> Translator` — [`Translator`](#translator)

- <span id="translator-translate"></span>`fn translate(&mut self, pattern: &str, ast: &Ast) -> core::result::Result<Hir, crate::hir::Error>` — [`Ast`](../../ast/index.md#ast), [`Hir`](../index.md#hir), [`Error`](../index.md#error)

#### Trait Implementations

##### `impl Clone for Translator`

- <span id="translator-clone"></span>`fn clone(&self) -> Translator` — [`Translator`](#translator)

##### `impl Debug for Translator`

- <span id="translator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TranslatorI<'t, 'p>`

```rust
struct TranslatorI<'t, 'p> {
    trans: &'t Translator,
    pattern: &'p str,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:674-677`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L674-L677)*

The internal implementation of a translator.

This type is responsible for carrying around the original pattern string,
which is not tied to the internal state of a translator.

A TranslatorI exists for the time it takes to translate a single Ast.

#### Implementations

- <span id="translatori-new"></span>`fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p>` — [`Translator`](#translator), [`TranslatorI`](#translatori)

- <span id="translatori-trans"></span>`fn trans(&self) -> &Translator` — [`Translator`](#translator)

- <span id="translatori-push"></span>`fn push(&self, frame: HirFrame)` — [`HirFrame`](#hirframe)

- <span id="translatori-push-char"></span>`fn push_char(&self, ch: char)`

- <span id="translatori-push-byte"></span>`fn push_byte(&self, byte: u8)`

- <span id="translatori-pop"></span>`fn pop(&self) -> Option<HirFrame>` — [`HirFrame`](#hirframe)

- <span id="translatori-pop-concat-expr"></span>`fn pop_concat_expr(&self) -> Option<Hir>` — [`Hir`](../index.md#hir)

- <span id="translatori-pop-alt-expr"></span>`fn pop_alt_expr(&self) -> Option<Hir>` — [`Hir`](../index.md#hir)

- <span id="translatori-error"></span>`fn error(&self, span: Span, kind: ErrorKind) -> Error` — [`Span`](../../ast/index.md#span), [`ErrorKind`](../index.md#errorkind), [`Error`](../index.md#error)

- <span id="translatori-flags"></span>`fn flags(&self) -> Flags` — [`Flags`](#flags)

- <span id="translatori-set-flags"></span>`fn set_flags(&self, ast_flags: &ast::Flags) -> Flags` — [`Flags`](../../ast/index.md#flags)

- <span id="translatori-ast-literal-to-scalar"></span>`fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> core::result::Result<Either<char, u8>, crate::hir::Error>` — [`Literal`](../../ast/index.md#literal), [`Either`](../../either/index.md#either), [`Error`](../index.md#error)

- <span id="translatori-case-fold-char"></span>`fn case_fold_char(&self, span: Span, c: char) -> core::result::Result<Option<Hir>, crate::hir::Error>` — [`Span`](../../ast/index.md#span), [`Hir`](../index.md#hir), [`Error`](../index.md#error)

- <span id="translatori-hir-dot"></span>`fn hir_dot(&self, span: Span) -> core::result::Result<Hir, crate::hir::Error>` — [`Span`](../../ast/index.md#span), [`Hir`](../index.md#hir), [`Error`](../index.md#error)

- <span id="translatori-hir-assertion"></span>`fn hir_assertion(&self, asst: &ast::Assertion) -> core::result::Result<Hir, crate::hir::Error>` — [`Assertion`](../../ast/index.md#assertion), [`Hir`](../index.md#hir), [`Error`](../index.md#error)

- <span id="translatori-hir-capture"></span>`fn hir_capture(&self, group: &ast::Group, expr: Hir) -> Hir` — [`Group`](../../ast/index.md#group), [`Hir`](../index.md#hir)

- <span id="translatori-hir-repetition"></span>`fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir` — [`Repetition`](../../ast/index.md#repetition), [`Hir`](../index.md#hir)

- <span id="translatori-hir-unicode-class"></span>`fn hir_unicode_class(&self, ast_class: &ast::ClassUnicode) -> core::result::Result<hir::ClassUnicode, crate::hir::Error>` — [`ClassUnicode`](../../ast/index.md#classunicode), [`Error`](../index.md#error)

- <span id="translatori-hir-ascii-unicode-class"></span>`fn hir_ascii_unicode_class(&self, ast: &ast::ClassAscii) -> core::result::Result<hir::ClassUnicode, crate::hir::Error>` — [`ClassAscii`](../../ast/index.md#classascii), [`ClassUnicode`](../index.md#classunicode), [`Error`](../index.md#error)

- <span id="translatori-hir-ascii-byte-class"></span>`fn hir_ascii_byte_class(&self, ast: &ast::ClassAscii) -> core::result::Result<hir::ClassBytes, crate::hir::Error>` — [`ClassAscii`](../../ast/index.md#classascii), [`ClassBytes`](../index.md#classbytes), [`Error`](../index.md#error)

- <span id="translatori-hir-perl-unicode-class"></span>`fn hir_perl_unicode_class(&self, ast_class: &ast::ClassPerl) -> core::result::Result<hir::ClassUnicode, crate::hir::Error>` — [`ClassPerl`](../../ast/index.md#classperl), [`ClassUnicode`](../index.md#classunicode), [`Error`](../index.md#error)

- <span id="translatori-hir-perl-byte-class"></span>`fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> core::result::Result<hir::ClassBytes, crate::hir::Error>` — [`ClassPerl`](../../ast/index.md#classperl), [`ClassBytes`](../index.md#classbytes), [`Error`](../index.md#error)

- <span id="translatori-convert-unicode-class-error"></span>`fn convert_unicode_class_error(&self, span: &Span, result: core::result::Result<hir::ClassUnicode, unicode::Error>) -> core::result::Result<hir::ClassUnicode, crate::hir::Error>` — [`Span`](../../ast/index.md#span), [`ClassUnicode`](../index.md#classunicode), [`Error`](../../unicode/index.md#error)

- <span id="translatori-unicode-fold-and-negate"></span>`fn unicode_fold_and_negate(&self, span: &Span, negated: bool, class: &mut hir::ClassUnicode) -> core::result::Result<(), crate::hir::Error>` — [`Span`](../../ast/index.md#span), [`ClassUnicode`](../index.md#classunicode), [`Error`](../index.md#error)

- <span id="translatori-bytes-fold-and-negate"></span>`fn bytes_fold_and_negate(&self, span: &Span, negated: bool, class: &mut hir::ClassBytes) -> core::result::Result<(), crate::hir::Error>` — [`Span`](../../ast/index.md#span), [`ClassBytes`](../index.md#classbytes), [`Error`](../index.md#error)

- <span id="translatori-class-literal-byte"></span>`fn class_literal_byte(&self, ast: &ast::Literal) -> core::result::Result<u8, crate::hir::Error>` — [`Literal`](../../ast/index.md#literal), [`Error`](../index.md#error)

#### Trait Implementations

##### `impl Clone for TranslatorI<'t, 'p>`

- <span id="translatori-clone"></span>`fn clone(&self) -> TranslatorI<'t, 'p>` — [`TranslatorI`](#translatori)

##### `impl Debug for TranslatorI<'t, 'p>`

- <span id="translatori-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for TranslatorI<'t, 'p>`

- <span id="translatori-visitor-type-output"></span>`type Output = Hir`

- <span id="translatori-visitor-type-err"></span>`type Err = Error`

- <span id="translatori-finish"></span>`fn finish(self) -> core::result::Result<Hir, crate::hir::Error>` — [`Hir`](../index.md#hir), [`Error`](../index.md#error)

- <span id="translatori-visit-pre"></span>`fn visit_pre(&mut self, ast: &Ast) -> core::result::Result<(), crate::hir::Error>` — [`Ast`](../../ast/index.md#ast), [`Error`](../index.md#error)

- <span id="translatori-visit-post"></span>`fn visit_post(&mut self, ast: &Ast) -> core::result::Result<(), crate::hir::Error>` — [`Ast`](../../ast/index.md#ast), [`Error`](../index.md#error)

- <span id="translatori-visit-alternation-in"></span>`fn visit_alternation_in(&mut self) -> core::result::Result<(), crate::hir::Error>` — [`Error`](../index.md#error)

- <span id="translatori-visit-class-set-item-pre"></span>`fn visit_class_set_item_pre(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetItem`](../../ast/index.md#classsetitem), [`Error`](../index.md#error)

- <span id="translatori-visit-class-set-item-post"></span>`fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetItem`](../../ast/index.md#classsetitem), [`Error`](../index.md#error)

- <span id="translatori-visit-class-set-binary-op-pre"></span>`fn visit_class_set_binary_op_pre(&mut self, _op: &ast::ClassSetBinaryOp) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetBinaryOp`](../../ast/index.md#classsetbinaryop), [`Error`](../index.md#error)

- <span id="translatori-visit-class-set-binary-op-in"></span>`fn visit_class_set_binary_op_in(&mut self, _op: &ast::ClassSetBinaryOp) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetBinaryOp`](../../ast/index.md#classsetbinaryop), [`Error`](../index.md#error)

- <span id="translatori-visit-class-set-binary-op-post"></span>`fn visit_class_set_binary_op_post(&mut self, op: &ast::ClassSetBinaryOp) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetBinaryOp`](../../ast/index.md#classsetbinaryop), [`Error`](../index.md#error)

### `Flags`

```rust
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
    crlf: Option<bool>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1222-1231`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L1222-L1231)*

A translator's representation of a regular expression's flags at any given
moment in time.

Each flag can be in one of three states: absent, present but disabled or
present but enabled.

#### Implementations

- <span id="flags-from-ast"></span>`fn from_ast(ast: &ast::Flags) -> Flags` — [`Flags`](../../ast/index.md#flags)

- <span id="flags-merge"></span>`fn merge(&mut self, previous: &Flags)` — [`Flags`](#flags)

- <span id="flags-case-insensitive"></span>`fn case_insensitive(&self) -> bool`

- <span id="flags-multi-line"></span>`fn multi_line(&self) -> bool`

- <span id="flags-dot-matches-new-line"></span>`fn dot_matches_new_line(&self) -> bool`

- <span id="flags-swap-greed"></span>`fn swap_greed(&self) -> bool`

- <span id="flags-unicode"></span>`fn unicode(&self) -> bool`

- <span id="flags-crlf"></span>`fn crlf(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Flags`

- <span id="flags-clone"></span>`fn clone(&self) -> Flags` — [`Flags`](#flags)

##### `impl Copy for Flags`

##### `impl Debug for Flags`

- <span id="flags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Flags`

- <span id="flags-default"></span>`fn default() -> Flags` — [`Flags`](#flags)

## Enums

### `HirFrame`

```rust
enum HirFrame {
    Expr(crate::hir::Hir),
    Literal(alloc::vec::Vec<u8>),
    ClassUnicode(hir::ClassUnicode),
    ClassBytes(hir::ClassBytes),
    Repetition,
    Group {
        old_flags: Flags,
    },
    Concat,
    Alternation,
    AlternationBranch,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:185-249`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L185-L249)*

An HirFrame is a single stack frame, represented explicitly, which is
created for each item in the Ast that we traverse.

Note that technically, this type doesn't represent our entire stack
frame. In particular, the Ast visitor represents any state associated with
traversing the Ast itself.

#### Variants

- **`Expr`**

  An arbitrary HIR expression. These get pushed whenever we hit a base
  case in the Ast. They get popped after an inductive (i.e., recursive)
  step is complete.

- **`Literal`**

  A literal that is being constructed, character by character, from the
  AST. We need this because the AST gives each individual character its
  own node. So as we see characters, we peek at the top-most HirFrame.
  If it's a literal, then we add to it. Otherwise, we push a new literal.
  When it comes time to pop it, we convert it to an Hir via Hir::literal.

- **`ClassUnicode`**

  A Unicode character class. This frame is mutated as we descend into
  the Ast of a character class (which is itself its own mini recursive
  structure).

- **`ClassBytes`**

  A byte-oriented character class. This frame is mutated as we descend
  into the Ast of a character class (which is itself its own mini
  recursive structure).
  
  Byte character classes are created when Unicode mode (`u`) is disabled.
  If `utf8` is enabled (the default), then a byte character is only
  permitted to match ASCII text.

- **`Repetition`**

  This is pushed whenever a repetition is observed. After visiting every
  sub-expression in the repetition, the translator's stack is expected to
  have this sentinel at the top.
  
  This sentinel only exists to stop other things (like flattening
  literals) from reaching across repetition operators.

- **`Group`**

  This is pushed on to the stack upon first seeing any kind of capture,
  indicated by parentheses (including non-capturing groups). It is popped
  upon leaving a group.

- **`Concat`**

  This is pushed whenever a concatenation is observed. After visiting
  every sub-expression in the concatenation, the translator's stack is
  popped until it sees a Concat frame.

- **`Alternation`**

  This is pushed whenever an alternation is observed. After visiting
  every sub-expression in the alternation, the translator's stack is
  popped until it sees an Alternation frame.

- **`AlternationBranch`**

  This is pushed immediately before each sub-expression in an
  alternation. This separates the branches of an alternation on the
  stack and prevents literal flattening from reaching across alternation
  branches.
  
  It is popped after each expression in a branch until an 'Alternation'
  frame is observed when doing a post visit on an alternation.

#### Implementations

- <span id="hirframe-unwrap-expr"></span>`fn unwrap_expr(self) -> Hir` — [`Hir`](../index.md#hir)

- <span id="hirframe-unwrap-class-unicode"></span>`fn unwrap_class_unicode(self) -> hir::ClassUnicode` — [`ClassUnicode`](../index.md#classunicode)

- <span id="hirframe-unwrap-class-bytes"></span>`fn unwrap_class_bytes(self) -> hir::ClassBytes` — [`ClassBytes`](../index.md#classbytes)

- <span id="hirframe-unwrap-repetition"></span>`fn unwrap_repetition(self)`

- <span id="hirframe-unwrap-group"></span>`fn unwrap_group(self) -> Flags` — [`Flags`](#flags)

- <span id="hirframe-unwrap-alternation-pipe"></span>`fn unwrap_alternation_pipe(self)`

#### Trait Implementations

##### `impl Clone for HirFrame`

- <span id="hirframe-clone"></span>`fn clone(&self) -> HirFrame` — [`HirFrame`](#hirframe)

##### `impl Debug for HirFrame`

- <span id="hirframe-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `hir_ascii_class_bytes`

```rust
fn hir_ascii_class_bytes(kind: &ast::ClassAsciiKind) -> hir::ClassBytes
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1312-1317`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L1312-L1317)*

### `ascii_class`

```rust
fn ascii_class(kind: &ast::ClassAsciiKind) -> impl Iterator<Item = (u8, u8)>
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1319-1346`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L1319-L1346)*

### `ascii_class_as_chars`

```rust
fn ascii_class_as_chars(kind: &ast::ClassAsciiKind) -> impl Iterator<Item = (char, char)>
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1348-1352`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L1348-L1352)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, crate::hir::Error>;
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:16`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/translate.rs#L16)*

