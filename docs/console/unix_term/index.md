*[console](../index.md) / [unix_term](index.md)*

---

# Module `unix_term`

## Enums

### `Input<T>`

```rust
enum Input<T> {
    Stdin(io::Stdin),
    File(T),
}
```

#### Implementations

- `fn buffered() -> io::Result<Self>`

#### Trait Implementations

##### `impl AsRawFd for Input<std::io::BufReader<fs::File>>`

- `fn as_raw_fd(self: &Self) -> RawFd`

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

