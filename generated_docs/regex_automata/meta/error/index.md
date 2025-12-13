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

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:27-29`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/error.rs#L27-L29)*

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

  If it is known which pattern ID caused this build error to occur, then

  this method returns it.

  

  Some errors are not associated with a particular pattern. However, any

  errors that occur as part of parsing a pattern are guaranteed to be

  associated with a pattern ID.

  

  # Example

  

  ```rust

  use regex_automata::{meta::Regex, PatternID};

  

  let err = Regex::new_many(&["a", "b", r"\p{Foo}", "c"]).unwrap_err();

  assert_eq!(Some(PatternID::must(2)), err.pattern());

  ```

- <span id="builderror-size-limit"></span>`fn size_limit(&self) -> Option<usize>`

  If this error occurred because the regex exceeded the configured size

  limit before being built, then this returns the configured size limit.

  

  The limit returned is what was configured, and corresponds to the

  maximum amount of heap usage in bytes.

- <span id="builderror-syntax-error"></span>`fn syntax_error(&self) -> Option<&regex_syntax::Error>`

  If this error corresponds to a syntax error, then a reference to it is

  returned by this method.

- <span id="builderror-ast"></span>`fn ast(pid: PatternID, err: ast::Error) -> BuildError` — [`PatternID`](../../util/primitives/index.md#patternid), [`BuildError`](#builderror)

- <span id="builderror-hir"></span>`fn hir(pid: PatternID, err: hir::Error) -> BuildError` — [`PatternID`](../../util/primitives/index.md#patternid), [`BuildError`](#builderror)

- <span id="builderror-nfa"></span>`fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md#builderror)

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

### `RetryQuadraticError`

```rust
struct RetryQuadraticError(());
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:164`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/error.rs#L164)*

An error that occurs when potential quadratic behavior has been detected
when applying either the "reverse suffix" or "reverse inner" optimizations.

When this error occurs, callers should abandon the "reverse" optimization
and use a normal forward search.

#### Implementations

- <span id="retryquadraticerror-new"></span>`fn new() -> RetryQuadraticError` — [`RetryQuadraticError`](#retryquadraticerror)

#### Trait Implementations

##### `impl Any for RetryQuadraticError`

- <span id="retryquadraticerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RetryQuadraticError`

- <span id="retryquadraticerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RetryQuadraticError`

- <span id="retryquadraticerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RetryQuadraticError`

- <span id="retryquadraticerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RetryQuadraticError`

- <span id="retryquadraticerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryQuadraticError`

##### `impl<T> From for RetryQuadraticError`

- <span id="retryquadraticerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RetryQuadraticError`

- <span id="retryquadraticerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for RetryQuadraticError`

- <span id="retryquadraticerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for RetryQuadraticError`

- <span id="retryquadraticerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="retryquadraticerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RetryQuadraticError`

- <span id="retryquadraticerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="retryquadraticerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RetryFailError`

```rust
struct RetryFailError {
    offset: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:200-202`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/error.rs#L200-L202)*

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

##### `impl Any for RetryFailError`

- <span id="retryfailerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RetryFailError`

- <span id="retryfailerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RetryFailError`

- <span id="retryfailerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RetryFailError`

- <span id="retryfailerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RetryFailError`

- <span id="retryfailerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryFailError`

##### `impl<T> From for RetryFailError`

- <span id="retryfailerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RetryFailError`

- <span id="retryfailerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for RetryFailError`

- <span id="retryfailerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for RetryFailError`

- <span id="retryfailerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="retryfailerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RetryFailError`

- <span id="retryfailerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="retryfailerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:32-35`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/error.rs#L32-L35)*

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

### `RetryError`

```rust
enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:135-138`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/error.rs#L135-L138)*

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

##### `impl Any for RetryError`

- <span id="retryerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RetryError`

- <span id="retryerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RetryError`

- <span id="retryerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RetryError`

- <span id="retryerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RetryError`

- <span id="retryerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for RetryError`

##### `impl<T> From for RetryError`

- <span id="retryerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RetryError`

- <span id="retryerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for RetryError`

- <span id="retryerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for RetryError`

- <span id="retryerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="retryerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RetryError`

- <span id="retryerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="retryerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

