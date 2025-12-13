*[regex_automata](../../index.md) / [hybrid](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BuildError`](#builderror) | struct | An error that occurs when initial construction of a lazy DFA fails. |
| [`CacheError`](#cacheerror) | struct | An error that occurs when cache usage has become inefficient. |
| [`BuildErrorKind`](#builderrorkind) | enum |  |
| [`StartError`](#starterror) | enum | An error that can occur when computing the start state for a search. |

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/error.rs:23-25`](../../../../.source_1765521767/regex-automata-0.4.13/src/hybrid/error.rs#L23-L25)*

An error that occurs when initial construction of a lazy DFA fails.

A build error can occur when insufficient cache capacity is configured or
if something about the NFA is unsupported. (For example, if one attempts
to build a lazy DFA without heuristic Unicode support but with an NFA that
contains a Unicode word boundary.)

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying
[`nfa::thompson::BuildError`](crate::nfa::thompson::BuildError)
type from its `source` method via the `std::error::Error` trait. This error
only occurs when using convenience routines for building a lazy DFA
directly from a pattern string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="builderror-nfa"></span>`fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md#builderror)

- <span id="builderror-insufficient-cache-capacity"></span>`fn insufficient_cache_capacity(minimum: usize, given: usize) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-insufficient-state-id-capacity"></span>`fn insufficient_state_id_capacity(err: LazyStateIDError) -> BuildError` — [`LazyStateIDError`](../id/index.md#lazystateiderror), [`BuildError`](#builderror)

- <span id="builderror-unsupported-dfa-word-boundary-unicode"></span>`fn unsupported_dfa_word_boundary_unicode() -> BuildError` — [`BuildError`](#builderror)

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

- <span id="builderror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

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

### `CacheError`

```rust
struct CacheError(());
```

*Defined in [`regex-automata-0.4.13/src/hybrid/error.rs:222`](../../../../.source_1765521767/regex-automata-0.4.13/src/hybrid/error.rs#L222)*

An error that occurs when cache usage has become inefficient.

One of the weaknesses of a lazy DFA is that it may need to clear its
cache repeatedly if it's not big enough. If this happens too much, then it
can slow searching down significantly. A mitigation to this is to use
heuristics to detect whether the cache is being used efficiently or not.
If not, then a lazy DFA can return a `CacheError`.

The default configuration of a lazy DFA in this crate is
set such that a `CacheError` will never occur. Instead,
callers must opt into this behavior with settings like
[`dfa::Config::minimum_cache_clear_count`](crate::hybrid::dfa::Config::minimum_cache_clear_count)
and
[`dfa::Config::minimum_bytes_per_state`](crate::hybrid::dfa::Config::minimum_bytes_per_state).

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="cacheerror-too-many-cache-clears"></span>`fn too_many_cache_clears() -> CacheError` — [`CacheError`](#cacheerror)

- <span id="cacheerror-bad-efficiency"></span>`fn bad_efficiency() -> CacheError` — [`CacheError`](#cacheerror)

#### Trait Implementations

##### `impl Any for CacheError`

- <span id="cacheerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CacheError`

- <span id="cacheerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CacheError`

- <span id="cacheerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CacheError`

- <span id="cacheerror-clone"></span>`fn clone(&self) -> CacheError` — [`CacheError`](#cacheerror)

##### `impl CloneToUninit for CacheError`

- <span id="cacheerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CacheError`

- <span id="cacheerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CacheError`

- <span id="cacheerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CacheError`

##### `impl<T> From for CacheError`

- <span id="cacheerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CacheError`

- <span id="cacheerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CacheError`

- <span id="cacheerror-toowned-type-owned"></span>`type Owned = T`

- <span id="cacheerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cacheerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for CacheError`

- <span id="cacheerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for CacheError`

- <span id="cacheerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cacheerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CacheError`

- <span id="cacheerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cacheerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `BuildErrorKind`

```rust
enum BuildErrorKind {
    NFA(nfa::thompson::BuildError),
    InsufficientCacheCapacity {
        minimum: usize,
        given: usize,
    },
    InsufficientStateIDCapacity {
        err: crate::hybrid::id::LazyStateIDError,
    },
    Unsupported(&'static str),
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/error.rs:28-33`](../../../../.source_1765521767/regex-automata-0.4.13/src/hybrid/error.rs#L28-L33)*

#### Trait Implementations

##### `impl Any for BuildErrorKind`

- <span id="builderrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildErrorKind`

- <span id="builderrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildErrorKind`

- <span id="builderrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildErrorKind`

- <span id="builderrorkind-clone"></span>`fn clone(&self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl CloneToUninit for BuildErrorKind`

- <span id="builderrorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildErrorKind`

- <span id="builderrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BuildErrorKind`

- <span id="builderrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildErrorKind`

- <span id="builderrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BuildErrorKind`

- <span id="builderrorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="builderrorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builderrorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BuildErrorKind`

- <span id="builderrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builderrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildErrorKind`

- <span id="builderrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builderrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StartError`

```rust
enum StartError {
    Cache {
        err: CacheError,
    },
    Quit {
        byte: u8,
    },
    UnsupportedAnchored {
        mode: crate::util::search::Anchored,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/error.rs:117-136`](../../../../.source_1765521767/regex-automata-0.4.13/src/hybrid/error.rs#L117-L136)*

An error that can occur when computing the start state for a search.

Computing a start state can fail for a few reasons, either
based on incorrect configuration or even based on whether
the look-behind byte triggers a quit state. Typically
one does not need to handle this error if you're using
[`DFA::start_state_forward`](crate::hybrid::dfa::DFA::start_state_forward)
(or its reverse counterpart), as that routine automatically converts
`StartError` to a [`MatchError`](crate::MatchError) for you.

This error may be returned by the
[`DFA::start_state`](crate::hybrid::dfa::DFA::start_state) routine.

This error implements the `std::error::Error` trait when the `std` feature
is enabled.

This error is marked as non-exhaustive. New variants may be added in a
semver compatible release.

#### Variants

- **`Cache`**

  An error that occurs when cache inefficiency has dropped below the
  configured heuristic thresholds.

- **`Quit`**

  An error that occurs when a starting configuration's look-behind byte
  is in this DFA's quit set.

- **`UnsupportedAnchored`**

  An error that occurs when the caller requests an anchored mode that
  isn't supported by the DFA.

#### Implementations

- <span id="starterror-cache"></span>`fn cache(err: CacheError) -> StartError` — [`CacheError`](#cacheerror), [`StartError`](#starterror)

- <span id="starterror-quit"></span>`fn quit(byte: u8) -> StartError` — [`StartError`](#starterror)

- <span id="starterror-unsupported-anchored"></span>`fn unsupported_anchored(mode: Anchored) -> StartError` — [`Anchored`](../../index.md#anchored), [`StartError`](#starterror)

#### Trait Implementations

##### `impl Any for StartError`

- <span id="starterror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartError`

- <span id="starterror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartError`

- <span id="starterror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartError`

- <span id="starterror-clone"></span>`fn clone(&self) -> StartError` — [`StartError`](#starterror)

##### `impl CloneToUninit for StartError`

- <span id="starterror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StartError`

- <span id="starterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StartError`

- <span id="starterror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for StartError`

- <span id="starterror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> From for StartError`

- <span id="starterror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartError`

- <span id="starterror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StartError`

- <span id="starterror-toowned-type-owned"></span>`type Owned = T`

- <span id="starterror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="starterror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StartError`

- <span id="starterror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StartError`

- <span id="starterror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="starterror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartError`

- <span id="starterror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="starterror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

