*[miette](../../index.md) / [eyreish](../index.md) / [into_diagnostic](index.md)*

---

# Module `into_diagnostic`

## Structs

### `DiagnosticError`

```rust
struct DiagnosticError(Box<dyn std::error::Error + Send + Sync>);
```

Convenience [`Diagnostic`](../../index.md) that can be used as an "anonymous" wrapper for
Errors. This is intended to be paired with [`IntoDiagnostic`](../index.md).

#### Trait Implementations

##### `impl Debug for DiagnosticError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E> Diag for DiagnosticError`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl Diagnostic for DiagnosticError`

##### `impl Display for DiagnosticError`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for DiagnosticError`

- `fn source(self: &Self) -> Option<&dyn Error>`

##### `impl<D> OwoColorize for DiagnosticError`

##### `impl<T> ToString for DiagnosticError`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for DiagnosticError`

## Traits

### `IntoDiagnostic<T, E>`

```rust
trait IntoDiagnostic<T, E> { ... }
```

Convenience trait that adds a [`.into_diagnostic()`](IntoDiagnostic::into_diagnostic) method that converts a type implementing
[`std::error::Error`](../../../addr2line/index.md) to a [`Result<T, Report>`](../../../clap_builder/error/index.md).

## Warning

Calling this on a type implementing [`Diagnostic`](../../index.md) will reduce it to the common denominator of
[`std::error::Error`](../../../addr2line/index.md). Meaning all extra information provided by [`Diagnostic`](../../index.md) will be
inaccessible. If you have a type implementing [`Diagnostic`](../../index.md) consider simply returning it or using
`Into` or the [`Try`](std::ops::Try) operator (`?`).

#### Required Methods

- `fn into_diagnostic(self: Self) -> Result<T, Report>`

  Converts [`Result`](../../../clap_builder/error/index.md) types that return regular [`std::error::Error`](../../../addr2line/index.md)s

