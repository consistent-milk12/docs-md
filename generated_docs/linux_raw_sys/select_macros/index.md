*[linux_raw_sys](../index.md) / [select_macros](index.md)*

---

# Module `select_macros`

## Functions

### `FD_CLR`

```rust
unsafe fn FD_CLR(fd: crate::ctypes::c_int, set: *mut crate::general::__kernel_fd_set)
```

### `FD_SET`

```rust
unsafe fn FD_SET(fd: crate::ctypes::c_int, set: *mut crate::general::__kernel_fd_set)
```

### `FD_ISSET`

```rust
unsafe fn FD_ISSET(fd: crate::ctypes::c_int, set: *const crate::general::__kernel_fd_set) -> bool
```

### `FD_ZERO`

```rust
unsafe fn FD_ZERO(set: *mut crate::general::__kernel_fd_set)
```

