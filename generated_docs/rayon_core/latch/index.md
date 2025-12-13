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

*Defined in [`rayon-core-1.13.0/src/latch.rs:75-77`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L75-L77)*

Spin latches are the simplest, most efficient kind, but they do
not support a `wait()` operation. They just have a boolean flag
that becomes true when `set()` is called.

#### Implementations

- <span id="corelatch-new"></span>`fn new() -> Self`

- <span id="corelatch-get-sleepy"></span>`fn get_sleepy(&self) -> bool`

  Invoked by owning thread as it prepares to sleep. Returns true

  if the owning thread may proceed to fall asleep, false if the

  latch was set in the meantime.

- <span id="corelatch-fall-asleep"></span>`fn fall_asleep(&self) -> bool`

  Invoked by owning thread as it falls asleep sleep. Returns

  true if the owning thread should block, or false if the latch

  was set in the meantime.

- <span id="corelatch-wake-up"></span>`fn wake_up(&self)`

  Invoked by owning thread as it falls asleep sleep. Returns

  true if the owning thread should block, or false if the latch

  was set in the meantime.

- <span id="corelatch-set"></span>`unsafe fn set(this: *const Self) -> bool`

  Set the latch. If this returns true, the owning thread was sleeping

  and must be awoken.

  

  This is private because, typically, setting a latch involves

  doing some wakeups; those are encapsulated in the surrounding

  latch code.

- <span id="corelatch-probe"></span>`fn probe(&self) -> bool`

  Test if this latch has been set.

#### Trait Implementations

##### `impl Any for CoreLatch`

- <span id="corelatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsCoreLatch for CoreLatch`

- <span id="corelatch-ascorelatch-as-core-latch"></span>`fn as_core_latch(&self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl<T> Borrow for CoreLatch`

- <span id="corelatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoreLatch`

- <span id="corelatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CoreLatch`

- <span id="corelatch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoreLatch`

- <span id="corelatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoreLatch`

- <span id="corelatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for CoreLatch`

- <span id="corelatch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="corelatch-pointable-type-init"></span>`type Init = T`

- <span id="corelatch-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="corelatch-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="corelatch-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="corelatch-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CoreLatch`

- <span id="corelatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="corelatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoreLatch`

- <span id="corelatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="corelatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SpinLatch<'r>`

```rust
struct SpinLatch<'r> {
    core_latch: CoreLatch,
    registry: &'r std::sync::Arc<crate::registry::Registry>,
    target_worker_index: usize,
    cross: bool,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:148-153`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L148-L153)*

Spin latches are the simplest, most efficient kind, but they do
not support a `wait()` operation. They just have a boolean flag
that becomes true when `set()` is called.

#### Implementations

