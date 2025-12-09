*[libc](../../../../../index.md) / [new](../../../../index.md) / [glibc](../../../index.md) / [sysdeps](../../index.md) / [nptl](../index.md) / [pthread](index.md)*

---

# Module `pthread`

Source header: `sysdeps/nptl/pthread.h`

<https://github.com/bminor/glibc/blob/master/sysdeps/nptl/pthread.h>

## Contents

- [Functions](#functions)
  - [`pthread_getaffinity_np`](#pthread_getaffinity_np)
  - [`pthread_getattr_np`](#pthread_getattr_np)
  - [`pthread_getname_np`](#pthread_getname_np)
  - [`pthread_setaffinity_np`](#pthread_setaffinity_np)
  - [`pthread_setname_np`](#pthread_setname_np)
  - [`pthread_atfork`](#pthread_atfork)
  - [`pthread_attr_getguardsize`](#pthread_attr_getguardsize)
  - [`pthread_attr_getinheritsched`](#pthread_attr_getinheritsched)
  - [`pthread_attr_getschedparam`](#pthread_attr_getschedparam)
  - [`pthread_attr_getschedpolicy`](#pthread_attr_getschedpolicy)
  - [`pthread_attr_getstack`](#pthread_attr_getstack)
  - [`pthread_attr_setguardsize`](#pthread_attr_setguardsize)
  - [`pthread_attr_setinheritsched`](#pthread_attr_setinheritsched)
  - [`pthread_attr_setschedparam`](#pthread_attr_setschedparam)
  - [`pthread_attr_setschedpolicy`](#pthread_attr_setschedpolicy)
  - [`pthread_attr_setstack`](#pthread_attr_setstack)
  - [`pthread_barrier_destroy`](#pthread_barrier_destroy)
  - [`pthread_barrier_init`](#pthread_barrier_init)
  - [`pthread_barrier_wait`](#pthread_barrier_wait)
  - [`pthread_barrierattr_destroy`](#pthread_barrierattr_destroy)
  - [`pthread_barrierattr_getpshared`](#pthread_barrierattr_getpshared)
  - [`pthread_barrierattr_init`](#pthread_barrierattr_init)
  - [`pthread_barrierattr_setpshared`](#pthread_barrierattr_setpshared)
  - [`pthread_cancel`](#pthread_cancel)
  - [`pthread_condattr_getclock`](#pthread_condattr_getclock)
  - [`pthread_condattr_getpshared`](#pthread_condattr_getpshared)
  - [`pthread_condattr_setclock`](#pthread_condattr_setclock)
  - [`pthread_condattr_setpshared`](#pthread_condattr_setpshared)
  - [`pthread_create`](#pthread_create)
  - [`pthread_getcpuclockid`](#pthread_getcpuclockid)
  - [`pthread_getschedparam`](#pthread_getschedparam)
  - [`pthread_kill`](#pthread_kill)
  - [`pthread_mutex_consistent`](#pthread_mutex_consistent)
  - [`pthread_mutex_timedlock`](#pthread_mutex_timedlock)
  - [`pthread_mutexattr_getprotocol`](#pthread_mutexattr_getprotocol)
  - [`pthread_mutexattr_getpshared`](#pthread_mutexattr_getpshared)
  - [`pthread_mutexattr_getrobust`](#pthread_mutexattr_getrobust)
  - [`pthread_mutexattr_setprotocol`](#pthread_mutexattr_setprotocol)
  - [`pthread_mutexattr_setpshared`](#pthread_mutexattr_setpshared)
  - [`pthread_mutexattr_setrobust`](#pthread_mutexattr_setrobust)
  - [`pthread_once`](#pthread_once)
  - [`pthread_rwlockattr_getpshared`](#pthread_rwlockattr_getpshared)
  - [`pthread_rwlockattr_setpshared`](#pthread_rwlockattr_setpshared)
  - [`pthread_setschedparam`](#pthread_setschedparam)
  - [`pthread_setschedprio`](#pthread_setschedprio)
  - [`pthread_sigmask`](#pthread_sigmask)
  - [`pthread_spin_destroy`](#pthread_spin_destroy)
  - [`pthread_spin_init`](#pthread_spin_init)
  - [`pthread_spin_lock`](#pthread_spin_lock)
  - [`pthread_spin_trylock`](#pthread_spin_trylock)
  - [`pthread_spin_unlock`](#pthread_spin_unlock)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pthread_getaffinity_np`](#pthread_getaffinity_np) | fn |  |
| [`pthread_getattr_np`](#pthread_getattr_np) | fn |  |
| [`pthread_getname_np`](#pthread_getname_np) | fn |  |
| [`pthread_setaffinity_np`](#pthread_setaffinity_np) | fn |  |
| [`pthread_setname_np`](#pthread_setname_np) | fn |  |
| [`pthread_atfork`](#pthread_atfork) | fn |  |
| [`pthread_attr_getguardsize`](#pthread_attr_getguardsize) | fn |  |
| [`pthread_attr_getinheritsched`](#pthread_attr_getinheritsched) | fn |  |
| [`pthread_attr_getschedparam`](#pthread_attr_getschedparam) | fn |  |
| [`pthread_attr_getschedpolicy`](#pthread_attr_getschedpolicy) | fn |  |
| [`pthread_attr_getstack`](#pthread_attr_getstack) | fn |  |
| [`pthread_attr_setguardsize`](#pthread_attr_setguardsize) | fn |  |
| [`pthread_attr_setinheritsched`](#pthread_attr_setinheritsched) | fn |  |
| [`pthread_attr_setschedparam`](#pthread_attr_setschedparam) | fn |  |
| [`pthread_attr_setschedpolicy`](#pthread_attr_setschedpolicy) | fn |  |
| [`pthread_attr_setstack`](#pthread_attr_setstack) | fn |  |
| [`pthread_barrier_destroy`](#pthread_barrier_destroy) | fn |  |
| [`pthread_barrier_init`](#pthread_barrier_init) | fn |  |
| [`pthread_barrier_wait`](#pthread_barrier_wait) | fn |  |
| [`pthread_barrierattr_destroy`](#pthread_barrierattr_destroy) | fn |  |
| [`pthread_barrierattr_getpshared`](#pthread_barrierattr_getpshared) | fn |  |
| [`pthread_barrierattr_init`](#pthread_barrierattr_init) | fn |  |
| [`pthread_barrierattr_setpshared`](#pthread_barrierattr_setpshared) | fn |  |
| [`pthread_cancel`](#pthread_cancel) | fn |  |
| [`pthread_condattr_getclock`](#pthread_condattr_getclock) | fn |  |
| [`pthread_condattr_getpshared`](#pthread_condattr_getpshared) | fn |  |
| [`pthread_condattr_setclock`](#pthread_condattr_setclock) | fn |  |
| [`pthread_condattr_setpshared`](#pthread_condattr_setpshared) | fn |  |
| [`pthread_create`](#pthread_create) | fn |  |
| [`pthread_getcpuclockid`](#pthread_getcpuclockid) | fn |  |
| [`pthread_getschedparam`](#pthread_getschedparam) | fn |  |
| [`pthread_kill`](#pthread_kill) | fn |  |
| [`pthread_mutex_consistent`](#pthread_mutex_consistent) | fn |  |
| [`pthread_mutex_timedlock`](#pthread_mutex_timedlock) | fn |  |
| [`pthread_mutexattr_getprotocol`](#pthread_mutexattr_getprotocol) | fn |  |
| [`pthread_mutexattr_getpshared`](#pthread_mutexattr_getpshared) | fn |  |
| [`pthread_mutexattr_getrobust`](#pthread_mutexattr_getrobust) | fn |  |
| [`pthread_mutexattr_setprotocol`](#pthread_mutexattr_setprotocol) | fn |  |
| [`pthread_mutexattr_setpshared`](#pthread_mutexattr_setpshared) | fn |  |
| [`pthread_mutexattr_setrobust`](#pthread_mutexattr_setrobust) | fn |  |
| [`pthread_once`](#pthread_once) | fn |  |
| [`pthread_rwlockattr_getpshared`](#pthread_rwlockattr_getpshared) | fn |  |
| [`pthread_rwlockattr_setpshared`](#pthread_rwlockattr_setpshared) | fn |  |
| [`pthread_setschedparam`](#pthread_setschedparam) | fn |  |
| [`pthread_setschedprio`](#pthread_setschedprio) | fn |  |
| [`pthread_sigmask`](#pthread_sigmask) | fn |  |
| [`pthread_spin_destroy`](#pthread_spin_destroy) | fn |  |
| [`pthread_spin_init`](#pthread_spin_init) | fn |  |
| [`pthread_spin_lock`](#pthread_spin_lock) | fn |  |
| [`pthread_spin_trylock`](#pthread_spin_trylock) | fn |  |
| [`pthread_spin_unlock`](#pthread_spin_unlock) | fn |  |

## Functions

