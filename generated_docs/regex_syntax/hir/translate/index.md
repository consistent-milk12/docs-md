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

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:20-24`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L20-L24)*

A builder for constructing an AST->HIR translator.

#### Implementations

- <span id="translatorbuilder-new"></span>`fn new() -> TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Create a new translator builder with a default configuration.

- <span id="translatorbuilder-build"></span>`fn build(&self) -> Translator` — [`Translator`](#translator)

  Build a translator using the current configuration.

- <span id="translatorbuilder-utf8"></span>`fn utf8(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  When disabled, translation will permit the construction of a regular

  expression that may match invalid UTF-8.

  

  When enabled (the default), the translator is guaranteed to produce an

  expression that, for non-empty matches, will only ever produce spans

  that are entirely valid UTF-8 (otherwise, the translator will return an

  error).

  

  Perhaps surprisingly, when UTF-8 is enabled, an empty regex or even

  a negated ASCII word boundary (uttered as `(?-u:\B)` in the concrete

  syntax) will be allowed even though they can produce matches that split

  a UTF-8 encoded codepoint. This only applies to zero-width or "empty"

  matches, and it is expected that the regex engine itself must handle

  these cases if necessary (perhaps by suppressing any zero-width matches

  that split a codepoint).

- <span id="translatorbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Sets the line terminator for use with `(?u-s:.)` and `(?-us:.)`.

  

  Namely, instead of `.` (by default) matching everything except for `\n`,

  this will cause `.` to match everything except for the byte given.

  

  If `.` is used in a context where Unicode mode is enabled and this byte

  isn't ASCII, then an error will be returned. When Unicode mode is

  disabled, then any byte is permitted, but will return an error if UTF-8

  mode is enabled and it is a non-ASCII byte.

  

  In short, any ASCII value for a line terminator is always okay. But a

  non-ASCII byte might result in an error depending on whether Unicode

  mode or UTF-8 mode are enabled.

  

  Note that if `R` mode is enabled then it always takes precedence and

  the line terminator will be treated as `\r` and `\n` simultaneously.

  

  Note also that this *doesn't* impact the look-around assertions

  `(?m:^)` and `(?m:$)`. That's usually controlled by additional

  configuration in the regex engine itself.

- <span id="translatorbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Enable or disable the case insensitive flag (`i`) by default.

- <span id="translatorbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Enable or disable the multi-line matching flag (`m`) by default.

- <span id="translatorbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Enable or disable the "dot matches any character" flag (`s`) by

  default.

- <span id="translatorbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Enable or disable the CRLF mode flag (`R`) by default.

- <span id="translatorbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Enable or disable the "swap greed" flag (`U`) by default.

- <span id="translatorbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

  Enable or disable the Unicode flag (`u`) by default.

#### Trait Implementations

##### `impl Any for TranslatorBuilder`

- <span id="translatorbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TranslatorBuilder`

- <span id="translatorbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TranslatorBuilder`

- <span id="translatorbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TranslatorBuilder`

- <span id="translatorbuilder-clone"></span>`fn clone(&self) -> TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

##### `impl CloneToUninit for TranslatorBuilder`

- <span id="translatorbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TranslatorBuilder`

- <span id="translatorbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TranslatorBuilder`

- <span id="translatorbuilder-default"></span>`fn default() -> TranslatorBuilder` — [`TranslatorBuilder`](#translatorbuilder)

##### `impl<T> From for TranslatorBuilder`

- <span id="translatorbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TranslatorBuilder`

- <span id="translatorbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for TranslatorBuilder`

- <span id="translatorbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="translatorbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="translatorbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TranslatorBuilder`

- <span id="translatorbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="translatorbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TranslatorBuilder`

- <span id="translatorbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="translatorbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Translator`

```rust
struct Translator {
    stack: core::cell::RefCell<alloc::vec::Vec<HirFrame>>,
    flags: core::cell::Cell<Flags>,
    utf8: bool,
    line_terminator: u8,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:147-156`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L147-L156)*

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

  Create a new translator using the default configuration.

- <span id="translator-translate"></span>`fn translate(&mut self, pattern: &str, ast: &Ast) -> core::result::Result<Hir, crate::hir::Error>` — [`Ast`](../../ast/index.md#ast), [`Hir`](../index.md#hir), [`Error`](../index.md#error)

  Translate the given abstract syntax tree (AST) into a high level

  intermediate representation (HIR).

  

  If there was a problem doing the translation, then an HIR-specific

  error is returned.

  

  The original pattern string used to produce the `Ast` *must* also be

  provided. The translator does not use the pattern string during any

  correct translation, but is used for error reporting.

#### Trait Implementations

##### `impl Any for Translator`

- <span id="translator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Translator`

- <span id="translator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Translator`

- <span id="translator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Translator`

- <span id="translator-clone"></span>`fn clone(&self) -> Translator` — [`Translator`](#translator)

##### `impl CloneToUninit for Translator`

- <span id="translator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Translator`

- <span id="translator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Translator`

- <span id="translator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Translator`

- <span id="translator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Translator`

- <span id="translator-toowned-type-owned"></span>`type Owned = T`

- <span id="translator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="translator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Translator`

- <span id="translator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="translator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Translator`

- <span id="translator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="translator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TranslatorI<'t, 'p>`

```rust
struct TranslatorI<'t, 'p> {
    trans: &'t Translator,
    pattern: &'p str,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:674-677`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L674-L677)*

The internal implementation of a translator.

This type is responsible for carrying around the original pattern string,
which is not tied to the internal state of a translator.

A TranslatorI exists for the time it takes to translate a single Ast.

#### Implementations

- <span id="translatori-new"></span>`fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p>` — [`Translator`](#translator), [`TranslatorI`](#translatori)

  Build a new internal translator.

- <span id="translatori-trans"></span>`fn trans(&self) -> &Translator` — [`Translator`](#translator)

  Return a reference to the underlying translator.

- <span id="translatori-push"></span>`fn push(&self, frame: HirFrame)` — [`HirFrame`](#hirframe)

  Push the given frame on to the call stack.

- <span id="translatori-push-char"></span>`fn push_char(&self, ch: char)`

  Push the given literal char on to the call stack.

  

  If the top-most element of the stack is a literal, then the char

  is appended to the end of that literal. Otherwise, a new literal

  containing just the given char is pushed to the top of the stack.

- <span id="translatori-push-byte"></span>`fn push_byte(&self, byte: u8)`

  Push the given literal byte on to the call stack.

  

  If the top-most element of the stack is a literal, then the byte

  is appended to the end of that literal. Otherwise, a new literal

  containing just the given byte is pushed to the top of the stack.

- <span id="translatori-pop"></span>`fn pop(&self) -> Option<HirFrame>` — [`HirFrame`](#hirframe)

  Pop the top of the call stack. If the call stack is empty, return None.

- <span id="translatori-pop-concat-expr"></span>`fn pop_concat_expr(&self) -> Option<Hir>` — [`Hir`](../index.md#hir)

  Pop an HIR expression from the top of the stack for a concatenation.

  

  This returns None if the stack is empty or when a concat frame is seen.

  Otherwise, it panics if it could not find an HIR expression.

- <span id="translatori-pop-alt-expr"></span>`fn pop_alt_expr(&self) -> Option<Hir>` — [`Hir`](../index.md#hir)

  Pop an HIR expression from the top of the stack for an alternation.

  

  This returns None if the stack is empty or when an alternation frame is

  seen. Otherwise, it panics if it could not find an HIR expression.

- <span id="translatori-error"></span>`fn error(&self, span: Span, kind: ErrorKind) -> Error` — [`Span`](../../ast/index.md#span), [`ErrorKind`](../index.md#errorkind), [`Error`](../index.md#error)

  Create a new error with the given span and error type.

- <span id="translatori-flags"></span>`fn flags(&self) -> Flags` — [`Flags`](#flags)

  Return a copy of the active flags.

- <span id="translatori-set-flags"></span>`fn set_flags(&self, ast_flags: &ast::Flags) -> Flags` — [`Flags`](../../ast/index.md#flags)

  Set the flags of this translator from the flags set in the given AST.

  Then, return the old flags.

- <span id="translatori-ast-literal-to-scalar"></span>`fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> core::result::Result<Either<char, u8>, crate::hir::Error>` — [`Literal`](../../ast/index.md#literal), [`Either`](../../either/index.md#either), [`Error`](../index.md#error)

  Convert an Ast literal to its scalar representation.

  

  When Unicode mode is enabled, then this always succeeds and returns a

  `char` (Unicode scalar value).

  

  When Unicode mode is disabled, then a `char` will still be returned

  whenever possible. A byte is returned only when invalid UTF-8 is

  allowed and when the byte is not ASCII. Otherwise, a non-ASCII byte

  will result in an error when invalid UTF-8 is not allowed.

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

  Converts the given Unicode specific error to an HIR translation error.

  

  The span given should approximate the position at which an error would

  occur.

- <span id="translatori-unicode-fold-and-negate"></span>`fn unicode_fold_and_negate(&self, span: &Span, negated: bool, class: &mut hir::ClassUnicode) -> core::result::Result<(), crate::hir::Error>` — [`Span`](../../ast/index.md#span), [`ClassUnicode`](../index.md#classunicode), [`Error`](../index.md#error)

- <span id="translatori-bytes-fold-and-negate"></span>`fn bytes_fold_and_negate(&self, span: &Span, negated: bool, class: &mut hir::ClassBytes) -> core::result::Result<(), crate::hir::Error>` — [`Span`](../../ast/index.md#span), [`ClassBytes`](../index.md#classbytes), [`Error`](../index.md#error)

- <span id="translatori-class-literal-byte"></span>`fn class_literal_byte(&self, ast: &ast::Literal) -> core::result::Result<u8, crate::hir::Error>` — [`Literal`](../../ast/index.md#literal), [`Error`](../index.md#error)

  Return a scalar byte value suitable for use as a literal in a byte

  character class.

#### Trait Implementations

##### `impl Any for TranslatorI<'t, 'p>`

- <span id="translatori-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TranslatorI<'t, 'p>`

- <span id="translatori-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TranslatorI<'t, 'p>`

- <span id="translatori-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TranslatorI<'t, 'p>`

- <span id="translatori-clone"></span>`fn clone(&self) -> TranslatorI<'t, 'p>` — [`TranslatorI`](#translatori)

##### `impl CloneToUninit for TranslatorI<'t, 'p>`

- <span id="translatori-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TranslatorI<'t, 'p>`

- <span id="translatori-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TranslatorI<'t, 'p>`

- <span id="translatori-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TranslatorI<'t, 'p>`

- <span id="translatori-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for TranslatorI<'t, 'p>`

- <span id="translatori-toowned-type-owned"></span>`type Owned = T`

- <span id="translatori-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="translatori-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TranslatorI<'t, 'p>`

- <span id="translatori-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="translatori-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TranslatorI<'t, 'p>`

- <span id="translatori-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="translatori-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for TranslatorI<'t, 'p>`

- <span id="translatori-visitor-type-output"></span>`type Output = Hir`

- <span id="translatori-visitor-type-err"></span>`type Err = Error`

- <span id="translatori-visitor-finish"></span>`fn finish(self) -> core::result::Result<Hir, crate::hir::Error>` — [`Hir`](../index.md#hir), [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-pre"></span>`fn visit_pre(&mut self, ast: &Ast) -> core::result::Result<(), crate::hir::Error>` — [`Ast`](../../ast/index.md#ast), [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-post"></span>`fn visit_post(&mut self, ast: &Ast) -> core::result::Result<(), crate::hir::Error>` — [`Ast`](../../ast/index.md#ast), [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-alternation-in"></span>`fn visit_alternation_in(&mut self) -> core::result::Result<(), crate::hir::Error>` — [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-class-set-item-pre"></span>`fn visit_class_set_item_pre(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetItem`](../../ast/index.md#classsetitem), [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-class-set-item-post"></span>`fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetItem`](../../ast/index.md#classsetitem), [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-class-set-binary-op-pre"></span>`fn visit_class_set_binary_op_pre(&mut self, _op: &ast::ClassSetBinaryOp) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetBinaryOp`](../../ast/index.md#classsetbinaryop), [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-class-set-binary-op-in"></span>`fn visit_class_set_binary_op_in(&mut self, _op: &ast::ClassSetBinaryOp) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetBinaryOp`](../../ast/index.md#classsetbinaryop), [`Error`](../index.md#error)

- <span id="translatori-visitor-visit-class-set-binary-op-post"></span>`fn visit_class_set_binary_op_post(&mut self, op: &ast::ClassSetBinaryOp) -> core::result::Result<(), crate::hir::Error>` — [`ClassSetBinaryOp`](../../ast/index.md#classsetbinaryop), [`Error`](../index.md#error)

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

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1222-1231`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L1222-L1231)*

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

##### `impl Any for Flags`

- <span id="flags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flags`

- <span id="flags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flags`

- <span id="flags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Flags`

- <span id="flags-clone"></span>`fn clone(&self) -> Flags` — [`Flags`](#flags)

##### `impl CloneToUninit for Flags`

- <span id="flags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Flags`

##### `impl Debug for Flags`

- <span id="flags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Flags`

- <span id="flags-default"></span>`fn default() -> Flags` — [`Flags`](#flags)

##### `impl<T> From for Flags`

- <span id="flags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flags`

- <span id="flags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Flags`

- <span id="flags-toowned-type-owned"></span>`type Owned = T`

- <span id="flags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Flags`

- <span id="flags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flags`

- <span id="flags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:185-249`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L185-L249)*

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

  Assert that the current stack frame is an Hir expression and return it.

- <span id="hirframe-unwrap-class-unicode"></span>`fn unwrap_class_unicode(self) -> hir::ClassUnicode` — [`ClassUnicode`](../index.md#classunicode)

  Assert that the current stack frame is a Unicode class expression and

  return it.

- <span id="hirframe-unwrap-class-bytes"></span>`fn unwrap_class_bytes(self) -> hir::ClassBytes` — [`ClassBytes`](../index.md#classbytes)

  Assert that the current stack frame is a byte class expression and

  return it.

- <span id="hirframe-unwrap-repetition"></span>`fn unwrap_repetition(self)`

  Assert that the current stack frame is a repetition sentinel. If it

  isn't, then panic.

- <span id="hirframe-unwrap-group"></span>`fn unwrap_group(self) -> Flags` — [`Flags`](#flags)

  Assert that the current stack frame is a group indicator and return

  its corresponding flags (the flags that were active at the time the

  group was entered).

- <span id="hirframe-unwrap-alternation-pipe"></span>`fn unwrap_alternation_pipe(self)`

  Assert that the current stack frame is an alternation pipe sentinel. If

  it isn't, then panic.

#### Trait Implementations

##### `impl Any for HirFrame`

- <span id="hirframe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HirFrame`

- <span id="hirframe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HirFrame`

- <span id="hirframe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for HirFrame`

- <span id="hirframe-clone"></span>`fn clone(&self) -> HirFrame` — [`HirFrame`](#hirframe)

##### `impl CloneToUninit for HirFrame`

- <span id="hirframe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for HirFrame`

- <span id="hirframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HirFrame`

- <span id="hirframe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HirFrame`

- <span id="hirframe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for HirFrame`

- <span id="hirframe-toowned-type-owned"></span>`type Owned = T`

- <span id="hirframe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hirframe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for HirFrame`

- <span id="hirframe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hirframe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HirFrame`

- <span id="hirframe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hirframe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `hir_ascii_class_bytes`

```rust
fn hir_ascii_class_bytes(kind: &ast::ClassAsciiKind) -> hir::ClassBytes
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1312-1317`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L1312-L1317)*

### `ascii_class`

```rust
fn ascii_class(kind: &ast::ClassAsciiKind) -> impl Iterator<Item = (u8, u8)>
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1319-1346`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L1319-L1346)*

### `ascii_class_as_chars`

```rust
fn ascii_class_as_chars(kind: &ast::ClassAsciiKind) -> impl Iterator<Item = (char, char)>
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:1348-1352`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L1348-L1352)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, crate::hir::Error>;
```

*Defined in [`regex-syntax-0.8.8/src/hir/translate.rs:16`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/translate.rs#L16)*

