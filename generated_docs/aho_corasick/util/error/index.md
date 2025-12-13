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

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:17-19`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/error.rs#L17-L19)*

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

- <span id="builderror-pattern-too-long"></span>`fn pattern_too_long(pattern: PatternID, len: usize) -> BuildError` — [`PatternID`](../primitives/index.md#patternid), [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Any for BuildError`

- <span id="builderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildError`

- <span id="builderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildError`

- <span id="builderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](#builderror)

##### `impl CloneToUninit for BuildError`

- <span id="builderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildError`

- <span id="builderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

##### `impl<T> From for BuildError`

- <span id="builderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildError`

- <span id="builderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BuildError`

- <span id="builderror-toowned-type-owned"></span>`type Owned = T`

- <span id="builderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for BuildError`

- <span id="builderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BuildError`

- <span id="builderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildError`

- <span id="builderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MatchError`

```rust
struct MatchError(alloc::boxed::Box<MatchErrorKind>);
```

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:130`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/error.rs#L130)*

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

  Create a new error value with the given kind.

  

  This is a more verbose version of the kind-specific constructors, e.g.,

  `MatchError::unsupported_stream`.

- <span id="matcherror-kind"></span>`fn kind(&self) -> &MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

  Returns a reference to the underlying error kind.

- <span id="matcherror-invalid-input-anchored"></span>`fn invalid_input_anchored() -> MatchError` — [`MatchError`](#matcherror)

  Create a new "invalid anchored search" error. This occurs when the

  caller requests an anchored search but where anchored searches aren't

  supported.

  

  This is the same as calling `MatchError::new` with a

  [`MatchErrorKind::InvalidInputAnchored`](../../index.md) kind.

- <span id="matcherror-invalid-input-unanchored"></span>`fn invalid_input_unanchored() -> MatchError` — [`MatchError`](#matcherror)

  Create a new "invalid unanchored search" error. This occurs when the

  caller requests an unanchored search but where unanchored searches

  aren't supported.

  

  This is the same as calling `MatchError::new` with a

  [`MatchErrorKind::InvalidInputUnanchored`](../../index.md) kind.

- <span id="matcherror-unsupported-stream"></span>`fn unsupported_stream(got: MatchKind) -> MatchError` — [`MatchKind`](../search/index.md#matchkind), [`MatchError`](#matcherror)

  Create a new "unsupported stream search" error. This occurs when the

  caller requests a stream search while using an Aho-Corasick automaton

  with a match kind other than [`MatchKind::Standard`](../../index.md).

  

  The match kind given should be the match kind of the automaton. It

  should never be `MatchKind::Standard`.

- <span id="matcherror-unsupported-overlapping"></span>`fn unsupported_overlapping(got: MatchKind) -> MatchError` — [`MatchKind`](../search/index.md#matchkind), [`MatchError`](#matcherror)

  Create a new "unsupported overlapping search" error. This occurs when

  the caller requests an overlapping search while using an Aho-Corasick

  automaton with a match kind other than [`MatchKind::Standard`](../../index.md).

  

  The match kind given should be the match kind of the automaton. It

  should never be `MatchKind::Standard`.

- <span id="matcherror-unsupported-empty"></span>`fn unsupported_empty() -> MatchError` — [`MatchError`](#matcherror)

  Create a new "unsupported empty pattern" error. This occurs when the

  caller requests a search for which matching an automaton that contains

  an empty pattern string is not supported.

#### Trait Implementations

##### `impl Any for MatchError`

- <span id="matcherror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchError`

- <span id="matcherror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchError`

- <span id="matcherror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchError`

- <span id="matcherror-clone"></span>`fn clone(&self) -> MatchError` — [`MatchError`](#matcherror)

##### `impl CloneToUninit for MatchError`

- <span id="matcherror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MatchError`

- <span id="matcherror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for MatchError`

- <span id="matcherror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl<T> From for MatchError`

- <span id="matcherror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchError`

- <span id="matcherror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MatchError`

- <span id="matcherror-partialeq-eq"></span>`fn eq(&self, other: &MatchError) -> bool` — [`MatchError`](#matcherror)

##### `impl StructuralPartialEq for MatchError`

##### `impl ToOwned for MatchError`

- <span id="matcherror-toowned-type-owned"></span>`type Owned = T`

- <span id="matcherror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matcherror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for MatchError`

- <span id="matcherror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for MatchError`

- <span id="matcherror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matcherror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchError`

- <span id="matcherror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matcherror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:23-49`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/error.rs#L23-L49)*

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

##### `impl<T> From for ErrorKind`

- <span id="errorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorKind`

- <span id="errorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ErrorKind`

- <span id="errorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="errorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="errorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ErrorKind`

- <span id="errorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorKind`

- <span id="errorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:200-222`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/error.rs#L200-L222)*

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

##### `impl Any for MatchErrorKind`

- <span id="matcherrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchErrorKind`

- <span id="matcherrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchErrorKind`

- <span id="matcherrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchErrorKind`

- <span id="matcherrorkind-clone"></span>`fn clone(&self) -> MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

##### `impl CloneToUninit for MatchErrorKind`

- <span id="matcherrorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MatchErrorKind`

- <span id="matcherrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl<T> From for MatchErrorKind`

- <span id="matcherrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchErrorKind`

- <span id="matcherrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MatchErrorKind`

- <span id="matcherrorkind-partialeq-eq"></span>`fn eq(&self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](#matcherrorkind)

##### `impl StructuralPartialEq for MatchErrorKind`

##### `impl ToOwned for MatchErrorKind`

- <span id="matcherrorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="matcherrorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matcherrorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MatchErrorKind`

- <span id="matcherrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matcherrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchErrorKind`

- <span id="matcherrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matcherrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

