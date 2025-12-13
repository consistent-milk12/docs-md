*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [lru](index.md)*

---

# Module `lru`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Lru`](#lru) | struct | least-recently-used cache with static size |

## Structs

### `Lru<T, const N: usize>`

```rust
struct Lru<T, const N: usize> {
    len: usize,
    arr: [core::mem::MaybeUninit<T>; N],
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/lru.rs:5-9`](../../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli/lru.rs#L5-L9)*

least-recently-used cache with static size

#### Implementations

- <span id="lru-clear"></span>`fn clear(&mut self)`

- <span id="lru-iter"></span>`fn iter(&self) -> impl Iterator<Item = &T>`

- <span id="lru-push-front"></span>`fn push_front(&mut self, value: T) -> Option<&mut T>`

- <span id="lru-move-to-front"></span>`fn move_to_front(&mut self, idx: usize) -> Option<&mut T>`

#### Trait Implementations

##### `impl<T> Any for Lru<T, N>`

- <span id="lru-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lru<T, N>`

- <span id="lru-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lru<T, N>`

- <span id="lru-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Default for Lru<T, N>`

- <span id="lru-default"></span>`fn default() -> Self`

##### `impl<T> From for Lru<T, N>`

- <span id="lru-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Lru<T, N>`

- <span id="lru-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Lru<T, N>`

- <span id="lru-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lru-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Lru<T, N>`

- <span id="lru-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lru-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

