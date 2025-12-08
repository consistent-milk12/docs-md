*[rustix](../../index.md) / [io](../index.md) / [close](index.md)*

---

# Module `close`

The unsafe `close` for raw file descriptors.

# Safety

Operating on raw file descriptors is unsafe.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`close`](#close) | fn | `close(raw_fd)`—Closes a `RawFd` directly. |

## Functions

### `close`

```rust
unsafe fn close(raw_fd: backend::fd::RawFd)
```

`close(raw_fd)`—Closes a `RawFd` directly.

Most users won't need to use this, as [`OwnedFd`](../../fd/index.md) automatically closes its
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
 - [`glibc`](../../../libc/new/glibc/index.md)












# Safety

This function takes a `RawFd`, which must be valid before the call, and is
not valid after the call.

