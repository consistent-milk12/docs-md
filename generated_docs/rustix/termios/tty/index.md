*[rustix](../../index.md) / [termios](../index.md) / [tty](index.md)*

---

# Module `tty`

Functions which operate on file descriptors which might be terminals.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`isatty`](#isatty) | fn | `isatty(fd)`—Tests whether a file descriptor refers to a terminal. |

## Functions

### `isatty`

```rust
fn isatty<Fd: AsFd>(fd: Fd) -> bool
```

*Defined in [`rustix-1.1.2/src/termios/tty.rs:22-24`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tty.rs#L22-L24)*

`isatty(fd)`—Tests whether a file descriptor refers to a terminal.

# References
 - [POSIX]
 - [Linux]



