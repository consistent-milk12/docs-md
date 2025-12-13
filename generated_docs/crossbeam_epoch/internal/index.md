*[crossbeam_epoch](../index.md) / [internal](index.md)*

---

# Module `internal`

The global data and participant for garbage collection.

# Registration

In order to track all participants in one place, we need some form of participant
registration. When a participant is created, it is registered to a global lock-free
singly-linked list of registries; and when a participant is leaving, it is unregistered from the
list.

# Pinning

Every participant contains an integer that tells whether the participant is pinned and if so,
what was the global epoch at the time it was pinned. Participants also hold a pin counter that
aids in periodic global epoch advancement.

When a participant is pinned, a `Guard` is returned as a witness that the participant is pinned.
Guards are necessary for performing atomic operations, and for freeing/dropping locations.

# Thread-local bag

Objects that get unlinked from concurrent data structures must be stashed away until the global
epoch sufficiently advances so that they become safe for destruction. Pointers to such objects
are pushed into a thread-local bag, and when it becomes full, the bag is marked with the current
global epoch and pushed into the global queue of bags. We store objects in thread-local storages
for amortizing the synchronization cost of pushing the garbages to a global queue.

# Global queue

Whenever a bag is pushed into a queue, the objects in some bags in the queue are collected and
destroyed along the way. This design reduces contention on data structures. The global queue
cannot be explicitly accessed: the only way to interact with it is by calling functions
`defer()` that adds an object to the thread-local bag, or `collect()` that manually triggers
garbage collection.

