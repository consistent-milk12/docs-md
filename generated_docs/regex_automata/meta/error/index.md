*[regex_automata](../../index.md) / [meta](../index.md) / [error](index.md)*

---

# Module `error`

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

An error that occurs when construction of a `Regex` fails.

A build error is generally a result of one of two possible failure
modes. First is a parse or syntax error in the concrete syntax of a
pattern. Second is that the construction of the underlying regex matcher
fails, usually because it gets too big with respect to limits like
[`Config::nfa_size_limit`](crate::meta::Config::nfa_size_limit).

This error provides very little introspection capabilities. You can:

* Ask for the [`PatternID`](../../index.md) of the pattern that caused an error, if one
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

- `fn pattern(self: &Self) -> Option<PatternID>` — [`PatternID`](../../index.md)

- `fn size_limit(self: &Self) -> Option<usize>`

- `fn syntax_error(self: &Self) -> Option<&regex_syntax::Error>`

- `fn ast(pid: PatternID, err: ast::Error) -> BuildError` — [`PatternID`](../../index.md), [`BuildError`](../index.md)

- `fn hir(pid: PatternID, err: hir::Error) -> BuildError` — [`PatternID`](../../index.md), [`BuildError`](../index.md)

- `fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/index.md)

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

### `RetryQuadraticError`

```rust
struct RetryQuadraticError(());
```

An error that occurs when potential quadratic behavior has been detected
when applying either the "reverse suffix" or "reverse inner" optimizations.

When this error occurs, callers should abandon the "reverse" optimization
and use a normal forward search.

#### Implementations

- `fn new() -> RetryQuadraticError` — [`RetryQuadraticError`](#retryquadraticerror)

#### Trait Implementations

##### `impl Debug for RetryQuadraticError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for RetryQuadraticError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryQuadraticError`

##### `impl<T> ToString for RetryQuadraticError`

- `fn to_string(self: &Self) -> String`

### `RetryFailError`

```rust
struct RetryFailError {
    offset: usize,
}
```

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

- `fn from_offset(offset: usize) -> RetryFailError` — [`RetryFailError`](#retryfailerror)

#### Trait Implementations

##### `impl Debug for RetryFailError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for RetryFailError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryFailError`

##### `impl<T> ToString for RetryFailError`

- `fn to_string(self: &Self) -> String`

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

#### Trait Implementations

##### `impl Clone for BuildErrorKind`

- `fn clone(self: &Self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl Debug for BuildErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RetryError`

```rust
enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
```

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for RetryError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryError`

##### `impl<T> ToString for RetryError`

- `fn to_string(self: &Self) -> String`

