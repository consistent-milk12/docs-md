*[terminal_size](../index.md) / [unix](index.md)*

---

# Module `unix`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`terminal_size`](#terminal_size) | fn | Returns the size of the terminal. |
| [`terminal_size_of`](#terminal_size_of) | fn | Returns the size of the terminal using the given file descriptor, if available. |
| [`terminal_size_using_fd`](#terminal_size_using_fd) | fn | Returns the size of the terminal using the given raw file descriptor, if available. |

## Functions

### `terminal_size`

```rust
fn terminal_size() -> Option<(super::Width, super::Height)>
```

Returns the size of the terminal.

This function checks the stdout, stderr, and stdin streams (in that order).
The size of the first stream that is a TTY will be returned.  If nothing
is a TTY, then `None` is returned.

### `terminal_size_of`

```rust
fn terminal_size_of<Fd: AsFd>(fd: Fd) -> Option<(super::Width, super::Height)>
```

Returns the size of the terminal using the given file descriptor, if available.

If the given file descriptor is not a tty, returns `None`

### `terminal_size_using_fd`

```rust
unsafe fn terminal_size_using_fd(fd: std::os::unix::io::RawFd) -> Option<(super::Width, super::Height)>
```

Returns the size of the terminal using the given raw file descriptor, if available.

The given file descriptor must be an open file descriptor.

If the given file descriptor is not a tty, returns `None`

# Safety

`fd` must be a valid open file descriptor.

