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

```rust
fn test() -> Result<(), Box<dyn std::error::Error>> {
use std::thread;
use std::time::Duration;

use console::Term;

let term = Term::stdout();
term.write_line("Hello World!")?;
thread::sleep(Duration::from_millis(2000));
term.clear_line()?;
Ok(()) } test().unwrap();
```

# Colors and Styles

`console` automatically detects when to use colors based on the tty flag.  It also
provides higher level wrappers for styling text and other things that can be
displayed with the `style` function and utility types.

Example usage:

```rust
use console::style;

println!("This is {} neat", style("quite").cyan());
```

You can also store styles and apply them to text later:

```rust
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

## Modules

- [`common_term`](common_term/index.md) - 
- [`kb`](kb/index.md) - 
- [`term`](term/index.md) - 
- [`unix_term`](unix_term/index.md) - 
- [`utils`](utils/index.md) - 
- [`ansi`](ansi/index.md) - 

## Structs

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

- `fn with_inner(inner: TermInner) -> Term` — [`TermInner`](term/index.md), [`Term`](term/index.md)

- `fn stdout() -> Term` — [`Term`](term/index.md)

- `fn stderr() -> Term` — [`Term`](term/index.md)

- `fn buffered_stdout() -> Term` — [`Term`](term/index.md)

- `fn buffered_stderr() -> Term` — [`Term`](term/index.md)

- `fn read_write_pair<R, W>(read: R, write: W) -> Term` — [`Term`](term/index.md)

- `fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term` — [`Style`](utils/index.md), [`Term`](term/index.md)

- `fn style(self: &Self) -> Style` — [`Style`](utils/index.md)

- `fn target(self: &Self) -> TermTarget` — [`TermTarget`](term/index.md)

- `fn write_line(self: &Self, s: &str) -> io::Result<()>`

- `fn read_char(self: &Self) -> io::Result<char>`

- `fn read_key(self: &Self) -> io::Result<Key>` — [`Key`](kb/index.md)

- `fn read_key_raw(self: &Self) -> io::Result<Key>` — [`Key`](kb/index.md)

- `fn read_line(self: &Self) -> io::Result<String>`

- `fn read_line_initial_text(self: &Self, initial: &str) -> io::Result<String>`

- `fn read_secure_line(self: &Self) -> io::Result<String>`

- `fn flush(self: &Self) -> io::Result<()>`

- `fn is_term(self: &Self) -> bool`

- `fn features(self: &Self) -> TermFeatures<'_>` — [`TermFeatures`](term/index.md)

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

- `fn clone(self: &Self) -> Term` — [`Term`](term/index.md)

##### `impl Debug for Term`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Read for Term`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TermLike for console::Term`

##### `impl Write for Term`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

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

- `fn family(self: &Self) -> TermFamily` — [`TermFamily`](term/index.md)

#### Trait Implementations

##### `impl<'a> Clone for TermFeatures<'a>`

- `fn clone(self: &Self) -> TermFeatures<'a>` — [`TermFeatures`](term/index.md)

##### `impl<'a> Debug for TermFeatures<'a>`

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

- `fn new(emoji: &'a str, fallback: &'b str) -> Emoji<'a, 'b>` — [`Emoji`](utils/index.md)

#### Trait Implementations

##### `impl<'a, 'b> Clone for Emoji<'a, 'b>`

- `fn clone(self: &Self) -> Emoji<'a, 'b>` — [`Emoji`](utils/index.md)

##### `impl<'a, 'b> Copy for Emoji<'a, 'b>`

##### `impl Display for Emoji<'_, '_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Emoji<'a, 'b>`

- `fn to_string(self: &Self) -> String`

### `Style`

```rust
struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    fg_bright: bool,
    bg_bright: bool,
    attrs: Attributes,
    force: Option<bool>,
    for_stderr: bool,
}
```

A stored style that can be applied.

#### Implementations

- `const fn new() -> Self`

- `fn from_dotted_str(s: &str) -> Self`

- `fn apply_to<D>(self: &Self, val: D) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn force_styling(self: Self, value: bool) -> Self`

- `const fn for_stderr(self: Self) -> Self`

- `const fn for_stdout(self: Self) -> Self`

- `const fn fg(self: Self, color: Color) -> Self` — [`Color`](utils/index.md)

- `const fn bg(self: Self, color: Color) -> Self` — [`Color`](utils/index.md)

- `const fn attr(self: Self, attr: Attribute) -> Self` — [`Attribute`](utils/index.md)

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

##### `impl Clone for Style`

- `fn clone(self: &Self) -> Style` — [`Style`](utils/index.md)

##### `impl Debug for Style`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Style`

- `fn default() -> Self`

##### `impl Eq for Style`

##### `impl PartialEq for Style`

- `fn eq(self: &Self, other: &Style) -> bool` — [`Style`](utils/index.md)

##### `impl StructuralPartialEq for Style`

### `StyledObject<D>`

```rust
struct StyledObject<D> {
    style: Style,
    val: D,
}
```

A formatting wrapper that can be styled for a terminal.

#### Implementations

