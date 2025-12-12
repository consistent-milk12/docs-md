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
  - [`user_attended`](#user-attended)
  - [`user_attended_stderr`](#user-attended-stderr)

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
| [`user_attended`](#user-attended) | fn | A fast way to check if the application has a user attended for stdout. |
| [`user_attended_stderr`](#user-attended-stderr) | fn | A fast way to check if the application has a user attended for stderr. |

## Structs

### `ReadWritePair`

```rust
struct ReadWritePair {
    read: alloc::sync::Arc<std::sync::Mutex<dyn TermRead>>,
    write: alloc::sync::Arc<std::sync::Mutex<dyn TermWrite>>,
    style: crate::utils::Style,
}
```

*Defined in [`console-0.16.1/src/term.rs:24-29`](../../../.source_1765210505/console-0.16.1/src/term.rs#L24-L29)*

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

*Defined in [`console-0.16.1/src/term.rs:41-46`](../../../.source_1765210505/console-0.16.1/src/term.rs#L41-L46)*

#### Trait Implementations

##### `impl Debug for TermInner`

- <span id="terminner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TermFeatures<'a>`

```rust
struct TermFeatures<'a>(&'a Term);
```

*Defined in [`console-0.16.1/src/term.rs:63`](../../../.source_1765210505/console-0.16.1/src/term.rs#L63)*

Gives access to the terminal features.

#### Implementations

- <span id="termfeatures-is-attended"></span>`fn is_attended(&self) -> bool`

- <span id="termfeatures-colors-supported"></span>`fn colors_supported(&self) -> bool`

- <span id="termfeatures-is-msys-tty"></span>`fn is_msys_tty(&self) -> bool`

- <span id="termfeatures-wants-emoji"></span>`fn wants_emoji(&self) -> bool`

- <span id="termfeatures-family"></span>`fn family(&self) -> TermFamily` — [`TermFamily`](#termfamily)

#### Trait Implementations

##### `impl Clone for TermFeatures<'a>`

- <span id="termfeatures-clone"></span>`fn clone(&self) -> TermFeatures<'a>` — [`TermFeatures`](#termfeatures)

##### `impl Debug for TermFeatures<'a>`

- <span id="termfeatures-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Term`

```rust
struct Term {
    inner: alloc::sync::Arc<TermInner>,
    is_msys_tty: bool,
    is_tty: bool,
}
```

*Defined in [`console-0.16.1/src/term.rs:129-133`](../../../.source_1765210505/console-0.16.1/src/term.rs#L129-L133)*

Abstraction around a terminal.

A terminal can be cloned.  If a buffer is used it's shared across all
clones which means it largely acts as a handle.

#### Implementations

- <span id="term-with-inner"></span>`fn with_inner(inner: TermInner) -> Term` — [`TermInner`](#terminner), [`Term`](#term)

- <span id="term-stdout"></span>`fn stdout() -> Term` — [`Term`](#term)

- <span id="term-stderr"></span>`fn stderr() -> Term` — [`Term`](#term)

- <span id="term-buffered-stdout"></span>`fn buffered_stdout() -> Term` — [`Term`](#term)

- <span id="term-buffered-stderr"></span>`fn buffered_stderr() -> Term` — [`Term`](#term)

- <span id="term-read-write-pair"></span>`fn read_write_pair<R, W>(read: R, write: W) -> Term` — [`Term`](#term)

- <span id="term-read-write-pair-with-style"></span>`fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term` — [`Style`](../utils/index.md#style), [`Term`](#term)

- <span id="term-style"></span>`fn style(&self) -> Style` — [`Style`](../utils/index.md#style)

- <span id="term-target"></span>`fn target(&self) -> TermTarget` — [`TermTarget`](#termtarget)

- <span id="term-write-line"></span>`fn write_line(&self, s: &str) -> io::Result<()>`

- <span id="term-read-char"></span>`fn read_char(&self) -> io::Result<char>`

- <span id="term-read-key"></span>`fn read_key(&self) -> io::Result<Key>` — [`Key`](../kb/index.md#key)

- <span id="term-read-key-raw"></span>`fn read_key_raw(&self) -> io::Result<Key>` — [`Key`](../kb/index.md#key)

- <span id="term-read-line"></span>`fn read_line(&self) -> io::Result<String>`

- <span id="term-read-line-initial-text"></span>`fn read_line_initial_text(&self, initial: &str) -> io::Result<String>`

- <span id="term-read-secure-line"></span>`fn read_secure_line(&self) -> io::Result<String>`

- <span id="term-flush"></span>`fn flush(&self) -> io::Result<()>`

- <span id="term-is-term"></span>`fn is_term(&self) -> bool`

- <span id="term-features"></span>`fn features(&self) -> TermFeatures<'_>` — [`TermFeatures`](#termfeatures)

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

- <span id="term-clone"></span>`fn clone(&self) -> Term` — [`Term`](#term)

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

*Defined in [`console-0.16.1/src/term.rs:33-38`](../../../.source_1765210505/console-0.16.1/src/term.rs#L33-L38)*

Where the term is writing.

#### Trait Implementations

##### `impl Clone for TermTarget`

- <span id="termtarget-clone"></span>`fn clone(&self) -> TermTarget` — [`TermTarget`](#termtarget)

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

*Defined in [`console-0.16.1/src/term.rs:50-59`](../../../.source_1765210505/console-0.16.1/src/term.rs#L50-L59)*

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

- <span id="termfamily-clone"></span>`fn clone(&self) -> TermFamily` — [`TermFamily`](#termfamily)

##### `impl Copy for TermFamily`

##### `impl Debug for TermFamily`

- <span id="termfamily-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TermFamily`

##### `impl PartialEq for TermFamily`

- <span id="termfamily-eq"></span>`fn eq(&self, other: &TermFamily) -> bool` — [`TermFamily`](#termfamily)

##### `impl StructuralPartialEq for TermFamily`

## Traits

### `TermWrite`

```rust
trait TermWrite: Write + Debug + AsRawFd + Send { ... }
```

*Defined in [`console-0.16.1/src/term.rs:13`](../../../.source_1765210505/console-0.16.1/src/term.rs#L13)*

#### Implementors

- `T`

### `TermRead`

```rust
trait TermRead: Read + Debug + AsRawFd + Send { ... }
```

*Defined in [`console-0.16.1/src/term.rs:18`](../../../.source_1765210505/console-0.16.1/src/term.rs#L18)*

#### Implementors

- `T`

## Functions

### `user_attended`

```rust
fn user_attended() -> bool
```

*Defined in [`console-0.16.1/src/term.rs:578-580`](../../../.source_1765210505/console-0.16.1/src/term.rs#L578-L580)*

A fast way to check if the application has a user attended for stdout.

This means that stdout is connected to a terminal instead of a
file or redirected by other means. This is a shortcut for
checking the `is_attended` feature on the stdout terminal.

### `user_attended_stderr`

```rust
fn user_attended_stderr() -> bool
```

*Defined in [`console-0.16.1/src/term.rs:588-590`](../../../.source_1765210505/console-0.16.1/src/term.rs#L588-L590)*

A fast way to check if the application has a user attended for stderr.

This means that stderr is connected to a terminal instead of a
file or redirected by other means. This is a shortcut for
checking the `is_attended` feature on the stderr terminal.

