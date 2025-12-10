*[regex_automata](../../index.md) / [meta](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BuildError`](#builderror) | struct | An error that occurs when construction of a `Regex` fails. |
| [`RetryQuadraticError`](#retryquadraticerror) | struct | An error that occurs when potential quadratic behavior has been detected when applying either the "reverse suffix" or "reverse inner" optimizations. |
| [`RetryFailError`](#retryfailerror) | struct | An error that occurs when a regex engine "gives up" for some reason before finishing a search. |
| [`BuildErrorKind`](#builderrorkind) | enum |  |
| [`RetryError`](#retryerror) | enum | An error that occurs when a search should be retried. |

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:27-29`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/error.rs#L27-L29)*

An error that occurs when construction of a `Regex` fails.

A build error is generally a result of one of two possible failure
modes. First is a parse or syntax error in the concrete syntax of a
pattern. Second is that the construction of the underlying regex matcher
fails, usually because it gets too big with respect to limits like
[`Config::nfa_size_limit`](crate::meta::Config::nfa_size_limit).

This error provides very little introspection capabilities. You can:

* Ask for the [`PatternID`](../../util/primitives/index.md) of the pattern that caused an error, if one
is available. This is available for things like syntax errors, but not for
cases where build limits are exceeded.
* Ask for the underlying syntax error, but only if the error is a syntax
error.
* Ask for a human readable message corresponding to the underlying error.
* The `BuildError::source` method (from the `std::error::Error`
trait implementation) may be used to query for an underlying error if one
exists. There are no API guarantees about which error is returned.

When the `std` feature is enabled, this implements `std::error::Error`.

#### Implementations

- <span id="builderror-pattern"></span>`fn pattern(&self) -> Option<PatternID>` — [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="builderror-size-limit"></span>`fn size_limit(&self) -> Option<usize>`

- <span id="builderror-syntax-error"></span>`fn syntax_error(&self) -> Option<&regex_syntax::Error>`

- <span id="builderror-ast"></span>`fn ast(pid: PatternID, err: ast::Error) -> BuildError` — [`PatternID`](../../util/primitives/index.md#patternid), [`BuildError`](#builderror)

- <span id="builderror-hir"></span>`fn hir(pid: PatternID, err: hir::Error) -> BuildError` — [`PatternID`](../../util/primitives/index.md#patternid), [`BuildError`](#builderror)

- <span id="builderror-nfa"></span>`fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md#builderror)

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

### `RetryQuadraticError`

```rust
struct RetryQuadraticError(());
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:164`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/error.rs#L164)*

An error that occurs when potential quadratic behavior has been detected
when applying either the "reverse suffix" or "reverse inner" optimizations.

When this error occurs, callers should abandon the "reverse" optimization
and use a normal forward search.

#### Implementations

- <span id="retryquadraticerror-new"></span>`fn new() -> RetryQuadraticError` — [`RetryQuadraticError`](#retryquadraticerror)

#### Trait Implementations

##### `impl Debug for RetryQuadraticError`

- <span id="retryquadraticerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RetryQuadraticError`

- <span id="retryquadraticerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryQuadraticError`

##### `impl ToString for RetryQuadraticError`

- <span id="retryquadraticerror-to-string"></span>`fn to_string(&self) -> String`

### `RetryFailError`

```rust
struct RetryFailError {
    offset: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:200-202`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/error.rs#L200-L202)*

An error that occurs when a regex engine "gives up" for some reason before
finishing a search. Usually this occurs because of heuristic Unicode word
boundary support or because of ineffective cache usage in the lazy DFA.

When this error occurs, callers should retry the regex search with a
different regex engine.

Note that this has convenient `From` impls that will automatically
convert a `MatchError` into this error. This works because the meta
regex engine internals guarantee that errors like `HaystackTooLong` and
`UnsupportedAnchored` will never occur. The only errors left are `Quit` and
`GaveUp`, which both correspond to this "failure" error.

#### Implementations

- <span id="retryfailerror-from-offset"></span>`fn from_offset(offset: usize) -> RetryFailError` — [`RetryFailError`](#retryfailerror)

#### Trait Implementations

##### `impl Debug for RetryFailError`

- <span id="retryfailerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RetryFailError`

- <span id="retryfailerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryFailError`

##### `impl ToString for RetryFailError`

- <span id="retryfailerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `BuildErrorKind`

```rust
enum BuildErrorKind {
    Syntax {
        pid: crate::PatternID,
        err: regex_syntax::Error,
    },
    NFA(nfa::thompson::BuildError),
}
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:32-35`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/error.rs#L32-L35)*

#### Trait Implementations

##### `impl Clone for BuildErrorKind`

- <span id="builderrorkind-clone"></span>`fn clone(&self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl Debug for BuildErrorKind`

- <span id="builderrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RetryError`

```rust
enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:135-138`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/error.rs#L135-L138)*

An error that occurs when a search should be retried.

This retry error distinguishes between two different failure modes.

The first is one where potential quadratic behavior has been detected.
In this case, whatever optimization that led to this behavior should be
stopped, and the next best strategy should be used.

The second indicates that the underlying regex engine has failed for some
reason. This usually occurs because either a lazy DFA's cache has become
ineffective or because a non-ASCII byte has been seen *and* a Unicode word
boundary was used in one of the patterns. In this failure case, a different
regex engine that won't fail in these ways (PikeVM, backtracker or the
one-pass DFA) should be used.

This is an internal error only and should never bleed into the public
API.

#### Trait Implementations

##### `impl Debug for RetryError`

- <span id="retryerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RetryError`

- <span id="retryerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryError`

##### `impl ToString for RetryError`

- <span id="retryerror-to-string"></span>`fn to_string(&self) -> String`

