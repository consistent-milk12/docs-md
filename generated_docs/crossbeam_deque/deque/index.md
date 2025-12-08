*[crossbeam_deque](../index.md) / [deque](index.md)*

---

# Module `deque`

## Contents

- [Structs](#structs)
  - [`Buffer`](#buffer)
  - [`Inner`](#inner)
  - [`Worker`](#worker)
  - [`Stealer`](#stealer)
  - [`Slot`](#slot)
  - [`Block`](#block)
  - [`Position`](#position)
  - [`Injector`](#injector)
- [Enums](#enums)
  - [`Flavor`](#flavor)
  - [`Steal`](#steal)
- [Constants](#constants)
  - [`MIN_CAP`](#min_cap)
  - [`MAX_BATCH`](#max_batch)
  - [`FLUSH_THRESHOLD_BYTES`](#flush_threshold_bytes)
  - [`WRITE`](#write)
  - [`READ`](#read)
  - [`DESTROY`](#destroy)
  - [`LAP`](#lap)
  - [`BLOCK_CAP`](#block_cap)
  - [`SHIFT`](#shift)
  - [`HAS_NEXT`](#has_next)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buffer`](#buffer) | struct | A buffer that holds tasks in a worker queue. |
| [`Inner`](#inner) | struct | Internal queue data shared between the worker and stealers. |
| [`Worker`](#worker) | struct | A worker queue. |
| [`Stealer`](#stealer) | struct | A stealer handle of a worker queue. |
| [`Slot`](#slot) | struct | A slot in a block. |
| [`Block`](#block) | struct | A block in a linked list. |
| [`Position`](#position) | struct | A position in a queue. |
| [`Injector`](#injector) | struct | An injector queue. |
| [`Flavor`](#flavor) | enum | Worker queue flavor: FIFO or LIFO. |
| [`Steal`](#steal) | enum | Possible outcomes of a steal operation. |
| [`MIN_CAP`](#min_cap) | const |  |
| [`MAX_BATCH`](#max_batch) | const |  |
| [`FLUSH_THRESHOLD_BYTES`](#flush_threshold_bytes) | const |  |
| [`WRITE`](#write) | const |  |
| [`READ`](#read) | const |  |
| [`DESTROY`](#destroy) | const |  |
| [`LAP`](#lap) | const |  |
| [`BLOCK_CAP`](#block_cap) | const |  |
| [`SHIFT`](#shift) | const |  |
| [`HAS_NEXT`](#has_next) | const |  |

## Structs

### `Buffer<T>`

```rust
struct Buffer<T> {
    ptr: *mut T,
    cap: usize,
}
```

A buffer that holds tasks in a worker queue.

This is just a pointer to the buffer and its length - dropping an instance of this struct will
*not* deallocate the buffer.

#### Fields

- **`ptr`**: `*mut T`

  Pointer to the allocated memory.

- **`cap`**: `usize`

  Capacity of the buffer. Always a power of two.

#### Implementations

- <span id="buffer-alloc"></span>`fn alloc(cap: usize) -> Buffer<T>` — [`Buffer`](#buffer)

- <span id="buffer-dealloc"></span>`unsafe fn dealloc(self)`

- <span id="buffer-at"></span>`unsafe fn at(&self, index: isize) -> *mut T`

- <span id="buffer-write"></span>`unsafe fn write(&self, index: isize, task: MaybeUninit<T>)`

- <span id="buffer-read"></span>`unsafe fn read(&self, index: isize) -> MaybeUninit<T>`

#### Trait Implementations

##### `impl<T> Clone for Buffer<T>`

- <span id="buffer-clone"></span>`fn clone(&self) -> Buffer<T>` — [`Buffer`](#buffer)

##### `impl<T> Copy for Buffer<T>`

##### `impl<T> Pointable for Buffer<T>`

- <span id="buffer-align"></span>`const ALIGN: usize`

- <span id="buffer-init"></span>`type Init = T`

- <span id="buffer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="buffer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="buffer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="buffer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Send for Buffer<T>`

### `Inner<T>`

```rust
struct Inner<T> {
    front: std::sync::atomic::AtomicIsize,
    back: std::sync::atomic::AtomicIsize,
    buffer: crossbeam_utils::CachePadded<crossbeam_epoch::Atomic<Buffer<T>>>,
}
```

Internal queue data shared between the worker and stealers.

The implementation is based on the following work:

1. [Chase and Lev. Dynamic circular work-stealing deque. SPAA 2005.][chase-lev]
2. [Le, Pop, Cohen, and Nardelli. Correct and efficient work-stealing for weak memory models.
   PPoPP 2013.][weak-mem]
3. [Norris and Demsky. CDSchecker: checking concurrent data structures written with C/C++
   atomics. OOPSLA 2013.][checker]




#### Fields

- **`front`**: `std::sync::atomic::AtomicIsize`

  The front index.

- **`back`**: `std::sync::atomic::AtomicIsize`

  The back index.

- **`buffer`**: `crossbeam_utils::CachePadded<crossbeam_epoch::Atomic<Buffer<T>>>`

  The underlying buffer.

#### Trait Implementations

##### `impl<T> Drop for Inner<T>`

- <span id="inner-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Inner<T>`

- <span id="inner-align"></span>`const ALIGN: usize`

- <span id="inner-init"></span>`type Init = T`

- <span id="inner-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inner-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inner-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inner-drop"></span>`unsafe fn drop(ptr: usize)`

### `Worker<T>`

```rust
struct Worker<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    buffer: std::cell::Cell<Buffer<T>>,
    flavor: Flavor,
    _marker: std::marker::PhantomData<*mut ()>,
}
```

A worker queue.

This is a FIFO or LIFO queue that is owned by a single thread, but other threads may steal
tasks from it. Task schedulers typically create a single worker queue per thread.

# Examples

A FIFO worker:

```rust
use crossbeam_deque::{Steal, Worker};

let w = Worker::new_fifo();
let s = w.stealer();

w.push(1);
w.push(2);
w.push(3);

assert_eq!(s.steal(), Steal::Success(1));
assert_eq!(w.pop(), Some(2));
assert_eq!(w.pop(), Some(3));
```

A LIFO worker:

```rust
use crossbeam_deque::{Steal, Worker};

let w = Worker::new_lifo();
let s = w.stealer();

w.push(1);
w.push(2);
w.push(3);

assert_eq!(s.steal(), Steal::Success(1));
assert_eq!(w.pop(), Some(3));
assert_eq!(w.pop(), Some(2));
```

#### Fields

- **`inner`**: `std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>`

  A reference to the inner representation of the queue.

- **`buffer`**: `std::cell::Cell<Buffer<T>>`

  A copy of `inner.buffer` for quick access.

- **`flavor`**: `Flavor`

  The flavor of the queue.

- **`_marker`**: `std::marker::PhantomData<*mut ()>`

  Indicates that the worker cannot be shared among threads.

#### Implementations

- <span id="worker-new-fifo"></span>`fn new_fifo() -> Worker<T>` — [`Worker`](../index.md)

- <span id="worker-new-lifo"></span>`fn new_lifo() -> Worker<T>` — [`Worker`](../index.md)

- <span id="worker-stealer"></span>`fn stealer(&self) -> Stealer<T>` — [`Stealer`](../index.md)

- <span id="worker-resize"></span>`unsafe fn resize(&self, new_cap: usize)`

- <span id="worker-reserve"></span>`fn reserve(&self, reserve_cap: usize)`

- <span id="worker-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="worker-len"></span>`fn len(&self) -> usize`

- <span id="worker-push"></span>`fn push(&self, task: T)`

- <span id="worker-pop"></span>`fn pop(&self) -> Option<T>`

#### Trait Implementations

##### `impl<T> Debug for Worker<T>`

- <span id="worker-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for Worker<T>`

- <span id="worker-align"></span>`const ALIGN: usize`

- <span id="worker-init"></span>`type Init = T`

- <span id="worker-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="worker-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="worker-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="worker-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Worker<T>`

### `Stealer<T>`

```rust
struct Stealer<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    flavor: Flavor,
}
```

A stealer handle of a worker queue.

Stealers can be shared among threads.

Task schedulers typically have a single worker queue per worker thread.

# Examples

```rust
use crossbeam_deque::{Steal, Worker};

let w = Worker::new_lifo();
w.push(1);
w.push(2);

let s = w.stealer();
assert_eq!(s.steal(), Steal::Success(1));
assert_eq!(s.steal(), Steal::Success(2));
assert_eq!(s.steal(), Steal::Empty);
```

#### Fields

- **`inner`**: `std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>`

  A reference to the inner representation of the queue.

- **`flavor`**: `Flavor`

  The flavor of the queue.

#### Implementations

- <span id="stealer-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="stealer-len"></span>`fn len(&self) -> usize`

- <span id="stealer-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](../index.md)

- <span id="stealer-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](../index.md), [`Steal`](../index.md)

- <span id="stealer-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](../index.md), [`Steal`](../index.md)

- <span id="stealer-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](../index.md), [`Steal`](../index.md)

- <span id="stealer-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](../index.md), [`Steal`](../index.md)

#### Trait Implementations

##### `impl<T> Clone for Stealer<T>`

- <span id="stealer-clone"></span>`fn clone(&self) -> Stealer<T>` — [`Stealer`](../index.md)

##### `impl<T> Debug for Stealer<T>`

- <span id="stealer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for Stealer<T>`

- <span id="stealer-align"></span>`const ALIGN: usize`

- <span id="stealer-init"></span>`type Init = T`

- <span id="stealer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stealer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stealer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stealer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Stealer<T>`

##### `impl<T: Send> Sync for Stealer<T>`

### `Slot<T>`

```rust
struct Slot<T> {
    task: std::cell::UnsafeCell<std::mem::MaybeUninit<T>>,
    state: std::sync::atomic::AtomicUsize,
}
```

A slot in a block.

#### Fields

- **`task`**: `std::cell::UnsafeCell<std::mem::MaybeUninit<T>>`

  The task.

- **`state`**: `std::sync::atomic::AtomicUsize`

  The state of the slot.

#### Implementations

- <span id="slot-wait-write"></span>`fn wait_write(&self)`

#### Trait Implementations

##### `impl<T> Pointable for Slot<T>`

- <span id="slot-align"></span>`const ALIGN: usize`

- <span id="slot-init"></span>`type Init = T`

- <span id="slot-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="slot-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="slot-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="slot-drop"></span>`unsafe fn drop(ptr: usize)`

### `Block<T>`

```rust
struct Block<T> {
    next: std::sync::atomic::AtomicPtr<Block<T>>,
    slots: [Slot<T>; 63],
}
```

A block in a linked list.

Each block in the list can hold up to `BLOCK_CAP` values.

#### Fields

- **`next`**: `std::sync::atomic::AtomicPtr<Block<T>>`

  The next block in the linked list.

- **`slots`**: `[Slot<T>; 63]`

  Slots for values.

#### Implementations

- <span id="block-layout"></span>`const LAYOUT: Layout`

- <span id="block-new"></span>`fn new() -> Box<Self>`

- <span id="block-wait-next"></span>`fn wait_next(&self) -> *mut Block<T>` — [`Block`](#block)

- <span id="block-destroy"></span>`unsafe fn destroy(this: *mut Block<T>, count: usize)` — [`Block`](#block)

#### Trait Implementations

##### `impl<T> Pointable for Block<T>`

- <span id="block-align"></span>`const ALIGN: usize`

- <span id="block-init"></span>`type Init = T`

- <span id="block-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="block-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="block-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="block-drop"></span>`unsafe fn drop(ptr: usize)`

### `Position<T>`

```rust
struct Position<T> {
    index: std::sync::atomic::AtomicUsize,
    block: std::sync::atomic::AtomicPtr<Block<T>>,
}
```

A position in a queue.

#### Fields

- **`index`**: `std::sync::atomic::AtomicUsize`

  The index in the queue.

- **`block`**: `std::sync::atomic::AtomicPtr<Block<T>>`

  The block in the linked list.

#### Trait Implementations

##### `impl<T> Pointable for Position<T>`

- <span id="position-align"></span>`const ALIGN: usize`

- <span id="position-init"></span>`type Init = T`

- <span id="position-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="position-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="position-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="position-drop"></span>`unsafe fn drop(ptr: usize)`

### `Injector<T>`

```rust
struct Injector<T> {
    head: crossbeam_utils::CachePadded<Position<T>>,
    tail: crossbeam_utils::CachePadded<Position<T>>,
    _marker: std::marker::PhantomData<T>,
}
```

An injector queue.

This is a FIFO queue that can be shared among multiple threads. Task schedulers typically have
a single injector queue, which is the entry point for new tasks.

# Examples

```rust
use crossbeam_deque::{Injector, Steal};

let q = Injector::new();
q.push(1);
q.push(2);

assert_eq!(q.steal(), Steal::Success(1));
assert_eq!(q.steal(), Steal::Success(2));
assert_eq!(q.steal(), Steal::Empty);
```

#### Fields

- **`head`**: `crossbeam_utils::CachePadded<Position<T>>`

  The head of the queue.

- **`tail`**: `crossbeam_utils::CachePadded<Position<T>>`

  The tail of the queue.

- **`_marker`**: `std::marker::PhantomData<T>`

  Indicates that dropping a `Injector<T>` may drop values of type `T`.

#### Implementations

- <span id="injector-new"></span>`fn new() -> Injector<T>` — [`Injector`](../index.md)

- <span id="injector-push"></span>`fn push(&self, task: T)`

- <span id="injector-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](../index.md)

- <span id="injector-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](../index.md), [`Steal`](../index.md)

- <span id="injector-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](../index.md), [`Steal`](../index.md)

- <span id="injector-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](../index.md), [`Steal`](../index.md)

- <span id="injector-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](../index.md), [`Steal`](../index.md)

- <span id="injector-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="injector-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl<T> Debug for Injector<T>`

- <span id="injector-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Injector<T>`

- <span id="injector-default"></span>`fn default() -> Self`

##### `impl<T> Drop for Injector<T>`

- <span id="injector-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Injector<T>`

- <span id="injector-align"></span>`const ALIGN: usize`

- <span id="injector-init"></span>`type Init = T`

- <span id="injector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="injector-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="injector-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="injector-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Injector<T>`

##### `impl<T: Send> Sync for Injector<T>`

## Enums

### `Flavor`

```rust
enum Flavor {
    Fifo,
    Lifo,
}
```

Worker queue flavor: FIFO or LIFO.

#### Variants

- **`Fifo`**

  The first-in first-out flavor.

- **`Lifo`**

  The last-in first-out flavor.

#### Trait Implementations

##### `impl Clone for Flavor`

- <span id="flavor-clone"></span>`fn clone(&self) -> Flavor` — [`Flavor`](#flavor)

##### `impl Copy for Flavor`

##### `impl Debug for Flavor`

- <span id="flavor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Flavor`

##### `impl PartialEq for Flavor`

- <span id="flavor-eq"></span>`fn eq(&self, other: &Flavor) -> bool` — [`Flavor`](#flavor)

##### `impl<T> Pointable for Flavor`

- <span id="flavor-align"></span>`const ALIGN: usize`

- <span id="flavor-init"></span>`type Init = T`

- <span id="flavor-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flavor-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flavor-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flavor-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Flavor`

### `Steal<T>`

```rust
enum Steal<T> {
    Empty,
    Success(T),
    Retry,
}
```

Possible outcomes of a steal operation.

# Examples

There are lots of ways to chain results of steal operations together:

```rust
use crossbeam_deque::Steal::{self, Empty, Retry, Success};

let collect = |v: Vec<Steal<i32>>| v.into_iter().collect::<Steal<i32>>();

assert_eq!(collect(vec![Empty, Empty, Empty]), Empty);
assert_eq!(collect(vec![Empty, Retry, Empty]), Retry);
assert_eq!(collect(vec![Retry, Success(1), Empty]), Success(1));

assert_eq!(collect(vec![Empty, Empty]).or_else(|| Retry), Retry);
assert_eq!(collect(vec![Retry, Empty]).or_else(|| Success(1)), Success(1));
```

#### Variants

- **`Empty`**

  The queue was empty at the time of stealing.

- **`Success`**

  At least one task was successfully stolen.

- **`Retry`**

  The steal operation needs to be retried.

#### Implementations

- <span id="steal-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="steal-is-success"></span>`fn is_success(&self) -> bool`

- <span id="steal-is-retry"></span>`fn is_retry(&self) -> bool`

- <span id="steal-success"></span>`fn success(self) -> Option<T>`

- <span id="steal-or-else"></span>`fn or_else<F>(self, f: F) -> Steal<T>` — [`Steal`](../index.md)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Steal<T>`

- <span id="steal-clone"></span>`fn clone(&self) -> Steal<T>` — [`Steal`](../index.md)

##### `impl<T: marker::Copy> Copy for Steal<T>`

##### `impl<T> Debug for Steal<T>`

- <span id="steal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for Steal<T>`

##### `impl<T> FromIterator for Steal<T>`

- <span id="steal-from-iter"></span>`fn from_iter<I>(iter: I) -> Steal<T>` — [`Steal`](../index.md)

##### `impl<T: cmp::PartialEq> PartialEq for Steal<T>`

- <span id="steal-eq"></span>`fn eq(&self, other: &Steal<T>) -> bool` — [`Steal`](../index.md)

##### `impl<T> Pointable for Steal<T>`

- <span id="steal-align"></span>`const ALIGN: usize`

- <span id="steal-init"></span>`type Init = T`

- <span id="steal-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="steal-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="steal-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="steal-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> StructuralPartialEq for Steal<T>`

## Constants

### `MIN_CAP`

```rust
const MIN_CAP: usize = 64usize;
```

### `MAX_BATCH`

```rust
const MAX_BATCH: usize = 32usize;
```

### `FLUSH_THRESHOLD_BYTES`

```rust
const FLUSH_THRESHOLD_BYTES: usize = 1_024usize;
```

### `WRITE`

```rust
const WRITE: usize = 1usize;
```

### `READ`

```rust
const READ: usize = 2usize;
```

### `DESTROY`

```rust
const DESTROY: usize = 4usize;
```

### `LAP`

```rust
const LAP: usize = 64usize;
```

### `BLOCK_CAP`

```rust
const BLOCK_CAP: usize = 63usize;
```

### `SHIFT`

```rust
const SHIFT: usize = 1usize;
```

### `HAS_NEXT`

```rust
const HAS_NEXT: usize = 1usize;
```

