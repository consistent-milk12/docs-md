# Crate `crossbeam_utils`

Miscellaneous tools for concurrent programming.

## Atomics

* [`AtomicCell`](atomic/atomic_cell/index.md), a thread-safe mutable memory location.
* [`AtomicConsume`](atomic/consume/index.md), for reading from primitive atomic types with "consume" ordering.

## Thread synchronization

* [`Parker`](sync/parker/index.md), a thread parking primitive.
* [`ShardedLock`](sync/sharded_lock/index.md), a sharded reader-writer lock with fast concurrent reads.
* [`WaitGroup`](sync/wait_group/index.md), for synchronizing the beginning or end of some computation.

## Utilities

* [`Backoff`](backoff/index.md), for exponential backoff in spin loops.
* [`CachePadded`](cache_padded/index.md), for padding and aligning a value to the length of a cache line.
* [`scope`](thread/index.md), for spawning threads that borrow local variables from the stack.







## Contents

- [Modules](#modules)
  - [`primitive`](#primitive)
  - [`atomic`](#atomic)
  - [`cache_padded`](#cache-padded)
  - [`backoff`](#backoff)
  - [`sync`](#sync)
  - [`thread`](#thread)
- [Structs](#structs)
  - [`CachePadded`](#cachepadded)
  - [`Backoff`](#backoff)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`primitive`](#primitive) | mod |  |
| [`atomic`](#atomic) | mod | Atomic types. |
| [`cache_padded`](#cache-padded) | mod |  |
| [`backoff`](#backoff) | mod |  |
| [`sync`](#sync) | mod | Thread synchronization primitives. |
| [`thread`](#thread) | mod | Threads that can borrow variables from the stack. |
| [`CachePadded`](#cachepadded) | struct |  |
| [`Backoff`](#backoff) | struct |  |

## Modules

- [`primitive`](primitive/index.md)
- [`atomic`](atomic/index.md) — Atomic types.
- [`cache_padded`](cache_padded/index.md)
- [`backoff`](backoff/index.md)
- [`sync`](sync/index.md) — Thread synchronization primitives.
- [`thread`](thread/index.md) — Threads that can borrow variables from the stack.

## Structs

### `CachePadded<T>`

```rust
struct CachePadded<T> {
    value: T,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/cache_padded.rs:148-150`](../../.source_1765210505/crossbeam-utils-0.8.21/src/cache_padded.rs#L148-L150)*

Pads and aligns a value to the length of a cache line.

In concurrent programming, sometimes it is desirable to make sure commonly accessed pieces of
data are not placed into the same cache line. Updating an atomic value invalidates the whole
cache line it belongs to, which makes the next access to the same cache line slower for other
CPU cores. Use `CachePadded` to ensure updating one piece of data doesn't invalidate other
cached data.

# Size and alignment

Cache lines are assumed to be N bytes long, depending on the architecture:

* On x86-64, aarch64, and powerpc64, N = 128.
* On arm, mips, mips64, sparc, and hexagon, N = 32.
* On m68k, N = 16.
* On s390x, N = 256.
* On all others, N = 64.

Note that N is just a reasonable guess and is not guaranteed to match the actual cache line
length of the machine the program is running on. On modern Intel architectures, spatial
prefetcher is pulling pairs of 64-byte cache lines at a time, so we pessimistically assume that
cache lines are 128 bytes long.

The size of `CachePadded<T>` is the smallest multiple of N bytes large enough to accommodate
a value of type `T`.

The alignment of `CachePadded<T>` is the maximum of N bytes and the alignment of `T`.

# Examples

Alignment and padding:

```rust
use crossbeam_utils::CachePadded;

let array = [CachePadded::new(1i8), CachePadded::new(2i8)];
let addr1 = &*array[0] as *const i8 as usize;
let addr2 = &*array[1] as *const i8 as usize;

assert!(addr2 - addr1 >= 32);
assert_eq!(addr1 % 32, 0);
assert_eq!(addr2 % 32, 0);
```

When building a concurrent queue with a head and a tail index, it is wise to place them in
different cache lines so that concurrent threads pushing and popping elements don't invalidate
each other's cache lines:

```rust
use crossbeam_utils::CachePadded;
use std::sync::atomic::AtomicUsize;

struct Queue<T> {
    head: CachePadded<AtomicUsize>,
    tail: CachePadded<AtomicUsize>,
    buffer: *mut T,
}
```

#### Implementations

- <span id="cachepadded-new"></span>`const fn new(t: T) -> CachePadded<T>` — [`CachePadded`](cache_padded/index.md#cachepadded)

- <span id="cachepadded-into-inner"></span>`fn into_inner(self) -> T`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for CachePadded<T>`

- <span id="cachepadded-clone"></span>`fn clone(&self) -> CachePadded<T>` — [`CachePadded`](cache_padded/index.md#cachepadded)

##### `impl<T: marker::Copy> Copy for CachePadded<T>`

##### `impl<T: fmt::Debug> Debug for CachePadded<T>`

- <span id="cachepadded-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for CachePadded<T>`

- <span id="cachepadded-default"></span>`fn default() -> CachePadded<T>` — [`CachePadded`](cache_padded/index.md#cachepadded)

##### `impl<T> Deref for CachePadded<T>`

- <span id="cachepadded-deref-type-target"></span>`type Target = T`

- <span id="cachepadded-deref"></span>`fn deref(&self) -> &T`

##### `impl<T> DerefMut for CachePadded<T>`

- <span id="cachepadded-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Display> Display for CachePadded<T>`

- <span id="cachepadded-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for CachePadded<T>`

##### `impl<T: hash::Hash> Hash for CachePadded<T>`

- <span id="cachepadded-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::PartialEq> PartialEq for CachePadded<T>`

- <span id="cachepadded-eq"></span>`fn eq(&self, other: &CachePadded<T>) -> bool` — [`CachePadded`](cache_padded/index.md#cachepadded)

##### `impl<T> Receiver for CachePadded<T>`

- <span id="cachepadded-receiver-type-target"></span>`type Target = T`

##### `impl<T: Send> Send for CachePadded<T>`

##### `impl<T> StructuralPartialEq for CachePadded<T>`

##### `impl<T: Sync> Sync for CachePadded<T>`

##### `impl<T> ToString for CachePadded<T>`

- <span id="cachepadded-to-string"></span>`fn to_string(&self) -> String`

### `Backoff`

```rust
struct Backoff {
    step: core::cell::Cell<u32>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/backoff.rs:80-82`](../../.source_1765210505/crossbeam-utils-0.8.21/src/backoff.rs#L80-L82)*

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

- <span id="backoff-reset"></span>`fn reset(&self)`

- <span id="backoff-spin"></span>`fn spin(&self)`

- <span id="backoff-snooze"></span>`fn snooze(&self)`

- <span id="backoff-is-completed"></span>`fn is_completed(&self) -> bool`

#### Trait Implementations

##### `impl Debug for Backoff`

- <span id="backoff-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Backoff`

- <span id="backoff-default"></span>`fn default() -> Backoff` — [`Backoff`](backoff/index.md#backoff)

