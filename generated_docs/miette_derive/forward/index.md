*[miette_derive](../index.md) / [forward](index.md)*

---

# Module `forward`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Forward`](#forward) | enum |  |
| [`WhichFn`](#whichfn) | enum |  |

## Enums

### `Forward`

```rust
enum Forward {
    Unnamed(usize),
    Named(syn::Ident),
}
```

*Defined in [`miette-derive-7.6.0/src/forward.rs:9-12`](../../../.source_1765633015/miette-derive-7.6.0/src/forward.rs#L9-L12)*

#### Implementations

- <span id="forward-for-transparent-field"></span>`fn for_transparent_field(fields: &syn::Fields) -> syn::Result<Self>`

- <span id="forward-gen-struct-method"></span>`fn gen_struct_method(&self, which_fn: WhichFn) -> TokenStream` — [`WhichFn`](#whichfn)

- <span id="forward-gen-enum-match-arm"></span>`fn gen_enum_match_arm(&self, variant: &syn::Ident, which_fn: WhichFn) -> TokenStream` — [`WhichFn`](#whichfn)

#### Trait Implementations

##### `impl Any for Forward`

- <span id="forward-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Forward`

- <span id="forward-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Forward`

- <span id="forward-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Forward`

- <span id="forward-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Forward`

- <span id="forward-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Forward`

- <span id="forward-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for Forward`

- <span id="forward-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="forward-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Forward`

- <span id="forward-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="forward-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WhichFn`

```rust
enum WhichFn {
    Code,
    Help,
    Url,
    Severity,
    Labels,
    SourceCode,
    Related,
    DiagnosticSource,
}
```

*Defined in [`miette-derive-7.6.0/src/forward.rs:33-42`](../../../.source_1765633015/miette-derive-7.6.0/src/forward.rs#L33-L42)*

#### Implementations

- <span id="whichfn-method-call"></span>`fn method_call(&self) -> TokenStream`

- <span id="whichfn-signature"></span>`fn signature(&self) -> TokenStream`

- <span id="whichfn-catchall-arm"></span>`fn catchall_arm(&self) -> TokenStream`

#### Trait Implementations

##### `impl Any for WhichFn`

- <span id="whichfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhichFn`

- <span id="whichfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhichFn`

- <span id="whichfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WhichFn`

- <span id="whichfn-clone"></span>`fn clone(&self) -> WhichFn` — [`WhichFn`](#whichfn)

##### `impl CloneToUninit for WhichFn`

- <span id="whichfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for WhichFn`

##### `impl<T> From for WhichFn`

- <span id="whichfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhichFn`

- <span id="whichfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for WhichFn`

- <span id="whichfn-toowned-type-owned"></span>`type Owned = T`

- <span id="whichfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whichfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WhichFn`

- <span id="whichfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whichfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhichFn`

- <span id="whichfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whichfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

