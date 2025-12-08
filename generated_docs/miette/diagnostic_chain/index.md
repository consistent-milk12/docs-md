*[miette](../index.md) / [diagnostic_chain](index.md)*

---

# Module `diagnostic_chain`

Iterate over error `.diagnostic_source()` chains.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticChain`](#diagnosticchain) | struct | Iterator of a chain of cause errors. |
| [`ErrorKind`](#errorkind) | enum |  |

## Structs

### `DiagnosticChain<'a>`

```rust
struct DiagnosticChain<'a> {
    state: Option<ErrorKind<'a>>,
}
```

Iterator of a chain of cause errors.

#### Implementations

- <span id="diagnosticchain-from-diagnostic"></span>`fn from_diagnostic(head: &'a dyn Diagnostic) -> Self` — [`Diagnostic`](../index.md)

- <span id="diagnosticchain-from-stderror"></span>`fn from_stderror(head: &'a dyn std::error::Error) -> Self`

#### Trait Implementations

##### `impl<'a> Clone for DiagnosticChain<'a>`

- <span id="diagnosticchain-clone"></span>`fn clone(&self) -> DiagnosticChain<'a>` — [`DiagnosticChain`](#diagnosticchain)

##### `impl<'a> Default for DiagnosticChain<'a>`

- <span id="diagnosticchain-default"></span>`fn default() -> DiagnosticChain<'a>` — [`DiagnosticChain`](#diagnosticchain)

##### `impl ExactSizeIterator for DiagnosticChain<'_>`

- <span id="diagnosticchain-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for DiagnosticChain<'a>`

- <span id="diagnosticchain-item"></span>`type Item = <I as Iterator>::Item`

- <span id="diagnosticchain-intoiter"></span>`type IntoIter = I`

- <span id="diagnosticchain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a> Iterator for DiagnosticChain<'a>`

- <span id="diagnosticchain-item"></span>`type Item = ErrorKind<'a>`

- <span id="diagnosticchain-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="diagnosticchain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<D> OwoColorize for DiagnosticChain<'a>`

## Enums

### `ErrorKind<'a>`

```rust
enum ErrorKind<'a> {
    Diagnostic(&'a dyn Diagnostic),
    StdError(&'a dyn std::error::Error),
}
```

#### Implementations

- <span id="errorkind-get-nested"></span>`fn get_nested(&self) -> Option<ErrorKind<'a>>` — [`ErrorKind`](#errorkind)

#### Trait Implementations

##### `impl<'a> Clone for ErrorKind<'a>`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind<'a>` — [`ErrorKind`](#errorkind)

##### `impl Debug for ErrorKind<'_>`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Display for ErrorKind<'_>`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<D> OwoColorize for ErrorKind<'a>`

##### `impl<T> ToString for ErrorKind<'a>`

- <span id="errorkind-to-string"></span>`fn to_string(&self) -> String`

