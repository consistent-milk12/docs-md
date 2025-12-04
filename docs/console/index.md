# Crate `console`

console is a library for Rust that provides access to various terminal
features so you can build nicer looking command line interfaces.  It
comes with various tools and utilities for working with Terminals and
formatting text.

Best paired with other libraries in the family:

* [dialoguer](https://docs.rs/dialoguer)
* [indicatif](https://docs.rs/indicatif)

# Terminal Access

The terminal is abstracted through the `console::Term` type.  It can
either directly provide access to the connected terminal or by buffering
up commands.  A buffered terminal will however not be completely buffered
on windows where cursor movements are currently directly passed through.

Example usage:

```
# fn test() -> Result<(), Box<dyn std::error::Error>> {
use std::thread;
use std::time::Duration;

use console::Term;

let term = Term::stdout();
term.write_line("Hello World!")?;
thread::sleep(Duration::from_millis(2000));
term.clear_line()?;
# Ok(()) } test().unwrap();
```

# Colors and Styles

`console` automatically detects when to use colors based on the tty flag.  It also
provides higher level wrappers for styling text and other things that can be
displayed with the `style` function and utility types.

Example usage:

```
use console::style;

println!("This is {} neat", style("quite").cyan());
```

You can also store styles and apply them to text later:

```
use console::Style;

let cyan = Style::new().cyan();
println!("This is {} neat", cyan.apply_to("quite"));
```

# Working with ANSI Codes

The crate provides the function `strip_ansi_codes` to remove ANSI codes
from a string as well as `measure_text_width` to calculate the width of a
string as it would be displayed by the terminal.  Both of those together
are useful for more complex formatting.

# Unicode Width Support

By default this crate depends on the `unicode-width` crate to calculate
the width of terminal characters.  If you do not need this you can disable
the `unicode-width` feature which will cut down on dependencies.

# Features

By default all features are enabled.  The following features exist:

* `unicode-width`: adds support for unicode width calculations
* `ansi-parsing`: adds support for parsing ansi codes (this adds support
  for stripping and taking ansi escape codes into account for length
  calculations).

## Structs

### `Term`

```rust
struct Term {
    // [REDACTED: Private Fields]
}
```

Abstraction around a terminal.

A terminal can be cloned.  If a buffer is used it's shared across all
clones which means it largely acts as a handle.

#### Implementations

- `fn stdout() -> Term`
  Return a new unbuffered terminal.

- `fn stderr() -> Term`
  Return a new unbuffered terminal to stderr.

- `fn buffered_stdout() -> Term`
  Return a new buffered terminal.

- `fn buffered_stderr() -> Term`
  Return a new buffered terminal to stderr.

- `fn read_write_pair<R, W>(read: R, write: W) -> Term`
  Return a terminal for the given Read/Write pair styled like stderr.

- `fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term`
  Return a terminal for the given Read/Write pair.

- `fn style(self: &Self) -> Style`
  Return the style for this terminal.

- `fn target(self: &Self) -> TermTarget`
  Return the target of this terminal.

- `fn write_line(self: &Self, s: &str) -> io::Result<()>`
  Write a string to the terminal and add a newline.

- `fn read_char(self: &Self) -> io::Result<char>`
  Read a single character from the terminal.

- `fn read_key(self: &Self) -> io::Result<Key>`
  Read a single key from the terminal.

- `fn read_key_raw(self: &Self) -> io::Result<Key>`

- `fn read_line(self: &Self) -> io::Result<String>`
  Read one line of input.

- `fn read_line_initial_text(self: &Self, initial: &str) -> io::Result<String>`
  Read one line of input with initial text.

- `fn read_secure_line(self: &Self) -> io::Result<String>`
  Read a line of input securely.

- `fn flush(self: &Self) -> io::Result<()>`
  Flush internal buffers.

- `fn is_term(self: &Self) -> bool`
  Check if the terminal is indeed a terminal.

- `fn features(self: &Self) -> TermFeatures<'_>`
  Check for common terminal features.

- `fn size(self: &Self) -> (u16, u16)`
  Return the terminal size in rows and columns or gets sensible defaults.

- `fn size_checked(self: &Self) -> Option<(u16, u16)>`
  Return the terminal size in rows and columns.

- `fn move_cursor_to(self: &Self, x: usize, y: usize) -> io::Result<()>`
  Move the cursor to row `x` and column `y`. Values are 0-based.

- `fn move_cursor_up(self: &Self, n: usize) -> io::Result<()>`
  Move the cursor up by `n` lines, if possible.

- `fn move_cursor_down(self: &Self, n: usize) -> io::Result<()>`
  Move the cursor down by `n` lines, if possible.

- `fn move_cursor_left(self: &Self, n: usize) -> io::Result<()>`
  Move the cursor `n` characters to the left, if possible.

- `fn move_cursor_right(self: &Self, n: usize) -> io::Result<()>`
  Move the cursor `n` characters to the right.

- `fn clear_line(self: &Self) -> io::Result<()>`
  Clear the current line.

- `fn clear_last_lines(self: &Self, n: usize) -> io::Result<()>`
  Clear the last `n` lines before the current line.

- `fn clear_screen(self: &Self) -> io::Result<()>`
  Clear the entire screen.

- `fn clear_to_end_of_screen(self: &Self) -> io::Result<()>`
  Clear everything from the current cursor position to the end of the screen.

- `fn clear_chars(self: &Self, n: usize) -> io::Result<()>`
  Clear the last `n` characters of the current line.

- `fn set_title<T: Display>(self: &Self, title: T)`
  Set the terminal title.

- `fn show_cursor(self: &Self) -> io::Result<()>`
  Make the cursor visible again.

- `fn hide_cursor(self: &Self) -> io::Result<()>`
  Hide the cursor.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRawFd`

- `fn as_raw_fd(self: &Self) -> RawFd`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Term`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Read`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl TermLike`

### `TermFeatures<'a>`

```rust
struct TermFeatures<'a>();
```

Gives access to the terminal features.

#### Implementations

- `fn is_attended(self: &Self) -> bool`
  Check if this is a real user attended terminal (`isatty`)

- `fn colors_supported(self: &Self) -> bool`
  Check if colors are supported by this terminal.

- `fn is_msys_tty(self: &Self) -> bool`
  Check if this terminal is an msys terminal.

- `fn wants_emoji(self: &Self) -> bool`
  Check if this terminal wants emojis.

- `fn family(self: &Self) -> TermFamily`
  Return the family of the terminal.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> TermFeatures<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Emoji<'a, 'b>`

```rust
struct Emoji<'a, 'b>(&'a str, &'b str);
```

"Intelligent" emoji formatter.

This struct intelligently wraps an emoji so that it is rendered
only on systems that want emojis and renders a fallback on others.

Example:

```rust
use console::Emoji;
println!("[3/4] {}Downloading ...", Emoji("🚚 ", ""));
println!("[4/4] {} Done!", Emoji("✨", ":-)"));
```

#### Implementations

- `fn new(emoji: &'a str, fallback: &'b str) -> Emoji<'a, 'b>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a, 'b>`

- `fn clone(self: &Self) -> Emoji<'a, 'b>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a, 'b>`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Style`

```rust
struct Style {
    // [REDACTED: Private Fields]
}
```

A stored style that can be applied.

#### Implementations

- `const fn new() -> Self`
  Returns an empty default style.

- `fn from_dotted_str(s: &str) -> Self`
  Creates a style from a dotted string.

- `fn apply_to<D>(self: &Self, val: D) -> StyledObject<D>`
  Apply the style to something that can be displayed.

- `const fn force_styling(self: Self, value: bool) -> Self`
  Forces styling on or off.

- `const fn for_stderr(self: Self) -> Self`
  Specifies that style is applying to something being written on stderr.

- `const fn for_stdout(self: Self) -> Self`
  Specifies that style is applying to something being written on stdout.

- `const fn fg(self: Self, color: Color) -> Self`
  Sets a foreground color.

- `const fn bg(self: Self, color: Color) -> Self`
  Sets a background color.

- `const fn attr(self: Self, attr: Attribute) -> Self`
  Adds a attr.

- `const fn black(self: Self) -> Self`

- `const fn red(self: Self) -> Self`

- `const fn green(self: Self) -> Self`

- `const fn yellow(self: Self) -> Self`

- `const fn blue(self: Self) -> Self`

- `const fn magenta(self: Self) -> Self`

- `const fn cyan(self: Self) -> Self`

- `const fn white(self: Self) -> Self`

- `const fn color256(self: Self, color: u8) -> Self`

- `const fn bright(self: Self) -> Self`

- `const fn on_black(self: Self) -> Self`

- `const fn on_red(self: Self) -> Self`

- `const fn on_green(self: Self) -> Self`

- `const fn on_yellow(self: Self) -> Self`

- `const fn on_blue(self: Self) -> Self`

- `const fn on_magenta(self: Self) -> Self`

- `const fn on_cyan(self: Self) -> Self`

- `const fn on_white(self: Self) -> Self`

- `const fn on_color256(self: Self, color: u8) -> Self`

- `const fn on_bright(self: Self) -> Self`

- `const fn bold(self: Self) -> Self`

- `const fn dim(self: Self) -> Self`

- `const fn italic(self: Self) -> Self`

- `const fn underlined(self: Self) -> Self`

- `const fn blink(self: Self) -> Self`

- `const fn blink_fast(self: Self) -> Self`

- `const fn reverse(self: Self) -> Self`

- `const fn hidden(self: Self) -> Self`

- `const fn strikethrough(self: Self) -> Self`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Style`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Style) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `StyledObject<D>`

```rust
struct StyledObject<D> {
    // [REDACTED: Private Fields]
}
```

A formatting wrapper that can be styled for a terminal.

#### Implementations

- `fn force_styling(self: Self, value: bool) -> StyledObject<D>`
  Forces styling on or off.

- `fn for_stderr(self: Self) -> StyledObject<D>`
  Specifies that style is applying to something being written on stderr

- `const fn for_stdout(self: Self) -> StyledObject<D>`
  Specifies that style is applying to something being written on stdout

- `const fn fg(self: Self, color: Color) -> StyledObject<D>`
  Sets a foreground color.

- `const fn bg(self: Self, color: Color) -> StyledObject<D>`
  Sets a background color.

- `const fn attr(self: Self, attr: Attribute) -> StyledObject<D>`
  Adds a attr.

- `const fn black(self: Self) -> StyledObject<D>`

- `const fn red(self: Self) -> StyledObject<D>`

- `const fn green(self: Self) -> StyledObject<D>`

- `const fn yellow(self: Self) -> StyledObject<D>`

- `const fn blue(self: Self) -> StyledObject<D>`

- `const fn magenta(self: Self) -> StyledObject<D>`

- `const fn cyan(self: Self) -> StyledObject<D>`

- `const fn white(self: Self) -> StyledObject<D>`

- `const fn color256(self: Self, color: u8) -> StyledObject<D>`

- `const fn bright(self: Self) -> StyledObject<D>`

- `const fn on_black(self: Self) -> StyledObject<D>`

- `const fn on_red(self: Self) -> StyledObject<D>`

- `const fn on_green(self: Self) -> StyledObject<D>`

- `const fn on_yellow(self: Self) -> StyledObject<D>`

- `const fn on_blue(self: Self) -> StyledObject<D>`

- `const fn on_magenta(self: Self) -> StyledObject<D>`

- `const fn on_cyan(self: Self) -> StyledObject<D>`

- `const fn on_white(self: Self) -> StyledObject<D>`

- `const fn on_color256(self: Self, color: u8) -> StyledObject<D>`

- `const fn on_bright(self: Self) -> StyledObject<D>`

- `const fn bold(self: Self) -> StyledObject<D>`

- `const fn dim(self: Self) -> StyledObject<D>`

- `const fn italic(self: Self) -> StyledObject<D>`

- `const fn underlined(self: Self) -> StyledObject<D>`

- `const fn blink(self: Self) -> StyledObject<D>`

- `const fn blink_fast(self: Self) -> StyledObject<D>`

- `const fn reverse(self: Self) -> StyledObject<D>`

- `const fn hidden(self: Self) -> StyledObject<D>`

- `const fn strikethrough(self: Self) -> StyledObject<D>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<D: fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<D: $crate::clone::Clone>`

- `fn clone(self: &Self) -> StyledObject<D>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display<D: fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<D: fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<D: fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<D: fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointer<D: fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<D: fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<D: fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<D: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AnsiCodeIterator<'a>`

```rust
struct AnsiCodeIterator<'a> {
    // [REDACTED: Private Fields]
}
```

An iterator over ansi codes in a string.

This type can be used to scan over ansi codes in a string.
It yields tuples in the form `(s, is_ansi)` where `s` is a slice of
the original string and `is_ansi` indicates if the slice contains
ansi codes or string values.

#### Implementations

- `fn new(s: &'a str) -> AnsiCodeIterator<'a>`
  Creates a new ansi code iterator.

- `fn current_slice(self: &Self) -> &str`
  Returns the string slice up to the current match.

- `fn rest_slice(self: &Self) -> &str`
  Returns the string slice from the current match to the end.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator`

##### `impl Iterator<'a>`

- `type Item = (&'a str, bool)`

- `fn next(self: &mut Self) -> Option<(&'a str, bool)>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `WithoutAnsi<'a>`

```rust
struct WithoutAnsi<'a> {
    // [REDACTED: Private Fields]
}
```

A wrapper struct that implements [`core::fmt::Display`](#display), only displaying non-ansi parts.

#### Implementations

- `fn new(str: &'a str) -> Self`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Key`

```rust
enum Key {
    Unknown,
    UnknownEscSeq(alloc::vec::Vec<char>),
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Enter,
    Escape,
    Backspace,
    Home,
    End,
    Tab,
    BackTab,
    Alt,
    Del,
    Shift,
    Insert,
    PageUp,
    PageDown,
    Char(char),
    CtrlC,
}
```

Key mapping

This is an incomplete mapping of keys that are supported for reading
from the keyboard.

#### Variants

- **`UnknownEscSeq`**

  Unrecognized sequence containing Esc and a list of chars

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Key`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Key) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> TermFamily`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &TermFamily) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> TermTarget`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Alignment`

```rust
enum Alignment {
    Left,
    Center,
    Right,
}
```

Defines the alignment for padding operations.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Alignment`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Alignment) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Attribute`

```rust
enum Attribute {
    Bold,
    Dim,
    Italic,
    Underlined,
    Blink,
    BlinkFast,
    Reverse,
    Hidden,
    StrikeThrough,
}
```

A terminal style attribute.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Attribute`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Attribute) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Attribute) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Attribute) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Color`

```rust
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Color256(u8),
}
```

A terminal color.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Color`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Color) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

