*[miette_derive](../index.md) / [severity](index.md)*

---

# Module `severity`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Severity`](#severity) | struct |  |
| [`get_severity`](#get-severity) | fn |  |

## Structs

### `Severity`

```rust
struct Severity(syn::Ident);
```

*Defined in [`miette-derive-7.6.0/src/severity.rs:15`](../../../.source_1765633015/miette-derive-7.6.0/src/severity.rs#L15)*

#### Implementations

- <span id="severity-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

- <span id="severity-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Any for Severity`

- <span id="severity-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Severity`

- <span id="severity-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Severity`

- <span id="severity-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Severity`

- <span id="severity-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Severity`

- <span id="severity-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Severity`

- <span id="severity-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for Severity`

- <span id="severity-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="severity-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Severity`

- <span id="severity-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="severity-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `get_severity`

```rust
fn get_severity(input: &str, span: proc_macro2::Span) -> syn::Result<String>
```

*Defined in [`miette-derive-7.6.0/src/severity.rs:50-60`](../../../.source_1765633015/miette-derive-7.6.0/src/severity.rs#L50-L60)*

