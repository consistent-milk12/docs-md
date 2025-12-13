*[miette_derive](../index.md) / [source_code](index.md)*

---

# Module `source_code`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourceCode`](#sourcecode) | struct |  |

## Structs

### `SourceCode`

```rust
struct SourceCode {
    source_code: syn::Member,
    is_option: bool,
}
```

*Defined in [`miette-derive-7.6.0/src/source_code.rs:11-14`](../../../.source_1765633015/miette-derive-7.6.0/src/source_code.rs#L11-L14)*

#### Implementations

- <span id="sourcecode-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="sourcecode-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="sourcecode-gen-struct"></span>`fn gen_struct(&self, fields: &syn::Fields) -> Option<TokenStream>`

- <span id="sourcecode-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

#### Trait Implementations

##### `impl Any for SourceCode`

- <span id="sourcecode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceCode`

- <span id="sourcecode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceCode`

- <span id="sourcecode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SourceCode`

- <span id="sourcecode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SourceCode`

- <span id="sourcecode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SourceCode`

- <span id="sourcecode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcecode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceCode`

- <span id="sourcecode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcecode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

