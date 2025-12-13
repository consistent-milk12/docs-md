*[gimli](../../../index.md) / [read](../../index.md) / [util](../index.md) / [sealed](index.md)*

---

# Module `sealed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CapacityFull`](#capacityfull) | struct |  |
| [`Sealed`](#sealed) | trait | # Safety Implementer must not modify the content in storage. |

## Structs

### `CapacityFull`

```rust
struct CapacityFull;
```

*Defined in [`gimli-0.32.3/src/read/util.rs:25`](../../../../../.source_1765633015/gimli-0.32.3/src/read/util.rs#L25)*

#### Trait Implementations

##### `impl Any for CapacityFull`

- <span id="capacityfull-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CapacityFull`

- <span id="capacityfull-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CapacityFull`

- <span id="capacityfull-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CapacityFull`

- <span id="capacityfull-clone"></span>`fn clone(&self) -> CapacityFull` — [`CapacityFull`](#capacityfull)

##### `impl CloneToUninit for CapacityFull`

- <span id="capacityfull-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CapacityFull`

##### `impl Debug for CapacityFull`

- <span id="capacityfull-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CapacityFull`

- <span id="capacityfull-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CapacityFull`

- <span id="capacityfull-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CapacityFull`

- <span id="capacityfull-toowned-type-owned"></span>`type Owned = T`

- <span id="capacityfull-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="capacityfull-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CapacityFull`

- <span id="capacityfull-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capacityfull-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CapacityFull`

- <span id="capacityfull-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capacityfull-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

*Defined in [`gimli-0.32.3/src/read/util.rs:14-22`](../../../../../.source_1765633015/gimli-0.32.3/src/read/util.rs#L14-L22)*

# Safety
Implementer must not modify the content in storage.

#### Associated Types

- `type Storage`

#### Required Methods

- `fn new_storage() -> <Self as >::Storage`

#### Provided Methods

- `fn grow(_storage: &mut <Self as >::Storage, _additional: usize) -> Result<(), CapacityFull>`

#### Implementors

- `[T; N]`
- `alloc::boxed::Box<[T; N]>`
- `alloc::vec::Vec<T>`

