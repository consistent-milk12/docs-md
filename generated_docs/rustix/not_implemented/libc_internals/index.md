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

*Defined in [`rustix-1.1.2/src/not_implemented.rs:67`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L67)*

See the [module comment](self).

### `fork`

```rust
fn fork()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:68`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L68)*

See the [module comment](self).

### `clone`

```rust
fn clone()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:69`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L69)*

See the [module comment](self).

### `clone3`

```rust
fn clone3()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:70`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L70)*

See the [module comment](self).

### `brk`

```rust
fn brk()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:71`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L71)*

See the [module comment](self).

### `sigaction`

```rust
fn sigaction()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:72`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L72)*

See the [module comment](self).

### `sigaltstack`

```rust
fn sigaltstack()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:73`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L73)*

See the [module comment](self).

### `sigprocmask`

```rust
fn sigprocmask()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:74`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L74)*

See the [module comment](self).

### `sigwait`

```rust
fn sigwait()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:75`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L75)*

See the [module comment](self).

### `sigwaitinfo`

```rust
fn sigwaitinfo()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:76`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L76)*

See the [module comment](self).

### `sigtimedwait`

```rust
fn sigtimedwait()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:77`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L77)*

See the [module comment](self).

### `set_thread_area`

```rust
fn set_thread_area()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:78`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L78)*

See the [module comment](self).

### `set_tid_address`

```rust
fn set_tid_address()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:79`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L79)*

See the [module comment](self).

### `tkill`

```rust
fn tkill()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:80`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L80)*

See the [module comment](self).

### `sched_setscheduler`

```rust
fn sched_setscheduler()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:81`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L81)*

See the [module comment](self).

### `rseq`

```rust
fn rseq()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:82`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L82)*

See the [module comment](self).

### `setuid`

```rust
fn setuid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:83`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L83)*

See the [module comment](self).

### `setgid`

```rust
fn setgid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:84`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L84)*

See the [module comment](self).

### `seteuid`

```rust
fn seteuid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:85`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L85)*

See the [module comment](self).

### `setegid`

```rust
fn setegid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:86`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L86)*

See the [module comment](self).

### `setreuid`

```rust
fn setreuid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:87`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L87)*

See the [module comment](self).

### `setregid`

```rust
fn setregid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:88`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L88)*

See the [module comment](self).

### `setresuid`

```rust
fn setresuid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:89`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L89)*

See the [module comment](self).

### `setresgid`

```rust
fn setresgid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:90`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L90)*

See the [module comment](self).

### `setgroups`

```rust
fn setgroups()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:91`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L91)*

See the [module comment](self).

### `pthread_atfork`

```rust
fn pthread_atfork()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:93`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L93)*

See the [module comment](self).

### `pthread_attr_destroy`

```rust
fn pthread_attr_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:94`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L94)*

See the [module comment](self).

### `pthread_attr_getaffinity_np`

```rust
fn pthread_attr_getaffinity_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:95`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L95)*

See the [module comment](self).

### `pthread_attr_getdetachstate`

```rust
fn pthread_attr_getdetachstate()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:96`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L96)*

See the [module comment](self).

### `pthread_attr_getguardsize`

```rust
fn pthread_attr_getguardsize()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:97`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L97)*

See the [module comment](self).

### `pthread_attr_getinheritsched`

```rust
fn pthread_attr_getinheritsched()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:98`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L98)*

See the [module comment](self).

### `pthread_attr_getschedparam`

```rust
fn pthread_attr_getschedparam()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:99`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L99)*

See the [module comment](self).

### `pthread_attr_getschedpolicy`

```rust
fn pthread_attr_getschedpolicy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:100`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L100)*

See the [module comment](self).

### `pthread_attr_getscope`

```rust
fn pthread_attr_getscope()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:101`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L101)*

See the [module comment](self).

### `pthread_attr_getsigmask_np`

```rust
fn pthread_attr_getsigmask_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:102`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L102)*

See the [module comment](self).

### `pthread_attr_getstack`

```rust
fn pthread_attr_getstack()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:103`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L103)*

See the [module comment](self).

### `pthread_attr_getstackaddr`

```rust
fn pthread_attr_getstackaddr()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:104`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L104)*

See the [module comment](self).

### `pthread_attr_getstacksize`

```rust
fn pthread_attr_getstacksize()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:105`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L105)*

See the [module comment](self).

### `pthread_attr_init`

```rust
fn pthread_attr_init()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:106`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L106)*

See the [module comment](self).

### `pthread_attr_setaffinity_np`

```rust
fn pthread_attr_setaffinity_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:107`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L107)*

See the [module comment](self).

### `pthread_attr_setdetachstate`

