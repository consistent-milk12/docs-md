*[crossbeam_epoch](../index.md) / [collector](index.md)*

---

# Module `collector`

## Structs

### `Collector`

```rust
struct Collector {
    global: alloc::sync::Arc<crate::internal::Global>,
}
```

An epoch-based garbage collector.

#### Implementations

- `fn new() -> Self`

- `fn register(self: &Self) -> LocalHandle` — [`LocalHandle`](#localhandle)

#### Trait Implementations

##### `impl Clone for Collector`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for Collector`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- `fn default() -> Self`

##### `impl Eq for Collector`

##### `impl PartialEq for Collector`

- `fn eq(self: &Self, rhs: &Collector) -> bool` — [`Collector`](#collector)

##### `impl<T> Pointable for Collector`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn pin(self: &Self) -> Guard` — [`Guard`](../guard/index.md)

- `fn is_pinned(self: &Self) -> bool`

- `fn collector(self: &Self) -> &Collector` — [`Collector`](#collector)

#### Trait Implementations

##### `impl Debug for LocalHandle`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for LocalHandle`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

