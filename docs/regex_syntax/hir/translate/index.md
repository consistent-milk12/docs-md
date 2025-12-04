*[regex_syntax](../../index.md) / [hir](../index.md) / [translate](index.md)*

---

# Module `translate`

Defines a translator that converts an `Ast` to an `Hir`.

## Structs

### `TranslatorBuilder`

```rust
struct TranslatorBuilder {
    // [REDACTED: Private Fields]
}
```

A builder for constructing an AST->HIR translator.

#### Implementations

- `fn new() -> TranslatorBuilder`
  Create a new translator builder with a default configuration.

- `fn build(self: &Self) -> Translator`
  Build a translator using the current configuration.

- `fn utf8(self: &mut Self, yes: bool) -> &mut TranslatorBuilder`
  When disabled, translation will permit the construction of a regular

- `fn line_terminator(self: &mut Self, byte: u8) -> &mut TranslatorBuilder`
  Sets the line terminator for use with `(?u-s:.)` and `(?-us:.)`.

- `fn case_insensitive(self: &mut Self, yes: bool) -> &mut TranslatorBuilder`
  Enable or disable the case insensitive flag (`i`) by default.

- `fn multi_line(self: &mut Self, yes: bool) -> &mut TranslatorBuilder`
  Enable or disable the multi-line matching flag (`m`) by default.

- `fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut TranslatorBuilder`
  Enable or disable the "dot matches any character" flag (`s`) by

- `fn crlf(self: &mut Self, yes: bool) -> &mut TranslatorBuilder`
  Enable or disable the CRLF mode flag (`R`) by default.

- `fn swap_greed(self: &mut Self, yes: bool) -> &mut TranslatorBuilder`
  Enable or disable the "swap greed" flag (`U`) by default.

- `fn unicode(self: &mut Self, yes: bool) -> &mut TranslatorBuilder`
  Enable or disable the Unicode flag (`u`) by default.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> TranslatorBuilder`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> TranslatorBuilder`

### `Translator`

```rust
struct Translator {
    // [REDACTED: Private Fields]
}
```

A translator maps abstract syntax to a high level intermediate
representation.

A translator may be benefit from reuse. That is, a translator can translate
many abstract syntax trees.

A `Translator` can be configured in more detail via a
[`TranslatorBuilder`](regex_syntax/hir/translate/index.md).

#### Implementations

- `fn new() -> Translator`
  Create a new translator using the default configuration.

- `fn translate(self: &mut Self, pattern: &str, ast: &Ast) -> core::result::Result<Hir, crate::hir::Error>`
  Translate the given abstract syntax tree (AST) into a high level

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Translator`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

