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

- <span id="builderror-nfa"></span>`fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/index.md)

- <span id="builderror-insufficient-cache-capacity"></span>`fn insufficient_cache_capacity(minimum: usize, given: usize) -> BuildError` — [`BuildError`](../index.md)

- <span id="builderror-insufficient-state-id-capacity"></span>`fn insufficient_state_id_capacity(err: LazyStateIDError) -> BuildError` — [`LazyStateIDError`](../id/index.md), [`BuildError`](../index.md)

- <span id="builderror-unsupported-dfa-word-boundary-unicode"></span>`fn unsupported_dfa_word_boundary_unicode() -> BuildError` — [`BuildError`](../index.md)

#### Trait Implementations

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](../index.md)

##### `impl Debug for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- <span id="builderror-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for BuildError`

- <span id="builderror-to-string"></span>`fn to_string(&self) -> String`

### `CacheError`

```rust
struct CacheError(());
```

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

- <span id="cacheerror-too-many-cache-clears"></span>`fn too_many_cache_clears() -> CacheError` — [`CacheError`](../index.md)

- <span id="cacheerror-bad-efficiency"></span>`fn bad_efficiency() -> CacheError` — [`CacheError`](../index.md)

#### Trait Implementations

##### `impl Clone for CacheError`

- <span id="cacheerror-clone"></span>`fn clone(&self) -> CacheError` — [`CacheError`](../index.md)

##### `impl Debug for CacheError`

- <span id="cacheerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CacheError`

- <span id="cacheerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CacheError`

##### `impl<T> ToString for CacheError`

- <span id="cacheerror-to-string"></span>`fn to_string(&self) -> String`

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

#### Trait Implementations

##### `impl Clone for BuildErrorKind`

- <span id="builderrorkind-clone"></span>`fn clone(&self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl Debug for BuildErrorKind`

- <span id="builderrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="starterror-cache"></span>`fn cache(err: CacheError) -> StartError` — [`CacheError`](../index.md), [`StartError`](../index.md)

- <span id="starterror-quit"></span>`fn quit(byte: u8) -> StartError` — [`StartError`](../index.md)

- <span id="starterror-unsupported-anchored"></span>`fn unsupported_anchored(mode: Anchored) -> StartError` — [`Anchored`](../../index.md), [`StartError`](../index.md)

#### Trait Implementations

##### `impl Clone for StartError`

- <span id="starterror-clone"></span>`fn clone(&self) -> StartError` — [`StartError`](../index.md)

##### `impl Debug for StartError`

- <span id="starterror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StartError`

- <span id="starterror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for StartError`

- <span id="starterror-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for StartError`

- <span id="starterror-to-string"></span>`fn to_string(&self) -> String`