```rust
fn pthread_attr_setdetachstate()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:108`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L108)*

See the [module comment](self).

### `pthread_attr_setguardsize`

```rust
fn pthread_attr_setguardsize()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:109`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L109)*

See the [module comment](self).

### `pthread_attr_setinheritsched`

```rust
fn pthread_attr_setinheritsched()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:110`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L110)*

See the [module comment](self).

### `pthread_attr_setschedparam`

```rust
fn pthread_attr_setschedparam()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:111`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L111)*

See the [module comment](self).

### `pthread_attr_setschedpolicy`

```rust
fn pthread_attr_setschedpolicy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:112`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L112)*

See the [module comment](self).

### `pthread_attr_setscope`

```rust
fn pthread_attr_setscope()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:113`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L113)*

See the [module comment](self).

### `pthread_attr_setsigmask_np`

```rust
fn pthread_attr_setsigmask_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:114`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L114)*

See the [module comment](self).

### `pthread_attr_setstack`

```rust
fn pthread_attr_setstack()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:115`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L115)*

See the [module comment](self).

### `pthread_attr_setstackaddr`

```rust
fn pthread_attr_setstackaddr()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:116`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L116)*

See the [module comment](self).

### `pthread_attr_setstacksize`

```rust
fn pthread_attr_setstacksize()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:117`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L117)*

See the [module comment](self).

### `pthread_barrierattr_destroy`

```rust
fn pthread_barrierattr_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:118`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L118)*

See the [module comment](self).

### `pthread_barrierattr_getpshared`

```rust
fn pthread_barrierattr_getpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:119`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L119)*

See the [module comment](self).

### `pthread_barrierattr_init`

```rust
fn pthread_barrierattr_init()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:120`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L120)*

See the [module comment](self).

### `pthread_barrierattr_setpshared`

```rust
fn pthread_barrierattr_setpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:121`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L121)*

See the [module comment](self).

### `pthread_barrier_destroy`

```rust
fn pthread_barrier_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:122`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L122)*

See the [module comment](self).

### `pthread_barrier_wait`

```rust
fn pthread_barrier_wait()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:123`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L123)*

See the [module comment](self).

### `pthread_cancel`

```rust
fn pthread_cancel()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:124`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L124)*

See the [module comment](self).

### `pthread_cleanup_pop`

```rust
fn pthread_cleanup_pop()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:125`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L125)*

See the [module comment](self).

### `pthread_cleanup_pop_restore_np`

```rust
fn pthread_cleanup_pop_restore_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:126`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L126)*

See the [module comment](self).

### `pthread_cleanup_push`

```rust
fn pthread_cleanup_push()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:127`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L127)*

See the [module comment](self).

### `pthread_cleanup_push_defer_np`

```rust
fn pthread_cleanup_push_defer_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:128`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L128)*

See the [module comment](self).

### `pthread_condattr_destroy`

```rust
fn pthread_condattr_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:129`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L129)*

See the [module comment](self).

### `pthread_condattr_getclock`

```rust
fn pthread_condattr_getclock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:130`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L130)*

See the [module comment](self).

### `pthread_condattr_getpshared`

```rust
fn pthread_condattr_getpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:131`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L131)*

See the [module comment](self).

### `pthread_condattr_init`

```rust
fn pthread_condattr_init()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:132`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L132)*

See the [module comment](self).

### `pthread_condattr_setclock`

```rust
fn pthread_condattr_setclock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:133`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L133)*

See the [module comment](self).

### `pthread_condattr_setpshared`

```rust
fn pthread_condattr_setpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:134`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L134)*

See the [module comment](self).

### `pthread_cond_broadcast`

```rust
fn pthread_cond_broadcast()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:135`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L135)*

See the [module comment](self).

### `pthread_cond_destroy`

```rust
fn pthread_cond_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:136`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L136)*

See the [module comment](self).

### `pthread_cond_signal`

```rust
fn pthread_cond_signal()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:137`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L137)*

See the [module comment](self).

### `pthread_cond_timedwait`

```rust
fn pthread_cond_timedwait()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:138`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L138)*

See the [module comment](self).

### `pthread_create`

```rust
fn pthread_create()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:139`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L139)*

See the [module comment](self).

### `pthread_detach`

```rust
fn pthread_detach()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:140`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L140)*

See the [module comment](self).

### `pthread_equal`

```rust
fn pthread_equal()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:141`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L141)*

See the [module comment](self).

### `pthread_exit`

```rust
fn pthread_exit()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:142`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L142)*

See the [module comment](self).

### `pthread_getaffinity_np`

```rust
fn pthread_getaffinity_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:143`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L143)*

See the [module comment](self).

### `pthread_getattr_default_np`

```rust
fn pthread_getattr_default_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:144`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L144)*

