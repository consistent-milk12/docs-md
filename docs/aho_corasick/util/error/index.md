*[aho_corasick](../../index.md) / [util](../index.md) / [error](index.md)*

---

# Module `error`

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: ErrorKind,
}
```

An error that occurred during the construction of an Aho-Corasick
automaton.

Build errors occur when some kind of limit has been exceeded, either in the
number of states, the number of patterns of the length of a pattern. These
limits aren't part of the public API, but they should generally be large
enough to handle most use cases.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- `fn state_id_overflow(max: u64, requested_max: u64) -> BuildError` — [`BuildError`](#builderror)

- `fn pattern_id_overflow(max: u64, requested_max: u64) -> BuildError` — [`BuildError`](#builderror)

- `fn pattern_too_long(pattern: PatternID, len: usize) -> BuildError` — [`PatternID`](../primitives/index.md), [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- `fn clone(self: &Self) -> BuildError` — [`BuildError`](#builderror)

##### `impl Debug for BuildError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for BuildError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

##### `impl<T> ToString for BuildError`

- `fn to_string(self: &Self) -> String`

### `MatchError`

```rust
struct MatchError(alloc::boxed::Box<MatchErrorKind>);
```

An error that occurred during an Aho-Corasick search.

An error that occurs during a search is limited to some kind of
misconfiguration that resulted in an illegal call. Stated differently,
whether an error occurs is not dependent on the specific bytes in the
haystack.

Examples of misconfiguration:

* Executing a stream or overlapping search on a searcher that was built was
something other than [`MatchKind::Standard`](crate::MatchKind::Standard)
semantics.
* Requested an anchored or an unanchored search on a searcher that doesn't
support unanchored or anchored searches, respectively.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- `fn new(kind: MatchErrorKind) -> MatchError` — [`MatchErrorKind`](#matcherrorkind), [`MatchError`](#matcherror)

- `fn kind(self: &Self) -> &MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

- `fn invalid_input_anchored() -> MatchError` — [`MatchError`](#matcherror)

- `fn invalid_input_unanchored() -> MatchError` — [`MatchError`](#matcherror)

- `fn unsupported_stream(got: MatchKind) -> MatchError` — [`MatchKind`](../search/index.md), [`MatchError`](#matcherror)

- `fn unsupported_overlapping(got: MatchKind) -> MatchError` — [`MatchKind`](../search/index.md), [`MatchError`](#matcherror)

- `fn unsupported_empty() -> MatchError` — [`MatchError`](#matcherror)

#### Trait Implementations

##### `impl Clone for MatchError`

- `fn clone(self: &Self) -> MatchError` — [`MatchError`](#matcherror)

##### `impl Debug for MatchError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for MatchError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl PartialEq for MatchError`

- `fn eq(self: &Self, other: &MatchError) -> bool` — [`MatchError`](#matcherror)

##### `impl StructuralPartialEq for MatchError`

##### `impl<T> ToString for MatchError`

- `fn to_string(self: &Self) -> String`

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    StateIDOverflow {
        max: u64,
        requested_max: u64,
    },
    PatternIDOverflow {
        max: u64,
        requested_max: u64,
    },
    PatternTooLong {
        pattern: crate::util::primitives::PatternID,
        len: usize,
    },
}
```

The kind of error that occurred.

#### Variants

- **`StateIDOverflow`**

  An error that occurs when allocating a new state would result in an
  identifier that exceeds the capacity of a `StateID`.

- **`PatternIDOverflow`**

  An error that occurs when adding a pattern to an Aho-Corasick
  automaton would result in an identifier that exceeds the capacity of a
  `PatternID`.

- **`PatternTooLong`**

  Occurs when a pattern string is given to the Aho-Corasick constructor
  that is too long.

#### Trait Implementations

##### `impl Clone for ErrorKind`

- `fn clone(self: &Self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Debug for ErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MatchErrorKind`

```rust
enum MatchErrorKind {
    InvalidInputAnchored,
    InvalidInputUnanchored,
    UnsupportedStream {
        got: crate::util::search::MatchKind,
    },
    UnsupportedOverlapping {
        got: crate::util::search::MatchKind,
    },
    UnsupportedEmpty,
}
```

The underlying kind of a [`MatchError`](#matcherror).

This is a **non-exhaustive** enum. That means new variants may be added in
a semver-compatible release.

#### Variants

- **`InvalidInputAnchored`**

  An error indicating that an anchored search was requested, but from a
  searcher that was built without anchored support.

- **`InvalidInputUnanchored`**

  An error indicating that an unanchored search was requested, but from a
  searcher that was built without unanchored support.

- **`UnsupportedStream`**

  An error indicating that a stream search was attempted on an
  Aho-Corasick automaton with an unsupported `MatchKind`.

- **`UnsupportedOverlapping`**

  An error indicating that an overlapping search was attempted on an
  Aho-Corasick automaton with an unsupported `MatchKind`.

- **`UnsupportedEmpty`**

  An error indicating that the operation requested doesn't support
  automatons that contain an empty pattern string.

#### Trait Implementations

##### `impl Clone for MatchErrorKind`

- `fn clone(self: &Self) -> MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

##### `impl Debug for MatchErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl PartialEq for MatchErrorKind`

- `fn eq(self: &Self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](#matcherrorkind)

##### `impl StructuralPartialEq for MatchErrorKind`

