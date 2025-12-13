*[miette_derive](../index.md) / [related](index.md)*

---

# Module `related`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Related`](#related) | struct |  |

## Structs

### `Related`

```rust
struct Related(syn::Member);
```

*Defined in [`miette-derive-7.6.0/src/related.rs:11`](../../../.source_1765521767/miette-derive-7.6.0/src/related.rs#L11)*

#### Implementations

- <span id="related-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="related-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="related-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

- <span id="related-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Any for Related`

- <span id="related-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Related`

- <span id="related-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Related`

- <span id="related-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Related`

- <span id="related-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Related`

- <span id="related-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Related`

- <span id="related-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="related-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Related`

- <span id="related-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="related-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