Ideally each instance of concurrent data structure may have its own queue that gets fully
destroyed as soon as the data structure gets dropped.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bag`](#bag) | struct | A bag of deferred functions. |
| [`SealedBag`](#sealedbag) | struct | A pair of an epoch and a bag. |
| [`Global`](#global) | struct | The global data for a garbage collector. |
| [`Local`](#local) | struct | Participant for garbage collection. |
| [`MAX_OBJECTS`](#max-objects) | const | Maximum number of objects a bag can contain. |

## Structs

### `Bag`

```rust
struct Bag {
    deferreds: [crate::deferred::Deferred; 64],
    len: usize,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:63-67`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/internal.rs#L63-L67)*

A bag of deferred functions.

#### Fields

- **`deferreds`**: `[crate::deferred::Deferred; 64]`

  Stashed objects.

#### Implementations

- <span id="bag-new"></span>`fn new() -> Self`

  Returns a new, empty bag.

- <span id="bag-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the bag is empty.

- <span id="bag-try-push"></span>`unsafe fn try_push(&mut self, deferred: Deferred) -> Result<(), Deferred>` — [`Deferred`](../deferred/index.md#deferred)

  Attempts to insert a deferred function into the bag.

  

  Returns `Ok(())` if successful, and `Err(deferred)` for the given `deferred` if the bag is

  full.

  

  # Safety

  

  It should be safe for another thread to execute the given function.

- <span id="bag-seal"></span>`fn seal(self, epoch: Epoch) -> SealedBag` — [`Epoch`](../epoch/index.md#epoch), [`SealedBag`](#sealedbag)

  Seals the bag with the given epoch.

#### Trait Implementations

##### `impl Any for Bag`

- <span id="bag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bag`

- <span id="bag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bag`

- <span id="bag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Bag`

- <span id="bag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bag`

- <span id="bag-default"></span>`fn default() -> Self`

##### `impl Drop for Bag`

- <span id="bag-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Bag`

- <span id="bag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bag`

- <span id="bag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Bag`

- <span id="bag-pointable-const-align"></span>`const ALIGN: usize`

- <span id="bag-pointable-type-init"></span>`type Init = T`

- <span id="bag-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="bag-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="bag-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="bag-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Send for Bag`

##### `impl<U> TryFrom for Bag`

- <span id="bag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bag`

- <span id="bag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SealedBag`

```rust
struct SealedBag {
    epoch: crate::epoch::Epoch,
    _bag: Bag,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:138-141`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/internal.rs#L138-L141)*

A pair of an epoch and a bag.

#### Implementations

- <span id="sealedbag-is-expired"></span>`fn is_expired(&self, global_epoch: Epoch) -> bool` — [`Epoch`](../epoch/index.md#epoch)

  Checks if it is safe to drop the bag w.r.t. the given global epoch.

#### Trait Implementations

##### `impl Any for SealedBag`

- <span id="sealedbag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SealedBag`

- <span id="sealedbag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SealedBag`

- <span id="sealedbag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SealedBag`

- <span id="sealedbag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SealedBag`

- <span id="sealedbag-default"></span>`fn default() -> SealedBag` — [`SealedBag`](#sealedbag)

##### `impl<T> From for SealedBag`

- <span id="sealedbag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SealedBag`

- <span id="sealedbag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for SealedBag`

- <span id="sealedbag-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sealedbag-pointable-type-init"></span>`type Init = T`

- <span id="sealedbag-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="sealedbag-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sealedbag-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sealedbag-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Sync for SealedBag`

##### `impl<U> TryFrom for SealedBag`

- <span id="sealedbag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sealedbag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SealedBag`

- <span id="sealedbag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sealedbag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Global`

```rust
struct Global {
    locals: crate::sync::list::List<Local>,
    queue: crate::sync::queue::Queue<SealedBag>,
    epoch: crossbeam_utils::CachePadded<crate::epoch::AtomicEpoch>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:156-165`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/internal.rs#L156-L165)*

The global data for a garbage collector.

#### Fields

- **`locals`**: `crate::sync::list::List<Local>`

  The intrusive linked list of `Local`s.

- **`queue`**: `crate::sync::queue::Queue<SealedBag>`

  The global queue of bags of deferred functions.

- **`epoch`**: `crossbeam_utils::CachePadded<crate::epoch::AtomicEpoch>`

  The global epoch.

#### Implementations

- <span id="global-const-collect-steps"></span>`const COLLECT_STEPS: usize`

- <span id="global-new"></span>`fn new() -> Self`

  Creates a new global data for garbage collection.

- <span id="global-push-bag"></span>`fn push_bag(&self, bag: &mut Bag, guard: &Guard)` — [`Bag`](#bag), [`Guard`](../guard/index.md#guard)

  Pushes the bag into the global queue and replaces the bag with a new empty bag.

- <span id="global-collect"></span>`fn collect(&self, guard: &Guard)` — [`Guard`](../guard/index.md#guard)

  Collects several bags from the global queue and executes deferred functions in them.

  

  Note: This may itself produce garbage and in turn allocate new bags.

  

  `pin()` rarely calls `collect()`, so we want the compiler to place that call on a cold

  path. In other words, we want the compiler to optimize branching for the case when

  `collect()` is not called.

- <span id="global-try-advance"></span>`fn try_advance(&self, guard: &Guard) -> Epoch` — [`Guard`](../guard/index.md#guard), [`Epoch`](../epoch/index.md#epoch)

  Attempts to advance the global epoch.

  

  The global epoch can advance only if all currently pinned participants have been pinned in

  the current epoch.

  

  Returns the current global epoch.

  

  `try_advance()` is annotated `#[cold]` because it is rarely called.

#### Trait Implementations

##### `impl Any for Global`

- <span id="global-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Global`

- <span id="global-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Global`

- <span id="global-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Global`

- <span id="global-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Global`

- <span id="global-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Global`

- <span id="global-pointable-const-align"></span>`const ALIGN: usize`

- <span id="global-pointable-type-init"></span>`type Init = T`

- <span id="global-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="global-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="global-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="global-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Global`

- <span id="global-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="global-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Global`

- <span id="global-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="global-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Local`

```rust
struct Local {
    entry: crate::sync::list::Entry,
    collector: crate::primitive::cell::UnsafeCell<core::mem::ManuallyDrop<crate::collector::Collector>>,
    bag: crate::primitive::cell::UnsafeCell<Bag>,
    guard_count: core::cell::Cell<usize>,
    handle_count: core::cell::Cell<usize>,
    pin_count: core::cell::Cell<core::num::Wrapping<usize>>,
    epoch: crossbeam_utils::CachePadded<crate::epoch::AtomicEpoch>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:271-296`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/internal.rs#L271-L296)*

Participant for garbage collection.

#### Fields

- **`entry`**: `crate::sync::list::Entry`

  A node in the intrusive linked list of `Local`s.

- **`collector`**: `crate::primitive::cell::UnsafeCell<core::mem::ManuallyDrop<crate::collector::Collector>>`

  A reference to the global data.
  
  When all guards and handles get dropped, this reference is destroyed.

- **`bag`**: `crate::primitive::cell::UnsafeCell<Bag>`

  The local bag of deferred functions.

- **`guard_count`**: `core::cell::Cell<usize>`

  The number of guards keeping this participant pinned.

- **`handle_count`**: `core::cell::Cell<usize>`

  The number of active handles.

- **`pin_count`**: `core::cell::Cell<core::num::Wrapping<usize>>`

  Total number of pinnings performed.
  
  This is just an auxiliary counter that sometimes kicks off collection.

- **`epoch`**: `crossbeam_utils::CachePadded<crate::epoch::AtomicEpoch>`

  The local epoch.

#### Implementations

- <span id="local-const-pinnings-between-collect"></span>`const PINNINGS_BETWEEN_COLLECT: usize`

- <span id="local-register"></span>`fn register(collector: &Collector) -> LocalHandle` — [`Collector`](../collector/index.md#collector), [`LocalHandle`](../collector/index.md#localhandle)

  Registers a new `Local` in the provided `Global`.

- <span id="local-global"></span>`fn global(&self) -> &Global` — [`Global`](#global)

  Returns a reference to the `Global` in which this `Local` resides.

- <span id="local-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](../collector/index.md#collector)

  Returns a reference to the `Collector` in which this `Local` resides.

- <span id="local-is-pinned"></span>`fn is_pinned(&self) -> bool`

  Returns `true` if the current participant is pinned.

- <span id="local-defer"></span>`unsafe fn defer(&self, deferred: Deferred, guard: &Guard)` — [`Deferred`](../deferred/index.md#deferred), [`Guard`](../guard/index.md#guard)

  Adds `deferred` to the thread-local bag.

  

  # Safety

  

  It should be safe for another thread to execute the given function.

- <span id="local-flush"></span>`fn flush(&self, guard: &Guard)` — [`Guard`](../guard/index.md#guard)

- <span id="local-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](../guard/index.md#guard)

  Pins the `Local`.

- <span id="local-unpin"></span>`fn unpin(&self)`

  Unpins the `Local`.

- <span id="local-repin"></span>`fn repin(&self)`

  Unpins and then pins the `Local`.

- <span id="local-acquire-handle"></span>`fn acquire_handle(&self)`

  Increments the handle count.

- <span id="local-release-handle"></span>`fn release_handle(&self)`

  Decrements the handle count.

- <span id="local-finalize"></span>`fn finalize(&self)`

  Removes the `Local` from the global linked list.

#### Trait Implementations

##### `impl Any for Local`

- <span id="local-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Local`

- <span id="local-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Local`

- <span id="local-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Local`

- <span id="local-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Local`

- <span id="local-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IsElement for Local`

- <span id="local-iselement-entry-of"></span>`fn entry_of(local: &Self) -> &Entry` — [`Entry`](../sync/list/index.md#entry)

- <span id="local-iselement-element-of"></span>`unsafe fn element_of(entry: &Entry) -> &Self` — [`Entry`](../sync/list/index.md#entry)

- <span id="local-iselement-finalize"></span>`unsafe fn finalize(entry: &Entry, guard: &Guard)` — [`Entry`](../sync/list/index.md#entry), [`Guard`](../guard/index.md#guard)

##### `impl Pointable for Local`

- <span id="local-pointable-const-align"></span>`const ALIGN: usize`

- <span id="local-pointable-type-init"></span>`type Init = T`

- <span id="local-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="local-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="local-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="local-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Local`

- <span id="local-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="local-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Local`

- <span id="local-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="local-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `MAX_OBJECTS`
```rust
const MAX_OBJECTS: usize = 64usize;
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:57`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/internal.rs#L57)*

Maximum number of objects a bag can contain.

