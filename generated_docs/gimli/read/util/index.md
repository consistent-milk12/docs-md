*[gimli](../../index.md) / [read](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sealed`](#sealed) | mod |  |
| [`ArrayVec`](#arrayvec) | struct |  |
| [`ArrayLike`](#arraylike) | trait | Marker trait for types that can be used as backing storage when a growable array type is needed. |

## Modules

- [`sealed`](sealed/index.md)

## Structs

### `ArrayVec<A: ArrayLike>`

```rust
struct ArrayVec<A: ArrayLike> {
    storage: <A as >::Storage,
    len: usize,
}
```

*Defined in [`gimli-0.32.3/src/read/util.rs:121-124`](../../../../.source_1765521767/gimli-0.32.3/src/read/util.rs#L121-L124)*

#### Implementations

- <span id="arrayvec-new"></span>`fn new() -> Self`

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, value: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](../index.md#arraylike), [`CapacityFull`](sealed/index.md#capacityfull)

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, element: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](../index.md#arraylike), [`CapacityFull`](sealed/index.md#capacityfull)

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`ArrayLike`](../index.md#arraylike)

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`ArrayLike`](../index.md#arraylike)

#### Trait Implementations

##### `impl Any for ArrayVec<A>`

- <span id="arrayvec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayVec<A>`

- <span id="arrayvec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVec<A>`

- <span id="arrayvec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: ArrayLike> Clone for ArrayVec<A>`

- <span id="arrayvec-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ArrayVec<A>`

- <span id="arrayvec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: ArrayLike> Debug for ArrayVec<A>`

- <span id="arrayvec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: ArrayLike> Default for ArrayVec<A>`

- <span id="arrayvec-default"></span>`fn default() -> Self`

##### `impl<A: ArrayLike> Deref for ArrayVec<A>`

- <span id="arrayvec-deref-type-target"></span>`type Target = [<A as ArrayLike>::Item]`

- <span id="arrayvec-deref"></span>`fn deref(&self) -> &[<A as >::Item]` — [`ArrayLike`](../index.md#arraylike)

##### `impl<A: ArrayLike> DerefMut for ArrayVec<A>`

- <span id="arrayvec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [<A as >::Item]` — [`ArrayLike`](../index.md#arraylike)

##### `impl<A: ArrayLike> Drop for ArrayVec<A>`

- <span id="arrayvec-drop"></span>`fn drop(&mut self)`

##### `impl<A: ArrayLike> Eq for ArrayVec<A>`

##### `impl<T> From for ArrayVec<A>`

- <span id="arrayvec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArrayVec<A>`

- <span id="arrayvec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<A: ArrayLike> PartialEq for ArrayVec<A>`

- <span id="arrayvec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Receiver for ArrayVec<A>`

- <span id="arrayvec-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for ArrayVec<A>`

- <span id="arrayvec-toowned-type-owned"></span>`type Owned = T`

- <span id="arrayvec-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arrayvec-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArrayVec<A>`

- <span id="arrayvec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayvec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayVec<A>`

- <span id="arrayvec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayvec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ArrayLike`

```rust
trait ArrayLike: Sealed { ... }
```

*Defined in [`gimli-0.32.3/src/read/util.rs:33-42`](../../../../.source_1765521767/gimli-0.32.3/src/read/util.rs#L33-L42)*

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Item`

#### Implementors

- `[T; N]`
- `alloc::boxed::Box<[T; N]>`
- `alloc::vec::Vec<T>`

