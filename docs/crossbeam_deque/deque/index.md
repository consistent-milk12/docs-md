*[crossbeam_deque](../index.md) / [deque](index.md)*

---

# Module `deque`

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

- `fn alloc(cap: usize) -> Buffer<T>` — [`Buffer`](#buffer)

- `unsafe fn dealloc(self: Self)`

- `unsafe fn at(self: &Self, index: isize) -> *mut T`

- `unsafe fn write(self: &Self, index: isize, task: MaybeUninit<T>)`

- `unsafe fn read(self: &Self, index: isize) -> MaybeUninit<T>`

#### Trait Implementations

##### `impl<T> Clone for Buffer<T>`

- `fn clone(self: &Self) -> Buffer<T>` — [`Buffer`](#buffer)

##### `impl<T> Copy for Buffer<T>`

##### `impl<T> Pointable for Buffer<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Inner<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new_fifo() -> Worker<T>` — [`Worker`](#worker)

- `fn new_lifo() -> Worker<T>` — [`Worker`](#worker)

- `fn stealer(self: &Self) -> Stealer<T>` — [`Stealer`](#stealer)

- `unsafe fn resize(self: &Self, new_cap: usize)`

- `fn reserve(self: &Self, reserve_cap: usize)`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn push(self: &Self, task: T)`

- `fn pop(self: &Self) -> Option<T>`

#### Trait Implementations

##### `impl<T> Debug for Worker<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for Worker<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn steal(self: &Self) -> Steal<T>` — [`Steal`](#steal)

- `fn steal_batch(self: &Self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

- `fn steal_batch_with_limit(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

- `fn steal_batch_and_pop(self: &Self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

- `fn steal_batch_with_limit_and_pop(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

#### Trait Implementations

##### `impl<T> Clone for Stealer<T>`

- `fn clone(self: &Self) -> Stealer<T>` — [`Stealer`](#stealer)

##### `impl<T> Debug for Stealer<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for Stealer<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn wait_write(self: &Self)`

#### Trait Implementations

##### `impl<T> Pointable for Slot<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const LAYOUT: Layout`

- `fn new() -> Box<Self>`

- `fn wait_next(self: &Self) -> *mut Block<T>` — [`Block`](#block)

- `unsafe fn destroy(this: *mut Block<T>, count: usize)` — [`Block`](#block)

#### Trait Implementations

##### `impl<T> Pointable for Block<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new() -> Injector<T>` — [`Injector`](#injector)

- `fn push(self: &Self, task: T)`

- `fn steal(self: &Self) -> Steal<T>` — [`Steal`](#steal)

- `fn steal_batch(self: &Self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

- `fn steal_batch_with_limit(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

- `fn steal_batch_and_pop(self: &Self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

- `fn steal_batch_with_limit_and_pop(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl<T> Debug for Injector<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Injector<T>`

- `fn default() -> Self`

##### `impl<T> Drop for Injector<T>`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Injector<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn clone(self: &Self) -> Flavor` — [`Flavor`](#flavor)

##### `impl Copy for Flavor`

##### `impl Debug for Flavor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Flavor`

##### `impl PartialEq for Flavor`

- `fn eq(self: &Self, other: &Flavor) -> bool` — [`Flavor`](#flavor)

##### `impl<T> Pointable for Flavor`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn is_empty(self: &Self) -> bool`

- `fn is_success(self: &Self) -> bool`

- `fn is_retry(self: &Self) -> bool`

- `fn success(self: Self) -> Option<T>`

- `fn or_else<F>(self: Self, f: F) -> Steal<T>` — [`Steal`](#steal)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Steal<T>`

- `fn clone(self: &Self) -> Steal<T>` — [`Steal`](#steal)

##### `impl<T: $crate::marker::Copy> Copy for Steal<T>`

##### `impl<T> Debug for Steal<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for Steal<T>`

##### `impl<T> FromIterator for Steal<T>`

- `fn from_iter<I>(iter: I) -> Steal<T>` — [`Steal`](#steal)

##### `impl<T: $crate::cmp::PartialEq> PartialEq for Steal<T>`

- `fn eq(self: &Self, other: &Steal<T>) -> bool` — [`Steal`](#steal)

##### `impl<T> Pointable for Steal<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

