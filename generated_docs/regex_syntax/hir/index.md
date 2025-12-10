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
  - [`class_chars`](#class_chars)
  - [`class_bytes`](#class_bytes)
  - [`singleton_chars`](#singleton_chars)
  - [`singleton_bytes`](#singleton_bytes)
  - [`lift_common_prefix`](#lift_common_prefix)

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
| [`class_chars`](#class_chars) | fn | Given a sequence of HIR values where each value corresponds to a Unicode class (or an all-ASCII byte class), return a single Unicode class corresponding to the union of the classes found. |
| [`class_bytes`](#class_bytes) | fn | Given a sequence of HIR values where each value corresponds to a byte class (or an all-ASCII Unicode class), return a single byte class corresponding to the union of the classes found. |
| [`singleton_chars`](#singleton_chars) | fn | Given a sequence of HIR values where each value corresponds to a literal that is a single `char`, return that sequence of `char`s. |
| [`singleton_bytes`](#singleton_bytes) | fn | Given a sequence of HIR values where each value corresponds to a literal that is a single byte, return that sequence of bytes. |
| [`lift_common_prefix`](#lift_common_prefix) | fn | Looks for a common prefix in the list of alternation branches given. |

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

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:31`](../../../.source_1765210505/regex-syntax-0.8.8/src/unicode.rs#L31)*

An error that occurs when Unicode-aware simple case folding fails.

This error can occur when the case mapping tables necessary for Unicode
aware case folding are unavailable. This only occurs when the
`unicode-case` feature is disabled. (The feature is enabled by default.)

#### Trait Implementations

##### `impl Debug for CaseFoldError`

- <span id="casefolderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CaseFoldError`

- <span id="casefolderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CaseFoldError`

##### `impl ToString for CaseFoldError`

- <span id="casefolderror-to-string"></span>`fn to_string(&self) -> String`

### `Error`

```rust
struct Error {
    kind: ErrorKind,
    pattern: alloc::string::String,
    span: crate::ast::Span,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:49-57`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L49-L57)*

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

- <span id="error-pattern"></span>`fn pattern(&self) -> &str`

- <span id="error-span"></span>`fn span(&self) -> &Span` — [`Span`](../ast/index.md#span)

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `Hir`

```rust
struct Hir {
    kind: HirKind,
    props: Properties,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:205-210`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L205-L210)*

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

- <span id="hir-into-kind"></span>`fn into_kind(self) -> HirKind` — [`HirKind`](#hirkind)

- <span id="hir-properties"></span>`fn properties(&self) -> &Properties` — [`Properties`](#properties)

- <span id="hir-into-parts"></span>`fn into_parts(self) -> (HirKind, Properties)` — [`HirKind`](#hirkind), [`Properties`](#properties)

#### Trait Implementations

##### `impl Clone for Hir`

- <span id="hir-clone"></span>`fn clone(&self) -> Hir` — [`Hir`](#hir)

##### `impl Debug for Hir`

- <span id="hir-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for Hir`

- <span id="hir-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Drop for Hir`

- <span id="hir-drop"></span>`fn drop(&mut self)`

##### `impl Eq for Hir`

##### `impl PartialEq for Hir`

- <span id="hir-eq"></span>`fn eq(&self, other: &Hir) -> bool` — [`Hir`](#hir)

##### `impl StructuralPartialEq for Hir`

##### `impl ToString for Hir`

- <span id="hir-to-string"></span>`fn to_string(&self) -> String`

### `Literal`

```rust
struct Literal(alloc::boxed::Box<[u8]>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:801`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L801)*

The high-level intermediate representation of a literal.

A literal corresponds to `0` or more bytes that should be matched
literally. The smart constructors defined on `Hir` will automatically
concatenate adjacent literals into one literal, and will even automatically
replace empty literals with `Hir::empty()`.

Note that despite a literal being represented by a sequence of bytes, its
`Debug` implementation will attempt to print it as a normal string. (That
is, not a sequence of decimal numbers.)

#### Trait Implementations

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- <span id="literal-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Literal`

##### `impl PartialEq for Literal`

- <span id="literal-eq"></span>`fn eq(&self, other: &Literal) -> bool` — [`Literal`](#literal)

##### `impl StructuralPartialEq for Literal`

### `ClassUnicode`

```rust
struct ClassUnicode {
    set: crate::hir::interval::IntervalSet<ClassUnicodeRange>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1051-1053`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1051-L1053)*

A set of characters represented by Unicode scalar values.

#### Implementations

- <span id="classunicode-new"></span>`fn new<I>(ranges: I) -> ClassUnicode` — [`ClassUnicode`](#classunicode)

- <span id="classunicode-empty"></span>`fn empty() -> ClassUnicode` — [`ClassUnicode`](#classunicode)

- <span id="classunicode-push"></span>`fn push(&mut self, range: ClassUnicodeRange)` — [`ClassUnicodeRange`](#classunicoderange)

- <span id="classunicode-iter"></span>`fn iter(&self) -> ClassUnicodeIter<'_>` — [`ClassUnicodeIter`](#classunicodeiter)

- <span id="classunicode-ranges"></span>`fn ranges(&self) -> &[ClassUnicodeRange]` — [`ClassUnicodeRange`](#classunicoderange)

- <span id="classunicode-case-fold-simple"></span>`fn case_fold_simple(&mut self)`

- <span id="classunicode-try-case-fold-simple"></span>`fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError>` — [`CaseFoldError`](../unicode/index.md#casefolderror)

- <span id="classunicode-negate"></span>`fn negate(&mut self)`

- <span id="classunicode-union"></span>`fn union(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

- <span id="classunicode-intersect"></span>`fn intersect(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

- <span id="classunicode-difference"></span>`fn difference(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

- <span id="classunicode-symmetric-difference"></span>`fn symmetric_difference(&mut self, other: &ClassUnicode)` — [`ClassUnicode`](#classunicode)

- <span id="classunicode-is-ascii"></span>`fn is_ascii(&self) -> bool`

- <span id="classunicode-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

- <span id="classunicode-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

- <span id="classunicode-literal"></span>`fn literal(&self) -> Option<Vec<u8>>`

- <span id="classunicode-to-byte-class"></span>`fn to_byte_class(&self) -> Option<ClassBytes>` — [`ClassBytes`](#classbytes)

#### Trait Implementations

##### `impl Clone for ClassUnicode`

- <span id="classunicode-clone"></span>`fn clone(&self) -> ClassUnicode` — [`ClassUnicode`](#classunicode)

##### `impl Debug for ClassUnicode`

- <span id="classunicode-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicode`

##### `impl PartialEq for ClassUnicode`

- <span id="classunicode-eq"></span>`fn eq(&self, other: &ClassUnicode) -> bool` — [`ClassUnicode`](#classunicode)

##### `impl StructuralPartialEq for ClassUnicode`

### `ClassUnicodeIter<'a>`

```rust
struct ClassUnicodeIter<'a>(crate::hir::interval::IntervalSetIter<'a, ClassUnicodeRange>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1226`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1226)*

An iterator over all ranges in a Unicode character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl Debug for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="classunicodeiter-type-intoiter"></span>`type IntoIter = I`

- <span id="classunicodeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ClassUnicodeIter<'a>`

- <span id="classunicodeiter-type-item"></span>`type Item = &'a ClassUnicodeRange`

- <span id="classunicodeiter-next"></span>`fn next(&mut self) -> Option<&'a ClassUnicodeRange>` — [`ClassUnicodeRange`](#classunicoderange)

### `ClassUnicodeRange`

```rust
struct ClassUnicodeRange {
    start: char,
    end: char,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1241-1244`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1241-L1244)*

A single range of characters represented by Unicode scalar values.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- <span id="classunicoderange-new"></span>`fn new(start: char, end: char) -> ClassUnicodeRange` — [`ClassUnicodeRange`](#classunicoderange)

- <span id="classunicoderange-start"></span>`fn start(&self) -> char`

- <span id="classunicoderange-end"></span>`fn end(&self) -> char`

- <span id="classunicoderange-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for ClassUnicodeRange`

- <span id="classunicoderange-clone"></span>`fn clone(&self) -> ClassUnicodeRange` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl Copy for ClassUnicodeRange`

##### `impl Debug for ClassUnicodeRange`

- <span id="classunicoderange-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for ClassUnicodeRange`

- <span id="classunicoderange-default"></span>`fn default() -> ClassUnicodeRange` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl Eq for ClassUnicodeRange`

##### `impl Interval for ClassUnicodeRange`

- <span id="classunicoderange-type-bound"></span>`type Bound = char`

- <span id="classunicoderange-lower"></span>`fn lower(&self) -> char`

- <span id="classunicoderange-upper"></span>`fn upper(&self) -> char`

- <span id="classunicoderange-set-lower"></span>`fn set_lower(&mut self, bound: char)`

- <span id="classunicoderange-set-upper"></span>`fn set_upper(&mut self, bound: char)`

- <span id="classunicoderange-case-fold-simple"></span>`fn case_fold_simple(&self, ranges: &mut Vec<ClassUnicodeRange>) -> Result<(), unicode::CaseFoldError>` — [`ClassUnicodeRange`](#classunicoderange), [`CaseFoldError`](../unicode/index.md#casefolderror)

##### `impl Ord for ClassUnicodeRange`

- <span id="classunicoderange-cmp"></span>`fn cmp(&self, other: &ClassUnicodeRange) -> cmp::Ordering` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl PartialEq for ClassUnicodeRange`

- <span id="classunicoderange-eq"></span>`fn eq(&self, other: &ClassUnicodeRange) -> bool` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl PartialOrd for ClassUnicodeRange`

- <span id="classunicoderange-partial-cmp"></span>`fn partial_cmp(&self, other: &ClassUnicodeRange) -> option::Option<cmp::Ordering>` — [`ClassUnicodeRange`](#classunicoderange)

##### `impl StructuralPartialEq for ClassUnicodeRange`

### `ClassBytes`

```rust
struct ClassBytes {
    set: crate::hir::interval::IntervalSet<ClassBytesRange>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1350-1352`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1350-L1352)*

A set of characters represented by arbitrary bytes.

Each byte corresponds to one character.

#### Implementations

- <span id="classbytes-new"></span>`fn new<I>(ranges: I) -> ClassBytes` — [`ClassBytes`](#classbytes)

- <span id="classbytes-empty"></span>`fn empty() -> ClassBytes` — [`ClassBytes`](#classbytes)

- <span id="classbytes-push"></span>`fn push(&mut self, range: ClassBytesRange)` — [`ClassBytesRange`](#classbytesrange)

- <span id="classbytes-iter"></span>`fn iter(&self) -> ClassBytesIter<'_>` — [`ClassBytesIter`](#classbytesiter)

- <span id="classbytes-ranges"></span>`fn ranges(&self) -> &[ClassBytesRange]` — [`ClassBytesRange`](#classbytesrange)

- <span id="classbytes-case-fold-simple"></span>`fn case_fold_simple(&mut self)`

- <span id="classbytes-negate"></span>`fn negate(&mut self)`

- <span id="classbytes-union"></span>`fn union(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

- <span id="classbytes-intersect"></span>`fn intersect(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

- <span id="classbytes-difference"></span>`fn difference(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

- <span id="classbytes-symmetric-difference"></span>`fn symmetric_difference(&mut self, other: &ClassBytes)` — [`ClassBytes`](#classbytes)

- <span id="classbytes-is-ascii"></span>`fn is_ascii(&self) -> bool`

- <span id="classbytes-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

- <span id="classbytes-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

- <span id="classbytes-literal"></span>`fn literal(&self) -> Option<Vec<u8>>`

- <span id="classbytes-to-unicode-class"></span>`fn to_unicode_class(&self) -> Option<ClassUnicode>` — [`ClassUnicode`](#classunicode)

#### Trait Implementations

##### `impl Clone for ClassBytes`

- <span id="classbytes-clone"></span>`fn clone(&self) -> ClassBytes` — [`ClassBytes`](#classbytes)

##### `impl Debug for ClassBytes`

- <span id="classbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassBytes`

##### `impl PartialEq for ClassBytes`

- <span id="classbytes-eq"></span>`fn eq(&self, other: &ClassBytes) -> bool` — [`ClassBytes`](#classbytes)

##### `impl StructuralPartialEq for ClassBytes`

### `ClassBytesIter<'a>`

```rust
struct ClassBytesIter<'a>(crate::hir::interval::IntervalSetIter<'a, ClassBytesRange>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1504`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1504)*

An iterator over all ranges in a byte character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl Debug for ClassBytesIter<'a>`

- <span id="classbytesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ClassBytesIter<'a>`

- <span id="classbytesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="classbytesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="classbytesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ClassBytesIter<'a>`

- <span id="classbytesiter-type-item"></span>`type Item = &'a ClassBytesRange`

- <span id="classbytesiter-next"></span>`fn next(&mut self) -> Option<&'a ClassBytesRange>` — [`ClassBytesRange`](#classbytesrange)

### `ClassBytesRange`

```rust
struct ClassBytesRange {
    start: u8,
    end: u8,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1519-1522`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1519-L1522)*

A single range of characters represented by arbitrary bytes.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- <span id="classbytesrange-new"></span>`fn new(start: u8, end: u8) -> ClassBytesRange` — [`ClassBytesRange`](#classbytesrange)

- <span id="classbytesrange-start"></span>`fn start(&self) -> u8`

- <span id="classbytesrange-end"></span>`fn end(&self) -> u8`

- <span id="classbytesrange-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for ClassBytesRange`

- <span id="classbytesrange-clone"></span>`fn clone(&self) -> ClassBytesRange` — [`ClassBytesRange`](#classbytesrange)

##### `impl Copy for ClassBytesRange`

##### `impl Debug for ClassBytesRange`

- <span id="classbytesrange-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for ClassBytesRange`

- <span id="classbytesrange-default"></span>`fn default() -> ClassBytesRange` — [`ClassBytesRange`](#classbytesrange)

##### `impl Eq for ClassBytesRange`

##### `impl Interval for ClassBytesRange`

- <span id="classbytesrange-type-bound"></span>`type Bound = u8`

- <span id="classbytesrange-lower"></span>`fn lower(&self) -> u8`

- <span id="classbytesrange-upper"></span>`fn upper(&self) -> u8`

- <span id="classbytesrange-set-lower"></span>`fn set_lower(&mut self, bound: u8)`

- <span id="classbytesrange-set-upper"></span>`fn set_upper(&mut self, bound: u8)`

- <span id="classbytesrange-case-fold-simple"></span>`fn case_fold_simple(&self, ranges: &mut Vec<ClassBytesRange>) -> Result<(), unicode::CaseFoldError>` — [`ClassBytesRange`](#classbytesrange), [`CaseFoldError`](../unicode/index.md#casefolderror)

##### `impl Ord for ClassBytesRange`

- <span id="classbytesrange-cmp"></span>`fn cmp(&self, other: &ClassBytesRange) -> cmp::Ordering` — [`ClassBytesRange`](#classbytesrange)

##### `impl PartialEq for ClassBytesRange`

- <span id="classbytesrange-eq"></span>`fn eq(&self, other: &ClassBytesRange) -> bool` — [`ClassBytesRange`](#classbytesrange)

##### `impl PartialOrd for ClassBytesRange`

- <span id="classbytesrange-partial-cmp"></span>`fn partial_cmp(&self, other: &ClassBytesRange) -> option::Option<cmp::Ordering>` — [`ClassBytesRange`](#classbytesrange)

##### `impl StructuralPartialEq for ClassBytesRange`

### `Capture`

```rust
struct Capture {
    pub index: u32,
    pub name: Option<alloc::boxed::Box<str>>,
    pub sub: alloc::boxed::Box<Hir>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1799-1806`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1799-L1806)*

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

##### `impl Clone for Capture`

- <span id="capture-clone"></span>`fn clone(&self) -> Capture` — [`Capture`](#capture)

##### `impl Debug for Capture`

- <span id="capture-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Capture`

##### `impl PartialEq for Capture`

- <span id="capture-eq"></span>`fn eq(&self, other: &Capture) -> bool` — [`Capture`](#capture)

##### `impl StructuralPartialEq for Capture`

### `Repetition`

```rust
struct Repetition {
    pub min: u32,
    pub max: Option<u32>,
    pub greedy: bool,
    pub sub: alloc::boxed::Box<Hir>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1813-1839`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1813-L1839)*

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

#### Trait Implementations

##### `impl Clone for Repetition`

- <span id="repetition-clone"></span>`fn clone(&self) -> Repetition` — [`Repetition`](#repetition)

##### `impl Debug for Repetition`

- <span id="repetition-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Repetition`

##### `impl PartialEq for Repetition`

- <span id="repetition-eq"></span>`fn eq(&self, other: &Repetition) -> bool` — [`Repetition`](#repetition)

##### `impl StructuralPartialEq for Repetition`

### `Properties`

```rust
struct Properties(alloc::boxed::Box<PropertiesI>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1964`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1964)*

A type that collects various properties of an HIR value.

Properties are always scalar values and represent meta data that is
computed inductively on an HIR value. Properties are defined for all
HIR values.

All methods on a `Properties` value take constant time and are meant to
be cheap to call.

#### Implementations

- <span id="properties-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

- <span id="properties-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

- <span id="properties-look-set"></span>`fn look_set(&self) -> LookSet` — [`LookSet`](#lookset)

- <span id="properties-look-set-prefix"></span>`fn look_set_prefix(&self) -> LookSet` — [`LookSet`](#lookset)

- <span id="properties-look-set-prefix-any"></span>`fn look_set_prefix_any(&self) -> LookSet` — [`LookSet`](#lookset)

- <span id="properties-look-set-suffix"></span>`fn look_set_suffix(&self) -> LookSet` — [`LookSet`](#lookset)

- <span id="properties-look-set-suffix-any"></span>`fn look_set_suffix_any(&self) -> LookSet` — [`LookSet`](#lookset)

- <span id="properties-is-utf8"></span>`fn is_utf8(&self) -> bool`

- <span id="properties-explicit-captures-len"></span>`fn explicit_captures_len(&self) -> usize`

- <span id="properties-static-explicit-captures-len"></span>`fn static_explicit_captures_len(&self) -> Option<usize>`

- <span id="properties-is-literal"></span>`fn is_literal(&self) -> bool`

- <span id="properties-is-alternation-literal"></span>`fn is_alternation_literal(&self) -> bool`

- <span id="properties-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="properties-union"></span>`fn union<I, P>(props: I) -> Properties` — [`Properties`](#properties)

#### Trait Implementations

##### `impl Clone for Properties`

- <span id="properties-clone"></span>`fn clone(&self) -> Properties` — [`Properties`](#properties)

##### `impl Debug for Properties`

- <span id="properties-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Properties`

##### `impl PartialEq for Properties`

- <span id="properties-eq"></span>`fn eq(&self, other: &Properties) -> bool` — [`Properties`](#properties)

##### `impl StructuralPartialEq for Properties`

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

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1974-1987`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1974-L1987)*

The property definition. It is split out so that we can box it, and
there by make `Properties` use less stack size. This is kind-of important
because every HIR value has a `Properties` attached to it.

This does have the unfortunate consequence that creating any HIR value
always leads to at least one alloc for properties, but this is generally
true anyway (for pretty much all HirKinds except for look-arounds).

#### Trait Implementations

##### `impl Clone for PropertiesI`

- <span id="propertiesi-clone"></span>`fn clone(&self) -> PropertiesI` — [`PropertiesI`](#propertiesi)

##### `impl Debug for PropertiesI`

- <span id="propertiesi-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PropertiesI`

##### `impl PartialEq for PropertiesI`

- <span id="propertiesi-eq"></span>`fn eq(&self, other: &PropertiesI) -> bool` — [`PropertiesI`](#propertiesi)

##### `impl StructuralPartialEq for PropertiesI`

### `LookSet`

```rust
struct LookSet {
    pub bits: u32,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2665-2676`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L2665-L2676)*

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

- <span id="lookset-full"></span>`fn full() -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-singleton"></span>`fn singleton(look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- <span id="lookset-len"></span>`fn len(self) -> usize`

- <span id="lookset-is-empty"></span>`fn is_empty(self) -> bool`

- <span id="lookset-contains"></span>`fn contains(self, look: Look) -> bool` — [`Look`](#look)

- <span id="lookset-contains-anchor"></span>`fn contains_anchor(&self) -> bool`

- <span id="lookset-contains-anchor-haystack"></span>`fn contains_anchor_haystack(&self) -> bool`

- <span id="lookset-contains-anchor-line"></span>`fn contains_anchor_line(&self) -> bool`

- <span id="lookset-contains-anchor-lf"></span>`fn contains_anchor_lf(&self) -> bool`

- <span id="lookset-contains-anchor-crlf"></span>`fn contains_anchor_crlf(&self) -> bool`

- <span id="lookset-contains-word"></span>`fn contains_word(self) -> bool`

- <span id="lookset-contains-word-unicode"></span>`fn contains_word_unicode(self) -> bool`

- <span id="lookset-contains-word-ascii"></span>`fn contains_word_ascii(self) -> bool`

- <span id="lookset-iter"></span>`fn iter(self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

- <span id="lookset-insert"></span>`fn insert(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- <span id="lookset-set-insert"></span>`fn set_insert(&mut self, look: Look)` — [`Look`](#look)

- <span id="lookset-remove"></span>`fn remove(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- <span id="lookset-set-remove"></span>`fn set_remove(&mut self, look: Look)` — [`Look`](#look)

- <span id="lookset-subtract"></span>`fn subtract(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-set-subtract"></span>`fn set_subtract(&mut self, other: LookSet)` — [`LookSet`](#lookset)

- <span id="lookset-union"></span>`fn union(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-set-union"></span>`fn set_union(&mut self, other: LookSet)` — [`LookSet`](#lookset)

- <span id="lookset-intersect"></span>`fn intersect(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-set-intersect"></span>`fn set_intersect(&mut self, other: LookSet)` — [`LookSet`](#lookset)

- <span id="lookset-read-repr"></span>`fn read_repr(slice: &[u8]) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-write-repr"></span>`fn write_repr(self, slice: &mut [u8])`

#### Trait Implementations

##### `impl Clone for LookSet`

- <span id="lookset-clone"></span>`fn clone(&self) -> LookSet` — [`LookSet`](#lookset)

##### `impl Copy for LookSet`

##### `impl Debug for LookSet`

- <span id="lookset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for LookSet`

- <span id="lookset-default"></span>`fn default() -> LookSet` — [`LookSet`](#lookset)

##### `impl Eq for LookSet`

##### `impl PartialEq for LookSet`

- <span id="lookset-eq"></span>`fn eq(&self, other: &LookSet) -> bool` — [`LookSet`](#lookset)

##### `impl StructuralPartialEq for LookSet`

### `LookSetIter`

```rust
struct LookSetIter {
    set: LookSet,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2916-2918`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L2916-L2918)*

An iterator over all look-around assertions in a [`LookSet`](#lookset).

This iterator is created by `LookSet::iter`.

#### Trait Implementations

##### `impl Clone for LookSetIter`

- <span id="looksetiter-clone"></span>`fn clone(&self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

##### `impl Debug for LookSetIter`

- <span id="looksetiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for LookSetIter`

- <span id="looksetiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="looksetiter-type-intoiter"></span>`type IntoIter = I`

- <span id="looksetiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LookSetIter`

- <span id="looksetiter-type-item"></span>`type Item = Look`

- <span id="looksetiter-next"></span>`fn next(&mut self) -> Option<Look>` — [`Look`](#look)

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

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:84-108`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L84-L108)*

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

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Debug for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

##### `impl ToString for ErrorKind`

- <span id="errorkind-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:717-752`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L717-L752)*

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

#### Trait Implementations

##### `impl Clone for HirKind`

- <span id="hirkind-clone"></span>`fn clone(&self) -> HirKind` — [`HirKind`](#hirkind)

##### `impl Debug for HirKind`

- <span id="hirkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HirKind`

##### `impl PartialEq for HirKind`

- <span id="hirkind-eq"></span>`fn eq(&self, other: &HirKind) -> bool` — [`HirKind`](#hirkind)

##### `impl StructuralPartialEq for HirKind`

### `Class`

```rust
enum Class {
    Unicode(ClassUnicode),
    Bytes(ClassBytes),
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:830-836`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L830-L836)*

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

- <span id="class-try-case-fold-simple"></span>`fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError>` — [`CaseFoldError`](../unicode/index.md#casefolderror)

- <span id="class-negate"></span>`fn negate(&mut self)`

- <span id="class-is-utf8"></span>`fn is_utf8(&self) -> bool`

- <span id="class-minimum-len"></span>`fn minimum_len(&self) -> Option<usize>`

- <span id="class-maximum-len"></span>`fn maximum_len(&self) -> Option<usize>`

- <span id="class-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="class-literal"></span>`fn literal(&self) -> Option<Vec<u8>>`

#### Trait Implementations

##### `impl Clone for Class`

- <span id="class-clone"></span>`fn clone(&self) -> Class` — [`Class`](#class)

##### `impl Debug for Class`

- <span id="class-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Class`

##### `impl PartialEq for Class`

- <span id="class-eq"></span>`fn eq(&self, other: &Class) -> bool` — [`Class`](#class)

##### `impl StructuralPartialEq for Class`

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

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1613-1686`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1613-L1686)*

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

- <span id="look-as-repr"></span>`const fn as_repr(self) -> u32`

- <span id="look-from-repr"></span>`const fn from_repr(repr: u32) -> Option<Look>` — [`Look`](#look)

- <span id="look-as-char"></span>`const fn as_char(self) -> char`

#### Trait Implementations

##### `impl Clone for Look`

- <span id="look-clone"></span>`fn clone(&self) -> Look` — [`Look`](#look)

##### `impl Copy for Look`

##### `impl Debug for Look`

- <span id="look-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Look`

##### `impl PartialEq for Look`

- <span id="look-eq"></span>`fn eq(&self, other: &Look) -> bool` — [`Look`](#look)

##### `impl StructuralPartialEq for Look`

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

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:1860-1909`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L1860-L1909)*

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

##### `impl Clone for Dot`

- <span id="dot-clone"></span>`fn clone(&self) -> Dot` — [`Dot`](#dot)

##### `impl Copy for Dot`

##### `impl Debug for Dot`

- <span id="dot-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Dot`

##### `impl PartialEq for Dot`

- <span id="dot-eq"></span>`fn eq(&self, other: &Dot) -> bool` — [`Dot`](#dot)

##### `impl StructuralPartialEq for Dot`

## Traits

### `Visitor`

```rust
trait Visitor { ... }
```

*Defined in [`regex-syntax-0.8.8/src/hir/visitor.rs:15-49`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/visitor.rs#L15-L49)*

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

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:37`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L37)*

### `class_chars`

```rust
fn class_chars(hirs: &[Hir]) -> Option<Class>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2940-2954`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L2940-L2954)*

Given a sequence of HIR values where each value corresponds to a Unicode
class (or an all-ASCII byte class), return a single Unicode class
corresponding to the union of the classes found.

### `class_bytes`

```rust
fn class_bytes(hirs: &[Hir]) -> Option<Class>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2959-2973`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L2959-L2973)*

Given a sequence of HIR values where each value corresponds to a byte class
(or an all-ASCII Unicode class), return a single byte class corresponding
to the union of the classes found.

### `singleton_chars`

```rust
fn singleton_chars(hirs: &[Hir]) -> Option<alloc::vec::Vec<char>>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:2978-2996`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L2978-L2996)*

Given a sequence of HIR values where each value corresponds to a literal
that is a single `char`, return that sequence of `char`s. Otherwise return
None. No deduplication is done.

### `singleton_bytes`

```rust
fn singleton_bytes(hirs: &[Hir]) -> Option<alloc::vec::Vec<u8>>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:3001-3014`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L3001-L3014)*

Given a sequence of HIR values where each value corresponds to a literal
that is a single byte, return that sequence of bytes. Otherwise return
None. No deduplication is done.

### `lift_common_prefix`

```rust
fn lift_common_prefix(hirs: alloc::vec::Vec<Hir>) -> Result<Hir, alloc::vec::Vec<Hir>>
```

*Defined in [`regex-syntax-0.8.8/src/hir/mod.rs:3027-3073`](../../../.source_1765210505/regex-syntax-0.8.8/src/hir/mod.rs#L3027-L3073)*

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

