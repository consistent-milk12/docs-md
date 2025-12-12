*[crossbeam_epoch](../../index.md) / [primitive](../index.md) / [cell](index.md)*

---

# Module `cell`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UnsafeCell`](#unsafecell) | struct |  |

## Structs

### `UnsafeCell<T>`

```rust
struct UnsafeCell<T>(::core::cell::UnsafeCell<T>);
```

*Defined in [`crossbeam-epoch-0.9.18/src/lib.rs:97`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/lib.rs#L97)*

#### Implementations

- <span id="unsafecell-new"></span>`const fn new(data: T) -> UnsafeCell<T>` — [`UnsafeCell`](#unsafecell)

- <span id="unsafecell-with"></span>`fn with<R>(&self, f: impl FnOnce(*const T) -> R) -> R`

- <span id="unsafecell-with-mut"></span>`fn with_mut<R>(&self, f: impl FnOnce(*mut T) -> R) -> R`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for UnsafeCell<T>`

- <span id="unsafecell-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for UnsafeCell<T>`

- <span id="unsafecell-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unsafecell-pointable-type-init"></span>`type Init = T`

- <span id="unsafecell-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="unsafecell-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unsafecell-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unsafecell-drop"></span>`unsafe fn drop(ptr: usize)`

