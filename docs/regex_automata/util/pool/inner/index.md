*[regex_automata](../../../index.md) / [util](../../index.md) / [pool](../index.md) / [inner](index.md)*

---

# Module `inner`

## Structs

### `CacheLine<T>`

```rust
struct CacheLine<T>(T);
```

This puts each stack in the pool below into its own cache line. This is
an absolutely critical optimization that tends to have the most impact
in high contention workloads. Without forcing each mutex protected
into its own cache line, high contention exacerbates the performance
problem by causing "false sharing." By putting each mutex in its own
cache-line, we avoid the false sharing problem and the affects of
contention are greatly reduced.

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for CacheLine<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Pool<T, F>`

```rust
struct Pool<T, F> {
    create: F,
    stacks: alloc::vec::Vec<CacheLine<std::sync::Mutex<alloc::vec::Vec<alloc::boxed::Box<T>>>>>,
    owner: core::sync::atomic::AtomicUsize,
    owner_val: core::cell::UnsafeCell<Option<T>>,
}
```

A thread safe pool utilizing std-only features.

The main difference between this and the simplistic alloc-only pool is
the use of std::sync::Mutex and an "owner thread" optimization that
makes accesses by the owner of a pool faster than all other threads.
This makes the common case of running a regex within a single thread
faster by avoiding mutex unlocking.

#### Fields

- **`create`**: `F`

  A function to create more T values when stack is empty and a caller
  has requested a T.

- **`stacks`**: `alloc::vec::Vec<CacheLine<std::sync::Mutex<alloc::vec::Vec<alloc::boxed::Box<T>>>>>`

  Multiple stacks of T values to hand out. These are used when a Pool
  is accessed by a thread that didn't create it.
  
  Conceptually this is `Mutex<Vec<Box<T>>>`, but sharded out to make
  it scale better under high contention work-loads. We index into
  this sequence via `thread_id % stacks.len()`.

- **`owner`**: `core::sync::atomic::AtomicUsize`

  The ID of the thread that owns this pool. The owner is the thread
  that makes the first call to 'get'. When the owner calls 'get', it
  gets 'owner_val' directly instead of returning a T from 'stack'.
  See comments elsewhere for details, but this is intended to be an
  optimization for the common case that makes getting a T faster.
  
  It is initialized to a value of zero (an impossible thread ID) as a
  sentinel to indicate that it is unowned.

- **`owner_val`**: `core::cell::UnsafeCell<Option<T>>`

  A value to return when the caller is in the same thread that
  first called `Pool::get`.
  
  This is set to None when a Pool is first created, and set to Some
  once the first thread calls Pool::get.

#### Implementations

- `fn new(create: F) -> Pool<T, F>` — [`Pool`](#pool)

#### Trait Implementations

##### `impl<T: core::fmt::Debug, F> Debug for Pool<T, F>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T: UnwindSafe, F: UnwindSafe + RefUnwindSafe> RefUnwindSafe for Pool<T, F>`

##### `impl<T: Send, F: Send + Sync> Sync for Pool<T, F>`

##### `impl<T: UnwindSafe, F: UnwindSafe + RefUnwindSafe> UnwindSafe for Pool<T, F>`

### `PoolGuard<'a, T: Send, F: Fn() -> T>`

```rust
struct PoolGuard<'a, T: Send, F: Fn() -> T> {
    pool: &'a Pool<T, F>,
    value: Result<alloc::boxed::Box<T>, usize>,
    discard: bool,
}
```

A guard that is returned when a caller requests a value from the pool.

#### Fields

- **`pool`**: `&'a Pool<T, F>`

  The pool that this guard is attached to.

- **`value`**: `Result<alloc::boxed::Box<T>, usize>`

  This is Err when the guard represents the special "owned" value.
  In which case, the value is retrieved from 'pool.owner_val'. And
  in the special case of `Err(THREAD_ID_DROPPED)`, it means the
  guard has been put back into the pool and should no longer be used.

- **`discard`**: `bool`

  When true, the value should be discarded instead of being pushed
  back into the pool. We tend to use this under high contention, and
  this allows us to avoid inflating the size of the pool. (Because
  under contention, we tend to create more values instead of waiting
  for access to a stack of existing values.)

#### Implementations

- `fn value(self: &Self) -> &T`

- `fn value_mut(self: &mut Self) -> &mut T`

- `fn put(this: PoolGuard<'_, T, F>)` — [`PoolGuard`](#poolguard)

- `fn put_imp(self: &mut Self)`

#### Trait Implementations

##### `impl<'a, T: Send + core::fmt::Debug, F: Fn() -> T> Debug for PoolGuard<'a, T, F>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<'a, T: Send, F: Fn() -> T> Drop for PoolGuard<'a, T, F>`

- `fn drop(self: &mut Self)`

## Constants

### `MAX_POOL_STACKS`

```rust
const MAX_POOL_STACKS: usize = 8usize;
```

The number of stacks we use inside of the pool. These are only used for
non-owners. That is, these represent the "slow" path.

In the original implementation of this pool, we only used a single
stack. While this might be okay for a couple threads, the prevalence of
32, 64 and even 128 core CPUs has made it untenable. The contention
such an environment introduces when threads are doing a lot of searches
on short haystacks (a not uncommon use case) is palpable and leads to
huge slowdowns.

This constant reflects a change from using one stack to the number of
stacks that this constant is set to. The stack for a particular thread
is simply chosen by `thread_id % MAX_POOL_STACKS`. The idea behind
this setup is that there should be a good chance that accesses to the
pool will be distributed over several stacks instead of all of them
converging to one.

This is not a particularly smart or dynamic strategy. Fixing this to a
specific number has at least two downsides. First is that it will help,
say, an 8 core CPU more than it will a 128 core CPU. (But, crucially,
it will still help the 128 core case.) Second is that this may wind
up being a little wasteful with respect to memory usage. Namely, if a
regex is used on one thread and then moved to another thread, then it
could result in creating a new copy of the data in the pool even though
only one is actually needed.

And that memory usage bit is why this is set to 8 and not, say, 64.
Keeping it at 8 limits, to an extent, how much unnecessary memory can
be allocated.

In an ideal world, we'd be able to have something like this:

* Grow the number of stacks as the number of concurrent callers
increases. I spent a little time trying this, but even just adding an
atomic addition/subtraction for each pop/push for tracking concurrent
callers led to a big perf hit. Since even more work would seemingly be
required than just an addition/subtraction, I abandoned this approach.
* The maximum amount of memory used should scale with respect to the
number of concurrent callers and *not* the total number of existing
threads. This is primarily why the `thread_local` crate isn't used, as
as some environments spin up a lot of threads. This led to multiple
reports of extremely high memory usage (often described as memory
leaks).
* Even more ideally, the pool should contract in size. That is, it
should grow with bursts and then shrink. But this is a pretty thorny
issue to tackle and it might be better to just not.
* It would be nice to explore the use of, say, a lock-free stack
instead of using a mutex to guard a `Vec` that is ultimately just
treated as a stack. The main thing preventing me from exploring this
is the ABA problem. The `crossbeam` crate has tools for dealing with
this sort of problem (via its epoch based memory reclamation strategy),
but I can't justify bringing in all of `crossbeam` as a dependency of
`regex` for this.

See this issue for more context and discussion:
https://github.com/rust-lang/regex/issues/934

### `THREAD_ID`

```rust
const THREAD_ID: $crate::thread::LocalKey<usize>;
```

A thread local used to assign an ID to a thread.

