*[miette_derive](../index.md) / [help](index.md)*

---

# Module `help`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Help`](#help) | enum |  |

## Enums

### `Help`

```rust
enum Help {
    Display(crate::fmt::Display),
    Field(syn::Member, Box<syn::Type>),
}
```

*Defined in [`miette-derive-7.6.0/src/help.rs:19-22`](../../../.source_1765633015/miette-derive-7.6.0/src/help.rs#L19-L22)*

#### Implementations

- <span id="help-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="help-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="help-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

- <span id="help-gen-struct"></span>`fn gen_struct(&self, fields: &Fields) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Any for Help`

- <span id="help-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Help`

- <span id="help-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Help`

- <span id="help-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Help`

- <span id="help-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Help`

- <span id="help-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Help`

- <span id="help-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for Help`

- <span id="help-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="help-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Help`

- <span id="help-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="help-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