- `fn force_styling(self: Self, value: bool) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `fn for_stderr(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn for_stdout(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn fg(self: Self, color: Color) -> StyledObject<D>` — [`Color`](utils/index.md), [`StyledObject`](utils/index.md)

- `const fn bg(self: Self, color: Color) -> StyledObject<D>` — [`Color`](utils/index.md), [`StyledObject`](utils/index.md)

- `const fn attr(self: Self, attr: Attribute) -> StyledObject<D>` — [`Attribute`](utils/index.md), [`StyledObject`](utils/index.md)

- `const fn black(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn red(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn green(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn yellow(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn blue(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn magenta(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn cyan(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn white(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn color256(self: Self, color: u8) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn bright(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_black(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_red(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_green(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_yellow(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_blue(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_magenta(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_cyan(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_white(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_color256(self: Self, color: u8) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn on_bright(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn bold(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn dim(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn italic(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn underlined(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn blink(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn blink_fast(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn reverse(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn hidden(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

- `const fn strikethrough(self: Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

#### Trait Implementations

##### `impl<D: fmt::Binary> Binary for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: $crate::clone::Clone> Clone for StyledObject<D>`

- `fn clone(self: &Self) -> StyledObject<D>` — [`StyledObject`](utils/index.md)

##### `impl<D: fmt::Debug> Debug for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Display> Display for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerExp> LowerExp for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerHex> LowerHex for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Octal> Octal for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Pointer> Pointer for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for StyledObject<D>`

- `fn to_string(self: &Self) -> String`

##### `impl<D: fmt::UpperExp> UpperExp for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::UpperHex> UpperHex for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AnsiCodeIterator<'a>`

```rust
struct AnsiCodeIterator<'a> {
    s: &'a str,
    pending_item: Option<(&'a str, bool)>,
    last_idx: usize,
    cur_idx: usize,
    iter: Matches<'a>,
}
```

An iterator over ansi codes in a string.

This type can be used to scan over ansi codes in a string.
It yields tuples in the form `(s, is_ansi)` where `s` is a slice of
the original string and `is_ansi` indicates if the slice contains
ansi codes or string values.

#### Implementations

- `fn new(s: &'a str) -> AnsiCodeIterator<'a>` — [`AnsiCodeIterator`](ansi/index.md)

- `fn current_slice(self: &Self) -> &str`

- `fn rest_slice(self: &Self) -> &str`

#### Trait Implementations

##### `impl FusedIterator for AnsiCodeIterator<'_>`

##### `impl<I> IntoIterator for AnsiCodeIterator<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for AnsiCodeIterator<'a>`

- `type Item = (&'a str, bool)`

- `fn next(self: &mut Self) -> Option<(&'a str, bool)>`

### `WithoutAnsi<'a>`

```rust
struct WithoutAnsi<'a> {
    str: &'a str,
}
```

A wrapper struct that implements [`core::fmt::Display`](../miette_derive/fmt/index.md), only displaying non-ansi parts.

#### Implementations

- `fn new(str: &'a str) -> Self`

#### Trait Implementations

##### `impl Display for WithoutAnsi<'_>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for WithoutAnsi<'a>`

- `fn to_string(self: &Self) -> String`

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

##### `impl Clone for Key`

- `fn clone(self: &Self) -> Key` — [`Key`](kb/index.md)

##### `impl Debug for Key`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Key`

##### `impl Hash for Key`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Key`

- `fn eq(self: &Self, other: &Key) -> bool` — [`Key`](kb/index.md)

##### `impl StructuralPartialEq for Key`

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

- `fn clone(self: &Self) -> TermFamily` — [`TermFamily`](term/index.md)

##### `impl Copy for TermFamily`

##### `impl Debug for TermFamily`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TermFamily`

##### `impl PartialEq for TermFamily`

- `fn eq(self: &Self, other: &TermFamily) -> bool` — [`TermFamily`](term/index.md)

##### `impl StructuralPartialEq for TermFamily`

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

- `fn clone(self: &Self) -> TermTarget` — [`TermTarget`](term/index.md)

##### `impl Debug for TermTarget`

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

##### `impl Clone for Alignment`

- `fn clone(self: &Self) -> Alignment` — [`Alignment`](utils/index.md)

##### `impl Copy for Alignment`

##### `impl Debug for Alignment`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Alignment`

##### `impl PartialEq for Alignment`

- `fn eq(self: &Self, other: &Alignment) -> bool` — [`Alignment`](utils/index.md)

##### `impl StructuralPartialEq for Alignment`

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

#### Implementations

- `const MAP: [Attribute; 9]`

#### Trait Implementations

##### `impl Clone for Attribute`

- `fn clone(self: &Self) -> Attribute` — [`Attribute`](utils/index.md)

##### `impl Copy for Attribute`

##### `impl Debug for Attribute`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Attribute`

##### `impl Ord for Attribute`

- `fn cmp(self: &Self, other: &Attribute) -> $crate::cmp::Ordering` — [`Attribute`](utils/index.md)

##### `impl PartialEq for Attribute`

- `fn eq(self: &Self, other: &Attribute) -> bool` — [`Attribute`](utils/index.md)

##### `impl PartialOrd for Attribute`

- `fn partial_cmp(self: &Self, other: &Attribute) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Attribute`](utils/index.md)

##### `impl StructuralPartialEq for Attribute`

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

#### Implementations

- `fn ansi_num(self: Self) -> usize`

- `fn is_color256(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for Color`

- `fn clone(self: &Self) -> Color` — [`Color`](utils/index.md)

##### `impl Copy for Color`

##### `impl Debug for Color`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Color`

##### `impl PartialEq for Color`

- `fn eq(self: &Self, other: &Color) -> bool` — [`Color`](utils/index.md)

##### `impl StructuralPartialEq for Color`

## Functions

