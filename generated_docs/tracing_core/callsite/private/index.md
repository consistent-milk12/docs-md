*[tracing_core](../../index.md) / [callsite](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Private`](#private) | struct | Don't call this function, it's private. |

## Structs

### `Private<T>`

```rust
struct Private<T>(T);
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:512`](../../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L512)*

Don't call this function, it's private.

#### Trait Implementations

##### `impl<T> Any for Private<T>`

- <span id="private-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Private<T>`

- <span id="private-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Private<T>`

- <span id="private-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Private<T>`

- <span id="private-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Private<T>`

- <span id="private-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Private<T>`

- <span id="private-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="private-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Private<T>`

- <span id="private-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="private-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

