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

*Defined in [`console-0.16.1/src/term.rs:24-29`](../../../.source_1765521767/console-0.16.1/src/term.rs#L24-L29)*

#### Trait Implementations

##### `impl Any for ReadWritePair`

- <span id="readwritepair-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadWritePair`

- <span id="readwritepair-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadWritePair`

- <span id="readwritepair-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ReadWritePair`

- <span id="readwritepair-clone"></span>`fn clone(&self) -> ReadWritePair` — [`ReadWritePair`](#readwritepair)

##### `impl CloneToUninit for ReadWritePair`

- <span id="readwritepair-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ReadWritePair`

- <span id="readwritepair-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadWritePair`

- <span id="readwritepair-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadWritePair`

- <span id="readwritepair-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ReadWritePair`

- <span id="readwritepair-toowned-type-owned"></span>`type Owned = T`

- <span id="readwritepair-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="readwritepair-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReadWritePair`

- <span id="readwritepair-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readwritepair-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadWritePair`

- <span id="readwritepair-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readwritepair-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TermInner`

```rust
struct TermInner {
    target: TermTarget,
    buffer: Option<std::sync::Mutex<Vec<u8>>>,
    prompt: std::sync::RwLock<String>,
    prompt_guard: std::sync::Mutex<()>,
}
```

*Defined in [`console-0.16.1/src/term.rs:41-46`](../../../.source_1765521767/console-0.16.1/src/term.rs#L41-L46)*

#### Trait Implementations

##### `impl Any for TermInner`

- <span id="terminner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TermInner`

- <span id="terminner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TermInner`

- <span id="terminner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for TermInner`

- <span id="terminner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TermInner`

- <span id="terminner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TermInner`

- <span id="terminner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for TermInner`

- <span id="terminner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="terminner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TermInner`

- <span id="terminner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="terminner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TermFeatures<'a>`

```rust
struct TermFeatures<'a>(&'a Term);
```

*Defined in [`console-0.16.1/src/term.rs:63`](../../../.source_1765521767/console-0.16.1/src/term.rs#L63)*

Gives access to the terminal features.

#### Implementations

- <span id="termfeatures-is-attended"></span>`fn is_attended(&self) -> bool`

  Check if this is a real user attended terminal (`isatty`)

- <span id="termfeatures-colors-supported"></span>`fn colors_supported(&self) -> bool`

  Check if colors are supported by this terminal.

  

  This does not check if colors are enabled.  Currently all terminals

  are considered to support colors

- <span id="termfeatures-is-msys-tty"></span>`fn is_msys_tty(&self) -> bool`

  Check if this terminal is an msys terminal.

  

  This is sometimes useful to disable features that are known to not

  work on msys terminals or require special handling.

- <span id="termfeatures-wants-emoji"></span>`fn wants_emoji(&self) -> bool`

  Check if this terminal wants emojis.

- <span id="termfeatures-family"></span>`fn family(&self) -> TermFamily` — [`TermFamily`](#termfamily)

  Return the family of the terminal.

#### Trait Implementations

##### `impl Any for TermFeatures<'a>`

- <span id="termfeatures-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TermFeatures<'a>`

- <span id="termfeatures-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TermFeatures<'a>`

- <span id="termfeatures-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TermFeatures<'a>`

- <span id="termfeatures-clone"></span>`fn clone(&self) -> TermFeatures<'a>` — [`TermFeatures`](#termfeatures)

##### `impl CloneToUninit for TermFeatures<'a>`

- <span id="termfeatures-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TermFeatures<'a>`

- <span id="termfeatures-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TermFeatures<'a>`

- <span id="termfeatures-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TermFeatures<'a>`

- <span id="termfeatures-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for TermFeatures<'a>`

- <span id="termfeatures-toowned-type-owned"></span>`type Owned = T`

- <span id="termfeatures-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="termfeatures-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TermFeatures<'a>`

- <span id="termfeatures-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termfeatures-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TermFeatures<'a>`

- <span id="termfeatures-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termfeatures-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Term`

```rust
struct Term {
    inner: alloc::sync::Arc<TermInner>,
    is_msys_tty: bool,
    is_tty: bool,
}
```

*Defined in [`console-0.16.1/src/term.rs:129-133`](../../../.source_1765521767/console-0.16.1/src/term.rs#L129-L133)*

Abstraction around a terminal.

A terminal can be cloned.  If a buffer is used it's shared across all
clones which means it largely acts as a handle.

#### Implementations

- <span id="term-with-inner"></span>`fn with_inner(inner: TermInner) -> Term` — [`TermInner`](#terminner), [`Term`](#term)

- <span id="term-stdout"></span>`fn stdout() -> Term` — [`Term`](#term)

  Return a new unbuffered terminal.

- <span id="term-stderr"></span>`fn stderr() -> Term` — [`Term`](#term)

  Return a new unbuffered terminal to stderr.

- <span id="term-buffered-stdout"></span>`fn buffered_stdout() -> Term` — [`Term`](#term)

  Return a new buffered terminal.

- <span id="term-buffered-stderr"></span>`fn buffered_stderr() -> Term` — [`Term`](#term)

  Return a new buffered terminal to stderr.

- <span id="term-read-write-pair"></span>`fn read_write_pair<R, W>(read: R, write: W) -> Term` — [`Term`](#term)

  Return a terminal for the given Read/Write pair styled like stderr.

- <span id="term-read-write-pair-with-style"></span>`fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term` — [`Style`](../utils/index.md#style), [`Term`](#term)

  Return a terminal for the given Read/Write pair.

- <span id="term-style"></span>`fn style(&self) -> Style` — [`Style`](../utils/index.md#style)

  Return the style for this terminal.

- <span id="term-target"></span>`fn target(&self) -> TermTarget` — [`TermTarget`](#termtarget)

  Return the target of this terminal.

- <span id="term-write-line"></span>`fn write_line(&self, s: &str) -> io::Result<()>`

  Write a string to the terminal and add a newline.

- <span id="term-read-char"></span>`fn read_char(&self) -> io::Result<char>`

  Read a single character from the terminal.

  

  This does not echo the character and blocks until a single character

  or complete key chord is entered.  If the terminal is not user attended

  the return value will be an error.

- <span id="term-read-key"></span>`fn read_key(&self) -> io::Result<Key>` — [`Key`](../kb/index.md#key)

  Read a single key from the terminal.

  

  This does not echo anything.  If the terminal is not user attended

  the return value will always be the unknown key.

- <span id="term-read-key-raw"></span>`fn read_key_raw(&self) -> io::Result<Key>` — [`Key`](../kb/index.md#key)

- <span id="term-read-line"></span>`fn read_line(&self) -> io::Result<String>`

  Read one line of input.

  

  This does not include the trailing newline.  If the terminal is not

  user attended the return value will always be an empty string.

- <span id="term-read-line-initial-text"></span>`fn read_line_initial_text(&self, initial: &str) -> io::Result<String>`

  Read one line of input with initial text.

  

  This method blocks until no other thread is waiting for this read_line

  before reading a line from the terminal.

  This does not include the trailing newline.  If the terminal is not

  user attended the return value will always be an empty string.

- <span id="term-read-secure-line"></span>`fn read_secure_line(&self) -> io::Result<String>`

  Read a line of input securely.

  

  This is similar to `read_line` but will not echo the output.  This

  also switches the terminal into a different mode where not all

  characters might be accepted.

- <span id="term-flush"></span>`fn flush(&self) -> io::Result<()>`

  Flush internal buffers.

  

  This forces the contents of the internal buffer to be written to

  the terminal.  This is unnecessary for unbuffered terminals which

  will automatically flush.

- <span id="term-is-term"></span>`fn is_term(&self) -> bool`

  Check if the terminal is indeed a terminal.

- <span id="term-features"></span>`fn features(&self) -> TermFeatures<'_>` — [`TermFeatures`](#termfeatures)

  Check for common terminal features.

- <span id="term-size"></span>`fn size(&self) -> (u16, u16)`

  Return the terminal size in rows and columns or gets sensible defaults.

- <span id="term-size-checked"></span>`fn size_checked(&self) -> Option<(u16, u16)>`

  Return the terminal size in rows and columns.

  

  If the size cannot be reliably determined `None` is returned.

- <span id="term-move-cursor-to"></span>`fn move_cursor_to(&self, x: usize, y: usize) -> io::Result<()>`

  Move the cursor to row `x` and column `y`. Values are 0-based.

- <span id="term-move-cursor-up"></span>`fn move_cursor_up(&self, n: usize) -> io::Result<()>`

  Move the cursor up by `n` lines, if possible.

  

  If there are less than `n` lines above the current cursor position,

  the cursor is moved to the top line of the terminal (i.e., as far up as possible).

- <span id="term-move-cursor-down"></span>`fn move_cursor_down(&self, n: usize) -> io::Result<()>`

  Move the cursor down by `n` lines, if possible.

  

  If there are less than `n` lines below the current cursor position,

  the cursor is moved to the bottom line of the terminal (i.e., as far down as possible).

- <span id="term-move-cursor-left"></span>`fn move_cursor_left(&self, n: usize) -> io::Result<()>`

  Move the cursor `n` characters to the left, if possible.

  

  If there are fewer than `n` characters to the left of the current cursor position,

  the cursor is moved to the beginning of the line (i.e., as far to the left as possible).

- <span id="term-move-cursor-right"></span>`fn move_cursor_right(&self, n: usize) -> io::Result<()>`

  Move the cursor `n` characters to the right.

  

  If there are fewer than `n` characters to the right of the current cursor position,

  the cursor is moved to the end of the current line (i.e., as far to the right as possible).

- <span id="term-clear-line"></span>`fn clear_line(&self) -> io::Result<()>`

  Clear the current line.

  

  Position the cursor at the beginning of the current line.

- <span id="term-clear-last-lines"></span>`fn clear_last_lines(&self, n: usize) -> io::Result<()>`

  Clear the last `n` lines before the current line.

  

  Position the cursor at the beginning of the first line that was cleared.

- <span id="term-clear-screen"></span>`fn clear_screen(&self) -> io::Result<()>`

  Clear the entire screen.

  

  Move the cursor to the upper left corner of the screen.

- <span id="term-clear-to-end-of-screen"></span>`fn clear_to_end_of_screen(&self) -> io::Result<()>`

  Clear everything from the current cursor position to the end of the screen.

  The cursor stays in its position.

- <span id="term-clear-chars"></span>`fn clear_chars(&self, n: usize) -> io::Result<()>`

  Clear the last `n` characters of the current line.

- <span id="term-set-title"></span>`fn set_title<T: Display>(&self, title: T)`

  Set the terminal title.

- <span id="term-show-cursor"></span>`fn show_cursor(&self) -> io::Result<()>`

  Make the cursor visible again.

- <span id="term-hide-cursor"></span>`fn hide_cursor(&self) -> io::Result<()>`

  Hide the cursor.

- <span id="term-write-through"></span>`fn write_through(&self, bytes: &[u8]) -> io::Result<()>`

- <span id="term-write-through-common"></span>`fn write_through_common(&self, bytes: &[u8]) -> io::Result<()>`

#### Trait Implementations

##### `impl Any for Term`

- <span id="term-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRawFd for Term`

- <span id="term-asrawfd-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

##### `impl<T> Borrow for Term`

- <span id="term-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Term`

- <span id="term-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Term`

- <span id="term-clone"></span>`fn clone(&self) -> Term` — [`Term`](#term)

##### `impl CloneToUninit for Term`

- <span id="term-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Term`

- <span id="term-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Term`

- <span id="term-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Term`

- <span id="term-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Read for Term`

- <span id="term-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TermLike for console::Term`

##### `impl ToOwned for Term`

- <span id="term-toowned-type-owned"></span>`type Owned = T`

- <span id="term-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="term-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Term`

- <span id="term-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="term-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Term`

- <span id="term-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="term-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for Term`

- <span id="term-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="term-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

## Enums

### `TermTarget`

```rust
enum TermTarget {
    Stdout,
    Stderr,
    ReadWritePair(ReadWritePair),
}
```

*Defined in [`console-0.16.1/src/term.rs:33-38`](../../../.source_1765521767/console-0.16.1/src/term.rs#L33-L38)*

Where the term is writing.

#### Trait Implementations

##### `impl Any for TermTarget`

- <span id="termtarget-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TermTarget`

- <span id="termtarget-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TermTarget`

- <span id="termtarget-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TermTarget`

- <span id="termtarget-clone"></span>`fn clone(&self) -> TermTarget` — [`TermTarget`](#termtarget)

##### `impl CloneToUninit for TermTarget`

- <span id="termtarget-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TermTarget`

- <span id="termtarget-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TermTarget`

- <span id="termtarget-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TermTarget`

- <span id="termtarget-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for TermTarget`

- <span id="termtarget-toowned-type-owned"></span>`type Owned = T`

- <span id="termtarget-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="termtarget-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TermTarget`

- <span id="termtarget-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termtarget-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TermTarget`

- <span id="termtarget-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termtarget-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TermFamily`

```rust
enum TermFamily {
    File,
    UnixTerm,
    WindowsConsole,
    Dummy,
}
```

*Defined in [`console-0.16.1/src/term.rs:50-59`](../../../.source_1765521767/console-0.16.1/src/term.rs#L50-L59)*

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

##### `impl Any for TermFamily`

- <span id="termfamily-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TermFamily`

- <span id="termfamily-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TermFamily`

- <span id="termfamily-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TermFamily`

- <span id="termfamily-clone"></span>`fn clone(&self) -> TermFamily` — [`TermFamily`](#termfamily)

##### `impl CloneToUninit for TermFamily`

- <span id="termfamily-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for TermFamily`

##### `impl Debug for TermFamily`

- <span id="termfamily-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TermFamily`

##### `impl<T> From for TermFamily`

- <span id="termfamily-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TermFamily`

- <span id="termfamily-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TermFamily`

- <span id="termfamily-partialeq-eq"></span>`fn eq(&self, other: &TermFamily) -> bool` — [`TermFamily`](#termfamily)

##### `impl StructuralPartialEq for TermFamily`

##### `impl ToOwned for TermFamily`

- <span id="termfamily-toowned-type-owned"></span>`type Owned = T`

- <span id="termfamily-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="termfamily-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TermFamily`

- <span id="termfamily-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termfamily-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TermFamily`

- <span id="termfamily-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termfamily-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `TermWrite`

```rust
trait TermWrite: Write + Debug + AsRawFd + Send { ... }
```

*Defined in [`console-0.16.1/src/term.rs:13`](../../../.source_1765521767/console-0.16.1/src/term.rs#L13)*

#### Implementors

- `T`

### `TermRead`

```rust
trait TermRead: Read + Debug + AsRawFd + Send { ... }
```

*Defined in [`console-0.16.1/src/term.rs:18`](../../../.source_1765521767/console-0.16.1/src/term.rs#L18)*

#### Implementors

- `T`

## Functions

### `user_attended`

```rust
fn user_attended() -> bool
```

*Defined in [`console-0.16.1/src/term.rs:578-580`](../../../.source_1765521767/console-0.16.1/src/term.rs#L578-L580)*

A fast way to check if the application has a user attended for stdout.

This means that stdout is connected to a terminal instead of a
file or redirected by other means. This is a shortcut for
checking the `is_attended` feature on the stdout terminal.

### `user_attended_stderr`

```rust
fn user_attended_stderr() -> bool
```

*Defined in [`console-0.16.1/src/term.rs:588-590`](../../../.source_1765521767/console-0.16.1/src/term.rs#L588-L590)*

A fast way to check if the application has a user attended for stderr.

This means that stderr is connected to a terminal instead of a
file or redirected by other means. This is a shortcut for
checking the `is_attended` feature on the stderr terminal.

