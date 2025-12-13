*[crossbeam_utils](../../index.md) / [sync](../index.md) / [wait_group](index.md)*

---

# Module `wait_group`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WaitGroup`](#waitgroup) | struct | Enables threads to synchronize the beginning or end of some computation. |
| [`Inner`](#inner) | struct | Inner state of a `WaitGroup`. |

## Structs

### `WaitGroup`

```rust
struct WaitGroup {
    inner: std::sync::Arc<Inner>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/wait_group.rs:46-48`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/wait_group.rs#L46-L48)*

Enables threads to synchronize the beginning or end of some computation.

# Wait groups vs barriers

`WaitGroup` is very similar to `Barrier`, but there are a few differences:

* `Barrier` needs to know the number of threads at construction, while `WaitGroup` is cloned to
  register more threads.

* A `Barrier` can be reused even after all threads have synchronized, while a `WaitGroup`
  synchronizes threads only once.

* All threads wait for others to reach the `Barrier`. With `WaitGroup`, each thread can choose
  to either wait for other threads or to continue without blocking.

# Examples

```rust
use crossbeam_utils::sync::WaitGroup;
use std::thread;

// Create a new wait group.
let wg = WaitGroup::new();

for _ in 0..4 {
    // Create another reference to the wait group.
    let wg = wg.clone();

    thread::spawn(move || {
        // Do some work.

        // Drop the reference to the wait group.
        drop(wg);
    });
}

// Block until all threads have finished their work.
wg.wait();
std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371
```


#### Implementations

- <span id="waitgroup-new"></span>`fn new() -> Self`

  Creates a new wait group and returns the single reference to it.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::WaitGroup;

  

  let wg = WaitGroup::new();

  ```

- <span id="waitgroup-wait"></span>`fn wait(self)`

  Drops this reference and waits until all other references are dropped.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::WaitGroup;

  use std::thread;

  

  let wg = WaitGroup::new();

  

  thread::spawn({

      let wg = wg.clone();

      move || {

          // Block until both threads have reached `wait()`.

          wg.wait();

      }

  });

  

  // Block until both threads have reached `wait()`.

  wg.wait();

  std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371

  ```

#### Trait Implementations

##### `impl Any for WaitGroup`

- <span id="waitgroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WaitGroup`

- <span id="waitgroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WaitGroup`

- <span id="waitgroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WaitGroup`

- <span id="waitgroup-clone"></span>`fn clone(&self) -> WaitGroup` — [`WaitGroup`](#waitgroup)

##### `impl CloneToUninit for WaitGroup`

- <span id="waitgroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WaitGroup`

- <span id="waitgroup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WaitGroup`

- <span id="waitgroup-default"></span>`fn default() -> Self`

##### `impl Drop for WaitGroup`

- <span id="waitgroup-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for WaitGroup`

- <span id="waitgroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WaitGroup`

- <span id="waitgroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for WaitGroup`

- <span id="waitgroup-toowned-type-owned"></span>`type Owned = T`

- <span id="waitgroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="waitgroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WaitGroup`

- <span id="waitgroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="waitgroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WaitGroup`

- <span id="waitgroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="waitgroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Inner`

```rust
struct Inner {
    cvar: std::sync::Condvar,
    count: std::sync::Mutex<usize>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/wait_group.rs:51-54`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/wait_group.rs#L51-L54)*

Inner state of a `WaitGroup`.

#### Trait Implementations

##### `impl Any for Inner`

- <span id="inner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inner`

- <span id="inner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inner`

- <span id="inner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Inner`

- <span id="inner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Inner`

- <span id="inner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Inner`

- <span id="inner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Inner`

- <span id="inner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

