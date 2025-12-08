*[console](../index.md) / [term](index.md)*

---

# Module `term`

## Structs

### `ReadWritePair`

```rust
struct ReadWritePair {
    read: alloc::sync::Arc<std::sync::Mutex<dyn TermRead>>,
    write: alloc::sync::Arc<std::sync::Mutex<dyn TermWrite>>,
    style: crate::utils::Style,
}
```

#### Trait Implementations

##### `impl Clone for ReadWritePair`

- `fn clone(self: &Self) -> ReadWritePair` — [`ReadWritePair`](#readwritepair)

##### `impl Debug for ReadWritePair`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TermInner`

```rust
struct TermInner {
    target: TermTarget,
    buffer: Option<std::sync::Mutex<Vec<u8>>>,
    prompt: std::sync::RwLock<String>,
    prompt_guard: std::sync::Mutex<()>,
}
```

#### Trait Implementations

##### `impl Debug for TermInner`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TermFeatures<'a>`

```rust
struct TermFeatures<'a>(&'a Term);
```

Gives access to the terminal features.

#### Implementations

- `fn is_attended(self: &Self) -> bool`

- `fn colors_supported(self: &Self) -> bool`

- `fn is_msys_tty(self: &Self) -> bool`

- `fn wants_emoji(self: &Self) -> bool`

- `fn family(self: &Self) -> TermFamily` — [`TermFamily`](../index.md)

#### Trait Implementations

##### `impl<'a> Clone for TermFeatures<'a>`

- `fn clone(self: &Self) -> TermFeatures<'a>` — [`TermFeatures`](../index.md)

##### `impl<'a> Debug for TermFeatures<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Term`

```rust
struct Term {
    inner: alloc::sync::Arc<TermInner>,
    is_msys_tty: bool,
    is_tty: bool,
}
```

Abstraction around a terminal.

A terminal can be cloned.  If a buffer is used it's shared across all
clones which means it largely acts as a handle.

#### Implementations

- `fn with_inner(inner: TermInner) -> Term` — [`TermInner`](#terminner), [`Term`](../index.md)

- `fn stdout() -> Term` — [`Term`](../index.md)

- `fn stderr() -> Term` — [`Term`](../index.md)

- `fn buffered_stdout() -> Term` — [`Term`](../index.md)

- `fn buffered_stderr() -> Term` — [`Term`](../index.md)

- `fn read_write_pair<R, W>(read: R, write: W) -> Term` — [`Term`](../index.md)

- `fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term` — [`Style`](../index.md), [`Term`](../index.md)

- `fn style(self: &Self) -> Style` — [`Style`](../index.md)

- `fn target(self: &Self) -> TermTarget` — [`TermTarget`](../index.md)

- `fn write_line(self: &Self, s: &str) -> io::Result<()>`

- `fn read_char(self: &Self) -> io::Result<char>`

- `fn read_key(self: &Self) -> io::Result<Key>` — [`Key`](../index.md)

- `fn read_key_raw(self: &Self) -> io::Result<Key>` — [`Key`](../index.md)

- `fn read_line(self: &Self) -> io::Result<String>`

- `fn read_line_initial_text(self: &Self, initial: &str) -> io::Result<String>`

- `fn read_secure_line(self: &Self) -> io::Result<String>`

- `fn flush(self: &Self) -> io::Result<()>`

- `fn is_term(self: &Self) -> bool`

- `fn features(self: &Self) -> TermFeatures<'_>` — [`TermFeatures`](../index.md)

- `fn size(self: &Self) -> (u16, u16)`

- `fn size_checked(self: &Self) -> Option<(u16, u16)>`

- `fn move_cursor_to(self: &Self, x: usize, y: usize) -> io::Result<()>`

- `fn move_cursor_up(self: &Self, n: usize) -> io::Result<()>`

- `fn move_cursor_down(self: &Self, n: usize) -> io::Result<()>`

- `fn move_cursor_left(self: &Self, n: usize) -> io::Result<()>`

- `fn move_cursor_right(self: &Self, n: usize) -> io::Result<()>`

- `fn clear_line(self: &Self) -> io::Result<()>`

- `fn clear_last_lines(self: &Self, n: usize) -> io::Result<()>`

- `fn clear_screen(self: &Self) -> io::Result<()>`

- `fn clear_to_end_of_screen(self: &Self) -> io::Result<()>`

- `fn clear_chars(self: &Self, n: usize) -> io::Result<()>`

- `fn set_title<T: Display>(self: &Self, title: T)`

- `fn show_cursor(self: &Self) -> io::Result<()>`

- `fn hide_cursor(self: &Self) -> io::Result<()>`

- `fn write_through(self: &Self, bytes: &[u8]) -> io::Result<()>`

- `fn write_through_common(self: &Self, bytes: &[u8]) -> io::Result<()>`

#### Trait Implementations

##### `impl AsRawFd for Term`

- `fn as_raw_fd(self: &Self) -> RawFd`

##### `impl Clone for Term`

- `fn clone(self: &Self) -> Term` — [`Term`](../index.md)

##### `impl Debug for Term`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Read for Term`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TermLike for console::Term`

##### `impl Write for Term`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

## Enums

### `TermTarget`

```rust
enum TermTarget {
    Stdout,
    Stderr,
    ReadWritePair(ReadWritePair),
}
```

Where the term is writing.

#### Trait Implementations

##### `impl Clone for TermTarget`

- `fn clone(self: &Self) -> TermTarget` — [`TermTarget`](../index.md)

##### `impl Debug for TermTarget`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TermFamily`

```rust
enum TermFamily {
    File,
    UnixTerm,
    WindowsConsole,
    Dummy,
}
```

The family of the terminal.

#### Variants

- **`File`**

  Redirected to a file or file like thing.

- **`UnixTerm`**

  A standard unix terminal.

- **`WindowsConsole`**

  A cmd.exe like windows console.

- **`Dummy`**

  A dummy terminal (for instance on wasm)

#### Trait Implementations

##### `impl Clone for TermFamily`

- `fn clone(self: &Self) -> TermFamily` — [`TermFamily`](../index.md)

##### `impl Copy for TermFamily`

##### `impl Debug for TermFamily`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TermFamily`

##### `impl PartialEq for TermFamily`

- `fn eq(self: &Self, other: &TermFamily) -> bool` — [`TermFamily`](../index.md)

##### `impl StructuralPartialEq for TermFamily`

## Traits

### `TermWrite`

```rust
trait TermWrite: Write + Debug + AsRawFd + Send { ... }
```

### `TermRead`

```rust
trait TermRead: Read + Debug + AsRawFd + Send { ... }
```

## Functions

### `user_attended`

```rust
fn user_attended() -> bool
```

A fast way to check if the application has a user attended for stdout.

This means that stdout is connected to a terminal instead of a
file or redirected by other means. This is a shortcut for
checking the `is_attended` feature on the stdout terminal.

### `user_attended_stderr`

```rust
fn user_attended_stderr() -> bool
```

A fast way to check if the application has a user attended for stderr.

This means that stderr is connected to a terminal instead of a
file or redirected by other means. This is a shortcut for
checking the `is_attended` feature on the stderr terminal.

