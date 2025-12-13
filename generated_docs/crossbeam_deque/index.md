# Crate `crossbeam_deque`

Concurrent work-stealing deques.

These data structures are most commonly used in work-stealing schedulers. The typical setup
involves a number of threads, each having its own FIFO or LIFO queue (*worker*). There is also
one global FIFO queue (*injector*) and a list of references to *worker* queues that are able to
steal tasks (*stealers*).

We spawn a new task onto the scheduler by pushing it into the *injector* queue. Each worker
thread waits in a loop until it finds the next task to run and then runs it. To find a task, it
first looks into its local *worker* queue, and then into the *injector* and *stealers*.

# Queues

[`Injector`](deque/index.md) is a FIFO queue, where tasks are pushed and stolen from opposite ends. It is
shared among threads and is usually the entry point for new tasks.

[`Worker`](deque/index.md) has two constructors:

* `new_fifo()` - Creates a FIFO queue, in which tasks are pushed and popped from opposite
  ends.
* `new_lifo()` - Creates a LIFO queue, in which tasks are pushed and popped from the same
  end.

Each [`Worker`](deque/index.md) is owned by a single thread and supports only push and pop operations.

Method `stealer()` creates a [`Stealer`](deque/index.md) that may be shared among threads and can only steal
tasks from its [`Worker`](deque/index.md). Tasks are stolen from the end opposite to where they get pushed.

# Stealing

Steal operations come in three flavors:

1. `steal()` - Steals one task.
2. `steal_batch()` - Steals a batch of tasks and moves them into another worker.
3. `steal_batch_and_pop()` - Steals a batch of tasks, moves them into another queue, and pops
   one task from that worker.

In contrast to push and pop operations, stealing can spuriously fail with [`Steal::Retry`](#stealretry), in
which case the steal operation needs to be retried.

# Examples

Suppose a thread in a work-stealing scheduler is idle and looking for the next task to run. To
find an available task, it might do the following:

1. Try popping one task from the local worker queue.
2. Try stealing a batch of tasks from the global injector queue.
3. Try stealing one task from another thread using the stealer list.

An implementation of this work-stealing strategy:

```rust
use crossbeam_deque::{Injector, Stealer, Worker};
use std::iter;

fn find_task<T>(
    local: &Worker<T>,
    global: &Injector<T>,
    stealers: &[Stealer<T>],
) -> Option<T> {
    // Pop a task from the local queue, if not empty.
    local.pop().or_else(|| {
        // Otherwise, we need to look for a task elsewhere.
        iter::repeat_with(|| {
            // Try stealing a batch of tasks from the global queue.
            global.steal_batch_and_pop(local)
                // Or try stealing a task from one of the other threads.
                .or_else(|| stealers.iter().map(|s| s.steal()).collect())
        })
        // Loop while no task was stolen and any steal operation needs to be retried.
        .find(|s| !s.is_retry())
        // Extract the stolen task, if there is one.
        .and_then(|s| s.success())
    })
}
```







## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`deque`](#deque) | mod |  |
| [`Injector`](#injector) | struct |  |
| [`Stealer`](#stealer) | struct |  |
| [`Worker`](#worker) | struct |  |
| [`Steal`](#steal) | enum |  |

## Modules

- [`deque`](deque/index.md)

## Structs

### `Injector<T>`

```rust
struct Injector<T> {
    head: crossbeam_utils::CachePadded<Position<T>>,
    tail: crossbeam_utils::CachePadded<Position<T>>,
    _marker: std::marker::PhantomData<T>,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1315-1324`](../../.source_1765521767/crossbeam-deque-0.8.6/src/deque.rs#L1315-L1324)*

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

- <span id="injector-new"></span>`fn new() -> Injector<T>` — [`Injector`](deque/index.md#injector)

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

- <span id="injector-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](deque/index.md#steal)

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

- <span id="injector-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="injector-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="injector-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="injector-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

### `Stealer<T>`

```rust
struct Stealer<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    flavor: Flavor,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:566-572`](../../.source_1765521767/crossbeam-deque-0.8.6/src/deque.rs#L566-L572)*

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

- <span id="stealer-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](deque/index.md#steal)

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

- <span id="stealer-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="stealer-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="stealer-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="stealer-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="stealer-clone"></span>`fn clone(&self) -> Stealer<T>` — [`Stealer`](deque/index.md#stealer)

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

### `Worker<T>`

```rust
struct Worker<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    buffer: std::cell::Cell<Buffer<T>>,
    flavor: Flavor,
    _marker: std::marker::PhantomData<*mut ()>,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:193-205`](../../.source_1765521767/crossbeam-deque-0.8.6/src/deque.rs#L193-L205)*

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

- <span id="worker-new-fifo"></span>`fn new_fifo() -> Worker<T>` — [`Worker`](deque/index.md#worker)

  Creates a FIFO worker queue.

  

  Tasks are pushed and popped from opposite ends.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::<i32>::new_fifo();

  ```

- <span id="worker-new-lifo"></span>`fn new_lifo() -> Worker<T>` — [`Worker`](deque/index.md#worker)

  Creates a LIFO worker queue.

  

  Tasks are pushed and popped from the same end.

  

  # Examples

  

  ```rust

  use crossbeam_deque::Worker;

  

  let w = Worker::<i32>::new_lifo();

  ```

- <span id="worker-stealer"></span>`fn stealer(&self) -> Stealer<T>` — [`Stealer`](deque/index.md#stealer)

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

## Enums

### `Steal<T>`

```rust
enum Steal<T> {
    Empty,
    Success(T),
    Retry,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:2055-2064`](../../.source_1765521767/crossbeam-deque-0.8.6/src/deque.rs#L2055-L2064)*

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

- <span id="steal-or-else"></span>`fn or_else<F>(self, f: F) -> Steal<T>` — [`Steal`](deque/index.md#steal)

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

- <span id="steal-clone"></span>`fn clone(&self) -> Steal<T>` — [`Steal`](deque/index.md#steal)

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

- <span id="steal-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Steal<T>` — [`Steal`](deque/index.md#steal)

  Consumes items until a `Success` is found and returns it.

  

  If no `Success` was found, but there was at least one `Retry`, then returns `Retry`.

  Otherwise, `Empty` is returned.

##### `impl<T, U> Into for Steal<T>`

- <span id="steal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for Steal<T>`

- <span id="steal-partialeq-eq"></span>`fn eq(&self, other: &Steal<T>) -> bool` — [`Steal`](deque/index.md#steal)

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

