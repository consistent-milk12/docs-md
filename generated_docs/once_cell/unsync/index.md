*[once_cell](../index.md) / [unsync](index.md)*

---

# Module `unsync`

Single-threaded version of `OnceCell`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceCell`](#oncecell) | struct | A cell which can be written to only once. |
| [`Lazy`](#lazy) | struct | A value which is initialized on the first access. |

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T> {
    inner: core::cell::UnsafeCell<Option<T>>,
}
```

*Defined in [`once_cell-1.21.3/src/lib.rs:411-414`](../../../.source_1765521767/once_cell-1.21.3/src/lib.rs#L411-L414)*

A cell which can be written to only once. It is not thread safe.

Unlike `std::cell::RefCell`, a `OnceCell` provides simple `&`
references to the contents.

# Example
```rust
use once_cell::unsync::OnceCell;

let cell = OnceCell::new();
assert!(cell.get().is_none());

let value: &String = cell.get_or_init(|| {
    "Hello, World!".to_string()
});
assert_eq!(value, "Hello, World!");
assert!(cell.get().is_some());
```

#### Implementations

- <span id="oncecell-new"></span>`const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

  Creates a new empty cell.

- <span id="oncecell-with-value"></span>`const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

  Creates a new initialized cell.

- <span id="oncecell-get"></span>`fn get(&self) -> Option<&T>`

  Gets a reference to the underlying value.

  

  Returns `None` if the cell is empty.

- <span id="oncecell-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut T>`

  Gets a mutable reference to the underlying value.

  

  Returns `None` if the cell is empty.

  

  This method is allowed to violate the invariant of writing to a `OnceCell`

  at most once because it requires `&mut` access to `self`. As with all

  interior mutability, `&mut` access permits arbitrary modification:

  

  ```rust

  use once_cell::unsync::OnceCell;

  

  let mut cell: OnceCell<u32> = OnceCell::new();

  cell.set(92).unwrap();

  *cell.get_mut().unwrap() = 93;

  assert_eq!(cell.get(), Some(&93));

  ```

- <span id="oncecell-set"></span>`fn set(&self, value: T) -> Result<(), T>`

  Sets the contents of this cell to `value`.

  

  Returns `Ok(())` if the cell was empty and `Err(value)` if it was

  full.

  

  # Example

  ```rust

  use once_cell::unsync::OnceCell;

  

  let cell = OnceCell::new();

  assert!(cell.get().is_none());

  

  assert_eq!(cell.set(92), Ok(()));

  assert_eq!(cell.set(62), Err(62));

  

  assert!(cell.get().is_some());

  ```

- <span id="oncecell-try-insert"></span>`fn try_insert(&self, value: T) -> Result<&T, (&T, T)>`

  Like [`set`](Self::set), but also returns a reference to the final cell value.

  

  # Example

  ```rust

  use once_cell::unsync::OnceCell;

  

  let cell = OnceCell::new();

  assert!(cell.get().is_none());

  

  assert_eq!(cell.try_insert(92), Ok(&92));

  assert_eq!(cell.try_insert(62), Err((&92, 62)));

  

  assert!(cell.get().is_some());

  ```

- <span id="oncecell-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

  Gets the contents of the cell, initializing it with `f`

  if the cell was empty.

  

  # Panics

  

  If `f` panics, the panic is propagated to the caller, and the cell

  remains uninitialized.

  

  It is an error to reentrantly initialize the cell from `f`. Doing

  so results in a panic.

  

  # Example

  ```rust

  use once_cell::unsync::OnceCell;

  

  let cell = OnceCell::new();

  let value = cell.get_or_init(|| 92);

  assert_eq!(value, &92);

  let value = cell.get_or_init(|| unreachable!());

  assert_eq!(value, &92);

  ```

- <span id="oncecell-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>`

  Gets the contents of the cell, initializing it with `f` if

  the cell was empty. If the cell was empty and `f` failed, an

  error is returned.

  

  # Panics

  

  If `f` panics, the panic is propagated to the caller, and the cell

  remains uninitialized.

  

  It is an error to reentrantly initialize the cell from `f`. Doing

  so results in a panic.

  

  # Example

  ```rust

  use once_cell::unsync::OnceCell;

  

  let cell = OnceCell::new();

  assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));

  assert!(cell.get().is_none());

  let value = cell.get_or_try_init(|| -> Result<i32, ()> {

      Ok(92)

  });

  assert_eq!(value, Ok(&92));

  assert_eq!(cell.get(), Some(&92))

  ```

- <span id="oncecell-take"></span>`fn take(&mut self) -> Option<T>`

  Takes the value out of this `OnceCell`, moving it back to an uninitialized state.

  

  Has no effect and returns `None` if the `OnceCell` hasn't been initialized.

  

  # Examples

  

  ```rust

  use once_cell::unsync::OnceCell;

  

  let mut cell: OnceCell<String> = OnceCell::new();

  assert_eq!(cell.take(), None);

  

  let mut cell = OnceCell::new();

  cell.set("hello".to_string()).unwrap();

  assert_eq!(cell.take(), Some("hello".to_string()));

  assert_eq!(cell.get(), None);

  ```

  

  This method is allowed to violate the invariant of writing to a `OnceCell`

  at most once because it requires `&mut` access to `self`. As with all

  interior mutability, `&mut` access permits arbitrary modification:

  

  ```rust

  use once_cell::unsync::OnceCell;

  

  let mut cell: OnceCell<u32> = OnceCell::new();

  cell.set(92).unwrap();

  cell = OnceCell::new();

  ```

- <span id="oncecell-into-inner"></span>`fn into_inner(self) -> Option<T>`

  Consumes the `OnceCell`, returning the wrapped value.

  

  Returns `None` if the cell was empty.

  

  # Examples

  

  ```rust

  use once_cell::unsync::OnceCell;

  

  let cell: OnceCell<String> = OnceCell::new();

  assert_eq!(cell.into_inner(), None);

  

  let cell = OnceCell::new();

  cell.set("hello".to_string()).unwrap();

  assert_eq!(cell.into_inner(), Some("hello".to_string()));

  ```

#### Trait Implementations

##### `impl<T> Any for OnceCell<T>`

- <span id="oncecell-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceCell<T>`

- <span id="oncecell-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceCell<T>`

- <span id="oncecell-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone> Clone for OnceCell<T>`

- <span id="oncecell-clone"></span>`fn clone(&self) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T> CloneToUninit for OnceCell<T>`

- <span id="oncecell-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for OnceCell<T>`

- <span id="oncecell-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for OnceCell<T>`

- <span id="oncecell-default"></span>`fn default() -> Self`

##### `impl<T: Eq> Eq for OnceCell<T>`

##### `impl<T> From for OnceCell<T>`

- <span id="oncecell-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OnceCell<T>`

- <span id="oncecell-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: PartialEq> PartialEq for OnceCell<T>`

- <span id="oncecell-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceCell<T>`

##### `impl<T> ToOwned for OnceCell<T>`

- <span id="oncecell-toowned-type-owned"></span>`type Owned = T`

- <span id="oncecell-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="oncecell-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for OnceCell<T>`

- <span id="oncecell-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncecell-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OnceCell<T>`

- <span id="oncecell-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncecell-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: UnwindSafe> UnwindSafe for OnceCell<T>`

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    cell: OnceCell<T>,
    init: core::cell::Cell<Option<F>>,
}
```

*Defined in [`once_cell-1.21.3/src/lib.rs:714-717`](../../../.source_1765521767/once_cell-1.21.3/src/lib.rs#L714-L717)*

A value which is initialized on the first access.

# Example
```rust
use once_cell::unsync::Lazy;

