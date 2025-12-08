*[libc](../../../../index.md) / [new](../../../index.md) / [common](../../index.md) / [posix](../index.md) / [pthread](index.md)*

---

# Module `pthread`

Header: `pthread.h`

<https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/pthread.h.html>

## Functions

### `pthread_atfork`

```rust
unsafe fn pthread_atfork(prepare: Option<fn()>, parent: Option<fn()>, child: Option<fn()>) -> c_int
```

### `pthread_attr_getguardsize`

```rust
unsafe fn pthread_attr_getguardsize(attr: *const crate::pthread_attr_t, guardsize: *mut size_t) -> c_int
```

### `pthread_attr_getinheritsched`

```rust
unsafe fn pthread_attr_getinheritsched(attr: *const crate::pthread_attr_t, inheritsched: *mut c_int) -> c_int
```

### `pthread_attr_getschedparam`

```rust
unsafe fn pthread_attr_getschedparam(attr: *const crate::pthread_attr_t, param: *mut crate::sched_param) -> c_int
```

### `pthread_attr_getschedpolicy`

```rust
unsafe fn pthread_attr_getschedpolicy(attr: *const crate::pthread_attr_t, policy: *mut c_int) -> c_int
```

### `pthread_attr_getstack`

```rust
unsafe fn pthread_attr_getstack(attr: *const crate::pthread_attr_t, stackaddr: *mut *mut c_void, stacksize: *mut size_t) -> c_int
```

### `pthread_attr_setguardsize`

```rust
unsafe fn pthread_attr_setguardsize(attr: *mut crate::pthread_attr_t, guardsize: size_t) -> c_int
```

### `pthread_attr_setinheritsched`

```rust
unsafe fn pthread_attr_setinheritsched(attr: *mut crate::pthread_attr_t, inheritsched: c_int) -> c_int
```

### `pthread_attr_setschedparam`

```rust
unsafe fn pthread_attr_setschedparam(attr: *mut crate::pthread_attr_t, param: *const crate::sched_param) -> c_int
```

### `pthread_attr_setschedpolicy`

```rust
unsafe fn pthread_attr_setschedpolicy(attr: *mut crate::pthread_attr_t, policy: c_int) -> c_int
```

### `pthread_attr_setstack`

```rust
unsafe fn pthread_attr_setstack(attr: *mut crate::pthread_attr_t, stackaddr: *mut c_void, stacksize: size_t) -> c_int
```

### `pthread_barrier_destroy`

```rust
unsafe fn pthread_barrier_destroy(barrier: *mut crate::pthread_barrier_t) -> c_int
```

### `pthread_barrier_init`

```rust
unsafe fn pthread_barrier_init(barrier: *mut crate::pthread_barrier_t, attr: *const crate::pthread_barrierattr_t, count: c_uint) -> c_int
```

### `pthread_barrier_wait`

```rust
unsafe fn pthread_barrier_wait(barrier: *mut crate::pthread_barrier_t) -> c_int
```

### `pthread_barrierattr_destroy`

```rust
unsafe fn pthread_barrierattr_destroy(attr: *mut crate::pthread_barrierattr_t) -> c_int
```

### `pthread_barrierattr_getpshared`

```rust
unsafe fn pthread_barrierattr_getpshared(attr: *const crate::pthread_barrierattr_t, shared: *mut c_int) -> c_int
```

### `pthread_barrierattr_init`

```rust
unsafe fn pthread_barrierattr_init(attr: *mut crate::pthread_barrierattr_t) -> c_int
```

### `pthread_barrierattr_setpshared`

```rust
unsafe fn pthread_barrierattr_setpshared(attr: *mut crate::pthread_barrierattr_t, shared: c_int) -> c_int
```

### `pthread_cancel`

```rust
unsafe fn pthread_cancel(thread: crate::pthread_t) -> c_int
```

### `pthread_condattr_getclock`

```rust
unsafe fn pthread_condattr_getclock(attr: *const crate::pthread_condattr_t, clock_id: *mut crate::clockid_t) -> c_int
```

### `pthread_condattr_getpshared`

```rust
unsafe fn pthread_condattr_getpshared(attr: *const crate::pthread_condattr_t, pshared: *mut c_int) -> c_int
```

### `pthread_condattr_setclock`

```rust
unsafe fn pthread_condattr_setclock(attr: *mut crate::pthread_condattr_t, clock_id: crate::clockid_t) -> c_int
```

