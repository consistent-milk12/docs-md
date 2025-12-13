*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [set_len_on_drop](index.md)*

---

# Module `set_len_on_drop`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SetLenOnDrop`](#setlenondrop) | struct |  |

## Structs

### `SetLenOnDrop<'a>`

```rust
struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/set_len_on_drop.rs:6-9`](../../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/set_len_on_drop.rs#L6-L9)*

#### Implementations

- <span id="setlenondrop-new"></span>`fn new(len: &'a mut usize) -> Self`

- <span id="setlenondrop-increment-len"></span>`fn increment_len(&mut self, increment: usize)`

#### Trait Implementations

##### `impl Any for SetLenOnDrop<'a>`

- <span id="setlenondrop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SetLenOnDrop<'a>`

- <span id="setlenondrop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SetLenOnDrop<'a>`

- <span id="setlenondrop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for SetLenOnDrop<'_>`

- <span id="setlenondrop-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for SetLenOnDrop<'a>`

- <span id="setlenondrop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SetLenOnDrop<'a>`

- <span id="setlenondrop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SetLenOnDrop<'a>`

- <span id="setlenondrop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="setlenondrop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SetLenOnDrop<'a>`

- <span id="setlenondrop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="setlenondrop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

