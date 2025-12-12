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

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:22-24`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/collector.rs#L22-L24)*

An epoch-based garbage collector.

#### Implementations

- <span id="collector-new"></span>`fn new() -> Self`

- <span id="collector-register"></span>`fn register(&self) -> LocalHandle` — [`LocalHandle`](#localhandle)

#### Trait Implementations

##### `impl Clone for Collector`

- <span id="collector-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Collector`

- <span id="collector-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl Eq for Collector`

##### `impl PartialEq for Collector`

- <span id="collector-eq"></span>`fn eq(&self, rhs: &Collector) -> bool` — [`Collector`](#collector)

##### `impl Pointable for Collector`

- <span id="collector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collector-pointable-type-init"></span>`type Init = T`

- <span id="collector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

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

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:73-75`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/collector.rs#L73-L75)*

A handle to a garbage collector.

#### Implementations

- <span id="localhandle-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](../guard/index.md#guard)

- <span id="localhandle-is-pinned"></span>`fn is_pinned(&self) -> bool`

- <span id="localhandle-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](#collector)

#### Trait Implementations

##### `impl Debug for LocalHandle`

- <span id="localhandle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- <span id="localhandle-drop"></span>`fn drop(&mut self)`

##### `impl Pointable for LocalHandle`

- <span id="localhandle-pointable-const-align"></span>`const ALIGN: usize`

- <span id="localhandle-pointable-type-init"></span>`type Init = T`

- <span id="localhandle-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="localhandle-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="localhandle-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="localhandle-drop"></span>`unsafe fn drop(ptr: usize)`

