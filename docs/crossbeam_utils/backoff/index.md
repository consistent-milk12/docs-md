*[crossbeam_utils](../index.md) / [backoff](index.md)*

---

# Module `backoff`

## Structs

### `Backoff`

```rust
struct Backoff {
    step: core::cell::Cell<u32>,
}
```

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

- `fn new() -> Self`

- `fn reset(self: &Self)`

- `fn spin(self: &Self)`

- `fn snooze(self: &Self)`

- `fn is_completed(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for Backoff`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Backoff`

- `fn default() -> Backoff` — [`Backoff`](#backoff)

## Constants

### `SPIN_LIMIT`

```rust
const SPIN_LIMIT: u32 = 6u32;
```

### `YIELD_LIMIT`

```rust
const YIELD_LIMIT: u32 = 10u32;
```

