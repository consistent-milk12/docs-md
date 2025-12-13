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
  - [`MIN_CAP`](#min-cap)
  - [`MAX_BATCH`](#max-batch)
  - [`FLUSH_THRESHOLD_BYTES`](#flush-threshold-bytes)
  - [`WRITE`](#write)
  - [`READ`](#read)
  - [`DESTROY`](#destroy)
  - [`LAP`](#lap)
  - [`BLOCK_CAP`](#block-cap)
  - [`SHIFT`](#shift)
  - [`HAS_NEXT`](#has-next)

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
| [`MIN_CAP`](#min-cap) | const |  |
| [`MAX_BATCH`](#max-batch) | const |  |
| [`FLUSH_THRESHOLD_BYTES`](#flush-threshold-bytes) | const |  |
| [`WRITE`](#write) | const |  |
| [`READ`](#read) | const |  |
| [`DESTROY`](#destroy) | const |  |
| [`LAP`](#lap) | const |  |
| [`BLOCK_CAP`](#block-cap) | const |  |
| [`SHIFT`](#shift) | const |  |
| [`HAS_NEXT`](#has-next) | const |  |

## Structs

### `Buffer<T>`

```rust
struct Buffer<T> {
    ptr: *mut T,
    cap: usize,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:27-33`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L27-L33)*

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

  Allocates a new buffer with the specified capacity.

- <span id="buffer-dealloc"></span>`unsafe fn dealloc(self)`

  Deallocates the buffer.

- <span id="buffer-at"></span>`unsafe fn at(&self, index: isize) -> *mut T`

  Returns a pointer to the task at the specified `index`.

- <span id="buffer-write"></span>`unsafe fn write(&self, index: isize, task: MaybeUninit<T>)`

  Writes `task` into the specified `index`.

  

  This method might be concurrently called with another `read` at the same index, which is

  technically speaking a data race and therefore UB. We should use an atomic store here, but

  that would be more expensive and difficult to implement generically for all types `T`.

  Hence, as a hack, we use a volatile write instead.

- <span id="buffer-read"></span>`unsafe fn read(&self, index: isize) -> MaybeUninit<T>`

  Reads a task from the specified `index`.

  

  This method might be concurrently called with another `write` at the same index, which is

  technically speaking a data race and therefore UB. We should use an atomic load here, but

  that would be more expensive and difficult to implement generically for all types `T`.

  Hence, as a hack, we use a volatile load instead.

#### Trait Implementations

##### `impl<T> Any for Buffer<T>`

- <span id="buffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Buffer<T>`

- <span id="buffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Buffer<T>`

- <span id="buffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Buffer<T>`

- <span id="buffer-clone"></span>`fn clone(&self) -> Buffer<T>` — [`Buffer`](#buffer)

##### `impl<T> CloneToUninit for Buffer<T>`

- <span id="buffer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Copy for Buffer<T>`

##### `impl<T> From for Buffer<T>`

- <span id="buffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Buffer<T>`

- <span id="buffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Buffer<T>`

- <span id="buffer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="buffer-pointable-type-init"></span>`type Init = T`

- <span id="buffer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="buffer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="buffer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="buffer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Send for Buffer<T>`

##### `impl<T> ToOwned for Buffer<T>`

- <span id="buffer-toowned-type-owned"></span>`type Owned = T`

- <span id="buffer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="buffer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Buffer<T>`

- <span id="buffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Buffer<T>`

- <span id="buffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Inner<T>`

```rust
struct Inner<T> {
    front: std::sync::atomic::AtomicIsize,
    back: std::sync::atomic::AtomicIsize,
    buffer: crossbeam_utils::CachePadded<crossbeam_epoch::Atomic<Buffer<T>>>,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:110-119`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L110-L119)*

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

##### `impl<T> Any for Inner<T>`

- <span id="inner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inner<T>`

- <span id="inner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inner<T>`

- <span id="inner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Drop for Inner<T>`

- <span id="inner-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Inner<T>`

- <span id="inner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Inner<T>`

- <span id="inner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Inner<T>`

- <span id="inner-pointable-const-align"></span>`const ALIGN: usize`

- <span id="inner-pointable-type-init"></span>`type Init = T`

- <span id="inner-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inner-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inner-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inner-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Inner<T>`

- <span id="inner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Inner<T>`

- <span id="inner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Worker<T>`

```rust
struct Worker<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    buffer: std::cell::Cell<Buffer<T>>,
    flavor: Flavor,
    _marker: std::marker::PhantomData<*mut ()>,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:193-205`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L193-L205)*

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

- <span id="worker-new-fifo"></span>`fn new_fifo() -> Worker<T>` — [`Worker`](#worker)

  Creates a FIFO worker queue.

  

  Tasks are pushed and popped from opposite ends.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::<i32>::new_fifo();

  ```

- <span id="worker-new-lifo"></span>`fn new_lifo() -> Worker<T>` — [`Worker`](#worker)

  Creates a LIFO worker queue.

  

  Tasks are pushed and popped from the same end.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::<i32>::new_lifo();

  ```

- <span id="worker-stealer"></span>`fn stealer(&self) -> Stealer<T>` — [`Stealer`](#stealer)

  Creates a stealer for this queue.

  

  The returned stealer can be shared among threads and cloned.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::<i32>::new_lifo();

  let s = w.stealer();

  ```

- <span id="worker-resize"></span>`unsafe fn resize(&self, new_cap: usize)`

  Resizes the internal buffer to the new capacity of `new_cap`.

- <span id="worker-reserve"></span>`fn reserve(&self, reserve_cap: usize)`

  Reserves enough capacity so that `reserve_cap` tasks can be pushed without growing the

  buffer.

- <span id="worker-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the queue is empty.

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::new_lifo();

  

  assert!(w.is_empty());

  w.push(1);

  assert!(!w.is_empty());

  ```

- <span id="worker-len"></span>`fn len(&self) -> usize`

  Returns the number of tasks in the deque.

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::new_lifo();

  

  assert_eq!(w.len(), 0);

  w.push(1);

  assert_eq!(w.len(), 1);

  w.push(1);

  assert_eq!(w.len(), 2);

  ```

- <span id="worker-push"></span>`fn push(&self, task: T)`

  Pushes a task into the queue.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::new_lifo();

  w.push(1);

  w.push(2);

  ```

- <span id="worker-pop"></span>`fn pop(&self) -> Option<T>`

  Pops a task from the queue.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::new_fifo();

  w.push(1);

  w.push(2);

  

  assert_eq!(w.pop(), Some(1));

  assert_eq!(w.pop(), Some(2));

  assert_eq!(w.pop(), None);

  ```

#### Trait Implementations

##### `impl<T> Any for Worker<T>`

- <span id="worker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Worker<T>`

- <span id="worker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Worker<T>`

- <span id="worker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for Worker<T>`

- <span id="worker-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Worker<T>`

- <span id="worker-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Worker<T>`

- <span id="worker-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Worker<T>`

- <span id="worker-pointable-const-align"></span>`const ALIGN: usize`

- <span id="worker-pointable-type-init"></span>`type Init = T`

- <span id="worker-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="worker-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="worker-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="worker-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Worker<T>`

##### `impl<T, U> TryFrom for Worker<T>`

- <span id="worker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="worker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Worker<T>`

- <span id="worker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="worker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Stealer<T>`

```rust
struct Stealer<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    flavor: Flavor,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:566-572`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L566-L572)*

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

  Returns `true` if the queue is empty.

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::new_lifo();

  let s = w.stealer();

  

  assert!(s.is_empty());

  w.push(1);

  assert!(!s.is_empty());

  ```

- <span id="stealer-len"></span>`fn len(&self) -> usize`

  Returns the number of tasks in the deque.

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::new_lifo();

  let s = w.stealer();

  

  assert_eq!(s.len(), 0);

  w.push(1);

  assert_eq!(s.len(), 1);

  w.push(2);

  assert_eq!(s.len(), 2);

  ```

- <span id="stealer-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](#steal)

  Steals a task from the queue.

  

  # Examples

  

  ```rust

  use crossbeam_deque::{Steal, Worker};

  

  let w = Worker::new_lifo();

  w.push(1);

  w.push(2);

  

  let s = w.stealer();

  assert_eq!(s.steal(), Steal::Success(1));

  assert_eq!(s.steal(), Steal::Success(2));

  ```

- <span id="stealer-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals a batch of tasks and pushes them into another worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than some constant limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w1 = Worker::new_fifo();

  w1.push(1);

  w1.push(2);

  w1.push(3);

  w1.push(4);

  

  let s = w1.stealer();

  let w2 = Worker::new_fifo();

  

  let _ = s.steal_batch(&w2);

  assert_eq!(w2.pop(), Some(1));

  assert_eq!(w2.pop(), Some(2));

  ```

- <span id="stealer-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals no more than `limit` of tasks and pushes them into another worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than the given limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w1 = Worker::new_fifo();

  w1.push(1);

  w1.push(2);

  w1.push(3);

  w1.push(4);

  w1.push(5);

  w1.push(6);

  

  let s = w1.stealer();

  let w2 = Worker::new_fifo();

  

  let _ = s.steal_batch_with_limit(&w2, 2);

  assert_eq!(w2.pop(), Some(1));

  assert_eq!(w2.pop(), Some(2));

  assert_eq!(w2.pop(), None);

  

  w1.push(7);

  w1.push(8);

  // Setting a large limit does not guarantee that all elements will be popped. In this case,

  // half of the elements are currently popped, but the number of popped elements is considered

  // an implementation detail that may be changed in the future.

  let _ = s.steal_batch_with_limit(&w2, std::usize::MAX);

  assert_eq!(w2.len(), 3);

  ```

- <span id="stealer-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals a batch of tasks, pushes them into another worker, and pops a task from that worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than some constant limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::{Steal, Worker};

  

  let w1 = Worker::new_fifo();

  w1.push(1);

  w1.push(2);

  w1.push(3);

  w1.push(4);

  

  let s = w1.stealer();

  let w2 = Worker::new_fifo();

  

  assert_eq!(s.steal_batch_and_pop(&w2), Steal::Success(1));

  assert_eq!(w2.pop(), Some(2));

  ```

- <span id="stealer-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals no more than `limit` of tasks, pushes them into another worker, and pops a task from

  that worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than the given limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::{Steal, Worker};

  

  let w1 = Worker::new_fifo();

  w1.push(1);

  w1.push(2);

  w1.push(3);

  w1.push(4);

  w1.push(5);

  w1.push(6);

  

  let s = w1.stealer();

  let w2 = Worker::new_fifo();

  

  assert_eq!(s.steal_batch_with_limit_and_pop(&w2, 2), Steal::Success(1));

  assert_eq!(w2.pop(), Some(2));

  assert_eq!(w2.pop(), None);

  

  w1.push(7);

  w1.push(8);

  // Setting a large limit does not guarantee that all elements will be popped. In this case,

  // half of the elements are currently popped, but the number of popped elements is considered

  // an implementation detail that may be changed in the future.

  assert_eq!(s.steal_batch_with_limit_and_pop(&w2, std::usize::MAX), Steal::Success(3));

  assert_eq!(w2.pop(), Some(4));

  assert_eq!(w2.pop(), Some(5));

  assert_eq!(w2.pop(), None);

  ```

#### Trait Implementations

##### `impl<T> Any for Stealer<T>`

- <span id="stealer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Stealer<T>`

- <span id="stealer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Stealer<T>`

- <span id="stealer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Stealer<T>`

- <span id="stealer-clone"></span>`fn clone(&self) -> Stealer<T>` — [`Stealer`](#stealer)

##### `impl<T> CloneToUninit for Stealer<T>`

- <span id="stealer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for Stealer<T>`

- <span id="stealer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Stealer<T>`

- <span id="stealer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Stealer<T>`

- <span id="stealer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Stealer<T>`

- <span id="stealer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="stealer-pointable-type-init"></span>`type Init = T`

- <span id="stealer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stealer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stealer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stealer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Stealer<T>`

##### `impl<T: Send> Sync for Stealer<T>`

##### `impl<T> ToOwned for Stealer<T>`

- <span id="stealer-toowned-type-owned"></span>`type Owned = T`

- <span id="stealer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stealer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Stealer<T>`

- <span id="stealer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stealer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Stealer<T>`

- <span id="stealer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stealer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Slot<T>`

```rust
struct Slot<T> {
    task: std::cell::UnsafeCell<std::mem::MaybeUninit<T>>,
    state: std::sync::atomic::AtomicUsize,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1198-1204`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1198-L1204)*

A slot in a block.

#### Fields

- **`task`**: `std::cell::UnsafeCell<std::mem::MaybeUninit<T>>`

  The task.

- **`state`**: `std::sync::atomic::AtomicUsize`

  The state of the slot.

#### Implementations

- <span id="slot-wait-write"></span>`fn wait_write(&self)`

  Waits until a task is written into the slot.

#### Trait Implementations

##### `impl<T> Any for Slot<T>`

- <span id="slot-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Slot<T>`

- <span id="slot-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Slot<T>`

- <span id="slot-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Slot<T>`

- <span id="slot-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Slot<T>`

- <span id="slot-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Slot<T>`

- <span id="slot-pointable-const-align"></span>`const ALIGN: usize`

- <span id="slot-pointable-type-init"></span>`type Init = T`

- <span id="slot-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="slot-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="slot-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="slot-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Slot<T>`

- <span id="slot-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slot-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Slot<T>`

- <span id="slot-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slot-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Block<T>`

```rust
struct Block<T> {
    next: std::sync::atomic::AtomicPtr<Block<T>>,
    slots: [Slot<T>; 63],
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1219-1225`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1219-L1225)*

A block in a linked list.

Each block in the list can hold up to `BLOCK_CAP` values.

#### Fields

- **`next`**: `std::sync::atomic::AtomicPtr<Block<T>>`

  The next block in the linked list.

- **`slots`**: `[Slot<T>; 63]`

  Slots for values.

#### Implementations

- <span id="block-const-layout"></span>`const LAYOUT: Layout`

- <span id="block-new"></span>`fn new() -> Box<Self>`

  Creates an empty block.

- <span id="block-wait-next"></span>`fn wait_next(&self) -> *mut Block<T>` — [`Block`](#block)

  Waits until the next pointer is set.

- <span id="block-destroy"></span>`unsafe fn destroy(this: *mut Block<T>, count: usize)` — [`Block`](#block)

  Sets the `DESTROY` bit in slots starting from `start` and destroys the block.

#### Trait Implementations

##### `impl<T> Any for Block<T>`

- <span id="block-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Block<T>`

- <span id="block-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Block<T>`

- <span id="block-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Block<T>`

- <span id="block-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Block<T>`

- <span id="block-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Block<T>`

- <span id="block-pointable-const-align"></span>`const ALIGN: usize`

- <span id="block-pointable-type-init"></span>`type Init = T`

- <span id="block-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="block-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="block-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="block-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Block<T>`

- <span id="block-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="block-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Block<T>`

- <span id="block-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="block-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Position<T>`

```rust
struct Position<T> {
    index: std::sync::atomic::AtomicUsize,
    block: std::sync::atomic::AtomicPtr<Block<T>>,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1289-1295`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1289-L1295)*

A position in a queue.

#### Fields

- **`index`**: `std::sync::atomic::AtomicUsize`

  The index in the queue.

- **`block`**: `std::sync::atomic::AtomicPtr<Block<T>>`

  The block in the linked list.

#### Trait Implementations

##### `impl<T> Any for Position<T>`

- <span id="position-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Position<T>`

- <span id="position-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Position<T>`

- <span id="position-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Position<T>`

- <span id="position-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Position<T>`

- <span id="position-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Position<T>`

- <span id="position-pointable-const-align"></span>`const ALIGN: usize`

- <span id="position-pointable-type-init"></span>`type Init = T`

- <span id="position-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="position-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="position-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="position-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Position<T>`

- <span id="position-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="position-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Position<T>`

- <span id="position-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="position-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Injector<T>`

```rust
struct Injector<T> {
    head: crossbeam_utils::CachePadded<Position<T>>,
    tail: crossbeam_utils::CachePadded<Position<T>>,
    _marker: std::marker::PhantomData<T>,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1315-1324`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1315-L1324)*

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

- <span id="injector-new"></span>`fn new() -> Injector<T>` — [`Injector`](#injector)

  Creates a new injector queue.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Injector;

  

  let q = Injector::<i32>::new();

  ```

- <span id="injector-push"></span>`fn push(&self, task: T)`

  Pushes a task into the queue.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Injector;

  

  let w = Injector::new();

  w.push(1);

  w.push(2);

  ```

- <span id="injector-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](#steal)

  Steals a task from the queue.

  

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

- <span id="injector-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals a batch of tasks and pushes them into a worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than some constant limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::{Injector, Worker};

  

  let q = Injector::new();

  q.push(1);

  q.push(2);

  q.push(3);

  q.push(4);

  

  let w = Worker::new_fifo();

  let _ = q.steal_batch(&w);

  assert_eq!(w.pop(), Some(1));

  assert_eq!(w.pop(), Some(2));

  ```

- <span id="injector-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals no more than of tasks and pushes them into a worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than some constant limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::{Injector, Worker};

  

  let q = Injector::new();

  q.push(1);

  q.push(2);

  q.push(3);

  q.push(4);

  q.push(5);

  q.push(6);

  

  let w = Worker::new_fifo();

  let _ = q.steal_batch_with_limit(&w, 2);

  assert_eq!(w.pop(), Some(1));

  assert_eq!(w.pop(), Some(2));

  assert_eq!(w.pop(), None);

  

  q.push(7);

  q.push(8);

  // Setting a large limit does not guarantee that all elements will be popped. In this case,

  // half of the elements are currently popped, but the number of popped elements is considered

  // an implementation detail that may be changed in the future.

  let _ = q.steal_batch_with_limit(&w, std::usize::MAX);

  assert_eq!(w.len(), 3);

  ```

- <span id="injector-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals a batch of tasks, pushes them into a worker, and pops a task from that worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than some constant limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::{Injector, Steal, Worker};

  

  let q = Injector::new();

  q.push(1);

  q.push(2);

  q.push(3);

  q.push(4);

  

  let w = Worker::new_fifo();

  assert_eq!(q.steal_batch_and_pop(&w), Steal::Success(1));

  assert_eq!(w.pop(), Some(2));

  ```

- <span id="injector-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](#worker), [`Steal`](#steal)

  Steals no more than `limit` of tasks, pushes them into a worker, and pops a task from that worker.

  

  How many tasks exactly will be stolen is not specified. That said, this method will try to

  steal around half of the tasks in the queue, but also not more than the given limit.

  

  # Examples

  

  ```rust

  use crossbeam_deque::{Injector, Steal, Worker};

  

  let q = Injector::new();

  q.push(1);

  q.push(2);

  q.push(3);

  q.push(4);

  q.push(5);

  q.push(6);

  

  let w = Worker::new_fifo();

  assert_eq!(q.steal_batch_with_limit_and_pop(&w, 2), Steal::Success(1));

  assert_eq!(w.pop(), Some(2));

  assert_eq!(w.pop(), None);

  

  q.push(7);

  // Setting a large limit does not guarantee that all elements will be popped. In this case,

  // half of the elements are currently popped, but the number of popped elements is considered

  // an implementation detail that may be changed in the future.

  assert_eq!(q.steal_batch_with_limit_and_pop(&w, std::usize::MAX), Steal::Success(3));

  assert_eq!(w.pop(), Some(4));

  assert_eq!(w.pop(), Some(5));

  assert_eq!(w.pop(), None);

  ```

- <span id="injector-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the queue is empty.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Injector;

  

  let q = Injector::new();

  

  assert!(q.is_empty());

  q.push(1);

  assert!(!q.is_empty());

  ```

- <span id="injector-len"></span>`fn len(&self) -> usize`

  Returns the number of tasks in the queue.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Injector;

  

  let q = Injector::new();

  

  assert_eq!(q.len(), 0);

  q.push(1);

  assert_eq!(q.len(), 1);

  q.push(1);

  assert_eq!(q.len(), 2);

  ```

#### Trait Implementations

##### `impl<T> Any for Injector<T>`

- <span id="injector-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Injector<T>`

- <span id="injector-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Injector<T>`

- <span id="injector-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for Injector<T>`

- <span id="injector-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Injector<T>`

- <span id="injector-default"></span>`fn default() -> Self`

##### `impl<T> Drop for Injector<T>`

- <span id="injector-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Injector<T>`

- <span id="injector-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Injector<T>`

- <span id="injector-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Injector<T>`

- <span id="injector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="injector-pointable-type-init"></span>`type Init = T`

- <span id="injector-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="injector-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="injector-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="injector-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Injector<T>`

##### `impl<T: Send> Sync for Injector<T>`

##### `impl<T, U> TryFrom for Injector<T>`

- <span id="injector-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="injector-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Injector<T>`

- <span id="injector-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="injector-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Flavor`

```rust
enum Flavor {
    Fifo,
    Lifo,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:145-151`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L145-L151)*

Worker queue flavor: FIFO or LIFO.

#### Variants

- **`Fifo`**

  The first-in first-out flavor.

- **`Lifo`**

  The last-in first-out flavor.

#### Trait Implementations

##### `impl Any for Flavor`

- <span id="flavor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flavor`

- <span id="flavor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flavor`

- <span id="flavor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Flavor`

- <span id="flavor-clone"></span>`fn clone(&self) -> Flavor` — [`Flavor`](#flavor)

##### `impl CloneToUninit for Flavor`

- <span id="flavor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Flavor`

##### `impl Debug for Flavor`

- <span id="flavor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Flavor`

##### `impl<T> From for Flavor`

- <span id="flavor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flavor`

- <span id="flavor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Flavor`

- <span id="flavor-partialeq-eq"></span>`fn eq(&self, other: &Flavor) -> bool` — [`Flavor`](#flavor)

##### `impl Pointable for Flavor`

- <span id="flavor-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flavor-pointable-type-init"></span>`type Init = T`

- <span id="flavor-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flavor-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flavor-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flavor-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Flavor`

##### `impl ToOwned for Flavor`

- <span id="flavor-toowned-type-owned"></span>`type Owned = T`

- <span id="flavor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flavor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Flavor`

- <span id="flavor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flavor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flavor`

- <span id="flavor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flavor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Steal<T>`

```rust
enum Steal<T> {
    Empty,
    Success(T),
    Retry,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:2055-2064`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L2055-L2064)*

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

  Returns `true` if the queue was empty at the time of stealing.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Steal::{Empty, Retry, Success};

  

  assert!(!Success(7).is_empty());

  assert!(!Retry::<i32>.is_empty());

  

  assert!(Empty::<i32>.is_empty());

  ```

- <span id="steal-is-success"></span>`fn is_success(&self) -> bool`

  Returns `true` if at least one task was stolen.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Steal::{Empty, Retry, Success};

  

  assert!(!Empty::<i32>.is_success());

  assert!(!Retry::<i32>.is_success());

  

  assert!(Success(7).is_success());

  ```

- <span id="steal-is-retry"></span>`fn is_retry(&self) -> bool`

  Returns `true` if the steal operation needs to be retried.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Steal::{Empty, Retry, Success};

  

  assert!(!Empty::<i32>.is_retry());

  assert!(!Success(7).is_retry());

  

  assert!(Retry::<i32>.is_retry());

  ```

- <span id="steal-success"></span>`fn success(self) -> Option<T>`

  Returns the result of the operation, if successful.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Steal::{Empty, Retry, Success};

  

  assert_eq!(Empty::<i32>.success(), None);

  assert_eq!(Retry::<i32>.success(), None);

  

  assert_eq!(Success(7).success(), Some(7));

  ```

- <span id="steal-or-else"></span>`fn or_else<F>(self, f: F) -> Steal<T>` — [`Steal`](#steal)

  If no task was stolen, attempts another steal operation.

  

  Returns this steal result if it is `Success`. Otherwise, closure `f` is invoked and then:

  

  * If the second steal resulted in `Success`, it is returned.

  * If both steals were unsuccessful but any resulted in `Retry`, then `Retry` is returned.

  * If both resulted in `None`, then `None` is returned.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Steal::{Empty, Retry, Success};

  

  assert_eq!(Success(1).or_else(|| Success(2)), Success(1));

  assert_eq!(Retry.or_else(|| Success(2)), Success(2));

  

  assert_eq!(Retry.or_else(|| Empty), Retry::<i32>);

  assert_eq!(Empty.or_else(|| Retry), Retry::<i32>);

  

  assert_eq!(Empty.or_else(|| Empty), Empty::<i32>);

  ```

#### Trait Implementations

##### `impl<T> Any for Steal<T>`

- <span id="steal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Steal<T>`

- <span id="steal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Steal<T>`

- <span id="steal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Steal<T>`

- <span id="steal-clone"></span>`fn clone(&self) -> Steal<T>` — [`Steal`](#steal)

##### `impl<T> CloneToUninit for Steal<T>`

- <span id="steal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for Steal<T>`

##### `impl<T> Debug for Steal<T>`

- <span id="steal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for Steal<T>`

##### `impl<T> From for Steal<T>`

- <span id="steal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FromIterator for Steal<T>`

- <span id="steal-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Steal<T>` — [`Steal`](#steal)

  Consumes items until a `Success` is found and returns it.

  

  If no `Success` was found, but there was at least one `Retry`, then returns `Retry`.

  Otherwise, `Empty` is returned.

##### `impl<T, U> Into for Steal<T>`

- <span id="steal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for Steal<T>`

- <span id="steal-partialeq-eq"></span>`fn eq(&self, other: &Steal<T>) -> bool` — [`Steal`](#steal)

##### `impl<T> Pointable for Steal<T>`

- <span id="steal-pointable-const-align"></span>`const ALIGN: usize`

- <span id="steal-pointable-type-init"></span>`type Init = T`

- <span id="steal-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="steal-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="steal-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="steal-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> StructuralPartialEq for Steal<T>`

##### `impl<T> ToOwned for Steal<T>`

- <span id="steal-toowned-type-owned"></span>`type Owned = T`

- <span id="steal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="steal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Steal<T>`

- <span id="steal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="steal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Steal<T>`

- <span id="steal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="steal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `MIN_CAP`
```rust
const MIN_CAP: usize = 64usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:16`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L16)*

### `MAX_BATCH`
```rust
const MAX_BATCH: usize = 32usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:18`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L18)*

### `FLUSH_THRESHOLD_BYTES`
```rust
const FLUSH_THRESHOLD_BYTES: usize = 1_024usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:21`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L21)*

### `WRITE`
```rust
const WRITE: usize = 1usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1184`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1184)*

### `READ`
```rust
const READ: usize = 2usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1185`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1185)*

### `DESTROY`
```rust
const DESTROY: usize = 4usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1186`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1186)*

### `LAP`
```rust
const LAP: usize = 64usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1189`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1189)*

### `BLOCK_CAP`
```rust
const BLOCK_CAP: usize = 63usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1191`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1191)*

### `SHIFT`
```rust
const SHIFT: usize = 1usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1193`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1193)*

### `HAS_NEXT`
```rust
const HAS_NEXT: usize = 1usize;
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1195`](../../../.source_1765633015/crossbeam-deque-0.8.6/src/deque.rs#L1195)*

