*[regex_syntax](../index.md) / [hir](index.md)*

---

# Module `hir`

Defines a high-level intermediate (HIR) representation for regular expressions.

The HIR is represented by the [`Hir`](regex_syntax/hir/index.md) type, and it principally constructed via
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
struct CaseFoldError();
```

An error that occurs when Unicode-aware simple case folding fails.

This error can occur when the case mapping tables necessary for Unicode
aware case folding are unavailable. This only occurs when the
`unicode-case` feature is disabled. (The feature is enabled by default.)

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

### `Error`

```rust
struct Error {
    // [REDACTED: Private Fields]
}
```

An error that can occur while translating an `Ast` to a `Hir`.

#### Implementations

- `fn kind(self: &Self) -> &ErrorKind`
  Return the type of this error.

- `fn pattern(self: &Self) -> &str`
  The original pattern string in which this error occurred.

- `fn span(self: &Self) -> &Span`
  Return the span at which this error occurred.

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

### `Hir`

```rust
struct Hir {
    // [REDACTED: Private Fields]
}
```

A high-level intermediate representation (HIR) for a regular expression.

An HIR value is a combination of a [`HirKind`](regex_syntax/hir/index.md) and a set of [`Properties`](regex_syntax/hir/index.md).
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

#### Implementations

- `fn kind(self: &Self) -> &HirKind`
  Returns a reference to the underlying HIR kind.

- `fn into_kind(self: Self) -> HirKind`
  Consumes ownership of this HIR expression and returns its underlying

- `fn properties(self: &Self) -> &Properties`
  Returns the properties computed for this `Hir`.

- `fn empty() -> Hir`
  Returns an empty HIR expression.

- `fn fail() -> Hir`
  Returns an HIR expression that can never match anything. That is,

- `fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir`
  Creates a literal HIR expression.

- `fn class(class: Class) -> Hir`
  Creates a class HIR expression. The class may either be defined over

- `fn look(look: Look) -> Hir`
  Creates a look-around assertion HIR expression.

- `fn repetition(rep: Repetition) -> Hir`
  Creates a repetition HIR expression.

- `fn capture(capture: Capture) -> Hir`
  Creates a capture HIR expression.

- `fn concat(subs: Vec<Hir>) -> Hir`
  Returns the concatenation of the given expressions.

- `fn alternation(subs: Vec<Hir>) -> Hir`
  Returns the alternation of the given expressions.

- `fn dot(dot: Dot) -> Hir`
  Returns an HIR expression for `.`.

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

- `fn clone(self: &Self) -> Hir`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Hir) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

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

- `fn clone(self: &Self) -> Literal`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Literal) -> bool`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ClassUnicode`

```rust
struct ClassUnicode {
    // [REDACTED: Private Fields]
}
```

A set of characters represented by Unicode scalar values.

#### Implementations

- `fn new<I>(ranges: I) -> ClassUnicode`
  Create a new class from a sequence of ranges.

- `fn empty() -> ClassUnicode`
  Create a new class with no ranges.

- `fn push(self: &mut Self, range: ClassUnicodeRange)`
  Add a new range to this set.

- `fn iter(self: &Self) -> ClassUnicodeIter<'_>`
  Return an iterator over all ranges in this class.

- `fn ranges(self: &Self) -> &[ClassUnicodeRange]`
  Return the underlying ranges as a slice.

- `fn case_fold_simple(self: &mut Self)`
  Expand this character class such that it contains all case folded

- `fn try_case_fold_simple(self: &mut Self) -> core::result::Result<(), CaseFoldError>`
  Expand this character class such that it contains all case folded

- `fn negate(self: &mut Self)`
  Negate this character class.

- `fn union(self: &mut Self, other: &ClassUnicode)`
  Union this character class with the given character class, in place.

- `fn intersect(self: &mut Self, other: &ClassUnicode)`
  Intersect this character class with the given character class, in

- `fn difference(self: &mut Self, other: &ClassUnicode)`
  Subtract the given character class from this character class, in place.

- `fn symmetric_difference(self: &mut Self, other: &ClassUnicode)`
  Compute the symmetric difference of the given character classes, in

- `fn is_ascii(self: &Self) -> bool`
  Returns true if and only if this character class will either match

- `fn minimum_len(self: &Self) -> Option<usize>`
  Returns the length, in bytes, of the smallest string matched by this

