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

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:1315-1324`](../../.source_1765210505/crossbeam-deque-0.8.6/src/deque.rs#L1315-L1324)*

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

- <span id="injector-push"></span>`fn push(&self, task: T)`

- <span id="injector-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](deque/index.md#steal)

- <span id="injector-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

- <span id="injector-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

- <span id="injector-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

- <span id="injector-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

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

- <span id="injector-const-align"></span>`const ALIGN: usize`

- <span id="injector-type-init"></span>`type Init = T`

- <span id="injector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="injector-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="injector-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="injector-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Injector<T>`

##### `impl<T: Send> Sync for Injector<T>`

### `Stealer<T>`

```rust
struct Stealer<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    flavor: Flavor,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:566-572`](../../.source_1765210505/crossbeam-deque-0.8.6/src/deque.rs#L566-L572)*

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

- <span id="stealer-steal"></span>`fn steal(&self) -> Steal<T>` — [`Steal`](deque/index.md#steal)

- <span id="stealer-steal-batch"></span>`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

- <span id="stealer-steal-batch-with-limit"></span>`fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

- <span id="stealer-steal-batch-and-pop"></span>`fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

- <span id="stealer-steal-batch-with-limit-and-pop"></span>`fn steal_batch_with_limit_and_pop(&self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](deque/index.md#worker), [`Steal`](deque/index.md#steal)

#### Trait Implementations

##### `impl<T> Clone for Stealer<T>`

- <span id="stealer-clone"></span>`fn clone(&self) -> Stealer<T>` — [`Stealer`](deque/index.md#stealer)

##### `impl<T> Debug for Stealer<T>`

- <span id="stealer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for Stealer<T>`

- <span id="stealer-const-align"></span>`const ALIGN: usize`

- <span id="stealer-type-init"></span>`type Init = T`

- <span id="stealer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stealer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stealer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stealer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Stealer<T>`

##### `impl<T: Send> Sync for Stealer<T>`

### `Worker<T>`

```rust
struct Worker<T> {
    inner: std::sync::Arc<crossbeam_utils::CachePadded<Inner<T>>>,
    buffer: std::cell::Cell<Buffer<T>>,
    flavor: Flavor,
    _marker: std::marker::PhantomData<*mut ()>,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:193-205`](../../.source_1765210505/crossbeam-deque-0.8.6/src/deque.rs#L193-L205)*

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

- <span id="worker-new-lifo"></span>`fn new_lifo() -> Worker<T>` — [`Worker`](deque/index.md#worker)

- <span id="worker-stealer"></span>`fn stealer(&self) -> Stealer<T>` — [`Stealer`](deque/index.md#stealer)

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

- <span id="worker-const-align"></span>`const ALIGN: usize`

- <span id="worker-type-init"></span>`type Init = T`

- <span id="worker-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="worker-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="worker-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="worker-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for Worker<T>`

## Enums

### `Steal<T>`

```rust
enum Steal<T> {
    Empty,
    Success(T),
    Retry,
}
```

*Defined in [`crossbeam-deque-0.8.6/src/deque.rs:2055-2064`](../../.source_1765210505/crossbeam-deque-0.8.6/src/deque.rs#L2055-L2064)*

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

- <span id="steal-or-else"></span>`fn or_else<F>(self, f: F) -> Steal<T>` — [`Steal`](deque/index.md#steal)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Steal<T>`

- <span id="steal-clone"></span>`fn clone(&self) -> Steal<T>` — [`Steal`](deque/index.md#steal)

##### `impl<T: marker::Copy> Copy for Steal<T>`

##### `impl<T> Debug for Steal<T>`

- <span id="steal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for Steal<T>`

##### `impl<T> FromIterator for Steal<T>`

- <span id="steal-from-iter"></span>`fn from_iter<I>(iter: I) -> Steal<T>` — [`Steal`](deque/index.md#steal)

##### `impl<T: cmp::PartialEq> PartialEq for Steal<T>`

- <span id="steal-eq"></span>`fn eq(&self, other: &Steal<T>) -> bool` — [`Steal`](deque/index.md#steal)

##### `impl<T> Pointable for Steal<T>`

- <span id="steal-const-align"></span>`const ALIGN: usize`

- <span id="steal-type-init"></span>`type Init = T`

- <span id="steal-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="steal-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="steal-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="steal-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> StructuralPartialEq for Steal<T>`

