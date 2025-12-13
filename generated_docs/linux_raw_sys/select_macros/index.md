*[linux_raw_sys](../index.md) / [select_macros](index.md)*

---

# Module `select_macros`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FD_CLR`](#fd-clr) | fn |  |
| [`FD_SET`](#fd-set) | fn |  |
| [`FD_ISSET`](#fd-isset) | fn |  |
| [`FD_ZERO`](#fd-zero) | fn |  |

## Functions

### `FD_CLR`

```rust
unsafe fn FD_CLR(fd: crate::ctypes::c_int, set: *mut crate::general::__kernel_fd_set)
```

*Defined in [`linux-raw-sys-0.11.0/src/lib.rs:169-174`](../../../.source_1765633015/linux-raw-sys-0.11.0/src/lib.rs#L169-L174)*

### `FD_SET`

```rust
unsafe fn FD_SET(fd: crate::ctypes::c_int, set: *mut crate::general::__kernel_fd_set)
```

*Defined in [`linux-raw-sys-0.11.0/src/lib.rs:176-181`](../../../.source_1765633015/linux-raw-sys-0.11.0/src/lib.rs#L176-L181)*

### `FD_ISSET`

```rust
unsafe fn FD_ISSET(fd: crate::ctypes::c_int, set: *const crate::general::__kernel_fd_set) -> bool
```

*Defined in [`linux-raw-sys-0.11.0/src/lib.rs:183-190`](../../../.source_1765633015/linux-raw-sys-0.11.0/src/lib.rs#L183-L190)*

### `FD_ZERO`

```rust
unsafe fn FD_ZERO(set: *mut crate::general::__kernel_fd_set)
```

*Defined in [`linux-raw-sys-0.11.0/src/lib.rs:192-195`](../../../.source_1765633015/linux-raw-sys-0.11.0/src/lib.rs#L192-L195)*

