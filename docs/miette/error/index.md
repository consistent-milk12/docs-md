*[miette](../index.md) / [error](index.md)*

---

# Module `error`

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

  Wrapper around [`std::io::Error`](../../addr2line/index.md). This is returned when something went
  wrong while reading a [`SourceCode`](crate::SourceCode).

- **`OutOfBounds`**

  Returned when a [`SourceSpan`](crate::SourceSpan) extends beyond the
  bounds of a given [`SourceCode`](crate::SourceCode).

#### Trait Implementations

##### `impl Debug for MietteError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E> Diag for MietteError`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for MietteError`

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

##### `impl Display for MietteError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for MietteError`

- `fn source(self: &Self) -> Option<&dyn Error>`

##### `impl<D> OwoColorize for MietteError`

##### `impl<T> ToString for MietteError`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for MietteError`

