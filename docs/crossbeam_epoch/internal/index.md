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

## Structs

### `Bag`

```rust
struct Bag {
    deferreds: [crate::deferred::Deferred; 64],
    len: usize,
}
```

A bag of deferred functions.

#### Fields

- **`deferreds`**: `[crate::deferred::Deferred; 64]`

  Stashed objects.

#### Implementations

- `fn new() -> Self`

- `fn is_empty(self: &Self) -> bool`

- `unsafe fn try_push(self: &mut Self, deferred: Deferred) -> Result<(), Deferred>` — [`Deferred`](../deferred/index.md)

- `fn seal(self: Self, epoch: Epoch) -> SealedBag` — [`Epoch`](../epoch/index.md), [`SealedBag`](#sealedbag)

#### Trait Implementations

##### `impl Debug for Bag`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bag`

- `fn default() -> Self`

##### `impl Drop for Bag`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Bag`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Send for Bag`

### `SealedBag`

```rust
struct SealedBag {
    epoch: crate::epoch::Epoch,
    _bag: Bag,
}
```

A pair of an epoch and a bag.

#### Implementations

- `fn is_expired(self: &Self, global_epoch: Epoch) -> bool` — [`Epoch`](../epoch/index.md)

#### Trait Implementations

##### `impl Debug for SealedBag`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for SealedBag`

- `fn default() -> SealedBag` — [`SealedBag`](#sealedbag)

##### `impl<T> Pointable for SealedBag`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Sync for SealedBag`

### `Global`

```rust
struct Global {
    locals: crate::sync::list::List<Local>,
    queue: crate::sync::queue::Queue<SealedBag>,
    epoch: crossbeam_utils::CachePadded<crate::epoch::AtomicEpoch>,
}
```

The global data for a garbage collector.

#### Fields

- **`locals`**: `crate::sync::list::List<Local>`

  The intrusive linked list of `Local`s.

- **`queue`**: `crate::sync::queue::Queue<SealedBag>`

  The global queue of bags of deferred functions.

- **`epoch`**: `crossbeam_utils::CachePadded<crate::epoch::AtomicEpoch>`

  The global epoch.

#### Implementations

- `const COLLECT_STEPS: usize`

- `fn new() -> Self`

- `fn push_bag(self: &Self, bag: &mut Bag, guard: &Guard)` — [`Bag`](#bag), [`Guard`](../index.md)

- `fn collect(self: &Self, guard: &Guard)` — [`Guard`](../index.md)

- `fn try_advance(self: &Self, guard: &Guard) -> Epoch` — [`Guard`](../index.md), [`Epoch`](../epoch/index.md)

#### Trait Implementations

##### `impl<T> Pointable for Global`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const PINNINGS_BETWEEN_COLLECT: usize`

- `fn register(collector: &Collector) -> LocalHandle` — [`Collector`](../index.md), [`LocalHandle`](../index.md)

- `fn global(self: &Self) -> &Global` — [`Global`](#global)

- `fn collector(self: &Self) -> &Collector` — [`Collector`](../index.md)

- `fn is_pinned(self: &Self) -> bool`

- `unsafe fn defer(self: &Self, deferred: Deferred, guard: &Guard)` — [`Deferred`](../deferred/index.md), [`Guard`](../index.md)

- `fn flush(self: &Self, guard: &Guard)` — [`Guard`](../index.md)

- `fn pin(self: &Self) -> Guard` — [`Guard`](../index.md)

- `fn unpin(self: &Self)`

- `fn repin(self: &Self)`

- `fn acquire_handle(self: &Self)`

- `fn release_handle(self: &Self)`

- `fn finalize(self: &Self)`

#### Trait Implementations

##### `impl IsElement for Local`

- `fn entry_of(local: &Self) -> &Entry` — [`Entry`](../sync/list/index.md)

- `unsafe fn element_of(entry: &Entry) -> &Self` — [`Entry`](../sync/list/index.md)

- `unsafe fn finalize(entry: &Entry, guard: &Guard)` — [`Entry`](../sync/list/index.md), [`Guard`](../index.md)

##### `impl<T> Pointable for Local`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Constants

### `MAX_OBJECTS`

```rust
const MAX_OBJECTS: usize = 64usize;
```

Maximum number of objects a bag can contain.

