*[rustix](../../../index.md) / [backend](../../index.md) / [io](../index.md) / [syscalls](index.md)*

---

# Module `syscalls`

linux_raw syscalls supporting `rustix::io`.

# Safety

See the `rustix::backend` module documentation for details.

## Contents

- [Functions](#functions)
  - [`read`](#read)
  - [`pread`](#pread)
  - [`readv`](#readv)
  - [`preadv`](#preadv)
  - [`preadv2`](#preadv2)
  - [`write`](#write)
  - [`pwrite`](#pwrite)
  - [`writev`](#writev)
  - [`pwritev`](#pwritev)
  - [`pwritev2`](#pwritev2)
  - [`close`](#close)
  - [`ioctl`](#ioctl)
  - [`ioctl_readonly`](#ioctl-readonly)
  - [`dup`](#dup)
  - [`dup2`](#dup2)
  - [`dup3`](#dup3)
  - [`fcntl_getfd`](#fcntl-getfd)
  - [`fcntl_setfd`](#fcntl-setfd)
  - [`fcntl_dupfd_cloexec`](#fcntl-dupfd-cloexec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`read`](#read) | fn |  |
| [`pread`](#pread) | fn |  |
| [`readv`](#readv) | fn |  |
| [`preadv`](#preadv) | fn |  |
| [`preadv2`](#preadv2) | fn |  |
| [`write`](#write) | fn |  |
| [`pwrite`](#pwrite) | fn |  |
| [`writev`](#writev) | fn |  |
| [`pwritev`](#pwritev) | fn |  |
| [`pwritev2`](#pwritev2) | fn |  |
| [`close`](#close) | fn |  |
| [`ioctl`](#ioctl) | fn |  |
| [`ioctl_readonly`](#ioctl-readonly) | fn |  |
| [`dup`](#dup) | fn |  |
| [`dup2`](#dup2) | fn |  |
| [`dup3`](#dup3) | fn |  |
| [`fcntl_getfd`](#fcntl-getfd) | fn |  |
| [`fcntl_setfd`](#fcntl-setfd) | fn |  |
| [`fcntl_dupfd_cloexec`](#fcntl-dupfd-cloexec) | fn |  |

## Functions

### `read`

```rust
unsafe fn read(fd: crate::fd::BorrowedFd<'_>, buf: (*mut u8, usize)) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:35-37`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L35-L37)*

### `pread`

```rust
unsafe fn pread(fd: crate::fd::BorrowedFd<'_>, buf: (*mut u8, usize), pos: u64) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:40-93`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L40-L93)*

### `readv`

```rust
fn readv(fd: crate::fd::BorrowedFd<'_>, bufs: &mut [crate::io::IoSliceMut<'_>]) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:96-100`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L96-L100)*

### `preadv`

```rust
fn preadv(fd: crate::fd::BorrowedFd<'_>, bufs: &mut [crate::io::IoSliceMut<'_>], pos: u64) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:103-122`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L103-L122)*

### `preadv2`

```rust
fn preadv2(fd: crate::fd::BorrowedFd<'_>, bufs: &mut [crate::io::IoSliceMut<'_>], pos: u64, flags: crate::io::ReadWriteFlags) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:125-146`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L125-L146)*

### `write`

```rust
fn write(fd: crate::fd::BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:149-153`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L149-L153)*

### `pwrite`

```rust
fn pwrite(fd: crate::fd::BorrowedFd<'_>, buf: &[u8], pos: u64) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:156-209`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L156-L209)*

### `writev`

```rust
fn writev(fd: crate::fd::BorrowedFd<'_>, bufs: &[crate::io::IoSlice<'_>]) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:212-216`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L212-L216)*

### `pwritev`

```rust
fn pwritev(fd: crate::fd::BorrowedFd<'_>, bufs: &[crate::io::IoSlice<'_>], pos: u64) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:219-234`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L219-L234)*

### `pwritev2`

```rust
fn pwritev2(fd: crate::fd::BorrowedFd<'_>, bufs: &[crate::io::IoSlice<'_>], pos: u64, flags: crate::io::ReadWriteFlags) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:237-258`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L237-L258)*

### `close`

```rust
unsafe fn close(fd: crate::fd::RawFd)
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:261-264`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L261-L264)*

### `ioctl`

```rust
unsafe fn ioctl(fd: crate::fd::BorrowedFd<'_>, request: crate::ioctl::Opcode, arg: *mut c::c_void) -> io::Result<crate::ioctl::IoctlOutput>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:273-279`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L273-L279)*

### `ioctl_readonly`

```rust
unsafe fn ioctl_readonly(fd: crate::fd::BorrowedFd<'_>, request: crate::ioctl::Opcode, arg: *mut c::c_void) -> io::Result<crate::ioctl::IoctlOutput>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:282-288`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L282-L288)*

### `dup`

```rust
fn dup(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::fd::OwnedFd>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:291-293`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L291-L293)*

### `dup2`

```rust
fn dup2(fd: crate::fd::BorrowedFd<'_>, new: &mut crate::fd::OwnedFd) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:297-310`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L297-L310)*

### `dup3`

```rust
fn dup3(fd: crate::fd::BorrowedFd<'_>, new: &mut crate::fd::OwnedFd, flags: crate::io::DupFlags) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:314-316`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L314-L316)*

### `fcntl_getfd`

```rust
fn fcntl_getfd(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::io::FdFlags>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:319-330`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L319-L330)*

### `fcntl_setfd`

```rust
fn fcntl_setfd(fd: crate::fd::BorrowedFd<'_>, flags: crate::io::FdFlags) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:333-342`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L333-L342)*

### `fcntl_dupfd_cloexec`

```rust
fn fcntl_dupfd_cloexec(fd: crate::fd::BorrowedFd<'_>, min: crate::fd::RawFd) -> io::Result<crate::fd::OwnedFd>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs:345-364`](../../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/syscalls.rs#L345-L364)*