- <span id="spinlatch-new"></span>`fn new(thread: &'r WorkerThread) -> SpinLatch<'r>` — [`WorkerThread`](../registry/index.md#workerthread), [`SpinLatch`](#spinlatch)

  Creates a new spin latch that is owned by `thread`. This means

  that `thread` is the only thread that should be blocking on

  this latch -- it also means that when the latch is set, we

  will wake `thread` if it is sleeping.

- <span id="spinlatch-cross"></span>`fn cross(thread: &'r WorkerThread) -> SpinLatch<'r>` — [`WorkerThread`](../registry/index.md#workerthread), [`SpinLatch`](#spinlatch)

  Creates a new spin latch for cross-thread-pool blocking.  Notably, we

  need to make sure the registry is kept alive after setting, so we can

  safely call the notification.

- <span id="spinlatch-probe"></span>`fn probe(&self) -> bool`

#### Trait Implementations

##### `impl Any for SpinLatch<'r>`

- <span id="spinlatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsCoreLatch for SpinLatch<'_>`

- <span id="spinlatch-ascorelatch-as-core-latch"></span>`fn as_core_latch(&self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl<T> Borrow for SpinLatch<'r>`

- <span id="spinlatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpinLatch<'r>`

- <span id="spinlatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SpinLatch<'r>`

- <span id="spinlatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SpinLatch<'r>`

- <span id="spinlatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Latch for SpinLatch<'_>`

- <span id="spinlatch-latch-set"></span>`unsafe fn set(this: *const Self)`

##### `impl Pointable for SpinLatch<'r>`

- <span id="spinlatch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="spinlatch-pointable-type-init"></span>`type Init = T`

- <span id="spinlatch-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="spinlatch-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="spinlatch-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="spinlatch-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SpinLatch<'r>`

- <span id="spinlatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="spinlatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpinLatch<'r>`

- <span id="spinlatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="spinlatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LockLatch`

```rust
struct LockLatch {
    m: crate::sync::Mutex<bool>,
    v: crate::sync::Condvar,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:228-231`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L228-L231)*

A Latch starts as false and eventually becomes true. You can block
until it becomes true.

#### Implementations

- <span id="locklatch-new"></span>`const fn new() -> LockLatch` — [`LockLatch`](#locklatch)

- <span id="locklatch-wait-and-reset"></span>`fn wait_and_reset(&self)`

  Block until latch is set, then resets this lock latch so it can be reused again.

- <span id="locklatch-wait"></span>`fn wait(&self)`

  Block until latch is set.

#### Trait Implementations

##### `impl Any for LockLatch`

- <span id="locklatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LockLatch`

- <span id="locklatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LockLatch`

- <span id="locklatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LockLatch`

- <span id="locklatch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LockLatch`

- <span id="locklatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LockLatch`

- <span id="locklatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Latch for LockLatch`

- <span id="locklatch-latch-set"></span>`unsafe fn set(this: *const Self)`

##### `impl Pointable for LockLatch`

- <span id="locklatch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="locklatch-pointable-type-init"></span>`type Init = T`

- <span id="locklatch-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="locklatch-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="locklatch-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="locklatch-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for LockLatch`

- <span id="locklatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="locklatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LockLatch`

- <span id="locklatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="locklatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OnceLatch`

```rust
struct OnceLatch {
    core_latch: CoreLatch,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:282-284`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L282-L284)*

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

  Set the latch, then tickle the specific worker thread,

  which should be the one that owns this latch.

#### Trait Implementations

##### `impl Any for OnceLatch`

- <span id="oncelatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsCoreLatch for OnceLatch`

- <span id="oncelatch-ascorelatch-as-core-latch"></span>`fn as_core_latch(&self) -> &CoreLatch` — [`CoreLatch`](#corelatch)

##### `impl<T> Borrow for OnceLatch`

- <span id="oncelatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceLatch`

- <span id="oncelatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for OnceLatch`

- <span id="oncelatch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OnceLatch`

- <span id="oncelatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OnceLatch`

- <span id="oncelatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for OnceLatch`

- <span id="oncelatch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="oncelatch-pointable-type-init"></span>`type Init = T`

- <span id="oncelatch-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="oncelatch-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="oncelatch-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="oncelatch-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for OnceLatch`

- <span id="oncelatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncelatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OnceLatch`

- <span id="oncelatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncelatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CountLatch`

```rust
struct CountLatch {
    counter: std::sync::atomic::AtomicUsize,
    kind: CountLatchKind,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:321-324`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L321-L324)*

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

##### `impl Any for CountLatch`

- <span id="countlatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CountLatch`

- <span id="countlatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CountLatch`

- <span id="countlatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CountLatch`

- <span id="countlatch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CountLatch`

- <span id="countlatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CountLatch`

- <span id="countlatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Latch for CountLatch`

- <span id="countlatch-latch-set"></span>`unsafe fn set(this: *const Self)`

##### `impl Pointable for CountLatch`

- <span id="countlatch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="countlatch-pointable-type-init"></span>`type Init = T`

- <span id="countlatch-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="countlatch-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="countlatch-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="countlatch-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CountLatch`

- <span id="countlatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="countlatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CountLatch`

- <span id="countlatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="countlatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LatchRef<'a, L>`

```rust
struct LatchRef<'a, L> {
    inner: *const L,
    marker: std::marker::PhantomData<&'a L>,
}
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:427-430`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L427-L430)*

`&L` without any implication of `dereferenceable` for `Latch::set`

#### Implementations

- <span id="latchref-new"></span>`fn new(inner: &L) -> LatchRef<'_, L>` — [`LatchRef`](#latchref)

#### Trait Implementations

##### `impl Any for LatchRef<'a, L>`

- <span id="latchref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LatchRef<'a, L>`

- <span id="latchref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LatchRef<'a, L>`

- <span id="latchref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<L> Deref for LatchRef<'_, L>`

- <span id="latchref-deref-type-target"></span>`type Target = L`

- <span id="latchref-deref"></span>`fn deref(&self) -> &L`

##### `impl<T> From for LatchRef<'a, L>`

- <span id="latchref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LatchRef<'a, L>`

- <span id="latchref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<L: Latch> Latch for LatchRef<'_, L>`

- <span id="latchref-latch-set"></span>`unsafe fn set(this: *const Self)`

##### `impl Pointable for LatchRef<'a, L>`

- <span id="latchref-pointable-const-align"></span>`const ALIGN: usize`

- <span id="latchref-pointable-type-init"></span>`type Init = T`

- <span id="latchref-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="latchref-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="latchref-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="latchref-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Receiver for LatchRef<'a, L>`

- <span id="latchref-receiver-type-target"></span>`type Target = T`

##### `impl<L: Sync> Sync for LatchRef<'_, L>`

##### `impl<U> TryFrom for LatchRef<'a, L>`

- <span id="latchref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="latchref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LatchRef<'a, L>`

- <span id="latchref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="latchref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-core-1.13.0/src/latch.rs:326-344`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L326-L344)*

#### Variants

- **`Stealing`**

  A latch for scopes created on a rayon thread which will participate in work
  stealing while it waits for completion. This thread is not necessarily part
  of the same registry as the scope itself!

- **`Blocking`**

  A latch for scopes created on a non-rayon thread which will block to wait.

#### Trait Implementations

##### `impl Any for CountLatchKind`

- <span id="countlatchkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CountLatchKind`

- <span id="countlatchkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CountLatchKind`

- <span id="countlatchkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CountLatchKind`

- <span id="countlatchkind-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> From for CountLatchKind`

- <span id="countlatchkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CountLatchKind`

- <span id="countlatchkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for CountLatchKind`

- <span id="countlatchkind-pointable-const-align"></span>`const ALIGN: usize`

- <span id="countlatchkind-pointable-type-init"></span>`type Init = T`

- <span id="countlatchkind-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="countlatchkind-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="countlatchkind-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="countlatchkind-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CountLatchKind`

- <span id="countlatchkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="countlatchkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CountLatchKind`

- <span id="countlatchkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="countlatchkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Latch`

```rust
trait Latch { ... }
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:35-51`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L35-L51)*

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

*Defined in [`rayon-core-1.13.0/src/latch.rs:53-55`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L53-L55)*

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

*Defined in [`rayon-core-1.13.0/src/latch.rs:58`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L58)*

Latch is not set, owning thread is awake

### `SLEEPY`
```rust
const SLEEPY: usize = 1usize;
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:62`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L62)*

Latch is not set, owning thread is going to sleep on this latch
(but has not yet fallen asleep).

### `SLEEPING`
```rust
const SLEEPING: usize = 2usize;
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:66`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L66)*

Latch is not set, owning thread is asleep on this latch and
must be awoken.

### `SET`
```rust
const SET: usize = 3usize;
```

*Defined in [`rayon-core-1.13.0/src/latch.rs:69`](../../../.source_1765521767/rayon-core-1.13.0/src/latch.rs#L69)*

Latch is set.

