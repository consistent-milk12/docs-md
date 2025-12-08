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
  - [`ioctl_readonly`](#ioctl_readonly)
  - [`dup`](#dup)
  - [`dup2`](#dup2)
  - [`dup3`](#dup3)
  - [`fcntl_getfd`](#fcntl_getfd)
  - [`fcntl_setfd`](#fcntl_setfd)
  - [`fcntl_dupfd_cloexec`](#fcntl_dupfd_cloexec)

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
| [`ioctl_readonly`](#ioctl_readonly) | fn |  |
| [`dup`](#dup) | fn |  |
| [`dup2`](#dup2) | fn |  |
| [`dup3`](#dup3) | fn |  |
| [`fcntl_getfd`](#fcntl_getfd) | fn |  |
| [`fcntl_setfd`](#fcntl_setfd) | fn |  |
| [`fcntl_dupfd_cloexec`](#fcntl_dupfd_cloexec) | fn |  |

## Functions

### `read`

```rust
unsafe fn read(fd: crate::fd::BorrowedFd<'_>, buf: (*mut u8, usize)) -> io::Result<usize>
```

### `pread`

```rust
unsafe fn pread(fd: crate::fd::BorrowedFd<'_>, buf: (*mut u8, usize), pos: u64) -> io::Result<usize>
```

### `readv`

```rust
fn readv(fd: crate::fd::BorrowedFd<'_>, bufs: &mut [crate::io::IoSliceMut<'_>]) -> io::Result<usize>
```

### `preadv`

```rust
fn preadv(fd: crate::fd::BorrowedFd<'_>, bufs: &mut [crate::io::IoSliceMut<'_>], pos: u64) -> io::Result<usize>
```

### `preadv2`

```rust
fn preadv2(fd: crate::fd::BorrowedFd<'_>, bufs: &mut [crate::io::IoSliceMut<'_>], pos: u64, flags: crate::io::ReadWriteFlags) -> io::Result<usize>
```

### `write`

```rust
fn write(fd: crate::fd::BorrowedFd<'_>, buf: &[u8]) -> io::Result<usize>
```

### `pwrite`

```rust
fn pwrite(fd: crate::fd::BorrowedFd<'_>, buf: &[u8], pos: u64) -> io::Result<usize>
```

### `writev`

```rust
fn writev(fd: crate::fd::BorrowedFd<'_>, bufs: &[crate::io::IoSlice<'_>]) -> io::Result<usize>
```

### `pwritev`

```rust
fn pwritev(fd: crate::fd::BorrowedFd<'_>, bufs: &[crate::io::IoSlice<'_>], pos: u64) -> io::Result<usize>
```

### `pwritev2`

```rust
fn pwritev2(fd: crate::fd::BorrowedFd<'_>, bufs: &[crate::io::IoSlice<'_>], pos: u64, flags: crate::io::ReadWriteFlags) -> io::Result<usize>
```

### `close`

```rust
unsafe fn close(fd: crate::fd::RawFd)
```

### `ioctl`

```rust
unsafe fn ioctl(fd: crate::fd::BorrowedFd<'_>, request: crate::ioctl::Opcode, arg: *mut c::c_void) -> io::Result<crate::ioctl::IoctlOutput>
```

### `ioctl_readonly`

```rust
unsafe fn ioctl_readonly(fd: crate::fd::BorrowedFd<'_>, request: crate::ioctl::Opcode, arg: *mut c::c_void) -> io::Result<crate::ioctl::IoctlOutput>
```

### `dup`

```rust
fn dup(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::fd::OwnedFd>
```

### `dup2`

```rust
fn dup2(fd: crate::fd::BorrowedFd<'_>, new: &mut crate::fd::OwnedFd) -> io::Result<()>
```

### `dup3`

```rust
fn dup3(fd: crate::fd::BorrowedFd<'_>, new: &mut crate::fd::OwnedFd, flags: crate::io::DupFlags) -> io::Result<()>
```

### `fcntl_getfd`

```rust
fn fcntl_getfd(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::io::FdFlags>
```

### `fcntl_setfd`

```rust
fn fcntl_setfd(fd: crate::fd::BorrowedFd<'_>, flags: crate::io::FdFlags) -> io::Result<()>
```

### `fcntl_dupfd_cloexec`

```rust
fn fcntl_dupfd_cloexec(fd: crate::fd::BorrowedFd<'_>, min: crate::fd::RawFd) -> io::Result<crate::fd::OwnedFd>
```