let lazy: Lazy<i32> = Lazy::new(|| {
    println!("initializing");
    92
});
println!("ready");
println!("{}", *lazy);
println!("{}", *lazy);

// Prints:
//   ready
//   initializing
//   92
//   92
```

#### Implementations

- <span id="lazy-new"></span>`const fn new(init: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

  Creates a new lazy value with the given initializing function.

  

  # Example

  ```rust

  fn main() {

  use once_cell::unsync::Lazy;

  

  let hello = "Hello, World!".to_string();

  

  let lazy = Lazy::new(|| hello.to_uppercase());

  

  assert_eq!(&*lazy, "HELLO, WORLD!");

  }

  ```

- <span id="lazy-into-value"></span>`fn into_value(this: Lazy<T, F>) -> Result<T, F>` — [`Lazy`](#lazy)

  Consumes this `Lazy` returning the stored value.

  

  Returns `Ok(value)` if `Lazy` is initialized and `Err(f)` otherwise.

#### Trait Implementations

##### `impl<T> Any for Lazy<T, F>`

- <span id="lazy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lazy<T, F>`

- <span id="lazy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lazy<T, F>`

- <span id="lazy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, F> Debug for Lazy<T, F>`

- <span id="lazy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for Lazy<T>`

- <span id="lazy-default"></span>`fn default() -> Lazy<T>` — [`Lazy`](#lazy)

  Creates a new lazy value using `Default` as the initializing function.

##### `impl<T, F: FnOnce() -> T> Deref for Lazy<T, F>`

- <span id="lazy-deref-type-target"></span>`type Target = T`

- <span id="lazy-deref"></span>`fn deref(&self) -> &T`

##### `impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F>`

- <span id="lazy-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T> From for Lazy<T, F>`

- <span id="lazy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Lazy<T, F>`

- <span id="lazy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Receiver for Lazy<T, F>`

- <span id="lazy-receiver-type-target"></span>`type Target = T`

##### `impl<T, F: RefUnwindSafe> RefUnwindSafe for Lazy<T, F>`

##### `impl<T, U> TryFrom for Lazy<T, F>`

- <span id="lazy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Lazy<T, F>`

- <span id="lazy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

