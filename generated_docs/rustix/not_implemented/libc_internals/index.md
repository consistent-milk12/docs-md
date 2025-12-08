*[rustix](../../index.md) / [not_implemented](../index.md) / [libc_internals](index.md)*

---

# Module `libc_internals`

Functions which need access to libc internals are out of scope for rustix.

Most Rust programs have a libc present, and when a libc is present, it
expects to be the only thing in the process that can do certain operations.
For example, there can be only one `atexit` list in a process, only one
`pthread_atfork` list in a process, only one implementation of pthreads in
a process, and so on, and libc expects to own the one of each of those
things. And libc implementations may expect to be involved in signal
handling. So, these functions are believed to be out of scope for rustix.
This module contains an incomplete list of such functions.

It would be possible to make a rust library which provides safe or
ergonomic wrappers around these libc functions, however that is out of
scope for rustix itself.

If you would like to write a Rust program which does not use a libc, and
which does provide APIs for some of these functions, [Eyra] and [origin]
are two libraries which may be useful, and which provide public interfaces
for some of this functionality.

If you are otherwise writing Rust code which you know will not share a
process with a libc, perhaps because you are writing a libc or similar
yourself, rustix's codebase does include experimental implementations of
the primitives needed to implement most of these functions.



## Contents

- [Functions](#functions)
  - [`exit`](#exit)
  - [`fork`](#fork)
  - [`clone`](#clone)
  - [`clone3`](#clone3)
  - [`brk`](#brk)
  - [`sigaction`](#sigaction)
  - [`sigaltstack`](#sigaltstack)
  - [`sigprocmask`](#sigprocmask)
  - [`sigwait`](#sigwait)
  - [`sigwaitinfo`](#sigwaitinfo)
  - [`sigtimedwait`](#sigtimedwait)
  - [`set_thread_area`](#set_thread_area)
  - [`set_tid_address`](#set_tid_address)
  - [`tkill`](#tkill)
  - [`sched_setscheduler`](#sched_setscheduler)
  - [`rseq`](#rseq)
  - [`setuid`](#setuid)
  - [`setgid`](#setgid)
  - [`seteuid`](#seteuid)
  - [`setegid`](#setegid)
  - [`setreuid`](#setreuid)
  - [`setregid`](#setregid)
  - [`setresuid`](#setresuid)
  - [`setresgid`](#setresgid)
  - [`setgroups`](#setgroups)
  - [`pthread_atfork`](#pthread_atfork)
  - [`pthread_attr_destroy`](#pthread_attr_destroy)
  - [`pthread_attr_getaffinity_np`](#pthread_attr_getaffinity_np)
  - [`pthread_attr_getdetachstate`](#pthread_attr_getdetachstate)
  - [`pthread_attr_getguardsize`](#pthread_attr_getguardsize)
  - [`pthread_attr_getinheritsched`](#pthread_attr_getinheritsched)
  - [`pthread_attr_getschedparam`](#pthread_attr_getschedparam)
  - [`pthread_attr_getschedpolicy`](#pthread_attr_getschedpolicy)
  - [`pthread_attr_getscope`](#pthread_attr_getscope)
  - [`pthread_attr_getsigmask_np`](#pthread_attr_getsigmask_np)
  - [`pthread_attr_getstack`](#pthread_attr_getstack)
  - [`pthread_attr_getstackaddr`](#pthread_attr_getstackaddr)
  - [`pthread_attr_getstacksize`](#pthread_attr_getstacksize)
  - [`pthread_attr_init`](#pthread_attr_init)
  - [`pthread_attr_setaffinity_np`](#pthread_attr_setaffinity_np)
  - [`pthread_attr_setdetachstate`](#pthread_attr_setdetachstate)
  - [`pthread_attr_setguardsize`](#pthread_attr_setguardsize)
  - [`pthread_attr_setinheritsched`](#pthread_attr_setinheritsched)
  - [`pthread_attr_setschedparam`](#pthread_attr_setschedparam)
  - [`pthread_attr_setschedpolicy`](#pthread_attr_setschedpolicy)
  - [`pthread_attr_setscope`](#pthread_attr_setscope)
  - [`pthread_attr_setsigmask_np`](#pthread_attr_setsigmask_np)
  - [`pthread_attr_setstack`](#pthread_attr_setstack)
  - [`pthread_attr_setstackaddr`](#pthread_attr_setstackaddr)
  - [`pthread_attr_setstacksize`](#pthread_attr_setstacksize)
  - [`pthread_barrierattr_destroy`](#pthread_barrierattr_destroy)
  - [`pthread_barrierattr_getpshared`](#pthread_barrierattr_getpshared)
  - [`pthread_barrierattr_init`](#pthread_barrierattr_init)
  - [`pthread_barrierattr_setpshared`](#pthread_barrierattr_setpshared)
  - [`pthread_barrier_destroy`](#pthread_barrier_destroy)
  - [`pthread_barrier_wait`](#pthread_barrier_wait)
  - [`pthread_cancel`](#pthread_cancel)
  - [`pthread_cleanup_pop`](#pthread_cleanup_pop)
  - [`pthread_cleanup_pop_restore_np`](#pthread_cleanup_pop_restore_np)
  - [`pthread_cleanup_push`](#pthread_cleanup_push)
  - [`pthread_cleanup_push_defer_np`](#pthread_cleanup_push_defer_np)
  - [`pthread_condattr_destroy`](#pthread_condattr_destroy)
  - [`pthread_condattr_getclock`](#pthread_condattr_getclock)
  - [`pthread_condattr_getpshared`](#pthread_condattr_getpshared)
  - [`pthread_condattr_init`](#pthread_condattr_init)
  - [`pthread_condattr_setclock`](#pthread_condattr_setclock)
  - [`pthread_condattr_setpshared`](#pthread_condattr_setpshared)
  - [`pthread_cond_broadcast`](#pthread_cond_broadcast)
  - [`pthread_cond_destroy`](#pthread_cond_destroy)
  - [`pthread_cond_signal`](#pthread_cond_signal)
  - [`pthread_cond_timedwait`](#pthread_cond_timedwait)
  - [`pthread_create`](#pthread_create)
  - [`pthread_detach`](#pthread_detach)
  - [`pthread_equal`](#pthread_equal)
  - [`pthread_exit`](#pthread_exit)
  - [`pthread_getaffinity_np`](#pthread_getaffinity_np)
  - [`pthread_getattr_default_np`](#pthread_getattr_default_np)
  - [`pthread_getattr_np`](#pthread_getattr_np)
  - [`pthread_getconcurrency`](#pthread_getconcurrency)
  - [`pthread_getcpuclockid`](#pthread_getcpuclockid)
  - [`pthread_getname_np`](#pthread_getname_np)
  - [`pthread_getschedparam`](#pthread_getschedparam)
  - [`pthread_getspecific`](#pthread_getspecific)
  - [`pthread_join`](#pthread_join)
  - [`pthread_key_create`](#pthread_key_create)
  - [`pthread_key_delete`](#pthread_key_delete)
  - [`pthread_kill`](#pthread_kill)
  - [`pthread_kill_other_threads_np`](#pthread_kill_other_threads_np)
  - [`pthread_mutexattr_destroy`](#pthread_mutexattr_destroy)
  - [`pthread_mutexattr_getprioceiling`](#pthread_mutexattr_getprioceiling)
  - [`pthread_mutexattr_getprotocol`](#pthread_mutexattr_getprotocol)
  - [`pthread_mutexattr_getpshared`](#pthread_mutexattr_getpshared)
  - [`pthread_mutexattr_getrobust`](#pthread_mutexattr_getrobust)
  - [`pthread_mutexattr_getrobust_np`](#pthread_mutexattr_getrobust_np)
  - [`pthread_mutexattr_gettype`](#pthread_mutexattr_gettype)
  - [`pthread_mutexattr_init`](#pthread_mutexattr_init)
  - [`pthread_mutexattr_setprioceiling`](#pthread_mutexattr_setprioceiling)
  - [`pthread_mutexattr_setprotocol`](#pthread_mutexattr_setprotocol)
  - [`pthread_mutexattr_setpshared`](#pthread_mutexattr_setpshared)
  - [`pthread_mutexattr_setrobust`](#pthread_mutexattr_setrobust)
  - [`pthread_mutexattr_setrobust_np`](#pthread_mutexattr_setrobust_np)
  - [`pthread_mutexattr_settype`](#pthread_mutexattr_settype)
  - [`pthread_mutex_consistent`](#pthread_mutex_consistent)
  - [`pthread_mutex_consistent_np`](#pthread_mutex_consistent_np)
  - [`pthread_mutex_destroy`](#pthread_mutex_destroy)
  - [`pthread_mutex_getprioceiling`](#pthread_mutex_getprioceiling)
  - [`pthread_mutex_init`](#pthread_mutex_init)
  - [`pthread_mutex_lock`](#pthread_mutex_lock)
  - [`pthread_mutex_setprioceiling`](#pthread_mutex_setprioceiling)
  - [`pthread_mutex_timedlock`](#pthread_mutex_timedlock)
  - [`pthread_mutex_trylock`](#pthread_mutex_trylock)
  - [`pthread_once`](#pthread_once)
  - [`pthread_rwlockattr_destroy`](#pthread_rwlockattr_destroy)
  - [`pthread_rwlockattr_getkind_np`](#pthread_rwlockattr_getkind_np)
  - [`pthread_rwlockattr_getpshared`](#pthread_rwlockattr_getpshared)
  - [`pthread_rwlockattr_init`](#pthread_rwlockattr_init)
  - [`pthread_rwlockattr_setkind_np`](#pthread_rwlockattr_setkind_np)
  - [`pthread_rwlockattr_setpshared`](#pthread_rwlockattr_setpshared)
  - [`pthread_rwlock_destroy`](#pthread_rwlock_destroy)
  - [`pthread_rwlock_rdlock`](#pthread_rwlock_rdlock)
  - [`pthread_rwlock_timedrdlock`](#pthread_rwlock_timedrdlock)
  - [`pthread_rwlock_timedwrlock`](#pthread_rwlock_timedwrlock)
  - [`pthread_rwlock_tryrdlock`](#pthread_rwlock_tryrdlock)
  - [`pthread_rwlock_trywrlock`](#pthread_rwlock_trywrlock)
  - [`pthread_rwlock_unlock`](#pthread_rwlock_unlock)
  - [`pthread_rwlock_wrlock`](#pthread_rwlock_wrlock)
  - [`pthread_self`](#pthread_self)
  - [`pthread_setaffinity_np`](#pthread_setaffinity_np)
  - [`pthread_setattr_default_np`](#pthread_setattr_default_np)
  - [`pthread_setcancelstate`](#pthread_setcancelstate)
  - [`pthread_setcanceltype`](#pthread_setcanceltype)
  - [`pthread_setconcurrency`](#pthread_setconcurrency)
  - [`pthread_setname_np`](#pthread_setname_np)
  - [`pthread_setschedparam`](#pthread_setschedparam)
  - [`pthread_setschedprio`](#pthread_setschedprio)
  - [`pthread_setspecific`](#pthread_setspecific)
  - [`pthread_sigmask`](#pthread_sigmask)
  - [`pthread_sigqueue`](#pthread_sigqueue)
  - [`pthread_spin_destroy`](#pthread_spin_destroy)
  - [`pthread_spin_init`](#pthread_spin_init)
  - [`pthread_spin_lock`](#pthread_spin_lock)
  - [`pthread_spin_trylock`](#pthread_spin_trylock)
  - [`pthread_spin_unlock`](#pthread_spin_unlock)
  - [`pthread_testcancel`](#pthread_testcancel)
  - [`pthread_timedjoin_np`](#pthread_timedjoin_np)
  - [`pthread_tryjoin_np`](#pthread_tryjoin_np)
  - [`pthread_yield`](#pthread_yield)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`exit`](#exit) | fn | See the [module comment](self). |
| [`fork`](#fork) | fn | See the [module comment](self). |
| [`clone`](#clone) | fn | See the [module comment](self). |
| [`clone3`](#clone3) | fn | See the [module comment](self). |
| [`brk`](#brk) | fn | See the [module comment](self). |
| [`sigaction`](#sigaction) | fn | See the [module comment](self). |
| [`sigaltstack`](#sigaltstack) | fn | See the [module comment](self). |
| [`sigprocmask`](#sigprocmask) | fn | See the [module comment](self). |
| [`sigwait`](#sigwait) | fn | See the [module comment](self). |
| [`sigwaitinfo`](#sigwaitinfo) | fn | See the [module comment](self). |
| [`sigtimedwait`](#sigtimedwait) | fn | See the [module comment](self). |
| [`set_thread_area`](#set_thread_area) | fn | See the [module comment](self). |
| [`set_tid_address`](#set_tid_address) | fn | See the [module comment](self). |
| [`tkill`](#tkill) | fn | See the [module comment](self). |
| [`sched_setscheduler`](#sched_setscheduler) | fn | See the [module comment](self). |
| [`rseq`](#rseq) | fn | See the [module comment](self). |
| [`setuid`](#setuid) | fn | See the [module comment](self). |
| [`setgid`](#setgid) | fn | See the [module comment](self). |
| [`seteuid`](#seteuid) | fn | See the [module comment](self). |
| [`setegid`](#setegid) | fn | See the [module comment](self). |
| [`setreuid`](#setreuid) | fn | See the [module comment](self). |
| [`setregid`](#setregid) | fn | See the [module comment](self). |
| [`setresuid`](#setresuid) | fn | See the [module comment](self). |
| [`setresgid`](#setresgid) | fn | See the [module comment](self). |
| [`setgroups`](#setgroups) | fn | See the [module comment](self). |
| [`pthread_atfork`](#pthread_atfork) | fn | See the [module comment](self). |
| [`pthread_attr_destroy`](#pthread_attr_destroy) | fn | See the [module comment](self). |
| [`pthread_attr_getaffinity_np`](#pthread_attr_getaffinity_np) | fn | See the [module comment](self). |
| [`pthread_attr_getdetachstate`](#pthread_attr_getdetachstate) | fn | See the [module comment](self). |
| [`pthread_attr_getguardsize`](#pthread_attr_getguardsize) | fn | See the [module comment](self). |
| [`pthread_attr_getinheritsched`](#pthread_attr_getinheritsched) | fn | See the [module comment](self). |
| [`pthread_attr_getschedparam`](#pthread_attr_getschedparam) | fn | See the [module comment](self). |
| [`pthread_attr_getschedpolicy`](#pthread_attr_getschedpolicy) | fn | See the [module comment](self). |
| [`pthread_attr_getscope`](#pthread_attr_getscope) | fn | See the [module comment](self). |
| [`pthread_attr_getsigmask_np`](#pthread_attr_getsigmask_np) | fn | See the [module comment](self). |
| [`pthread_attr_getstack`](#pthread_attr_getstack) | fn | See the [module comment](self). |
| [`pthread_attr_getstackaddr`](#pthread_attr_getstackaddr) | fn | See the [module comment](self). |
| [`pthread_attr_getstacksize`](#pthread_attr_getstacksize) | fn | See the [module comment](self). |
| [`pthread_attr_init`](#pthread_attr_init) | fn | See the [module comment](self). |
| [`pthread_attr_setaffinity_np`](#pthread_attr_setaffinity_np) | fn | See the [module comment](self). |
| [`pthread_attr_setdetachstate`](#pthread_attr_setdetachstate) | fn | See the [module comment](self). |
| [`pthread_attr_setguardsize`](#pthread_attr_setguardsize) | fn | See the [module comment](self). |
| [`pthread_attr_setinheritsched`](#pthread_attr_setinheritsched) | fn | See the [module comment](self). |
| [`pthread_attr_setschedparam`](#pthread_attr_setschedparam) | fn | See the [module comment](self). |
| [`pthread_attr_setschedpolicy`](#pthread_attr_setschedpolicy) | fn | See the [module comment](self). |
| [`pthread_attr_setscope`](#pthread_attr_setscope) | fn | See the [module comment](self). |
| [`pthread_attr_setsigmask_np`](#pthread_attr_setsigmask_np) | fn | See the [module comment](self). |
| [`pthread_attr_setstack`](#pthread_attr_setstack) | fn | See the [module comment](self). |
| [`pthread_attr_setstackaddr`](#pthread_attr_setstackaddr) | fn | See the [module comment](self). |
| [`pthread_attr_setstacksize`](#pthread_attr_setstacksize) | fn | See the [module comment](self). |
| [`pthread_barrierattr_destroy`](#pthread_barrierattr_destroy) | fn | See the [module comment](self). |
| [`pthread_barrierattr_getpshared`](#pthread_barrierattr_getpshared) | fn | See the [module comment](self). |
| [`pthread_barrierattr_init`](#pthread_barrierattr_init) | fn | See the [module comment](self). |
| [`pthread_barrierattr_setpshared`](#pthread_barrierattr_setpshared) | fn | See the [module comment](self). |
| [`pthread_barrier_destroy`](#pthread_barrier_destroy) | fn | See the [module comment](self). |
| [`pthread_barrier_wait`](#pthread_barrier_wait) | fn | See the [module comment](self). |
| [`pthread_cancel`](#pthread_cancel) | fn | See the [module comment](self). |
| [`pthread_cleanup_pop`](#pthread_cleanup_pop) | fn | See the [module comment](self). |
| [`pthread_cleanup_pop_restore_np`](#pthread_cleanup_pop_restore_np) | fn | See the [module comment](self). |
| [`pthread_cleanup_push`](#pthread_cleanup_push) | fn | See the [module comment](self). |
| [`pthread_cleanup_push_defer_np`](#pthread_cleanup_push_defer_np) | fn | See the [module comment](self). |
| [`pthread_condattr_destroy`](#pthread_condattr_destroy) | fn | See the [module comment](self). |
| [`pthread_condattr_getclock`](#pthread_condattr_getclock) | fn | See the [module comment](self). |
| [`pthread_condattr_getpshared`](#pthread_condattr_getpshared) | fn | See the [module comment](self). |
| [`pthread_condattr_init`](#pthread_condattr_init) | fn | See the [module comment](self). |
| [`pthread_condattr_setclock`](#pthread_condattr_setclock) | fn | See the [module comment](self). |
| [`pthread_condattr_setpshared`](#pthread_condattr_setpshared) | fn | See the [module comment](self). |
| [`pthread_cond_broadcast`](#pthread_cond_broadcast) | fn | See the [module comment](self). |
| [`pthread_cond_destroy`](#pthread_cond_destroy) | fn | See the [module comment](self). |
| [`pthread_cond_signal`](#pthread_cond_signal) | fn | See the [module comment](self). |
| [`pthread_cond_timedwait`](#pthread_cond_timedwait) | fn | See the [module comment](self). |
| [`pthread_create`](#pthread_create) | fn | See the [module comment](self). |
| [`pthread_detach`](#pthread_detach) | fn | See the [module comment](self). |
| [`pthread_equal`](#pthread_equal) | fn | See the [module comment](self). |
| [`pthread_exit`](#pthread_exit) | fn | See the [module comment](self). |
| [`pthread_getaffinity_np`](#pthread_getaffinity_np) | fn | See the [module comment](self). |
| [`pthread_getattr_default_np`](#pthread_getattr_default_np) | fn | See the [module comment](self). |
| [`pthread_getattr_np`](#pthread_getattr_np) | fn | See the [module comment](self). |
| [`pthread_getconcurrency`](#pthread_getconcurrency) | fn | See the [module comment](self). |
| [`pthread_getcpuclockid`](#pthread_getcpuclockid) | fn | See the [module comment](self). |
| [`pthread_getname_np`](#pthread_getname_np) | fn | See the [module comment](self). |
| [`pthread_getschedparam`](#pthread_getschedparam) | fn | See the [module comment](self). |
| [`pthread_getspecific`](#pthread_getspecific) | fn | See the [module comment](self). |
| [`pthread_join`](#pthread_join) | fn | See the [module comment](self). |
| [`pthread_key_create`](#pthread_key_create) | fn | See the [module comment](self). |
| [`pthread_key_delete`](#pthread_key_delete) | fn | See the [module comment](self). |
| [`pthread_kill`](#pthread_kill) | fn | See the [module comment](self). |
| [`pthread_kill_other_threads_np`](#pthread_kill_other_threads_np) | fn | See the [module comment](self). |
| [`pthread_mutexattr_destroy`](#pthread_mutexattr_destroy) | fn | See the [module comment](self). |
| [`pthread_mutexattr_getprioceiling`](#pthread_mutexattr_getprioceiling) | fn | See the [module comment](self). |
| [`pthread_mutexattr_getprotocol`](#pthread_mutexattr_getprotocol) | fn | See the [module comment](self). |
| [`pthread_mutexattr_getpshared`](#pthread_mutexattr_getpshared) | fn | See the [module comment](self). |
| [`pthread_mutexattr_getrobust`](#pthread_mutexattr_getrobust) | fn | See the [module comment](self). |
| [`pthread_mutexattr_getrobust_np`](#pthread_mutexattr_getrobust_np) | fn | See the [module comment](self). |
| [`pthread_mutexattr_gettype`](#pthread_mutexattr_gettype) | fn | See the [module comment](self). |
| [`pthread_mutexattr_init`](#pthread_mutexattr_init) | fn | See the [module comment](self). |
| [`pthread_mutexattr_setprioceiling`](#pthread_mutexattr_setprioceiling) | fn | See the [module comment](self). |
| [`pthread_mutexattr_setprotocol`](#pthread_mutexattr_setprotocol) | fn | See the [module comment](self). |
| [`pthread_mutexattr_setpshared`](#pthread_mutexattr_setpshared) | fn | See the [module comment](self). |
| [`pthread_mutexattr_setrobust`](#pthread_mutexattr_setrobust) | fn | See the [module comment](self). |
| [`pthread_mutexattr_setrobust_np`](#pthread_mutexattr_setrobust_np) | fn | See the [module comment](self). |
| [`pthread_mutexattr_settype`](#pthread_mutexattr_settype) | fn | See the [module comment](self). |
| [`pthread_mutex_consistent`](#pthread_mutex_consistent) | fn | See the [module comment](self). |
| [`pthread_mutex_consistent_np`](#pthread_mutex_consistent_np) | fn | See the [module comment](self). |
| [`pthread_mutex_destroy`](#pthread_mutex_destroy) | fn | See the [module comment](self). |
| [`pthread_mutex_getprioceiling`](#pthread_mutex_getprioceiling) | fn | See the [module comment](self). |
| [`pthread_mutex_init`](#pthread_mutex_init) | fn | See the [module comment](self). |
| [`pthread_mutex_lock`](#pthread_mutex_lock) | fn | See the [module comment](self). |
| [`pthread_mutex_setprioceiling`](#pthread_mutex_setprioceiling) | fn | See the [module comment](self). |
| [`pthread_mutex_timedlock`](#pthread_mutex_timedlock) | fn | See the [module comment](self). |
| [`pthread_mutex_trylock`](#pthread_mutex_trylock) | fn | See the [module comment](self). |
| [`pthread_once`](#pthread_once) | fn | See the [module comment](self). |
| [`pthread_rwlockattr_destroy`](#pthread_rwlockattr_destroy) | fn | See the [module comment](self). |
| [`pthread_rwlockattr_getkind_np`](#pthread_rwlockattr_getkind_np) | fn | See the [module comment](self). |
| [`pthread_rwlockattr_getpshared`](#pthread_rwlockattr_getpshared) | fn | See the [module comment](self). |
| [`pthread_rwlockattr_init`](#pthread_rwlockattr_init) | fn | See the [module comment](self). |
| [`pthread_rwlockattr_setkind_np`](#pthread_rwlockattr_setkind_np) | fn | See the [module comment](self). |
| [`pthread_rwlockattr_setpshared`](#pthread_rwlockattr_setpshared) | fn | See the [module comment](self). |
| [`pthread_rwlock_destroy`](#pthread_rwlock_destroy) | fn | See the [module comment](self). |
| [`pthread_rwlock_rdlock`](#pthread_rwlock_rdlock) | fn | See the [module comment](self). |
| [`pthread_rwlock_timedrdlock`](#pthread_rwlock_timedrdlock) | fn | See the [module comment](self). |
| [`pthread_rwlock_timedwrlock`](#pthread_rwlock_timedwrlock) | fn | See the [module comment](self). |
| [`pthread_rwlock_tryrdlock`](#pthread_rwlock_tryrdlock) | fn | See the [module comment](self). |
| [`pthread_rwlock_trywrlock`](#pthread_rwlock_trywrlock) | fn | See the [module comment](self). |
| [`pthread_rwlock_unlock`](#pthread_rwlock_unlock) | fn | See the [module comment](self). |
| [`pthread_rwlock_wrlock`](#pthread_rwlock_wrlock) | fn | See the [module comment](self). |
| [`pthread_self`](#pthread_self) | fn | See the [module comment](self). |
| [`pthread_setaffinity_np`](#pthread_setaffinity_np) | fn | See the [module comment](self). |
| [`pthread_setattr_default_np`](#pthread_setattr_default_np) | fn | See the [module comment](self). |
| [`pthread_setcancelstate`](#pthread_setcancelstate) | fn | See the [module comment](self). |
| [`pthread_setcanceltype`](#pthread_setcanceltype) | fn | See the [module comment](self). |
| [`pthread_setconcurrency`](#pthread_setconcurrency) | fn | See the [module comment](self). |
| [`pthread_setname_np`](#pthread_setname_np) | fn | See the [module comment](self). |
| [`pthread_setschedparam`](#pthread_setschedparam) | fn | See the [module comment](self). |
| [`pthread_setschedprio`](#pthread_setschedprio) | fn | See the [module comment](self). |
| [`pthread_setspecific`](#pthread_setspecific) | fn | See the [module comment](self). |
| [`pthread_sigmask`](#pthread_sigmask) | fn | See the [module comment](self). |
| [`pthread_sigqueue`](#pthread_sigqueue) | fn | See the [module comment](self). |
| [`pthread_spin_destroy`](#pthread_spin_destroy) | fn | See the [module comment](self). |
| [`pthread_spin_init`](#pthread_spin_init) | fn | See the [module comment](self). |
| [`pthread_spin_lock`](#pthread_spin_lock) | fn | See the [module comment](self). |
| [`pthread_spin_trylock`](#pthread_spin_trylock) | fn | See the [module comment](self). |
| [`pthread_spin_unlock`](#pthread_spin_unlock) | fn | See the [module comment](self). |
| [`pthread_testcancel`](#pthread_testcancel) | fn | See the [module comment](self). |
| [`pthread_timedjoin_np`](#pthread_timedjoin_np) | fn | See the [module comment](self). |
| [`pthread_tryjoin_np`](#pthread_tryjoin_np) | fn | See the [module comment](self). |
| [`pthread_yield`](#pthread_yield) | fn | See the [module comment](self). |

## Functions

### `exit`

```rust
fn exit()
```

See the [module comment](self).

### `fork`

```rust
fn fork()
```

See the [module comment](self).

### `clone`

```rust
fn clone()
```

See the [module comment](self).

### `clone3`

```rust
fn clone3()
```

See the [module comment](self).

### `brk`

```rust
fn brk()
```

See the [module comment](self).

### `sigaction`

```rust
fn sigaction()
```

See the [module comment](self).

### `sigaltstack`

```rust
fn sigaltstack()
```

See the [module comment](self).

### `sigprocmask`

```rust
fn sigprocmask()
```

See the [module comment](self).

### `sigwait`

```rust
fn sigwait()
```

See the [module comment](self).

### `sigwaitinfo`

```rust
fn sigwaitinfo()
```

See the [module comment](self).

### `sigtimedwait`

```rust
fn sigtimedwait()
```

See the [module comment](self).

### `set_thread_area`

```rust
fn set_thread_area()
```

See the [module comment](self).

### `set_tid_address`

```rust
fn set_tid_address()
```

See the [module comment](self).

### `tkill`

```rust
fn tkill()
```

See the [module comment](self).

### `sched_setscheduler`

```rust
fn sched_setscheduler()
```

See the [module comment](self).

### `rseq`

```rust
fn rseq()
```

See the [module comment](self).

### `setuid`

```rust
fn setuid()
```

See the [module comment](self).

### `setgid`

```rust
fn setgid()
```

See the [module comment](self).

### `seteuid`

```rust
fn seteuid()
```

See the [module comment](self).

### `setegid`

```rust
fn setegid()
```

See the [module comment](self).

### `setreuid`

```rust
fn setreuid()
```

See the [module comment](self).

### `setregid`

```rust
fn setregid()
```

See the [module comment](self).

### `setresuid`

```rust
fn setresuid()
```

See the [module comment](self).

### `setresgid`

```rust
fn setresgid()
```

See the [module comment](self).

### `setgroups`

```rust
fn setgroups()
```

See the [module comment](self).

### `pthread_atfork`

```rust
fn pthread_atfork()
```

See the [module comment](self).

### `pthread_attr_destroy`

```rust
fn pthread_attr_destroy()
```

See the [module comment](self).

### `pthread_attr_getaffinity_np`

```rust
fn pthread_attr_getaffinity_np()
```

See the [module comment](self).

### `pthread_attr_getdetachstate`

```rust
fn pthread_attr_getdetachstate()
```

See the [module comment](self).

### `pthread_attr_getguardsize`

```rust
fn pthread_attr_getguardsize()
```

See the [module comment](self).

### `pthread_attr_getinheritsched`

```rust
fn pthread_attr_getinheritsched()
```

See the [module comment](self).

### `pthread_attr_getschedparam`

```rust
fn pthread_attr_getschedparam()
```

See the [module comment](self).

### `pthread_attr_getschedpolicy`

```rust
fn pthread_attr_getschedpolicy()
```

See the [module comment](self).

### `pthread_attr_getscope`

```rust
fn pthread_attr_getscope()
```

See the [module comment](self).

### `pthread_attr_getsigmask_np`

```rust
fn pthread_attr_getsigmask_np()
```

See the [module comment](self).

### `pthread_attr_getstack`

```rust
fn pthread_attr_getstack()
```

See the [module comment](self).

### `pthread_attr_getstackaddr`

```rust
fn pthread_attr_getstackaddr()
```

See the [module comment](self).

### `pthread_attr_getstacksize`

```rust
fn pthread_attr_getstacksize()
```

See the [module comment](self).

### `pthread_attr_init`

```rust
fn pthread_attr_init()
```

See the [module comment](self).

### `pthread_attr_setaffinity_np`

```rust
fn pthread_attr_setaffinity_np()
```

See the [module comment](self).

### `pthread_attr_setdetachstate`

```rust
fn pthread_attr_setdetachstate()
```

See the [module comment](self).

### `pthread_attr_setguardsize`

```rust
fn pthread_attr_setguardsize()
```

See the [module comment](self).

### `pthread_attr_setinheritsched`

```rust
fn pthread_attr_setinheritsched()
```

See the [module comment](self).

### `pthread_attr_setschedparam`

```rust
fn pthread_attr_setschedparam()
```

See the [module comment](self).

### `pthread_attr_setschedpolicy`

```rust
fn pthread_attr_setschedpolicy()
```

See the [module comment](self).

### `pthread_attr_setscope`

```rust
fn pthread_attr_setscope()
```

See the [module comment](self).

### `pthread_attr_setsigmask_np`

```rust
fn pthread_attr_setsigmask_np()
```

See the [module comment](self).

### `pthread_attr_setstack`

```rust
fn pthread_attr_setstack()
```

See the [module comment](self).

### `pthread_attr_setstackaddr`

```rust
fn pthread_attr_setstackaddr()
```

See the [module comment](self).

### `pthread_attr_setstacksize`

```rust
fn pthread_attr_setstacksize()
```

See the [module comment](self).

### `pthread_barrierattr_destroy`

```rust
fn pthread_barrierattr_destroy()
```

See the [module comment](self).

### `pthread_barrierattr_getpshared`

```rust
fn pthread_barrierattr_getpshared()
```

See the [module comment](self).

### `pthread_barrierattr_init`

```rust
fn pthread_barrierattr_init()
```

See the [module comment](self).

### `pthread_barrierattr_setpshared`

```rust
fn pthread_barrierattr_setpshared()
```

See the [module comment](self).

### `pthread_barrier_destroy`

```rust
fn pthread_barrier_destroy()
```

See the [module comment](self).

### `pthread_barrier_wait`

```rust
fn pthread_barrier_wait()
```

See the [module comment](self).

### `pthread_cancel`

```rust
fn pthread_cancel()
```

See the [module comment](self).

### `pthread_cleanup_pop`

```rust
fn pthread_cleanup_pop()
```

See the [module comment](self).

### `pthread_cleanup_pop_restore_np`

```rust
fn pthread_cleanup_pop_restore_np()
```

See the [module comment](self).

### `pthread_cleanup_push`

```rust
fn pthread_cleanup_push()
```

See the [module comment](self).

### `pthread_cleanup_push_defer_np`

```rust
fn pthread_cleanup_push_defer_np()
```

See the [module comment](self).

### `pthread_condattr_destroy`

```rust
fn pthread_condattr_destroy()
```

See the [module comment](self).

### `pthread_condattr_getclock`

```rust
fn pthread_condattr_getclock()
```

See the [module comment](self).

### `pthread_condattr_getpshared`

```rust
fn pthread_condattr_getpshared()
```

See the [module comment](self).

### `pthread_condattr_init`

```rust
fn pthread_condattr_init()
```

See the [module comment](self).

### `pthread_condattr_setclock`

```rust
fn pthread_condattr_setclock()
```

See the [module comment](self).

### `pthread_condattr_setpshared`

```rust
fn pthread_condattr_setpshared()
```

See the [module comment](self).

### `pthread_cond_broadcast`

```rust
fn pthread_cond_broadcast()
```

See the [module comment](self).

### `pthread_cond_destroy`

```rust
fn pthread_cond_destroy()
```

See the [module comment](self).

### `pthread_cond_signal`

```rust
fn pthread_cond_signal()
```

See the [module comment](self).

### `pthread_cond_timedwait`

```rust
fn pthread_cond_timedwait()
```

See the [module comment](self).

### `pthread_create`

```rust
fn pthread_create()
```

See the [module comment](self).

### `pthread_detach`

```rust
fn pthread_detach()
```

See the [module comment](self).

### `pthread_equal`

```rust
fn pthread_equal()
```

See the [module comment](self).

### `pthread_exit`

```rust
fn pthread_exit()
```

See the [module comment](self).

### `pthread_getaffinity_np`

```rust
fn pthread_getaffinity_np()
```

See the [module comment](self).

### `pthread_getattr_default_np`

```rust
fn pthread_getattr_default_np()
```

See the [module comment](self).

### `pthread_getattr_np`

```rust
fn pthread_getattr_np()
```

See the [module comment](self).

### `pthread_getconcurrency`

```rust
fn pthread_getconcurrency()
```

See the [module comment](self).

### `pthread_getcpuclockid`

```rust
fn pthread_getcpuclockid()
```

See the [module comment](self).

### `pthread_getname_np`

```rust
fn pthread_getname_np()
```

See the [module comment](self).

### `pthread_getschedparam`

```rust
fn pthread_getschedparam()
```

See the [module comment](self).

### `pthread_getspecific`

```rust
fn pthread_getspecific()
```

See the [module comment](self).

### `pthread_join`

```rust
fn pthread_join()
```

See the [module comment](self).

### `pthread_key_create`

```rust
fn pthread_key_create()
```

See the [module comment](self).

### `pthread_key_delete`

```rust
fn pthread_key_delete()
```

See the [module comment](self).

### `pthread_kill`

```rust
fn pthread_kill()
```

See the [module comment](self).

### `pthread_kill_other_threads_np`

```rust
fn pthread_kill_other_threads_np()
```

See the [module comment](self).

### `pthread_mutexattr_destroy`

```rust
fn pthread_mutexattr_destroy()
```

See the [module comment](self).

### `pthread_mutexattr_getprioceiling`

```rust
fn pthread_mutexattr_getprioceiling()
```

See the [module comment](self).

### `pthread_mutexattr_getprotocol`

```rust
fn pthread_mutexattr_getprotocol()
```

See the [module comment](self).

### `pthread_mutexattr_getpshared`

```rust
fn pthread_mutexattr_getpshared()
```

See the [module comment](self).

### `pthread_mutexattr_getrobust`

```rust
fn pthread_mutexattr_getrobust()
```

See the [module comment](self).

### `pthread_mutexattr_getrobust_np`

```rust
fn pthread_mutexattr_getrobust_np()
```

See the [module comment](self).

### `pthread_mutexattr_gettype`

```rust
fn pthread_mutexattr_gettype()
```

See the [module comment](self).

### `pthread_mutexattr_init`

```rust
fn pthread_mutexattr_init()
```

See the [module comment](self).

### `pthread_mutexattr_setprioceiling`

```rust
fn pthread_mutexattr_setprioceiling()
```

See the [module comment](self).

### `pthread_mutexattr_setprotocol`

```rust
fn pthread_mutexattr_setprotocol()
```

See the [module comment](self).

### `pthread_mutexattr_setpshared`

```rust
fn pthread_mutexattr_setpshared()
```

See the [module comment](self).

### `pthread_mutexattr_setrobust`

```rust
fn pthread_mutexattr_setrobust()
```

See the [module comment](self).

### `pthread_mutexattr_setrobust_np`

```rust
fn pthread_mutexattr_setrobust_np()
```

See the [module comment](self).

### `pthread_mutexattr_settype`

```rust
fn pthread_mutexattr_settype()
```

See the [module comment](self).

### `pthread_mutex_consistent`

```rust
fn pthread_mutex_consistent()
```

See the [module comment](self).

### `pthread_mutex_consistent_np`

```rust
fn pthread_mutex_consistent_np()
```

See the [module comment](self).

### `pthread_mutex_destroy`

```rust
fn pthread_mutex_destroy()
```

See the [module comment](self).

### `pthread_mutex_getprioceiling`

```rust
fn pthread_mutex_getprioceiling()
```

See the [module comment](self).

### `pthread_mutex_init`

```rust
fn pthread_mutex_init()
```

See the [module comment](self).

### `pthread_mutex_lock`

```rust
fn pthread_mutex_lock()
```

See the [module comment](self).

### `pthread_mutex_setprioceiling`

```rust
fn pthread_mutex_setprioceiling()
```

See the [module comment](self).

### `pthread_mutex_timedlock`

```rust
fn pthread_mutex_timedlock()
```

See the [module comment](self).

### `pthread_mutex_trylock`

```rust
fn pthread_mutex_trylock()
```

See the [module comment](self).

### `pthread_once`

```rust
fn pthread_once()
```

See the [module comment](self).

### `pthread_rwlockattr_destroy`

```rust
fn pthread_rwlockattr_destroy()
```

See the [module comment](self).

### `pthread_rwlockattr_getkind_np`

```rust
fn pthread_rwlockattr_getkind_np()
```

See the [module comment](self).

### `pthread_rwlockattr_getpshared`

```rust
fn pthread_rwlockattr_getpshared()
```

See the [module comment](self).

### `pthread_rwlockattr_init`

```rust
fn pthread_rwlockattr_init()
```

See the [module comment](self).

### `pthread_rwlockattr_setkind_np`

```rust
fn pthread_rwlockattr_setkind_np()
```

See the [module comment](self).

### `pthread_rwlockattr_setpshared`

```rust
fn pthread_rwlockattr_setpshared()
```

See the [module comment](self).

### `pthread_rwlock_destroy`

```rust
fn pthread_rwlock_destroy()
```

See the [module comment](self).

### `pthread_rwlock_rdlock`

```rust
fn pthread_rwlock_rdlock()
```

See the [module comment](self).

### `pthread_rwlock_timedrdlock`

```rust
fn pthread_rwlock_timedrdlock()
```

See the [module comment](self).

### `pthread_rwlock_timedwrlock`

```rust
fn pthread_rwlock_timedwrlock()
```

See the [module comment](self).

### `pthread_rwlock_tryrdlock`

```rust
fn pthread_rwlock_tryrdlock()
```

See the [module comment](self).

### `pthread_rwlock_trywrlock`

```rust
fn pthread_rwlock_trywrlock()
```

See the [module comment](self).

### `pthread_rwlock_unlock`

```rust
fn pthread_rwlock_unlock()
```

See the [module comment](self).

### `pthread_rwlock_wrlock`

```rust
fn pthread_rwlock_wrlock()
```

See the [module comment](self).

### `pthread_self`

```rust
fn pthread_self()
```

See the [module comment](self).

### `pthread_setaffinity_np`

```rust
fn pthread_setaffinity_np()
```

See the [module comment](self).

### `pthread_setattr_default_np`

```rust
fn pthread_setattr_default_np()
```

See the [module comment](self).

### `pthread_setcancelstate`

```rust
fn pthread_setcancelstate()
```

See the [module comment](self).

### `pthread_setcanceltype`

```rust
fn pthread_setcanceltype()
```

See the [module comment](self).

### `pthread_setconcurrency`

```rust
fn pthread_setconcurrency()
```

See the [module comment](self).

### `pthread_setname_np`

```rust
fn pthread_setname_np()
```

See the [module comment](self).

### `pthread_setschedparam`

```rust
fn pthread_setschedparam()
```

See the [module comment](self).

### `pthread_setschedprio`

```rust
fn pthread_setschedprio()
```

See the [module comment](self).

### `pthread_setspecific`

```rust
fn pthread_setspecific()
```

See the [module comment](self).

### `pthread_sigmask`

```rust
fn pthread_sigmask()
```

See the [module comment](self).

### `pthread_sigqueue`

```rust
fn pthread_sigqueue()
```

See the [module comment](self).

### `pthread_spin_destroy`

```rust
fn pthread_spin_destroy()
```

See the [module comment](self).

### `pthread_spin_init`

```rust
fn pthread_spin_init()
```

See the [module comment](self).

### `pthread_spin_lock`

```rust
fn pthread_spin_lock()
```

See the [module comment](self).

### `pthread_spin_trylock`

```rust
fn pthread_spin_trylock()
```

See the [module comment](self).

### `pthread_spin_unlock`

```rust
fn pthread_spin_unlock()
```

See the [module comment](self).

### `pthread_testcancel`

```rust
fn pthread_testcancel()
```

See the [module comment](self).

### `pthread_timedjoin_np`

```rust
fn pthread_timedjoin_np()
```

See the [module comment](self).

### `pthread_tryjoin_np`

```rust
fn pthread_tryjoin_np()
```

See the [module comment](self).

### `pthread_yield`

```rust
fn pthread_yield()
```

See the [module comment](self).

