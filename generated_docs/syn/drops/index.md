*[syn](../index.md) / [drops](index.md)*

---

# Module `drops`

## Structs

### `NoDrop<T: ?Sized>`

```rust
struct NoDrop<T: ?Sized>(std::mem::ManuallyDrop<T>);
```

#### Implementations

- `fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T: ?Sized> Deref for NoDrop<T>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<T: ?Sized> DerefMut for NoDrop<T>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl<P, T> Receiver for NoDrop<T>`

- `type Target = T`

## Traits

### `TrivialDrop`

```rust
trait TrivialDrop { ... }
```

