*[miette_derive](../index.md) / [diagnostic_source](index.md)*

---

# Module `diagnostic_source`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticSource`](#diagnosticsource) | struct |  |

## Structs

### `DiagnosticSource`

```rust
struct DiagnosticSource(syn::Member);
```

*Defined in [`miette-derive-7.6.0/src/diagnostic_source.rs:11`](../../../.source_1765633015/miette-derive-7.6.0/src/diagnostic_source.rs#L11)*

#### Implementations

- <span id="diagnosticsource-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="diagnosticsource-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="diagnosticsource-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

- <span id="diagnosticsource-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Any for DiagnosticSource`

- <span id="diagnosticsource-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DiagnosticSource`

- <span id="diagnosticsource-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DiagnosticSource`

- <span id="diagnosticsource-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DiagnosticSource`

- <span id="diagnosticsource-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DiagnosticSource`

- <span id="diagnosticsource-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DiagnosticSource`

- <span id="diagnosticsource-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnosticsource-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DiagnosticSource`

- <span id="diagnosticsource-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnosticsource-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

