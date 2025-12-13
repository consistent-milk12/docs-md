*[compact_str](../../index.md) / [repr](../index.md) / [static_str](index.md)*

---

# Module `static_str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StaticStr`](#staticstr) | struct | A buffer stored on the stack whose size is equal to the stack size of `String` The last byte is set to 0. |
| [`DISCRIMINANT_SIZE`](#discriminant-size) | const |  |

## Structs

### `StaticStr`

```rust
struct StaticStr {
    ptr: ptr::NonNull<u8>,
    len: usize,
    discriminant: [u8; 8],
}
```

*Defined in [`compact_str-0.9.0/src/repr/static_str.rs:11-16`](../../../../.source_1765633015/compact_str-0.9.0/src/repr/static_str.rs#L11-L16)*

A buffer stored on the stack whose size is equal to the stack size of `String`
The last byte is set to 0.

#### Implementations

- <span id="staticstr-new"></span>`const fn new(text: &'static str) -> Self`

- <span id="staticstr-get-text"></span>`const fn get_text(&self) -> &'static str`

- <span id="staticstr-set-len"></span>`unsafe fn set_len(&mut self, len: usize)`

  # Safety

  * `len` bytes in the buffer must be valid UTF-8 and

  * `len` must be <= `self.get_text().len()`

#### Trait Implementations

##### `impl Any for StaticStr`

- <span id="staticstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StaticStr`

- <span id="staticstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StaticStr`

- <span id="staticstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StaticStr`

- <span id="staticstr-clone"></span>`fn clone(&self) -> StaticStr` — [`StaticStr`](#staticstr)

##### `impl CloneToUninit for StaticStr`

- <span id="staticstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StaticStr`

##### `impl<T> From for StaticStr`

- <span id="staticstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StaticStr`

- <span id="staticstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StaticStr`

- <span id="staticstr-toowned-type-owned"></span>`type Owned = T`

- <span id="staticstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="staticstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StaticStr`

- <span id="staticstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="staticstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StaticStr`

- <span id="staticstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="staticstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `DISCRIMINANT_SIZE`
```rust
const DISCRIMINANT_SIZE: usize = 8usize;
```

*Defined in [`compact_str-0.9.0/src/repr/static_str.rs:5`](../../../../.source_1765633015/compact_str-0.9.0/src/repr/static_str.rs#L5)*

