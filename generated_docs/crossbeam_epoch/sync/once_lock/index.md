*[crossbeam_epoch](../../index.md) / [sync](../index.md) / [once_lock](index.md)*

---

# Module `once_lock`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceLock`](#oncelock) | struct |  |

## Structs

### `OnceLock<T>`

```rust
struct OnceLock<T> {
    once: std::sync::Once,
    value: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/once_lock.rs:9-14`](../../../../.source_1765210505/crossbeam-epoch-0.9.18/src/sync/once_lock.rs#L9-L14)*

#### Implementations

- <span id="oncelock-new"></span>`const fn new() -> Self`

- <span id="oncelock-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

- <span id="oncelock-initialize"></span>`fn initialize<F>(&self, f: F)`

- <span id="oncelock-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> &T`

#### Trait Implementations

##### `impl<T> Drop for OnceLock<T>`

- <span id="oncelock-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for OnceLock<T>`

- <span id="oncelock-const-align"></span>`const ALIGN: usize`

- <span id="oncelock-type-init"></span>`type Init = T`

- <span id="oncelock-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md)

- <span id="oncelock-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="oncelock-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="oncelock-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for OnceLock<T>`

##### `impl<T: Sync + Send> Sync for OnceLock<T>`

