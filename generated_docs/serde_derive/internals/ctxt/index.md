*[serde_derive](../../index.md) / [internals](../index.md) / [ctxt](index.md)*

---

# Module `ctxt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ctxt`](#ctxt) | struct | A type to collect errors together and format them. |

## Structs

### `Ctxt`

```rust
struct Ctxt {
    errors: std::cell::RefCell<Option<Vec<syn::Error>>>,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ctxt.rs:12-16`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/ctxt.rs#L12-L16)*

A type to collect errors together and format them.

Dropping this object will cause a panic. It must be consumed using `check`.

References can be shared since this type uses run-time exclusive mut checking.

#### Implementations

- <span id="ctxt-new"></span>`fn new() -> Self`

  Create a new context object.

  

  This object contains no errors, but will still trigger a panic if it is not `check`ed.

- <span id="ctxt-error-spanned-by"></span>`fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T)`

  Add an error to the context object with a tokenenizable object.

  

  The object is used for spanning in error messages.

- <span id="ctxt-syn-error"></span>`fn syn_error(&self, err: syn::Error)`

  Add one of Syn's parse errors.

- <span id="ctxt-check"></span>`fn check(self) -> syn::Result<()>`

  Consume this object, producing a formatted error string if there are errors.

#### Trait Implementations

##### `impl Any for Ctxt`

- <span id="ctxt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ctxt`

- <span id="ctxt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ctxt`

- <span id="ctxt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for Ctxt`

- <span id="ctxt-default"></span>`fn default() -> Ctxt` — [`Ctxt`](#ctxt)

##### `impl Drop for Ctxt`

- <span id="ctxt-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Ctxt`

- <span id="ctxt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ctxt`

- <span id="ctxt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Ctxt`

- <span id="ctxt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ctxt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ctxt`

- <span id="ctxt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ctxt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

