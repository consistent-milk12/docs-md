*[rayon_core](../index.md) / [job](index.md)*

---

# Module `job`

## Contents

- [Structs](#structs)
  - [`JobRef`](#jobref)
  - [`StackJob`](#stackjob)
  - [`HeapJob`](#heapjob)
  - [`ArcJob`](#arcjob)
  - [`JobFifo`](#jobfifo)
- [Enums](#enums)
  - [`JobResult`](#jobresult)
- [Traits](#traits)
  - [`Job`](#job)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`JobRef`](#jobref) | struct | Effectively a Job trait object. |
| [`StackJob`](#stackjob) | struct | A job that will be owned by a stack slot. |
| [`HeapJob`](#heapjob) | struct | Represents a job stored in the heap. |
| [`ArcJob`](#arcjob) | struct | Represents a job stored in an `Arc` -- like `HeapJob`, but may be turned into multiple `JobRef`s and called multiple times. |
| [`JobFifo`](#jobfifo) | struct | Indirect queue to provide FIFO job priority. |
| [`JobResult`](#jobresult) | enum |  |
| [`Job`](#job) | trait | A `Job` is used to advertise work for other threads that they may want to steal. |

## Structs

### `JobRef`

```rust
struct JobRef {
    pointer: *const (),
    execute_fn: fn(*const ()),
}
```

*Defined in [`rayon-core-1.13.0/src/job.rs:33-36`](../../../.source_1765521767/rayon-core-1.13.0/src/job.rs#L33-L36)*

Effectively a Job trait object. Each JobRef **must** be executed
exactly once, or else data may leak.

Internally, we store the job's data in a `*const ()` pointer.  The
true type is something like `*const StackJob<...>`, but we hide
it. We also carry the "execute fn" from the `Job` trait.

#### Implementations

- <span id="jobref-new"></span>`unsafe fn new<T>(data: *const T) -> JobRef` — [`JobRef`](#jobref)

- <span id="jobref-id"></span>`fn id(&self) -> impl Eq`

- <span id="jobref-execute"></span>`unsafe fn execute(self)`

#### Trait Implementations

##### `impl Pointable for JobRef`

- <span id="jobref-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobref-pointable-type-init"></span>`type Init = T`

- <span id="jobref-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobref-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobref-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobref-drop"></span>`unsafe fn drop(ptr: usize)`

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

*Defined in [`rayon-core-1.13.0/src/job.rs:72-81`](../../../.source_1765521767/rayon-core-1.13.0/src/job.rs#L72-L81)*

A job that will be owned by a stack slot. This means that when it
executes it need not free any heap data, the cleanup occurs when
the stack frame is later popped.  The function parameter indicates
`true` if the job was stolen -- executed on a different thread.

#### Implementations

- <span id="stackjob-new"></span>`fn new(func: F, latch: L) -> StackJob<L, F, R>` — [`StackJob`](#stackjob)

- <span id="stackjob-as-job-ref"></span>`unsafe fn as_job_ref(&self) -> JobRef` — [`JobRef`](#jobref)

- <span id="stackjob-run-inline"></span>`unsafe fn run_inline(self, stolen: bool) -> R`

- <span id="stackjob-into-result"></span>`unsafe fn into_result(self) -> R`

#### Trait Implementations

##### `impl<L, F, R> Job for StackJob<L, F, R>`

- <span id="stackjob-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for StackJob<L, F, R>`

- <span id="stackjob-pointable-const-align"></span>`const ALIGN: usize`

- <span id="stackjob-pointable-type-init"></span>`type Init = T`

- <span id="stackjob-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stackjob-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stackjob-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stackjob-drop"></span>`unsafe fn drop(ptr: usize)`

### `HeapJob<BODY>`

```rust
struct HeapJob<BODY>
where
    BODY: FnOnce() + Send {
    job: BODY,
}
```

*Defined in [`rayon-core-1.13.0/src/job.rs:132-137`](../../../.source_1765521767/rayon-core-1.13.0/src/job.rs#L132-L137)*

Represents a job stored in the heap. Used to implement
`scope`. Unlike `StackJob`, when executed, `HeapJob` simply
invokes a closure, which then triggers the appropriate logic to
signal that the job executed.

(Probably `StackJob` should be refactored in a similar fashion.)

#### Implementations

- <span id="heapjob-new"></span>`fn new(job: BODY) -> Box<Self>`

- <span id="heapjob-into-job-ref"></span>`unsafe fn into_job_ref(self: Box<Self>) -> JobRef` — [`JobRef`](#jobref)

- <span id="heapjob-into-static-job-ref"></span>`fn into_static_job_ref(self: Box<Self>) -> JobRef` — [`JobRef`](#jobref)

#### Trait Implementations

##### `impl<BODY> Job for HeapJob<BODY>`

- <span id="heapjob-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for HeapJob<BODY>`

- <span id="heapjob-pointable-const-align"></span>`const ALIGN: usize`

- <span id="heapjob-pointable-type-init"></span>`type Init = T`

- <span id="heapjob-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="heapjob-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="heapjob-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="heapjob-drop"></span>`unsafe fn drop(ptr: usize)`

### `ArcJob<BODY>`

```rust
struct ArcJob<BODY>
where
    BODY: Fn() + Send + Sync {
    job: BODY,
}
```

*Defined in [`rayon-core-1.13.0/src/job.rs:175-180`](../../../.source_1765521767/rayon-core-1.13.0/src/job.rs#L175-L180)*

Represents a job stored in an `Arc` -- like `HeapJob`, but may
be turned into multiple `JobRef`s and called multiple times.

#### Implementations

- <span id="arcjob-new"></span>`fn new(job: BODY) -> Arc<Self>`

- <span id="arcjob-as-job-ref"></span>`unsafe fn as_job_ref(this: &Arc<Self>) -> JobRef` — [`JobRef`](#jobref)

- <span id="arcjob-as-static-job-ref"></span>`fn as_static_job_ref(this: &Arc<Self>) -> JobRef` — [`JobRef`](#jobref)

#### Trait Implementations

##### `impl<BODY> Job for ArcJob<BODY>`

- <span id="arcjob-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for ArcJob<BODY>`

- <span id="arcjob-pointable-const-align"></span>`const ALIGN: usize`

- <span id="arcjob-pointable-type-init"></span>`type Init = T`

- <span id="arcjob-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="arcjob-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="arcjob-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="arcjob-drop"></span>`unsafe fn drop(ptr: usize)`

### `JobFifo`

```rust
struct JobFifo {
    inner: crossbeam_deque::Injector<JobRef>,
}
```

*Defined in [`rayon-core-1.13.0/src/job.rs:238-240`](../../../.source_1765521767/rayon-core-1.13.0/src/job.rs#L238-L240)*

Indirect queue to provide FIFO job priority.

#### Implementations

- <span id="jobfifo-new"></span>`fn new() -> Self`

- <span id="jobfifo-push"></span>`unsafe fn push(&self, job_ref: JobRef) -> JobRef` — [`JobRef`](#jobref)

#### Trait Implementations

##### `impl Job for JobFifo`

- <span id="jobfifo-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for JobFifo`

- <span id="jobfifo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobfifo-pointable-type-init"></span>`type Init = T`

- <span id="jobfifo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobfifo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobfifo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobfifo-drop"></span>`unsafe fn drop(ptr: usize)`

## Enums

### `JobResult<T>`

```rust
enum JobResult<T> {
    None,
    Ok(T),
    Panic(Box<dyn Any + Send>),
}
```

*Defined in [`rayon-core-1.13.0/src/job.rs:9-13`](../../../.source_1765521767/rayon-core-1.13.0/src/job.rs#L9-L13)*

#### Implementations

- <span id="jobresult-call"></span>`fn call(func: impl FnOnce(bool) -> T) -> Self`

- <span id="jobresult-into-return-value"></span>`fn into_return_value(self) -> T`

#### Trait Implementations

##### `impl<T> Pointable for JobResult<T>`

- <span id="jobresult-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobresult-pointable-type-init"></span>`type Init = T`

- <span id="jobresult-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobresult-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobresult-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobresult-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `Job`

```rust
trait Job { ... }
```

*Defined in [`rayon-core-1.13.0/src/job.rs:20-25`](../../../.source_1765521767/rayon-core-1.13.0/src/job.rs#L20-L25)*

A `Job` is used to advertise work for other threads that they may
want to steal. In accordance with time honored tradition, jobs are
arranged in a deque, so that thieves can take from the top of the
deque while the main worker manages the bottom of the deque. This
deque is managed by the `thread_pool` module.

#### Required Methods

- `fn execute(this: *const ())`

  Unsafe: this may be called from a different thread than the one

#### Implementors

- [`ArcJob`](#arcjob)
- [`HeapJob`](#heapjob)
- [`JobFifo`](#jobfifo)
- [`StackJob`](#stackjob)

