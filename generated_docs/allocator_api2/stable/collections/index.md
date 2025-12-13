*[allocator_api2](../../index.md) / [stable](../index.md) / [collections](index.md)*

---

# Module `collections`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReserveError`](#tryreserveerror) | struct |  |
| [`TryReserveErrorKind`](#tryreserveerrorkind) | enum |  |

## Structs

### `TryReserveError`

```rust
struct TryReserveError {
    kind: TryReserveErrorKind,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/raw_vec.rs:19-21`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/raw_vec.rs#L19-L21)*

The error type for `try_reserve` methods.

#### Implementations

- <span id="tryreserveerror-kind"></span>`fn kind(&self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](../raw_vec/index.md#tryreserveerrorkind)

  Details about the allocation that caused the error

#### Trait Implementations

##### `impl Any for TryReserveError`

- <span id="tryreserveerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryReserveError`

- <span id="tryreserveerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryReserveError`

- <span id="tryreserveerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TryReserveError`

- <span id="tryreserveerror-clone"></span>`fn clone(&self) -> TryReserveError` — [`TryReserveError`](../raw_vec/index.md#tryreserveerror)

##### `impl CloneToUninit for TryReserveError`

- <span id="tryreserveerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TryReserveError`

- <span id="tryreserveerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryReserveError`

- <span id="tryreserveerror-display-fmt"></span>`fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error>`

##### `impl Eq for TryReserveError`

##### `impl<T> From for TryReserveError`

- <span id="tryreserveerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryReserveError`

- <span id="tryreserveerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TryReserveError`

- <span id="tryreserveerror-partialeq-eq"></span>`fn eq(&self, other: &TryReserveError) -> bool` — [`TryReserveError`](../raw_vec/index.md#tryreserveerror)

##### `impl StructuralPartialEq for TryReserveError`

##### `impl ToOwned for TryReserveError`

- <span id="tryreserveerror-toowned-type-owned"></span>`type Owned = T`

- <span id="tryreserveerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryreserveerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for TryReserveError`

- <span id="tryreserveerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for TryReserveError`

- <span id="tryreserveerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryreserveerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryReserveError`

- <span id="tryreserveerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryreserveerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `TryReserveErrorKind`

```rust
enum TryReserveErrorKind {
    CapacityOverflow,
    AllocError {
        layout: super::alloc::Layout,
    },
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/raw_vec.rs:32-45`](../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/raw_vec.rs#L32-L45)*

Details of the allocation that caused a `TryReserveError`

#### Variants

- **`CapacityOverflow`**

  Error due to the computed capacity exceeding the collection's maximum
  (usually `isize::MAX` bytes).

- **`AllocError`**

  The memory allocator returned an error

#### Trait Implementations

##### `impl Any for TryReserveErrorKind`

- <span id="tryreserveerrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryReserveErrorKind`

- <span id="tryreserveerrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryReserveErrorKind`

- <span id="tryreserveerrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TryReserveErrorKind`

- <span id="tryreserveerrorkind-clone"></span>`fn clone(&self) -> TryReserveErrorKind` — [`TryReserveErrorKind`](../raw_vec/index.md#tryreserveerrorkind)

##### `impl CloneToUninit for TryReserveErrorKind`

- <span id="tryreserveerrorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TryReserveErrorKind`

- <span id="tryreserveerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryReserveErrorKind`

##### `impl<T> From for TryReserveErrorKind`

- <span id="tryreserveerrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryReserveErrorKind`

- <span id="tryreserveerrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TryReserveErrorKind`

- <span id="tryreserveerrorkind-partialeq-eq"></span>`fn eq(&self, other: &TryReserveErrorKind) -> bool` — [`TryReserveErrorKind`](../raw_vec/index.md#tryreserveerrorkind)

##### `impl StructuralPartialEq for TryReserveErrorKind`

##### `impl ToOwned for TryReserveErrorKind`

- <span id="tryreserveerrorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="tryreserveerrorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryreserveerrorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryReserveErrorKind`

- <span id="tryreserveerrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryreserveerrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryReserveErrorKind`

- <span id="tryreserveerrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryreserveerrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

