*[console](../index.md) / [unix_term](index.md)*

---

# Module `unix_term`

## Contents

- [Enums](#enums)
  - [`Input`](#input)
- [Functions](#functions)
  - [`is_a_terminal`](#is-a-terminal)
  - [`is_a_color_terminal`](#is-a-color-terminal)
  - [`c_result`](#c-result)
  - [`terminal_size`](#terminal-size)
  - [`read_secure`](#read-secure)
  - [`poll_fd`](#poll-fd)
  - [`select_or_poll_term_fd`](#select-or-poll-term-fd)
  - [`read_single_char`](#read-single-char)
  - [`read_bytes`](#read-bytes)
  - [`read_single_key_impl`](#read-single-key-impl)
  - [`read_single_key`](#read-single-key)
  - [`key_from_utf8`](#key-from-utf8)
  - [`wants_emoji`](#wants-emoji)
  - [`set_title`](#set-title)
  - [`make_raw`](#make-raw)
- [Constants](#constants)
  - [`DEFAULT_WIDTH`](#default-width)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | enum |  |
| [`is_a_terminal`](#is-a-terminal) | fn |  |
| [`is_a_color_terminal`](#is-a-color-terminal) | fn |  |
| [`c_result`](#c-result) | fn |  |
| [`terminal_size`](#terminal-size) | fn |  |
| [`read_secure`](#read-secure) | fn |  |
| [`poll_fd`](#poll-fd) | fn |  |
| [`select_or_poll_term_fd`](#select-or-poll-term-fd) | fn |  |
| [`read_single_char`](#read-single-char) | fn |  |
| [`read_bytes`](#read-bytes) | fn |  |
| [`read_single_key_impl`](#read-single-key-impl) | fn |  |
| [`read_single_key`](#read-single-key) | fn |  |
| [`key_from_utf8`](#key-from-utf8) | fn |  |
| [`wants_emoji`](#wants-emoji) | fn |  |
| [`set_title`](#set-title) | fn |  |
| [`make_raw`](#make-raw) | fn |  |
| [`DEFAULT_WIDTH`](#default-width) | const |  |

## Enums

### `Input<T>`

```rust
enum Input<T> {
    Stdin(io::Stdin),
    File(T),
}
```

*Defined in [`console-0.16.1/src/unix_term.rs:68-71`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L68-L71)*

#### Implementations

- <span id="input-buffered"></span>`fn buffered() -> io::Result<Self>`

#### Trait Implementations

##### `impl AsRawFd for Input<fs::File>`

- <span id="input-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

## Functions

### `is_a_terminal`

```rust
fn is_a_terminal(out: &impl AsRawFd) -> bool
```

*Defined in [`console-0.16.1/src/unix_term.rs:20-22`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L20-L22)*

### `is_a_color_terminal`

```rust
fn is_a_color_terminal(out: &crate::term::Term) -> bool
```

*Defined in [`console-0.16.1/src/unix_term.rs:24-37`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L24-L37)*

### `c_result`

```rust
fn c_result<F: FnOnce() -> libc::c_int>(f: F) -> io::Result<()>
```

*Defined in [`console-0.16.1/src/unix_term.rs:39-46`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L39-L46)*

### `terminal_size`

```rust
fn terminal_size(out: &crate::term::Term) -> Option<(u16, u16)>
```

*Defined in [`console-0.16.1/src/unix_term.rs:48-66`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L48-L66)*

### `read_secure`

```rust
fn read_secure() -> io::Result<String>
```

*Defined in [`console-0.16.1/src/unix_term.rs:125-145`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L125-L145)*

### `poll_fd`

```rust
fn poll_fd(fd: std::os::fd::RawFd, timeout: i32) -> io::Result<bool>
```

*Defined in [`console-0.16.1/src/unix_term.rs:147-159`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L147-L159)*

### `select_or_poll_term_fd`

```rust
fn select_or_poll_term_fd(fd: std::os::fd::RawFd, timeout: i32) -> io::Result<bool>
```

*Defined in [`console-0.16.1/src/unix_term.rs:194-205`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L194-L205)*

### `read_single_char`

```rust
fn read_single_char(fd: std::os::fd::RawFd) -> io::Result<Option<char>>
```

*Defined in [`console-0.16.1/src/unix_term.rs:207-221`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L207-L221)*

### `read_bytes`

```rust
fn read_bytes(fd: std::os::fd::RawFd, buf: &mut [u8], count: u8) -> io::Result<u8>
```

*Defined in [`console-0.16.1/src/unix_term.rs:226-243`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L226-L243)*

### `read_single_key_impl`

```rust
fn read_single_key_impl(fd: std::os::fd::RawFd) -> Result<crate::kb::Key, io::Error>
```

*Defined in [`console-0.16.1/src/unix_term.rs:245-336`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L245-L336)*

### `read_single_key`

```rust
fn read_single_key(ctrlc_key: bool) -> io::Result<crate::kb::Key>
```

*Defined in [`console-0.16.1/src/unix_term.rs:338-365`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L338-L365)*

### `key_from_utf8`

```rust
fn key_from_utf8(buf: &[u8]) -> crate::kb::Key
```

*Defined in [`console-0.16.1/src/unix_term.rs:367-374`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L367-L374)*

### `wants_emoji`

```rust
fn wants_emoji() -> bool
```

*Defined in [`console-0.16.1/src/unix_term.rs:388-390`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L388-L390)*

### `set_title`

```rust
fn set_title<T: Display>(title: T)
```

*Defined in [`console-0.16.1/src/unix_term.rs:392-394`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L392-L394)*

### `make_raw`

```rust
fn make_raw(termios: &mut libc::termios)
```

*Defined in [`console-0.16.1/src/unix_term.rs:415-418`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L415-L418)*

## Constants

### `DEFAULT_WIDTH`
```rust
const DEFAULT_WIDTH: u16 = 80u16;
```

*Defined in [`console-0.16.1/src/unix_term.rs:17`](../../../.source_1765210505/console-0.16.1/src/unix_term.rs#L17)*

