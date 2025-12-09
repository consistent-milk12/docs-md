*[console](../index.md) / [term](index.md)*

---

# Module `term`

## Contents

- [Structs](#structs)
  - [`ReadWritePair`](#readwritepair)
  - [`TermInner`](#terminner)
  - [`TermFeatures`](#termfeatures)
  - [`Term`](#term)
- [Enums](#enums)
  - [`TermTarget`](#termtarget)
  - [`TermFamily`](#termfamily)
- [Traits](#traits)
  - [`TermWrite`](#termwrite)
  - [`TermRead`](#termread)
- [Functions](#functions)
  - [`user_attended`](#user_attended)
  - [`user_attended_stderr`](#user_attended_stderr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReadWritePair`](#readwritepair) | struct |  |
| [`TermInner`](#terminner) | struct |  |
| [`TermFeatures`](#termfeatures) | struct | Gives access to the terminal features. |
| [`Term`](#term) | struct | Abstraction around a terminal. |
| [`TermTarget`](#termtarget) | enum | Where the term is writing. |
| [`TermFamily`](#termfamily) | enum | The family of the terminal. |
| [`TermWrite`](#termwrite) | trait |  |
| [`TermRead`](#termread) | trait |  |
| [`user_attended`](#user_attended) | fn | A fast way to check if the application has a user attended for stdout. |
| [`user_attended_stderr`](#user_attended_stderr) | fn | A fast way to check if the application has a user attended for stderr. |

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

- <span id="readwritepair-clone"></span>`fn clone(&self) -> ReadWritePair` — [`ReadWritePair`](#readwritepair)

##### `impl Debug for ReadWritePair`

- <span id="readwritepair-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="terminner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TermFeatures<'a>`

```rust
struct TermFeatures<'a>(&'a Term);
```

Gives access to the terminal features.

#### Implementations

- <span id="termfeatures-is-attended"></span>`fn is_attended(&self) -> bool`

- <span id="termfeatures-colors-supported"></span>`fn colors_supported(&self) -> bool`

- <span id="termfeatures-is-msys-tty"></span>`fn is_msys_tty(&self) -> bool`

- <span id="termfeatures-wants-emoji"></span>`fn wants_emoji(&self) -> bool`

- <span id="termfeatures-family"></span>`fn family(&self) -> TermFamily` — [`TermFamily`](../index.md)

#### Trait Implementations

##### `impl<'a> Clone for TermFeatures<'a>`

- <span id="termfeatures-clone"></span>`fn clone(&self) -> TermFeatures<'a>` — [`TermFeatures`](../index.md)

##### `impl<'a> Debug for TermFeatures<'a>`

- <span id="termfeatures-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="term-with-inner"></span>`fn with_inner(inner: TermInner) -> Term` — [`TermInner`](#terminner), [`Term`](../index.md)

- <span id="term-stdout"></span>`fn stdout() -> Term` — [`Term`](../index.md)

- <span id="term-stderr"></span>`fn stderr() -> Term` — [`Term`](../index.md)

- <span id="term-buffered-stdout"></span>`fn buffered_stdout() -> Term` — [`Term`](../index.md)

- <span id="term-buffered-stderr"></span>`fn buffered_stderr() -> Term` — [`Term`](../index.md)

- <span id="term-read-write-pair"></span>`fn read_write_pair<R, W>(read: R, write: W) -> Term` — [`Term`](../index.md)

- <span id="term-read-write-pair-with-style"></span>`fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term` — [`Style`](../index.md), [`Term`](../index.md)

- <span id="term-style"></span>`fn style(&self) -> Style` — [`Style`](../index.md)

- <span id="term-target"></span>`fn target(&self) -> TermTarget` — [`TermTarget`](../index.md)

- <span id="term-write-line"></span>`fn write_line(&self, s: &str) -> io::Result<()>`

- <span id="term-read-char"></span>`fn read_char(&self) -> io::Result<char>`

- <span id="term-read-key"></span>`fn read_key(&self) -> io::Result<Key>` — [`Key`](../index.md)

- <span id="term-read-key-raw"></span>`fn read_key_raw(&self) -> io::Result<Key>` — [`Key`](../index.md)

- <span id="term-read-line"></span>`fn read_line(&self) -> io::Result<String>`

- <span id="term-read-line-initial-text"></span>`fn read_line_initial_text(&self, initial: &str) -> io::Result<String>`

- <span id="term-read-secure-line"></span>`fn read_secure_line(&self) -> io::Result<String>`

- <span id="term-flush"></span>`fn flush(&self) -> io::Result<()>`

- <span id="term-is-term"></span>`fn is_term(&self) -> bool`

- <span id="term-features"></span>`fn features(&self) -> TermFeatures<'_>` — [`TermFeatures`](../index.md)

- <span id="term-size"></span>`fn size(&self) -> (u16, u16)`

- <span id="term-size-checked"></span>`fn size_checked(&self) -> Option<(u16, u16)>`

- <span id="term-move-cursor-to"></span>`fn move_cursor_to(&self, x: usize, y: usize) -> io::Result<()>`

- <span id="term-move-cursor-up"></span>`fn move_cursor_up(&self, n: usize) -> io::Result<()>`

- <span id="term-move-cursor-down"></span>`fn move_cursor_down(&self, n: usize) -> io::Result<()>`

- <span id="term-move-cursor-left"></span>`fn move_cursor_left(&self, n: usize) -> io::Result<()>`

- <span id="term-move-cursor-right"></span>`fn move_cursor_right(&self, n: usize) -> io::Result<()>`

- <span id="term-clear-line"></span>`fn clear_line(&self) -> io::Result<()>`

- <span id="term-clear-last-lines"></span>`fn clear_last_lines(&self, n: usize) -> io::Result<()>`

- <span id="term-clear-screen"></span>`fn clear_screen(&self) -> io::Result<()>`

- <span id="term-clear-to-end-of-screen"></span>`fn clear_to_end_of_screen(&self) -> io::Result<()>`

- <span id="term-clear-chars"></span>`fn clear_chars(&self, n: usize) -> io::Result<()>`

- <span id="term-set-title"></span>`fn set_title<T: Display>(&self, title: T)`

- <span id="term-show-cursor"></span>`fn show_cursor(&self) -> io::Result<()>`

- <span id="term-hide-cursor"></span>`fn hide_cursor(&self) -> io::Result<()>`

- <span id="term-write-through"></span>`fn write_through(&self, bytes: &[u8]) -> io::Result<()>`

- <span id="term-write-through-common"></span>`fn write_through_common(&self, bytes: &[u8]) -> io::Result<()>`

#### Trait Implementations

##### `impl AsRawFd for Term`

- <span id="term-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

##### `impl Clone for Term`

- <span id="term-clone"></span>`fn clone(&self) -> Term` — [`Term`](../index.md)

##### `impl Debug for Term`

- <span id="term-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Read for Term`

- <span id="term-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TermLike for console::Term`

##### `impl Write for Term`

- <span id="term-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="term-flush"></span>`fn flush(&mut self) -> io::Result<()>`

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

- <span id="termtarget-clone"></span>`fn clone(&self) -> TermTarget` — [`TermTarget`](../index.md)

##### `impl Debug for TermTarget`

- <span id="termtarget-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="termfamily-clone"></span>`fn clone(&self) -> TermFamily` — [`TermFamily`](../index.md)

##### `impl Copy for TermFamily`

##### `impl Debug for TermFamily`

- <span id="termfamily-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TermFamily`

##### `impl PartialEq for TermFamily`

- <span id="termfamily-eq"></span>`fn eq(&self, other: &TermFamily) -> bool` — [`TermFamily`](../index.md)

##### `impl StructuralPartialEq for TermFamily`

## Traits

### `TermWrite`

```rust
trait TermWrite: Write + Debug + AsRawFd + Send { ... }
```

#### Implementors

- `T`

### `TermRead`

```rust
trait TermRead: Read + Debug + AsRawFd + Send { ... }
```

#### Implementors

- `T`

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

