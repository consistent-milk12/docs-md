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

* [`new_fifo()`](#new-fifo) - Creates a FIFO queue, in which tasks are pushed and popped from opposite
  ends.
* [`new_lifo()`](#new-lifo) - Creates a LIFO queue, in which tasks are pushed and popped from the same
  end.

Each [`Worker`](deque/index.md) is owned by a single thread and supports only push and pop operations.

Method [`stealer()`](#stealer) creates a [`Stealer`](deque/index.md) that may be shared among threads and can only steal
tasks from its [`Worker`](deque/index.md). Tasks are stolen from the end opposite to where they get pushed.

# Stealing

Steal operations come in three flavors:

1. [`steal()`](#steal) - Steals one task.
2. [`steal_batch()`](#steal-batch) - Steals a batch of tasks and moves them into another worker.
3. [`steal_batch_and_pop()`](#steal-batch-and-pop) - Steals a batch of tasks, moves them into another queue, and pops
   one task from that worker.

In contrast to push and pop operations, stealing can spuriously fail with `Steal::Retry`, in
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







## Structs

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

- `fn new() -> Injector<T>` — [`Injector`](../deque/index.md)

- `fn push(self: &Self, task: T)`

- `fn steal(self: &Self) -> Steal<T>` — [`Steal`](../deque/index.md)

- `fn steal_batch(self: &Self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

- `fn steal_batch_with_limit(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

- `fn steal_batch_and_pop(self: &Self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

- `fn steal_batch_with_limit_and_pop(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T>`

- `fn default() -> Self`

##### `impl Drop<T>`

- `fn drop(self: &mut Self)`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Send<T: Send>`

##### `impl Sync<T: Send>`

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

- `fn steal(self: &Self) -> Steal<T>` — [`Steal`](../deque/index.md)

- `fn steal_batch(self: &Self, dest: &Worker<T>) -> Steal<()>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

- `fn steal_batch_with_limit(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<()>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

- `fn steal_batch_and_pop(self: &Self, dest: &Worker<T>) -> Steal<T>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

- `fn steal_batch_with_limit_and_pop(self: &Self, dest: &Worker<T>, limit: usize) -> Steal<T>` — [`Worker`](../deque/index.md), [`Steal`](../deque/index.md)

#### Trait Implementations

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Stealer<T>` — [`Stealer`](../deque/index.md)

##### `impl Debug<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Send<T: Send>`

##### `impl Sync<T: Send>`

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

- `fn new_fifo() -> Worker<T>` — [`Worker`](../deque/index.md)

- `fn new_lifo() -> Worker<T>` — [`Worker`](../deque/index.md)

- `fn stealer(self: &Self) -> Stealer<T>` — [`Stealer`](../deque/index.md)

- `unsafe fn resize(self: &Self, new_cap: usize)`

- `fn reserve(self: &Self, reserve_cap: usize)`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn push(self: &Self, task: T)`

- `fn pop(self: &Self) -> Option<T>`

#### Trait Implementations

##### `impl Debug<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Send<T: Send>`

## Enums

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

- `fn or_else<F>(self: Self, f: F) -> Steal<T>` — [`Steal`](../deque/index.md)

#### Trait Implementations

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Steal<T>` — [`Steal`](../deque/index.md)

##### `impl Copy<T: $crate::marker::Copy>`

##### `impl Debug<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq<T: $crate::cmp::Eq>`

##### `impl FromIterator<T>`

- `fn from_iter<I>(iter: I) -> Steal<T>` — [`Steal`](../deque/index.md)

##### `impl PartialEq<T: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &Steal<T>) -> bool` — [`Steal`](../deque/index.md)

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq<T>`

