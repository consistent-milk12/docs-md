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
  - [`read_write`](#read-write)
- [Structs](#structs)
  - [`Errno`](#errno)
- [Functions](#functions)
  - [`retry_on_intr`](#retry-on-intr)
  - [`close`](#close)
  - [`dup`](#dup)
  - [`dup2`](#dup2)
  - [`dup3`](#dup3)
  - [`fcntl_getfd`](#fcntl-getfd)
  - [`fcntl_setfd`](#fcntl-setfd)
  - [`fcntl_dupfd_cloexec`](#fcntl-dupfd-cloexec)
  - [`ioctl_fioclex`](#ioctl-fioclex)
  - [`ioctl_fionclex`](#ioctl-fionclex)
  - [`ioctl_fionbio`](#ioctl-fionbio)
  - [`ioctl_fionread`](#ioctl-fionread)
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
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`close`](#close) | mod | The unsafe `close` for raw file descriptors. |
| [`dup`](#dup) | mod | Functions which duplicate file descriptors. |
| [`errno`](#errno) | mod | The `Errno` type, which is a minimal wrapper around an error code. |
| [`fcntl`](#fcntl) | mod | The Unix `fcntl` function is effectively lots of different functions hidden behind a single dynamic dispatch interface. |
| [`ioctl`](#ioctl) | mod | The Unix `ioctl` function is effectively lots of different functions hidden behind a single dynamic dispatch interface. |
| [`read_write`](#read-write) | mod | `read` and `write`, optionally positioned, optionally vectored. |
| [`Errno`](#errno) | struct |  |
| [`retry_on_intr`](#retry-on-intr) | fn |  |
| [`close`](#close) | fn | `close(raw_fd)`—Closes a `RawFd` directly. |
| [`dup`](#dup) | fn | `dup(fd)`—Creates a new `OwnedFd` instance that shares the same underlying [file description] as `fd`. |
| [`dup2`](#dup2) | fn | `dup2(fd, new)`—Changes the [file description] of a file descriptor. |
| [`dup3`](#dup3) | fn | `dup3(fd, new, flags)`—Changes the [file description] of a file descriptor, with flags. |
| [`fcntl_getfd`](#fcntl-getfd) | fn | `fcntl(fd, F_GETFD)`—Returns a file descriptor's flags. |
| [`fcntl_setfd`](#fcntl-setfd) | fn | `fcntl(fd, F_SETFD, flags)`—Sets a file descriptor's flags. |
| [`fcntl_dupfd_cloexec`](#fcntl-dupfd-cloexec) | fn | `fcntl(fd, F_DUPFD_CLOEXEC)`—Creates a new `OwnedFd` instance, with value at least `min`, that has `O_CLOEXEC` set and that shares the same underlying [file description] as `fd`. |
| [`ioctl_fioclex`](#ioctl-fioclex) | fn | `ioctl(fd, FIOCLEX, NULL)`—Set the close-on-exec flag. |
| [`ioctl_fionclex`](#ioctl-fionclex) | fn | `ioctl(fd, FIONCLEX, NULL)`—Remove the close-on-exec flag. |
| [`ioctl_fionbio`](#ioctl-fionbio) | fn | `ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode. |
| [`ioctl_fionread`](#ioctl-fionread) | fn | `ioctl(fd, FIONREAD)`—Returns the number of bytes ready to be read. |
| [`read`](#read) | fn | `read(fd, buf)`—Reads from a stream. |
| [`write`](#write) | fn | `write(fd, buf)`—Writes to a stream. |
| [`pread`](#pread) | fn | `pread(fd, buf, offset)`—Reads from a file at a given position. |
| [`pwrite`](#pwrite) | fn | `pwrite(fd, bufs)`—Writes to a file at a given position. |
| [`readv`](#readv) | fn | `readv(fd, bufs)`—Reads from a stream into multiple buffers. |
| [`writev`](#writev) | fn | `writev(fd, bufs)`—Writes to a stream from multiple buffers. |
| [`preadv`](#preadv) | fn | `preadv(fd, bufs, offset)`—Reads from a file at a given position into multiple buffers. |
| [`pwritev`](#pwritev) | fn | `pwritev(fd, bufs, offset)`—Writes to a file at a given position from multiple buffers. |
| [`preadv2`](#preadv2) | fn | `preadv2(fd, bufs, offset, flags)`—Reads data, with several options. |
| [`pwritev2`](#pwritev2) | fn | `pwritev2(fd, bufs, offset, flags)`—Writes data, with several options. |
| [`Result`](#result) | type |  |

## Modules

- [`close`](close/index.md) — The unsafe `close` for raw file descriptors.
- [`dup`](dup/index.md) — Functions which duplicate file descriptors.
- [`errno`](errno/index.md) — The `Errno` type, which is a minimal wrapper around an error code.
- [`fcntl`](fcntl/index.md) — The Unix `fcntl` function is effectively lots of different functions hidden
- [`ioctl`](ioctl/index.md) — The Unix `ioctl` function is effectively lots of different functions hidden
- [`read_write`](read_write/index.md) — `read` and `write`, optionally positioned, optionally vectored.

## Structs

### `Errno`

```rust
struct Errno(u16);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:51`](../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L51)*

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

- <span id="errno-from-io-error"></span>`fn from_io_error(io_err: &std::io::Error) -> Option<Self>`

- <span id="errno-raw-os-error"></span>`const fn raw_os_error(self) -> i32`

- <span id="errno-from-raw-os-error"></span>`const fn from_raw_os_error(raw: i32) -> Self`

- <span id="errno-from-errno"></span>`const fn from_errno(raw: u32) -> Self`

#### Trait Implementations

##### `impl Clone for Errno`

- <span id="errno-clone"></span>`fn clone(&self) -> Errno` — [`Errno`](../backend/io/errno/index.md#errno)

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

- <span id="errno-eq"></span>`fn eq(&self, other: &Errno) -> bool` — [`Errno`](../backend/io/errno/index.md#errno)

##### `impl StructuralPartialEq for Errno`

##### `impl ToString for Errno`

- <span id="errno-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `retry_on_intr`

```rust
fn retry_on_intr<T, F: FnMut() -> Result<T>>(f: F) -> Result<T>
```

*Defined in [`rustix-1.1.2/src/io/errno.rs:67-74`](../../../.source_1765521767/rustix-1.1.2/src/io/errno.rs#L67-L74)*

Call `f` until it either succeeds or fails other than `Errno::INTR`.

### `close`

```rust
unsafe fn close(raw_fd: backend::fd::RawFd)
```

*Defined in [`rustix-1.1.2/src/io/close.rs:54-56`](../../../.source_1765521767/rustix-1.1.2/src/io/close.rs#L54-L56)*

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

*Defined in [`rustix-1.1.2/src/io/dup.rs:44-46`](../../../.source_1765521767/rustix-1.1.2/src/io/dup.rs#L44-L46)*

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

*Defined in [`rustix-1.1.2/src/io/dup.rs:89-91`](../../../.source_1765521767/rustix-1.1.2/src/io/dup.rs#L89-L91)*

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

*Defined in [`rustix-1.1.2/src/io/dup.rs:123-125`](../../../.source_1765521767/rustix-1.1.2/src/io/dup.rs#L123-L125)*

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

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:40-42`](../../../.source_1765521767/rustix-1.1.2/src/io/fcntl.rs#L40-L42)*

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

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:68-70`](../../../.source_1765521767/rustix-1.1.2/src/io/fcntl.rs#L68-L70)*

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

*Defined in [`rustix-1.1.2/src/io/fcntl.rs:105-107`](../../../.source_1765521767/rustix-1.1.2/src/io/fcntl.rs#L105-L107)*

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

*Defined in [`rustix-1.1.2/src/io/ioctl.rs:26-32`](../../../.source_1765521767/rustix-1.1.2/src/io/ioctl.rs#L26-L32)*

`ioctl(fd, FIOCLEX, NULL)`—Set the close-on-exec flag.

This is similar to `fcntl(fd, F_SETFD, FD_CLOEXEC)`, except that it avoids
clearing any other flags that might be set.

Linux: Note that `ioctl` can not be used on `OFlags::PATH` file
descriptors.

### `ioctl_fionclex`

```rust
fn ioctl_fionclex<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/io/ioctl.rs:44-50`](../../../.source_1765521767/rustix-1.1.2/src/io/ioctl.rs#L44-L50)*

`ioctl(fd, FIONCLEX, NULL)`—Remove the close-on-exec flag.

This is similar to `fcntl_setfd(fd, FdFlags::empty())`, except that it avoids
clearing any other flags that might be set.

Linux: Note that `ioctl` can not be used on `OFlags::PATH` file
descriptors.

### `ioctl_fionbio`

```rust
fn ioctl_fionbio<Fd: AsFd>(fd: Fd, value: bool) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/io/ioctl.rs:64-70`](../../../.source_1765521767/rustix-1.1.2/src/io/ioctl.rs#L64-L70)*

`ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode.

# References
 - [Winsock]
 - [NetBSD]
 - [OpenBSD]




### `ioctl_fionread`

```rust
fn ioctl_fionread<Fd: AsFd>(fd: Fd) -> io::Result<u64>
```

*Defined in [`rustix-1.1.2/src/io/ioctl.rs:92-98`](../../../.source_1765521767/rustix-1.1.2/src/io/ioctl.rs#L92-L98)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:39-44`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L39-L44)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:69-71`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L69-L71)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:97-106`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L97-L106)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:136-138`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L136-L138)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:164-166`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L164-L166)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:192-194`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L192-L194)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:227-229`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L227-L229)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:266-268`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L266-L268)*

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

*Defined in [`rustix-1.1.2/src/io/read_write.rs:282-289`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L282-L289)*

`preadv2(fd, bufs, offset, flags)`—Reads data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../libc/new/glibc/index.md)



### `pwritev2`

```rust
fn pwritev2<Fd: AsFd>(fd: Fd, bufs: &[IoSlice<'_>], offset: u64, flags: ReadWriteFlags) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/io/read_write.rs:303-310`](../../../.source_1765521767/rustix-1.1.2/src/io/read_write.rs#L303-L310)*

`pwritev2(fd, bufs, offset, flags)`—Writes data, with several options.

An `offset` of `u64::MAX` means to use and update the current file offset.

# References
 - [Linux]
 - [`glibc`](../../libc/new/glibc/index.md)



## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Errno>;
```

*Defined in [`rustix-1.1.2/src/io/errno.rs:15`](../../../.source_1765521767/rustix-1.1.2/src/io/errno.rs#L15)*

A specialized [`Result`](errno/index.md) type for `rustix` APIs.

