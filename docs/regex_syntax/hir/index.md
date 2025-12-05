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

## Modules

- [`literal`](literal/index.md) - Provides literal extraction from `Hir` expressions.
- [`print`](print/index.md) - This module provides a regular expression printer for `Hir`.
- [`translate`](translate/index.md) - Defines a translator that converts an `Ast` to an `Hir`.

## Structs

### `CaseFoldError`

```rust
struct CaseFoldError(());
```

An error that occurs when Unicode-aware simple case folding fails.

This error can occur when the case mapping tables necessary for Unicode
aware case folding are unavailable. This only occurs when the
`unicode-case` feature is disabled. (The feature is enabled by default.)

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `Error`

```rust
struct Error {
    kind: ErrorKind,
    pattern: alloc::string::String,
    span: crate::ast::Span,
}
```

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

- `fn kind(self: &Self) -> &ErrorKind` — [`ErrorKind`](../../hir/index.md)

- `fn pattern(self: &Self) -> &str`

- `fn span(self: &Self) -> &Span` — [`Span`](../../ast/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Error` — [`Error`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool` — [`Error`](../../hir/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `Hir`

```rust
struct Hir {
    kind: HirKind,
    props: Properties,
}
```

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

- `fn empty() -> Hir` — [`Hir`](../../hir/index.md)

- `fn fail() -> Hir` — [`Hir`](../../hir/index.md)

- `fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir` — [`Hir`](../../hir/index.md)

- `fn class(class: Class) -> Hir` — [`Class`](../../hir/index.md), [`Hir`](../../hir/index.md)

- `fn look(look: Look) -> Hir` — [`Look`](../../hir/index.md), [`Hir`](../../hir/index.md)

- `fn repetition(rep: Repetition) -> Hir` — [`Repetition`](../../hir/index.md), [`Hir`](../../hir/index.md)

- `fn capture(capture: Capture) -> Hir` — [`Capture`](../../hir/index.md), [`Hir`](../../hir/index.md)

- `fn concat(subs: Vec<Hir>) -> Hir` — [`Hir`](../../hir/index.md)

- `fn alternation(subs: Vec<Hir>) -> Hir` — [`Hir`](../../hir/index.md)

- `fn dot(dot: Dot) -> Hir` — [`Dot`](../../hir/index.md), [`Hir`](../../hir/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Hir` — [`Hir`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Hir) -> bool` — [`Hir`](../../hir/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `Literal`

```rust
struct Literal(alloc::boxed::Box<[u8]>);
```

The high-level intermediate representation of a literal.

A literal corresponds to `0` or more bytes that should be matched
literally. The smart constructors defined on `Hir` will automatically
concatenate adjacent literals into one literal, and will even automatically
replace empty literals with `Hir::empty()`.

Note that despite a literal being represented by a sequence of bytes, its
`Debug` implementation will attempt to print it as a normal string. (That
is, not a sequence of decimal numbers.)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Literal` — [`Literal`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Literal) -> bool` — [`Literal`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `ClassUnicode`

```rust
struct ClassUnicode {
    set: crate::hir::interval::IntervalSet<ClassUnicodeRange>,
}
```

A set of characters represented by Unicode scalar values.

#### Implementations

- `fn new<I>(ranges: I) -> ClassUnicode` — [`ClassUnicode`](../../hir/index.md)

- `fn empty() -> ClassUnicode` — [`ClassUnicode`](../../hir/index.md)

- `fn push(self: &mut Self, range: ClassUnicodeRange)` — [`ClassUnicodeRange`](../../hir/index.md)

- `fn iter(self: &Self) -> ClassUnicodeIter<'_>` — [`ClassUnicodeIter`](../../hir/index.md)

- `fn ranges(self: &Self) -> &[ClassUnicodeRange]` — [`ClassUnicodeRange`](../../hir/index.md)

- `fn case_fold_simple(self: &mut Self)`

- `fn try_case_fold_simple(self: &mut Self) -> core::result::Result<(), CaseFoldError>` — [`CaseFoldError`](../../unicode/index.md)

- `fn negate(self: &mut Self)`

- `fn union(self: &mut Self, other: &ClassUnicode)` — [`ClassUnicode`](../../hir/index.md)

- `fn intersect(self: &mut Self, other: &ClassUnicode)` — [`ClassUnicode`](../../hir/index.md)

- `fn difference(self: &mut Self, other: &ClassUnicode)` — [`ClassUnicode`](../../hir/index.md)

- `fn symmetric_difference(self: &mut Self, other: &ClassUnicode)` — [`ClassUnicode`](../../hir/index.md)

- `fn is_ascii(self: &Self) -> bool`

- `fn minimum_len(self: &Self) -> Option<usize>`

- `fn maximum_len(self: &Self) -> Option<usize>`

- `fn literal(self: &Self) -> Option<Vec<u8>>`

- `fn to_byte_class(self: &Self) -> Option<ClassBytes>` — [`ClassBytes`](../../hir/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ClassUnicode` — [`ClassUnicode`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassUnicode) -> bool` — [`ClassUnicode`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `ClassUnicodeIter<'a>`

```rust
struct ClassUnicodeIter<'a>(crate::hir::interval::IntervalSetIter<'a, ClassUnicodeRange>);
```

An iterator over all ranges in a Unicode character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

- `type Item = &'a ClassUnicodeRange`

- `fn next(self: &mut Self) -> Option<&'a ClassUnicodeRange>` — [`ClassUnicodeRange`](../../hir/index.md)

### `ClassUnicodeRange`

```rust
struct ClassUnicodeRange {
    start: char,
    end: char,
}
```

A single range of characters represented by Unicode scalar values.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- `fn new(start: char, end: char) -> ClassUnicodeRange` — [`ClassUnicodeRange`](../../hir/index.md)

- `fn start(self: &Self) -> char`

- `fn end(self: &Self) -> char`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ClassUnicodeRange` — [`ClassUnicodeRange`](../../hir/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> ClassUnicodeRange` — [`ClassUnicodeRange`](../../hir/index.md)

##### `impl Eq`

##### `impl Interval`

- `type Bound = char`

- `fn lower(self: &Self) -> char`

- `fn upper(self: &Self) -> char`

- `fn set_lower(self: &mut Self, bound: char)`

- `fn set_upper(self: &mut Self, bound: char)`

- `fn case_fold_simple(self: &Self, ranges: &mut Vec<ClassUnicodeRange>) -> Result<(), unicode::CaseFoldError>` — [`ClassUnicodeRange`](../../hir/index.md), [`CaseFoldError`](../../unicode/index.md)

##### `impl Ord`

- `fn cmp(self: &Self, other: &ClassUnicodeRange) -> $crate::cmp::Ordering` — [`ClassUnicodeRange`](../../hir/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassUnicodeRange) -> bool` — [`ClassUnicodeRange`](../../hir/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &ClassUnicodeRange) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ClassUnicodeRange`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `ClassBytes`

```rust
struct ClassBytes {
    set: crate::hir::interval::IntervalSet<ClassBytesRange>,
}
```

A set of characters represented by arbitrary bytes.

Each byte corresponds to one character.

#### Implementations

- `fn new<I>(ranges: I) -> ClassBytes` — [`ClassBytes`](../../hir/index.md)

- `fn empty() -> ClassBytes` — [`ClassBytes`](../../hir/index.md)

- `fn push(self: &mut Self, range: ClassBytesRange)` — [`ClassBytesRange`](../../hir/index.md)

- `fn iter(self: &Self) -> ClassBytesIter<'_>` — [`ClassBytesIter`](../../hir/index.md)

- `fn ranges(self: &Self) -> &[ClassBytesRange]` — [`ClassBytesRange`](../../hir/index.md)

- `fn case_fold_simple(self: &mut Self)`

- `fn negate(self: &mut Self)`

- `fn union(self: &mut Self, other: &ClassBytes)` — [`ClassBytes`](../../hir/index.md)

- `fn intersect(self: &mut Self, other: &ClassBytes)` — [`ClassBytes`](../../hir/index.md)

- `fn difference(self: &mut Self, other: &ClassBytes)` — [`ClassBytes`](../../hir/index.md)

- `fn symmetric_difference(self: &mut Self, other: &ClassBytes)` — [`ClassBytes`](../../hir/index.md)

- `fn is_ascii(self: &Self) -> bool`

- `fn minimum_len(self: &Self) -> Option<usize>`

- `fn maximum_len(self: &Self) -> Option<usize>`

- `fn literal(self: &Self) -> Option<Vec<u8>>`

- `fn to_unicode_class(self: &Self) -> Option<ClassUnicode>` — [`ClassUnicode`](../../hir/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ClassBytes` — [`ClassBytes`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassBytes) -> bool` — [`ClassBytes`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `ClassBytesIter<'a>`

```rust
struct ClassBytesIter<'a>(crate::hir::interval::IntervalSetIter<'a, ClassBytesRange>);
```

An iterator over all ranges in a byte character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

- `type Item = &'a ClassBytesRange`

- `fn next(self: &mut Self) -> Option<&'a ClassBytesRange>` — [`ClassBytesRange`](../../hir/index.md)

### `ClassBytesRange`

```rust
struct ClassBytesRange {
    start: u8,
    end: u8,
}
```

A single range of characters represented by arbitrary bytes.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- `fn new(start: u8, end: u8) -> ClassBytesRange` — [`ClassBytesRange`](../../hir/index.md)

- `fn start(self: &Self) -> u8`

- `fn end(self: &Self) -> u8`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ClassBytesRange` — [`ClassBytesRange`](../../hir/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> ClassBytesRange` — [`ClassBytesRange`](../../hir/index.md)

##### `impl Eq`

##### `impl Interval`

- `type Bound = u8`

- `fn lower(self: &Self) -> u8`

- `fn upper(self: &Self) -> u8`

- `fn set_lower(self: &mut Self, bound: u8)`

- `fn set_upper(self: &mut Self, bound: u8)`

- `fn case_fold_simple(self: &Self, ranges: &mut Vec<ClassBytesRange>) -> Result<(), unicode::CaseFoldError>` — [`ClassBytesRange`](../../hir/index.md), [`CaseFoldError`](../../unicode/index.md)

##### `impl Ord`

- `fn cmp(self: &Self, other: &ClassBytesRange) -> $crate::cmp::Ordering` — [`ClassBytesRange`](../../hir/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassBytesRange) -> bool` — [`ClassBytesRange`](../../hir/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &ClassBytesRange) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ClassBytesRange`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `Capture`

```rust
struct Capture {
    pub index: u32,
    pub name: Option<alloc::boxed::Box<str>>,
    pub sub: alloc::boxed::Box<Hir>,
}
```

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

##### `impl Clone`

- `fn clone(self: &Self) -> Capture` — [`Capture`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Capture) -> bool` — [`Capture`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `Repetition`

```rust
struct Repetition {
    pub min: u32,
    pub max: Option<u32>,
    pub greedy: bool,
    pub sub: alloc::boxed::Box<Hir>,
}
```

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

- `fn with(self: &Self, sub: Hir) -> Repetition` — [`Hir`](../../hir/index.md), [`Repetition`](../../hir/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Repetition` — [`Repetition`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Repetition) -> bool` — [`Repetition`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `Properties`

```rust
struct Properties(alloc::boxed::Box<PropertiesI>);
```

A type that collects various properties of an HIR value.

Properties are always scalar values and represent meta data that is
computed inductively on an HIR value. Properties are defined for all
HIR values.

All methods on a `Properties` value take constant time and are meant to
be cheap to call.

#### Implementations

- `fn minimum_len(self: &Self) -> Option<usize>`

- `fn maximum_len(self: &Self) -> Option<usize>`

- `fn look_set(self: &Self) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn look_set_prefix(self: &Self) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn look_set_prefix_any(self: &Self) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn look_set_suffix(self: &Self) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn look_set_suffix_any(self: &Self) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn is_utf8(self: &Self) -> bool`

- `fn explicit_captures_len(self: &Self) -> usize`

- `fn static_explicit_captures_len(self: &Self) -> Option<usize>`

- `fn is_literal(self: &Self) -> bool`

- `fn is_alternation_literal(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn union<I, P>(props: I) -> Properties` — [`Properties`](../../hir/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Properties` — [`Properties`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Properties) -> bool` — [`Properties`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `LookSet`

```rust
struct LookSet {
    pub bits: u32,
}
```

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

- `fn empty() -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn full() -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn singleton(look: Look) -> LookSet` — [`Look`](../../hir/index.md), [`LookSet`](../../hir/index.md)

- `fn len(self: Self) -> usize`

- `fn is_empty(self: Self) -> bool`

- `fn contains(self: Self, look: Look) -> bool` — [`Look`](../../hir/index.md)

- `fn contains_anchor(self: &Self) -> bool`

- `fn contains_anchor_haystack(self: &Self) -> bool`

- `fn contains_anchor_line(self: &Self) -> bool`

- `fn contains_anchor_lf(self: &Self) -> bool`

- `fn contains_anchor_crlf(self: &Self) -> bool`

- `fn contains_word(self: Self) -> bool`

- `fn contains_word_unicode(self: Self) -> bool`

- `fn contains_word_ascii(self: Self) -> bool`

- `fn iter(self: Self) -> LookSetIter` — [`LookSetIter`](../../hir/index.md)

- `fn insert(self: Self, look: Look) -> LookSet` — [`Look`](../../hir/index.md), [`LookSet`](../../hir/index.md)

- `fn set_insert(self: &mut Self, look: Look)` — [`Look`](../../hir/index.md)

- `fn remove(self: Self, look: Look) -> LookSet` — [`Look`](../../hir/index.md), [`LookSet`](../../hir/index.md)

- `fn set_remove(self: &mut Self, look: Look)` — [`Look`](../../hir/index.md)

- `fn subtract(self: Self, other: LookSet) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn set_subtract(self: &mut Self, other: LookSet)` — [`LookSet`](../../hir/index.md)

- `fn union(self: Self, other: LookSet) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn set_union(self: &mut Self, other: LookSet)` — [`LookSet`](../../hir/index.md)

- `fn intersect(self: Self, other: LookSet) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn set_intersect(self: &mut Self, other: LookSet)` — [`LookSet`](../../hir/index.md)

- `fn read_repr(slice: &[u8]) -> LookSet` — [`LookSet`](../../hir/index.md)

- `fn write_repr(self: Self, slice: &mut [u8])`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> LookSet` — [`LookSet`](../../hir/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> LookSet` — [`LookSet`](../../hir/index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LookSet) -> bool` — [`LookSet`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `LookSetIter`

```rust
struct LookSetIter {
    set: LookSet,
}
```

An iterator over all look-around assertions in a [`LookSet`](#lookset).

This iterator is created by `LookSet::iter`.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> LookSetIter` — [`LookSetIter`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator`

- `type Item = Look`

- `fn next(self: &mut Self) -> Option<Look>` — [`Look`](../../hir/index.md)

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

##### `impl Clone`

- `fn clone(self: &Self) -> ErrorKind` — [`ErrorKind`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ErrorKind) -> bool` — [`ErrorKind`](../../hir/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

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

- `fn subs(self: &Self) -> &[Hir]` — [`Hir`](../../hir/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> HirKind` — [`HirKind`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HirKind) -> bool` — [`HirKind`](../../hir/index.md)

##### `impl StructuralPartialEq`

### `Class`

```rust
enum Class {
    Unicode(ClassUnicode),
    Bytes(ClassBytes),
}
```

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

- `fn case_fold_simple(self: &mut Self)`

- `fn try_case_fold_simple(self: &mut Self) -> core::result::Result<(), CaseFoldError>` — [`CaseFoldError`](../../unicode/index.md)

- `fn negate(self: &mut Self)`

- `fn is_utf8(self: &Self) -> bool`

- `fn minimum_len(self: &Self) -> Option<usize>`

- `fn maximum_len(self: &Self) -> Option<usize>`

- `fn is_empty(self: &Self) -> bool`

- `fn literal(self: &Self) -> Option<Vec<u8>>`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Class` — [`Class`](../../hir/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Class) -> bool` — [`Class`](../../hir/index.md)

##### `impl StructuralPartialEq`

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

- `const fn reversed(self: Self) -> Look` — [`Look`](../../hir/index.md)

- `const fn as_repr(self: Self) -> u32`

- `const fn from_repr(repr: u32) -> Option<Look>` — [`Look`](../../hir/index.md)

- `const fn as_char(self: Self) -> char`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Look` — [`Look`](../../hir/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Look) -> bool` — [`Look`](../../hir/index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Dot` — [`Dot`](../../hir/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Dot) -> bool` — [`Dot`](../../hir/index.md)

##### `impl StructuralPartialEq`

## Traits

## Functions

