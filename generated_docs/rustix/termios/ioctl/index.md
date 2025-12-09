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

*Defined in [`rustix-1.1.2/src/termios/ioctl.rs:32-38`](../../../../.source_1765210505/rustix-1.1.2/src/termios/ioctl.rs#L32-L38)*

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

*Defined in [`rustix-1.1.2/src/termios/ioctl.rs:60-66`](../../../../.source_1765210505/rustix-1.1.2/src/termios/ioctl.rs#L60-L66)*

`ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]





