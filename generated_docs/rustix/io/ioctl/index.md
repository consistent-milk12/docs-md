*[rustix](../../index.md) / [io](../index.md) / [ioctl](index.md)*

---

# Module `ioctl`

The Unix `ioctl` function is effectively lots of different functions hidden
behind a single dynamic dispatch interface. In order to provide a type-safe
API, rustix makes them all separate functions so that they can have
dedicated static type signatures.

Some ioctls, such as those related to filesystems, terminals, and
processes, live in other top-level API modules.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ioctl_fioclex`](#ioctl_fioclex) | fn | `ioctl(fd, FIOCLEX, NULL)`—Set the close-on-exec flag. |
| [`ioctl_fionclex`](#ioctl_fionclex) | fn | `ioctl(fd, FIONCLEX, NULL)`—Remove the close-on-exec flag. |
| [`ioctl_fionbio`](#ioctl_fionbio) | fn | `ioctl(fd, FIONBIO, &value)`—Enables or disables non-blocking mode. |
| [`ioctl_fionread`](#ioctl_fionread) | fn | `ioctl(fd, FIONREAD)`—Returns the number of bytes ready to be read. |

## Functions

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