- `fn maximum_len(self: &Self) -> Option<usize>`
  Returns the length, in bytes, of the longest string matched by this

- `fn literal(self: &Self) -> Option<Vec<u8>>`
  If this class consists of exactly one codepoint, then return it as

- `fn to_byte_class(self: &Self) -> Option<ClassBytes>`
  If this class consists of only ASCII ranges, then return its

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

- `fn clone(self: &Self) -> ClassUnicode`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassUnicode) -> bool`

##### `impl StructuralPartialEq`

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

### `ClassUnicodeIter<'a>`

```rust
struct ClassUnicodeIter<'a>();
```

An iterator over all ranges in a Unicode character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'a>`

- `type Item = &'a ClassUnicodeRange`

- `fn next(self: &mut Self) -> Option<&'a ClassUnicodeRange>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ClassUnicodeRange`

```rust
struct ClassUnicodeRange {
    // [REDACTED: Private Fields]
}
```

A single range of characters represented by Unicode scalar values.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- `fn new(start: char, end: char) -> ClassUnicodeRange`
  Create a new Unicode scalar value range for a character class.

- `fn start(self: &Self) -> char`
  Return the start of this range.

- `fn end(self: &Self) -> char`
  Return the end of this range.

- `fn len(self: &Self) -> usize`
  Returns the number of codepoints in this range.

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

- `fn clone(self: &Self) -> ClassUnicodeRange`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &ClassUnicodeRange) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassUnicodeRange) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &ClassUnicodeRange) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> ClassUnicodeRange`

### `ClassBytes`

```rust
struct ClassBytes {
    // [REDACTED: Private Fields]
}
```

A set of characters represented by arbitrary bytes.

Each byte corresponds to one character.

#### Implementations

- `fn new<I>(ranges: I) -> ClassBytes`
  Create a new class from a sequence of ranges.

- `fn empty() -> ClassBytes`
  Create a new class with no ranges.

- `fn push(self: &mut Self, range: ClassBytesRange)`
  Add a new range to this set.

- `fn iter(self: &Self) -> ClassBytesIter<'_>`
  Return an iterator over all ranges in this class.

- `fn ranges(self: &Self) -> &[ClassBytesRange]`
  Return the underlying ranges as a slice.

- `fn case_fold_simple(self: &mut Self)`
  Expand this character class such that it contains all case folded

- `fn negate(self: &mut Self)`
  Negate this byte class.

- `fn union(self: &mut Self, other: &ClassBytes)`
  Union this byte class with the given byte class, in place.

- `fn intersect(self: &mut Self, other: &ClassBytes)`
  Intersect this byte class with the given byte class, in place.

- `fn difference(self: &mut Self, other: &ClassBytes)`
  Subtract the given byte class from this byte class, in place.

- `fn symmetric_difference(self: &mut Self, other: &ClassBytes)`
  Compute the symmetric difference of the given byte classes, in place.

- `fn is_ascii(self: &Self) -> bool`
  Returns true if and only if this character class will either match

- `fn minimum_len(self: &Self) -> Option<usize>`
  Returns the length, in bytes, of the smallest string matched by this

- `fn maximum_len(self: &Self) -> Option<usize>`
  Returns the length, in bytes, of the longest string matched by this

- `fn literal(self: &Self) -> Option<Vec<u8>>`
  If this class consists of exactly one byte, then return it as

- `fn to_unicode_class(self: &Self) -> Option<ClassUnicode>`
  If this class consists of only ASCII ranges, then return its

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

- `fn clone(self: &Self) -> ClassBytes`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassBytes) -> bool`

##### `impl StructuralPartialEq`

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

### `ClassBytesIter<'a>`

```rust
struct ClassBytesIter<'a>();
```

An iterator over all ranges in a byte character class.

The lifetime `'a` refers to the lifetime of the underlying class.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'a>`

- `type Item = &'a ClassBytesRange`

- `fn next(self: &mut Self) -> Option<&'a ClassBytesRange>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ClassBytesRange`

```rust
struct ClassBytesRange {
    // [REDACTED: Private Fields]
}
```

A single range of characters represented by arbitrary bytes.

The range is closed. That is, the start and end of the range are included
in the range.

#### Implementations

- `fn new(start: u8, end: u8) -> ClassBytesRange`
  Create a new byte range for a character class.

- `fn start(self: &Self) -> u8`
  Return the start of this range.

