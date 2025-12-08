*[gimli](../../index.md) / [read](../index.md) / [util](index.md)*

---

# Module `util`

## Modules

- [`sealed`](sealed/index.md) - 

## Structs

### `ArrayVec<A: ArrayLike>`

```rust
struct ArrayVec<A: ArrayLike> {
    storage: <A as >::Storage,
    len: usize,
}
```

#### Implementations

- `fn new() -> Self`

- `fn clear(self: &mut Self)`

- `fn try_push(self: &mut Self, value: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](../index.md), [`CapacityFull`](sealed/index.md)

- `fn try_insert(self: &mut Self, index: usize, element: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](../index.md), [`CapacityFull`](sealed/index.md)

- `fn pop(self: &mut Self) -> Option<<A as >::Item>` — [`ArrayLike`](../index.md)

- `fn swap_remove(self: &mut Self, index: usize) -> <A as >::Item` — [`ArrayLike`](../index.md)

#### Trait Implementations

##### `impl<A: ArrayLike> Clone for ArrayVec<A>`

- `fn clone(self: &Self) -> Self`

##### `impl<A: ArrayLike> Debug for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: ArrayLike> Default for ArrayVec<A>`

- `fn default() -> Self`

##### `impl<A: ArrayLike> Deref for ArrayVec<A>`

- `type Target = [<A as ArrayLike>::Item]`

- `fn deref(self: &Self) -> &[<A as >::Item]` — [`ArrayLike`](../index.md)

##### `impl<A: ArrayLike> DerefMut for ArrayVec<A>`

- `fn deref_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`ArrayLike`](../index.md)

##### `impl<A: ArrayLike> Drop for ArrayVec<A>`

- `fn drop(self: &mut Self)`

##### `impl<A: ArrayLike> Eq for ArrayVec<A>`

##### `impl<A: ArrayLike> PartialEq for ArrayVec<A>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<P, T> Receiver for ArrayVec<A>`

- `type Target = T`

## Traits

### `ArrayLike`

```rust
trait ArrayLike: Sealed { ... }
```

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

#### Required Methods

- `type Item`

