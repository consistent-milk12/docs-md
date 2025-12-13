*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [mmap](index.md)*

---

# Module `mmap`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Mmap`](#mmap) | struct |  |

## Structs

### `Mmap`

```rust
struct Mmap {
    ptr: *mut libc::c_void,
    len: usize,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/mmap_unix.rs:12-15`](../../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli/mmap_unix.rs#L12-L15)*

#### Implementations

- <span id="mmap-map"></span>`unsafe fn map(file: &File, len: usize, offset: u64) -> Option<Mmap>` — [`Mmap`](#mmap)

#### Trait Implementations

##### `impl Any for Mmap`

- <span id="mmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mmap`

- <span id="mmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mmap`

- <span id="mmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for Mmap`

- <span id="mmap-deref-type-target"></span>`type Target = [u8]`

- <span id="mmap-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl Drop for Mmap`

- <span id="mmap-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Mmap`

- <span id="mmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Mmap`

- <span id="mmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for Mmap`

- <span id="mmap-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for Mmap`

- <span id="mmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Mmap`

- <span id="mmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

