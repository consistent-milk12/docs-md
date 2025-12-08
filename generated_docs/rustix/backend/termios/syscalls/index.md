*[rustix](../../../index.md) / [backend](../../index.md) / [termios](../index.md) / [syscalls](index.md)*

---

# Module `syscalls`

linux_raw syscalls supporting `rustix::termios`.

# Safety

See the `rustix::backend` module documentation for details.

## Functions

### `tcgetwinsize`

```rust
fn tcgetwinsize(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::termios::Winsize>
```

### `tcgetattr`

```rust
fn tcgetattr(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::termios::Termios>
```

### `tcgetattr_fallback`

```rust
fn tcgetattr_fallback(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::termios::Termios>
```

Implement `tcgetattr` using the old `TCGETS` ioctl.

### `tcgetpgrp`

```rust
fn tcgetpgrp(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::pid::Pid>
```

### `tcsetattr`

```rust
fn tcsetattr(fd: crate::fd::BorrowedFd<'_>, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

### `tcsetattr_fallback`

```rust
fn tcsetattr_fallback(fd: crate::fd::BorrowedFd<'_>, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

Implement `tcsetattr` using the old `TCSETS` ioctl.

### `tcsendbreak`

```rust
fn tcsendbreak(fd: crate::fd::BorrowedFd<'_>) -> io::Result<()>
```

### `tcdrain`

```rust
fn tcdrain(fd: crate::fd::BorrowedFd<'_>) -> io::Result<()>
```

### `tcflush`

```rust
fn tcflush(fd: crate::fd::BorrowedFd<'_>, queue_selector: crate::termios::QueueSelector) -> io::Result<()>
```

### `tcflow`

```rust
fn tcflow(fd: crate::fd::BorrowedFd<'_>, action: crate::termios::Action) -> io::Result<()>
```

### `tcgetsid`

```rust
fn tcgetsid(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::pid::Pid>
```

### `tcsetwinsize`

```rust
fn tcsetwinsize(fd: crate::fd::BorrowedFd<'_>, winsize: crate::termios::Winsize) -> io::Result<()>
```

### `tcsetpgrp`

```rust
fn tcsetpgrp(fd: crate::fd::BorrowedFd<'_>, pid: crate::pid::Pid) -> io::Result<()>
```

### `set_speed`

```rust
fn set_speed(termios: &mut crate::termios::Termios, arbitrary_speed: u32) -> io::Result<()>
```

A wrapper around a conceptual `cfsetspeed` which handles an arbitrary
integer speed value.

### `set_output_speed`

```rust
fn set_output_speed(termios: &mut crate::termios::Termios, arbitrary_speed: u32) -> io::Result<()>
```

A wrapper around a conceptual `cfsetospeed` which handles an arbitrary
integer speed value.

### `set_input_speed`

```rust
fn set_input_speed(termios: &mut crate::termios::Termios, arbitrary_speed: u32) -> io::Result<()>
```

A wrapper around a conceptual `cfsetispeed` which handles an arbitrary
integer speed value.

### `cfmakeraw`

```rust
fn cfmakeraw(termios: &mut crate::termios::Termios)
```

### `isatty`

```rust
fn isatty(fd: crate::fd::BorrowedFd<'_>) -> bool
```

