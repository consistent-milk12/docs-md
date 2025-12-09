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

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:6`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L6)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:7`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L7)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:8`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L8)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:9`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L9)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:10`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L10)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:13`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L13)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:14`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L14)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:15`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L15)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:16`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L16)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:17`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L17)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:18`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L18)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:19`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L19)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:20`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L20)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:21`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L21)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:22`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L22)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:23`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L23)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:24`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L24)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:25`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L25)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:26`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L26)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:27`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L27)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:28`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L28)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:29`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L29)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:30`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L30)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:31`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L31)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:32`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L32)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:33`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L33)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:34`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L34)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:35`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L35)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:36`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L36)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:37`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L37)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:38`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L38)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:39`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L39)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:40`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L40)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:41`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L41)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:42`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L42)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:43`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L43)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:44`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L44)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:45`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L45)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:46`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L46)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:47`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L47)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:48`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L48)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:49`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L49)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:50`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L50)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:51`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L51)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:52`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L52)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:53`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L53)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:54`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L54)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:55`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L55)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:56`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L56)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:57`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L57)*

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs:58`](../../../../../../../.source_1765210505/libc-0.2.178/src/new/glibc/sysdeps/nptl/pthread.rs#L58)*

