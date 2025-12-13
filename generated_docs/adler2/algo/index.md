*[adler2](../index.md) / [algo](index.md)*

---

# Module `algo`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`U32X4`](#u32x4) | struct |  |

## Structs

### `U32X4`

```rust
struct U32X4([u32; 4]);
```

*Defined in [`adler2-2.0.1/src/algo.rs:112`](../../../.source_1765633015/adler2-2.0.1/src/algo.rs#L112)*

#### Implementations

- <span id="u32x4-from"></span>`fn from(bytes: &[u8]) -> Self`

#### Trait Implementations

##### `impl AddAssign for U32X4`

- <span id="u32x4-addassign-add-assign"></span>`fn add_assign(&mut self, other: Self)`

##### `impl Any for U32X4`

- <span id="u32x4-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for U32X4`

- <span id="u32x4-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for U32X4`

- <span id="u32x4-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for U32X4`

- <span id="u32x4-clone"></span>`fn clone(&self) -> U32X4` — [`U32X4`](#u32x4)

##### `impl CloneToUninit for U32X4`

- <span id="u32x4-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for U32X4`

##### `impl<T> From for U32X4`

- <span id="u32x4-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for U32X4`

- <span id="u32x4-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl MulAssign for U32X4`

- <span id="u32x4-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: u32)`

##### `impl RemAssign for U32X4`

- <span id="u32x4-remassign-rem-assign"></span>`fn rem_assign(&mut self, quotient: u32)`

##### `impl<U> TryFrom for U32X4`

- <span id="u32x4-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="u32x4-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for U32X4`

- <span id="u32x4-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="u32x4-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

