*[rustix](../index.md) / [io](index.md)*

---

# Module `io`

I/O operations.

If you're looking for [`SeekFrom`](#seekfrom), it's in the `fs` module.



## Contents

- [Modules](#modules)
  - [`close`](#close)
  - [`dup`](#dup)
  - [`errno`](#errno)
  - [`fcntl`](#fcntl)
  - [`ioctl`](#ioctl)
  - [`read_write`](#read_write)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`unnamed`](#unnamed)
  - [`close`](#close)
  - [`dup`](#dup)
  - [`dup2`](#dup2)
  - [`dup3`](#dup3)
  - [`fcntl_getfd`](#fcntl_getfd)
  - [`fcntl_setfd`](#fcntl_setfd)
  - [`fcntl_dupfd_cloexec`](#fcntl_dupfd_cloexec)
  - [`ioctl_fioclex`](#ioctl_fioclex)
  - [`ioctl_fionclex`](#ioctl_fionclex)
  - [`ioctl_fionbio`](#ioctl_fionbio)
  - [`ioctl_fionread`](#ioctl_fionread)
  - [`read`](#read)
  - [`write`](#write)
  - [`pread`](#pread)
  - [`pwrite`](#pwrite)
  - [`readv`](#readv)
  - [`writev`](#writev)
  - [`preadv`](#preadv)
  - [`pwritev`](#pwritev)
  - [`preadv2`](#preadv2)
  - [`pwritev2`](#pwritev2)
- [Type Aliases](#type-aliases)
  - [`unnamed`](#unnamed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`close`](#close) | mod | The unsafe `close` for raw file descriptors. |
| [`dup`](#dup) | mod | Functions which duplicate file descriptors. |
| [`errno`](#errno) | mod | The `Errno` type, which is a minimal wrapper around an error code. |
| [`fcntl`](#fcntl) | mod | The Unix `fcntl` function is effectively lots of different functions hidden |
| [`ioctl`](#ioctl) | mod | The Unix `ioctl` function is effectively lots of different functions hidden |
| [`read_write`](#read_write) | mod | `read` and `write`, optionally positioned, optionally vectored. |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | fn |  |
| [`close`](#close) | fn | `close(raw_fd)`—Closes a `RawFd` directly. |
| [`dup`](#dup) | fn | `dup(fd)`—Creates a new `OwnedFd` instance that shares the same |
| [`dup2`](#dup2) | fn | `dup2(fd, new)`—Changes the [file description] of a file descriptor. |
| [`dup3`](#dup3) | fn | `dup3(fd, new, flags)`—Changes the [file description] of a file |
| [`fcntl_getfd`](#fcntl_getfd) | fn | `fcntl(fd, F_GETFD)`—Returns a file descriptor's flags. |
| [`fcntl_setfd`](#fcntl_setfd) | fn | `fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags. |
| [`fcntl_dupfd_cloexec`](#fcntl_dupfd_cloexec) | fn | `fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value |
| [`ioctl_fioclex`](#ioctl_fioclex) | fn | `ioctl(fd, FIOCLEX, NULL)`—Set the close-on-exec flag. |
| [`ioctl_fionclex`](#ioctl_fionclex) | fn | `ioctl(fd, FIONCLEX, NULL)`—Remove the close-on-exec flag. |
| [`ioctl_fionbio`](#ioctl_fionbio) | fn | `ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode. |
| [`ioctl_fionread`](#ioctl_fionread) | fn | `ioctl(fd, FIONREAD)`—Returns the number of bytes ready to be read. |
| [`read`](#read) | fn | `read(fd, buf)`—Reads from a stream. |
| [`write`](#write) | fn | `write(fd, buf)`—Writes to a stream. |
| [`pread`](#pread) | fn | `pread(fd, buf, offset)`—Reads from a file at a given position. |
| [`pwrite`](#pwrite) | fn | `pwrite(fd, bufs)`—Writes to a file at a given position. |
| [`readv`](#readv) | fn | `readv(fd, bufs)`—Reads from a stream into multiple buffers. |
| [`writev`](#writev) | fn | `writev(fd, bufs)`—Writes to a stream from multiple buffers. |
| [`preadv`](#preadv) | fn | `preadv(fd, bufs, offset)`—Reads from a file at a given position into |
| [`pwritev`](#pwritev) | fn | `pwritev(fd, bufs, offset)`—Writes to a file at a given position from |
| [`preadv2`](#preadv2) | fn | `preadv2(fd, bufs, offset, flags)`—Reads data, with several options. |
| [`pwritev2`](#pwritev2) | fn | `pwritev2(fd, bufs, offset, flags)`—Writes data, with several options. |
| [`unnamed`](#unnamed) | type |  |

## Modules

- [`close`](close/index.md) - The unsafe `close` for raw file descriptors.
- [`dup`](dup/index.md) - Functions which duplicate file descriptors.
- [`errno`](errno/index.md) - The `Errno` type, which is a minimal wrapper around an error code.
- [`fcntl`](fcntl/index.md) - The Unix `fcntl` function is effectively lots of different functions hidden
- [`ioctl`](ioctl/index.md) - The Unix `ioctl` function is effectively lots of different functions hidden
- [`read_write`](read_write/index.md) - `read` and `write`, optionally positioned, optionally vectored.

## Structs

### `Errno`

```rust
struct Errno(u16);
```

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../addr2line/index.md),
but only holds an OS error code, and no extra error value.

# References
 - [POSIX]
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










#### Implementations

- <span id="errno-kind"></span>`fn kind(self) -> std::io::ErrorKind`

#### Trait Implementations

##### `impl Clone for Errno`

- <span id="errno-clone"></span>`fn clone(&self) -> Errno` — [`Errno`](#errno)

##### `impl Copy for Errno`

##### `impl Debug for Errno`

- <span id="errno-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Errno`

- <span id="errno-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Errno`

##### `impl Error for Errno`

##### `impl Hash for Errno`

- <span id="errno-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Errno`

- <span id="errno-eq"></span>`fn eq(&self, other: &Errno) -> bool` — [`Errno`](#errno)

##### `impl StructuralPartialEq for Errno`

##### `impl<T> ToString for Errno`

- <span id="errno-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `close`

```rust
unsafe fn close(raw_fd: backend::fd::RawFd)
```

`close(raw_fd)`—Closes a `RawFd` directly.

Most users won't need to use this, as [`OwnedFd`](../fd/index.md) automatically closes its
file descriptor on `Drop`.

This function does not return a `Result`, as it is the [responsibility] of
filesystem designers to not return errors from `close`. Users who chose to
use NFS or similar filesystems should take care to monitor for problems
externally.

# References
 - [Beej's Guide to Network Programming]
 - [POSIX]
 - [Linux]
 - [Apple]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)












# Safety

This function takes a `RawFd`, which must be valid before the call, and is
not valid after the call.

### `dup`

```rust
fn dup<Fd: AsFd>(fd: Fd) -> io::Result<crate::fd::OwnedFd>
```

`dup(fd)`—Creates a new `OwnedFd` instance that shares the same
underlying [file description] as `fd`.

This function does not set the `O_CLOEXEC` flag. To do a `dup` that does
set `O_CLOEXEC`, use [`fcntl_dupfd_cloexec`](#fcntl-dupfd-cloexec).

POSIX guarantees that `dup` will use the lowest unused file descriptor,
however it is not safe in general to rely on this, as file descriptors may
be unexpectedly allocated on other threads or in libraries.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)












### `dup2`

```rust
fn dup2<Fd: AsFd>(fd: Fd, new: &mut crate::fd::OwnedFd) -> io::Result<()>
```

`dup2(fd, new)`—Changes the [file description] of a file descriptor.

`dup2` conceptually closes `new` and then sets the file description for
`new` to be the same as the one for `fd`. This is a very unusual operation,
and should only be used on file descriptors where you know how `new` will
be subsequently used.

This function does not set the `O_CLOEXEC` flag. To do a `dup2` that does
set `O_CLOEXEC`, use [`dup3`](#dup3) with `DupFlags::CLOEXEC` on platforms which
support it, or [`fcntl_dupfd_cloexec`](#fcntl-dupfd-cloexec).

For `dup2` to stdin, stdout, and stderr, see `stdio::dup2_stdin`,
`stdio::dup2_stdout`, and `stdio::dup2_stderr`.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)















### `dup3`

```rust
fn dup3<Fd: AsFd>(fd: Fd, new: &mut crate::fd::OwnedFd, flags: DupFlags) -> io::Result<()>
```

`dup3(fd, new, flags)`—Changes the [file description] of a file
descriptor, with flags.

`dup3` is the same as [`dup2`](#dup2) but adds an additional flags operand, and it
fails in the case that `fd` and `new` have the same file descriptor value.
This additional difference is the reason this function isn't named
`dup2_with`.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]







### `fcntl_getfd`

```rust
fn fcntl_getfd<Fd: AsFd>(fd: Fd) -> io::Result<FdFlags>
```

`fcntl(fd, F_GETFD)`—Returns a file descriptor's flags.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `fcntl_setfd`

```rust
fn fcntl_setfd<Fd: AsFd>(fd: Fd, flags: FdFlags) -> io::Result<()>
```

`fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `fcntl_dupfd_cloexec`

```rust
fn fcntl_dupfd_cloexec<Fd: AsFd>(fd: Fd, min: backend::fd::RawFd) -> io::Result<backend::fd::OwnedFd>
```

`fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value
at least `min`, that has `O_CLOEXEC` set and that shares the same
underlying [file description] as `fd`.

POSIX guarantees that `F_DUPFD_CLOEXEC` will use the lowest unused file
descriptor which is at least `min`, however it is not safe in general to
rely on this, as file descriptors may be unexpectedly allocated on other
threads or in libraries.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)











### `ioctl_fioclex`

```rust
fn ioctl_fioclex<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`ioctl(fd, FIOCLEX, NULL)`—Set the close-on-exec flag.

This is similar to `fcntl(fd, F_SETFD, FD_CLOEXEC)`, except that it avoids
clearing any other flags that might be set.

Linux: Note that `ioctl` can not be used on `OFlags::PATH` file
descriptors.

### `ioctl_fionclex`

```rust
fn ioctl_fionclex<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`ioctl(fd, FIONCLEX, NULL)`—Remove the close-on-exec flag.

This is similar to `fcntl_setfd(fd, FdFlags::empty())`, except that it avoids
clearing any other flags that might be set.

Linux: Note that `ioctl` can not be used on `OFlags::PATH` file
descriptors.

### `ioctl_fionbio`

```rust
fn ioctl_fionbio<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()>
```

`ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode.

# References
 - [Winsock]
 - [NetBSD]
 - [OpenBSD]




### `ioctl_fionread`

```rust
fn ioctl_fionread<Fd: AsFd>(fd: Fd) -> io::Result<u64>
```

`ioctl(fd, FIONREAD)`—Returns the number of bytes ready to be read.

The result of this function gets silently coerced into a C `int` by the OS,
so it may contain a wrapped value.

# References
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]






### `read`

```rust
fn read<Fd: AsFd, Buf: Buffer<u8>>(fd: Fd, buf: Buf) -> io::Result<<Buf as >::Output>
```

`read(fd, buf)`—Reads from a stream.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `write`

```rust
fn write<Fd: AsFd>(fd: Fd, buf: &[u8]) -> io::Result<usize>
```

`write(fd, buf)`—Writes to a stream.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `pread`

```rust
fn pread<Fd: AsFd, Buf: Buffer<u8>>(fd: Fd, buf: Buf, offset: u64) -> io::Result<<Buf as >::Output>
```

`pread(fd, buf, offset)`—Reads from a file at a given position.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `pwrite`

```rust
fn pwrite<Fd: AsFd>(fd: Fd, buf: &[u8], offset: u64) -> io::Result<usize>
```

`pwrite(fd, bufs)`—Writes to a file at a given position.

Contrary to POSIX, on many popular platforms including Linux and FreeBSD,
if the file is opened in append mode, this ignores the offset appends the
data to the end of the file.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `readv`

```rust
fn readv<Fd: AsFd>(fd: Fd, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>
```

`readv(fd, bufs)`—Reads from a stream into multiple buffers.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `writev`

```rust
fn writev<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>]) -> io::Result<usize>
```

`writev(fd, bufs)`—Writes to a stream from multiple buffers.

# References
 - [POSIX]
 - [Linux]
 - [Apple]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)










### `preadv`

```rust
fn preadv<Fd: AsFd>(fd: Fd, bufs: &mut [IoSliceMut<'_>], offset: u64) -> io::Result<usize>
```

`preadv(fd, bufs, offset)`—Reads from a file at a given position into
multiple buffers.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)








### `pwritev`

```rust
fn pwritev<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>], offset: u64) -> io::Result<usize>
```

`pwritev(fd, bufs, offset)`—Writes to a file at a given position from
multiple buffers.

Contrary to POSIX, on many popular platforms including Linux and FreeBSD,
if the file is opened in append mode, this ignores the offset appends the
data to the end of the file.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../libc/new/glibc/index.md)








### `preadv2`

```rust
fn preadv2<Fd: AsFd>(fd: Fd, bufs: &mut [IoSliceMut<'_>], offset: u64, flags: ReadWriteFlags) -> io::Result<usize>
```

`preadv2(fd, bufs, offset, flags)`—Reads data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../libc/new/glibc/index.md)



### `pwritev2`

```rust
fn pwritev2<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>], offset: u64, flags: ReadWriteFlags) -> io::Result<usize>
```

`pwritev2(fd, bufs, offset, flags)`—Writes data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../libc/new/glibc/index.md)



## Type Aliases

