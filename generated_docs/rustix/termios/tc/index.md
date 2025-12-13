*[rustix](../../index.md) / [termios](../index.md) / [tc](index.md)*

---

# Module `tc`

## Contents

- [Structs](#structs)
  - [`Pid`](#pid)
- [Functions](#functions)
  - [`tcgetattr`](#tcgetattr)
  - [`tcgetwinsize`](#tcgetwinsize)
  - [`tcgetpgrp`](#tcgetpgrp)
  - [`tcsetpgrp`](#tcsetpgrp)
  - [`tcsetattr`](#tcsetattr)
  - [`tcsendbreak`](#tcsendbreak)
  - [`tcdrain`](#tcdrain)
  - [`tcflush`](#tcflush)
  - [`tcflow`](#tcflow)
  - [`tcgetsid`](#tcgetsid)
  - [`tcsetwinsize`](#tcsetwinsize)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pid`](#pid) | struct |  |
| [`tcgetattr`](#tcgetattr) | fn | `tcgetattr(fd)`—Get terminal attributes. |
| [`tcgetwinsize`](#tcgetwinsize) | fn | `tcgetwinsize(fd)`—Get the current terminal window size. |
| [`tcgetpgrp`](#tcgetpgrp) | fn | `tcgetpgrp(fd)`—Get the terminal foreground process group. |
| [`tcsetpgrp`](#tcsetpgrp) | fn | `tcsetpgrp(fd, pid)`—Set the terminal foreground process group. |
| [`tcsetattr`](#tcsetattr) | fn | `tcsetattr(fd)`—Set terminal attributes. |
| [`tcsendbreak`](#tcsendbreak) | fn | `tcsendbreak(fd, 0)`—Transmit zero-valued bits. |
| [`tcdrain`](#tcdrain) | fn | `tcdrain(fd, duration)`—Wait until all pending output has been written. |
| [`tcflush`](#tcflush) | fn | `tcflush(fd, queue_selector)`—Wait until all pending output has been written. |
| [`tcflow`](#tcflow) | fn | `tcflow(fd, action)`—Suspend or resume transmission or reception. |
| [`tcgetsid`](#tcgetsid) | fn | `tcgetsid(fd)`—Return the session ID of the current session with `fd` as its controlling terminal. |
| [`tcsetwinsize`](#tcsetwinsize) | fn | `tcsetwinsize(fd)`—Set the current terminal window size. |

## Structs

### `Pid`

```rust
struct Pid(core::num::NonZeroI32);
```

*Defined in [`rustix-1.1.2/src/pid.rs:19`](../../../../.source_1765633015/rustix-1.1.2/src/pid.rs#L19)*

`pid_t`—A non-zero Unix process ID.

This is a pid, and not a pidfd. It is not a file descriptor, and the
process it refers to could disappear at any time and be replaced by
another, unrelated, process.

On Linux, `Pid` values are also used to identify threads.

#### Implementations

- <span id="pid-const-init"></span>`const INIT: Self`

- <span id="pid-from-raw"></span>`const fn from_raw(raw: i32) -> Option<Self>`

  Converts a `RawPid` into a `Pid`.

  

  Returns `Some` for positive values, and `None` for zero values.

  

  This is safe because a `Pid` is a number without any guarantees for the

  kernel. Non-child `Pid`s are always racy for any syscalls, but can only

  cause logic errors. If you want race-free access to or control of

  non-child processes, please consider other mechanisms like [pidfd] on

  Linux.

  

  Passing a negative number doesn't invoke undefined behavior, but it

  may cause unexpected behavior.

- <span id="pid-from-raw-unchecked"></span>`const unsafe fn from_raw_unchecked(raw: i32) -> Self`

  Converts a known positive `RawPid` into a `Pid`.

  

  Passing a negative number doesn't invoke undefined behavior, but it

  may cause unexpected behavior.

  

  # Safety

  

  The caller must guarantee `raw` is non-zero.

- <span id="pid-from-child"></span>`fn from_child(child: &std::process::Child) -> Self`

  Creates a `Pid` holding the ID of the given child process.

- <span id="pid-as-raw-nonzero"></span>`const fn as_raw_nonzero(self) -> NonZeroI32`

  Converts a `Pid` into a `NonZeroI32`.

- <span id="pid-as-raw-pid"></span>`const fn as_raw_pid(self) -> i32`

  Converts a `Pid` into a `RawPid`.

  

  This is the same as `self.as_raw_nonzero().get()`.

- <span id="pid-as-raw"></span>`const fn as_raw(pid: Option<Self>) -> i32`

  Converts an `Option<Pid>` into a `RawPid`.

- <span id="pid-is-init"></span>`const fn is_init(self) -> bool`

  Test whether this pid represents the init process (`Pid::INIT`).

#### Trait Implementations

##### `impl Any for Pid`

- <span id="pid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for Pid`

- <span id="pid-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for Pid`

- <span id="pid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pid`

- <span id="pid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Pid`

- <span id="pid-clone"></span>`fn clone(&self) -> Pid` — [`Pid`](../../pid/index.md#pid)

##### `impl CloneToUninit for Pid`

- <span id="pid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Pid`

##### `impl Debug for Pid`

- <span id="pid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Pid`

- <span id="pid-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Pid`

##### `impl<T> From for Pid`

- <span id="pid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Pid`

- <span id="pid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Pid`

- <span id="pid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl LowerExp for Pid`

- <span id="pid-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex for Pid`

- <span id="pid-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal for Pid`

- <span id="pid-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Pid`

- <span id="pid-partialeq-eq"></span>`fn eq(&self, other: &Pid) -> bool` — [`Pid`](../../pid/index.md#pid)

##### `impl StructuralPartialEq for Pid`

##### `impl ToOwned for Pid`

- <span id="pid-toowned-type-owned"></span>`type Owned = T`

- <span id="pid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Pid`

- <span id="pid-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Pid`

- <span id="pid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pid`

- <span id="pid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp for Pid`

- <span id="pid-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex for Pid`

- <span id="pid-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `tcgetattr`

```rust
fn tcgetattr<Fd: AsFd>(fd: Fd) -> io::Result<crate::termios::Termios>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:30-32`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L30-L32)*

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

*Defined in [`rustix-1.1.2/src/termios/tc.rs:50-52`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L50-L52)*

`tcgetwinsize(fd)`—Get the current terminal window size.

Also known as the `TIOCGWINSZ` operation with `ioctl`.

# References
 - [Linux]


### `tcgetpgrp`

```rust
fn tcgetpgrp<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:72-74`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L72-L74)*

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

*Defined in [`rustix-1.1.2/src/termios/tc.rs:89-91`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L89-L91)*

`tcsetpgrp(fd, pid)`—Set the terminal foreground process group.

Also known as the `TIOCSPGRP` operation with `ioctl`.

# References
 - [POSIX]
 - [Linux]



### `tcsetattr`

```rust
fn tcsetattr<Fd: AsFd>(fd: Fd, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:114-120`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L114-L120)*

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

*Defined in [`rustix-1.1.2/src/termios/tc.rs:142-144`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L142-L144)*

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

*Defined in [`rustix-1.1.2/src/termios/tc.rs:158-160`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L158-L160)*

`tcdrain(fd, duration)`—Wait until all pending output has been written.

# References
 - [POSIX `tcdrain`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcflush`

```rust
fn tcflush<Fd: AsFd>(fd: Fd, queue_selector: crate::termios::QueueSelector) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:176-178`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L176-L178)*

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

*Defined in [`rustix-1.1.2/src/termios/tc.rs:193-195`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L193-L195)*

`tcflow(fd, action)`—Suspend or resume transmission or reception.

# References
 - [POSIX `tcflow`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcgetsid`

```rust
fn tcgetsid<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:208-210`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L208-L210)*

`tcgetsid(fd)`—Return the session ID of the current session with `fd` as
its controlling terminal.

# References
 - [POSIX]
 - [Linux]



### `tcsetwinsize`

```rust
fn tcsetwinsize<Fd: AsFd>(fd: Fd, winsize: crate::termios::Winsize) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:223-225`](../../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L223-L225)*

`tcsetwinsize(fd)`—Set the current terminal window size.

Also known as the `TIOCSWINSZ` operation with `ioctl`.

# References
 - [Linux]


