*[crossbeam_epoch](../../index.md) / [primitive](../index.md) / [cell](index.md)*

---

# Module `cell`

## Structs

### `UnsafeCell<T>`

```rust
struct UnsafeCell<T>(::core::cell::UnsafeCell<T>);
```

#### Implementations

- `const fn new(data: T) -> UnsafeCell<T>` — [`UnsafeCell`](#unsafecell)

- `fn with<R>(self: &Self, f: impl FnOnce(*const T) -> R) -> R`

- `fn with_mut<R>(self: &Self, f: impl FnOnce(*mut T) -> R) -> R`

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for UnsafeCell<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Pointable for UnsafeCell<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