See the [module comment](self).

### `pthread_getattr_np`

```rust
fn pthread_getattr_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:145`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L145)*

See the [module comment](self).

### `pthread_getconcurrency`

```rust
fn pthread_getconcurrency()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:146`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L146)*

See the [module comment](self).

### `pthread_getcpuclockid`

```rust
fn pthread_getcpuclockid()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:147`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L147)*

See the [module comment](self).

### `pthread_getname_np`

```rust
fn pthread_getname_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:148`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L148)*

See the [module comment](self).

### `pthread_getschedparam`

```rust
fn pthread_getschedparam()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:149`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L149)*

See the [module comment](self).

### `pthread_getspecific`

```rust
fn pthread_getspecific()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:150`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L150)*

See the [module comment](self).

### `pthread_join`

```rust
fn pthread_join()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:151`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L151)*

See the [module comment](self).

### `pthread_key_create`

```rust
fn pthread_key_create()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:152`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L152)*

See the [module comment](self).

### `pthread_key_delete`

```rust
fn pthread_key_delete()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:153`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L153)*

See the [module comment](self).

### `pthread_kill`

```rust
fn pthread_kill()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:154`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L154)*

See the [module comment](self).

### `pthread_kill_other_threads_np`

```rust
fn pthread_kill_other_threads_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:155`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L155)*

See the [module comment](self).

### `pthread_mutexattr_destroy`

```rust
fn pthread_mutexattr_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:156`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L156)*

See the [module comment](self).

### `pthread_mutexattr_getprioceiling`

```rust
fn pthread_mutexattr_getprioceiling()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:157`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L157)*

See the [module comment](self).

### `pthread_mutexattr_getprotocol`

```rust
fn pthread_mutexattr_getprotocol()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:158`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L158)*

See the [module comment](self).

### `pthread_mutexattr_getpshared`

```rust
fn pthread_mutexattr_getpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:159`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L159)*

See the [module comment](self).

### `pthread_mutexattr_getrobust`

```rust
fn pthread_mutexattr_getrobust()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:160`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L160)*

See the [module comment](self).

### `pthread_mutexattr_getrobust_np`

```rust
fn pthread_mutexattr_getrobust_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:161`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L161)*

See the [module comment](self).

### `pthread_mutexattr_gettype`

```rust
fn pthread_mutexattr_gettype()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:162`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L162)*

See the [module comment](self).

### `pthread_mutexattr_init`

```rust
fn pthread_mutexattr_init()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:163`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L163)*

See the [module comment](self).

### `pthread_mutexattr_setprioceiling`

```rust
fn pthread_mutexattr_setprioceiling()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:164`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L164)*

See the [module comment](self).

### `pthread_mutexattr_setprotocol`

```rust
fn pthread_mutexattr_setprotocol()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:165`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L165)*

See the [module comment](self).

### `pthread_mutexattr_setpshared`

```rust
fn pthread_mutexattr_setpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:166`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L166)*

See the [module comment](self).

### `pthread_mutexattr_setrobust`

```rust
fn pthread_mutexattr_setrobust()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:167`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L167)*

See the [module comment](self).

### `pthread_mutexattr_setrobust_np`

```rust
fn pthread_mutexattr_setrobust_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:168`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L168)*

See the [module comment](self).

### `pthread_mutexattr_settype`

```rust
fn pthread_mutexattr_settype()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:169`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L169)*

See the [module comment](self).

### `pthread_mutex_consistent`

```rust
fn pthread_mutex_consistent()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:170`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L170)*

See the [module comment](self).

### `pthread_mutex_consistent_np`

```rust
fn pthread_mutex_consistent_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:171`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L171)*

See the [module comment](self).

### `pthread_mutex_destroy`

```rust
fn pthread_mutex_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:172`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L172)*

See the [module comment](self).

### `pthread_mutex_getprioceiling`

```rust
fn pthread_mutex_getprioceiling()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:173`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L173)*

See the [module comment](self).

### `pthread_mutex_init`

```rust
fn pthread_mutex_init()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:174`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L174)*

See the [module comment](self).

### `pthread_mutex_lock`

```rust
fn pthread_mutex_lock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:175`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L175)*

See the [module comment](self).

### `pthread_mutex_setprioceiling`

```rust
fn pthread_mutex_setprioceiling()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:176`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L176)*

See the [module comment](self).

### `pthread_mutex_timedlock`

```rust
fn pthread_mutex_timedlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:177`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L177)*

See the [module comment](self).

### `pthread_mutex_trylock`

```rust
fn pthread_mutex_trylock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:178`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L178)*

See the [module comment](self).

### `pthread_once`

```rust
fn pthread_once()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:179`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L179)*