- `fn end(self: &Self) -> u8`
  Return the end of this range.

- `fn len(self: &Self) -> usize`
  Returns the number of bytes in this range.

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

- `fn clone(self: &Self) -> ClassBytesRange`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &ClassBytesRange) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ClassBytesRange) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &ClassBytesRange) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> ClassBytesRange`

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

- `fn clone(self: &Self) -> Capture`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Capture) -> bool`

##### `impl StructuralPartialEq`

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

- `fn with(self: &Self, sub: Hir) -> Repetition`
  Returns a new repetition with the same `min`, `max` and `greedy`

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

- `fn clone(self: &Self) -> Repetition`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Repetition) -> bool`

##### `impl StructuralPartialEq`

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

### `Properties`

```rust
struct Properties();
```

A type that collects various properties of an HIR value.

Properties are always scalar values and represent meta data that is
computed inductively on an HIR value. Properties are defined for all
HIR values.

All methods on a `Properties` value take constant time and are meant to
be cheap to call.

#### Implementations

- `fn minimum_len(self: &Self) -> Option<usize>`
  Returns the length (in bytes) of the smallest string matched by this

- `fn maximum_len(self: &Self) -> Option<usize>`
  Returns the length (in bytes) of the longest string matched by this

- `fn look_set(self: &Self) -> LookSet`
  Returns a set of all look-around assertions that appear at least once

- `fn look_set_prefix(self: &Self) -> LookSet`
  Returns a set of all look-around assertions that appear as a prefix for

- `fn look_set_prefix_any(self: &Self) -> LookSet`
  Returns a set of all look-around assertions that appear as a _possible_

- `fn look_set_suffix(self: &Self) -> LookSet`
  Returns a set of all look-around assertions that appear as a suffix for

- `fn look_set_suffix_any(self: &Self) -> LookSet`
  Returns a set of all look-around assertions that appear as a _possible_

- `fn is_utf8(self: &Self) -> bool`
  Return true if and only if the corresponding HIR will always match

- `fn explicit_captures_len(self: &Self) -> usize`
  Returns the total number of explicit capturing groups in the

- `fn static_explicit_captures_len(self: &Self) -> Option<usize>`
  Returns the total number of explicit capturing groups that appear in

- `fn is_literal(self: &Self) -> bool`
  Return true if and only if this HIR is a simple literal. This is

- `fn is_alternation_literal(self: &Self) -> bool`
  Return true if and only if this HIR is either a simple literal or an

- `fn memory_usage(self: &Self) -> usize`
  Returns the total amount of heap memory usage, in bytes, used by this

- `fn union<I, P>(props: I) -> Properties`
  Returns a new set of properties that corresponds to the union of the

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

- `fn clone(self: &Self) -> Properties`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Properties) -> bool`

##### `impl StructuralPartialEq`

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

### `LookSet`

```rust
struct LookSet {
    pub bits: u32,
}
```

A set of look-around assertions.

This is useful for efficiently tracking look-around assertions. For
example, an [`Hir`](regex_syntax/hir/index.md) provides properties that return `LookSet`s.

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

- `fn empty() -> LookSet`
  Create an empty set of look-around assertions.

- `fn full() -> LookSet`
  Create a full set of look-around assertions.

- `fn singleton(look: Look) -> LookSet`
  Create a look-around set containing the look-around assertion given.

- `fn len(self: Self) -> usize`
  Returns the total number of look-around assertions in this set.

- `fn is_empty(self: Self) -> bool`
  Returns true if and only if this set is empty.

- `fn contains(self: Self, look: Look) -> bool`
  Returns true if and only if the given look-around assertion is in this

- `fn contains_anchor(self: &Self) -> bool`
  Returns true if and only if this set contains any anchor assertions.

