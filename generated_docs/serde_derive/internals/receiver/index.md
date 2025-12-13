*[serde_derive](../../index.md) / [internals](../index.md) / [receiver](index.md)*

---

# Module `receiver`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReplaceReceiver`](#replacereceiver) | struct |  |
| [`replace_receiver`](#replace-receiver) | fn |  |

## Structs

### `ReplaceReceiver<'a>`

```rust
struct ReplaceReceiver<'a>(&'a syn::TypePath);
```

*Defined in [`serde_derive-1.0.228/src/internals/receiver.rs:21`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/receiver.rs#L21)*

#### Implementations

- <span id="replacereceiver-self-ty"></span>`fn self_ty(&self, span: Span) -> TypePath`

- <span id="replacereceiver-self-to-qself"></span>`fn self_to_qself(&self, qself: &mut Option<QSelf>, path: &mut Path)`

- <span id="replacereceiver-self-to-expr-path"></span>`fn self_to_expr_path(&self, path: &mut Path)`

#### Trait Implementations

##### `impl Any for ReplaceReceiver<'a>`

- <span id="replacereceiver-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReplaceReceiver<'a>`

- <span id="replacereceiver-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReplaceReceiver<'a>`

- <span id="replacereceiver-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ReplaceReceiver<'a>`

- <span id="replacereceiver-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReplaceReceiver<'a>`

- <span id="replacereceiver-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReplaceReceiver<'a>`

- <span id="replacereceiver-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="replacereceiver-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReplaceReceiver<'a>`

- <span id="replacereceiver-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="replacereceiver-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `replace_receiver`

```rust
fn replace_receiver(input: &mut syn::DeriveInput)
```

*Defined in [`serde_derive-1.0.228/src/internals/receiver.rs:10-19`](../../../../.source_1765633015/serde_derive-1.0.228/src/internals/receiver.rs#L10-L19)*

