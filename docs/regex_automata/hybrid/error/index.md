*[regex_automata](../../index.md) / [hybrid](../index.md) / [error](index.md)*

---

# Module `error`

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

- `fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md)

- `fn insufficient_cache_capacity(minimum: usize, given: usize) -> BuildError` — [`BuildError`](#builderror)

- `fn insufficient_state_id_capacity(err: LazyStateIDError) -> BuildError` — [`LazyStateIDError`](../id/index.md), [`BuildError`](#builderror)

- `fn unsupported_dfa_word_boundary_unicode() -> BuildError` — [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- `fn clone(self: &Self) -> BuildError` — [`BuildError`](#builderror)

##### `impl Debug for BuildError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for BuildError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for BuildError`

- `fn to_string(self: &Self) -> String`

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

- `fn too_many_cache_clears() -> CacheError` — [`CacheError`](#cacheerror)

- `fn bad_efficiency() -> CacheError` — [`CacheError`](#cacheerror)

#### Trait Implementations

##### `impl Clone for CacheError`

- `fn clone(self: &Self) -> CacheError` — [`CacheError`](#cacheerror)

##### `impl Debug for CacheError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for CacheError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CacheError`

##### `impl<T> ToString for CacheError`

- `fn to_string(self: &Self) -> String`

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

- `fn clone(self: &Self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl Debug for BuildErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn cache(err: CacheError) -> StartError` — [`CacheError`](#cacheerror), [`StartError`](#starterror)

- `fn quit(byte: u8) -> StartError` — [`StartError`](#starterror)

- `fn unsupported_anchored(mode: Anchored) -> StartError` — [`Anchored`](../../index.md), [`StartError`](#starterror)

#### Trait Implementations

##### `impl Clone for StartError`

- `fn clone(self: &Self) -> StartError` — [`StartError`](#starterror)

##### `impl Debug for StartError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for StartError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for StartError`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for StartError`

- `fn to_string(self: &Self) -> String`