- `fn contains_anchor_haystack(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of

- `fn contains_anchor_line(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of line"

- `fn contains_anchor_lf(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of line"

- `fn contains_anchor_crlf(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of line"

- `fn contains_word(self: Self) -> bool`
  Returns true if and only if this set contains any word boundary or

- `fn contains_word_unicode(self: Self) -> bool`
  Returns true if and only if this set contains any Unicode word boundary

- `fn contains_word_ascii(self: Self) -> bool`
  Returns true if and only if this set contains any ASCII word boundary

- `fn iter(self: Self) -> LookSetIter`
  Returns an iterator over all of the look-around assertions in this set.

- `fn insert(self: Self, look: Look) -> LookSet`
  Return a new set that is equivalent to the original, but with the given

- `fn set_insert(self: &mut Self, look: Look)`
  Updates this set in place with the result of inserting the given

- `fn remove(self: Self, look: Look) -> LookSet`
  Return a new set that is equivalent to the original, but with the given

- `fn set_remove(self: &mut Self, look: Look)`
  Updates this set in place with the result of removing the given

- `fn subtract(self: Self, other: LookSet) -> LookSet`
  Returns a new set that is the result of subtracting the given set from

- `fn set_subtract(self: &mut Self, other: LookSet)`
  Updates this set in place with the result of subtracting the given set

- `fn union(self: Self, other: LookSet) -> LookSet`
  Returns a new set that is the union of this and the one given.

- `fn set_union(self: &mut Self, other: LookSet)`
  Updates this set in place with the result of unioning it with the one

- `fn intersect(self: Self, other: LookSet) -> LookSet`
  Returns a new set that is the intersection of this and the one given.

- `fn set_intersect(self: &mut Self, other: LookSet)`
  Updates this set in place with the result of intersecting it with the

- `fn read_repr(slice: &[u8]) -> LookSet`
  Return a `LookSet` from the slice given as a native endian 32-bit

- `fn write_repr(self: Self, slice: &mut [u8])`
  Write a `LookSet` as a native endian 32-bit integer to the beginning

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

- `fn clone(self: &Self) -> LookSet`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LookSet) -> bool`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> LookSet`

### `LookSetIter`

```rust
struct LookSetIter {
    // [REDACTED: Private Fields]
}
```

An iterator over all look-around assertions in a [`LookSet`](regex_syntax/hir/index.md).

This iterator is created by [`LookSet::iter`](#iter).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> LookSetIter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Iterator`

- `type Item = Look`

- `fn next(self: &mut Self) -> Option<Look>`

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

- `fn clone(self: &Self) -> ErrorKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ErrorKind) -> bool`

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

The underlying kind of an arbitrary [`Hir`](regex_syntax/hir/index.md) expression.

An `HirKind` is principally useful for doing case analysis on the type
of a regular expression. If you're looking to build new `Hir` values,
then you _must_ use the smart constructors defined on `Hir`, like
[`Hir::repetition`](#repetition), to build new `Hir` values. The API intentionally does
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

- `fn subs(self: &Self) -> &[Hir]`
  Returns a slice of this kind's sub-expressions, if any.

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

- `fn clone(self: &Self) -> HirKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HirKind) -> bool`

##### `impl StructuralPartialEq`

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
  Apply Unicode simple case folding to this character class, in place.

- `fn try_case_fold_simple(self: &mut Self) -> core::result::Result<(), CaseFoldError>`
  Apply Unicode simple case folding to this character class, in place.

- `fn negate(self: &mut Self)`
  Negate this character class in place.

- `fn is_utf8(self: &Self) -> bool`
  Returns true if and only if this character class will only ever match

- `fn minimum_len(self: &Self) -> Option<usize>`
  Returns the length, in bytes, of the smallest string matched by this

- `fn maximum_len(self: &Self) -> Option<usize>`
  Returns the length, in bytes, of the longest string matched by this

- `fn is_empty(self: &Self) -> bool`
  Returns true if and only if this character class is empty. That is,

- `fn literal(self: &Self) -> Option<Vec<u8>>`
  If this class consists of exactly one element (whether a codepoint or a

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

- `fn clone(self: &Self) -> Class`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Class) -> bool`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

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

- `const fn reversed(self: Self) -> Look`
  Flip the look-around assertion to its equivalent for reverse searches.

- `const fn as_repr(self: Self) -> u32`
  Return the underlying representation of this look-around enumeration

- `const fn from_repr(repr: u32) -> Option<Look>`
  Given the underlying representation of a `Look` value, return the

- `const fn as_char(self: Self) -> char`
  Returns a convenient single codepoint representation of this

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

- `fn clone(self: &Self) -> Look`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Look) -> bool`

##### `impl StructuralPartialEq`

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

This type is meant to be used with [`Hir::dot`](#dot), which is a convenience
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

- `fn clone(self: &Self) -> Dot`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Dot) -> bool`

##### `impl StructuralPartialEq`

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

## Traits

## Functions

