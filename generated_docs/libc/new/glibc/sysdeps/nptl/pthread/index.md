*[libc](../../../../../index.md) / [new](../../../../index.md) / [glibc](../../../index.md) / [sysdeps](../../index.md) / [nptl](../index.md) / [pthread](index.md)*

---

# Module `pthread`

Source header: `sysdeps/nptl/pthread.h`

<https://github.com/bminor/glibc/blob/master/sysdeps/nptl/pthread.h>

## Contents

- [Functions](#functions)
  - [`pthread_getaffinity_np`](#pthread-getaffinity-np)
  - [`pthread_getattr_np`](#pthread-getattr-np)
  - [`pthread_getname_np`](#pthread-getname-np)
  - [`pthread_setaffinity_np`](#pthread-setaffinity-np)
  - [`pthread_setname_np`](#pthread-setname-np)
  - [`pthread_atfork`](#pthread-atfork)
  - [`pthread_attr_getguardsize`](#pthread-attr-getguardsize)
  - [`pthread_attr_getinheritsched`](#pthread-attr-getinheritsched)
  - [`pthread_attr_getschedparam`](#pthread-attr-getschedparam)
  - [`pthread_attr_getschedpolicy`](#pthread-attr-getschedpolicy)
  - [`pthread_attr_getstack`](#pthread-attr-getstack)
  - [`pthread_attr_setguardsize`](#pthread-attr-setguardsize)
  - [`pthread_attr_setinheritsched`](#pthread-attr-setinheritsched)
  - [`pthread_attr_setschedparam`](#pthread-attr-setschedparam)
  - [`pthread_attr_setschedpolicy`](#pthread-attr-setschedpolicy)
  - [`pthread_attr_setstack`](#pthread-attr-setstack)
  - [`pthread_barrier_destroy`](#pthread-barrier-destroy)
  - [`pthread_barrier_init`](#pthread-barrier-init)
  - [`pthread_barrier_wait`](#pthread-barrier-wait)
  - [`pthread_barrierattr_destroy`](#pthread-barrierattr-destroy)
  - [`pthread_barrierattr_getpshared`](#pthread-barrierattr-getpshared)
  - [`pthread_barrierattr_init`](#pthread-barrierattr-init)
  - [`pthread_barrierattr_setpshared`](#pthread-barrierattr-setpshared)
  - [`pthread_cancel`](#pthread-cancel)
  - [`pthread_condattr_getclock`](#pthread-condattr-getclock)
  - [`pthread_condattr_getpshared`](#pthread-condattr-getpshared)
  - [`pthread_condattr_setclock`](#pthread-condattr-setclock)
  - [`pthread_condattr_setpshared`](#pthread-condattr-setpshared)
  - [`pthread_create`](#pthread-create)
  - [`pthread_getcpuclockid`](#pthread-getcpuclockid)
  - [`pthread_getschedparam`](#pthread-getschedparam)
  - [`pthread_kill`](#pthread-kill)
  - [`pthread_mutex_consistent`](#pthread-mutex-consistent)
  - [`pthread_mutex_timedlock`](#pthread-mutex-timedlock)
  - [`pthread_mutexattr_getprotocol`](#pthread-mutexattr-getprotocol)
  - [`pthread_mutexattr_getpshared`](#pthread-mutexattr-getpshared)
  - [`pthread_mutexattr_getrobust`](#pthread-mutexattr-getrobust)
  - [`pthread_mutexattr_setprotocol`](#pthread-mutexattr-setprotocol)
  - [`pthread_mutexattr_setpshared`](#pthread-mutexattr-setpshared)
  - [`pthread_mutexattr_setrobust`](#pthread-mutexattr-setrobust)
  - [`pthread_once`](#pthread-once)
  - [`pthread_rwlockattr_getpshared`](#pthread-rwlockattr-getpshared)
  - [`pthread_rwlockattr_setpshared`](#pthread-rwlockattr-setpshared)
  - [`pthread_setschedparam`](#pthread-setschedparam)
  - [`pthread_setschedprio`](#pthread-setschedprio)
  - [`pthread_sigmask`](#pthread-sigmask)
  - [`pthread_spin_destroy`](#pthread-spin-destroy)
  - [`pthread_spin_init`](#pthread-spin-init)
  - [`pthread_spin_lock`](#pthread-spin-lock)
  - [`pthread_spin_trylock`](#pthread-spin-trylock)
  - [`pthread_spin_unlock`](#pthread-spin-unlock)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pthread_getaffinity_np`](#pthread-getaffinity-np) | fn |  |
| [`pthread_getattr_np`](#pthread-getattr-np) | fn |  |
| [`pthread_getname_np`](#pthread-getname-np) | fn |  |
| [`pthread_setaffinity_np`](#pthread-setaffinity-np) | fn |  |
| [`pthread_setname_np`](#pthread-setname-np) | fn |  |
| [`pthread_atfork`](#pthread-atfork) | fn |  |
| [`pthread_attr_getguardsize`](#pthread-attr-getguardsize) | fn |  |
| [`pthread_attr_getinheritsched`](#pthread-attr-getinheritsched) | fn |  |
| [`pthread_attr_getschedparam`](#pthread-attr-getschedparam) | fn |  |
| [`pthread_attr_getschedpolicy`](#pthread-attr-getschedpolicy) | fn |  |
| [`pthread_attr_getstack`](#pthread-attr-getstack) | fn |  |
| [`pthread_attr_setguardsize`](#pthread-attr-setguardsize) | fn |  |
| [`pthread_attr_setinheritsched`](#pthread-attr-setinheritsched) | fn |  |
| [`pthread_attr_setschedparam`](#pthread-attr-setschedparam) | fn |  |
| [`pthread_attr_setschedpolicy`](#pthread-attr-setschedpolicy) | fn |  |
| [`pthread_attr_setstack`](#pthread-attr-setstack) | fn |  |
| [`pthread_barrier_destroy`](#pthread-barrier-destroy) | fn |  |
| [`pthread_barrier_init`](#pthread-barrier-init) | fn |  |
| [`pthread_barrier_wait`](#pthread-barrier-wait) | fn |  |
| [`pthread_barrierattr_destroy`](#pthread-barrierattr-destroy) | fn |  |
| [`pthread_barrierattr_getpshared`](#pthread-barrierattr-getpshared) | fn |  |
| [`pthread_barrierattr_init`](#pthread-barrierattr-init) | fn |  |
| [`pthread_barrierattr_setpshared`](#pthread-barrierattr-setpshared) | fn |  |
| [`pthread_cancel`](#pthread-cancel) | fn |  |
| [`pthread_condattr_getclock`](#pthread-condattr-getclock) | fn |  |
| [`pthread_condattr_getpshared`](#pthread-condattr-getpshared) | fn |  |
| [`pthread_condattr_setclock`](#pthread-condattr-setclock) | fn |  |
| [`pthread_condattr_setpshared`](#pthread-condattr-setpshared) | fn |  |
| [`pthread_create`](#pthread-create) | fn |  |
| [`pthread_getcpuclockid`](#pthread-getcpuclockid) | fn |  |
| [`pthread_getschedparam`](#pthread-getschedparam) | fn |  |
| [`pthread_kill`](#pthread-kill) | fn |  |
| [`pthread_mutex_consistent`](#pthread-mutex-consistent) | fn |  |
| [`pthread_mutex_timedlock`](#pthread-mutex-timedlock) | fn |  |
| [`pthread_mutexattr_getprotocol`](#pthread-mutexattr-getprotocol) | fn |  |
| [`pthread_mutexattr_getpshared`](#pthread-mutexattr-getpshared) | fn |  |
| [`pthread_mutexattr_getrobust`](#pthread-mutexattr-getrobust) | fn |  |
| [`pthread_mutexattr_setprotocol`](#pthread-mutexattr-setprotocol) | fn |  |
| [`pthread_mutexattr_setpshared`](#pthread-mutexattr-setpshared) | fn |  |
| [`pthread_mutexattr_setrobust`](#pthread-mutexattr-setrobust) | fn |  |
| [`pthread_once`](#pthread-once) | fn |  |
| [`pthread_rwlockattr_getpshared`](#pthread-rwlockattr-getpshared) | fn |  |
| [`pthread_rwlockattr_setpshared`](#pthread-rwlockattr-setpshared) | fn |  |
| [`pthread_setschedparam`](#pthread-setschedparam) | fn |  |
| [`pthread_setschedprio`](#pthread-setschedprio) | fn |  |
| [`pthread_sigmask`](#pthread-sigmask) | fn |  |
| [`pthread_spin_destroy`](#pthread-spin-destroy) | fn |  |
| [`pthread_spin_init`](#pthread-spin-init) | fn |  |
| [`pthread_spin_lock`](#pthread-spin-lock) | fn |  |
| [`pthread_spin_trylock`](#pthread-spin-trylock) | fn |  |
| [`pthread_spin_unlock`](#pthread-spin-unlock) | fn |  |

## Functions

### `pthread_getaffinity_np`

```rust
unsafe fn pthread_getaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:5-9`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/linux_like/pthread.rs#L5-L9)*

### `pthread_getattr_np`

```rust
unsafe fn pthread_getattr_np(native: crate::pthread_t, attr: *mut crate::pthread_attr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:11`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/linux_like/pthread.rs#L11)*

### `pthread_getname_np`

```rust
unsafe fn pthread_getname_np(thread: crate::pthread_t, name: *mut c_char, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:14`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/linux_like/pthread.rs#L14)*

### `pthread_setaffinity_np`

```rust
unsafe fn pthread_setaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:17-21`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/linux_like/pthread.rs#L17-L21)*

### `pthread_setname_np`

```rust
unsafe fn pthread_setname_np(thread: crate::pthread_t, name: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:24`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/linux_like/pthread.rs#L24)*

### `pthread_atfork`

```rust
unsafe fn pthread_atfork(prepare: Option<fn()>, parent: Option<fn()>, child: Option<fn()>) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:9-13`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L9-L13)*

### `pthread_attr_getguardsize`

```rust
unsafe fn pthread_attr_getguardsize(attr: *const crate::pthread_attr_t, guardsize: *mut size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:16-19`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L16-L19)*

### `pthread_attr_getinheritsched`

```rust
unsafe fn pthread_attr_getinheritsched(attr: *const crate::pthread_attr_t, inheritsched: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:27-30`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L27-L30)*

### `pthread_attr_getschedparam`

```rust
unsafe fn pthread_attr_getschedparam(attr: *const crate::pthread_attr_t, param: *mut crate::sched_param) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:33-36`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L33-L36)*

### `pthread_attr_getschedpolicy`

```rust
unsafe fn pthread_attr_getschedpolicy(attr: *const crate::pthread_attr_t, policy: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:39-42`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L39-L42)*

### `pthread_attr_getstack`

```rust
unsafe fn pthread_attr_getstack(attr: *const crate::pthread_attr_t, stackaddr: *mut *mut c_void, stacksize: *mut size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:50-54`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L50-L54)*

### `pthread_attr_setguardsize`

```rust
unsafe fn pthread_attr_setguardsize(attr: *mut crate::pthread_attr_t, guardsize: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:57`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L57)*

### `pthread_attr_setinheritsched`

```rust
unsafe fn pthread_attr_setinheritsched(attr: *mut crate::pthread_attr_t, inheritsched: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:65-68`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L65-L68)*

### `pthread_attr_setschedparam`

```rust
unsafe fn pthread_attr_setschedparam(attr: *mut crate::pthread_attr_t, param: *const crate::sched_param) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:71-74`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L71-L74)*

### `pthread_attr_setschedpolicy`

```rust
unsafe fn pthread_attr_setschedpolicy(attr: *mut crate::pthread_attr_t, policy: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:77`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L77)*

### `pthread_attr_setstack`

```rust
unsafe fn pthread_attr_setstack(attr: *mut crate::pthread_attr_t, stackaddr: *mut c_void, stacksize: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:85-89`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L85-L89)*

### `pthread_barrier_destroy`

```rust
unsafe fn pthread_barrier_destroy(barrier: *mut crate::pthread_barrier_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:92`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L92)*

### `pthread_barrier_init`

```rust
unsafe fn pthread_barrier_init(barrier: *mut crate::pthread_barrier_t, attr: *const crate::pthread_barrierattr_t, count: c_uint) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:95-99`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L95-L99)*

### `pthread_barrier_wait`

```rust
unsafe fn pthread_barrier_wait(barrier: *mut crate::pthread_barrier_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:102`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L102)*

### `pthread_barrierattr_destroy`

```rust
unsafe fn pthread_barrierattr_destroy(attr: *mut crate::pthread_barrierattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:105`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L105)*

### `pthread_barrierattr_getpshared`

```rust
unsafe fn pthread_barrierattr_getpshared(attr: *const crate::pthread_barrierattr_t, shared: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:108-111`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L108-L111)*

### `pthread_barrierattr_init`

```rust
unsafe fn pthread_barrierattr_init(attr: *mut crate::pthread_barrierattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:114`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L114)*

### `pthread_barrierattr_setpshared`

```rust
unsafe fn pthread_barrierattr_setpshared(attr: *mut crate::pthread_barrierattr_t, shared: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:117-120`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L117-L120)*

### `pthread_cancel`

```rust
unsafe fn pthread_cancel(thread: crate::pthread_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:123`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L123)*

### `pthread_condattr_getclock`

```rust
unsafe fn pthread_condattr_getclock(attr: *const crate::pthread_condattr_t, clock_id: *mut crate::clockid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:131-134`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L131-L134)*

### `pthread_condattr_getpshared`

```rust
unsafe fn pthread_condattr_getpshared(attr: *const crate::pthread_condattr_t, pshared: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:142-145`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L142-L145)*

### `pthread_condattr_setclock`

```rust
unsafe fn pthread_condattr_setclock(attr: *mut crate::pthread_condattr_t, clock_id: crate::clockid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:153-156`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L153-L156)*

### `pthread_condattr_setpshared`

```rust
unsafe fn pthread_condattr_setpshared(attr: *mut crate::pthread_condattr_t, pshared: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:165-168`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L165-L168)*

### `pthread_create`

```rust
unsafe fn pthread_create(native: *mut crate::pthread_t, attr: *const crate::pthread_attr_t, f: fn(*mut c_void) -> *mut c_void, value: *mut c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:176-181`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L176-L181)*

### `pthread_getcpuclockid`

```rust
unsafe fn pthread_getcpuclockid(thread: crate::pthread_t, clk_id: *mut crate::clockid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:184`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L184)*

### `pthread_getschedparam`

```rust
unsafe fn pthread_getschedparam(native: crate::pthread_t, policy: *mut c_int, param: *mut crate::sched_param) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:192-196`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L192-L196)*

### `pthread_kill`

```rust
unsafe fn pthread_kill(thread: crate::pthread_t, sig: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:201`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L201)*

### `pthread_mutex_consistent`

```rust
unsafe fn pthread_mutex_consistent(mutex: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:204`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L204)*

### `pthread_mutex_timedlock`

```rust
unsafe fn pthread_mutex_timedlock(lock: *mut crate::pthread_mutex_t, abstime: *const crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:208-211`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L208-L211)*

### `pthread_mutexattr_getprotocol`

```rust
unsafe fn pthread_mutexattr_getprotocol(attr: *const crate::pthread_mutexattr_t, protocol: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:214-217`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L214-L217)*

### `pthread_mutexattr_getpshared`

```rust
unsafe fn pthread_mutexattr_getpshared(attr: *const crate::pthread_mutexattr_t, pshared: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:225-228`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L225-L228)*

### `pthread_mutexattr_getrobust`

```rust
unsafe fn pthread_mutexattr_getrobust(attr: *const crate::pthread_mutexattr_t, robustness: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:231-234`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L231-L234)*

### `pthread_mutexattr_setprotocol`

```rust
unsafe fn pthread_mutexattr_setprotocol(attr: *mut crate::pthread_mutexattr_t, protocol: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:237-240`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L237-L240)*

### `pthread_mutexattr_setpshared`

```rust
unsafe fn pthread_mutexattr_setpshared(attr: *mut crate::pthread_mutexattr_t, pshared: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:249-252`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L249-L252)*

### `pthread_mutexattr_setrobust`

```rust
unsafe fn pthread_mutexattr_setrobust(attr: *mut crate::pthread_mutexattr_t, robustness: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:255-258`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L255-L258)*

### `pthread_once`

```rust
unsafe fn pthread_once(control: *mut crate::pthread_once_t, routine: fn()) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:286`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L286)*

### `pthread_rwlockattr_getpshared`

```rust
unsafe fn pthread_rwlockattr_getpshared(attr: *const crate::pthread_rwlockattr_t, val: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:267-270`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L267-L270)*

### `pthread_rwlockattr_setpshared`

```rust
unsafe fn pthread_rwlockattr_setpshared(attr: *mut crate::pthread_rwlockattr_t, val: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:279-282`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L279-L282)*

### `pthread_setschedparam`

```rust
unsafe fn pthread_setschedparam(native: crate::pthread_t, policy: c_int, param: *const crate::sched_param) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:299-303`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L299-L303)*

### `pthread_setschedprio`

```rust
unsafe fn pthread_setschedprio(native: crate::pthread_t, priority: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:306`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L306)*

### `pthread_sigmask`

```rust
unsafe fn pthread_sigmask(how: c_int, set: *const crate::sigset_t, oldset: *mut crate::sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:311-315`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L311-L315)*

### `pthread_spin_destroy`

```rust
unsafe fn pthread_spin_destroy(lock: *mut crate::pthread_spinlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:318`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L318)*

### `pthread_spin_init`

```rust
unsafe fn pthread_spin_init(lock: *mut crate::pthread_spinlock_t, pshared: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:321`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L321)*

### `pthread_spin_lock`

```rust
unsafe fn pthread_spin_lock(lock: *mut crate::pthread_spinlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:324`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L324)*

### `pthread_spin_trylock`

```rust
unsafe fn pthread_spin_trylock(lock: *mut crate::pthread_spinlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:327`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L327)*

### `pthread_spin_unlock`

```rust
unsafe fn pthread_spin_unlock(lock: *mut crate::pthread_spinlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/posix/pthread.rs:330`](../../../../../../../.source_1765521767/libc-0.2.178/src/new/common/posix/pthread.rs#L330)*

