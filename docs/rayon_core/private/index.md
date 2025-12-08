*[rayon_core](../index.md) / [private](index.md)*

---

# Module `private`

The public parts of this private module are used to create traits
that cannot be implemented outside of our own crate.  This way we
can feel free to extend those traits without worrying about it
being a breaking change for other implementations.

## Structs

### `PrivateMarker`

```rust
struct PrivateMarker;
```

If this type is pub but not publicly reachable, third parties
can't name it and can't implement traits using it.

#### Trait Implementations

##### `impl<T> Pointable for PrivateMarker`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Macros

### `private_decl!`

### `private_impl!`

