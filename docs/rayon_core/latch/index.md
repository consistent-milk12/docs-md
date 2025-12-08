*[rayon_core](../index.md) / [latch](index.md)*

---

# Module `latch`

## Structs

### `CoreLatch`

```rust
struct CoreLatch {
    state: std::sync::atomic::AtomicUsize,
}
```

Spin latches are the simplest, most efficient kind, but they do
not support a `wait()` operation. They just have a boolean flag
that becomes true when `set()` is called.

#### Implementations

- `fn new() -> Self`

- `fn get_sleepy(self: &Self) -> bool`

- `fn fall_asleep(self: &Self) -> bool`

- `fn wake_up(self: &Self)`

- `unsafe fn set(this: *const Self) -> bool`

- `fn probe(self: &Self) -> bool`

#### Trait Implementations

##### `impl AsCoreLatch for CoreLatch`

- `fn as_core_latch(self: &Self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl Debug for CoreLatch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Pointable for CoreLatch`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SpinLatch<'r>`

```rust
struct SpinLatch<'r> {
    core_latch: CoreLatch,
    registry: &'r std::sync::Arc<crate::registry::Registry>,
    target_worker_index: usize,
    cross: bool,
}
```

Spin latches are the simplest, most efficient kind, but they do
not support a `wait()` operation. They just have a boolean flag
that becomes true when `set()` is called.

#### Implementations

