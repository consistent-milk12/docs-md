*[rayon](../../index.md) / [collections](../index.md) / [drain_guard](index.md)*

---

# Module `drain_guard`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DrainGuard`](#drainguard) | struct | A proxy for draining a collection by converting to a `Vec` and back. |

## Structs

### `DrainGuard<'a, T, C: From<Vec<T>>>`

```rust
struct DrainGuard<'a, T, C: From<Vec<T>>> {
    collection: &'a mut C,
    vec: Vec<T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/mod.rs:46-49`](../../../../.source_1765633015/rayon-1.11.0/src/collections/mod.rs#L46-L49)*

A proxy for draining a collection by converting to a `Vec` and back.

This is used for draining `BinaryHeap` and `VecDeque`, which both have
zero-allocation conversions to/from `Vec`, though not zero-cost:
- `BinaryHeap` will heapify from `Vec`, but at least that will be empty.
- `VecDeque` has to shift items to offset 0 when converting to `Vec`.

#### Implementations

- <span id="drainguard-new"></span>`fn new(collection: &'a mut C) -> Self`

#### Trait Implementations

##### `impl<T> Any for DrainGuard<'a, T, C>`

- <span id="drainguard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DrainGuard<'a, T, C>`

- <span id="drainguard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DrainGuard<'a, T, C>`

- <span id="drainguard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C: From<Vec<T>>> Drop for DrainGuard<'a, T, C>`

- <span id="drainguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for DrainGuard<'a, T, C>`

- <span id="drainguard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DrainGuard<'a, T, C>`

- <span id="drainguard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for DrainGuard<'a, T, C>`

##### `impl<T, C> ParallelDrainRange for &'a mut DrainGuard<'_, T, C>`

- <span id="a-mut-drainguard-paralleldrainrange-type-iter"></span>`type Iter = Drain<'a, T>`

- <span id="a-mut-drainguard-paralleldrainrange-type-item"></span>`type Item = T`

- <span id="a-mut-drainguard-paralleldrainrange-par-drain"></span>`fn par_drain<R: RangeBounds<usize>>(self, range: R) -> <Self as >::Iter` — [`ParallelDrainRange`](../../iter/index.md#paralleldrainrange)

##### `impl<T> Pointable for DrainGuard<'a, T, C>`

- <span id="drainguard-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drainguard-pointable-type-init"></span>`type Init = T`

- <span id="drainguard-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drainguard-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drainguard-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drainguard-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for DrainGuard<'a, T, C>`

- <span id="drainguard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drainguard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DrainGuard<'a, T, C>`

- <span id="drainguard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drainguard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

