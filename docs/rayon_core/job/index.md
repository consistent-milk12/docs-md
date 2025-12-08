*[rayon_core](../index.md) / [job](index.md)*

---

# Module `job`

## Structs

### `JobRef`

```rust
struct JobRef {
    pointer: *const (),
    execute_fn: fn(*const ()),
}
```

Effectively a Job trait object. Each JobRef **must** be executed
exactly once, or else data may leak.

Internally, we store the job's data in a `*const ()` pointer.  The
true type is something like `*const StackJob<...>`, but we hide
it. We also carry the "execute fn" from the `Job` trait.

#### Implementations

- `unsafe fn new<T>(data: *const T) -> JobRef` — [`JobRef`](#jobref)

- `fn id(self: &Self) -> impl Eq`

- `unsafe fn execute(self: Self)`

#### Trait Implementations

##### `impl<T> Pointable for JobRef`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Send for JobRef`

##### `impl Sync for JobRef`

### `StackJob<L, F, R>`

```rust
struct StackJob<L, F, R>
where
    L: Latch + Sync,
    F: FnOnce(bool) -> R + Send,
    R: Send {
    latch: L,
    func: std::cell::UnsafeCell<Option<F>>,
    result: std::cell::UnsafeCell<JobResult<R>>,
}
```

A job that will be owned by a stack slot. This means that when it
executes it need not free any heap data, the cleanup occurs when
the stack frame is later popped.  The function parameter indicates
`true` if the job was stolen -- executed on a different thread.

#### Implementations

- `fn new(func: F, latch: L) -> StackJob<L, F, R>` — [`StackJob`](#stackjob)

- `unsafe fn as_job_ref(self: &Self) -> JobRef` — [`JobRef`](#jobref)

- `unsafe fn run_inline(self: Self, stolen: bool) -> R`

- `unsafe fn into_result(self: Self) -> R`

#### Trait Implementations

##### `impl<L, F, R> Job for StackJob<L, F, R>`

- `unsafe fn execute(this: *const ())`

##### `impl<T> Pointable for StackJob<L, F, R>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `HeapJob<BODY>`

```rust
struct HeapJob<BODY>
where
    BODY: FnOnce() + Send {
    job: BODY,
}
```

Represents a job stored in the heap. Used to implement
`scope`. Unlike `StackJob`, when executed, `HeapJob` simply
invokes a closure, which then triggers the appropriate logic to
signal that the job executed.

(Probably `StackJob` should be refactored in a similar fashion.)

#### Implementations

- `fn new(job: BODY) -> Box<Self>`

- `unsafe fn into_job_ref(self: Box<Self>) -> JobRef` — [`JobRef`](#jobref)

- `fn into_static_job_ref(self: Box<Self>) -> JobRef` — [`JobRef`](#jobref)

#### Trait Implementations

##### `impl<BODY> Job for HeapJob<BODY>`

- `unsafe fn execute(this: *const ())`

##### `impl<T> Pointable for HeapJob<BODY>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ArcJob<BODY>`

```rust
struct ArcJob<BODY>
where
    BODY: Fn() + Send + Sync {
    job: BODY,
}
```

Represents a job stored in an `Arc` -- like `HeapJob`, but may
be turned into multiple `JobRef`s and called multiple times.

#### Implementations

- `fn new(job: BODY) -> Arc<Self>`

- `unsafe fn as_job_ref(this: &Arc<Self>) -> JobRef` — [`JobRef`](#jobref)

- `fn as_static_job_ref(this: &Arc<Self>) -> JobRef` — [`JobRef`](#jobref)

#### Trait Implementations

##### `impl<BODY> Job for ArcJob<BODY>`

- `unsafe fn execute(this: *const ())`

##### `impl<T> Pointable for ArcJob<BODY>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `JobFifo`

```rust
struct JobFifo {
    inner: crossbeam_deque::Injector<JobRef>,
}
```

Indirect queue to provide FIFO job priority.

#### Implementations

- `fn new() -> Self`

- `unsafe fn push(self: &Self, job_ref: JobRef) -> JobRef` — [`JobRef`](#jobref)

#### Trait Implementations

##### `impl Job for JobFifo`

- `unsafe fn execute(this: *const ())`

##### `impl<T> Pointable for JobFifo`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Enums

### `JobResult<T>`

```rust
enum JobResult<T> {
    None,
    Ok(T),
    Panic(Box<dyn Any + Send>),
}
```

#### Implementations

- `fn call(func: impl FnOnce(bool) -> T) -> Self`

- `fn into_return_value(self: Self) -> T`

#### Trait Implementations

##### `impl<T> Pointable for JobResult<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `Job`

```rust
trait Job { ... }
```

A `Job` is used to advertise work for other threads that they may
want to steal. In accordance with time honored tradition, jobs are
arranged in a deque, so that thieves can take from the top of the
deque while the main worker manages the bottom of the deque. This
deque is managed by the `thread_pool` module.

#### Required Methods

- `fn execute(this: *const ())`

  Unsafe: this may be called from a different thread than the one

