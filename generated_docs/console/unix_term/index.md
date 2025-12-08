*[console](../index.md) / [unix_term](index.md)*

---

# Module `unix_term`

## Contents

- [Enums](#enums)
  - [`Input`](#input)
- [Functions](#functions)
  - [`is_a_terminal`](#is_a_terminal)
  - [`is_a_color_terminal`](#is_a_color_terminal)
  - [`c_result`](#c_result)
  - [`terminal_size`](#terminal_size)
  - [`read_secure`](#read_secure)
  - [`poll_fd`](#poll_fd)
  - [`select_or_poll_term_fd`](#select_or_poll_term_fd)
  - [`read_single_char`](#read_single_char)
  - [`read_bytes`](#read_bytes)
  - [`read_single_key_impl`](#read_single_key_impl)
  - [`read_single_key`](#read_single_key)
  - [`key_from_utf8`](#key_from_utf8)
  - [`wants_emoji`](#wants_emoji)
  - [`set_title`](#set_title)
  - [`make_raw`](#make_raw)
- [Constants](#constants)
  - [`DEFAULT_WIDTH`](#default_width)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | enum |  |
| [`is_a_terminal`](#is_a_terminal) | fn |  |
| [`is_a_color_terminal`](#is_a_color_terminal) | fn |  |
| [`c_result`](#c_result) | fn |  |
| [`terminal_size`](#terminal_size) | fn |  |
| [`read_secure`](#read_secure) | fn |  |
| [`poll_fd`](#poll_fd) | fn |  |
| [`select_or_poll_term_fd`](#select_or_poll_term_fd) | fn |  |
| [`read_single_char`](#read_single_char) | fn |  |
| [`read_bytes`](#read_bytes) | fn |  |
| [`read_single_key_impl`](#read_single_key_impl) | fn |  |
| [`read_single_key`](#read_single_key) | fn |  |
| [`key_from_utf8`](#key_from_utf8) | fn |  |
| [`wants_emoji`](#wants_emoji) | fn |  |
| [`set_title`](#set_title) | fn |  |
| [`make_raw`](#make_raw) | fn |  |
| [`DEFAULT_WIDTH`](#default_width) | const |  |

## Enums

### `Input<T>`

```rust
enum Input<T> {
    Stdin(io::Stdin),
    File(T),
}
```

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

### `is_a_color_terminal`

```rust
fn is_a_color_terminal(out: &crate::term::Term) -> bool
```

### `c_result`

```rust
fn c_result<F: FnOnce() -> libc::c_int>(f: F) -> io::Result<()>
```

### `terminal_size`

```rust
fn terminal_size(out: &crate::term::Term) -> Option<(u16, u16)>
```

### `read_secure`

```rust
fn read_secure() -> io::Result<String>
```

### `poll_fd`

```rust
fn poll_fd(fd: std::os::fd::RawFd, timeout: i32) -> io::Result<bool>
```

### `select_or_poll_term_fd`

```rust
fn select_or_poll_term_fd(fd: std::os::fd::RawFd, timeout: i32) -> io::Result<bool>
```

### `read_single_char`

```rust
fn read_single_char(fd: std::os::fd::RawFd) -> io::Result<Option<char>>
```

### `read_bytes`

```rust
fn read_bytes(fd: std::os::fd::RawFd, buf: &mut [u8], count: u8) -> io::Result<u8>
```

### `read_single_key_impl`

```rust
fn read_single_key_impl(fd: std::os::fd::RawFd) -> Result<crate::kb::Key, io::Error>
```

### `read_single_key`

```rust
fn read_single_key(ctrlc_key: bool) -> io::Result<crate::kb::Key>
```

### `key_from_utf8`

```rust
fn key_from_utf8(buf: &[u8]) -> crate::kb::Key
```

### `wants_emoji`

```rust
fn wants_emoji() -> bool
```

### `set_title`

```rust
fn set_title<T: Display>(title: T)
```

### `make_raw`

```rust
fn make_raw(termios: &mut libc::termios)
```

## Constants

### `DEFAULT_WIDTH`

```rust
const DEFAULT_WIDTH: u16 = 80u16;
```

