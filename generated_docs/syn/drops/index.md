*[syn](../index.md) / [drops](index.md)*

---

# Module `drops`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoDrop`](#nodrop) | struct |  |
| [`TrivialDrop`](#trivialdrop) | trait |  |

## Structs

### `NoDrop<T: ?Sized>`

```rust
struct NoDrop<T: ?Sized>(std::mem::ManuallyDrop<T>);
```

*Defined in [`syn-2.0.111/src/drops.rs:8`](../../../.source_1765633015/syn-2.0.111/src/drops.rs#L8)*

#### Implementations

- <span id="nodrop-new"></span>`fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T> Any for NoDrop<T>`

- <span id="nodrop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoDrop<T>`

- <span id="nodrop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoDrop<T>`

- <span id="nodrop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized> Deref for NoDrop<T>`

- <span id="nodrop-deref-type-target"></span>`type Target = T`

- <span id="nodrop-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: ?Sized> DerefMut for NoDrop<T>`

- <span id="nodrop-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> From for NoDrop<T>`

- <span id="nodrop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for NoDrop<T>`

- <span id="nodrop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Receiver for NoDrop<T>`

- <span id="nodrop-receiver-type-target"></span>`type Target = T`

##### `impl<T, U> TryFrom for NoDrop<T>`

- <span id="nodrop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nodrop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for NoDrop<T>`

- <span id="nodrop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nodrop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `TrivialDrop`

```rust
trait TrivialDrop { ... }
```

*Defined in [`syn-2.0.111/src/drops.rs:32`](../../../.source_1765633015/syn-2.0.111/src/drops.rs#L32)*

#### Implementors

- [`PrivateIterMut`](../punctuated/index.md#privateitermut)
- [`PrivateIter`](../punctuated/index.md#privateiter)
- `iter::Empty<T>`
- `option::IntoIter<&T>`
- `option::IntoIter<&mut T>`
- `slice::Iter<'_, T>`
- `slice::IterMut<'_, T>`

