*[rustix](../../index.md) / [termios](../index.md) / [ioctl](index.md)*

---

# Module `ioctl`

Terminal-related `ioctl` functions.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ioctl_tiocexcl`](#ioctl_tiocexcl) | fn | `ioctl(fd, TIOCEXCL)`—Enables exclusive mode on a terminal. |
| [`ioctl_tiocnxcl`](#ioctl_tiocnxcl) | fn | `ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal. |

## Functions

### `ioctl_tiocexcl`

```rust
fn ioctl_tiocexcl<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`ioctl(fd, TIOCEXCL)`—Enables exclusive mode on a terminal.

In exclusive mode, subsequent unprivileged `open` calls on the terminal
device fail with `io::Errno::BUSY`.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]





### `ioctl_tiocnxcl`

```rust
fn ioctl_tiocnxcl<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]





