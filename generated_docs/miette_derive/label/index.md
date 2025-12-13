*[miette_derive](../index.md) / [label](index.md)*

---

# Module `label`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Labels`](#labels) | struct |  |
| [`Label`](#label) | struct |  |
| [`LabelAttr`](#labelattr) | struct |  |
| [`LabelType`](#labeltype) | enum |  |

## Structs

### `Labels`

```rust
struct Labels(Vec<Label>);
```

*Defined in [`miette-derive-7.6.0/src/label.rs:17`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L17)*

#### Implementations

- <span id="labels-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="labels-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="labels-gen-struct"></span>`fn gen_struct(&self, fields: &syn::Fields) -> Option<TokenStream>`

- <span id="labels-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

#### Trait Implementations

##### `impl Any for Labels`

- <span id="labels-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Labels`

- <span id="labels-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Labels`

- <span id="labels-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Labels`

- <span id="labels-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Labels`

- <span id="labels-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Labels`

- <span id="labels-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="labels-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Labels`

- <span id="labels-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="labels-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Label`

```rust
struct Label {
    label: Option<crate::fmt::Display>,
    ty: syn::Type,
    span: syn::Member,
    lbl_ty: LabelType,
}
```

*Defined in [`miette-derive-7.6.0/src/label.rs:26-31`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L26-L31)*

#### Trait Implementations

##### `impl Any for Label`

- <span id="label-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Label`

- <span id="label-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Label`

- <span id="label-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Label`

- <span id="label-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Label`

- <span id="label-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Label`

- <span id="label-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="label-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Label`

- <span id="label-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="label-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LabelAttr`

```rust
struct LabelAttr {
    label: Option<crate::fmt::Display>,
    lbl_ty: LabelType,
}
```

*Defined in [`miette-derive-7.6.0/src/label.rs:33-36`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L33-L36)*

#### Trait Implementations

##### `impl Any for LabelAttr`

- <span id="labelattr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LabelAttr`

- <span id="labelattr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LabelAttr`

- <span id="labelattr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LabelAttr`

- <span id="labelattr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LabelAttr`

- <span id="labelattr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for LabelAttr`

- <span id="labelattr-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for LabelAttr`

- <span id="labelattr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="labelattr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LabelAttr`

- <span id="labelattr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="labelattr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `LabelType`

```rust
enum LabelType {
    Default,
    Primary,
    Collection,
}
```

*Defined in [`miette-derive-7.6.0/src/label.rs:20-24`](../../../.source_1765521767/miette-derive-7.6.0/src/label.rs#L20-L24)*

#### Trait Implementations

##### `impl Any for LabelType`

- <span id="labeltype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LabelType`

- <span id="labeltype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LabelType`

- <span id="labeltype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Eq for LabelType`

##### `impl<T> From for LabelType`

- <span id="labeltype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LabelType`

- <span id="labeltype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LabelType`

- <span id="labeltype-partialeq-eq"></span>`fn eq(&self, other: &LabelType) -> bool` — [`LabelType`](#labeltype)

##### `impl StructuralPartialEq for LabelType`

##### `impl<U> TryFrom for LabelType`

- <span id="labeltype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="labeltype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LabelType`

- <span id="labeltype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="labeltype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

