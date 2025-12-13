*[thiserror](../index.md) / [var](index.md)*

---

# Module `var`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Var`](#var) | struct |  |

## Structs

### `Var<'a, T: ?Sized>`

```rust
struct Var<'a, T: ?Sized>(&'a T);
```

*Defined in [`thiserror-2.0.17/src/var.rs:3`](../../../.source_1765521767/thiserror-2.0.17/src/var.rs#L3)*

#### Trait Implementations

##### `impl<T> Any for Var<'a, T>`

- <span id="var-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Var<'a, T>`

- <span id="var-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Var<'a, T>`

- <span id="var-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Var<'a, T>`

- <span id="var-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Var<'a, T>`

- <span id="var-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Pointer + ?Sized> Pointer for Var<'a, T>`

- <span id="var-pointer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for Var<'a, T>`

- <span id="var-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="var-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Var<'a, T>`

- <span id="var-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="var-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

