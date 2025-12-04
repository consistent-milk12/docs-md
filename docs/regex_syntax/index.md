# Crate `regex_syntax`

This crate provides a robust regular expression parser.

This crate defines two primary types:

* [`Ast`](ast::Ast) is the abstract syntax of a regular expression.
  An abstract syntax corresponds to a *structured representation* of the
  concrete syntax of a regular expression, where the concrete syntax is the
  pattern string itself (e.g., `foo(bar)+`). Given some abstract syntax, it
  can be converted back to the original concrete syntax (modulo some details,
  like whitespace). To a first approximation, the abstract syntax is complex
  and difficult to analyze.
* [`Hir`](hir::Hir) is the high-level intermediate representation
  ("HIR" or "high-level IR" for short) of regular expression. It corresponds to
  an intermediate state of a regular expression that sits between the abstract
  syntax and the low level compiled opcodes that are eventually responsible for
  executing a regular expression search. Given some high-level IR, it is not
  possible to produce the original concrete syntax (although it is possible to
  produce an equivalent concrete syntax, but it will likely scarcely resemble
  the original pattern). To a first approximation, the high-level IR is simple
  and easy to analyze.

These two types come with conversion routines:

* An [`ast::parse::Parser`](#parser) converts concrete syntax (a `&str`) to an
[`Ast`](ast::Ast).
* A [`hir::translate::Translator`](#translator) converts an [`Ast`](ast::Ast) to a
[`Hir`](hir::Hir).

As a convenience, the above two conversion routines are combined into one via
the top-level [`Parser`](regex_syntax/ast/parse/index.md) type. This `Parser` will first convert your pattern to
an `Ast` and then convert the `Ast` to an `Hir`. It's also exposed as top-level
[`parse`](regex_syntax/ast/parse/index.md) free function.


# Example

This example shows how to parse a pattern string into its HIR:

```
use regex_syntax::{hir::Hir, parse};

let hir = parse("a|b")?;
assert_eq!(hir, Hir::alternation(vec![
    Hir::literal("a".as_bytes()),
    Hir::literal("b".as_bytes()),
]));
# Ok::<(), Box<dyn std::error::Error>>(())
```


# Concrete syntax supported

The concrete syntax is documented as part of the public API of the
[`regex` crate](https://docs.rs/regex/%2A/regex/#syntax).


# Input safety

A key feature of this library is that it is safe to use with end user facing
input. This plays a significant role in the internal implementation. In
particular:

1. Parsers provide a `nest_limit` option that permits callers to control how
   deeply nested a regular expression is allowed to be. This makes it possible
   to do case analysis over an `Ast` or an `Hir` using recursion without
   worrying about stack overflow.
2. Since relying on a particular stack size is brittle, this crate goes to
   great lengths to ensure that all interactions with both the `Ast` and the
   `Hir` do not use recursion. Namely, they use constant stack space and heap
   space proportional to the size of the original pattern string (in bytes).
   This includes the type's corresponding destructors. (One exception to this
   is literal extraction, but this will eventually get fixed.)


# Error reporting

The `Display` implementations on all `Error` types exposed in this library
provide nice human readable errors that are suitable for showing to end users
in a monospace font.


# Literal extraction

This crate provides limited support for [literal extraction from `Hir`
values](hir::literal). Be warned that literal extraction uses recursion, and
therefore, stack size proportional to the size of the `Hir`.

The purpose of literal extraction is to speed up searches. That is, if you
know a regular expression must match a prefix or suffix literal, then it is
often quicker to search for instances of that literal, and then confirm or deny
the match using the full regular expression engine. These optimizations are
done automatically in the `regex` crate.


# Crate features

An important feature provided by this crate is its Unicode support. This
includes things like case folding, boolean properties, general categories,
scripts and Unicode-aware support for the Perl classes `\w`, `\s` and `\d`.
However, a downside of this support is that it requires bundling several
Unicode data tables that are substantial in size.

A fair number of use cases do not require full Unicode support. For this
reason, this crate exposes a number of features to control which Unicode
data is available.

If a regular expression attempts to use a Unicode feature that is not available
because the corresponding crate feature was disabled, then translating that
regular expression to an `Hir` will return an error. (It is still possible
construct an `Ast` for such a regular expression, since Unicode data is not
used until translation to an `Hir`.) Stated differently, enabling or disabling
any of the features below can only add or subtract from the total set of valid
regular expressions. Enabling or disabling a feature will never modify the
match semantics of a regular expression.

The following features are available:

* **std** -
  Enables support for the standard library. This feature is enabled by default.
  When disabled, only `core` and `alloc` are used. Otherwise, enabling `std`
  generally just enables `std::error::Error` trait impls for the various error
  types.
* **unicode** -
  Enables all Unicode features. This feature is enabled by default, and will
  always cover all Unicode features, even if more are added in the future.
* **unicode-age** -
  Provide the data for the
  [Unicode `Age` property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
  This makes it possible to use classes like `\p{Age:6.0}` to refer to all
  codepoints first introduced in Unicode 6.0
* **unicode-bool** -
  Provide the data for numerous Unicode boolean properties. The full list
  is not included here, but contains properties like `Alphabetic`, `Emoji`,
  `Lowercase`, `Math`, `Uppercase` and `White_Space`.
* **unicode-case** -
  Provide the data for case insensitive matching using
  [Unicode's "simple loose matches" specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
* **unicode-gencat** -
  Provide the data for
  [Unicode general categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
  This includes, but is not limited to, `Decimal_Number`, `Letter`,
  `Math_Symbol`, `Number` and `Punctuation`.
* **unicode-perl** -
  Provide the data for supporting the Unicode-aware Perl character classes,
  corresponding to `\w`, `\s` and `\d`. This is also necessary for using
  Unicode-aware word boundary assertions. Note that if this feature is
  disabled, the `\s` and `\d` character classes are still available if the
  `unicode-bool` and `unicode-gencat` features are enabled, respectively.
* **unicode-script** -
  Provide the data for
  [Unicode scripts and script extensions](https://www.unicode.org/reports/tr24/).
  This includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
  `Latin` and `Thai`.
* **unicode-segment** -
  Provide the data necessary to provide the properties used to implement the
  [Unicode text segmentation algorithms](https://www.unicode.org/reports/tr29/).
  This enables using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
  `\p{sb=ATerm}`.
* **arbitrary** -
  Enabling this feature introduces a public dependency on the
  [`arbitrary`](https://crates.io/crates/arbitrary)
  crate. Namely, it implements the `Arbitrary` trait from that crate for the
  [`Ast`](crate::ast::Ast) type. This feature is disabled by default.

## Modules

- [`ast`](ast/index.md) - Defines an abstract syntax for regular expressions.
- [`hir`](hir/index.md) - Defines a high-level intermediate (HIR) representation for regular expressions.
- [`utf8`](utf8/index.md) - Converts ranges of Unicode scalar values to equivalent ranges of UTF-8 bytes.

## Structs

### `Parser`

```rust
struct Parser {
}
```

A convenience parser for regular expressions.

This parser takes as input a regular expression pattern string (the
"concrete syntax") and returns a high-level intermediate representation
(the HIR) suitable for most types of analysis. In particular, this parser
hides the intermediate state of producing an AST (the "abstract syntax").
The AST is itself far more complex than the HIR, so this parser serves as a
convenience for never having to deal with it at all.

If callers have more fine grained use cases that need an AST, then please
see the [`ast::parse`](#parse) module.

A `Parser` can be configured in more detail via a [`ParserBuilder`](regex_syntax/ast/parse/index.md).

#### Implementations

- `fn new() -> Parser`
  Create a new parser with a default configuration.

- `fn parse(self: &mut Self, pattern: &str) -> Result<hir::Hir, Error>`
  Parse the regular expression into a high level intermediate

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

- `fn clone(self: &Self) -> Parser`

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

### `ParserBuilder`

```rust
struct ParserBuilder {
}
```

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

This type combines the builder options for both the [AST
`ParserBuilder`](ast::parse::ParserBuilder) and the [HIR
`TranslatorBuilder`](hir::translate::TranslatorBuilder).

#### Implementations

- `fn new() -> ParserBuilder`
  Create a new parser builder with a default configuration.

- `fn build(self: &Self) -> Parser`
  Build a parser from this configuration with the given pattern.

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut ParserBuilder`
  Set the nesting limit for this parser.

- `fn octal(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Whether to support octal syntax or not.

- `fn utf8(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  When disabled, translation will permit the construction of a regular

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Enable verbose mode in the regular expression.

- `fn case_insensitive(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Enable or disable the case insensitive flag by default.

- `fn multi_line(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Enable or disable the multi-line matching flag by default.

- `fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Enable or disable the "dot matches any character" flag by default.

- `fn crlf(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Enable or disable the CRLF mode flag by default.

- `fn line_terminator(self: &mut Self, byte: u8) -> &mut ParserBuilder`
  Sets the line terminator for use with `(?u-s:.)` and `(?-us:.)`.

- `fn swap_greed(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Enable or disable the "swap greed" flag by default.

- `fn unicode(self: &mut Self, yes: bool) -> &mut ParserBuilder`
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

- `fn clone(self: &Self) -> ParserBuilder`

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

- `fn default() -> ParserBuilder`

### `UnicodeWordError`

```rust
struct UnicodeWordError();
```

An error that occurs when the Unicode-aware `\w` class is unavailable.

This error can occur when the data tables necessary for the Unicode aware
Perl character class `\w` are unavailable. This only occurs when the
`unicode-perl` feature is disabled. (The feature is enabled by default.)

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `Error`

```rust
enum Error {
    Parse(ast::Error),
    Translate(hir::Error),
}
```

This error type encompasses any error that can be returned by this crate.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

#### Variants

- **`Parse`**

  An error that occurred while translating concrete syntax into abstract
  syntax (AST).

- **`Translate`**

  An error that occurred while translating abstract syntax into a high
  level intermediate representation (HIR).

#### Trait Implementations

##### `impl From`

- `fn from(err: hir::Error) -> Error`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(err: ast::Error) -> Error`

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

- `fn clone(self: &Self) -> Error`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `escape`

```rust
fn escape(text: &str) -> alloc::string::String
```

Escapes all regular expression meta characters in `text`.

The string returned may be safely used as a literal in a regular
expression.

### `escape_into`

```rust
fn escape_into(text: &str, buf: &mut alloc::string::String)
```

Escapes all meta characters in `text` and writes the result into `buf`.

This will append escape characters into the given buffer. The characters
that are appended are safe to use as a literal in a regular expression.

### `is_meta_character`

```rust
fn is_meta_character(c: char) -> bool
```

Returns true if the given character has significance in a regex.

Generally speaking, these are the only characters which _must_ be escaped
in order to match their literal meaning. For example, to match a literal
`|`, one could write `\|`. Sometimes escaping isn't always necessary. For
example, `-` is treated as a meta character because of its significance
for writing ranges inside of character classes, but the regex `-` will
match a literal `-` because `-` has no special meaning outside of character
classes.

In order to determine whether a character may be escaped at all, the
[`is_escapeable_character`](regex_syntax/index.md) routine should be used. The difference between
`is_meta_character` and `is_escapeable_character` is that the latter will
return true for some characters that are _not_ meta characters. For
example, `%` and `\%` both match a literal `%` in all contexts. In other
words, `is_escapeable_character` includes "superfluous" escapes.

Note that the set of characters for which this function returns `true` or
`false` is fixed and won't change in a semver compatible release. (In this
case, "semver compatible release" actually refers to the `regex` crate
itself, since reducing or expanding the set of meta characters would be a
breaking change for not just `regex-syntax` but also `regex` itself.)

# Example

```
use regex_syntax::is_meta_character;

assert!(is_meta_character('?'));
assert!(is_meta_character('-'));
assert!(is_meta_character('&'));
assert!(is_meta_character('#'));

assert!(!is_meta_character('%'));
assert!(!is_meta_character('/'));
assert!(!is_meta_character('!'));
assert!(!is_meta_character('"'));
assert!(!is_meta_character('e'));
```

### `is_escapeable_character`

```rust
fn is_escapeable_character(c: char) -> bool
```

Returns true if the given character can be escaped in a regex.

This returns true in all cases that `is_meta_character` returns true, but
also returns true in some cases where `is_meta_character` returns false.
For example, `%` is not a meta character, but it is escapable. That is,
`%` and `\%` both match a literal `%` in all contexts.

The purpose of this routine is to provide knowledge about what characters
may be escaped. Namely, most regex engines permit "superfluous" escapes
where characters without any special significance may be escaped even
though there is no actual _need_ to do so.

This will return false for some characters. For example, `e` is not
escapable. Therefore, `\e` will either result in a parse error (which is
true today), or it could backwards compatibly evolve into a new construct
with its own meaning. Indeed, that is the purpose of banning _some_
superfluous escapes: it provides a way to evolve the syntax in a compatible
manner.

# Example

```
use regex_syntax::is_escapeable_character;

assert!(is_escapeable_character('?'));
assert!(is_escapeable_character('-'));
assert!(is_escapeable_character('&'));
assert!(is_escapeable_character('#'));
assert!(is_escapeable_character('%'));
assert!(is_escapeable_character('/'));
assert!(is_escapeable_character('!'));
assert!(is_escapeable_character('"'));

assert!(!is_escapeable_character('e'));
```

### `is_word_character`

```rust
fn is_word_character(c: char) -> bool
```

Returns true if and only if the given character is a Unicode word
character.

A Unicode word character is defined by
[UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
In particular, a character
is considered a word character if it is in either of the `Alphabetic` or
`Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
or `Connector_Punctuation` general categories.

# Panics

If the `unicode-perl` feature is not enabled, then this function
panics. For this reason, it is recommended that callers use
[`try_is_word_character`](regex_syntax/index.md) instead.

### `try_is_word_character`

```rust
fn try_is_word_character(c: char) -> core::result::Result<bool, UnicodeWordError>
```

Returns true if and only if the given character is a Unicode word
character.

A Unicode word character is defined by
[UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
In particular, a character
is considered a word character if it is in either of the `Alphabetic` or
`Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
or `Connector_Punctuation` general categories.

# Errors

If the `unicode-perl` feature is not enabled, then this function always
returns an error.

### `is_word_byte`

```rust
fn is_word_byte(c: u8) -> bool
```

Returns true if and only if the given character is an ASCII word character.

An ASCII word character is defined by the following character class:
`[_0-9a-zA-Z]`.

