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

*Defined in [`gimli-0.32.3/src/read/util.rs:121-124`](../../../../.source_1765210505/gimli-0.32.3/src/read/util.rs#L121-L124)*

#### Implementations

- <span id="arrayvec-new"></span>`fn new() -> Self`

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, value: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](../index.md#arraylike), [`CapacityFull`](sealed/index.md#capacityfull)

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, element: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](../index.md#arraylike), [`CapacityFull`](sealed/index.md#capacityfull)

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`ArrayLike`](../index.md#arraylike)

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`ArrayLike`](../index.md#arraylike)

#### Trait Implementations

##### `impl<A: ArrayLike> Clone for ArrayVec<A>`

- <span id="arrayvec-clone"></span>`fn clone(&self) -> Self`

##### `impl<A: ArrayLike> Debug for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: ArrayLike> Default for ArrayVec<A>`

- <span id="arrayvec-default"></span>`fn default() -> Self`

##### `impl<A: ArrayLike> Deref for ArrayVec<A>`

- <span id="arrayvec-type-target"></span>`type Target = [<A as ArrayLike>::Item]`

- <span id="arrayvec-deref"></span>`fn deref(&self) -> &[<A as >::Item]` — [`ArrayLike`](../index.md#arraylike)

##### `impl<A: ArrayLike> DerefMut for ArrayVec<A>`

- <span id="arrayvec-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [<A as >::Item]` — [`ArrayLike`](../index.md#arraylike)

##### `impl<A: ArrayLike> Drop for ArrayVec<A>`

- <span id="arrayvec-drop"></span>`fn drop(&mut self)`

##### `impl<A: ArrayLike> Eq for ArrayVec<A>`

##### `impl<A: ArrayLike> PartialEq for ArrayVec<A>`

- <span id="arrayvec-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<P, T> Receiver for ArrayVec<A>`

- <span id="arrayvec-type-target"></span>`type Target = T`

## Traits

### `ArrayLike`

```rust
trait ArrayLike: Sealed { ... }
```

*Defined in [`gimli-0.32.3/src/read/util.rs:33-42`](../../../../.source_1765210505/gimli-0.32.3/src/read/util.rs#L33-L42)*

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Item`

#### Implementors

- `[T; N]`
- `alloc::boxed::Box<[T; N]>`
- `alloc::vec::Vec<T>`

