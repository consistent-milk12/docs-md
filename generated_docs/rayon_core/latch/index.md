*[rayon_core](../index.md) / [latch](index.md)*

---

# Module `latch`

## Contents

- [Structs](#structs)
  - [`CoreLatch`](#corelatch)
  - [`SpinLatch`](#spinlatch)
  - [`LockLatch`](#locklatch)
  - [`OnceLatch`](#oncelatch)
  - [`CountLatch`](#countlatch)
  - [`LatchRef`](#latchref)
- [Enums](#enums)
  - [`CountLatchKind`](#countlatchkind)
- [Traits](#traits)
  - [`Latch`](#latch)
  - [`AsCoreLatch`](#ascorelatch)
- [Constants](#constants)
  - [`UNSET`](#unset)
  - [`SLEEPY`](#sleepy)
  - [`SLEEPING`](#sleeping)
  - [`SET`](#set)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CoreLatch`](#corelatch) | struct | Spin latches are the simplest, most efficient kind, but they do not support a `wait()` operation. |
| [`SpinLatch`](#spinlatch) | struct | Spin latches are the simplest, most efficient kind, but they do not support a `wait()` operation. |
| [`LockLatch`](#locklatch) | struct | A Latch starts as false and eventually becomes true. |
| [`OnceLatch`](#oncelatch) | struct | Once latches are used to implement one-time blocking, primarily for the termination flag of the threads in the pool. |
| [`CountLatch`](#countlatch) | struct | Counting latches are used to implement scopes. |
| [`LatchRef`](#latchref) | struct | `&L` without any implication of `dereferenceable` for `Latch::set` |
| [`CountLatchKind`](#countlatchkind) | enum |  |
| [`Latch`](#latch) | trait | We define various kinds of latches, which are all a primitive signaling mechanism. |
| [`AsCoreLatch`](#ascorelatch) | trait |  |
| [`UNSET`](#unset) | const | Latch is not set, owning thread is awake |
| [`SLEEPY`](#sleepy) | const | Latch is not set, owning thread is going to sleep on this latch (but has not yet fallen asleep). |
| [`SLEEPING`](#sleeping) | const | Latch is not set, owning thread is asleep on this latch and must be awoken. |
| [`SET`](#set) | const | Latch is set. |

## Structs

### `CoreLatch`

```rust
struct CoreLatch {
    state: std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:75-77`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L75-L77)*

Spin latches are the simplest, most efficient kind, but they do
not support a `wait()` operation. They just have a boolean flag
that becomes true when `set()` is called.

#### Implementations

- <span id="corelatch-new"></span>`fn new() -> Self`

- <span id="corelatch-get-sleepy"></span>`fn get_sleepy(&self) -> bool`

- <span id="corelatch-fall-asleep"></span>`fn fall_asleep(&self) -> bool`

- <span id="corelatch-wake-up"></span>`fn wake_up(&self)`

- <span id="corelatch-set"></span>`unsafe fn set(this: *const Self) -> bool`

- <span id="corelatch-probe"></span>`fn probe(&self) -> bool`

#### Trait Implementations

##### `impl AsCoreLatch for CoreLatch`

- <span id="corelatch-as-core-latch"></span>`fn as_core_latch(&self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl Debug for CoreLatch`

- <span id="corelatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for CoreLatch`

- <span id="corelatch-const-align"></span>`const ALIGN: usize`

- <span id="corelatch-type-init"></span>`type Init = T`

- <span id="corelatch-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="corelatch-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="corelatch-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="corelatch-drop"></span>`unsafe fn drop(ptr: usize)`

### `SpinLatch<'r>`

```rust
struct SpinLatch<'r> {
    core_latch: CoreLatch,
    registry: &'r std::sync::Arc<crate::registry::Registry>,
    target_worker_index: usize,
    cross: bool,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:148-153`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L148-L153)*

Spin latches are the simplest, most efficient kind, but they do
not support a `wait()` operation. They just have a boolean flag
that becomes true when `set()` is called.

#### Implementations

- <span id="spinlatch-new"></span>`fn new(thread: &'r WorkerThread) -> SpinLatch<'r>` — [`WorkerThread`](../registry/index.md#workerthread), [`SpinLatch`](#spinlatch)

- <span id="spinlatch-cross"></span>`fn cross(thread: &'r WorkerThread) -> SpinLatch<'r>` — [`WorkerThread`](../registry/index.md#workerthread), [`SpinLatch`](#spinlatch)

- <span id="spinlatch-probe"></span>`fn probe(&self) -> bool`

#### Trait Implementations

##### `impl AsCoreLatch for SpinLatch<'_>`

- <span id="spinlatch-as-core-latch"></span>`fn as_core_latch(&self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl Latch for SpinLatch<'_>`

- <span id="spinlatch-set"></span>`unsafe fn set(this: *const Self)`

##### `impl Pointable for SpinLatch<'r>`

- <span id="spinlatch-const-align"></span>`const ALIGN: usize`

- <span id="spinlatch-type-init"></span>`type Init = T`

- <span id="spinlatch-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="spinlatch-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="spinlatch-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="spinlatch-drop"></span>`unsafe fn drop(ptr: usize)`

### `LockLatch`

```rust
struct LockLatch {
    m: crate::sync::Mutex<bool>,
    v: crate::sync::Condvar,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:228-231`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L228-L231)*

A Latch starts as false and eventually becomes true. You can block
until it becomes true.

#### Implementations

- <span id="locklatch-new"></span>`const fn new() -> LockLatch` — [`LockLatch`](#locklatch)

- <span id="locklatch-wait-and-reset"></span>`fn wait_and_reset(&self)`

- <span id="locklatch-wait"></span>`fn wait(&self)`

#### Trait Implementations

##### `impl Debug for LockLatch`

- <span id="locklatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Latch for LockLatch`

- <span id="locklatch-set"></span>`unsafe fn set(this: *const Self)`

##### `impl Pointable for LockLatch`

- <span id="locklatch-const-align"></span>`const ALIGN: usize`

- <span id="locklatch-type-init"></span>`type Init = T`

- <span id="locklatch-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="locklatch-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="locklatch-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="locklatch-drop"></span>`unsafe fn drop(ptr: usize)`

### `OnceLatch`

```rust
struct OnceLatch {
    core_latch: CoreLatch,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:282-284`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L282-L284)*

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

- <span id="oncelatch-new"></span>`fn new() -> OnceLatch` — [`OnceLatch`](#oncelatch)

- <span id="oncelatch-set-and-tickle-one"></span>`unsafe fn set_and_tickle_one(this: *const Self, registry: &Registry, target_worker_index: usize)` — [`Registry`](../registry/index.md#registry)

#### Trait Implementations

##### `impl AsCoreLatch for OnceLatch`

- <span id="oncelatch-as-core-latch"></span>`fn as_core_latch(&self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl Debug for OnceLatch`

- <span id="oncelatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for OnceLatch`

- <span id="oncelatch-const-align"></span>`const ALIGN: usize`

- <span id="oncelatch-type-init"></span>`type Init = T`

- <span id="oncelatch-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="oncelatch-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="oncelatch-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="oncelatch-drop"></span>`unsafe fn drop(ptr: usize)`

### `CountLatch`

```rust
struct CountLatch {
    counter: std::sync::atomic::AtomicUsize,
    kind: CountLatchKind,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:321-324`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L321-L324)*

Counting latches are used to implement scopes. They track a
counter. Unlike other latches, calling `set()` does not
necessarily make the latch be considered `set()`; instead, it just
decrements the counter. The latch is only "set" (in the sense that
`probe()` returns true) once the counter reaches zero.

#### Implementations

- <span id="countlatch-new"></span>`fn new(owner: Option<&WorkerThread>) -> Self` — [`WorkerThread`](../registry/index.md#workerthread)

- <span id="countlatch-with-count"></span>`fn with_count(count: usize, owner: Option<&WorkerThread>) -> Self` — [`WorkerThread`](../registry/index.md#workerthread)

- <span id="countlatch-increment"></span>`fn increment(&self)`

- <span id="countlatch-wait"></span>`fn wait(&self, owner: Option<&WorkerThread>)` — [`WorkerThread`](../registry/index.md#workerthread)

#### Trait Implementations

##### `impl Debug for CountLatch`

- <span id="countlatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Latch for CountLatch`

- <span id="countlatch-set"></span>`unsafe fn set(this: *const Self)`

##### `impl Pointable for CountLatch`

- <span id="countlatch-const-align"></span>`const ALIGN: usize`

- <span id="countlatch-type-init"></span>`type Init = T`

- <span id="countlatch-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="countlatch-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="countlatch-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="countlatch-drop"></span>`unsafe fn drop(ptr: usize)`

### `LatchRef<'a, L>`

```rust
struct LatchRef<'a, L> {
    inner: *const L,
    marker: std::marker::PhantomData<&'a L>,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:427-430`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L427-L430)*

`&L` without any implication of `dereferenceable` for `Latch::set`

#### Implementations

- <span id="latchref-new"></span>`fn new(inner: &L) -> LatchRef<'_, L>` — [`LatchRef`](#latchref)

#### Trait Implementations

##### `impl<L> Deref for LatchRef<'_, L>`

- <span id="latchref-type-target"></span>`type Target = L`

- <span id="latchref-deref"></span>`fn deref(&self) -> &L`

##### `impl<L: Latch> Latch for LatchRef<'_, L>`

- <span id="latchref-set"></span>`unsafe fn set(this: *const Self)`

##### `impl<T> Pointable for LatchRef<'a, L>`

- <span id="latchref-const-align"></span>`const ALIGN: usize`

- <span id="latchref-type-init"></span>`type Init = T`

- <span id="latchref-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="latchref-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="latchref-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="latchref-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, T> Receiver for LatchRef<'a, L>`

- <span id="latchref-type-target"></span>`type Target = T`

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

*Defined in [`rayon-core-1.13.0/src/latch.rs:326-344`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L326-L344)*

#### Variants

- **`Stealing`**

  A latch for scopes created on a rayon thread which will participate in work
  stealing while it waits for completion. This thread is not necessarily part
  of the same registry as the scope itself!

- **`Blocking`**

  A latch for scopes created on a non-rayon thread which will block to wait.

#### Trait Implementations

##### `impl Debug for CountLatchKind`

- <span id="countlatchkind-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Pointable for CountLatchKind`

- <span id="countlatchkind-const-align"></span>`const ALIGN: usize`

- <span id="countlatchkind-type-init"></span>`type Init = T`

- <span id="countlatchkind-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="countlatchkind-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="countlatchkind-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="countlatchkind-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `Latch`

```rust
trait Latch { ... }
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:35-51`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L35-L51)*

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

#### Implementors

- [`CountLatch`](#countlatch)
- [`LatchRef`](#latchref)
- [`LockLatch`](#locklatch)
- [`SpinLatch`](#spinlatch)

### `AsCoreLatch`

```rust
trait AsCoreLatch { ... }
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:53-55`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L53-L55)*

#### Required Methods

- `fn as_core_latch(&self) -> &CoreLatch`

#### Implementors

- [`CoreLatch`](#corelatch)
- [`OnceLatch`](#oncelatch)
- [`SpinLatch`](#spinlatch)

## Constants

### `UNSET`
```rust
const UNSET: usize = 0usize;
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:58`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L58)*

Latch is not set, owning thread is awake

### `SLEEPY`
```rust
const SLEEPY: usize = 1usize;
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:62`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L62)*

Latch is not set, owning thread is going to sleep on this latch
(but has not yet fallen asleep).

### `SLEEPING`
```rust
const SLEEPING: usize = 2usize;
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:66`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L66)*

Latch is not set, owning thread is asleep on this latch and
must be awoken.

### `SET`
```rust
const SET: usize = 3usize;
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:69`](../../../.source_1765210505/rayon-core-1.13.0/src/latch.rs#L69)*

Latch is set.

