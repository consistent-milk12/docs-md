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

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:63-67`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/internal.rs#L63-L67)*

A bag of deferred functions.

#### Fields

- **`deferreds`**: `[crate::deferred::Deferred; 64]`

  Stashed objects.

#### Implementations

- <span id="bag-new"></span>`fn new() -> Self`

- <span id="bag-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="bag-try-push"></span>`unsafe fn try_push(&mut self, deferred: Deferred) -> Result<(), Deferred>` — [`Deferred`](../deferred/index.md#deferred)

- <span id="bag-seal"></span>`fn seal(self, epoch: Epoch) -> SealedBag` — [`Epoch`](../epoch/index.md#epoch), [`SealedBag`](#sealedbag)

#### Trait Implementations

##### `impl Debug for Bag`

- <span id="bag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bag`

- <span id="bag-default"></span>`fn default() -> Self`

##### `impl Drop for Bag`

- <span id="bag-drop"></span>`fn drop(&mut self)`

##### `impl Pointable for Bag`

- <span id="bag-pointable-const-align"></span>`const ALIGN: usize`

- <span id="bag-pointable-type-init"></span>`type Init = T`

- <span id="bag-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="bag-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="bag-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="bag-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Send for Bag`

### `SealedBag`

```rust
struct SealedBag {
    epoch: crate::epoch::Epoch,
    _bag: Bag,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:138-141`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/internal.rs#L138-L141)*

A pair of an epoch and a bag.

#### Implementations

- <span id="sealedbag-is-expired"></span>`fn is_expired(&self, global_epoch: Epoch) -> bool` — [`Epoch`](../epoch/index.md#epoch)

#### Trait Implementations

##### `impl Debug for SealedBag`

- <span id="sealedbag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SealedBag`

- <span id="sealedbag-default"></span>`fn default() -> SealedBag` — [`SealedBag`](#sealedbag)

##### `impl Pointable for SealedBag`

- <span id="sealedbag-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sealedbag-pointable-type-init"></span>`type Init = T`

- <span id="sealedbag-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="sealedbag-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sealedbag-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sealedbag-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Sync for SealedBag`

### `Global`

```rust
struct Global {
    locals: crate::sync::list::List<Local>,
    queue: crate::sync::queue::Queue<SealedBag>,
    epoch: crossbeam_utils::CachePadded<crate::epoch::AtomicEpoch>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:156-165`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/internal.rs#L156-L165)*

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

- <span id="global-push-bag"></span>`fn push_bag(&self, bag: &mut Bag, guard: &Guard)` — [`Bag`](#bag), [`Guard`](../guard/index.md#guard)

- <span id="global-collect"></span>`fn collect(&self, guard: &Guard)` — [`Guard`](../guard/index.md#guard)

- <span id="global-try-advance"></span>`fn try_advance(&self, guard: &Guard) -> Epoch` — [`Guard`](../guard/index.md#guard), [`Epoch`](../epoch/index.md#epoch)

#### Trait Implementations

##### `impl Pointable for Global`

- <span id="global-pointable-const-align"></span>`const ALIGN: usize`

- <span id="global-pointable-type-init"></span>`type Init = T`

- <span id="global-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="global-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="global-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="global-drop"></span>`unsafe fn drop(ptr: usize)`

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

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:271-296`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/internal.rs#L271-L296)*

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

- <span id="local-global"></span>`fn global(&self) -> &Global` — [`Global`](#global)

- <span id="local-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](../collector/index.md#collector)

- <span id="local-is-pinned"></span>`fn is_pinned(&self) -> bool`

- <span id="local-defer"></span>`unsafe fn defer(&self, deferred: Deferred, guard: &Guard)` — [`Deferred`](../deferred/index.md#deferred), [`Guard`](../guard/index.md#guard)

- <span id="local-flush"></span>`fn flush(&self, guard: &Guard)` — [`Guard`](../guard/index.md#guard)

- <span id="local-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](../guard/index.md#guard)

- <span id="local-unpin"></span>`fn unpin(&self)`

- <span id="local-repin"></span>`fn repin(&self)`

- <span id="local-acquire-handle"></span>`fn acquire_handle(&self)`

- <span id="local-release-handle"></span>`fn release_handle(&self)`

- <span id="local-finalize"></span>`fn finalize(&self)`

#### Trait Implementations

##### `impl IsElement for Local`

- <span id="local-entry-of"></span>`fn entry_of(local: &Self) -> &Entry` — [`Entry`](../sync/list/index.md#entry)

- <span id="local-element-of"></span>`unsafe fn element_of(entry: &Entry) -> &Self` — [`Entry`](../sync/list/index.md#entry)

- <span id="local-finalize"></span>`unsafe fn finalize(entry: &Entry, guard: &Guard)` — [`Entry`](../sync/list/index.md#entry), [`Guard`](../guard/index.md#guard)

##### `impl Pointable for Local`

- <span id="local-pointable-const-align"></span>`const ALIGN: usize`

- <span id="local-pointable-type-init"></span>`type Init = T`

- <span id="local-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="local-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="local-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="local-drop"></span>`unsafe fn drop(ptr: usize)`

## Constants

### `MAX_OBJECTS`
```rust
const MAX_OBJECTS: usize = 64usize;
```

*Defined in [`crossbeam-epoch-0.9.18/src/internal.rs:57`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/internal.rs#L57)*

Maximum number of objects a bag can contain.

