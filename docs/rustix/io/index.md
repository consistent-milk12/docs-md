*[rustix](../index.md) / [io](index.md)*

---

# Module `io`

I/O operations.

If you're looking for [`SeekFrom`](#seekfrom), it's in the `fs` module.



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

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../docs_md/error/index.md),
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

- `const ACCESS: Self`

- `const ADDRINUSE: Self`

- `const ADDRNOTAVAIL: Self`

- `const ADV: Self`

- `const AFNOSUPPORT: Self`

- `const AGAIN: Self`

- `const ALREADY: Self`

- `const BADE: Self`

- `const BADF: Self`

- `const BADFD: Self`

- `const BADMSG: Self`

- `const BADR: Self`

- `const BADRQC: Self`

- `const BADSLT: Self`

- `const BFONT: Self`

- `const BUSY: Self`

- `const CANCELED: Self`

- `const CHILD: Self`

- `const CHRNG: Self`

- `const COMM: Self`

- `const CONNABORTED: Self`

- `const CONNREFUSED: Self`

- `const CONNRESET: Self`

- `const DEADLK: Self`

- `const DEADLOCK: Self`

- `const DESTADDRREQ: Self`

- `const DOM: Self`

- `const DOTDOT: Self`

- `const DQUOT: Self`

- `const EXIST: Self`

- `const FAULT: Self`

- `const FBIG: Self`

- `const HOSTDOWN: Self`

- `const HOSTUNREACH: Self`

- `const HWPOISON: Self`

- `const IDRM: Self`

- `const ILSEQ: Self`

- `const INPROGRESS: Self`

- `const INTR: Self`

- `const INVAL: Self`

- `const IO: Self`

- `const ISCONN: Self`

- `const ISDIR: Self`

- `const ISNAM: Self`

- `const KEYEXPIRED: Self`

- `const KEYREJECTED: Self`

- `const KEYREVOKED: Self`

- `const L2HLT: Self`

- `const L2NSYNC: Self`

- `const L3HLT: Self`

- `const L3RST: Self`

- `const LIBACC: Self`

- `const LIBBAD: Self`

- `const LIBEXEC: Self`

- `const LIBMAX: Self`

- `const LIBSCN: Self`

- `const LNRNG: Self`

- `const LOOP: Self`

- `const MEDIUMTYPE: Self`

- `const MFILE: Self`

- `const MLINK: Self`

- `const MSGSIZE: Self`

- `const MULTIHOP: Self`

- `const NAMETOOLONG: Self`

- `const NAVAIL: Self`

- `const NETDOWN: Self`

- `const NETRESET: Self`

- `const NETUNREACH: Self`

- `const NFILE: Self`

- `const NOANO: Self`

- `const NOBUFS: Self`

- `const NOCSI: Self`

- `const NODATA: Self`

- `const NODEV: Self`

- `const NOENT: Self`

- `const NOEXEC: Self`

- `const NOKEY: Self`

- `const NOLCK: Self`

- `const NOLINK: Self`

- `const NOMEDIUM: Self`

- `const NOMEM: Self`

- `const NOMSG: Self`

- `const NONET: Self`

- `const NOPKG: Self`

- `const NOPROTOOPT: Self`

- `const NOSPC: Self`

- `const NOSR: Self`

- `const NOSTR: Self`

- `const NOSYS: Self`

- `const NOTBLK: Self`

- `const NOTCONN: Self`

- `const NOTDIR: Self`

- `const NOTEMPTY: Self`

- `const NOTNAM: Self`

- `const NOTRECOVERABLE: Self`

- `const NOTSOCK: Self`

- `const NOTSUP: Self`

- `const NOTTY: Self`

- `const NOTUNIQ: Self`

- `const NXIO: Self`

- `const OPNOTSUPP: Self`

- `const OVERFLOW: Self`

- `const OWNERDEAD: Self`

- `const PERM: Self`

- `const PFNOSUPPORT: Self`

- `const PIPE: Self`

- `const PROTO: Self`

- `const PROTONOSUPPORT: Self`

- `const PROTOTYPE: Self`

- `const RANGE: Self`

- `const REMCHG: Self`

- `const REMOTE: Self`

- `const REMOTEIO: Self`

- `const RESTART: Self`

- `const RFKILL: Self`

- `const ROFS: Self`

- `const SHUTDOWN: Self`

- `const SOCKTNOSUPPORT: Self`

- `const SPIPE: Self`

- `const SRCH: Self`

- `const SRMNT: Self`

- `const STALE: Self`

- `const STRPIPE: Self`

- `const TIME: Self`

- `const TIMEDOUT: Self`

- `const TOOBIG: Self`

- `const TOOMANYREFS: Self`

- `const TXTBSY: Self`

- `const UCLEAN: Self`

- `const UNATCH: Self`

- `const USERS: Self`

- `const WOULDBLOCK: Self`

- `const XDEV: Self`

- `const XFULL: Self`

#### Trait Implementations

##### `impl Clone for Errno`

- `fn clone(self: &Self) -> Errno` — [`Errno`](#errno)

##### `impl Copy for Errno`

##### `impl Debug for Errno`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Errno`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Errno`

##### `impl Error for Errno`

##### `impl Hash for Errno`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Errno`

- `fn eq(self: &Self, other: &Errno) -> bool` — [`Errno`](#errno)

##### `impl StructuralPartialEq for Errno`

##### `impl<T> ToString for Errno`

- `fn to_string(self: &Self) -> String`

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
set `O_CLOEXEC`, use [`fcntl_dupfd_cloexec`](../backend/io/syscalls/index.md).

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
support it, or [`fcntl_dupfd_cloexec`](../backend/io/syscalls/index.md).

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

