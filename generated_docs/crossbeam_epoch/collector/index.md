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

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:22-24`](../../../.source_1765521767/crossbeam-epoch-0.9.18/src/collector.rs#L22-L24)*

An epoch-based garbage collector.

#### Implementations

- <span id="collector-new"></span>`fn new() -> Self`

  Creates a new collector.

- <span id="collector-register"></span>`fn register(&self) -> LocalHandle` — [`LocalHandle`](#localhandle)

  Registers a new handle for the collector.

#### Trait Implementations

##### `impl Any for Collector`

- <span id="collector-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Collector`

- <span id="collector-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Collector`

- <span id="collector-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Collector`

- <span id="collector-clone"></span>`fn clone(&self) -> Self`

  Creates another reference to the same garbage collector.

##### `impl CloneToUninit for Collector`

- <span id="collector-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Collector`

- <span id="collector-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl Eq for Collector`

##### `impl<T> From for Collector`

- <span id="collector-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Collector`

- <span id="collector-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Collector`

- <span id="collector-partialeq-eq"></span>`fn eq(&self, rhs: &Collector) -> bool` — [`Collector`](#collector)

  Checks if both handles point to the same collector.

##### `impl Pointable for Collector`

- <span id="collector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collector-pointable-type-init"></span>`type Init = T`

- <span id="collector-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="collector-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collector-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collector-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Send for Collector`

##### `impl Sync for Collector`

##### `impl ToOwned for Collector`

- <span id="collector-toowned-type-owned"></span>`type Owned = T`

- <span id="collector-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="collector-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Collector`

- <span id="collector-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collector-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Collector`

- <span id="collector-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collector-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocalHandle`

```rust
struct LocalHandle {
    local: *const crate::internal::Local,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:73-75`](../../../.source_1765521767/crossbeam-epoch-0.9.18/src/collector.rs#L73-L75)*

A handle to a garbage collector.

#### Implementations

- <span id="localhandle-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](../guard/index.md#guard)

  Pins the handle.

- <span id="localhandle-is-pinned"></span>`fn is_pinned(&self) -> bool`

  Returns `true` if the handle is pinned.

- <span id="localhandle-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](#collector)

  Returns the `Collector` associated with this handle.

#### Trait Implementations

##### `impl Any for LocalHandle`

- <span id="localhandle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocalHandle`

- <span id="localhandle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocalHandle`

- <span id="localhandle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LocalHandle`

- <span id="localhandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- <span id="localhandle-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for LocalHandle`

- <span id="localhandle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocalHandle`

- <span id="localhandle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for LocalHandle`

- <span id="localhandle-pointable-const-align"></span>`const ALIGN: usize`

- <span id="localhandle-pointable-type-init"></span>`type Init = T`

- <span id="localhandle-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="localhandle-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="localhandle-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="localhandle-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for LocalHandle`

- <span id="localhandle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="localhandle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocalHandle`

- <span id="localhandle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="localhandle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

