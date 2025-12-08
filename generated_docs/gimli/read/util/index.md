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

- <span id="arrayvec-into-vec"></span>`fn into_vec(self) -> Vec<T>`

#### Trait Implementations

##### `impl<A: ArrayLike> Clone for ArrayVec<A>`

- <span id="arrayvec-clone"></span>`fn clone(&self) -> Self`

##### `impl<A: ArrayLike> Debug for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: ArrayLike> Default for ArrayVec<A>`

- <span id="arrayvec-default"></span>`fn default() -> Self`

##### `impl<A: ArrayLike> Deref for ArrayVec<A>`

- <span id="arrayvec-target"></span>`type Target = [<A as ArrayLike>::Item]`

- <span id="arrayvec-deref"></span>`fn deref(&self) -> &[<A as >::Item]` — [`ArrayLike`](../index.md)

##### `impl<A: ArrayLike> DerefMut for ArrayVec<A>`

- <span id="arrayvec-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [<A as >::Item]` — [`ArrayLike`](../index.md)

##### `impl<A: ArrayLike> Drop for ArrayVec<A>`

- <span id="arrayvec-drop"></span>`fn drop(&mut self)`

##### `impl<A: ArrayLike> Eq for ArrayVec<A>`

##### `impl<A: ArrayLike> PartialEq for ArrayVec<A>`

- <span id="arrayvec-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<P, T> Receiver for ArrayVec<A>`

- <span id="arrayvec-target"></span>`type Target = T`

## Traits

### `ArrayLike`

```rust
trait ArrayLike: Sealed { ... }
```

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

#### Required Methods

- `type Item`

