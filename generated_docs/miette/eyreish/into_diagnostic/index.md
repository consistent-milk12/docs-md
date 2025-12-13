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

*Defined in [`miette-7.6.0/src/eyreish/into_diagnostic.rs:8`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/into_diagnostic.rs#L8)*

Convenience [`Diagnostic`](../../index.md) that can be used as an "anonymous" wrapper for
Errors. This is intended to be paired with [`IntoDiagnostic`](../index.md).

#### Trait Implementations

##### `impl Any for DiagnosticError`

- <span id="diagnosticerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DiagnosticError`

- <span id="diagnosticerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DiagnosticError`

- <span id="diagnosticerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DiagnosticError`

- <span id="diagnosticerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for DiagnosticError`

- <span id="diagnosticerror-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl Diagnostic for DiagnosticError`

##### `impl Display for DiagnosticError`

- <span id="diagnosticerror-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for DiagnosticError`

- <span id="diagnosticerror-error-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl<T> From for DiagnosticError`

- <span id="diagnosticerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DiagnosticError`

- <span id="diagnosticerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DiagnosticError`

##### `impl ToString for DiagnosticError`

- <span id="diagnosticerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for DiagnosticError`

##### `impl<U> TryFrom for DiagnosticError`

- <span id="diagnosticerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnosticerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DiagnosticError`

- <span id="diagnosticerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnosticerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `IntoDiagnostic<T, E>`

```rust
trait IntoDiagnostic<T, E> { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/into_diagnostic.rs:35-39`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/into_diagnostic.rs#L35-L39)*

Convenience trait that adds a [`.into_diagnostic()`](IntoDiagnostic::into_diagnostic) method that converts a type implementing
[`std::error::Error`](../../../addr2line/index.md) to a [`Result<T, Report>`](../../../cargo_metadata/errors/index.md).

## Warning

Calling this on a type implementing [`Diagnostic`](../../index.md) will reduce it to the common denominator of
[`std::error::Error`](../../../addr2line/index.md). Meaning all extra information provided by [`Diagnostic`](../../index.md) will be
inaccessible. If you have a type implementing [`Diagnostic`](../../index.md) consider simply returning it or using
`Into` or the [`Try`](std::ops::Try) operator (`?`).

#### Required Methods

- `fn into_diagnostic(self) -> Result<T, Report>`

  Converts [`Result`](../../../cargo_metadata/errors/index.md) types that return regular [`std::error::Error`](../../../addr2line/index.md)s

#### Implementors

- `Result<T, E>`

