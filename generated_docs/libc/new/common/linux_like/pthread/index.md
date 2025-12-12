*[libc](../../../../index.md) / [new](../../../index.md) / [common](../../index.md) / [linux_like](../index.md) / [pthread](index.md)*

---

# Module `pthread`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pthread_getaffinity_np`](#pthread-getaffinity-np) | fn |  |
| [`pthread_getattr_np`](#pthread-getattr-np) | fn |  |
| [`pthread_getname_np`](#pthread-getname-np) | fn |  |
| [`pthread_setaffinity_np`](#pthread-setaffinity-np) | fn |  |
| [`pthread_setname_np`](#pthread-setname-np) | fn |  |

## Functions

### `pthread_getaffinity_np`

```rust
unsafe fn pthread_getaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:5-9`](../../../../../../.source_1765210505/libc-0.2.178/src/new/common/linux_like/pthread.rs#L5-L9)*

### `pthread_getattr_np`

```rust
unsafe fn pthread_getattr_np(native: crate::pthread_t, attr: *mut crate::pthread_attr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:11`](../../../../../../.source_1765210505/libc-0.2.178/src/new/common/linux_like/pthread.rs#L11)*

### `pthread_getname_np`

```rust
unsafe fn pthread_getname_np(thread: crate::pthread_t, name: *mut c_char, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:14`](../../../../../../.source_1765210505/libc-0.2.178/src/new/common/linux_like/pthread.rs#L14)*

### `pthread_setaffinity_np`

```rust
unsafe fn pthread_setaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:17-21`](../../../../../../.source_1765210505/libc-0.2.178/src/new/common/linux_like/pthread.rs#L17-L21)*

### `pthread_setname_np`

```rust
unsafe fn pthread_setname_np(thread: crate::pthread_t, name: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/new/common/linux_like/pthread.rs:24`](../../../../../../.source_1765210505/libc-0.2.178/src/new/common/linux_like/pthread.rs#L24)*

