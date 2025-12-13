*[regex_syntax](../index.md) / [hir](index.md)*

---

# Module `hir`

Defines a high-level intermediate (HIR) representation for regular expressions.

The HIR is represented by the [`Hir`](#hir) type, and it principally constructed via
[translation](translate) from an [`Ast`](crate::ast::Ast). Alternatively, users
may use the smart constructors defined on `Hir` to build their own by hand. The
smart constructors simultaneously simplify and "optimize" the HIR, and are also
the same routines used by translation.

Most regex engines only have an HIR like this, and usually construct it
directly from the concrete syntax. This crate however first parses the
concrete syntax into an `Ast`, and only then creates the HIR from the `Ast`,
as mentioned above. It's done this way to facilitate better error reporting,
and to have a structured representation of a regex that faithfully represents
its concrete syntax. Namely, while an `Hir` value can be converted back to an
equivalent regex pattern string, it is unlikely to look like the original due
to its simplified structure.

## Contents

- [Modules](#modules)
  - [`interval`](#interval)
  - [`literal`](#literal)
  - [`print`](#print)
  - [`translate`](#translate)
  - [`visitor`](#visitor)
- [Structs](#structs)
  - [`CaseFoldError`](#casefolderror)
  - [`Error`](#error)
  - [`Hir`](#hir)
  - [`Literal`](#literal)
  - [`ClassUnicode`](#classunicode)
  - [`ClassUnicodeIter`](#classunicodeiter)
  - [`ClassUnicodeRange`](#classunicoderange)
  - [`ClassBytes`](#classbytes)
  - [`ClassBytesIter`](#classbytesiter)
  - [`ClassBytesRange`](#classbytesrange)
  - [`Capture`](#capture)
  - [`Repetition`](#repetition)
  - [`Properties`](#properties)
  - [`PropertiesI`](#propertiesi)
  - [`LookSet`](#lookset)
  - [`LookSetIter`](#looksetiter)
- [Enums](#enums)
  - [`ErrorKind`](#errorkind)
  - [`HirKind`](#hirkind)
  - [`Class`](#class)
  - [`Look`](#look)
  - [`Dot`](#dot)
- [Traits](#traits)
  - [`Visitor`](#visitor)
- [Functions](#functions)
  - [`visit`](#visit)
  - [`class_chars`](#class-chars)
  - [`class_bytes`](#class-bytes)
  - [`singleton_chars`](#singleton-chars)
  - [`singleton_bytes`](#singleton-bytes)
  - [`lift_common_prefix`](#lift-common-prefix)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`interval`](#interval) | mod |  |
| [`literal`](#literal) | mod | Provides literal extraction from `Hir` expressions. |
| [`print`](#print) | mod | This module provides a regular expression printer for `Hir`. |
| [`translate`](#translate) | mod | Defines a translator that converts an `Ast` to an `Hir`. |
| [`visitor`](#visitor) | mod |  |
| [`CaseFoldError`](#casefolderror) | struct |  |
| [`Error`](#error) | struct | An error that can occur while translating an `Ast` to a `Hir`. |
| [`Hir`](#hir) | struct | A high-level intermediate representation (HIR) for a regular expression. |
| [`Literal`](#literal) | struct | The high-level intermediate representation of a literal. |
| [`ClassUnicode`](#classunicode) | struct | A set of characters represented by Unicode scalar values. |
| [`ClassUnicodeIter`](#classunicodeiter) | struct | An iterator over all ranges in a Unicode character class. |
| [`ClassUnicodeRange`](#classunicoderange) | struct | A single range of characters represented by Unicode scalar values. |
| [`ClassBytes`](#classbytes) | struct | A set of characters represented by arbitrary bytes. |
| [`ClassBytesIter`](#classbytesiter) | struct | An iterator over all ranges in a byte character class. |
| [`ClassBytesRange`](#classbytesrange) | struct | A single range of characters represented by arbitrary bytes. |
| [`Capture`](#capture) | struct | The high-level intermediate representation for a capturing group. |
| [`Repetition`](#repetition) | struct | The high-level intermediate representation of a repetition operator. |
| [`Properties`](#properties) | struct | A type that collects various properties of an HIR value. |
| [`PropertiesI`](#propertiesi) | struct | The property definition. |
| [`LookSet`](#lookset) | struct | A set of look-around assertions. |
| [`LookSetIter`](#looksetiter) | struct | An iterator over all look-around assertions in a [`LookSet`]. |
| [`ErrorKind`](#errorkind) | enum | The type of an error that occurred while building an `Hir`. |
| [`HirKind`](#hirkind) | enum | The underlying kind of an arbitrary [`Hir`] expression. |
| [`Class`](#class) | enum | The high-level intermediate representation of a character class. |
| [`Look`](#look) | enum | The high-level intermediate representation for a look-around assertion. |
| [`Dot`](#dot) | enum | A type describing the different flavors of `.`. |
| [`Visitor`](#visitor) | trait |  |
| [`visit`](#visit) | fn |  |
| [`class_chars`](#class-chars) | fn | Given a sequence of HIR values where each value corresponds to a Unicode class (or an all-ASCII byte class), return a single Unicode class corresponding to the union of the classes found. |
| [`class_bytes`](#class-bytes) | fn | Given a sequence of HIR values where each value corresponds to a byte class (or an all-ASCII Unicode class), return a single byte class corresponding to the union of the classes found. |
| [`singleton_chars`](#singleton-chars) | fn | Given a sequence of HIR values where each value corresponds to a literal that is a single `char`, return that sequence of `char`s. |
| [`singleton_bytes`](#singleton-bytes) | fn | Given a sequence of HIR values where each value corresponds to a literal that is a single byte, return that sequence of bytes. |
| [`lift_common_prefix`](#lift-common-prefix) | fn | Looks for a common prefix in the list of alternation branches given. |

## Modules

- [`interval`](interval/index.md)
- [`literal`](literal/index.md) — Provides literal extraction from `Hir` expressions.
- [`print`](print/index.md) — This module provides a regular expression printer for `Hir`.
- [`translate`](translate/index.md) — Defines a translator that converts an `Ast` to an `Hir`.
- [`visitor`](visitor/index.md)

## Structs

### `CaseFoldError`

```rust
struct CaseFoldError(());
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:31`](../../../.source_1765633015/regex-syntax-0.8.8/src/unicode.rs#L31)*

An error that occurs when Unicode-aware simple case folding fails.

This error can occur when the case mapping tables necessary for Unicode
aware case folding are unavailable. This only occurs when the
`unicode-case` feature is disabled. (The feature is enabled by default.)

#### Trait Implementations

##### `impl Any for CaseFoldError`

- <span id="casefolderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaseFoldError`

- <span id="casefolderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaseFoldError`

- <span id="casefolderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CaseFoldError`

- <span id="casefolderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CaseFoldError`

- <span id="casefolderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CaseFoldError`

##### `impl<T> From for CaseFoldError`

- <span id="casefolderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CaseFoldError`

- <span id="casefolderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for CaseFoldError`

- <span id="casefolderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for CaseFoldError`

- <span id="casefolderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="casefolderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaseFoldError`

- <span id="casefolderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="casefolderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Error`

```rust
struct Error {
    kind: ErrorKind,
    pattern: alloc::string::String,
    span: crate::ast::Span,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:49-57`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L49-L57)*

An error that can occur while translating an `Ast` to a `Hir`.

#### Fields

- **`kind`**: `ErrorKind`

  The kind of error.

- **`pattern`**: `alloc::string::String`

  The original pattern that the translator's Ast was parsed from. Every
  span in an error is a valid range into this string.

- **`span`**: `crate::ast::Span`

  The span of this error, derived from the Ast given to the translator.

#### Implementations

- <span id="error-kind"></span>`fn kind(&self) -> &ErrorKind` — [`ErrorKind`](#errorkind)

  Return the type of this error.

- <span id="error-pattern"></span>`fn pattern(&self) -> &str`

  The original pattern string in which this error occurred.

  

  Every span reported by this error is reported in terms of this string.

- <span id="error-span"></span>`fn span(&self) -> &Span` — [`Span`](../ast/index.md#span)

  Return the span at which this error occurred.

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToOwned for Error`

- <span id="error-toowned-type-owned"></span>`type Owned = T`

- <span id="error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Hir`

```rust
struct Hir {
    kind: HirKind,
    props: Properties,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:205-210`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L205-L210)*

A high-level intermediate representation (HIR) for a regular expression.

An HIR value is a combination of a [`HirKind`](#hirkind) and a set of [`Properties`](#properties).
An `HirKind` indicates what kind of regular expression it is (a literal,
a repetition, a look-around assertion, etc.), where as a `Properties`
describes various facts about the regular expression. For example, whether
it matches UTF-8 or if it matches the empty string.

The HIR of a regular expression represents an intermediate step between
its abstract syntax (a structured description of the concrete syntax) and
an actual regex matcher. The purpose of HIR is to make regular expressions
easier to analyze. In particular, the AST is much more complex than the
HIR. For example, while an AST supports arbitrarily nested character
classes, the HIR will flatten all nested classes into a single set. The HIR
will also "compile away" every flag present in the concrete syntax. For
example, users of HIR expressions never need to worry about case folding;
it is handled automatically by the translator (e.g., by translating
`(?i:A)` to `[aA]`).

The specific type of an HIR expression can be accessed via its `kind`
or `into_kind` methods. This extra level of indirection exists for two
reasons:

1. Construction of an HIR expression *must* use the constructor methods on
this `Hir` type instead of building the `HirKind` values directly. This
permits construction to enforce invariants like "concatenations always
consist of two or more sub-expressions."
2. Every HIR expression contains attributes that are defined inductively,
and can be computed cheaply during the construction process. For example,
one such attribute is whether the expression must match at the beginning of
the haystack.

In particular, if you have an `HirKind` value, then there is intentionally
no way to build an `Hir` value from it. You instead need to do case
analysis on the `HirKind` value and build the `Hir` value using its smart
constructors.

# UTF-8

If the HIR was produced by a translator with
[`TranslatorBuilder::utf8`](translate::TranslatorBuilder::utf8) enabled,
then the HIR is guaranteed to match UTF-8 exclusively for all non-empty
matches.

For empty matches, those can occur at any position. It is the
responsibility of the regex engine to determine whether empty matches are
permitted between the code units of a single codepoint.

# Stack space

This type defines its own destructor that uses constant stack space and
heap space proportional to the size of the HIR.

Also, an `Hir`'s `fmt::Display` implementation prints an HIR as a regular
expression pattern string, and uses constant stack space and heap space
proportional to the size of the `Hir`. The regex it prints is guaranteed to
be _semantically_ equivalent to the original concrete syntax, but it may
look very different. (And potentially not practically readable by a human.)

An `Hir`'s `fmt::Debug` implementation currently does not use constant
stack space. The implementation will also suppress some details (such as
the `Properties` inlined into every `Hir` value to make it less noisy).

#### Fields

- **`kind`**: `HirKind`

  The underlying HIR kind.

- **`props`**: `Properties`

  Analysis info about this HIR, computed during construction.

#### Implementations

- <span id="hir-kind"></span>`fn kind(&self) -> &HirKind` — [`HirKind`](#hirkind)

  Returns a reference to the underlying HIR kind.

- <span id="hir-into-kind"></span>`fn into_kind(self) -> HirKind` — [`HirKind`](#hirkind)

  Consumes ownership of this HIR expression and returns its underlying

  `HirKind`.

- <span id="hir-properties"></span>`fn properties(&self) -> &Properties` — [`Properties`](#properties)

  Returns the properties computed for this `Hir`.

- <span id="hir-into-parts"></span>`fn into_parts(self) -> (HirKind, Properties)` — [`HirKind`](#hirkind), [`Properties`](#properties)

  Splits this HIR into its constituent parts.

  

  This is useful because `let Hir { kind, props } = hir;` does not work

  because of `Hir`'s custom `Drop` implementation.

#### Trait Implementations

##### `impl Any for Hir`

- <span id="hir-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Hir`

- <span id="hir-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Hir`

- <span id="hir-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Hir`

- <span id="hir-clone"></span>`fn clone(&self) -> Hir` — [`Hir`](#hir)

##### `impl CloneToUninit for Hir`

- <span id="hir-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Hir`

- <span id="hir-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for Hir`

- <span id="hir-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Drop for Hir`

- <span id="hir-drop"></span>`fn drop(&mut self)`

##### `impl Eq for Hir`

##### `impl<T> From for Hir`

- <span id="hir-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Hir`

- <span id="hir-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Hir`

- <span id="hir-partialeq-eq"></span>`fn eq(&self, other: &Hir) -> bool` — [`Hir`](#hir)

##### `impl StructuralPartialEq for Hir`

##### `impl ToOwned for Hir`

- <span id="hir-toowned-type-owned"></span>`type Owned = T`

- <span id="hir-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hir-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Hir`

- <span id="hir-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Hir`

- <span id="hir-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hir-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Hir`

- <span id="hir-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hir-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Literal`

```rust
struct Literal(alloc::boxed::Box<[u8]>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:801`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L801)*

The high-level intermediate representation of a literal.

A literal corresponds to `0` or more bytes that should be matched
literally. The smart constructors defined on `Hir` will automatically
concatenate adjacent literals into one literal, and will even automatically
replace empty literals with `Hir::empty()`.

Note that despite a literal being represented by a sequence of bytes, its
`Debug` implementation will attempt to print it as a normal string. (That
is, not a sequence of decimal numbers.)

#### Trait Implementations

##### `impl Any for Literal`

- <span id="literal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Literal`

- <span id="literal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Literal`

- <span id="literal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl CloneToUninit for Literal`

- <span id="literal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Literal`

- <span id="literal-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Literal`

##### `impl<T> From for Literal`

- <span id="literal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Literal`

- <span id="literal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Literal`

- <span id="literal-partialeq-eq"></span>`fn eq(&self, other: &Literal) -> bool` — [`Literal`](#literal)

##### `impl StructuralPartialEq for Literal`

##### `impl ToOwned for Literal`

- <span id="literal-toowned-type-owned"></span>`type Owned = T`

- <span id="literal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="literal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Literal`

- <span id="literal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="literal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Literal`

- <span id="literal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="literal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassUnicode`

```rust
struct ClassUnicode {
    set: crate::hir::interval::IntervalSet<ClassUnicodeRange>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1051-1053`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1051-L1053)*

A set of characters represented by Unicode scalar values.

#### Implementations

- <span id="classunicode-new"></span>`fn new<I>(ranges: I) -> ClassUnicode` — [`ClassUnicode`](#classunicode)

  Create a new class from a sequence of ranges.

  

  The given ranges do not need to be in any specific order, and ranges

  may overlap. Ranges will automatically be sorted into a canonical

  non-overlapping order.

- <span id="classunicode-empty"></span>`fn empty() -> ClassUnicode` — [`ClassUnicode`](#classunicode)

  Create a new class with no ranges.

  

  An empty class matches nothing. That is, it is equivalent to

  `Hir::fail`.

- <span id="classunicode-push"></span>`fn push(&mut self, range: ClassUnicodeRange)` — [`ClassUnicodeRange`](#classunicoderange)

  Add a new range to this set.

- <span id="classunicode-iter"></span>`fn iter(&self) -> ClassUnicodeIter<'_>` — [`ClassUnicodeIter`](#classunicodeiter)

  Return an iterator over all ranges in this class.

  

  The iterator yields ranges in ascending order.

- <span id="classunicode-ranges"></span>`fn ranges(&self) -> &[ClassUnicodeRange]` — [`ClassUnicodeRange`](#classunicoderange)

  Return the underlying ranges as a slice.

- <span id="classunicode-case-fold-simple"></span>`fn case_fold_simple(&mut self)`

  Expand this character class such that it contains all case folded

  characters, according to Unicode's "simple" mapping. For example, if

  this class consists of the range `a-z`, then applying case folding will

  result in the class containing both the ranges `a-z` and `A-Z`.

  

  # Panics

  

  This routine panics when the case mapping data necessary for this

  routine to complete is unavailable. This occurs when the `unicode-case`

  feature is not enabled.

  

  Callers should prefer using `try_case_fold_simple` instead, which will

  return an error instead of panicking.

- <span id="classunicode-try-case-fold-simple"></span>`fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError>` — [`CaseFoldError`](../unicode/index.md#casefolderror)

  Expand this character class such that it contains all case folded

  characters, according to Unicode's "simple" mapping. For example, if

  this class consists of the range `a-z`, then applying case folding will

  result in the class containing both the ranges `a-z` and `A-Z`.

  

  # Error

  

  This routine returns an error when the case mapping data necessary

  for this routine to complete is unavailable. This occurs when the

  `unicode-case` feature is not enabled.

- <span id="classunicode-negate"></span>`fn negate(&mut self)`

  Negate this character class.

  

  For all `c` where `c` is a Unicode scalar value, if `c` was in this

  set, then it will not be in this set after negation.

- <span id="classunicode-union"></span>`fn union(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

  Union this character class with the given character class, in place.

- <span id="classunicode-intersect"></span>`fn intersect(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

  Intersect this character class with the given character class, in

  place.

- <span id="classunicode-difference"></span>`fn difference(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

  Subtract the given character class from this character class, in place.

- <span id="classunicode-symmetric-difference"></span>`fn symmetric_difference(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

  Compute the symmetric difference of the given character classes, in

  place.

  

  This computes the symmetric difference of two character classes. This

  removes all elements in this class that are also in the given class,

  but all adds all elements from the given class that aren't in this

  class. That is, the class will contain all elements in either class,

  but will not contain any elements that are in both classes.

- <span id="classunicode-is-ascii"></span>`fn is_ascii(&self) -> bool`

  Returns true if and only if this character class will either match

  nothing or only ASCII bytes. Stated differently, this returns false

  if and only if this class contains a non-ASCII codepoint.

- <span id="classunicode-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

  Returns the length, in bytes, of the smallest string matched by this

  character class.

  

  Returns `None` when the class is empty.

- <span id="classunicode-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

  Returns the length, in bytes, of the longest string matched by this

  character class.

  

  Returns `None` when the class is empty.

- <span id="classunicode-literal"></span>`fn literal(&self) -> Option<Vec<u8>>`

  If this class consists of exactly one codepoint, then return it as

  a literal byte string.

  

  If this class is empty or contains more than one codepoint, then `None`

  is returned.

- <span id="classunicode-to-byte-class"></span>`fn to_byte_class(&self) -> Option<ClassBytes>` — [`ClassBytes`](#classbytes)

  If this class consists of only ASCII ranges, then return its

  corresponding and equivalent byte class.

#### Trait Implementations

##### `impl Any for ClassUnicode`

- <span id="classunicode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassUnicode`

- <span id="classunicode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassUnicode`

- <span id="classunicode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassUnicode`

- <span id="classunicode-clone"></span>`fn clone(&self) -> ClassUnicode` — [`ClassUnicode`](#classunicode)

##### `impl CloneToUninit for ClassUnicode`

- <span id="classunicode-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassUnicode`

- <span id="classunicode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicode`

##### `impl<T> From for ClassUnicode`

- <span id="classunicode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassUnicode`

- <span id="classunicode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassUnicode`

- <span id="classunicode-partialeq-eq"></span>`fn eq(&self, other: &ClassUnicode) -> bool` — [`ClassUnicode`](#classunicode)

##### `impl StructuralPartialEq for ClassUnicode`

##### `impl ToOwned for ClassUnicode`

- <span id="classunicode-toowned-type-owned"></span>`type Owned = T`

- <span id="classunicode-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classunicode-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassUnicode`

- <span id="classunicode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classunicode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassUnicode`

- <span id="classunicode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classunicode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassUnicodeIter<'a>`

```rust
struct ClassUnicodeIter<'a>(crate::hir::interval::IntervalSetIter<'a, ClassUnicodeRange>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1226`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1226)*

An iterator over all ranges in a Unicode character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl Any for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="classunicodeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="classunicodeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-iterator-type-item"></span>`type Item = &'a ClassUnicodeRange`

- <span id="classunicodeiter-iterator-next"></span>`fn next(&mut self) -> Option<&'a ClassUnicodeRange>` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl<U> TryFrom for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classunicodeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classunicodeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassUnicodeRange`

```rust
struct ClassUnicodeRange {
    start: char,
    end: char,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1241-1244`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1241-L1244)*

A single range of characters represented by Unicode scalar values.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- <span id="classunicoderange-new"></span>`fn new(start: char, end: char) -> ClassUnicodeRange` — [`ClassUnicodeRange`](#classunicoderange)

  Create a new Unicode scalar value range for a character class.

  

  The returned range is always in a canonical form. That is, the range

  returned always satisfies the invariant that `start <= end`.

- <span id="classunicoderange-start"></span>`fn start(&self) -> char`

  Return the start of this range.

  

  The start of a range is always less than or equal to the end of the

  range.

- <span id="classunicoderange-end"></span>`fn end(&self) -> char`

  Return the end of this range.

  

  The end of a range is always greater than or equal to the start of the

  range.

- <span id="classunicoderange-len"></span>`fn len(&self) -> usize`

  Returns the number of codepoints in this range.

#### Trait Implementations

##### `impl Any for ClassUnicodeRange`

- <span id="classunicoderange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassUnicodeRange`

- <span id="classunicoderange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassUnicodeRange`

- <span id="classunicoderange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassUnicodeRange`

- <span id="classunicoderange-clone"></span>`fn clone(&self) -> ClassUnicodeRange` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl CloneToUninit for ClassUnicodeRange`

- <span id="classunicoderange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ClassUnicodeRange`

##### `impl Debug for ClassUnicodeRange`

- <span id="classunicoderange-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for ClassUnicodeRange`

- <span id="classunicoderange-default"></span>`fn default() -> ClassUnicodeRange` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl Eq for ClassUnicodeRange`

##### `impl<T> From for ClassUnicodeRange`

- <span id="classunicoderange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Interval for ClassUnicodeRange`

- <span id="classunicoderange-interval-type-bound"></span>`type Bound = char`

- <span id="classunicoderange-interval-lower"></span>`fn lower(&self) -> char`

- <span id="classunicoderange-interval-upper"></span>`fn upper(&self) -> char`

- <span id="classunicoderange-interval-set-lower"></span>`fn set_lower(&mut self, bound: char)`

- <span id="classunicoderange-interval-set-upper"></span>`fn set_upper(&mut self, bound: char)`

- <span id="classunicoderange-interval-case-fold-simple"></span>`fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) -> Result<(), unicode::CaseFoldError>` — [`ClassUnicodeRange`](#classunicoderange), [`CaseFoldError`](../unicode/index.md#casefolderror)

  Apply simple case folding to this Unicode scalar value range.

  

  Additional ranges are appended to the given vector. Canonical ordering

  is *not* maintained in the given vector.

##### `impl<U> Into for ClassUnicodeRange`

- <span id="classunicoderange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ClassUnicodeRange`

- <span id="classunicoderange-ord-cmp"></span>`fn cmp(&self, other: &ClassUnicodeRange) -> cmp::Ordering` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl PartialEq for ClassUnicodeRange`

- <span id="classunicoderange-partialeq-eq"></span>`fn eq(&self, other: &ClassUnicodeRange) -> bool` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl PartialOrd for ClassUnicodeRange`

- <span id="classunicoderange-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ClassUnicodeRange) -> option::Option<cmp::Ordering>` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl StructuralPartialEq for ClassUnicodeRange`

##### `impl ToOwned for ClassUnicodeRange`

- <span id="classunicoderange-toowned-type-owned"></span>`type Owned = T`

- <span id="classunicoderange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classunicoderange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassUnicodeRange`

- <span id="classunicoderange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classunicoderange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassUnicodeRange`

- <span id="classunicoderange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classunicoderange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassBytes`

```rust
struct ClassBytes {
    set: crate::hir::interval::IntervalSet<ClassBytesRange>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1350-1352`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1350-L1352)*

A set of characters represented by arbitrary bytes.

Each byte corresponds to one character.

#### Implementations

- <span id="classbytes-new"></span>`fn new<I>(ranges: I) -> ClassBytes` — [`ClassBytes`](#classbytes)

  Create a new class from a sequence of ranges.

  

  The given ranges do not need to be in any specific order, and ranges

  may overlap. Ranges will automatically be sorted into a canonical

  non-overlapping order.

- <span id="classbytes-empty"></span>`fn empty() -> ClassBytes` — [`ClassBytes`](#classbytes)

  Create a new class with no ranges.

  

  An empty class matches nothing. That is, it is equivalent to

  `Hir::fail`.

- <span id="classbytes-push"></span>`fn push(&mut self, range: ClassBytesRange)` — [`ClassBytesRange`](#classbytesrange)

  Add a new range to this set.

- <span id="classbytes-iter"></span>`fn iter(&self) -> ClassBytesIter<'_>` — [`ClassBytesIter`](#classbytesiter)

  Return an iterator over all ranges in this class.

  

  The iterator yields ranges in ascending order.

- <span id="classbytes-ranges"></span>`fn ranges(&self) -> &[ClassBytesRange]` — [`ClassBytesRange`](#classbytesrange)

  Return the underlying ranges as a slice.

- <span id="classbytes-case-fold-simple"></span>`fn case_fold_simple(&mut self)`

  Expand this character class such that it contains all case folded

  characters. For example, if this class consists of the range `a-z`,

  then applying case folding will result in the class containing both the

  ranges `a-z` and `A-Z`.

  

  Note that this only applies ASCII case folding, which is limited to the

  characters `a-z` and `A-Z`.

- <span id="classbytes-negate"></span>`fn negate(&mut self)`

  Negate this byte class.

  

  For all `b` where `b` is a any byte, if `b` was in this set, then it

  will not be in this set after negation.

- <span id="classbytes-union"></span>`fn union(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

  Union this byte class with the given byte class, in place.

- <span id="classbytes-intersect"></span>`fn intersect(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

  Intersect this byte class with the given byte class, in place.

- <span id="classbytes-difference"></span>`fn difference(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

  Subtract the given byte class from this byte class, in place.

- <span id="classbytes-symmetric-difference"></span>`fn symmetric_difference(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

  Compute the symmetric difference of the given byte classes, in place.

  

  This computes the symmetric difference of two byte classes. This

  removes all elements in this class that are also in the given class,

  but all adds all elements from the given class that aren't in this

  class. That is, the class will contain all elements in either class,

  but will not contain any elements that are in both classes.

- <span id="classbytes-is-ascii"></span>`fn is_ascii(&self) -> bool`

  Returns true if and only if this character class will either match

  nothing or only ASCII bytes. Stated differently, this returns false

  if and only if this class contains a non-ASCII byte.

- <span id="classbytes-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

  Returns the length, in bytes, of the smallest string matched by this

  character class.

  

  Returns `None` when the class is empty.

- <span id="classbytes-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

  Returns the length, in bytes, of the longest string matched by this

  character class.

  

  Returns `None` when the class is empty.

- <span id="classbytes-literal"></span>`fn literal(&self) -> Option<Vec<u8>>`

  If this class consists of exactly one byte, then return it as

  a literal byte string.

  

  If this class is empty or contains more than one byte, then `None`

  is returned.

- <span id="classbytes-to-unicode-class"></span>`fn to_unicode_class(&self) -> Option<ClassUnicode>` — [`ClassUnicode`](#classunicode)

  If this class consists of only ASCII ranges, then return its

  corresponding and equivalent Unicode class.

#### Trait Implementations

##### `impl Any for ClassBytes`

- <span id="classbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassBytes`

- <span id="classbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassBytes`

- <span id="classbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassBytes`

- <span id="classbytes-clone"></span>`fn clone(&self) -> ClassBytes` — [`ClassBytes`](#classbytes)

##### `impl CloneToUninit for ClassBytes`

- <span id="classbytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassBytes`

- <span id="classbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassBytes`

##### `impl<T> From for ClassBytes`

- <span id="classbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassBytes`

- <span id="classbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassBytes`

- <span id="classbytes-partialeq-eq"></span>`fn eq(&self, other: &ClassBytes) -> bool` — [`ClassBytes`](#classbytes)

##### `impl StructuralPartialEq for ClassBytes`

##### `impl ToOwned for ClassBytes`

- <span id="classbytes-toowned-type-owned"></span>`type Owned = T`

- <span id="classbytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classbytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassBytes`

- <span id="classbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassBytes`

- <span id="classbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassBytesIter<'a>`

```rust
struct ClassBytesIter<'a>(crate::hir::interval::IntervalSetIter<'a, ClassBytesRange>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1504`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1504)*

An iterator over all ranges in a byte character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl Any for ClassBytesIter<'a>`

- <span id="classbytesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassBytesIter<'a>`

- <span id="classbytesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassBytesIter<'a>`

- <span id="classbytesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ClassBytesIter<'a>`

- <span id="classbytesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ClassBytesIter<'a>`

- <span id="classbytesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassBytesIter<'a>`

- <span id="classbytesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ClassBytesIter<'a>`

- <span id="classbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="classbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="classbytesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ClassBytesIter<'a>`

- <span id="classbytesiter-iterator-type-item"></span>`type Item = &'a ClassBytesRange`

- <span id="classbytesiter-iterator-next"></span>`fn next(&mut self) -> Option<&'a ClassBytesRange>` — [`ClassBytesRange`](#classbytesrange)

##### `impl<U> TryFrom for ClassBytesIter<'a>`

- <span id="classbytesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classbytesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassBytesIter<'a>`

- <span id="classbytesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classbytesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassBytesRange`

```rust
struct ClassBytesRange {
    start: u8,
    end: u8,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1519-1522`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1519-L1522)*

A single range of characters represented by arbitrary bytes.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- <span id="classbytesrange-new"></span>`fn new(start: u8, end: u8) -> ClassBytesRange` — [`ClassBytesRange`](#classbytesrange)

  Create a new byte range for a character class.

  

  The returned range is always in a canonical form. That is, the range

  returned always satisfies the invariant that `start <= end`.

- <span id="classbytesrange-start"></span>`fn start(&self) -> u8`

  Return the start of this range.

  

  The start of a range is always less than or equal to the end of the

  range.

- <span id="classbytesrange-end"></span>`fn end(&self) -> u8`

  Return the end of this range.

  

  The end of a range is always greater than or equal to the start of the

  range.

- <span id="classbytesrange-len"></span>`fn len(&self) -> usize`

  Returns the number of bytes in this range.

#### Trait Implementations

##### `impl Any for ClassBytesRange`

- <span id="classbytesrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassBytesRange`

- <span id="classbytesrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassBytesRange`

- <span id="classbytesrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassBytesRange`

- <span id="classbytesrange-clone"></span>`fn clone(&self) -> ClassBytesRange` — [`ClassBytesRange`](#classbytesrange)

##### `impl CloneToUninit for ClassBytesRange`

- <span id="classbytesrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ClassBytesRange`

##### `impl Debug for ClassBytesRange`

- <span id="classbytesrange-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for ClassBytesRange`

- <span id="classbytesrange-default"></span>`fn default() -> ClassBytesRange` — [`ClassBytesRange`](#classbytesrange)

##### `impl Eq for ClassBytesRange`

##### `impl<T> From for ClassBytesRange`

- <span id="classbytesrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Interval for ClassBytesRange`

- <span id="classbytesrange-interval-type-bound"></span>`type Bound = u8`

- <span id="classbytesrange-interval-lower"></span>`fn lower(&self) -> u8`

- <span id="classbytesrange-interval-upper"></span>`fn upper(&self) -> u8`

- <span id="classbytesrange-interval-set-lower"></span>`fn set_lower(&mut self, bound: u8)`

- <span id="classbytesrange-interval-set-upper"></span>`fn set_upper(&mut self, bound: u8)`

- <span id="classbytesrange-interval-case-fold-simple"></span>`fn case_fold_simple(&self, ranges: &mut Vec<ClassBytesRange>) -> Result<(), unicode::CaseFoldError>` — [`ClassBytesRange`](#classbytesrange), [`CaseFoldError`](../unicode/index.md#casefolderror)

  Apply simple case folding to this byte range. Only ASCII case mappings

  (for a-z) are applied.

  

  Additional ranges are appended to the given vector. Canonical ordering

  is *not* maintained in the given vector.

##### `impl<U> Into for ClassBytesRange`

- <span id="classbytesrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ClassBytesRange`

- <span id="classbytesrange-ord-cmp"></span>`fn cmp(&self, other: &ClassBytesRange) -> cmp::Ordering` — [`ClassBytesRange`](#classbytesrange)

##### `impl PartialEq for ClassBytesRange`

- <span id="classbytesrange-partialeq-eq"></span>`fn eq(&self, other: &ClassBytesRange) -> bool` — [`ClassBytesRange`](#classbytesrange)

##### `impl PartialOrd for ClassBytesRange`

- <span id="classbytesrange-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ClassBytesRange) -> option::Option<cmp::Ordering>` — [`ClassBytesRange`](#classbytesrange)

##### `impl StructuralPartialEq for ClassBytesRange`

##### `impl ToOwned for ClassBytesRange`

- <span id="classbytesrange-toowned-type-owned"></span>`type Owned = T`

- <span id="classbytesrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classbytesrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassBytesRange`

- <span id="classbytesrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classbytesrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassBytesRange`

- <span id="classbytesrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classbytesrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Capture`

```rust
struct Capture {
    pub index: u32,
    pub name: Option<alloc::boxed::Box<str>>,
    pub sub: alloc::boxed::Box<Hir>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1799-1806`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1799-L1806)*

The high-level intermediate representation for a capturing group.

A capturing group always has an index and a child expression. It may
also have a name associated with it (e.g., `(?P<foo>\w)`), but it's not
necessary.

Note that there is no explicit representation of a non-capturing group
in a `Hir`. Instead, non-capturing grouping is handled automatically by
the recursive structure of the `Hir` itself.

#### Fields

- **`index`**: `u32`

  The capture index of the capture.

- **`name`**: `Option<alloc::boxed::Box<str>>`

  The name of the capture, if it exists.

- **`sub`**: `alloc::boxed::Box<Hir>`

  The expression inside the capturing group, which may be empty.

#### Trait Implementations

##### `impl Any for Capture`

- <span id="capture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Capture`

- <span id="capture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Capture`

- <span id="capture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Capture`

- <span id="capture-clone"></span>`fn clone(&self) -> Capture` — [`Capture`](#capture)

##### `impl CloneToUninit for Capture`

- <span id="capture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Capture`

- <span id="capture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Capture`

##### `impl<T> From for Capture`

- <span id="capture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Capture`

- <span id="capture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Capture`

- <span id="capture-partialeq-eq"></span>`fn eq(&self, other: &Capture) -> bool` — [`Capture`](#capture)

##### `impl StructuralPartialEq for Capture`

##### `impl ToOwned for Capture`

- <span id="capture-toowned-type-owned"></span>`type Owned = T`

- <span id="capture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="capture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Capture`

- <span id="capture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Capture`

- <span id="capture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Repetition`

```rust
struct Repetition {
    pub min: u32,
    pub max: Option<u32>,
    pub greedy: bool,
    pub sub: alloc::boxed::Box<Hir>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1813-1839`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1813-L1839)*

The high-level intermediate representation of a repetition operator.

A repetition operator permits the repetition of an arbitrary
sub-expression.

#### Fields

- **`min`**: `u32`

  The minimum range of the repetition.
  
  Note that special cases like `?`, `+` and `*` all get translated into
  the ranges `{0,1}`, `{1,}` and `{0,}`, respectively.
  
  When `min` is zero, this expression can match the empty string
  regardless of what its sub-expression is.

- **`max`**: `Option<u32>`

  The maximum range of the repetition.
  
  Note that when `max` is `None`, `min` acts as a lower bound but where
  there is no upper bound. For something like `x{5}` where the min and
  max are equivalent, `min` will be set to `5` and `max` will be set to
  `Some(5)`.

- **`greedy`**: `bool`

  Whether this repetition operator is greedy or not. A greedy operator
  will match as much as it can. A non-greedy operator will match as
  little as it can.
  
  Typically, operators are greedy by default and are only non-greedy when
  a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
  not. However, this can be inverted via the `U` "ungreedy" flag.

- **`sub`**: `alloc::boxed::Box<Hir>`

  The expression being repeated.

#### Implementations

- <span id="repetition-with"></span>`fn with(&self, sub: Hir) -> Repetition` — [`Hir`](#hir), [`Repetition`](#repetition)

  Returns a new repetition with the same `min`, `max` and `greedy`

  values, but with its sub-expression replaced with the one given.

#### Trait Implementations

##### `impl Any for Repetition`

- <span id="repetition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Repetition`

- <span id="repetition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Repetition`

- <span id="repetition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Repetition`

- <span id="repetition-clone"></span>`fn clone(&self) -> Repetition` — [`Repetition`](#repetition)

##### `impl CloneToUninit for Repetition`

- <span id="repetition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Repetition`

- <span id="repetition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Repetition`

##### `impl<T> From for Repetition`

- <span id="repetition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Repetition`

- <span id="repetition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Repetition`

- <span id="repetition-partialeq-eq"></span>`fn eq(&self, other: &Repetition) -> bool` — [`Repetition`](#repetition)

##### `impl StructuralPartialEq for Repetition`

##### `impl ToOwned for Repetition`

- <span id="repetition-toowned-type-owned"></span>`type Owned = T`

- <span id="repetition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repetition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Repetition`

- <span id="repetition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repetition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Repetition`

- <span id="repetition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repetition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Properties`

```rust
struct Properties(alloc::boxed::Box<PropertiesI>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1964`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1964)*

A type that collects various properties of an HIR value.

Properties are always scalar values and represent meta data that is
computed inductively on an HIR value. Properties are defined for all
HIR values.

All methods on a `Properties` value take constant time and are meant to
be cheap to call.

#### Implementations

- <span id="properties-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

  Returns the length (in bytes) of the smallest string matched by this

  HIR.

  

  A return value of `0` is possible and occurs when the HIR can match an

  empty string.

  

  `None` is returned when there is no minimum length. This occurs in

  precisely the cases where the HIR matches nothing. i.e., The language

  the regex matches is empty. An example of such a regex is `\P{any}`.

- <span id="properties-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

  Returns the length (in bytes) of the longest string matched by this

  HIR.

  

  A return value of `0` is possible and occurs when nothing longer than

  the empty string is in the language described by this HIR.

  

  `None` is returned when there is no longest matching string. This

  occurs when the HIR matches nothing or when there is no upper bound on

  the length of matching strings. Example of such regexes are `\P{any}`

  (matches nothing) and `a+` (has no upper bound).

- <span id="properties-look-set"></span>`fn look_set(&self) -> LookSet` — [`LookSet`](#lookset)

  Returns a set of all look-around assertions that appear at least once

  in this HIR value.

- <span id="properties-look-set-prefix"></span>`fn look_set_prefix(&self) -> LookSet` — [`LookSet`](#lookset)

  Returns a set of all look-around assertions that appear as a prefix for

  this HIR value. That is, the set returned corresponds to the set of

  assertions that must be passed before matching any bytes in a haystack.

  

  For example, `hir.look_set_prefix().contains(Look::Start)` returns true

  if and only if the HIR is fully anchored at the start.

- <span id="properties-look-set-prefix-any"></span>`fn look_set_prefix_any(&self) -> LookSet` — [`LookSet`](#lookset)

  Returns a set of all look-around assertions that appear as a _possible_

  prefix for this HIR value. That is, the set returned corresponds to the

  set of assertions that _may_ be passed before matching any bytes in a

  haystack.

  

  For example, `hir.look_set_prefix_any().contains(Look::Start)` returns

  true if and only if it's possible for the regex to match through a

  anchored assertion before consuming any input.

- <span id="properties-look-set-suffix"></span>`fn look_set_suffix(&self) -> LookSet` — [`LookSet`](#lookset)

  Returns a set of all look-around assertions that appear as a suffix for

  this HIR value. That is, the set returned corresponds to the set of

  assertions that must be passed in order to be considered a match after

  all other consuming HIR expressions.

  

  For example, `hir.look_set_suffix().contains(Look::End)` returns true

  if and only if the HIR is fully anchored at the end.

- <span id="properties-look-set-suffix-any"></span>`fn look_set_suffix_any(&self) -> LookSet` — [`LookSet`](#lookset)

  Returns a set of all look-around assertions that appear as a _possible_

  suffix for this HIR value. That is, the set returned corresponds to the

  set of assertions that _may_ be passed before matching any bytes in a

  haystack.

  

  For example, `hir.look_set_suffix_any().contains(Look::End)` returns

  true if and only if it's possible for the regex to match through a

  anchored assertion at the end of a match without consuming any input.

- <span id="properties-is-utf8"></span>`fn is_utf8(&self) -> bool`

  Return true if and only if the corresponding HIR will always match

  valid UTF-8.

  

  When this returns false, then it is possible for this HIR expression to

  match invalid UTF-8, including by matching between the code units of

  a single UTF-8 encoded codepoint.

  

  Note that this returns true even when the corresponding HIR can match

  the empty string. Since an empty string can technically appear between

  UTF-8 code units, it is possible for a match to be reported that splits

  a codepoint which could in turn be considered matching invalid UTF-8.

  However, it is generally assumed that such empty matches are handled

  specially by the search routine if it is absolutely required that

  matches not split a codepoint.

  

  # Example

  

  This code example shows the UTF-8 property of a variety of patterns.

  

  ```rust

  use regex_syntax::{ParserBuilder, parse};

  

  // Examples of 'is_utf8() == true'.

  assert!(parse(r"a")?.properties().is_utf8());

  assert!(parse(r"[^a]")?.properties().is_utf8());

  assert!(parse(r".")?.properties().is_utf8());

  assert!(parse(r"\W")?.properties().is_utf8());

  assert!(parse(r"\b")?.properties().is_utf8());

  assert!(parse(r"\B")?.properties().is_utf8());

  assert!(parse(r"(?-u)\b")?.properties().is_utf8());

  assert!(parse(r"(?-u)\B")?.properties().is_utf8());

  // Unicode mode is enabled by default, and in

  // that mode, all \x hex escapes are treated as

  // codepoints. So this actually matches the UTF-8

  // encoding of U+00FF.

  assert!(parse(r"\xFF")?.properties().is_utf8());

  

  // Now we show examples of 'is_utf8() == false'.

  // The only way to do this is to force the parser

  // to permit invalid UTF-8, otherwise all of these

  // would fail to parse!

  let parse = |pattern| {

      ParserBuilder::new().utf8(false).build().parse(pattern)

  };

  assert!(!parse(r"(?-u)[^a]")?.properties().is_utf8());

  assert!(!parse(r"(?-u).")?.properties().is_utf8());

  assert!(!parse(r"(?-u)\W")?.properties().is_utf8());

  // Conversely to the equivalent example above,

  // when Unicode mode is disabled, \x hex escapes

  // are treated as their raw byte values.

  assert!(!parse(r"(?-u)\xFF")?.properties().is_utf8());

  // Note that just because we disabled UTF-8 in the

  // parser doesn't mean we still can't use Unicode.

  // It is enabled by default, so \xFF is still

  // equivalent to matching the UTF-8 encoding of

  // U+00FF by default.

  assert!(parse(r"\xFF")?.properties().is_utf8());

  // Even though we use raw bytes that individually

  // are not valid UTF-8, when combined together, the

  // overall expression *does* match valid UTF-8!

  assert!(parse(r"(?-u)\xE2\x98\x83")?.properties().is_utf8());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="properties-explicit-captures-len"></span>`fn explicit_captures_len(&self) -> usize`

  Returns the total number of explicit capturing groups in the

  corresponding HIR.

  

  Note that this does not include the implicit capturing group

  corresponding to the entire match that is typically included by regex

  engines.

  

  # Example

  

  This method will return `0` for `a` and `1` for `(a)`:

  

  ```rust

  use regex_syntax::parse;

  

  assert_eq!(0, parse("a")?.properties().explicit_captures_len());

  assert_eq!(1, parse("(a)")?.properties().explicit_captures_len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="properties-static-explicit-captures-len"></span>`fn static_explicit_captures_len(&self) -> Option<usize>`

  Returns the total number of explicit capturing groups that appear in

  every possible match.

  

  If the number of capture groups can vary depending on the match, then

  this returns `None`. That is, a value is only returned when the number

  of matching groups is invariant or "static."

  

  Note that this does not include the implicit capturing group

  corresponding to the entire match.

  

  # Example

  

  This shows a few cases where a static number of capture groups is

  available and a few cases where it is not.

  

  ```rust

  use regex_syntax::parse;

  

  let len = |pattern| {

      parse(pattern).map(|h| {

          h.properties().static_explicit_captures_len()

      })

  };

  

  assert_eq!(Some(0), len("a")?);

  assert_eq!(Some(1), len("(a)")?);

  assert_eq!(Some(1), len("(a)|(b)")?);

  assert_eq!(Some(2), len("(a)(b)|(c)(d)")?);

  assert_eq!(None, len("(a)|b")?);

  assert_eq!(None, len("a|(b)")?);

  assert_eq!(None, len("(b)*")?);

  assert_eq!(Some(1), len("(b)+")?);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="properties-is-literal"></span>`fn is_literal(&self) -> bool`

  Return true if and only if this HIR is a simple literal. This is

  only true when this HIR expression is either itself a `Literal` or a

  concatenation of only `Literal`s.

  

  For example, `f` and `foo` are literals, but `f+`, `(foo)`, `foo()` and

  the empty string are not (even though they contain sub-expressions that

  are literals).

- <span id="properties-is-alternation-literal"></span>`fn is_alternation_literal(&self) -> bool`

  Return true if and only if this HIR is either a simple literal or an

  alternation of simple literals. This is only

  true when this HIR expression is either itself a `Literal` or a

  concatenation of only `Literal`s or an alternation of only `Literal`s.

  

  For example, `f`, `foo`, `a|b|c`, and `foo|bar|baz` are alternation

  literals, but `f+`, `(foo)`, `foo()`, and the empty pattern are not

  (even though that contain sub-expressions that are literals).

- <span id="properties-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the total amount of heap memory usage, in bytes, used by this

  `Properties` value.

- <span id="properties-union"></span>`fn union<I, P>(props: I) -> Properties` — [`Properties`](#properties)

  Returns a new set of properties that corresponds to the union of the

  iterator of properties given.

  

  This is useful when one has multiple `Hir` expressions and wants

  to combine them into a single alternation without constructing the

  corresponding `Hir`. This routine provides a way of combining the

  properties of each `Hir` expression into one set of properties

  representing the union of those expressions.

  

  # Example: union with HIRs that never match

  

  This example shows that unioning properties together with one that

  represents a regex that never matches will "poison" certain attributes,

  like the minimum and maximum lengths.

  

  ```rust

  use regex_syntax::{hir::Properties, parse};

  

  let hir1 = parse("ab?c?")?;

  assert_eq!(Some(1), hir1.properties().minimum_len());

  assert_eq!(Some(3), hir1.properties().maximum_len());

  

  let hir2 = parse(r"[a&&b]")?;

  assert_eq!(None, hir2.properties().minimum_len());

  assert_eq!(None, hir2.properties().maximum_len());

  

  let hir3 = parse(r"wxy?z?")?;

  assert_eq!(Some(2), hir3.properties().minimum_len());

  assert_eq!(Some(4), hir3.properties().maximum_len());

  

  let unioned = Properties::union([

  	hir1.properties(),

  	hir2.properties(),

  	hir3.properties(),

  ]);

  assert_eq!(None, unioned.minimum_len());

  assert_eq!(None, unioned.maximum_len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  The maximum length can also be "poisoned" by a pattern that has no

  upper bound on the length of a match. The minimum length remains

  unaffected:

  

  ```rust

  use regex_syntax::{hir::Properties, parse};

  

  let hir1 = parse("ab?c?")?;

  assert_eq!(Some(1), hir1.properties().minimum_len());

  assert_eq!(Some(3), hir1.properties().maximum_len());

  

  let hir2 = parse(r"a+")?;

  assert_eq!(Some(1), hir2.properties().minimum_len());

  assert_eq!(None, hir2.properties().maximum_len());

  

  let hir3 = parse(r"wxy?z?")?;

  assert_eq!(Some(2), hir3.properties().minimum_len());

  assert_eq!(Some(4), hir3.properties().maximum_len());

  

  let unioned = Properties::union([

  	hir1.properties(),

  	hir2.properties(),

  	hir3.properties(),

  ]);

  assert_eq!(Some(1), unioned.minimum_len());

  assert_eq!(None, unioned.maximum_len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

#### Trait Implementations

##### `impl Any for Properties`

- <span id="properties-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Properties`

- <span id="properties-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Properties`

- <span id="properties-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Properties`

- <span id="properties-clone"></span>`fn clone(&self) -> Properties` — [`Properties`](#properties)

##### `impl CloneToUninit for Properties`

- <span id="properties-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Properties`

- <span id="properties-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Properties`

##### `impl<T> From for Properties`

- <span id="properties-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Properties`

- <span id="properties-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Properties`

- <span id="properties-partialeq-eq"></span>`fn eq(&self, other: &Properties) -> bool` — [`Properties`](#properties)

##### `impl StructuralPartialEq for Properties`

##### `impl ToOwned for Properties`

- <span id="properties-toowned-type-owned"></span>`type Owned = T`

- <span id="properties-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="properties-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Properties`

- <span id="properties-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="properties-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Properties`

- <span id="properties-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="properties-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PropertiesI`

```rust
struct PropertiesI {
    minimum_len: Option<usize>,
    maximum_len: Option<usize>,
    look_set: LookSet,
    look_set_prefix: LookSet,
    look_set_suffix: LookSet,
    look_set_prefix_any: LookSet,
    look_set_suffix_any: LookSet,
    utf8: bool,
    explicit_captures_len: usize,
    static_explicit_captures_len: Option<usize>,
    literal: bool,
    alternation_literal: bool,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1974-1987`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1974-L1987)*

The property definition. It is split out so that we can box it, and
there by make `Properties` use less stack size. This is kind-of important
because every HIR value has a `Properties` attached to it.

This does have the unfortunate consequence that creating any HIR value
always leads to at least one alloc for properties, but this is generally
true anyway (for pretty much all HirKinds except for look-arounds).

#### Trait Implementations

##### `impl Any for PropertiesI`

- <span id="propertiesi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PropertiesI`

- <span id="propertiesi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PropertiesI`

- <span id="propertiesi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PropertiesI`

- <span id="propertiesi-clone"></span>`fn clone(&self) -> PropertiesI` — [`PropertiesI`](#propertiesi)

##### `impl CloneToUninit for PropertiesI`

- <span id="propertiesi-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PropertiesI`

- <span id="propertiesi-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PropertiesI`

##### `impl<T> From for PropertiesI`

- <span id="propertiesi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PropertiesI`

- <span id="propertiesi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PropertiesI`

- <span id="propertiesi-partialeq-eq"></span>`fn eq(&self, other: &PropertiesI) -> bool` — [`PropertiesI`](#propertiesi)

##### `impl StructuralPartialEq for PropertiesI`

##### `impl ToOwned for PropertiesI`

- <span id="propertiesi-toowned-type-owned"></span>`type Owned = T`

- <span id="propertiesi-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="propertiesi-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PropertiesI`

- <span id="propertiesi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="propertiesi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PropertiesI`

- <span id="propertiesi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="propertiesi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LookSet`

```rust
struct LookSet {
    pub bits: u32,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2665-2676`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L2665-L2676)*

A set of look-around assertions.

This is useful for efficiently tracking look-around assertions. For
example, an [`Hir`](#hir) provides properties that return `LookSet`s.

#### Fields

- **`bits`**: `u32`

  The underlying representation this set is exposed to make it possible
  to store it somewhere efficiently. The representation is that
  of a bitset, where each assertion occupies bit `i` where `i =
  Look::as_repr()`.
  
  Note that users of this internal representation must permit the full
  range of `u16` values to be represented. For example, even if the
  current implementation only makes use of the 10 least significant bits,
  it may use more bits in a future semver compatible release.

#### Implementations

- <span id="lookset-empty"></span>`fn empty() -> LookSet` — [`LookSet`](#lookset)

  Create an empty set of look-around assertions.

- <span id="lookset-full"></span>`fn full() -> LookSet` — [`LookSet`](#lookset)

  Create a full set of look-around assertions.

  

  This set contains all possible look-around assertions.

- <span id="lookset-singleton"></span>`fn singleton(look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

  Create a look-around set containing the look-around assertion given.

  

  This is a convenience routine for creating an empty set and inserting

  one look-around assertions.

- <span id="lookset-len"></span>`fn len(self) -> usize`

  Returns the total number of look-around assertions in this set.

- <span id="lookset-is-empty"></span>`fn is_empty(self) -> bool`

  Returns true if and only if this set is empty.

- <span id="lookset-contains"></span>`fn contains(self, look: Look) -> bool` — [`Look`](#look)

  Returns true if and only if the given look-around assertion is in this

  set.

- <span id="lookset-contains-anchor"></span>`fn contains_anchor(&self) -> bool`

  Returns true if and only if this set contains any anchor assertions.

  This includes both "start/end of haystack" and "start/end of line."

- <span id="lookset-contains-anchor-haystack"></span>`fn contains_anchor_haystack(&self) -> bool`

  Returns true if and only if this set contains any "start/end of

  haystack" anchors. This doesn't include "start/end of line" anchors.

- <span id="lookset-contains-anchor-line"></span>`fn contains_anchor_line(&self) -> bool`

  Returns true if and only if this set contains any "start/end of line"

  anchors. This doesn't include "start/end of haystack" anchors. This

  includes both `\n` line anchors and CRLF (`\r\n`) aware line anchors.

- <span id="lookset-contains-anchor-lf"></span>`fn contains_anchor_lf(&self) -> bool`

  Returns true if and only if this set contains any "start/end of line"

  anchors that only treat `\n` as line terminators. This does not include

  haystack anchors or CRLF aware line anchors.

- <span id="lookset-contains-anchor-crlf"></span>`fn contains_anchor_crlf(&self) -> bool`

  Returns true if and only if this set contains any "start/end of line"

  anchors that are CRLF-aware. This doesn't include "start/end of

  haystack" or "start/end of line-feed" anchors.

- <span id="lookset-contains-word"></span>`fn contains_word(self) -> bool`

  Returns true if and only if this set contains any word boundary or

  negated word boundary assertions. This include both Unicode and ASCII

  word boundaries.

- <span id="lookset-contains-word-unicode"></span>`fn contains_word_unicode(self) -> bool`

  Returns true if and only if this set contains any Unicode word boundary

  or negated Unicode word boundary assertions.

- <span id="lookset-contains-word-ascii"></span>`fn contains_word_ascii(self) -> bool`

  Returns true if and only if this set contains any ASCII word boundary

  or negated ASCII word boundary assertions.

- <span id="lookset-iter"></span>`fn iter(self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

  Returns an iterator over all of the look-around assertions in this set.

- <span id="lookset-insert"></span>`fn insert(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

  Return a new set that is equivalent to the original, but with the given

  assertion added to it. If the assertion is already in the set, then the

  returned set is equivalent to the original.

- <span id="lookset-set-insert"></span>`fn set_insert(&mut self, look: Look)` — [`Look`](#look)

  Updates this set in place with the result of inserting the given

  assertion into this set.

- <span id="lookset-remove"></span>`fn remove(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

  Return a new set that is equivalent to the original, but with the given

  assertion removed from it. If the assertion is not in the set, then the

  returned set is equivalent to the original.

- <span id="lookset-set-remove"></span>`fn set_remove(&mut self, look: Look)` — [`Look`](#look)

  Updates this set in place with the result of removing the given

  assertion from this set.

- <span id="lookset-subtract"></span>`fn subtract(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

  Returns a new set that is the result of subtracting the given set from

  this set.

- <span id="lookset-set-subtract"></span>`fn set_subtract(&mut self, other: LookSet)` — [`LookSet`](#lookset)

  Updates this set in place with the result of subtracting the given set

  from this set.

- <span id="lookset-union"></span>`fn union(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

  Returns a new set that is the union of this and the one given.

- <span id="lookset-set-union"></span>`fn set_union(&mut self, other: LookSet)` — [`LookSet`](#lookset)

  Updates this set in place with the result of unioning it with the one

  given.

- <span id="lookset-intersect"></span>`fn intersect(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

  Returns a new set that is the intersection of this and the one given.

- <span id="lookset-set-intersect"></span>`fn set_intersect(&mut self, other: LookSet)` — [`LookSet`](#lookset)

  Updates this set in place with the result of intersecting it with the

  one given.

- <span id="lookset-read-repr"></span>`fn read_repr(slice: &[u8]) -> LookSet` — [`LookSet`](#lookset)

  Return a `LookSet` from the slice given as a native endian 32-bit

  integer.

  

  # Panics

  

  This panics if `slice.len() < 4`.

- <span id="lookset-write-repr"></span>`fn write_repr(self, slice: &mut [u8])`

  Write a `LookSet` as a native endian 32-bit integer to the beginning

  of the slice given.

  

  # Panics

  

  This panics if `slice.len() < 4`.

#### Trait Implementations

##### `impl Any for LookSet`

- <span id="lookset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LookSet`

- <span id="lookset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LookSet`

- <span id="lookset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LookSet`

- <span id="lookset-clone"></span>`fn clone(&self) -> LookSet` — [`LookSet`](#lookset)

##### `impl CloneToUninit for LookSet`

- <span id="lookset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LookSet`

##### `impl Debug for LookSet`

- <span id="lookset-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for LookSet`

- <span id="lookset-default"></span>`fn default() -> LookSet` — [`LookSet`](#lookset)

##### `impl Eq for LookSet`

##### `impl<T> From for LookSet`

- <span id="lookset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LookSet`

- <span id="lookset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LookSet`

- <span id="lookset-partialeq-eq"></span>`fn eq(&self, other: &LookSet) -> bool` — [`LookSet`](#lookset)

##### `impl StructuralPartialEq for LookSet`

##### `impl ToOwned for LookSet`

- <span id="lookset-toowned-type-owned"></span>`type Owned = T`

- <span id="lookset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lookset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LookSet`

- <span id="lookset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lookset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LookSet`

- <span id="lookset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lookset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LookSetIter`

```rust
struct LookSetIter {
    set: LookSet,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2916-2918`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L2916-L2918)*

An iterator over all look-around assertions in a [`LookSet`](#lookset).

This iterator is created by `LookSet::iter`.

#### Trait Implementations

##### `impl Any for LookSetIter`

- <span id="looksetiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LookSetIter`

- <span id="looksetiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LookSetIter`

- <span id="looksetiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LookSetIter`

- <span id="looksetiter-clone"></span>`fn clone(&self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

##### `impl CloneToUninit for LookSetIter`

- <span id="looksetiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for LookSetIter`

- <span id="looksetiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LookSetIter`

- <span id="looksetiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LookSetIter`

- <span id="looksetiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for LookSetIter`

- <span id="looksetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="looksetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="looksetiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LookSetIter`

- <span id="looksetiter-iterator-type-item"></span>`type Item = Look`

- <span id="looksetiter-iterator-next"></span>`fn next(&mut self) -> Option<Look>` — [`Look`](#look)

##### `impl ToOwned for LookSetIter`

- <span id="looksetiter-toowned-type-owned"></span>`type Owned = T`

- <span id="looksetiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="looksetiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LookSetIter`

- <span id="looksetiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="looksetiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LookSetIter`

- <span id="looksetiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="looksetiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    UnicodeNotAllowed,
    InvalidUtf8,
    InvalidLineTerminator,
    UnicodePropertyNotFound,
    UnicodePropertyValueNotFound,
    UnicodePerlClassNotFound,
    UnicodeCaseUnavailable,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:84-108`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L84-L108)*

The type of an error that occurred while building an `Hir`.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

#### Variants

- **`UnicodeNotAllowed`**

  This error occurs when a Unicode feature is used when Unicode
  support is disabled. For example `(?-u:\pL)` would trigger this error.

- **`InvalidUtf8`**

  This error occurs when translating a pattern that could match a byte
  sequence that isn't UTF-8 and `utf8` was enabled.

- **`InvalidLineTerminator`**

  This error occurs when one uses a non-ASCII byte for a line terminator,
  but where Unicode mode is enabled and UTF-8 mode is disabled.

- **`UnicodePropertyNotFound`**

  This occurs when an unrecognized Unicode property name could not
  be found.

- **`UnicodePropertyValueNotFound`**

  This occurs when an unrecognized Unicode property value could not
  be found.

- **`UnicodePerlClassNotFound`**

  This occurs when a Unicode-aware Perl character class (`\w`, `\s` or
  `\d`) could not be found. This can occur when the `unicode-perl`
  crate feature is not enabled.

- **`UnicodeCaseUnavailable`**

  This occurs when the Unicode simple case mapping tables are not
  available, and the regular expression required Unicode aware case
  insensitivity.

#### Trait Implementations

##### `impl Any for ErrorKind`

- <span id="errorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorKind`

- <span id="errorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorKind`

- <span id="errorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl CloneToUninit for ErrorKind`

- <span id="errorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ErrorKind`

- <span id="errorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl<T> From for ErrorKind`

- <span id="errorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorKind`

- <span id="errorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-partialeq-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

##### `impl ToOwned for ErrorKind`

- <span id="errorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="errorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="errorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ErrorKind`

- <span id="errorkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ErrorKind`

- <span id="errorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorKind`

- <span id="errorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HirKind`

```rust
enum HirKind {
    Empty,
    Literal(Literal),
    Class(Class),
    Look(Look),
    Repetition(Repetition),
    Capture(Capture),
    Concat(alloc::vec::Vec<Hir>),
    Alternation(alloc::vec::Vec<Hir>),
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:717-752`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L717-L752)*

The underlying kind of an arbitrary [`Hir`](#hir) expression.

An `HirKind` is principally useful for doing case analysis on the type
of a regular expression. If you're looking to build new `Hir` values,
then you _must_ use the smart constructors defined on `Hir`, like
`Hir::repetition`, to build new `Hir` values. The API intentionally does
not expose any way of building an `Hir` directly from an `HirKind`.

#### Variants

- **`Empty`**

  The empty regular expression, which matches everything, including the
  empty string.

- **`Literal`**

  A literal string that matches exactly these bytes.

- **`Class`**

  A single character class that matches any of the characters in the
  class. A class can either consist of Unicode scalar values as
  characters, or it can use bytes.
  
  A class may be empty. In which case, it matches nothing.

- **`Look`**

  A look-around assertion. A look-around match always has zero length.

- **`Repetition`**

  A repetition operation applied to a sub-expression.

- **`Capture`**

  A capturing group, which contains a sub-expression.

- **`Concat`**

  A concatenation of expressions.
  
  A concatenation matches only if each of its sub-expressions match one
  after the other.
  
  Concatenations are guaranteed by `Hir`'s smart constructors to always
  have at least two sub-expressions.

- **`Alternation`**

  An alternation of expressions.
  
  An alternation matches only if at least one of its sub-expressions
  match. If multiple sub-expressions match, then the leftmost is
  preferred.
  
  Alternations are guaranteed by `Hir`'s smart constructors to always
  have at least two sub-expressions.

#### Implementations

- <span id="hirkind-subs"></span>`fn subs(&self) -> &[Hir]` — [`Hir`](#hir)

  Returns a slice of this kind's sub-expressions, if any.

#### Trait Implementations

##### `impl Any for HirKind`

- <span id="hirkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HirKind`

- <span id="hirkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HirKind`

- <span id="hirkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for HirKind`

- <span id="hirkind-clone"></span>`fn clone(&self) -> HirKind` — [`HirKind`](#hirkind)

##### `impl CloneToUninit for HirKind`

- <span id="hirkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for HirKind`

- <span id="hirkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HirKind`

##### `impl<T> From for HirKind`

- <span id="hirkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HirKind`

- <span id="hirkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for HirKind`

- <span id="hirkind-partialeq-eq"></span>`fn eq(&self, other: &HirKind) -> bool` — [`HirKind`](#hirkind)

##### `impl StructuralPartialEq for HirKind`

##### `impl ToOwned for HirKind`

- <span id="hirkind-toowned-type-owned"></span>`type Owned = T`

- <span id="hirkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hirkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for HirKind`

- <span id="hirkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hirkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HirKind`

- <span id="hirkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hirkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Class`

```rust
enum Class {
    Unicode(ClassUnicode),
    Bytes(ClassBytes),
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:830-836`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L830-L836)*

The high-level intermediate representation of a character class.

A character class corresponds to a set of characters. A character is either
defined by a Unicode scalar value or a byte.

A character class, regardless of its character type, is represented by a
sequence of non-overlapping non-adjacent ranges of characters.

There are no guarantees about which class variant is used. Generally
speaking, the Unicode variant is used whenever a class needs to contain
non-ASCII Unicode scalar values. But the Unicode variant can be used even
when Unicode mode is disabled. For example, at the time of writing, the
regex `(?-u:a|\xc2\xa0)` will compile down to HIR for the Unicode class
`[a\u00A0]` due to optimizations.

Note that `Bytes` variant may be produced even when it exclusively matches
valid UTF-8. This is because a `Bytes` variant represents an intention by
the author of the regular expression to disable Unicode mode, which in turn
impacts the semantics of case insensitive matching. For example, `(?i)k`
and `(?i-u)k` will not match the same set of strings.

#### Variants

- **`Unicode`**

  A set of characters represented by Unicode scalar values.

- **`Bytes`**

  A set of characters represented by arbitrary bytes (one byte per
  character).

#### Implementations

- <span id="class-case-fold-simple"></span>`fn case_fold_simple(&mut self)`

  Apply Unicode simple case folding to this character class, in place.

  The character class will be expanded to include all simple case folded

  character variants.

  

  If this is a byte oriented character class, then this will be limited

  to the ASCII ranges `A-Z` and `a-z`.

  

  # Panics

  

  This routine panics when the case mapping data necessary for this

  routine to complete is unavailable. This occurs when the `unicode-case`

  feature is not enabled and the underlying class is Unicode oriented.

  

  Callers should prefer using `try_case_fold_simple` instead, which will

  return an error instead of panicking.

- <span id="class-try-case-fold-simple"></span>`fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError>` — [`CaseFoldError`](../unicode/index.md#casefolderror)

  Apply Unicode simple case folding to this character class, in place.

  The character class will be expanded to include all simple case folded

  character variants.

  

  If this is a byte oriented character class, then this will be limited

  to the ASCII ranges `A-Z` and `a-z`.

  

  # Error

  

  This routine returns an error when the case mapping data necessary

  for this routine to complete is unavailable. This occurs when the

  `unicode-case` feature is not enabled and the underlying class is

  Unicode oriented.

- <span id="class-negate"></span>`fn negate(&mut self)`

  Negate this character class in place.

  

  After completion, this character class will contain precisely the

  characters that weren't previously in the class.

- <span id="class-is-utf8"></span>`fn is_utf8(&self) -> bool`

  Returns true if and only if this character class will only ever match

  valid UTF-8.

  

  A character class can match invalid UTF-8 only when the following

  conditions are met:

  

  1. The translator was configured to permit generating an expression

     that can match invalid UTF-8. (By default, this is disabled.)

  2. Unicode mode (via the `u` flag) was disabled either in the concrete

     syntax or in the parser builder. By default, Unicode mode is

     enabled.

- <span id="class-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

  Returns the length, in bytes, of the smallest string matched by this

  character class.

  

  For non-empty byte oriented classes, this always returns `1`. For

  non-empty Unicode oriented classes, this can return `1`, `2`, `3` or

  `4`. For empty classes, `None` is returned. It is impossible for `0` to

  be returned.

  

  # Example

  

  This example shows some examples of regexes and their corresponding

  minimum length, if any.

  

  ```rust

  use regex_syntax::{hir::Properties, parse};

  

  // The empty string has a min length of 0.

  let hir = parse(r"")?;

  assert_eq!(Some(0), hir.properties().minimum_len());

  // As do other types of regexes that only match the empty string.

  let hir = parse(r"^$\b\B")?;

  assert_eq!(Some(0), hir.properties().minimum_len());

  // A regex that can match the empty string but match more is still 0.

  let hir = parse(r"a*")?;

  assert_eq!(Some(0), hir.properties().minimum_len());

  // A regex that matches nothing has no minimum defined.

  let hir = parse(r"[a&&b]")?;

  assert_eq!(None, hir.properties().minimum_len());

  // Character classes usually have a minimum length of 1.

  let hir = parse(r"\w")?;

  assert_eq!(Some(1), hir.properties().minimum_len());

  // But sometimes Unicode classes might be bigger!

  let hir = parse(r"\p{Cyrillic}")?;

  assert_eq!(Some(2), hir.properties().minimum_len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="class-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

  Returns the length, in bytes, of the longest string matched by this

  character class.

  

  For non-empty byte oriented classes, this always returns `1`. For

  non-empty Unicode oriented classes, this can return `1`, `2`, `3` or

  `4`. For empty classes, `None` is returned. It is impossible for `0` to

  be returned.

  

  # Example

  

  This example shows some examples of regexes and their corresponding

  maximum length, if any.

  

  ```rust

  use regex_syntax::{hir::Properties, parse};

  

  // The empty string has a max length of 0.

  let hir = parse(r"")?;

  assert_eq!(Some(0), hir.properties().maximum_len());

  // As do other types of regexes that only match the empty string.

  let hir = parse(r"^$\b\B")?;

  assert_eq!(Some(0), hir.properties().maximum_len());

  // A regex that matches nothing has no maximum defined.

  let hir = parse(r"[a&&b]")?;

  assert_eq!(None, hir.properties().maximum_len());

  // Bounded repeats work as you expect.

  let hir = parse(r"x{2,10}")?;

  assert_eq!(Some(10), hir.properties().maximum_len());

  // An unbounded repeat means there is no maximum.

  let hir = parse(r"x{2,}")?;

  assert_eq!(None, hir.properties().maximum_len());

  // With Unicode enabled, \w can match up to 4 bytes!

  let hir = parse(r"\w")?;

  assert_eq!(Some(4), hir.properties().maximum_len());

  // Without Unicode enabled, \w matches at most 1 byte.

  let hir = parse(r"(?-u)\w")?;

  assert_eq!(Some(1), hir.properties().maximum_len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="class-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if and only if this character class is empty. That is,

  it has no elements.

  

  An empty character can never match anything, including an empty string.

- <span id="class-literal"></span>`fn literal(&self) -> Option<Vec<u8>>`

  If this class consists of exactly one element (whether a codepoint or a

  byte), then return it as a literal byte string.

  

  If this class is empty or contains more than one element, then `None`

  is returned.

#### Trait Implementations

##### `impl Any for Class`

- <span id="class-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Class`

- <span id="class-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Class`

- <span id="class-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Class`

- <span id="class-clone"></span>`fn clone(&self) -> Class` — [`Class`](#class)

##### `impl CloneToUninit for Class`

- <span id="class-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Class`

- <span id="class-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Class`

##### `impl<T> From for Class`

- <span id="class-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Class`

- <span id="class-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Class`

- <span id="class-partialeq-eq"></span>`fn eq(&self, other: &Class) -> bool` — [`Class`](#class)

##### `impl StructuralPartialEq for Class`

##### `impl ToOwned for Class`

- <span id="class-toowned-type-owned"></span>`type Owned = T`

- <span id="class-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="class-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Class`

- <span id="class-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="class-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Class`

- <span id="class-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="class-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Look`

```rust
enum Look {
    Start,
    End,
    StartLF,
    EndLF,
    StartCRLF,
    EndCRLF,
    WordAscii,
    WordAsciiNegate,
    WordUnicode,
    WordUnicodeNegate,
    WordStartAscii,
    WordEndAscii,
    WordStartUnicode,
    WordEndUnicode,
    WordStartHalfAscii,
    WordEndHalfAscii,
    WordStartHalfUnicode,
    WordEndHalfUnicode,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1613-1686`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1613-L1686)*

The high-level intermediate representation for a look-around assertion.

An assertion match is always zero-length. Also called an "empty match."

#### Variants

- **`Start`**

  Match the beginning of text. Specifically, this matches at the starting
  position of the input.

- **`End`**

  Match the end of text. Specifically, this matches at the ending
  position of the input.

- **`StartLF`**

  Match the beginning of a line or the beginning of text. Specifically,
  this matches at the starting position of the input, or at the position
  immediately following a `\n` character.

- **`EndLF`**

  Match the end of a line or the end of text. Specifically, this matches
  at the end position of the input, or at the position immediately
  preceding a `\n` character.

- **`StartCRLF`**

  Match the beginning of a line or the beginning of text. Specifically,
  this matches at the starting position of the input, or at the position
  immediately following either a `\r` or `\n` character, but never after
  a `\r` when a `\n` follows.

- **`EndCRLF`**

  Match the end of a line or the end of text. Specifically, this matches
  at the end position of the input, or at the position immediately
  preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
  precedes it.

- **`WordAscii`**

  Match an ASCII-only word boundary. That is, this matches a position
  where the left adjacent character and right adjacent character
  correspond to a word and non-word or a non-word and word character.

- **`WordAsciiNegate`**

  Match an ASCII-only negation of a word boundary.

- **`WordUnicode`**

  Match a Unicode-aware word boundary. That is, this matches a position
  where the left adjacent character and right adjacent character
  correspond to a word and non-word or a non-word and word character.

- **`WordUnicodeNegate`**

  Match a Unicode-aware negation of a word boundary.

- **`WordStartAscii`**

  Match the start of an ASCII-only word boundary. That is, this matches a
  position at either the beginning of the haystack or where the previous
  character is not a word character and the following character is a word
  character.

- **`WordEndAscii`**

  Match the end of an ASCII-only word boundary. That is, this matches
  a position at either the end of the haystack or where the previous
  character is a word character and the following character is not a word
  character.

- **`WordStartUnicode`**

  Match the start of a Unicode word boundary. That is, this matches a
  position at either the beginning of the haystack or where the previous
  character is not a word character and the following character is a word
  character.

- **`WordEndUnicode`**

  Match the end of a Unicode word boundary. That is, this matches a
  position at either the end of the haystack or where the previous
  character is a word character and the following character is not a word
  character.

- **`WordStartHalfAscii`**

  Match the start half of an ASCII-only word boundary. That is, this
  matches a position at either the beginning of the haystack or where the
  previous character is not a word character.

- **`WordEndHalfAscii`**

  Match the end half of an ASCII-only word boundary. That is, this
  matches a position at either the end of the haystack or where the
  following character is not a word character.

- **`WordStartHalfUnicode`**

  Match the start half of a Unicode word boundary. That is, this matches
  a position at either the beginning of the haystack or where the
  previous character is not a word character.

- **`WordEndHalfUnicode`**

  Match the end half of a Unicode word boundary. That is, this matches
  a position at either the end of the haystack or where the following
  character is not a word character.

#### Implementations

- <span id="look-reversed"></span>`const fn reversed(self) -> Look` — [`Look`](#look)

  Flip the look-around assertion to its equivalent for reverse searches.

  For example, `StartLF` gets translated to `EndLF`.

  

  Some assertions, such as `WordUnicode`, remain the same since they

  match the same positions regardless of the direction of the search.

- <span id="look-as-repr"></span>`const fn as_repr(self) -> u32`

  Return the underlying representation of this look-around enumeration

  as an integer. Giving the return value to the `Look::from_repr`

  constructor is guaranteed to return the same look-around variant that

  one started with within a semver compatible release of this crate.

- <span id="look-from-repr"></span>`const fn from_repr(repr: u32) -> Option<Look>` — [`Look`](#look)

  Given the underlying representation of a `Look` value, return the

  corresponding `Look` value if the representation is valid. Otherwise

  `None` is returned.

- <span id="look-as-char"></span>`const fn as_char(self) -> char`

  Returns a convenient single codepoint representation of this

  look-around assertion. Each assertion is guaranteed to be represented

  by a distinct character.

  

  This is useful for succinctly representing a look-around assertion in

  human friendly but succinct output intended for a programmer working on

  regex internals.

#### Trait Implementations

##### `impl Any for Look`

- <span id="look-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Look`

- <span id="look-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Look`

- <span id="look-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Look`

- <span id="look-clone"></span>`fn clone(&self) -> Look` — [`Look`](#look)

##### `impl CloneToUninit for Look`

- <span id="look-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Look`

##### `impl Debug for Look`

- <span id="look-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Look`

##### `impl<T> From for Look`

- <span id="look-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Look`

- <span id="look-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Look`

- <span id="look-partialeq-eq"></span>`fn eq(&self, other: &Look) -> bool` — [`Look`](#look)

##### `impl StructuralPartialEq for Look`

##### `impl ToOwned for Look`

- <span id="look-toowned-type-owned"></span>`type Owned = T`

- <span id="look-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="look-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Look`

- <span id="look-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="look-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Look`

- <span id="look-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="look-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Dot`

```rust
enum Dot {
    AnyChar,
    AnyByte,
    AnyCharExcept(char),
    AnyCharExceptLF,
    AnyCharExceptCRLF,
    AnyByteExcept(u8),
    AnyByteExceptLF,
    AnyByteExceptCRLF,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1860-1909`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L1860-L1909)*

A type describing the different flavors of `.`.

This type is meant to be used with `Hir::dot`, which is a convenience
routine for building HIR values derived from the `.` regex.

#### Variants

- **`AnyChar`**

  Matches the UTF-8 encoding of any Unicode scalar value.
  
  This is equivalent to `(?su:.)` and also `\p{any}`.

- **`AnyByte`**

  Matches any byte value.
  
  This is equivalent to `(?s-u:.)` and also `(?-u:[\x00-\xFF])`.

- **`AnyCharExcept`**

  Matches the UTF-8 encoding of any Unicode scalar value except for the
  `char` given.
  
  This is equivalent to using `(?u-s:.)` with the line terminator set
  to a particular ASCII byte. (Because of peculiarities in the regex
  engines, a line terminator must be a single byte. It follows that when
  UTF-8 mode is enabled, this single byte must also be a Unicode scalar
  value. That is, ti must be ASCII.)
  
  (This and `AnyCharExceptLF` both exist because of legacy reasons.
  `AnyCharExceptLF` will be dropped in the next breaking change release.)

- **`AnyCharExceptLF`**

  Matches the UTF-8 encoding of any Unicode scalar value except for `\n`.
  
  This is equivalent to `(?u-s:.)` and also `[\p{any}--\n]`.

- **`AnyCharExceptCRLF`**

  Matches the UTF-8 encoding of any Unicode scalar value except for `\r`
  and `\n`.
  
  This is equivalent to `(?uR-s:.)` and also `[\p{any}--\r\n]`.

- **`AnyByteExcept`**

  Matches any byte value except for the `u8` given.
  
  This is equivalent to using `(?-us:.)` with the line terminator set
  to a particular ASCII byte. (Because of peculiarities in the regex
  engines, a line terminator must be a single byte. It follows that when
  UTF-8 mode is enabled, this single byte must also be a Unicode scalar
  value. That is, ti must be ASCII.)
  
  (This and `AnyByteExceptLF` both exist because of legacy reasons.
  `AnyByteExceptLF` will be dropped in the next breaking change release.)

- **`AnyByteExceptLF`**

  Matches any byte value except for `\n`.
  
  This is equivalent to `(?-su:.)` and also `(?-u:[[\x00-\xFF]--\n])`.

- **`AnyByteExceptCRLF`**

  Matches any byte value except for `\r` and `\n`.
  
  This is equivalent to `(?R-su:.)` and also `(?-u:[[\x00-\xFF]--\r\n])`.

#### Trait Implementations

##### `impl Any for Dot`

- <span id="dot-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dot`

- <span id="dot-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dot`

- <span id="dot-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Dot`

- <span id="dot-clone"></span>`fn clone(&self) -> Dot` — [`Dot`](#dot)

##### `impl CloneToUninit for Dot`

- <span id="dot-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Dot`

##### `impl Debug for Dot`

- <span id="dot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Dot`

##### `impl<T> From for Dot`

- <span id="dot-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Dot`

- <span id="dot-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Dot`

- <span id="dot-partialeq-eq"></span>`fn eq(&self, other: &Dot) -> bool` — [`Dot`](#dot)

##### `impl StructuralPartialEq for Dot`

##### `impl ToOwned for Dot`

- <span id="dot-toowned-type-owned"></span>`type Owned = T`

- <span id="dot-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dot-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Dot`

- <span id="dot-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dot-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dot`

- <span id="dot-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dot-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Visitor`

```rust
trait Visitor { ... }
```

*Defined in [`regex-syntax-0.8.8/src/hir/visitor.rs:15-49`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/visitor.rs#L15-L49)*

A trait for visiting the high-level IR (HIR) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on a high-level intermediate representation of a regular
expression without necessarily using recursion. In particular, this permits
callers to do case analysis with constant stack usage, which can be
important since the size of an HIR may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`](visitor/index.md) function.

#### Associated Types

- `type Output`

- `type Err`

#### Required Methods

- `fn finish(self) -> Result<<Self as >::Output, <Self as >::Err>`

  All implementors of `Visitor` must provide a `finish` method, which

#### Provided Methods

- `fn start(&mut self)`

  This method is called before beginning traversal of the HIR.

- `fn visit_pre(&mut self, _hir: &Hir) -> Result<(), <Self as >::Err>`

  This method is called on an `Hir` before descending into child `Hir`

- `fn visit_post(&mut self, _hir: &Hir) -> Result<(), <Self as >::Err>`

  This method is called on an `Hir` after descending all of its child

- `fn visit_alternation_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of an alternation.

- `fn visit_concat_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of a concatenation.

#### Implementors

- [`Writer`](print/index.md#writer)

## Functions

### `visit`

```rust
fn visit<V: Visitor>(hir: &crate::hir::Hir, visitor: V) -> Result<<V as >::Output, <V as >::Err>
```

*Defined in [`regex-syntax-0.8.8/src/hir/visitor.rs:65-67`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/visitor.rs#L65-L67)*

Executes an implementation of `Visitor` in constant stack space.

This function will visit every node in the given `Hir` while calling
appropriate methods provided by the [`Visitor`](visitor/index.md) trait.

The primary use case for this method is when one wants to perform case
analysis over an `Hir` without using a stack size proportional to the depth
of the `Hir`. Namely, this method will instead use constant stack space,
but will use heap space proportional to the size of the `Hir`. This may be
desirable in cases where the size of `Hir` is proportional to end user
input.

If the visitor returns an error at any point, then visiting is stopped and
the error is returned.

### `class_chars`

```rust
fn class_chars(hirs: &[Hir]) -> Option<Class>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2940-2954`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L2940-L2954)*

Given a sequence of HIR values where each value corresponds to a Unicode
class (or an all-ASCII byte class), return a single Unicode class
corresponding to the union of the classes found.

### `class_bytes`

```rust
fn class_bytes(hirs: &[Hir]) -> Option<Class>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2959-2973`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L2959-L2973)*

Given a sequence of HIR values where each value corresponds to a byte class
(or an all-ASCII Unicode class), return a single byte class corresponding
to the union of the classes found.

### `singleton_chars`

```rust
fn singleton_chars(hirs: &[Hir]) -> Option<alloc::vec::Vec<char>>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2978-2996`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L2978-L2996)*

Given a sequence of HIR values where each value corresponds to a literal
that is a single `char`, return that sequence of `char`s. Otherwise return
None. No deduplication is done.

### `singleton_bytes`

```rust
fn singleton_bytes(hirs: &[Hir]) -> Option<alloc::vec::Vec<u8>>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:3001-3014`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L3001-L3014)*

Given a sequence of HIR values where each value corresponds to a literal
that is a single byte, return that sequence of bytes. Otherwise return
None. No deduplication is done.

### `lift_common_prefix`

```rust
fn lift_common_prefix(hirs: alloc::vec::Vec<Hir>) -> Result<Hir, alloc::vec::Vec<Hir>>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:3027-3073`](../../../.source_1765633015/regex-syntax-0.8.8/src/hir/mod.rs#L3027-L3073)*

Looks for a common prefix in the list of alternation branches given. If one
is found, then an equivalent but (hopefully) simplified Hir is returned.
Otherwise, the original given list of branches is returned unmodified.

This is not quite as good as it could be. Right now, it requires that
all branches are 'Concat' expressions. It also doesn't do well with
literals. For example, given 'foofoo|foobar', it will not refactor it to
'foo(?:foo|bar)' because literals are flattened into their own special
concatenation. (One wonders if perhaps 'Literal' should be a single atom
instead of a string of bytes because of this. Otherwise, handling the
current representation in this routine will be pretty gnarly. Sigh.)

