*[rayon_core](../index.md) / [scope](index.md)*

---

# Module `scope`

Methods for custom fork-join scopes, created by the [`scope()`](#scope)
and [`in_place_scope()`](#in-place-scope) functions. These are a more flexible alternative to `join()`.


## Contents

- [Structs](#structs)
  - [`Scope`](#scope)
  - [`ScopeFifo`](#scopefifo)
  - [`ScopeBase`](#scopebase)
  - [`ScopePtr`](#scopeptr)
- [Functions](#functions)
  - [`scope`](#scope)
  - [`scope_fifo`](#scope-fifo)
  - [`in_place_scope`](#in-place-scope)
  - [`do_in_place_scope`](#do-in-place-scope)
  - [`get_in_place_thread_registry`](#get-in-place-thread-registry)
  - [`in_place_scope_fifo`](#in-place-scope-fifo)
  - [`do_in_place_scope_fifo`](#do-in-place-scope-fifo)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Scope`](#scope) | struct | Represents a fork-join scope which can be used to spawn any number of tasks. |
| [`ScopeFifo`](#scopefifo) | struct | Represents a fork-join scope which can be used to spawn any number of tasks. |
| [`ScopeBase`](#scopebase) | struct |  |
| [`ScopePtr`](#scopeptr) | struct | Used to capture a scope `&Self` pointer in jobs, without faking a lifetime. |
| [`scope`](#scope) | fn | Creates a "fork-join" scope `s` and invokes the closure with a reference to `s`. |
| [`scope_fifo`](#scope-fifo) | fn | Creates a "fork-join" scope `s` with FIFO order, and invokes the closure with a reference to `s`. |
| [`in_place_scope`](#in-place-scope) | fn | Creates a "fork-join" scope `s` and invokes the closure with a reference to `s`. |
| [`do_in_place_scope`](#do-in-place-scope) | fn |  |
| [`get_in_place_thread_registry`](#get-in-place-thread-registry) | fn |  |
| [`in_place_scope_fifo`](#in-place-scope-fifo) | fn | Creates a "fork-join" scope `s` with FIFO order, and invokes the closure with a reference to `s`. |
| [`do_in_place_scope_fifo`](#do-in-place-scope-fifo) | fn |  |

## Structs

### `Scope<'scope>`

```rust
struct Scope<'scope> {
    base: ScopeBase<'scope>,
}
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:24-26`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L24-L26)*

Represents a fork-join scope which can be used to spawn any number of tasks.
See [`scope()`](#scope) for more information.

#### Implementations

- <span id="scope-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](../registry/index.md#workerthread), [`Registry`](../registry/index.md#registry)

- <span id="scope-spawn"></span>`fn spawn<BODY>(&self, body: BODY)`

  Spawns a job into the fork-join scope `self`. This job will

  execute sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its

  own reference to the scope `self` as argument. This can be

  used to inject new jobs into `self`.

  

  # Returns

  

  Nothing. The spawned closures cannot pass back values to the

  caller directly, though they can write to local variables on

  the stack (if those variables outlive the scope) or

  communicate through shared channels.

  

  (The intention is to eventually integrate with Rust futures to

  support spawns of functions that compute a value.)

  

  # Examples

  

  ```rust

  use rayon_core as rayon;

  let mut value_a = None;

  let mut value_b = None;

  let mut value_c = None;

  rayon::scope(|s| {

      s.spawn(|s1| {

            // ^ this is the same scope as `s`; this handle `s1`

            //   is intended for use by the spawned task,

            //   since scope handles cannot cross thread boundaries.

  

          value_a = Some(22);

  

          // the scope `s` will not end until all these tasks are done

          s1.spawn(|_| {

              value_b = Some(44);

          });

      });

  

      s.spawn(|_| {

          value_c = Some(66);

      });

  });

  assert_eq!(value_a, Some(22));

  assert_eq!(value_b, Some(44));

  assert_eq!(value_c, Some(66));

  ```

  

  # See also

  

  The [`scope` function] has more extensive documentation about

  task spawning.

- <span id="scope-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

  Spawns a job into every thread of the fork-join scope `self`. This job will

  execute on each thread sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its own reference

  to the scope `self` as argument, as well as a `BroadcastContext`.

#### Trait Implementations

##### `impl Any for Scope<'scope>`

- <span id="scope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Scope<'scope>`

- <span id="scope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Scope<'scope>`

- <span id="scope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Scope<'scope>`

- <span id="scope-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Scope<'scope>`

- <span id="scope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Scope<'scope>`

- <span id="scope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Scope<'scope>`

- <span id="scope-pointable-const-align"></span>`const ALIGN: usize`

- <span id="scope-pointable-type-init"></span>`type Init = T`

- <span id="scope-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scope-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scope-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scope-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Scope<'scope>`

- <span id="scope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Scope<'scope>`

- <span id="scope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScopeFifo<'scope>`

```rust
struct ScopeFifo<'scope> {
    base: ScopeBase<'scope>,
    fifos: Vec<crate::job::JobFifo>,
}
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:31-34`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L31-L34)*

Represents a fork-join scope which can be used to spawn any number of tasks.
Those spawned from the same thread are prioritized in relative FIFO order.
See [`scope_fifo()`](#scope-fifo) for more information.

#### Implementations

- <span id="scopefifo-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](../registry/index.md#workerthread), [`Registry`](../registry/index.md#registry)

- <span id="scopefifo-spawn-fifo"></span>`fn spawn_fifo<BODY>(&self, body: BODY)`

  Spawns a job into the fork-join scope `self`. This job will

  execute sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its

  own reference to the scope `self` as argument. This can be

  used to inject new jobs into `self`.

  

  # See also

  

  This method is akin to `Scope::spawn()`, but with a FIFO

  priority.  The [`scope_fifo` function] has more details about

  this distinction.

- <span id="scopefifo-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

  Spawns a job into every thread of the fork-join scope `self`. This job will

  execute on each thread sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its own reference

  to the scope `self` as argument, as well as a `BroadcastContext`.

#### Trait Implementations

##### `impl Any for ScopeFifo<'scope>`

- <span id="scopefifo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScopeFifo<'scope>`

- <span id="scopefifo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScopeFifo<'scope>`

- <span id="scopefifo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ScopeFifo<'scope>`

- <span id="scopefifo-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ScopeFifo<'scope>`

- <span id="scopefifo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScopeFifo<'scope>`

- <span id="scopefifo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ScopeFifo<'scope>`

- <span id="scopefifo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="scopefifo-pointable-type-init"></span>`type Init = T`

- <span id="scopefifo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopefifo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopefifo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopefifo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ScopeFifo<'scope>`

- <span id="scopefifo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scopefifo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScopeFifo<'scope>`

- <span id="scopefifo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scopefifo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScopeBase<'scope>`

```rust
struct ScopeBase<'scope> {
    registry: std::sync::Arc<crate::registry::Registry>,
    panic: std::sync::atomic::AtomicPtr<Box<dyn Any + Send>>,
    job_completed_latch: crate::latch::CountLatch,
    marker: std::marker::PhantomData<Box<dyn FnOnce(&Scope<'scope>) + Send + Sync>>,
}
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:36-54`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L36-L54)*

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

- <span id="scopebase-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](../registry/index.md#workerthread), [`Registry`](../registry/index.md#registry)

  Creates the base of a new scope for the given registry

- <span id="scopebase-heap-job-ref"></span>`fn heap_job_ref<FUNC>(&self, job: Box<HeapJob<FUNC>>) -> JobRef` — [`HeapJob`](../job/index.md#heapjob), [`JobRef`](../job/index.md#jobref)

- <span id="scopebase-inject-broadcast"></span>`fn inject_broadcast<FUNC>(&self, job: Arc<ArcJob<FUNC>>)` — [`ArcJob`](../job/index.md#arcjob)

- <span id="scopebase-complete"></span>`fn complete<FUNC, R>(&self, owner: Option<&WorkerThread>, func: FUNC) -> R` — [`WorkerThread`](../registry/index.md#workerthread)

  Executes `func` as a job, either aborting or executing as

  appropriate.

- <span id="scopebase-execute-job"></span>`unsafe fn execute_job<FUNC>(this: *const Self, func: FUNC)`

  Executes `func` as a job, either aborting or executing as

  appropriate.

- <span id="scopebase-execute-job-closure"></span>`unsafe fn execute_job_closure<FUNC, R>(this: *const Self, func: FUNC) -> Option<R>`

  Executes `func` as a job in scope. Adjusts the "job completed"

  counters and also catches any panic and stores it into

  `scope`.

- <span id="scopebase-job-panicked"></span>`fn job_panicked(&self, err: Box<dyn Any + Send>)`

- <span id="scopebase-maybe-propagate-panic"></span>`fn maybe_propagate_panic(&self)`

#### Trait Implementations

##### `impl Any for ScopeBase<'scope>`

- <span id="scopebase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScopeBase<'scope>`

- <span id="scopebase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScopeBase<'scope>`

- <span id="scopebase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ScopeBase<'scope>`

- <span id="scopebase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScopeBase<'scope>`

- <span id="scopebase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ScopeBase<'scope>`

- <span id="scopebase-pointable-const-align"></span>`const ALIGN: usize`

- <span id="scopebase-pointable-type-init"></span>`type Init = T`

- <span id="scopebase-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopebase-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopebase-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopebase-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ScopeBase<'scope>`

- <span id="scopebase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scopebase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScopeBase<'scope>`

- <span id="scopebase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scopebase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScopePtr<T>`

```rust
struct ScopePtr<T>(*const T);
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:760`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L760)*

Used to capture a scope `&Self` pointer in jobs, without faking a lifetime.

Unsafe code is still required to dereference the pointer, but that's fine in
scope jobs that are guaranteed to execute before the scope ends.

#### Implementations

- <span id="scopeptr-as-ref"></span>`unsafe fn as_ref(&self) -> &T`

#### Trait Implementations

##### `impl<T> Any for ScopePtr<T>`

- <span id="scopeptr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScopePtr<T>`

- <span id="scopeptr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScopePtr<T>`

- <span id="scopeptr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ScopePtr<T>`

- <span id="scopeptr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ScopePtr<T>`

- <span id="scopeptr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for ScopePtr<T>`

- <span id="scopeptr-pointable-const-align"></span>`const ALIGN: usize`

- <span id="scopeptr-pointable-type-init"></span>`type Init = T`

- <span id="scopeptr-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopeptr-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopeptr-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopeptr-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Sync> Send for ScopePtr<T>`

##### `impl<T: Sync> Sync for ScopePtr<T>`

##### `impl<T, U> TryFrom for ScopePtr<T>`

- <span id="scopeptr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scopeptr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ScopePtr<T>`

- <span id="scopeptr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scopeptr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `scope`

```rust
fn scope<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&Scope<'scope>) -> R + Send,
    R: Send
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:277-286`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L277-L286)*

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
[`scope_fifo()`](#scope-fifo) instead.

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

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:366-375`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L366-L375)*

Creates a "fork-join" scope `s` with FIFO order, and invokes the
closure with a reference to `s`. This closure can then spawn
asynchronous tasks into `s`. Those tasks may run asynchronously with
respect to the closure; they may themselves spawn additional tasks
into `s`. When the closure returns, it will block until all tasks
that have been spawned into `s` complete.

# Task execution

Tasks in a `scope_fifo()` run similarly to [`scope()`](#scope), but there's a
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

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:398-403`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L398-L403)*

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

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:405-412`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L405-L412)*

### `get_in_place_thread_registry`

```rust
fn get_in_place_thread_registry(registry: Option<&std::sync::Arc<crate::registry::Registry>>) -> (Option<&crate::registry::WorkerThread>, Option<&std::sync::Arc<crate::registry::Registry>>)
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:414-426`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L414-L426)*

### `in_place_scope_fifo`

```rust
fn in_place_scope_fifo<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&ScopeFifo<'scope>) -> R
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:449-454`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L449-L454)*

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

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:456-463`](../../../.source_1765633015/rayon-core-1.13.0/src/scope/mod.rs#L456-L463)*