See the [module comment](self).

### `pthread_rwlockattr_destroy`

```rust
fn pthread_rwlockattr_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:180`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L180)*

See the [module comment](self).

### `pthread_rwlockattr_getkind_np`

```rust
fn pthread_rwlockattr_getkind_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:181`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L181)*

See the [module comment](self).

### `pthread_rwlockattr_getpshared`

```rust
fn pthread_rwlockattr_getpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:182`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L182)*

See the [module comment](self).

### `pthread_rwlockattr_init`

```rust
fn pthread_rwlockattr_init()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:183`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L183)*

See the [module comment](self).

### `pthread_rwlockattr_setkind_np`

```rust
fn pthread_rwlockattr_setkind_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:184`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L184)*

See the [module comment](self).

### `pthread_rwlockattr_setpshared`

```rust
fn pthread_rwlockattr_setpshared()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:185`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L185)*

See the [module comment](self).

### `pthread_rwlock_destroy`

```rust
fn pthread_rwlock_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:186`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L186)*

See the [module comment](self).

### `pthread_rwlock_rdlock`

```rust
fn pthread_rwlock_rdlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:187`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L187)*

See the [module comment](self).

### `pthread_rwlock_timedrdlock`

```rust
fn pthread_rwlock_timedrdlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:188`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L188)*

See the [module comment](self).

### `pthread_rwlock_timedwrlock`

```rust
fn pthread_rwlock_timedwrlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:189`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L189)*

See the [module comment](self).

### `pthread_rwlock_tryrdlock`

```rust
fn pthread_rwlock_tryrdlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:190`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L190)*

See the [module comment](self).

### `pthread_rwlock_trywrlock`

```rust
fn pthread_rwlock_trywrlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:191`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L191)*

See the [module comment](self).

### `pthread_rwlock_unlock`

```rust
fn pthread_rwlock_unlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:192`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L192)*

See the [module comment](self).

### `pthread_rwlock_wrlock`

```rust
fn pthread_rwlock_wrlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:193`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L193)*

See the [module comment](self).

### `pthread_self`

```rust
fn pthread_self()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:194`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L194)*

See the [module comment](self).

### `pthread_setaffinity_np`

```rust
fn pthread_setaffinity_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:195`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L195)*

See the [module comment](self).

### `pthread_setattr_default_np`

```rust
fn pthread_setattr_default_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:196`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L196)*

See the [module comment](self).

### `pthread_setcancelstate`

```rust
fn pthread_setcancelstate()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:197`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L197)*

See the [module comment](self).

### `pthread_setcanceltype`

```rust
fn pthread_setcanceltype()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:198`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L198)*

See the [module comment](self).

### `pthread_setconcurrency`

```rust
fn pthread_setconcurrency()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:199`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L199)*

See the [module comment](self).

### `pthread_setname_np`

```rust
fn pthread_setname_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:200`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L200)*

See the [module comment](self).

### `pthread_setschedparam`

```rust
fn pthread_setschedparam()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:201`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L201)*

See the [module comment](self).

### `pthread_setschedprio`

```rust
fn pthread_setschedprio()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:202`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L202)*

See the [module comment](self).

### `pthread_setspecific`

```rust
fn pthread_setspecific()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:203`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L203)*

See the [module comment](self).

### `pthread_sigmask`

```rust
fn pthread_sigmask()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:204`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L204)*

See the [module comment](self).

### `pthread_sigqueue`

```rust
fn pthread_sigqueue()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:205`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L205)*

See the [module comment](self).

### `pthread_spin_destroy`

```rust
fn pthread_spin_destroy()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:206`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L206)*

See the [module comment](self).

### `pthread_spin_init`

```rust
fn pthread_spin_init()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:207`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L207)*

See the [module comment](self).

### `pthread_spin_lock`

```rust
fn pthread_spin_lock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:208`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L208)*

See the [module comment](self).

### `pthread_spin_trylock`

```rust
fn pthread_spin_trylock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:209`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L209)*

See the [module comment](self).

### `pthread_spin_unlock`

```rust
fn pthread_spin_unlock()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:210`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L210)*

See the [module comment](self).

### `pthread_testcancel`

```rust
fn pthread_testcancel()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:211`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L211)*

See the [module comment](self).

### `pthread_timedjoin_np`

```rust
fn pthread_timedjoin_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:212`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L212)*

See the [module comment](self).

### `pthread_tryjoin_np`

```rust
fn pthread_tryjoin_np()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:213`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L213)*

See the [module comment](self).

### `pthread_yield`

```rust
fn pthread_yield()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:214`](../../../../.source_1765210505/rustix-1.1.2/src/not_implemented.rs#L214)*

See the [module comment](self).

