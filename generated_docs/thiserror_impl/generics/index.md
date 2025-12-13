*[thiserror_impl](../index.md) / [generics](index.md)*

---

# Module `generics`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParamsInScope`](#paramsinscope) | struct |  |
| [`InferredBounds`](#inferredbounds) | struct |  |
| [`crawl`](#crawl) | fn |  |

## Structs

### `ParamsInScope<'a>`

```rust
struct ParamsInScope<'a> {
    names: std::collections::BTreeSet<&'a syn::Ident>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/generics.rs:8-10`](../../../.source_1765521767/thiserror-impl-2.0.17/src/generics.rs#L8-L10)*

#### Implementations

- <span id="paramsinscope-new"></span>`fn new(generics: &'a Generics) -> Self`

- <span id="paramsinscope-intersects"></span>`fn intersects(&self, ty: &Type) -> bool`

#### Trait Implementations

##### `impl Any for ParamsInScope<'a>`

- <span id="paramsinscope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParamsInScope<'a>`

- <span id="paramsinscope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParamsInScope<'a>`

- <span id="paramsinscope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ParamsInScope<'a>`

- <span id="paramsinscope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParamsInScope<'a>`

- <span id="paramsinscope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ParamsInScope<'a>`

- <span id="paramsinscope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="paramsinscope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParamsInScope<'a>`

- <span id="paramsinscope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="paramsinscope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InferredBounds`

```rust
struct InferredBounds {
    bounds: std::collections::BTreeMap<String, (std::collections::BTreeSet<String>, syn::punctuated::Punctuated<proc_macro2::TokenStream, token::Plus>)>,
    order: Vec<proc_macro2::TokenStream>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/generics.rs:48-51`](../../../.source_1765521767/thiserror-impl-2.0.17/src/generics.rs#L48-L51)*

#### Implementations

- <span id="inferredbounds-new"></span>`fn new() -> Self`

- <span id="inferredbounds-insert"></span>`fn insert(&mut self, ty: impl ToTokens, bound: impl ToTokens)`

- <span id="inferredbounds-augment-where-clause"></span>`fn augment_where_clause(&self, generics: &Generics) -> WhereClause`

#### Trait Implementations

##### `impl Any for InferredBounds`

- <span id="inferredbounds-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InferredBounds`

- <span id="inferredbounds-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InferredBounds`

- <span id="inferredbounds-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for InferredBounds`

- <span id="inferredbounds-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InferredBounds`

- <span id="inferredbounds-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for InferredBounds`

- <span id="inferredbounds-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inferredbounds-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InferredBounds`

- <span id="inferredbounds-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inferredbounds-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `crawl`

```rust
fn crawl(in_scope: &ParamsInScope<'_>, ty: &syn::Type, found: &mut bool)
```

*Defined in [`thiserror-impl-2.0.17/src/generics.rs:26-46`](../../../.source_1765521767/thiserror-impl-2.0.17/src/generics.rs#L26-L46)*

