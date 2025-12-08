*[libc](../../../../index.md) / [new](../../../index.md) / [common](../../index.md) / [linux_like](../index.md) / [pthread](index.md)*

---

# Module `pthread`

## Functions

### `pthread_getaffinity_np`

```rust
unsafe fn pthread_getaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```

### `pthread_getattr_np`

```rust
unsafe fn pthread_getattr_np(native: crate::pthread_t, attr: *mut crate::pthread_attr_t) -> c_int
```

### `pthread_getname_np`

```rust
unsafe fn pthread_getname_np(thread: crate::pthread_t, name: *mut c_char, len: size_t) -> c_int
```

### `pthread_setaffinity_np`

```rust
unsafe fn pthread_setaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```

### `pthread_setname_np`

```rust
unsafe fn pthread_setname_np(thread: crate::pthread_t, name: *const c_char) -> c_int
```

