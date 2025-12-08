*[crossbeam_epoch](../../index.md) / [sync](../index.md) / [once_lock](index.md)*

---

# Module `once_lock`

## Structs

### `OnceLock<T>`

```rust
struct OnceLock<T> {
    once: std::sync::Once,
    value: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>,
}
```

#### Implementations

- `const fn new() -> Self`

- `fn get_or_init<F>(self: &Self, f: F) -> &T`

- `fn initialize<F>(self: &Self, f: F)`

- `unsafe fn get_unchecked(self: &Self) -> &T`

#### Trait Implementations

##### `impl<T> Drop for OnceLock<T>`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for OnceLock<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for OnceLock<T>`

##### `impl<T: Sync + Send> Sync for OnceLock<T>`

