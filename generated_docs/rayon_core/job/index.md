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

  Unsafe: caller asserts that `data` will remain valid until the

  job is executed.

- <span id="jobref-id"></span>`fn id(&self) -> impl Eq`

  Returns an opaque handle that can be saved and compared,

  without making `JobRef` itself `Copy + Eq`.

- <span id="jobref-execute"></span>`unsafe fn execute(self)`

#### Trait Implementations

##### `impl Any for JobRef`

- <span id="jobref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JobRef`

- <span id="jobref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JobRef`

- <span id="jobref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for JobRef`

- <span id="jobref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JobRef`

- <span id="jobref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for JobRef`

- <span id="jobref-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobref-pointable-type-init"></span>`type Init = T`

- <span id="jobref-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobref-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobref-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobref-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Send for JobRef`

##### `impl Sync for JobRef`

##### `impl<U> TryFrom for JobRef`

- <span id="jobref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jobref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JobRef`

- <span id="jobref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jobref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for StackJob<L, F, R>`

- <span id="stackjob-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StackJob<L, F, R>`

- <span id="stackjob-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StackJob<L, F, R>`

- <span id="stackjob-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StackJob<L, F, R>`

- <span id="stackjob-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StackJob<L, F, R>`

- <span id="stackjob-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<L, F, R> Job for StackJob<L, F, R>`

- <span id="stackjob-job-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for StackJob<L, F, R>`

- <span id="stackjob-pointable-const-align"></span>`const ALIGN: usize`

- <span id="stackjob-pointable-type-init"></span>`type Init = T`

- <span id="stackjob-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stackjob-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stackjob-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stackjob-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for StackJob<L, F, R>`

- <span id="stackjob-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stackjob-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StackJob<L, F, R>`

- <span id="stackjob-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stackjob-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Creates a `JobRef` from this job -- note that this hides all

  lifetimes, so it is up to you to ensure that this JobRef

  doesn't outlive any data that it closes over.

- <span id="heapjob-into-static-job-ref"></span>`fn into_static_job_ref(self: Box<Self>) -> JobRef` — [`JobRef`](#jobref)

  Creates a static `JobRef` from this job.

#### Trait Implementations

##### `impl Any for HeapJob<BODY>`

- <span id="heapjob-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HeapJob<BODY>`

- <span id="heapjob-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HeapJob<BODY>`

- <span id="heapjob-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for HeapJob<BODY>`

- <span id="heapjob-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HeapJob<BODY>`

- <span id="heapjob-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<BODY> Job for HeapJob<BODY>`

- <span id="heapjob-job-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for HeapJob<BODY>`

- <span id="heapjob-pointable-const-align"></span>`const ALIGN: usize`

- <span id="heapjob-pointable-type-init"></span>`type Init = T`

- <span id="heapjob-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="heapjob-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="heapjob-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="heapjob-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for HeapJob<BODY>`

- <span id="heapjob-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="heapjob-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HeapJob<BODY>`

- <span id="heapjob-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="heapjob-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Creates a `JobRef` from this job -- note that this hides all

  lifetimes, so it is up to you to ensure that this JobRef

  doesn't outlive any data that it closes over.

- <span id="arcjob-as-static-job-ref"></span>`fn as_static_job_ref(this: &Arc<Self>) -> JobRef` — [`JobRef`](#jobref)

  Creates a static `JobRef` from this job.

#### Trait Implementations

##### `impl Any for ArcJob<BODY>`

- <span id="arcjob-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArcJob<BODY>`

- <span id="arcjob-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArcJob<BODY>`

- <span id="arcjob-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ArcJob<BODY>`

- <span id="arcjob-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArcJob<BODY>`

- <span id="arcjob-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<BODY> Job for ArcJob<BODY>`

- <span id="arcjob-job-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for ArcJob<BODY>`

- <span id="arcjob-pointable-const-align"></span>`const ALIGN: usize`

- <span id="arcjob-pointable-type-init"></span>`type Init = T`

- <span id="arcjob-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="arcjob-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="arcjob-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="arcjob-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ArcJob<BODY>`

- <span id="arcjob-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arcjob-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArcJob<BODY>`

- <span id="arcjob-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arcjob-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for JobFifo`

- <span id="jobfifo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JobFifo`

- <span id="jobfifo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JobFifo`

- <span id="jobfifo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for JobFifo`

- <span id="jobfifo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JobFifo`

- <span id="jobfifo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Job for JobFifo`

- <span id="jobfifo-job-execute"></span>`unsafe fn execute(this: *const ())`

##### `impl Pointable for JobFifo`

- <span id="jobfifo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobfifo-pointable-type-init"></span>`type Init = T`

- <span id="jobfifo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobfifo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobfifo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobfifo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for JobFifo`

- <span id="jobfifo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jobfifo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JobFifo`

- <span id="jobfifo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jobfifo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Convert the `JobResult` for a job that has finished (and hence

  its JobResult is populated) into its return value.

  

  NB. This will panic if the job panicked.

#### Trait Implementations

##### `impl<T> Any for JobResult<T>`

- <span id="jobresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JobResult<T>`

- <span id="jobresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JobResult<T>`

- <span id="jobresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for JobResult<T>`

- <span id="jobresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for JobResult<T>`

- <span id="jobresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for JobResult<T>`

- <span id="jobresult-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobresult-pointable-type-init"></span>`type Init = T`

- <span id="jobresult-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobresult-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobresult-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobresult-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for JobResult<T>`

- <span id="jobresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jobresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for JobResult<T>`

- <span id="jobresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jobresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

