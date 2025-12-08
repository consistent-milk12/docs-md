*[rustix](../../index.md) / [termios](../index.md) / [tty](index.md)*

---

# Module `tty`

Functions which operate on file descriptors which might be terminals.

## Functions

### `isatty`

```rust
fn isatty<Fd: AsFd>(fd: Fd) -> bool
```

`isatty(fd)`—Tests whether a file descriptor refers to a terminal.

# References
 - [POSIX]
 - [Linux]



