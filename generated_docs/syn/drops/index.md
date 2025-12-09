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

*Defined in [`syn-2.0.111/src/drops.rs:8`](../../../.source_1765210505/syn-2.0.111/src/drops.rs#L8)*

#### Implementations

- <span id="nodrop-new"></span>`fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T: ?Sized> Deref for NoDrop<T>`

- <span id="nodrop-type-target"></span>`type Target = T`

- <span id="nodrop-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: ?Sized> DerefMut for NoDrop<T>`

- <span id="nodrop-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<P, T> Receiver for NoDrop<T>`

- <span id="nodrop-type-target"></span>`type Target = T`

## Traits

### `TrivialDrop`

```rust
trait TrivialDrop { ... }
```

*Defined in [`syn-2.0.111/src/drops.rs:32`](../../../.source_1765210505/syn-2.0.111/src/drops.rs#L32)*

#### Implementors

- [`PrivateIterMut`](../punctuated/index.md)
- [`PrivateIter`](../punctuated/index.md)
- `iter::Empty<T>`
- `option::IntoIter<&T>`
- `option::IntoIter<&mut T>`
- `slice::Iter<'_, T>`
- `slice::IterMut<'_, T>`

