*[once_cell](../index.md) / [race](index.md)*

---

# Module `race`

Thread-safe, non-blocking, "first one wins" flavor of `OnceCell`.

If two threads race to initialize a type from the `race` module, they
don't block, execute initialization function together, but only one of
them stores the result.

This module does not require `std` feature.

# Atomic orderings

All types in this module use `Acquire` and `Release`
[atomic orderings](Ordering) for all their operations. While this is not
strictly necessary for types other than `OnceBox`, it is useful for users as
it allows them to be certain that after `get` or `get_or_init` returns on
one thread, any side-effects caused by the setter thread prior to them
calling `set` or `get_or_init` will be made visible to that thread; without
it, it's possible for it to appear as if they haven't happened yet from the
getter thread's perspective. This is an acceptable tradeoff to make since
`Acquire` and `Release` have very little performance overhead on most
architectures versus `Relaxed`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`once_box`](#once-box) | mod |  |
| [`OnceNonZeroUsize`](#oncenonzerousize) | struct | A thread-safe cell which can be written to only once. |
| [`OnceBool`](#oncebool) | struct | A thread-safe cell which can be written to only once. |
| [`OnceRef`](#onceref) | struct | A thread-safe cell which can be written to only once. |
| [`OnceBox`](#oncebox) | struct |  |

## Modules

- [`once_box`](once_box/index.md)

## Structs

### `OnceNonZeroUsize`

```rust
struct OnceNonZeroUsize {
    inner: atomic::AtomicUsize,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:43-45`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L43-L45)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="oncenonzerousize-new"></span>`const fn new() -> Self`

  Creates a new empty cell.

- <span id="oncenonzerousize-get"></span>`fn get(&self) -> Option<NonZeroUsize>`

  Gets the underlying value.

- <span id="oncenonzerousize-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> NonZeroUsize`

  Get the reference to the underlying value, without checking if the cell

  is initialized.

  

  # Safety

  

  Caller must ensure that the cell is in initialized state, and that

  the contents are acquired by (synchronized to) this thread.

- <span id="oncenonzerousize-set"></span>`fn set(&self, value: NonZeroUsize) -> Result<(), ()>`

  Sets the contents of this cell to `value`.

  

  Returns `Ok(())` if the cell was empty and `Err(())` if it was

  full.

- <span id="oncenonzerousize-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> NonZeroUsize`

  Gets the contents of the cell, initializing it with `f` if the cell was

  empty.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncenonzerousize-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>`

  Gets the contents of the cell, initializing it with `f` if

  the cell was empty. If the cell was empty and `f` failed, an

  error is returned.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncenonzerousize-init"></span>`fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E>`

- <span id="oncenonzerousize-compare-exchange"></span>`fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize>`

#### Trait Implementations

##### `impl Any for OnceNonZeroUsize`

- <span id="oncenonzerousize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceNonZeroUsize`

- <span id="oncenonzerousize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceNonZeroUsize`

- <span id="oncenonzerousize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for OnceNonZeroUsize`

- <span id="oncenonzerousize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OnceNonZeroUsize`

- <span id="oncenonzerousize-default"></span>`fn default() -> OnceNonZeroUsize` — [`OnceNonZeroUsize`](#oncenonzerousize)

##### `impl<T> From for OnceNonZeroUsize`

- <span id="oncenonzerousize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OnceNonZeroUsize`

- <span id="oncenonzerousize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OnceNonZeroUsize`

- <span id="oncenonzerousize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncenonzerousize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OnceNonZeroUsize`

- <span id="oncenonzerousize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncenonzerousize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OnceBool`

```rust
struct OnceBool {
    inner: OnceNonZeroUsize,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:167-169`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L167-L169)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="oncebool-new"></span>`const fn new() -> Self`

  Creates a new empty cell.

- <span id="oncebool-get"></span>`fn get(&self) -> Option<bool>`

  Gets the underlying value.

- <span id="oncebool-set"></span>`fn set(&self, value: bool) -> Result<(), ()>`

  Sets the contents of this cell to `value`.

  

  Returns `Ok(())` if the cell was empty and `Err(())` if it was

  full.

- <span id="oncebool-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> bool`

  Gets the contents of the cell, initializing it with `f` if the cell was

  empty.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncebool-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>`

  Gets the contents of the cell, initializing it with `f` if

  the cell was empty. If the cell was empty and `f` failed, an

  error is returned.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncebool-from-usize"></span>`fn from_usize(value: NonZeroUsize) -> bool`

- <span id="oncebool-to-usize"></span>`fn to_usize(value: bool) -> NonZeroUsize`

#### Trait Implementations

##### `impl Any for OnceBool`

- <span id="oncebool-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceBool`

- <span id="oncebool-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceBool`

- <span id="oncebool-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for OnceBool`

- <span id="oncebool-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OnceBool`

- <span id="oncebool-default"></span>`fn default() -> OnceBool` — [`OnceBool`](#oncebool)

##### `impl<T> From for OnceBool`

- <span id="oncebool-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OnceBool`

- <span id="oncebool-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OnceBool`

- <span id="oncebool-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncebool-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OnceBool`

- <span id="oncebool-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncebool-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OnceRef<'a, T>`

```rust
struct OnceRef<'a, T> {
    inner: atomic::AtomicPtr<T>,
    ghost: core::marker::PhantomData<core::cell::UnsafeCell<&'a T>>,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:232-235`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L232-L235)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="onceref-new"></span>`const fn new() -> Self`

  Creates a new empty cell.

- <span id="onceref-get"></span>`fn get(&self) -> Option<&'a T>`

  Gets a reference to the underlying value.

- <span id="onceref-set"></span>`fn set(&self, value: &'a T) -> Result<(), ()>`

  Sets the contents of this cell to `value`.

  

  Returns `Ok(())` if the cell was empty and `Err(value)` if it was

  full.

- <span id="onceref-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &'a T`

  Gets the contents of the cell, initializing it with `f` if the cell was

  empty.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="onceref-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<&'a T, E>`

  Gets the contents of the cell, initializing it with `f` if

  the cell was empty. If the cell was empty and `f` failed, an

  error is returned.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="onceref-init"></span>`fn init<E>(&self, f: impl FnOnce() -> Result<&'a T, E>) -> Result<&'a T, E>`

- <span id="onceref-compare-exchange"></span>`fn compare_exchange(&self, value: &'a T) -> Result<(), *const T>`

- <span id="onceref-dummy"></span>`fn _dummy()`

  ```compile_fail

  use once_cell::race::OnceRef;

  

  let mut l = OnceRef::new();

  

  {

      let y = 2;

      let mut r = OnceRef::new();

      r.set(&y).unwrap();

      core::mem::swap(&mut l, &mut r);

  }

  

  // l now contains a dangling reference to y

  eprintln!("uaf: {}", l.get().unwrap());

  ```

#### Trait Implementations

##### `impl<T> Any for OnceRef<'a, T>`

- <span id="onceref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceRef<'a, T>`

- <span id="onceref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceRef<'a, T>`

- <span id="onceref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for OnceRef<'a, T>`

- <span id="onceref-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for OnceRef<'a, T>`

- <span id="onceref-default"></span>`fn default() -> Self`

##### `impl<T> From for OnceRef<'a, T>`

- <span id="onceref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OnceRef<'a, T>`

- <span id="onceref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Sync> Sync for OnceRef<'a, T>`

##### `impl<T, U> TryFrom for OnceRef<'a, T>`

- <span id="onceref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="onceref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OnceRef<'a, T>`

- <span id="onceref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="onceref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OnceBox<T>`

```rust
struct OnceBox<T> {
    inner: super::atomic::AtomicPtr<T>,
    ghost: core::marker::PhantomData<Option<alloc::boxed::Box<T>>>,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:361-364`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L361-L364)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="oncebox-new"></span>`const fn new() -> Self`

  Creates a new empty cell.

- <span id="oncebox-with-value"></span>`fn with_value(value: Box<T>) -> Self`

  Creates a new cell with the given value.

- <span id="oncebox-get"></span>`fn get(&self) -> Option<&T>`

  Gets a reference to the underlying value.

- <span id="oncebox-set"></span>`fn set(&self, value: Box<T>) -> Result<(), Box<T>>`

  Sets the contents of this cell to `value`.

  

  Returns `Ok(())` if the cell was empty and `Err(value)` if it was

  full.

- <span id="oncebox-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

  Gets the contents of the cell, initializing it with `f` if the cell was

  empty.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncebox-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>`

  Gets the contents of the cell, initializing it with `f` if

  the cell was empty. If the cell was empty and `f` failed, an

  error is returned.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncebox-init"></span>`fn init<E>(&self, f: impl FnOnce() -> Result<Box<T>, E>) -> Result<&T, E>`

#### Trait Implementations

##### `impl<T> Any for OnceBox<T>`

- <span id="oncebox-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceBox<T>`

- <span id="oncebox-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceBox<T>`

- <span id="oncebox-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone> Clone for OnceBox<T>`

- <span id="oncebox-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for OnceBox<T>`

- <span id="oncebox-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for OnceBox<T>`

- <span id="oncebox-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for OnceBox<T>`

- <span id="oncebox-default"></span>`fn default() -> Self`

##### `impl<T> Drop for OnceBox<T>`

- <span id="oncebox-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for OnceBox<T>`

- <span id="oncebox-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OnceBox<T>`

- <span id="oncebox-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Sync + Send> Sync for OnceBox<T>`

##### `impl<T> ToOwned for OnceBox<T>`

- <span id="oncebox-toowned-type-owned"></span>`type Owned = T`

- <span id="oncebox-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="oncebox-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for OnceBox<T>`

- <span id="oncebox-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncebox-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OnceBox<T>`

- <span id="oncebox-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncebox-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

