*[aho_corasick](../../index.md) / [util](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BuildError`](#builderror) | struct | An error that occurred during the construction of an Aho-Corasick automaton. |
| [`MatchError`](#matcherror) | struct | An error that occurred during an Aho-Corasick search. |
| [`ErrorKind`](#errorkind) | enum | The kind of error that occurred. |
| [`MatchErrorKind`](#matcherrorkind) | enum | The underlying kind of a [`MatchError`]. |

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: ErrorKind,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:17-19`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/error.rs#L17-L19)*

An error that occurred during the construction of an Aho-Corasick
automaton.

Build errors occur when some kind of limit has been exceeded, either in the
number of states, the number of patterns of the length of a pattern. These
limits aren't part of the public API, but they should generally be large
enough to handle most use cases.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="builderror-state-id-overflow"></span>`fn state_id_overflow(max: u64, requested_max: u64) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-pattern-id-overflow"></span>`fn pattern_id_overflow(max: u64, requested_max: u64) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-pattern-too-long"></span>`fn pattern_too_long(pattern: PatternID, len: usize) -> BuildError` — [`PatternID`](../primitives/index.md), [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](#builderror)

##### `impl Debug for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

##### `impl ToString for BuildError`

- <span id="builderror-to-string"></span>`fn to_string(&self) -> String`

### `MatchError`

```rust
struct MatchError(alloc::boxed::Box<MatchErrorKind>);
```

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:130`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/error.rs#L130)*

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

- <span id="matcherror-new"></span>`fn new(kind: MatchErrorKind) -> MatchError` — [`MatchErrorKind`](#matcherrorkind), [`MatchError`](#matcherror)

- <span id="matcherror-kind"></span>`fn kind(&self) -> &MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

- <span id="matcherror-invalid-input-anchored"></span>`fn invalid_input_anchored() -> MatchError` — [`MatchError`](#matcherror)

- <span id="matcherror-invalid-input-unanchored"></span>`fn invalid_input_unanchored() -> MatchError` — [`MatchError`](#matcherror)

- <span id="matcherror-unsupported-stream"></span>`fn unsupported_stream(got: MatchKind) -> MatchError` — [`MatchKind`](../search/index.md), [`MatchError`](#matcherror)

- <span id="matcherror-unsupported-overlapping"></span>`fn unsupported_overlapping(got: MatchKind) -> MatchError` — [`MatchKind`](../search/index.md), [`MatchError`](#matcherror)

- <span id="matcherror-unsupported-empty"></span>`fn unsupported_empty() -> MatchError` — [`MatchError`](#matcherror)

#### Trait Implementations

##### `impl Clone for MatchError`

- <span id="matcherror-clone"></span>`fn clone(&self) -> MatchError` — [`MatchError`](#matcherror)

##### `impl Debug for MatchError`

- <span id="matcherror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for MatchError`

- <span id="matcherror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl PartialEq for MatchError`

- <span id="matcherror-eq"></span>`fn eq(&self, other: &MatchError) -> bool` — [`MatchError`](#matcherror)

##### `impl StructuralPartialEq for MatchError`

##### `impl ToString for MatchError`

- <span id="matcherror-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:23-49`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/error.rs#L23-L49)*

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

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Debug for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:200-222`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/error.rs#L200-L222)*

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

- <span id="matcherrorkind-clone"></span>`fn clone(&self) -> MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

##### `impl Debug for MatchErrorKind`

- <span id="matcherrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl PartialEq for MatchErrorKind`

- <span id="matcherrorkind-eq"></span>`fn eq(&self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](#matcherrorkind)

##### `impl StructuralPartialEq for MatchErrorKind`

