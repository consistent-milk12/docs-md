*[rustix](../index.md) / [backend](index.md)*

---

# Module `backend`

The linux_raw backend.

This makes Linux syscalls directly, without going through libc.

# Safety

These files performs raw system calls, and sometimes passes them
uninitialized memory buffers. The signatures in this module are currently
manually maintained and must correspond with the signatures of the actual
Linux syscalls.

Some of this could be auto-generated from the Linux header file
<linux/syscalls.h>, but we often need more information than it provides,
such as which pointers are array slices, out parameters, or in-out
parameters, which integers are owned or borrowed file descriptors, etc.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`arch`](#arch) | mod | Architecture-specific syscall code. |
| [`conv`](#conv) | mod | Convert values to [`ArgReg`] and from [`RetReg`]. |
| [`reg`](#reg) | mod | Encapsulation for system call arguments and return values. |
| [`io`](#io) | mod |  |
| [`termios`](#termios) | mod |  |
| [`c`](#c) | mod | Adapt the Linux API to resemble a POSIX-style libc API. |
| [`MAX_IOV`](#max_iov) | const | The maximum number of buffers that can be passed into a vectored I/O system call on the current platform. |

## Modules

- [`arch`](arch/index.md) — Architecture-specific syscall code.
- [`conv`](conv/index.md) — Convert values to [`ArgReg`] and from [`RetReg`].
- [`reg`](reg/index.md) — Encapsulation for system call arguments and return values.
- [`io`](io/index.md)
- [`termios`](termios/index.md)
- [`c`](c/index.md) — Adapt the Linux API to resemble a POSIX-style libc API.

## Constants

### `MAX_IOV`
```rust
const MAX_IOV: usize = 1_024usize;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/mod.rs:112`](../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/mod.rs#L112)*

The maximum number of buffers that can be passed into a vectored I/O system
call on the current platform.

