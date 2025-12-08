*[rayon](../../index.md) / [collections](../index.md) / [drain_guard](index.md)*

---

# Module `drain_guard`

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

- `fn new(collection: &'a mut C) -> Self`

#### Trait Implementations

##### `impl<'a, T, C: From<Vec<T>>> Drop for DrainGuard<'a, T, C>`

- `fn drop(self: &mut Self)`

##### `impl<T> IntoEither for DrainGuard<'a, T, C>`

##### `impl<T> Pointable for DrainGuard<'a, T, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