### `pthread_condattr_setpshared`

```rust
unsafe fn pthread_condattr_setpshared(attr: *mut crate::pthread_condattr_t, pshared: c_int) -> c_int
```

### `pthread_create`

```rust
unsafe fn pthread_create(native: *mut crate::pthread_t, attr: *const crate::pthread_attr_t, f: fn(*mut c_void) -> *mut c_void, value: *mut c_void) -> c_int
```

### `pthread_getcpuclockid`

```rust
unsafe fn pthread_getcpuclockid(thread: crate::pthread_t, clk_id: *mut crate::clockid_t) -> c_int
```

### `pthread_getschedparam`

```rust
unsafe fn pthread_getschedparam(native: crate::pthread_t, policy: *mut c_int, param: *mut crate::sched_param) -> c_int
```

### `pthread_kill`

```rust
unsafe fn pthread_kill(thread: crate::pthread_t, sig: c_int) -> c_int
```

### `pthread_mutex_consistent`

```rust
unsafe fn pthread_mutex_consistent(mutex: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutex_timedlock`

```rust
unsafe fn pthread_mutex_timedlock(lock: *mut crate::pthread_mutex_t, abstime: *const crate::timespec) -> c_int
```

### `pthread_mutexattr_getprotocol`

```rust
unsafe fn pthread_mutexattr_getprotocol(attr: *const crate::pthread_mutexattr_t, protocol: *mut c_int) -> c_int
```

### `pthread_mutexattr_getpshared`

```rust
unsafe fn pthread_mutexattr_getpshared(attr: *const crate::pthread_mutexattr_t, pshared: *mut c_int) -> c_int
```

### `pthread_mutexattr_getrobust`

```rust
unsafe fn pthread_mutexattr_getrobust(attr: *const crate::pthread_mutexattr_t, robustness: *mut c_int) -> c_int
```

### `pthread_mutexattr_setprotocol`

```rust
unsafe fn pthread_mutexattr_setprotocol(attr: *mut crate::pthread_mutexattr_t, protocol: c_int) -> c_int
```

### `pthread_mutexattr_setpshared`

```rust
unsafe fn pthread_mutexattr_setpshared(attr: *mut crate::pthread_mutexattr_t, pshared: c_int) -> c_int
```

### `pthread_mutexattr_setrobust`

```rust
unsafe fn pthread_mutexattr_setrobust(attr: *mut crate::pthread_mutexattr_t, robustness: c_int) -> c_int
```

### `pthread_rwlockattr_getpshared`

```rust
unsafe fn pthread_rwlockattr_getpshared(attr: *const crate::pthread_rwlockattr_t, val: *mut c_int) -> c_int
```

### `pthread_rwlockattr_setpshared`

```rust
unsafe fn pthread_rwlockattr_setpshared(attr: *mut crate::pthread_rwlockattr_t, val: c_int) -> c_int
```

### `pthread_once`

```rust
unsafe fn pthread_once(control: *mut crate::pthread_once_t, routine: fn()) -> c_int
```

### `pthread_setschedparam`

```rust
unsafe fn pthread_setschedparam(native: crate::pthread_t, policy: c_int, param: *const crate::sched_param) -> c_int
```

### `pthread_setschedprio`

```rust
unsafe fn pthread_setschedprio(native: crate::pthread_t, priority: c_int) -> c_int
```

### `pthread_sigmask`

```rust
unsafe fn pthread_sigmask(how: c_int, set: *const crate::sigset_t, oldset: *mut crate::sigset_t) -> c_int
```

### `pthread_spin_destroy`

```rust
unsafe fn pthread_spin_destroy(lock: *mut crate::pthread_spinlock_t) -> c_int
```

### `pthread_spin_init`

```rust
unsafe fn pthread_spin_init(lock: *mut crate::pthread_spinlock_t, pshared: c_int) -> c_int
```

### `pthread_spin_lock`

```rust
unsafe fn pthread_spin_lock(lock: *mut crate::pthread_spinlock_t) -> c_int
```

### `pthread_spin_trylock`

```rust
unsafe fn pthread_spin_trylock(lock: *mut crate::pthread_spinlock_t) -> c_int
```

### `pthread_spin_unlock`

```rust
unsafe fn pthread_spin_unlock(lock: *mut crate::pthread_spinlock_t) -> c_int
```

