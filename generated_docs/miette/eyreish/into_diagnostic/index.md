*[miette](../../index.md) / [eyreish](../index.md) / [into_diagnostic](index.md)*

---

# Module `into_diagnostic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticError`](#diagnosticerror) | struct | Convenience [`Diagnostic`] that can be used as an "anonymous" wrapper for Errors. |
| [`IntoDiagnostic`](#intodiagnostic) | trait | Convenience trait that adds a [`.into_diagnostic()`](IntoDiagnostic::into_diagnostic) method that converts a type implementing [`std::error::Error`] to a [`Result<T, Report>`]. |

## Structs

### `DiagnosticError`

```rust
struct DiagnosticError(Box<dyn std::error::Error + Send + Sync>);
```

*Defined in [`miette-7.6.0/src/eyreish/into_diagnostic.rs:8`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/into_diagnostic.rs#L8)*

Convenience [`Diagnostic`](../../index.md) that can be used as an "anonymous" wrapper for
Errors. This is intended to be paired with [`IntoDiagnostic`](../index.md).

#### Trait Implementations

##### `impl Debug for DiagnosticError`

- <span id="diagnosticerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for DiagnosticError`

- <span id="diagnosticerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl Diagnostic for DiagnosticError`

##### `impl Display for DiagnosticError`

- <span id="diagnosticerror-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for DiagnosticError`

- <span id="diagnosticerror-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl OwoColorize for DiagnosticError`

##### `impl ToString for DiagnosticError`

- <span id="diagnosticerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for DiagnosticError`

## Traits

### `IntoDiagnostic<T, E>`

```rust
trait IntoDiagnostic<T, E> { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/into_diagnostic.rs:35-39`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/into_diagnostic.rs#L35-L39)*

Convenience trait that adds a [`.into_diagnostic()`](IntoDiagnostic::into_diagnostic) method that converts a type implementing
[`std::error::Error`](../../../addr2line/index.md) to a [`Result<T, Report>`](../../../clap_builder/error/index.md).

## Warning

Calling this on a type implementing [`Diagnostic`](../../index.md) will reduce it to the common denominator of
[`std::error::Error`](../../../addr2line/index.md). Meaning all extra information provided by [`Diagnostic`](../../index.md) will be
inaccessible. If you have a type implementing [`Diagnostic`](../../index.md) consider simply returning it or using
`Into` or the [`Try`](std::ops::Try) operator (`?`).

#### Required Methods

- `fn into_diagnostic(self) -> Result<T, Report>`

  Converts [`Result`](../../../clap_builder/error/index.md) types that return regular [`std::error::Error`](../../../addr2line/index.md)s

#### Implementors

- `Result<T, E>`

