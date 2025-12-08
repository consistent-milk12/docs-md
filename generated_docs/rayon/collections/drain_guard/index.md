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

A proxy for draining a collection by converting to a `Vec` and back.

This is used for draining `BinaryHeap` and `VecDeque`, which both have
zero-allocation conversions to/from `Vec`, though not zero-cost:
- `BinaryHeap` will heapify from `Vec`, but at least that will be empty.
- `VecDeque` has to shift items to offset 0 when converting to `Vec`.

#### Implementations

- <span id="drainguard-new"></span>`fn new(collection: &'a mut C) -> Self`

#### Trait Implementations

##### `impl<'a, T, C: From<Vec<T>>> Drop for DrainGuard<'a, T, C>`

- <span id="drainguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for DrainGuard<'a, T, C>`

##### `impl<T> Pointable for DrainGuard<'a, T, C>`

- <span id="drainguard-align"></span>`const ALIGN: usize`

- <span id="drainguard-init"></span>`type Init = T`

- <span id="drainguard-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drainguard-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drainguard-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drainguard-drop"></span>`unsafe fn drop(ptr: usize)`

