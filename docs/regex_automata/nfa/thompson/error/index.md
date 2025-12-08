*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [error](index.md)*

---

# Module `error`

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

An error that can occurred during the construction of a thompson NFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`regex_syntax::Error`](../../../../regex_syntax/hir/index.md) type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building an NFA directly from a pattern string.

Otherwise, errors typically occur when a limit has been breached. For
example, if the total heap usage of the compiled NFA exceeds the limit
set by [`Config::nfa_size_limit`](crate::nfa::thompson::Config), then
building the NFA will fail.

#### Implementations

- `fn size_limit(self: &Self) -> Option<usize>`

- `fn kind(self: &Self) -> &BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

- `fn syntax(err: regex_syntax::Error) -> BuildError` — [`BuildError`](../index.md)

- `fn captures(err: captures::GroupInfoError) -> BuildError` — [`GroupInfoError`](../../../util/captures/index.md), [`BuildError`](../index.md)

- `fn word(err: look::UnicodeWordBoundaryError) -> BuildError` — [`UnicodeWordBoundaryError`](../../../util/look/index.md), [`BuildError`](../index.md)

- `fn too_many_patterns(given: usize) -> BuildError` — [`BuildError`](../index.md)

- `fn too_many_states(given: usize) -> BuildError` — [`BuildError`](../index.md)

- `fn exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](../index.md)

- `fn invalid_capture_index(index: u32) -> BuildError` — [`BuildError`](../index.md)

- `fn unsupported_captures() -> BuildError` — [`BuildError`](../index.md)

#### Trait Implementations

##### `impl Clone for BuildError`

- `fn clone(self: &Self) -> BuildError` — [`BuildError`](../index.md)

##### `impl Debug for BuildError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for BuildError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for BuildError`

- `fn to_string(self: &Self) -> String`

## Enums

### `BuildErrorKind`

```rust
enum BuildErrorKind {
    Syntax(regex_syntax::Error),
    Captures(captures::GroupInfoError),
    Word(look::UnicodeWordBoundaryError),
    TooManyPatterns {
        given: usize,
        limit: usize,
    },
    TooManyStates {
        given: usize,
        limit: usize,
    },
    ExceededSizeLimit {
        limit: usize,
    },
    InvalidCaptureIndex {
        index: u32,
    },
    UnsupportedCaptures,
}
```

The kind of error that occurred during the construction of a thompson NFA.

#### Variants

- **`Syntax`**

  An error that occurred while parsing a regular expression. Note that
  this error may be printed over multiple lines, and is generally
  intended to be end user readable on its own.

- **`Captures`**

  An error that occurs if the capturing groups provided to an NFA builder
  do not satisfy the documented invariants. For example, things like
  too many groups, missing groups, having the first (zeroth) group be
  named or duplicate group names within the same pattern.

- **`Word`**

  An error that occurs when an NFA contains a Unicode word boundary, but
  where the crate was compiled without the necessary data for dealing
  with Unicode word boundaries.

- **`TooManyPatterns`**

  An error that occurs if too many patterns were given to the NFA
  compiler.

- **`TooManyStates`**

  An error that occurs if too states are produced while building an NFA.

- **`ExceededSizeLimit`**

  An error that occurs when NFA compilation exceeds a configured heap
  limit.

- **`InvalidCaptureIndex`**

  An error that occurs when an invalid capture group index is added to
  the NFA. An "invalid" index can be one that would otherwise overflow
  a `usize` on the current target.

- **`UnsupportedCaptures`**

  An error that occurs when one tries to build a reverse NFA with
  captures enabled. Currently, this isn't supported, but we probably
  should support it at some point.

#### Trait Implementations

##### `impl Clone for BuildErrorKind`

- `fn clone(self: &Self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl Debug for BuildErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