- `fn new(thread: &'r WorkerThread) -> SpinLatch<'r>` — [`WorkerThread`](../registry/index.md), [`SpinLatch`](#spinlatch)

- `fn cross(thread: &'r WorkerThread) -> SpinLatch<'r>` — [`WorkerThread`](../registry/index.md), [`SpinLatch`](#spinlatch)

- `fn probe(self: &Self) -> bool`

#### Trait Implementations

##### `impl AsCoreLatch for SpinLatch<'_>`

- `fn as_core_latch(self: &Self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl Latch for SpinLatch<'_>`

- `unsafe fn set(this: *const Self)`

##### `impl<T> Pointable for SpinLatch<'r>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `LockLatch`

```rust
struct LockLatch {
    m: crate::sync::Mutex<bool>,
    v: crate::sync::Condvar,
}
```

A Latch starts as false and eventually becomes true. You can block
until it becomes true.

#### Implementations

- `const fn new() -> LockLatch` — [`LockLatch`](#locklatch)

- `fn wait_and_reset(self: &Self)`

- `fn wait(self: &Self)`

#### Trait Implementations

##### `impl Debug for LockLatch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Latch for LockLatch`

- `unsafe fn set(this: *const Self)`

##### `impl<T> Pointable for LockLatch`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `OnceLatch`

```rust
struct OnceLatch {
    core_latch: CoreLatch,
}
```

Once latches are used to implement one-time blocking, primarily
for the termination flag of the threads in the pool.

Note: like a `SpinLatch`, once-latches are always associated with
some registry that is probing them, which must be tickled when
they are set. *Unlike* a `SpinLatch`, they don't themselves hold a
reference to that registry. This is because in some cases the
registry owns the once-latch, and that would create a cycle. So a
`OnceLatch` must be given a reference to its owning registry when
it is set. For this reason, it does not implement the `Latch`
trait (but it doesn't have to, as it is not used in those generic
contexts).

#### Implementations

- `fn new() -> OnceLatch` — [`OnceLatch`](#oncelatch)

- `unsafe fn set_and_tickle_one(this: *const Self, registry: &Registry, target_worker_index: usize)` — [`Registry`](../registry/index.md)

#### Trait Implementations

##### `impl AsCoreLatch for OnceLatch`

- `fn as_core_latch(self: &Self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl Debug for OnceLatch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Pointable for OnceLatch`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `CountLatch`

```rust
struct CountLatch {
    counter: std::sync::atomic::AtomicUsize,
    kind: CountLatchKind,
}
```

Counting latches are used to implement scopes. They track a
counter. Unlike other latches, calling `set()` does not
necessarily make the latch be considered `set()`; instead, it just
decrements the counter. The latch is only "set" (in the sense that
`probe()` returns true) once the counter reaches zero.

#### Implementations

- `fn new(owner: Option<&WorkerThread>) -> Self` — [`WorkerThread`](../registry/index.md)

- `fn with_count(count: usize, owner: Option<&WorkerThread>) -> Self` — [`WorkerThread`](../registry/index.md)

- `fn increment(self: &Self)`

- `fn wait(self: &Self, owner: Option<&WorkerThread>)` — [`WorkerThread`](../registry/index.md)

#### Trait Implementations

##### `impl Debug for CountLatch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Latch for CountLatch`

- `unsafe fn set(this: *const Self)`

##### `impl<T> Pointable for CountLatch`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `LatchRef<'a, L>`

```rust
struct LatchRef<'a, L> {
    inner: *const L,
    marker: std::marker::PhantomData<&'a L>,
}
```

`&L` without any implication of `dereferenceable` for `Latch::set`

#### Implementations

- `fn new(inner: &L) -> LatchRef<'_, L>` — [`LatchRef`](#latchref)

#### Trait Implementations

##### `impl<L> Deref for LatchRef<'_, L>`

- `type Target = L`

- `fn deref(self: &Self) -> &L`

##### `impl<L: Latch> Latch for LatchRef<'_, L>`

- `unsafe fn set(this: *const Self)`

##### `impl<T> Pointable for LatchRef<'a, L>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P, T> Receiver for LatchRef<'a, L>`

- `type Target = T`

##### `impl<L: Sync> Sync for LatchRef<'_, L>`

## Enums

### `CountLatchKind`

```rust
enum CountLatchKind {
    Stealing {
        latch: CoreLatch,
        registry: std::sync::Arc<crate::registry::Registry>,
        worker_index: usize,
    },
    Blocking {
        latch: LockLatch,
    },
}
```

#### Variants

- **`Stealing`**

  A latch for scopes created on a rayon thread which will participate in work
  stealing while it waits for completion. This thread is not necessarily part
  of the same registry as the scope itself!

- **`Blocking`**

  A latch for scopes created on a non-rayon thread which will block to wait.

#### Trait Implementations

##### `impl Debug for CountLatchKind`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> Pointable for CountLatchKind`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `Latch`

```rust
trait Latch { ... }
```

We define various kinds of latches, which are all a primitive signaling
mechanism. A latch starts as false. Eventually someone calls `set()` and
it becomes true. You can test if it has been set by calling `probe()`.

Some kinds of latches, but not all, support a `wait()` operation
that will wait until the latch is set, blocking efficiently. That
is not part of the trait since it is not possibly to do with all
latches.

The intention is that `set()` is called once, but `probe()` may be
called any number of times. Once `probe()` returns true, the memory
effects that occurred before `set()` become visible.

It'd probably be better to refactor the API into two paired types,
but that's a bit of work, and this is not a public API.

## Memory ordering

Latches need to guarantee two things:

- Once `probe()` returns true, all memory effects from the `set()`
  are visible (in other words, the set should synchronize-with
  the probe).
- Once `set()` occurs, the next `probe()` *will* observe it.  This
  typically requires a seq-cst ordering. See [the "tickle-then-get-sleepy" scenario in the sleep
  README](/src/sleep/README.md#tickle-then-get-sleepy) for details.

#### Required Methods

- `fn set(this: *const Self)`

  Set the latch, signalling others.

### `AsCoreLatch`

```rust
trait AsCoreLatch { ... }
```

#### Required Methods

- `fn as_core_latch(self: &Self) -> &CoreLatch`

## Constants

### `UNSET`

```rust
const UNSET: usize = 0usize;
```

Latch is not set, owning thread is awake

### `SLEEPY`

```rust
const SLEEPY: usize = 1usize;
```

Latch is not set, owning thread is going to sleep on this latch
(but has not yet fallen asleep).

### `SLEEPING`

```rust
const SLEEPING: usize = 2usize;
```

Latch is not set, owning thread is asleep on this latch and
must be awoken.

### `SET`

```rust
const SET: usize = 3usize;
```

Latch is set.

