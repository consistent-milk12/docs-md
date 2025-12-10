*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BuildError`](#builderror) | struct | An error that can occurred during the construction of a thompson NFA. |
| [`BuildErrorKind`](#builderrorkind) | enum | The kind of error that occurred during the construction of a thompson NFA. |

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/error.rs:21-23`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/error.rs#L21-L23)*

An error that can occurred during the construction of a thompson NFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`regex_syntax::Error`](../../../../regex_syntax/error/index.md) type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building an NFA directly from a pattern string.

Otherwise, errors typically occur when a limit has been breached. For
example, if the total heap usage of the compiled NFA exceeds the limit
set by [`Config::nfa_size_limit`](crate::nfa::thompson::Config), then
building the NFA will fail.

#### Implementations

- <span id="builderror-size-limit"></span>`fn size_limit(&self) -> Option<usize>`

- <span id="builderror-kind"></span>`fn kind(&self) -> &BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

- <span id="builderror-syntax"></span>`fn syntax(err: regex_syntax::Error) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-captures"></span>`fn captures(err: captures::GroupInfoError) -> BuildError` — [`GroupInfoError`](../../../util/captures/index.md#groupinfoerror), [`BuildError`](#builderror)

- <span id="builderror-word"></span>`fn word(err: look::UnicodeWordBoundaryError) -> BuildError` — [`UnicodeWordBoundaryError`](../../../util/look/index.md#unicodewordboundaryerror), [`BuildError`](#builderror)

- <span id="builderror-too-many-patterns"></span>`fn too_many_patterns(given: usize) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-too-many-states"></span>`fn too_many_states(given: usize) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-exceeded-size-limit"></span>`fn exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-invalid-capture-index"></span>`fn invalid_capture_index(index: u32) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-unsupported-captures"></span>`fn unsupported_captures() -> BuildError` — [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](#builderror)

##### `impl Debug for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- <span id="builderror-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for BuildError`

- <span id="builderror-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/error.rs:27-76`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/error.rs#L27-L76)*

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

- <span id="builderrorkind-clone"></span>`fn clone(&self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl Debug for BuildErrorKind`

- <span id="builderrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

