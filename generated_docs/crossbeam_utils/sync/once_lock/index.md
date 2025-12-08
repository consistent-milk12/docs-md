*[crossbeam_utils](../../index.md) / [sync](../index.md) / [once_lock](index.md)*

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

#### Implementations

- <span id="oncelock-new"></span>`const fn new() -> Self`

- <span id="oncelock-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

- <span id="oncelock-initialize"></span>`fn initialize<F>(&self, f: F)`

- <span id="oncelock-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> &T`

#### Trait Implementations

##### `impl<T> Drop for OnceLock<T>`

- <span id="oncelock-drop"></span>`fn drop(&mut self)`

##### `impl<T: Send> Send for OnceLock<T>`

##### `impl<T: Sync + Send> Sync for OnceLock<T>`

