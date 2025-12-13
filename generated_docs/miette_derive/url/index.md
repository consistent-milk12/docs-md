*[miette_derive](../index.md) / [url](index.md)*

---

# Module `url`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Url`](#url) | enum |  |

## Enums

### `Url`

```rust
enum Url {
    Display(crate::fmt::Display),
    DocsRs,
}
```

*Defined in [`miette-derive-7.6.0/src/url.rs:18-21`](../../../.source_1765633015/miette-derive-7.6.0/src/url.rs#L18-L21)*

#### Implementations

- <span id="url-gen-enum"></span>`fn gen_enum(enum_name: &syn::Ident, variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

- <span id="url-gen-struct"></span>`fn gen_struct(&self, struct_name: &syn::Ident, fields: &Fields) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Any for Url`

- <span id="url-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Url`

- <span id="url-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Url`

- <span id="url-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Url`

- <span id="url-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Url`

- <span id="url-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Url`

- <span id="url-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for Url`

- <span id="url-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="url-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Url`

- <span id="url-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="url-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

