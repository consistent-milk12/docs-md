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

## Contents

- [Modules](#modules)
  - [`common_term`](#common_term)
  - [`kb`](#kb)
  - [`term`](#term)
  - [`unix_term`](#unix_term)
  - [`utils`](#utils)
  - [`ansi`](#ansi)
- [Structs](#structs)
  - [`Term`](#term)
  - [`TermFeatures`](#termfeatures)
  - [`Emoji`](#emoji)
  - [`Style`](#style)
  - [`StyledObject`](#styledobject)
  - [`AnsiCodeIterator`](#ansicodeiterator)
  - [`WithoutAnsi`](#withoutansi)
- [Enums](#enums)
  - [`Key`](#key)
  - [`TermFamily`](#termfamily)
  - [`TermTarget`](#termtarget)
  - [`Alignment`](#alignment)
  - [`Attribute`](#attribute)
  - [`Color`](#color)
- [Functions](#functions)
  - [`user_attended`](#user_attended)
  - [`user_attended_stderr`](#user_attended_stderr)
  - [`colors_enabled`](#colors_enabled)
  - [`colors_enabled_stderr`](#colors_enabled_stderr)
  - [`measure_text_width`](#measure_text_width)
  - [`pad_str`](#pad_str)
  - [`pad_str_with`](#pad_str_with)
  - [`set_colors_enabled`](#set_colors_enabled)
  - [`set_colors_enabled_stderr`](#set_colors_enabled_stderr)
  - [`style`](#style)
  - [`truncate_str`](#truncate_str)
  - [`strip_ansi_codes`](#strip_ansi_codes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common_term`](#common_term) | mod |  |
| [`kb`](#kb) | mod |  |
| [`term`](#term) | mod |  |
| [`unix_term`](#unix_term) | mod |  |
| [`utils`](#utils) | mod |  |
| [`ansi`](#ansi) | mod |  |
| [`Term`](#term) | struct |  |
| [`TermFeatures`](#termfeatures) | struct |  |
| [`Emoji`](#emoji) | struct |  |
| [`Style`](#style) | struct |  |
| [`StyledObject`](#styledobject) | struct |  |
| [`AnsiCodeIterator`](#ansicodeiterator) | struct |  |
| [`WithoutAnsi`](#withoutansi) | struct |  |
| [`Key`](#key) | enum |  |
| [`TermFamily`](#termfamily) | enum |  |
| [`TermTarget`](#termtarget) | enum |  |
| [`Alignment`](#alignment) | enum |  |
| [`Attribute`](#attribute) | enum |  |
| [`Color`](#color) | enum |  |
| [`user_attended`](#user_attended) | fn |  |
| [`user_attended_stderr`](#user_attended_stderr) | fn |  |
| [`colors_enabled`](#colors_enabled) | fn |  |
| [`colors_enabled_stderr`](#colors_enabled_stderr) | fn |  |
| [`measure_text_width`](#measure_text_width) | fn |  |
| [`pad_str`](#pad_str) | fn |  |
| [`pad_str_with`](#pad_str_with) | fn |  |
| [`set_colors_enabled`](#set_colors_enabled) | fn |  |
| [`set_colors_enabled_stderr`](#set_colors_enabled_stderr) | fn |  |
| [`style`](#style) | fn |  |
| [`truncate_str`](#truncate_str) | fn |  |
| [`strip_ansi_codes`](#strip_ansi_codes) | fn |  |

## Modules

- [`common_term`](common_term/index.md)
- [`kb`](kb/index.md)
- [`term`](term/index.md)
- [`unix_term`](unix_term/index.md)
- [`utils`](utils/index.md)
- [`ansi`](ansi/index.md)

## Structs

### `Term`

```rust
struct Term {
    inner: alloc::sync::Arc<TermInner>,
    is_msys_tty: bool,
    is_tty: bool,
}
```

*Defined in [`console-0.16.1/src/term.rs:129-133`](../../.source_1765210505/console-0.16.1/src/term.rs#L129-L133)*

Abstraction around a terminal.

A terminal can be cloned.  If a buffer is used it's shared across all
clones which means it largely acts as a handle.

#### Implementations

- <span id="term-with-inner"></span>`fn with_inner(inner: TermInner) -> Term` — [`TermInner`](term/index.md#terminner), [`Term`](term/index.md#term)

- <span id="term-stdout"></span>`fn stdout() -> Term` — [`Term`](term/index.md#term)

- <span id="term-stderr"></span>`fn stderr() -> Term` — [`Term`](term/index.md#term)

- <span id="term-buffered-stdout"></span>`fn buffered_stdout() -> Term` — [`Term`](term/index.md#term)

- <span id="term-buffered-stderr"></span>`fn buffered_stderr() -> Term` — [`Term`](term/index.md#term)

- <span id="term-read-write-pair"></span>`fn read_write_pair<R, W>(read: R, write: W) -> Term` — [`Term`](term/index.md#term)

- <span id="term-read-write-pair-with-style"></span>`fn read_write_pair_with_style<R, W>(read: R, write: W, style: Style) -> Term` — [`Style`](utils/index.md#style), [`Term`](term/index.md#term)

- <span id="term-style"></span>`fn style(&self) -> Style` — [`Style`](utils/index.md#style)

- <span id="term-target"></span>`fn target(&self) -> TermTarget` — [`TermTarget`](term/index.md#termtarget)

- <span id="term-write-line"></span>`fn write_line(&self, s: &str) -> io::Result<()>`

- <span id="term-read-char"></span>`fn read_char(&self) -> io::Result<char>`

- <span id="term-read-key"></span>`fn read_key(&self) -> io::Result<Key>` — [`Key`](kb/index.md#key)

- <span id="term-read-key-raw"></span>`fn read_key_raw(&self) -> io::Result<Key>` — [`Key`](kb/index.md#key)

- <span id="term-read-line"></span>`fn read_line(&self) -> io::Result<String>`

- <span id="term-read-line-initial-text"></span>`fn read_line_initial_text(&self, initial: &str) -> io::Result<String>`

- <span id="term-read-secure-line"></span>`fn read_secure_line(&self) -> io::Result<String>`

- <span id="term-flush"></span>`fn flush(&self) -> io::Result<()>`

- <span id="term-is-term"></span>`fn is_term(&self) -> bool`

- <span id="term-features"></span>`fn features(&self) -> TermFeatures<'_>` — [`TermFeatures`](term/index.md#termfeatures)

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

- <span id="term-clone"></span>`fn clone(&self) -> Term` — [`Term`](term/index.md#term)

##### `impl Debug for Term`

- <span id="term-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Read for Term`

- <span id="term-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

##### `impl TermLike for console::Term`

##### `impl Write for Term`

- <span id="term-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="term-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `TermFeatures<'a>`

```rust
struct TermFeatures<'a>(&'a Term);
```

*Defined in [`console-0.16.1/src/term.rs:63`](../../.source_1765210505/console-0.16.1/src/term.rs#L63)*

Gives access to the terminal features.

#### Implementations

- <span id="termfeatures-is-attended"></span>`fn is_attended(&self) -> bool`

- <span id="termfeatures-colors-supported"></span>`fn colors_supported(&self) -> bool`

- <span id="termfeatures-is-msys-tty"></span>`fn is_msys_tty(&self) -> bool`

- <span id="termfeatures-wants-emoji"></span>`fn wants_emoji(&self) -> bool`

- <span id="termfeatures-family"></span>`fn family(&self) -> TermFamily` — [`TermFamily`](term/index.md#termfamily)

#### Trait Implementations

##### `impl Clone for TermFeatures<'a>`

- <span id="termfeatures-clone"></span>`fn clone(&self) -> TermFeatures<'a>` — [`TermFeatures`](term/index.md#termfeatures)

##### `impl Debug for TermFeatures<'a>`

- <span id="termfeatures-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Emoji<'a, 'b>`

```rust
struct Emoji<'a, 'b>(&'a str, &'b str);
```

*Defined in [`console-0.16.1/src/utils.rs:762`](../../.source_1765210505/console-0.16.1/src/utils.rs#L762)*

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

- <span id="emoji-new"></span>`fn new(emoji: &'a str, fallback: &'b str) -> Emoji<'a, 'b>` — [`Emoji`](utils/index.md#emoji)

#### Trait Implementations

##### `impl Clone for Emoji<'a, 'b>`

- <span id="emoji-clone"></span>`fn clone(&self) -> Emoji<'a, 'b>` — [`Emoji`](utils/index.md#emoji)

##### `impl Copy for Emoji<'a, 'b>`

##### `impl Display for Emoji<'_, '_>`

- <span id="emoji-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for Emoji<'a, 'b>`

- <span id="emoji-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`console-0.16.1/src/utils.rs:229-237`](../../.source_1765210505/console-0.16.1/src/utils.rs#L229-L237)*

A stored style that can be applied.

#### Implementations

- <span id="style-new"></span>`const fn new() -> Self`

- <span id="style-from-dotted-str"></span>`fn from_dotted_str(s: &str) -> Self`

- <span id="style-apply-to"></span>`fn apply_to<D>(&self, val: D) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="style-force-styling"></span>`const fn force_styling(self, value: bool) -> Self`

- <span id="style-for-stderr"></span>`const fn for_stderr(self) -> Self`

- <span id="style-for-stdout"></span>`const fn for_stdout(self) -> Self`

- <span id="style-fg"></span>`const fn fg(self, color: Color) -> Self` — [`Color`](utils/index.md#color)

- <span id="style-bg"></span>`const fn bg(self, color: Color) -> Self` — [`Color`](utils/index.md#color)

- <span id="style-attr"></span>`const fn attr(self, attr: Attribute) -> Self` — [`Attribute`](utils/index.md#attribute)

- <span id="style-black"></span>`const fn black(self) -> Self`

- <span id="style-red"></span>`const fn red(self) -> Self`

- <span id="style-green"></span>`const fn green(self) -> Self`

- <span id="style-yellow"></span>`const fn yellow(self) -> Self`

- <span id="style-blue"></span>`const fn blue(self) -> Self`

- <span id="style-magenta"></span>`const fn magenta(self) -> Self`

- <span id="style-cyan"></span>`const fn cyan(self) -> Self`

- <span id="style-white"></span>`const fn white(self) -> Self`

- <span id="style-color256"></span>`const fn color256(self, color: u8) -> Self`

- <span id="style-bright"></span>`const fn bright(self) -> Self`

- <span id="style-on-black"></span>`const fn on_black(self) -> Self`

- <span id="style-on-red"></span>`const fn on_red(self) -> Self`

- <span id="style-on-green"></span>`const fn on_green(self) -> Self`

- <span id="style-on-yellow"></span>`const fn on_yellow(self) -> Self`

- <span id="style-on-blue"></span>`const fn on_blue(self) -> Self`

- <span id="style-on-magenta"></span>`const fn on_magenta(self) -> Self`

- <span id="style-on-cyan"></span>`const fn on_cyan(self) -> Self`

- <span id="style-on-white"></span>`const fn on_white(self) -> Self`

- <span id="style-on-color256"></span>`const fn on_color256(self, color: u8) -> Self`

- <span id="style-on-bright"></span>`const fn on_bright(self) -> Self`

- <span id="style-bold"></span>`const fn bold(self) -> Self`

- <span id="style-dim"></span>`const fn dim(self) -> Self`

- <span id="style-italic"></span>`const fn italic(self) -> Self`

- <span id="style-underlined"></span>`const fn underlined(self) -> Self`

- <span id="style-blink"></span>`const fn blink(self) -> Self`

- <span id="style-blink-fast"></span>`const fn blink_fast(self) -> Self`

- <span id="style-reverse"></span>`const fn reverse(self) -> Self`

- <span id="style-hidden"></span>`const fn hidden(self) -> Self`

- <span id="style-strikethrough"></span>`const fn strikethrough(self) -> Self`

#### Trait Implementations

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](utils/index.md#style)

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Self`

##### `impl Eq for Style`

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](utils/index.md#style)

##### `impl StructuralPartialEq for Style`

### `StyledObject<D>`

```rust
struct StyledObject<D> {
    style: Style,
    val: D,
}
```

*Defined in [`console-0.16.1/src/utils.rs:515-518`](../../.source_1765210505/console-0.16.1/src/utils.rs#L515-L518)*

A formatting wrapper that can be styled for a terminal.

#### Implementations

- <span id="styledobject-force-styling"></span>`fn force_styling(self, value: bool) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-for-stderr"></span>`fn for_stderr(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-for-stdout"></span>`const fn for_stdout(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-fg"></span>`const fn fg(self, color: Color) -> StyledObject<D>` — [`Color`](utils/index.md#color), [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-bg"></span>`const fn bg(self, color: Color) -> StyledObject<D>` — [`Color`](utils/index.md#color), [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-attr"></span>`const fn attr(self, attr: Attribute) -> StyledObject<D>` — [`Attribute`](utils/index.md#attribute), [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-black"></span>`const fn black(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-red"></span>`const fn red(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-green"></span>`const fn green(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-yellow"></span>`const fn yellow(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-blue"></span>`const fn blue(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-magenta"></span>`const fn magenta(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-cyan"></span>`const fn cyan(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-white"></span>`const fn white(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-color256"></span>`const fn color256(self, color: u8) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-bright"></span>`const fn bright(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-black"></span>`const fn on_black(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-red"></span>`const fn on_red(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-green"></span>`const fn on_green(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-yellow"></span>`const fn on_yellow(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-blue"></span>`const fn on_blue(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-magenta"></span>`const fn on_magenta(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-cyan"></span>`const fn on_cyan(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-white"></span>`const fn on_white(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-color256"></span>`const fn on_color256(self, color: u8) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-on-bright"></span>`const fn on_bright(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-bold"></span>`const fn bold(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-dim"></span>`const fn dim(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-italic"></span>`const fn italic(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-underlined"></span>`const fn underlined(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-blink"></span>`const fn blink(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-blink-fast"></span>`const fn blink_fast(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-reverse"></span>`const fn reverse(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-hidden"></span>`const fn hidden(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

- <span id="styledobject-strikethrough"></span>`const fn strikethrough(self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

#### Trait Implementations

##### `impl<D: fmt::Binary> Binary for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: clone::Clone> Clone for StyledObject<D>`

- <span id="styledobject-clone"></span>`fn clone(&self) -> StyledObject<D>` — [`StyledObject`](utils/index.md#styledobject)

##### `impl<D: fmt::Debug> Debug for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Display> Display for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerExp> LowerExp for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerHex> LowerHex for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Octal> Octal for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Pointer> Pointer for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for StyledObject<D>`

- <span id="styledobject-to-string"></span>`fn to_string(&self) -> String`

##### `impl<D: fmt::UpperExp> UpperExp for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::UpperHex> UpperHex for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`console-0.16.1/src/ansi.rs:233-239`](../../.source_1765210505/console-0.16.1/src/ansi.rs#L233-L239)*

An iterator over ansi codes in a string.

This type can be used to scan over ansi codes in a string.
It yields tuples in the form `(s, is_ansi)` where `s` is a slice of
the original string and `is_ansi` indicates if the slice contains
ansi codes or string values.

#### Implementations

- <span id="ansicodeiterator-new"></span>`fn new(s: &'a str) -> AnsiCodeIterator<'a>` — [`AnsiCodeIterator`](ansi/index.md#ansicodeiterator)

- <span id="ansicodeiterator-current-slice"></span>`fn current_slice(&self) -> &str`

- <span id="ansicodeiterator-rest-slice"></span>`fn rest_slice(&self) -> &str`

#### Trait Implementations

##### `impl FusedIterator for AnsiCodeIterator<'_>`

##### `impl IntoIterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ansicodeiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="ansicodeiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-type-item"></span>`type Item = (&'a str, bool)`

- <span id="ansicodeiterator-next"></span>`fn next(&mut self) -> Option<(&'a str, bool)>`

### `WithoutAnsi<'a>`

```rust
struct WithoutAnsi<'a> {
    str: &'a str,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:206-208`](../../.source_1765210505/console-0.16.1/src/ansi.rs#L206-L208)*

A wrapper struct that implements [`core::fmt::Display`](../miette_derive/fmt/index.md), only displaying non-ansi parts.

#### Implementations

- <span id="withoutansi-new"></span>`fn new(str: &'a str) -> Self`

#### Trait Implementations

##### `impl Display for WithoutAnsi<'_>`

- <span id="withoutansi-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for WithoutAnsi<'a>`

- <span id="withoutansi-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`console-0.16.1/src/kb.rs:9-32`](../../.source_1765210505/console-0.16.1/src/kb.rs#L9-L32)*

Key mapping

This is an incomplete mapping of keys that are supported for reading
from the keyboard.

#### Variants

- **`UnknownEscSeq`**

  Unrecognized sequence containing Esc and a list of chars

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](kb/index.md#key)

##### `impl Debug for Key`

- <span id="key-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Key`

##### `impl Hash for Key`

- <span id="key-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Key`

- <span id="key-eq"></span>`fn eq(&self, other: &Key) -> bool` — [`Key`](kb/index.md#key)

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

*Defined in [`console-0.16.1/src/term.rs:50-59`](../../.source_1765210505/console-0.16.1/src/term.rs#L50-L59)*

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

- <span id="termfamily-clone"></span>`fn clone(&self) -> TermFamily` — [`TermFamily`](term/index.md#termfamily)

##### `impl Copy for TermFamily`

##### `impl Debug for TermFamily`

- <span id="termfamily-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TermFamily`

##### `impl PartialEq for TermFamily`

- <span id="termfamily-eq"></span>`fn eq(&self, other: &TermFamily) -> bool` — [`TermFamily`](term/index.md#termfamily)

##### `impl StructuralPartialEq for TermFamily`

### `TermTarget`

```rust
enum TermTarget {
    Stdout,
    Stderr,
    ReadWritePair(ReadWritePair),
}
```

*Defined in [`console-0.16.1/src/term.rs:33-38`](../../.source_1765210505/console-0.16.1/src/term.rs#L33-L38)*

Where the term is writing.

#### Trait Implementations

##### `impl Clone for TermTarget`

- <span id="termtarget-clone"></span>`fn clone(&self) -> TermTarget` — [`TermTarget`](term/index.md#termtarget)

##### `impl Debug for TermTarget`

- <span id="termtarget-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Alignment`

```rust
enum Alignment {
    Left,
    Center,
    Right,
}
```

*Defined in [`console-0.16.1/src/utils.rs:221-225`](../../.source_1765210505/console-0.16.1/src/utils.rs#L221-L225)*

Defines the alignment for padding operations.

#### Trait Implementations

##### `impl Clone for Alignment`

- <span id="alignment-clone"></span>`fn clone(&self) -> Alignment` — [`Alignment`](utils/index.md#alignment)

##### `impl Copy for Alignment`

##### `impl Debug for Alignment`

- <span id="alignment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Alignment`

##### `impl PartialEq for Alignment`

- <span id="alignment-eq"></span>`fn eq(&self, other: &Alignment) -> bool` — [`Alignment`](utils/index.md#alignment)

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

*Defined in [`console-0.16.1/src/utils.rs:128-140`](../../.source_1765210505/console-0.16.1/src/utils.rs#L128-L140)*

A terminal style attribute.

#### Implementations

- <span id="attribute-const-map"></span>`const MAP: [Attribute; 9]`

#### Trait Implementations

##### `impl Clone for Attribute`

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute` — [`Attribute`](utils/index.md#attribute)

##### `impl Copy for Attribute`

##### `impl Debug for Attribute`

- <span id="attribute-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Attribute`

##### `impl Ord for Attribute`

- <span id="attribute-cmp"></span>`fn cmp(&self, other: &Attribute) -> cmp::Ordering` — [`Attribute`](utils/index.md#attribute)

##### `impl PartialEq for Attribute`

- <span id="attribute-eq"></span>`fn eq(&self, other: &Attribute) -> bool` — [`Attribute`](utils/index.md#attribute)

##### `impl PartialOrd for Attribute`

- <span id="attribute-partial-cmp"></span>`fn partial_cmp(&self, other: &Attribute) -> option::Option<cmp::Ordering>` — [`Attribute`](utils/index.md#attribute)

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

*Defined in [`console-0.16.1/src/utils.rs:87-97`](../../.source_1765210505/console-0.16.1/src/utils.rs#L87-L97)*

A terminal color.

#### Implementations

- <span id="color-ansi-num"></span>`fn ansi_num(self) -> usize`

- <span id="color-is-color256"></span>`fn is_color256(self) -> bool`

#### Trait Implementations

##### `impl Clone for Color`

- <span id="color-clone"></span>`fn clone(&self) -> Color` — [`Color`](utils/index.md#color)

##### `impl Copy for Color`

##### `impl Debug for Color`

- <span id="color-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Color`

##### `impl PartialEq for Color`

- <span id="color-eq"></span>`fn eq(&self, other: &Color) -> bool` — [`Color`](utils/index.md#color)

##### `impl StructuralPartialEq for Color`

## Functions

*Defined in [`console-0.16.1/src/lib.rs:92`](../../.source_1765210505/console-0.16.1/src/lib.rs#L92)*

*Defined in [`console-0.16.1/src/lib.rs:92`](../../.source_1765210505/console-0.16.1/src/lib.rs#L92)*

*Defined in [`console-0.16.1/src/lib.rs:96`](../../.source_1765210505/console-0.16.1/src/lib.rs#L96)*

*Defined in [`console-0.16.1/src/lib.rs:96`](../../.source_1765210505/console-0.16.1/src/lib.rs#L96)*

*Defined in [`console-0.16.1/src/lib.rs:96`](../../.source_1765210505/console-0.16.1/src/lib.rs#L96)*

*Defined in [`console-0.16.1/src/lib.rs:96`](../../.source_1765210505/console-0.16.1/src/lib.rs#L96)*

*Defined in [`console-0.16.1/src/lib.rs:96`](../../.source_1765210505/console-0.16.1/src/lib.rs#L96)*

*Defined in [`console-0.16.1/src/lib.rs:97`](../../.source_1765210505/console-0.16.1/src/lib.rs#L97)*

*Defined in [`console-0.16.1/src/lib.rs:97`](../../.source_1765210505/console-0.16.1/src/lib.rs#L97)*

*Defined in [`console-0.16.1/src/lib.rs:97`](../../.source_1765210505/console-0.16.1/src/lib.rs#L97)*

*Defined in [`console-0.16.1/src/lib.rs:97`](../../.source_1765210505/console-0.16.1/src/lib.rs#L97)*

*Defined in [`console-0.16.1/src/lib.rs:102`](../../.source_1765210505/console-0.16.1/src/lib.rs#L102)*

