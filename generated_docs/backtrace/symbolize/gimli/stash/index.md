*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [stash](index.md)*

---

# Module `stash`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Stash`](#stash) | struct | A simple arena allocator for byte buffers. |

## Structs

### `Stash`

```rust
struct Stash {
    buffers: core::cell::UnsafeCell<alloc::vec::Vec<alloc::vec::Vec<u8>>>,
    mmaps: core::cell::UnsafeCell<alloc::vec::Vec<self::mmap::Mmap>>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/stash.rs:11-14`](../../../../../.source_1765633015/backtrace-0.3.76/src/symbolize/gimli/stash.rs#L11-L14)*

A simple arena allocator for byte buffers.

#### Implementations

- <span id="stash-new"></span>`fn new() -> Stash` — [`Stash`](#stash)

- <span id="stash-allocate"></span>`fn allocate(&self, size: usize) -> &mut [u8]`

  Allocates a buffer of the specified size and returns a mutable reference

  to it.

- <span id="stash-cache-mmap"></span>`fn cache_mmap(&self, map: Mmap) -> &[u8]` — [`Mmap`](../mmap/index.md#mmap)

  Stores a `Mmap` for the lifetime of this `Stash`, returning a pointer

  which is scoped to just this lifetime.

#### Trait Implementations

##### `impl Any for Stash`

- <span id="stash-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Stash`

- <span id="stash-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Stash`

- <span id="stash-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Stash`

- <span id="stash-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Stash`

- <span id="stash-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Stash`

- <span id="stash-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stash-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Stash`

- <span id="stash-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stash-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

