*[miette_derive](../index.md) / [diagnostic_arg](index.md)*

---

# Module `diagnostic_arg`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticArg`](#diagnosticarg) | enum |  |

## Enums

### `DiagnosticArg`

```rust
enum DiagnosticArg {
    Transparent,
    Code(crate::code::Code),
    Severity(crate::severity::Severity),
    Help(crate::help::Help),
    Url(crate::url::Url),
    Forward(crate::forward::Forward),
}
```

*Defined in [`miette-derive-7.6.0/src/diagnostic_arg.rs:9-16`](../../../.source_1765521767/miette-derive-7.6.0/src/diagnostic_arg.rs#L9-L16)*

#### Trait Implementations

##### `impl Any for DiagnosticArg`

- <span id="diagnosticarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DiagnosticArg`

- <span id="diagnosticarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DiagnosticArg`

- <span id="diagnosticarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DiagnosticArg`

- <span id="diagnosticarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DiagnosticArg`

- <span id="diagnosticarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for DiagnosticArg`

- <span id="diagnosticarg-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for DiagnosticArg`

- <span id="diagnosticarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnosticarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DiagnosticArg`

- <span id="diagnosticarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnosticarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

