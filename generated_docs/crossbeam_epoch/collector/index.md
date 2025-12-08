*[crossbeam_epoch](../index.md) / [collector](index.md)*

---

# Module `collector`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Collector`](#collector) | struct | An epoch-based garbage collector. |
| [`LocalHandle`](#localhandle) | struct | A handle to a garbage collector. |

## Structs

### `Collector`

```rust
struct Collector {
    global: alloc::sync::Arc<crate::internal::Global>,
}
```

An epoch-based garbage collector.

#### Implementations

- <span id="collector-new"></span>`fn new() -> Self`

- <span id="collector-register"></span>`fn register(&self) -> LocalHandle` — [`LocalHandle`](../index.md)

#### Trait Implementations

##### `impl Clone for Collector`

- <span id="collector-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Collector`

- <span id="collector-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl Eq for Collector`

##### `impl PartialEq for Collector`

- <span id="collector-eq"></span>`fn eq(&self, rhs: &Collector) -> bool` — [`Collector`](../index.md)

##### `impl<T> Pointable for Collector`

- <span id="collector-align"></span>`const ALIGN: usize`

- <span id="collector-init"></span>`type Init = T`

- <span id="collector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- <span id="collector-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collector-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collector-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Send for Collector`

##### `impl Sync for Collector`

### `LocalHandle`

```rust
struct LocalHandle {
    local: *const crate::internal::Local,
}
```

A handle to a garbage collector.

#### Implementations

- <span id="localhandle-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](../index.md)

- <span id="localhandle-is-pinned"></span>`fn is_pinned(&self) -> bool`

- <span id="localhandle-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](../index.md)

#### Trait Implementations

##### `impl Debug for LocalHandle`

- <span id="localhandle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- <span id="localhandle-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for LocalHandle`

- <span id="localhandle-align"></span>`const ALIGN: usize`

- <span id="localhandle-init"></span>`type Init = T`

- <span id="localhandle-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- <span id="localhandle-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="localhandle-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="localhandle-drop"></span>`unsafe fn drop(ptr: usize)`

