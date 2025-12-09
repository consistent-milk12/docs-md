*[rustix](../../../index.md) / [backend](../../index.md) / [termios](../index.md) / [syscalls](index.md)*

---

# Module `syscalls`

linux_raw syscalls supporting `rustix::termios`.

# Safety

See the `rustix::backend` module documentation for details.

## Contents

- [Functions](#functions)
  - [`tcgetwinsize`](#tcgetwinsize)
  - [`tcgetattr`](#tcgetattr)
  - [`tcgetattr_fallback`](#tcgetattr_fallback)
  - [`tcgetpgrp`](#tcgetpgrp)
  - [`tcsetattr`](#tcsetattr)
  - [`tcsetattr_fallback`](#tcsetattr_fallback)
  - [`tcsendbreak`](#tcsendbreak)
  - [`tcdrain`](#tcdrain)
  - [`tcflush`](#tcflush)
  - [`tcflow`](#tcflow)
  - [`tcgetsid`](#tcgetsid)
  - [`tcsetwinsize`](#tcsetwinsize)
  - [`tcsetpgrp`](#tcsetpgrp)
  - [`set_speed`](#set_speed)
  - [`set_output_speed`](#set_output_speed)
  - [`set_input_speed`](#set_input_speed)
  - [`cfmakeraw`](#cfmakeraw)
  - [`isatty`](#isatty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`tcgetwinsize`](#tcgetwinsize) | fn |  |
| [`tcgetattr`](#tcgetattr) | fn |  |
| [`tcgetattr_fallback`](#tcgetattr_fallback) | fn | Implement `tcgetattr` using the old `TCGETS` ioctl. |
| [`tcgetpgrp`](#tcgetpgrp) | fn |  |
| [`tcsetattr`](#tcsetattr) | fn |  |
| [`tcsetattr_fallback`](#tcsetattr_fallback) | fn | Implement `tcsetattr` using the old `TCSETS` ioctl. |
| [`tcsendbreak`](#tcsendbreak) | fn |  |
| [`tcdrain`](#tcdrain) | fn |  |
| [`tcflush`](#tcflush) | fn |  |
| [`tcflow`](#tcflow) | fn |  |
| [`tcgetsid`](#tcgetsid) | fn |  |
| [`tcsetwinsize`](#tcsetwinsize) | fn |  |
| [`tcsetpgrp`](#tcsetpgrp) | fn |  |
| [`set_speed`](#set_speed) | fn | A wrapper around a conceptual `cfsetspeed` which handles an arbitrary integer speed value. |
| [`set_output_speed`](#set_output_speed) | fn | A wrapper around a conceptual `cfsetospeed` which handles an arbitrary integer speed value. |
| [`set_input_speed`](#set_input_speed) | fn | A wrapper around a conceptual `cfsetispeed` which handles an arbitrary integer speed value. |
| [`cfmakeraw`](#cfmakeraw) | fn |  |
| [`isatty`](#isatty) | fn |  |

## Functions

### `tcgetwinsize`

```rust
fn tcgetwinsize(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::termios::Winsize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:25-31`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L25-L31)*

### `tcgetattr`

```rust
fn tcgetattr(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::termios::Termios>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:34-52`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L34-L52)*

### `tcgetattr_fallback`

```rust
fn tcgetattr_fallback(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::termios::Termios>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:57-98`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L57-L98)*

Implement `tcgetattr` using the old `TCGETS` ioctl.

### `tcgetpgrp`

```rust
fn tcgetpgrp(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::pid::Pid>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:101-116`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L101-L116)*

### `tcsetattr`

```rust
fn tcsetattr(fd: crate::fd::BorrowedFd<'_>, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:119-158`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L119-L158)*

### `tcsetattr_fallback`

```rust
fn tcsetattr_fallback(fd: crate::fd::BorrowedFd<'_>, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:163-199`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L163-L199)*

Implement `tcsetattr` using the old `TCSETS` ioctl.

### `tcsendbreak`

```rust
fn tcsendbreak(fd: crate::fd::BorrowedFd<'_>) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:202-211`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L202-L211)*

### `tcdrain`

```rust
fn tcdrain(fd: crate::fd::BorrowedFd<'_>) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:214-223`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L214-L223)*

### `tcflush`

```rust
fn tcflush(fd: crate::fd::BorrowedFd<'_>, queue_selector: crate::termios::QueueSelector) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:226-235`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L226-L235)*

### `tcflow`

```rust
fn tcflow(fd: crate::fd::BorrowedFd<'_>, action: crate::termios::Action) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:238-247`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L238-L247)*

### `tcgetsid`

```rust
fn tcgetsid(fd: crate::fd::BorrowedFd<'_>) -> io::Result<crate::pid::Pid>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:250-257`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L250-L257)*

### `tcsetwinsize`

```rust
fn tcsetwinsize(fd: crate::fd::BorrowedFd<'_>, winsize: crate::termios::Winsize) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:260-269`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L260-L269)*

### `tcsetpgrp`

```rust
fn tcsetpgrp(fd: crate::fd::BorrowedFd<'_>, pid: crate::pid::Pid) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:272-282`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L272-L282)*

### `set_speed`

```rust
fn set_speed(termios: &mut crate::termios::Termios, arbitrary_speed: u32) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:287-300`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L287-L300)*

A wrapper around a conceptual `cfsetspeed` which handles an arbitrary
integer speed value.

### `set_output_speed`

```rust
fn set_output_speed(termios: &mut crate::termios::Termios, arbitrary_speed: u32) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:305-316`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L305-L316)*

A wrapper around a conceptual `cfsetospeed` which handles an arbitrary
integer speed value.

### `set_input_speed`

```rust
fn set_input_speed(termios: &mut crate::termios::Termios, arbitrary_speed: u32) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:321-332`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L321-L332)*

A wrapper around a conceptual `cfsetispeed` which handles an arbitrary
integer speed value.

### `cfmakeraw`

```rust
fn cfmakeraw(termios: &mut crate::termios::Termios)
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:335-359`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L335-L359)*

### `isatty`

```rust
fn isatty(fd: crate::fd::BorrowedFd<'_>) -> bool
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs:362-368`](../../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/termios/syscalls.rs#L362-L368)*

