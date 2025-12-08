*[miette](../index.md) / [diagnostic_chain](index.md)*

---

# Module `diagnostic_chain`

Iterate over error `.diagnostic_source()` chains.

## Structs

### `DiagnosticChain<'a>`

```rust
struct DiagnosticChain<'a> {
    state: Option<ErrorKind<'a>>,
}
```

Iterator of a chain of cause errors.

#### Implementations

- `fn from_diagnostic(head: &'a dyn Diagnostic) -> Self` — [`Diagnostic`](../index.md)

- `fn from_stderror(head: &'a dyn std::error::Error) -> Self`

#### Trait Implementations

##### `impl<'a> Clone for DiagnosticChain<'a>`

- `fn clone(self: &Self) -> DiagnosticChain<'a>` — [`DiagnosticChain`](#diagnosticchain)

##### `impl<'a> Default for DiagnosticChain<'a>`

- `fn default() -> DiagnosticChain<'a>` — [`DiagnosticChain`](#diagnosticchain)

##### `impl ExactSizeIterator for DiagnosticChain<'_>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for DiagnosticChain<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for DiagnosticChain<'a>`

- `type Item = ErrorKind<'a>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

- `fn get_nested(self: &Self) -> Option<ErrorKind<'a>>` — [`ErrorKind`](#errorkind)

#### Trait Implementations

##### `impl<'a> Clone for ErrorKind<'a>`

- `fn clone(self: &Self) -> ErrorKind<'a>` — [`ErrorKind`](#errorkind)

##### `impl Debug for ErrorKind<'_>`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Display for ErrorKind<'_>`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<D> OwoColorize for ErrorKind<'a>`

##### `impl<T> ToString for ErrorKind<'a>`

- `fn to_string(self: &Self) -> String`

