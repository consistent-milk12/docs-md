*[crossbeam_utils](../index.md) / [backoff](index.md)*

---

# Module `backoff`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Backoff`](#backoff) | struct | Performs exponential backoff in spin loops. |
| [`SPIN_LIMIT`](#spin-limit) | const |  |
| [`YIELD_LIMIT`](#yield-limit) | const |  |

## Structs

### `Backoff`

```rust
struct Backoff {
    step: core::cell::Cell<u32>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/backoff.rs:80-82`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/backoff.rs#L80-L82)*

Performs exponential backoff in spin loops.

Backing off in spin loops reduces contention and improves overall performance.

This primitive can execute *YIELD* and *PAUSE* instructions, yield the current thread to the OS
scheduler, and tell when is a good time to block the thread using a different synchronization
mechanism. Each step of the back off procedure takes roughly twice as long as the previous
step.

# Examples

Backing off in a lock-free loop:

```rust
use crossbeam_utils::Backoff;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

fn fetch_mul(a: &AtomicUsize, b: usize) -> usize {
    let backoff = Backoff::new();
    loop {
        let val = a.load(SeqCst);
        if a.compare_exchange(val, val.wrapping_mul(b), SeqCst, SeqCst).is_ok() {
            return val;
        }
        backoff.spin();
    }
}
```

Waiting for an [`AtomicBool`](#atomicbool) to become `true`:

```rust
use crossbeam_utils::Backoff;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

fn spin_wait(ready: &AtomicBool) {
    let backoff = Backoff::new();
    while !ready.load(SeqCst) {
        backoff.snooze();
    }
}
```

Waiting for an [`AtomicBool`](#atomicbool) to become `true` and parking the thread after a long wait.
Note that whoever sets the atomic variable to `true` must notify the parked thread by calling

use crossbeam_utils::Backoff;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

fn blocking_wait(ready: &AtomicBool) {
    let backoff = Backoff::new();
    while !ready.load(SeqCst) {
        if backoff.is_completed() {
            thread::park();
        } else {
            backoff.snooze();
        }
    }
}
```rust






#### Implementations

- <span id="backoff-new"></span>`fn new() -> Self`

  Creates a new `Backoff`.

  

  # Examples

  

  ```rust

  use crossbeam_utils::Backoff;

  

  let backoff = Backoff::new();

  ```

- <span id="backoff-reset"></span>`fn reset(&self)`

  Resets the `Backoff`.

  

  # Examples

  

  ```rust

  use crossbeam_utils::Backoff;

  

  let backoff = Backoff::new();

  backoff.reset();

  ```

- <span id="backoff-spin"></span>`fn spin(&self)`

  Backs off in a lock-free loop.

  

  This method should be used when we need to retry an operation because another thread made

  progress.

  

  The processor may yield using the *YIELD* or *PAUSE* instruction.

  

  # Examples

  

  Backing off in a lock-free loop:

  

  ```rust

  use crossbeam_utils::Backoff;

  use std::sync::atomic::AtomicUsize;

  use std::sync::atomic::Ordering::SeqCst;

  

  fn fetch_mul(a: &AtomicUsize, b: usize) -> usize {

      let backoff = Backoff::new();

      loop {

          let val = a.load(SeqCst);

          if a.compare_exchange(val, val.wrapping_mul(b), SeqCst, SeqCst).is_ok() {

              return val;

          }

          backoff.spin();

      }

  }

  

  let a = AtomicUsize::new(7);

  assert_eq!(fetch_mul(&a, 8), 7);

  assert_eq!(a.load(SeqCst), 56);

  ```

- <span id="backoff-snooze"></span>`fn snooze(&self)`

  Backs off in a blocking loop.

  

  This method should be used when we need to wait for another thread to make progress.

  

  The processor may yield using the *YIELD* or *PAUSE* instruction and the current thread

  may yield by giving up a timeslice to the OS scheduler.

  

  In `#[no_std]` environments, this method is equivalent to `spin`.

  

  If possible, use `is_completed` to check when it is advised to stop using backoff and

  block the current thread using a different synchronization mechanism instead.

  

  

  # Examples

  

  Waiting for an [`AtomicBool`](#atomicbool) to become `true`:

  

  ```rust

  use crossbeam_utils::Backoff;

  use std::sync::Arc;

  use std::sync::atomic::AtomicBool;

  use std::sync::atomic::Ordering::SeqCst;

  use std::thread;

  use std::time::Duration;

  

  fn spin_wait(ready: &AtomicBool) {

      let backoff = Backoff::new();

      while !ready.load(SeqCst) {

          backoff.snooze();

      }

  }

  

  let ready = Arc::new(AtomicBool::new(false));

  let ready2 = ready.clone();

  

  thread::spawn(move || {

      thread::sleep(Duration::from_millis(100));

      ready2.store(true, SeqCst);

  });

  

  assert_eq!(ready.load(SeqCst), false);

  spin_wait(&ready);

  assert_eq!(ready.load(SeqCst), true);

  std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371

  ```

- <span id="backoff-is-completed"></span>`fn is_completed(&self) -> bool`

  Returns `true` if exponential backoff has completed and blocking the thread is advised.

  

  # Examples

  

  Waiting for an [`AtomicBool`](#atomicbool) to become `true` and parking the thread after a long wait:

  

  ```rust

  use crossbeam_utils::Backoff;

  use std::sync::Arc;

  use std::sync::atomic::AtomicBool;

  use std::sync::atomic::Ordering::SeqCst;

  use std::thread;

  use std::time::Duration;

  

  fn blocking_wait(ready: &AtomicBool) {

      let backoff = Backoff::new();

      while !ready.load(SeqCst) {

          if backoff.is_completed() {

              thread::park();

          } else {

              backoff.snooze();

          }

      }

  }

  

  let ready = Arc::new(AtomicBool::new(false));

  let ready2 = ready.clone();

  let waiter = thread::current();

  

  thread::spawn(move || {

      thread::sleep(Duration::from_millis(100));

      ready2.store(true, SeqCst);

      waiter.unpark();

  });

  

  assert_eq!(ready.load(SeqCst), false);

  blocking_wait(&ready);

  assert_eq!(ready.load(SeqCst), true);

  std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371

  ```

#### Trait Implementations

##### `impl Any for Backoff`

- <span id="backoff-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Backoff`

- <span id="backoff-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Backoff`

- <span id="backoff-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Backoff`

- <span id="backoff-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Backoff`

- <span id="backoff-default"></span>`fn default() -> Backoff` — [`Backoff`](#backoff)

##### `impl<T> From for Backoff`

- <span id="backoff-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Backoff`

- <span id="backoff-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Backoff`

- <span id="backoff-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="backoff-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Backoff`

- <span id="backoff-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="backoff-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `SPIN_LIMIT`
```rust
const SPIN_LIMIT: u32 = 6u32;
```

*Defined in [`crossbeam-utils-0.8.21/src/backoff.rs:5`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/backoff.rs#L5)*

### `YIELD_LIMIT`
```rust
const YIELD_LIMIT: u32 = 10u32;
```

*Defined in [`crossbeam-utils-0.8.21/src/backoff.rs:6`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/backoff.rs#L6)*

