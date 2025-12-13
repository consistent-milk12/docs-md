*[once_cell](../index.md) / [sync](index.md)*

---

# Module `sync`

Thread-safe, blocking version of `OnceCell`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceCell`](#oncecell) | struct | A thread-safe cell which can be written to only once. |
| [`Lazy`](#lazy) | struct | A value which is initialized on the first access. |
| [`_dummy`](#dummy) | fn | ```compile_fail struct S(*mut ()); unsafe impl Sync for S {} |

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T>(super::imp::OnceCell<T>);
```

*Defined in [`once_cell-1.21.3/src/lib.rs:901`](../../../.source_1765521767/once_cell-1.21.3/src/lib.rs#L901)*

A thread-safe cell which can be written to only once.

`OnceCell` provides `&` references to the contents without RAII guards.

Reading a non-`None` value out of `OnceCell` establishes a
happens-before relationship with a corresponding write. For example, if
thread A initializes the cell with `get_or_init(f)`, and thread B
subsequently reads the result of this call, B also observes all the side
effects of `f`.

# Example
```rust
use once_cell::sync::OnceCell;

static CELL: OnceCell<String> = OnceCell::new();
assert!(CELL.get().is_none());

std::thread::spawn(|| {
    let value: &String = CELL.get_or_init(|| {
        "Hello, World!".to_string()
    });
    assert_eq!(value, "Hello, World!");
}).join().unwrap();

let value: Option<&String> = CELL.get();
assert!(value.is_some());
assert_eq!(value.unwrap().as_str(), "Hello, World!");
```

#### Implementations

- <span id="oncecell-new"></span>`const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

  Creates a new empty cell.

- <span id="oncecell-with-value"></span>`const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

  Creates a new initialized cell.

- <span id="oncecell-get"></span>`fn get(&self) -> Option<&T>`

  Gets the reference to the underlying value.

  

  Returns `None` if the cell is empty, or being initialized. This

  method never blocks.

- <span id="oncecell-wait"></span>`fn wait(&self) -> &T`

  Gets the reference to the underlying value, blocking the current

  thread until it is set.

  

  ```rust

  use once_cell::sync::OnceCell;

  

  let mut cell = std::sync::Arc::new(OnceCell::new());

  let t = std::thread::spawn({

      let cell = std::sync::Arc::clone(&cell);

      move || cell.set(92).unwrap()

  });

  

  // Returns immediately, but might return None.

  let _value_or_none = cell.get();

  

  // Will return 92, but might block until the other thread does `.set`.

  let value: &u32 = cell.wait();

  assert_eq!(*value, 92);

  t.join().unwrap();

  ```

- <span id="oncecell-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut T>`

  Gets the mutable reference to the underlying value.

  

  Returns `None` if the cell is empty.

  

  This method is allowed to violate the invariant of writing to a `OnceCell`

  at most once because it requires `&mut` access to `self`. As with all

  interior mutability, `&mut` access permits arbitrary modification:

  

  ```rust

  use once_cell::sync::OnceCell;

  

  let mut cell: OnceCell<u32> = OnceCell::new();

  cell.set(92).unwrap();

  cell = OnceCell::new();

  ```

- <span id="oncecell-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> &T`

  Get the reference to the underlying value, without checking if the

  cell is initialized.

  

  # Safety

  

  Caller must ensure that the cell is in initialized state, and that

  the contents are acquired by (synchronized to) this thread.

- <span id="oncecell-set"></span>`fn set(&self, value: T) -> Result<(), T>`

  Sets the contents of this cell to `value`.

  

  Returns `Ok(())` if the cell was empty and `Err(value)` if it was

  full.

  

  # Example

  

  ```rust

  use once_cell::sync::OnceCell;

  

  static CELL: OnceCell<i32> = OnceCell::new();

  

  fn main() {

      assert!(CELL.get().is_none());

  

      std::thread::spawn(|| {

          assert_eq!(CELL.set(92), Ok(()));

      }).join().unwrap();

  

      assert_eq!(CELL.set(62), Err(62));

      assert_eq!(CELL.get(), Some(&92));

  }

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

  Gets the contents of the cell, initializing it with `f` if the cell

  was empty.

  

  Many threads may call `get_or_init` concurrently with different

  initializing functions, but it is guaranteed that only one function

  will be executed.

  

  # Panics

  

  If `f` panics, the panic is propagated to the caller, and the cell

  remains uninitialized.

  

  It is an error to reentrantly initialize the cell from `f`. The

  exact outcome is unspecified. Current implementation deadlocks, but

  this may be changed to a panic in the future.

  

  # Example

  ```rust

  use once_cell::sync::OnceCell;

  

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

  

  If `f` panics, the panic is propagated to the caller, and

  the cell remains uninitialized.

  

  It is an error to reentrantly initialize the cell from `f`.

  The exact outcome is unspecified. Current implementation

  deadlocks, but this may be changed to a panic in the future.

  

  # Example

  ```rust

  use once_cell::sync::OnceCell;

  

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

  use once_cell::sync::OnceCell;

  

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

  use once_cell::sync::OnceCell;

  

  let mut cell: OnceCell<u32> = OnceCell::new();

  cell.set(92).unwrap();

  cell = OnceCell::new();

  ```

- <span id="oncecell-into-inner"></span>`fn into_inner(self) -> Option<T>`

  Consumes the `OnceCell`, returning the wrapped value. Returns

  `None` if the cell was empty.

  

  # Examples

  

  ```rust

  use once_cell::sync::OnceCell;

  

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

- <span id="oncecell-default"></span>`fn default() -> OnceCell<T>` — [`OnceCell`](#oncecell)

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

- <span id="oncecell-partialeq-eq"></span>`fn eq(&self, other: &OnceCell<T>) -> bool` — [`OnceCell`](#oncecell)

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

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    cell: OnceCell<T>,
    init: core::cell::Cell<Option<F>>,
}
```

*Defined in [`once_cell-1.21.3/src/lib.rs:1255-1258`](../../../.source_1765521767/once_cell-1.21.3/src/lib.rs#L1255-L1258)*

A value which is initialized on the first access.

This type is thread-safe and can be used in statics.

# Example

```rust
use std::collections::HashMap;

use once_cell::sync::Lazy;

static HASHMAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

fn main() {
    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    }).join().unwrap();
    println!("{:?}", HASHMAP.get(&74));

    // Prints:
    //   ready
    //   initializing
    //   Some("Spica")
    //   Some("Hoyten")
}
```

#### Implementations

- <span id="lazy-new"></span>`const fn new(f: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

  Creates a new lazy value with the given initializing

  function.

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

##### `impl<T, F: Send> Sync for Lazy<T, F>`

##### `impl<T, U> TryFrom for Lazy<T, F>`

- <span id="lazy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Lazy<T, F>`

- <span id="lazy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `_dummy`

```rust
fn _dummy()
```

*Defined in [`once_cell-1.21.3/src/lib.rs:1408`](../../../.source_1765521767/once_cell-1.21.3/src/lib.rs#L1408)*

```compile_fail
struct S(*mut ());
unsafe impl Sync for S {}

fn share<T: Sync>(_: &T) {}
share(&once_cell::sync::OnceCell::<S>::new());
```

```compile_fail
struct S(*mut ());
unsafe impl Sync for S {}

fn share<T: Sync>(_: &T) {}
share(&once_cell::sync::Lazy::<S>::new(|| unimplemented!()));
```

