*[miette](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MietteError`](#mietteerror) | enum | Error enum for miette. |

## Enums

### `MietteError`

```rust
enum MietteError {
    IoError(io::Error),
    OutOfBounds,
}
```

Error enum for miette. Used by certain operations in the protocol.

#### Variants

- **`IoError`**

  Wrapper around [`std::io::Error`](../../cargo_docs_md/error/index.md). This is returned when something went
  wrong while reading a [`SourceCode`](crate::SourceCode).

- **`OutOfBounds`**

  Returned when a [`SourceSpan`](crate::SourceSpan) extends beyond the
  bounds of a given [`SourceCode`](crate::SourceCode).

#### Trait Implementations

##### `impl Debug for MietteError`

- <span id="mietteerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for MietteError`

- <span id="mietteerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for MietteError`

- <span id="mietteerror-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- <span id="mietteerror-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- <span id="mietteerror-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

##### `impl Display for MietteError`

- <span id="mietteerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for MietteError`

- <span id="mietteerror-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl<D> OwoColorize for MietteError`

##### `impl<T> ToString for MietteError`

- <span id="mietteerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for MietteError`

