*[rayon_core](../index.md) / [scope](index.md)*

---

# Module `scope`

Methods for custom fork-join scopes, created by the [`scope()`](../index.md)
and [`in_place_scope()`](../index.md) functions. These are a more flexible alternative to `join()`.


## Contents

- [Structs](#structs)
  - [`Scope`](#scope)
  - [`ScopeFifo`](#scopefifo)
  - [`ScopeBase`](#scopebase)
  - [`ScopePtr`](#scopeptr)
- [Functions](#functions)
  - [`scope`](#scope)
  - [`scope_fifo`](#scope_fifo)
  - [`in_place_scope`](#in_place_scope)
  - [`do_in_place_scope`](#do_in_place_scope)
  - [`get_in_place_thread_registry`](#get_in_place_thread_registry)
  - [`in_place_scope_fifo`](#in_place_scope_fifo)
  - [`do_in_place_scope_fifo`](#do_in_place_scope_fifo)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Scope`](#scope) | struct | Represents a fork-join scope which can be used to spawn any number of tasks. |
| [`ScopeFifo`](#scopefifo) | struct | Represents a fork-join scope which can be used to spawn any number of tasks. |
| [`ScopeBase`](#scopebase) | struct |  |
| [`ScopePtr`](#scopeptr) | struct | Used to capture a scope `&Self` pointer in jobs, without faking a lifetime. |
| [`scope`](#scope) | fn | Creates a "fork-join" scope `s` and invokes the closure with a |
| [`scope_fifo`](#scope_fifo) | fn | Creates a "fork-join" scope `s` with FIFO order, and invokes the |
| [`in_place_scope`](#in_place_scope) | fn | Creates a "fork-join" scope `s` and invokes the closure with a |
| [`do_in_place_scope`](#do_in_place_scope) | fn |  |
| [`get_in_place_thread_registry`](#get_in_place_thread_registry) | fn |  |
| [`in_place_scope_fifo`](#in_place_scope_fifo) | fn | Creates a "fork-join" scope `s` with FIFO order, and invokes the |
| [`do_in_place_scope_fifo`](#do_in_place_scope_fifo) | fn |  |

## Structs

### `Scope<'scope>`

```rust
struct Scope<'scope> {
    base: ScopeBase<'scope>,
}
```

Represents a fork-join scope which can be used to spawn any number of tasks.
See [`scope()`](../index.md) for more information.

#### Implementations

- <span id="scope-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](../registry/index.md), [`Registry`](../registry/index.md)

- <span id="scope-spawn"></span>`fn spawn<BODY>(&self, body: BODY)`

- <span id="scope-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

#### Trait Implementations

##### `impl<'scope> Debug for Scope<'scope>`

- <span id="scope-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for Scope<'scope>`

- <span id="scope-align"></span>`const ALIGN: usize`

- <span id="scope-init"></span>`type Init = T`

- <span id="scope-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scope-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scope-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scope-drop"></span>`unsafe fn drop(ptr: usize)`

### `ScopeFifo<'scope>`

```rust
struct ScopeFifo<'scope> {
    base: ScopeBase<'scope>,
    fifos: Vec<crate::job::JobFifo>,
}
```

Represents a fork-join scope which can be used to spawn any number of tasks.
Those spawned from the same thread are prioritized in relative FIFO order.
See [`scope_fifo()`](../index.md) for more information.

#### Implementations

- <span id="scopefifo-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](../registry/index.md), [`Registry`](../registry/index.md)

- <span id="scopefifo-spawn-fifo"></span>`fn spawn_fifo<BODY>(&self, body: BODY)`

- <span id="scopefifo-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

#### Trait Implementations

##### `impl<'scope> Debug for ScopeFifo<'scope>`

- <span id="scopefifo-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for ScopeFifo<'scope>`

- <span id="scopefifo-align"></span>`const ALIGN: usize`

- <span id="scopefifo-init"></span>`type Init = T`

- <span id="scopefifo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopefifo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopefifo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopefifo-drop"></span>`unsafe fn drop(ptr: usize)`

### `ScopeBase<'scope>`

```rust
struct ScopeBase<'scope> {
    registry: std::sync::Arc<crate::registry::Registry>,
    panic: std::sync::atomic::AtomicPtr<Box<dyn Any + Send>>,
    job_completed_latch: crate::latch::CountLatch,
    marker: std::marker::PhantomData<Box<dyn FnOnce(&Scope<'scope>) + Send + Sync>>,
}
```

#### Fields

- **`registry`**: `std::sync::Arc<crate::registry::Registry>`

  thread registry where `scope()` was executed or where `in_place_scope()`
  should spawn jobs.

- **`panic`**: `std::sync::atomic::AtomicPtr<Box<dyn Any + Send>>`

  if some job panicked, the error is stored here; it will be
  propagated to the one who created the scope

- **`job_completed_latch`**: `crate::latch::CountLatch`

  latch to track job counts

- **`marker`**: `std::marker::PhantomData<Box<dyn FnOnce(&Scope<'scope>) + Send + Sync>>`

  You can think of a scope as containing a list of closures to execute,
  all of which outlive `'scope`.  They're not actually required to be
  `Sync`, but it's still safe to let the `Scope` implement `Sync` because
  the closures are only *moved* across threads to be executed.

#### Implementations

- <span id="scopebase-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](../registry/index.md), [`Registry`](../registry/index.md)

- <span id="scopebase-heap-job-ref"></span>`fn heap_job_ref<FUNC>(&self, job: Box<HeapJob<FUNC>>) -> JobRef` — [`HeapJob`](../job/index.md), [`JobRef`](../job/index.md)

- <span id="scopebase-inject-broadcast"></span>`fn inject_broadcast<FUNC>(&self, job: Arc<ArcJob<FUNC>>)` — [`ArcJob`](../job/index.md)

- <span id="scopebase-complete"></span>`fn complete<FUNC, R>(&self, owner: Option<&WorkerThread>, func: FUNC) -> R` — [`WorkerThread`](../registry/index.md)

- <span id="scopebase-execute-job"></span>`unsafe fn execute_job<FUNC>(this: *const Self, func: FUNC)`

- <span id="scopebase-execute-job-closure"></span>`unsafe fn execute_job_closure<FUNC, R>(this: *const Self, func: FUNC) -> Option<R>`

- <span id="scopebase-job-panicked"></span>`fn job_panicked(&self, err: Box<dyn Any + Send>)`

- <span id="scopebase-maybe-propagate-panic"></span>`fn maybe_propagate_panic(&self)`

#### Trait Implementations

##### `impl<T> Pointable for ScopeBase<'scope>`

- <span id="scopebase-align"></span>`const ALIGN: usize`

- <span id="scopebase-init"></span>`type Init = T`

- <span id="scopebase-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopebase-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopebase-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopebase-drop"></span>`unsafe fn drop(ptr: usize)`

### `ScopePtr<T>`

```rust
struct ScopePtr<T>(*const T);
```

Used to capture a scope `&Self` pointer in jobs, without faking a lifetime.

Unsafe code is still required to dereference the pointer, but that's fine in
scope jobs that are guaranteed to execute before the scope ends.

#### Implementations

- <span id="scopeptr-as-ref"></span>`unsafe fn as_ref(&self) -> &T`

#### Trait Implementations

##### `impl<T> Pointable for ScopePtr<T>`

- <span id="scopeptr-align"></span>`const ALIGN: usize`

- <span id="scopeptr-init"></span>`type Init = T`

- <span id="scopeptr-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopeptr-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopeptr-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopeptr-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Sync> Send for ScopePtr<T>`

##### `impl<T: Sync> Sync for ScopePtr<T>`

## Functions

### `scope`

```rust
fn scope<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&Scope<'scope>) -> R + Send,
    R: Send
```

Creates a "fork-join" scope `s` and invokes the closure with a
reference to `s`. This closure can then spawn asynchronous tasks
into `s`. Those tasks may run asynchronously with respect to the
closure; they may themselves spawn additional tasks into `s`. When
the closure returns, it will block until all tasks that have been
spawned into `s` complete.

`scope()` is a more flexible building block compared to `join()`,
since a loop can be used to spawn any number of tasks without
recursing. However, that flexibility comes at a performance price:
tasks spawned using `scope()` must be allocated onto the heap,
whereas `join()` can make exclusive use of the stack. **Prefer
`join()` (or, even better, parallel iterators) where possible.**

# Example

The Rayon `join()` function launches two closures and waits for them
to stop. One could implement `join()` using a scope like so, although
it would be less efficient than the real implementation:

```rust
use rayon_core as rayon;
pub fn join<A,B,RA,RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where A: FnOnce() -> RA + Send,
          B: FnOnce() -> RB + Send,
          RA: Send,
          RB: Send,
{
    let mut result_a: Option<RA> = None;
    let mut result_b: Option<RB> = None;
    rayon::scope(|s| {
        s.spawn(|_| result_a = Some(oper_a()));
        s.spawn(|_| result_b = Some(oper_b()));
    });
    (result_a.unwrap(), result_b.unwrap())
}
```

# A note on threading

The closure given to `scope()` executes in the Rayon thread pool,
as do those given to `spawn()`. This means that you can't access
thread-local variables (well, you can, but they may have
unexpected values).

# Task execution

Task execution potentially starts as soon as `spawn()` is called.
The task will end sometime before `scope()` returns. Note that the
*closure* given to scope may return much earlier. In general
the lifetime of a scope created like `scope(body)` goes something like this:

- Scope begins when `scope(body)` is called
- Scope body `body()` is invoked
    - Scope tasks may be spawned
- Scope body returns
- Scope tasks execute, possibly spawning more tasks
- Once all tasks are done, scope ends and `scope()` returns

To see how and when tasks are joined, consider this example:

```rust
use rayon_core as rayon;
// point start
rayon::scope(|s| {
    s.spawn(|s| { // task s.1
        s.spawn(|s| { // task s.1.1
            rayon::scope(|t| {
                t.spawn(|_| ()); // task t.1
                t.spawn(|_| ()); // task t.2
            });
        });
    });
    s.spawn(|s| { // task s.2
    });
    // point mid
});
// point end
```

The various tasks that are run will execute roughly like so:

```notrust
| (start)
|
| (scope `s` created)
+-----------------------------------------------+ (task s.2)
+-------+ (task s.1)                            |
|       |                                       |
|       +---+ (task s.1.1)                      |
|       |   |                                   |
|       |   | (scope `t` created)               |
|       |   +----------------+ (task t.2)       |
|       |   +---+ (task t.1) |                  |
| (mid) |   |   |            |                  |
:       |   + <-+------------+ (scope `t` ends) |
:       |   |                                   |
|<------+---+-----------------------------------+ (scope `s` ends)
|
| (end)
```

The point here is that everything spawned into scope `s` will
terminate (at latest) at the same point -- right before the
original call to `rayon::scope` returns. This includes new
subtasks created by other subtasks (e.g., task `s.1.1`). If a new
scope is created (such as `t`), the things spawned into that scope
will be joined before that scope returns, which in turn occurs
before the creating task (task `s.1.1` in this case) finishes.

There is no guaranteed order of execution for spawns in a scope,
given that other threads may steal tasks at any time. However, they
are generally prioritized in a LIFO order on the thread from which
they were spawned. So in this example, absent any stealing, we can
expect `s.2` to execute before `s.1`, and `t.2` before `t.1`. Other
threads always steal from the other end of the deque, like FIFO
order.  The idea is that "recent" tasks are most likely to be fresh
in the local CPU's cache, while other threads can steal older
"stale" tasks.  For an alternate approach, consider
[`scope_fifo()`](../index.md) instead.

# Accessing stack data

In general, spawned tasks may access stack data in place that
outlives the scope itself. Other data must be fully owned by the
spawned task.

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    s.spawn(|_| {
        // We can access `ok` because outlives the scope `s`.
        println!("ok: {:?}", ok);

        // If we just try to use `bad` here, the closure will borrow `bad`
        // (because we are just printing it out, and that only requires a
        // borrow), which will result in a compilation error. Read on
        // for options.
        // println!("bad: {:?}", bad);
   });
});
```

As the comments example above suggest, to reference `bad` we must
take ownership of it. One way to do this is to detach the closure
from the surrounding stack frame, using the `move` keyword. This
will cause it to take ownership of *all* the variables it touches,
in this case including both `ok` *and* `bad`:

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    s.spawn(move |_| {
        println!("ok: {:?}", ok);
        println!("bad: {:?}", bad);
    });

    // That closure is fine, but now we can't use `ok` anywhere else,
    // since it is owned by the previous task:
    // s.spawn(|_| println!("ok: {:?}", ok));
});
```

While this works, it could be a problem if we want to use `ok` elsewhere.
There are two choices. We can keep the closure as a `move` closure, but
instead of referencing the variable `ok`, we create a shadowed variable that
is a borrow of `ok` and capture *that*:

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    let ok: &Vec<i32> = &ok; // shadow the original `ok`
    s.spawn(move |_| {
        println!("ok: {:?}", ok); // captures the shadowed version
        println!("bad: {:?}", bad);
    });

    // Now we too can use the shadowed `ok`, since `&Vec<i32>` references
    // can be shared freely. Note that we need a `move` closure here though,
    // because otherwise we'd be trying to borrow the shadowed `ok`,
    // and that doesn't outlive `scope`.
    s.spawn(move |_| println!("ok: {:?}", ok));
});
```

Another option is not to use the `move` keyword but instead to take ownership
of individual variables:

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    s.spawn(|_| {
        // Transfer ownership of `bad` into a local variable (also named `bad`).
        // This will force the closure to take ownership of `bad` from the environment.
        let bad = bad;
        println!("ok: {:?}", ok); // `ok` is only borrowed.
        println!("bad: {:?}", bad); // refers to our local variable, above.
    });

    s.spawn(|_| println!("ok: {:?}", ok)); // we too can borrow `ok`
});
```

# Panics

If a panic occurs, either in the closure given to `scope()` or in
any of the spawned jobs, that panic will be propagated and the
call to `scope()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn()`, it will
execute, even if the spawning task should later panic. `scope()`
returns once all spawned jobs have completed, and any panics are
propagated at that point.

### `scope_fifo`

```rust
fn scope_fifo<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&ScopeFifo<'scope>) -> R + Send,
    R: Send
```

Creates a "fork-join" scope `s` with FIFO order, and invokes the
closure with a reference to `s`. This closure can then spawn
asynchronous tasks into `s`. Those tasks may run asynchronously with
respect to the closure; they may themselves spawn additional tasks
into `s`. When the closure returns, it will block until all tasks
that have been spawned into `s` complete.

# Task execution

Tasks in a `scope_fifo()` run similarly to [`scope()`](../index.md), but there's a
difference in the order of execution. Consider a similar example:

```rust
use rayon_core as rayon;
// point start
rayon::scope_fifo(|s| {
    s.spawn_fifo(|s| { // task s.1
        s.spawn_fifo(|s| { // task s.1.1
            rayon::scope_fifo(|t| {
                t.spawn_fifo(|_| ()); // task t.1
                t.spawn_fifo(|_| ()); // task t.2
            });
        });
    });
    s.spawn_fifo(|s| { // task s.2
    });
    // point mid
});
// point end
```

The various tasks that are run will execute roughly like so:

```notrust
| (start)
|
| (FIFO scope `s` created)
+--------------------+ (task s.1)
+-------+ (task s.2) |
|       |            +---+ (task s.1.1)
|       |            |   |
|       |            |   | (FIFO scope `t` created)
|       |            |   +----------------+ (task t.1)
|       |            |   +---+ (task t.2) |
| (mid) |            |   |   |            |
:       |            |   + <-+------------+ (scope `t` ends)
:       |            |   |
|<------+------------+---+ (scope `s` ends)
|
| (end)
```

Under `scope_fifo()`, the spawns are prioritized in a FIFO order on
the thread from which they were spawned, as opposed to `scope()`'s
LIFO.  So in this example, we can expect `s.1` to execute before
`s.2`, and `t.1` before `t.2`. Other threads also steal tasks in
FIFO order, as usual. Overall, this has roughly the same order as
the now-deprecated `breadth_first` option, except the effect is
isolated to a particular scope. If spawns are intermingled from any
combination of `scope()` and `scope_fifo()`, or from different
threads, their order is only specified with respect to spawns in the
same scope and thread.

For more details on this design, see Rayon [RFC #1].


# Panics

If a panic occurs, either in the closure given to `scope_fifo()` or
in any of the spawned jobs, that panic will be propagated and the
call to `scope_fifo()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn_fifo()`, it
will execute, even if the spawning task should later panic.
`scope_fifo()` returns once all spawned jobs have completed, and any
panics are propagated at that point.

### `in_place_scope`

```rust
fn in_place_scope<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&Scope<'scope>) -> R
```

Creates a "fork-join" scope `s` and invokes the closure with a
reference to `s`. This closure can then spawn asynchronous tasks
into `s`. Those tasks may run asynchronously with respect to the
closure; they may themselves spawn additional tasks into `s`. When
the closure returns, it will block until all tasks that have been
spawned into `s` complete.

This is just like `scope()` except the closure runs on the same thread
that calls `in_place_scope()`. Only work that it spawns runs in the
thread pool.

# Panics

If a panic occurs, either in the closure given to `in_place_scope()` or in
any of the spawned jobs, that panic will be propagated and the
call to `in_place_scope()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn()`, it will
execute, even if the spawning task should later panic. `in_place_scope()`
returns once all spawned jobs have completed, and any panics are
propagated at that point.

### `do_in_place_scope`

```rust
fn do_in_place_scope<'scope, OP, R>(registry: Option<&std::sync::Arc<crate::registry::Registry>>, op: OP) -> R
where
    OP: FnOnce(&Scope<'scope>) -> R
```

### `get_in_place_thread_registry`

```rust
fn get_in_place_thread_registry(registry: Option<&std::sync::Arc<crate::registry::Registry>>) -> (Option<&crate::registry::WorkerThread>, Option<&std::sync::Arc<crate::registry::Registry>>)
```

### `in_place_scope_fifo`

```rust
fn in_place_scope_fifo<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&ScopeFifo<'scope>) -> R
```

Creates a "fork-join" scope `s` with FIFO order, and invokes the
closure with a reference to `s`. This closure can then spawn
asynchronous tasks into `s`. Those tasks may run asynchronously with
respect to the closure; they may themselves spawn additional tasks
into `s`. When the closure returns, it will block until all tasks
that have been spawned into `s` complete.

This is just like `scope_fifo()` except the closure runs on the same thread
that calls `in_place_scope_fifo()`. Only work that it spawns runs in the
thread pool.

# Panics

If a panic occurs, either in the closure given to `in_place_scope_fifo()` or in
any of the spawned jobs, that panic will be propagated and the
call to `in_place_scope_fifo()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn_fifo()`, it will
execute, even if the spawning task should later panic. `in_place_scope_fifo()`
returns once all spawned jobs have completed, and any panics are
propagated at that point.

### `do_in_place_scope_fifo`

```rust
fn do_in_place_scope_fifo<'scope, OP, R>(registry: Option<&std::sync::Arc<crate::registry::Registry>>, op: OP) -> R
where
    OP: FnOnce(&ScopeFifo<'scope>) -> R
```

