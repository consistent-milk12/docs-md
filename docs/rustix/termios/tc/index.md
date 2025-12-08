*[rustix](../../index.md) / [termios](../index.md) / [tc](index.md)*

---

# Module `tc`

## Structs

### `Pid`

```rust
struct Pid(core::num::NonZeroI32);
```

`pid_t`—A non-zero Unix process ID.

This is a pid, and not a pidfd. It is not a file descriptor, and the
process it refers to could disappear at any time and be replaced by
another, unrelated, process.

On Linux, `Pid` values are also used to identify threads.

#### Implementations

- `const INIT: Self`

- `const fn from_raw(raw: i32) -> Option<Self>`

- `const unsafe fn from_raw_unchecked(raw: i32) -> Self`

- `fn from_child(child: &std::process::Child) -> Self`

- `const fn as_raw_nonzero(self: Self) -> NonZeroI32`

- `const fn as_raw_pid(self: Self) -> i32`

- `const fn as_raw(pid: Option<Self>) -> i32`

- `const fn is_init(self: Self) -> bool`

#### Trait Implementations

##### `impl Binary for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Clone for Pid`

- `fn clone(self: &Self) -> Pid` — [`Pid`](../../pid/index.md)

##### `impl Copy for Pid`

##### `impl Debug for Pid`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Pid`

##### `impl Hash for Pid`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl LowerExp for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Pid`

- `fn eq(self: &Self, other: &Pid) -> bool` — [`Pid`](../../pid/index.md)

##### `impl StructuralPartialEq for Pid`

##### `impl<T> ToString for Pid`

- `fn to_string(self: &Self) -> String`

##### `impl UpperExp for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex for Pid`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `tcgetattr`

```rust
fn tcgetattr<Fd: AsFd>(fd: Fd) -> io::Result<crate::termios::Termios>
```

`tcgetattr(fd)`—Get terminal attributes.

Also known as the `TCGETS` (or `TCGETS2` on Linux) operation with `ioctl`.

On Linux, this uses `TCGETS2`. If that fails in a way that indicates that
the host doesn't support it, this falls back to the old `TCGETS`, manually
initializes the fields that `TCGETS` doesn't initialize, and fails with
`io::Errno::RANGE` if the input or output speeds cannot be supported.

# References
 - [POSIX `tcgetattr`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcgetwinsize`

```rust
fn tcgetwinsize<Fd: AsFd>(fd: Fd) -> io::Result<crate::termios::Winsize>
```

`tcgetwinsize(fd)`—Get the current terminal window size.

Also known as the `TIOCGWINSZ` operation with `ioctl`.

# References
 - [Linux]


### `tcgetpgrp`

```rust
fn tcgetpgrp<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

`tcgetpgrp(fd)`—Get the terminal foreground process group.

Also known as the `TIOCGPGRP` operation with `ioctl`.

On Linux, if `fd` is a pseudo-terminal, the underlying system call here can
return a pid of 0, which rustix's `Pid` type doesn't support. So rustix
instead handles this case by failing with `io::Errno::OPNOTSUPP` if the
pid is 0.

# References
 - [POSIX]
 - [Linux]



### `tcsetpgrp`

```rust
fn tcsetpgrp<Fd: AsFd>(fd: Fd, pid: Pid) -> io::Result<()>
```

`tcsetpgrp(fd, pid)`—Set the terminal foreground process group.

Also known as the `TIOCSPGRP` operation with `ioctl`.

# References
 - [POSIX]
 - [Linux]



### `tcsetattr`

```rust
fn tcsetattr<Fd: AsFd>(fd: Fd, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

`tcsetattr(fd)`—Set terminal attributes.

Also known as the `TCSETS` (or `TCSETS2` on Linux) operation with `ioctl`.

On Linux, this uses `TCSETS2`. If that fails in a way that indicates that
the host doesn't support it, this falls back to the old `TCSETS`, and fails
with `io::Errno::RANGE` if the input or output speeds cannot be supported.

# References
 - [POSIX `tcsetattr`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcsendbreak`

```rust
fn tcsendbreak<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`tcsendbreak(fd, 0)`—Transmit zero-valued bits.

This transmits zero-valued bits for at least 0.25 seconds.

This function does not have a `duration` parameter, and always uses the
implementation-defined value, which transmits for at least 0.25 seconds.

Also known as the `TCSBRK` operation with `ioctl`, with a duration
parameter of 0.

# References
 - [POSIX `tcsendbreak`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcdrain`

```rust
fn tcdrain<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`tcdrain(fd, duration)`—Wait until all pending output has been written.

# References
 - [POSIX `tcdrain`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcflush`

```rust
fn tcflush<Fd: AsFd>(fd: Fd, queue_selector: crate::termios::QueueSelector) -> io::Result<()>
```

`tcflush(fd, queue_selector)`—Wait until all pending output has been
written.

# References
 - [POSIX `tcflush`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcflow`

```rust
fn tcflow<Fd: AsFd>(fd: Fd, action: crate::termios::Action) -> io::Result<()>
```

`tcflow(fd, action)`—Suspend or resume transmission or reception.

# References
 - [POSIX `tcflow`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcgetsid`

```rust
fn tcgetsid<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

`tcgetsid(fd)`—Return the session ID of the current session with `fd` as
its controlling terminal.

# References
 - [POSIX]
 - [Linux]



### `tcsetwinsize`

```rust
fn tcsetwinsize<Fd: AsFd>(fd: Fd, winsize: crate::termios::Winsize) -> io::Result<()>
```

`tcsetwinsize(fd)`—Set the current terminal window size.

Also known as the `TIOCSWINSZ` operation with `ioctl`.

# References
 - [Linux]


