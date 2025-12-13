*[libc](../index.md) / [types](index.md)*

---

# Module `types`

Platform-agnostic support types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Padding`](#padding) | struct | A transparent wrapper over `MaybeUninit<T>` to represent uninitialized padding while providing `Default`. |
| [`CEnumRepr`](#cenumrepr) | type |  |

## Structs

### `Padding<T: Copy>`

```rust
struct Padding<T: Copy>(core::mem::MaybeUninit<T>);
```

*Defined in [`libc-0.2.178/src/types.rs:17`](../../../.source_1765633015/libc-0.2.178/src/types.rs#L17)*

A transparent wrapper over `MaybeUninit<T>` to represent uninitialized padding
while providing `Default`.

#### Trait Implementations

##### `impl<T> Any for Padding<T>`

- <span id="padding-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Padding<T>`

- <span id="padding-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Padding<T>`

- <span id="padding-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + Copy> Clone for Padding<T>`

- <span id="padding-clone"></span>`fn clone(&self) -> Padding<T>` — [`Padding`](#padding)

##### `impl<T> CloneToUninit for Padding<T>`

- <span id="padding-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy + Copy> Copy for Padding<T>`

##### `impl<T: Copy> Debug for Padding<T>`

- <span id="padding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Copy> Default for Padding<T>`

- <span id="padding-default"></span>`fn default() -> Self`

##### `impl<T> From for Padding<T>`

- <span id="padding-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Padding<T>`

- <span id="padding-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Padding<T>`

- <span id="padding-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="padding-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Padding<T>`

- <span id="padding-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="padding-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `CEnumRepr`

```rust
type CEnumRepr = crate::c_uint;
```

*Defined in [`libc-0.2.178/src/types.rs:59`](../../../.source_1765633015/libc-0.2.178/src/types.rs#L59)*

