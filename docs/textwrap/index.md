# Crate `textwrap`

The textwrap library provides functions for word wrapping and
indenting text.

# Wrapping Text

Wrapping text can be very useful in command-line programs where
you want to format dynamic output nicely so it looks good in a
terminal. A quick example:

```
# #[cfg(feature = "smawk")] {
let text = "textwrap: a small library for wrapping text.";
assert_eq!(textwrap::wrap(text, 18),
           vec!["textwrap: a",
                "small library for",
                "wrapping text."]);
# }
```

The [`wrap()`](#wrap) function returns the individual lines, use
[`fill()`](#fill) is you want the lines joined with `'\n'` to form a
`String`.

If you enable the `hyphenation` Cargo feature, you can get
automatic hyphenation for a number of languages:

```
#[cfg(feature = "hyphenation")] {
use hyphenation::{Language, Load, Standard};
use textwrap::{wrap, Options, WordSplitter};

let text = "textwrap: a small library for wrapping text.";
let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
let options = Options::new(18).word_splitter(WordSplitter::Hyphenation(dictionary));
assert_eq!(wrap(text, &options),
           vec!["textwrap: a small",
                "library for wrap-",
                "ping text."]);
}
```

See also the [`unfill()`](#unfill) and [`refill()`](#refill) functions which allow
you to manipulate already wrapped text.

## Wrapping Strings at Compile Time

If your strings are known at compile time, please take a look at
the procedural macros from the [textwrap-macros] crate.

## Displayed Width vs Byte Size

To word wrap text, one must know the width of each word so one can
know when to break lines. This library will by default measure the
width of text using the _displayed width_, not the size in bytes.
The `unicode-width` Cargo feature controls this.

This is important for non-ASCII text. ASCII characters such as `a`
and `!` are simple and take up one column each. This means that
the displayed width is equal to the string length in bytes.
However, non-ASCII characters and symbols take up more than one
byte when UTF-8 encoded: `é` is `0xc3 0xa9` (two bytes) and `⚙` is
`0xe2 0x9a 0x99` (three bytes) in UTF-8, respectively.

This is why we take care to use the displayed width instead of the
byte count when computing line lengths. All functions in this
library handle Unicode characters like this when the
`unicode-width` Cargo feature is enabled (it is enabled by
default).

# Indentation and Dedentation

The textwrap library also offers functions for adding a prefix to
every line of a string and to remove leading whitespace. As an
example, [`indent()`](#indent) allows you to turn lines of text into a
bullet list:

```
let before = "\
foo
bar
baz
";
let after = "\
* foo
* bar
* baz
";
assert_eq!(textwrap::indent(before, "* "), after);
```

Removing leading whitespace is done with [`dedent()`](#dedent):

```
let before = "
    Some
      indented
        text
";
let after = "
Some
  indented
    text
";
assert_eq!(textwrap::dedent(before), after);
```

# Cargo Features

The textwrap library can be slimmed down as needed via a number of
Cargo features. This means you only pay for the features you
actually use.

The full dependency graph, where dashed lines indicate optional
dependencies, is shown below:

<img src="https://raw.githubusercontent.com/mgeisler/textwrap/master/images/textwrap-0.16.2.svg">

## Default Features

These features are enabled by default:

* `unicode-linebreak`: enables finding words using the
  [unicode-linebreak] crate, which implements the line breaking
  algorithm described in [Unicode Standard Annex
  #14](https://www.unicode.org/reports/tr14/).

  This feature can be disabled if you are happy to find words
  separated by ASCII space characters only. People wrapping text
  with emojis or East-Asian characters will want most likely want
  to enable this feature. See [`WordSeparator`](#wordseparator) for details.

* `unicode-width`: enables correct width computation of non-ASCII
  characters via the [unicode-width] crate. Without this feature,
  every [`char`](#char) is 1 column wide, except for emojis which are 2
  columns wide. See [`core::display_width()`](#display-width) for details.

  This feature can be disabled if you only need to wrap ASCII
  text, or if the functions in [`core`](textwrap/core/index.md) are used directly with
  [`core::Fragment`](#fragment)s for which the widths have been computed in
  other ways.

* `smawk`: enables linear-time wrapping of the whole paragraph via
  the [smawk](#smawk)
 crate. See [`wrap_algorithms::wrap_optimal_fit()`](#wrap-optimal-fit)
  for details on the optimal-fit algorithm.

  This feature can be disabled if you only ever intend to use
  [`wrap_algorithms::wrap_first_fit()`](#wrap-first-fit).

<!-- begin binary-sizes -->

With Rust 1.64.0, the size impact of the above features on your
binary is as follows:

| Configuration                            |  Binary Size |    Delta |
| :---                                     |         ---: |     ---: |
| quick-and-dirty implementation           |       289 KB |     — KB |
| textwrap without default features        |       305 KB |    16 KB |
| textwrap with smawk                      |       317 KB |    28 KB |
| textwrap with unicode-width              |       309 KB |    20 KB |
| textwrap with unicode-linebreak          |       342 KB |    53 KB |

<!-- end binary-sizes -->

The above sizes are the stripped sizes and the binary is compiled
in release mode with this profile:

```toml
[profile.release]
lto = true
codegen-units = 1
```

See the [binary-sizes demo] if you want to reproduce these
results.

## Optional Features

These Cargo features enable new functionality:

* `terminal_size`: enables automatic detection of the terminal
  width via the [terminal_size](#terminal-size)
 crate. See
  [`Options::with_termwidth()`](#with-termwidth) for details.

* `hyphenation`: enables language-sensitive hyphenation via the
  [hyphenation](#hyphenation)
 crate. See the [`word_splitters::WordSplitter`](#wordsplitter)
  trait for details.

[unicode-linebreak]: https://docs.rs/unicode-linebreak/
[unicode-width]: https://docs.rs/unicode-width/
[smawk](#smawk)
: https://docs.rs/smawk/
[binary-sizes demo]: https://github.com/mgeisler/textwrap/tree/master/examples/binary-sizes
[textwrap-macros]: https://docs.rs/textwrap-macros/
[terminal_size](#terminal-size)
: https://docs.rs/terminal_size/
[hyphenation](#hyphenation)
: https://docs.rs/hyphenation/

## Modules

- [`core`](core/index.md) - Building blocks for advanced wrapping functionality.
- [`word_splitters`](word_splitters/index.md) - Word splitting functionality.
- [`wrap_algorithms`](wrap_algorithms/index.md) - Word wrapping algorithms.

## Structs

### `Options<'a>`

```rust
struct Options<'a> {
    pub width: usize,
    pub line_ending: crate::LineEnding,
    pub initial_indent: &'a str,
    pub subsequent_indent: &'a str,
    pub break_words: bool,
    pub wrap_algorithm: crate::WrapAlgorithm,
    pub word_separator: crate::WordSeparator,
    pub word_splitter: crate::WordSplitter,
}
```

Holds configuration options for wrapping and filling text.

#### Fields

- **`width`**: `usize`

  The width in columns at which the text will be wrapped.

- **`line_ending`**: `crate::LineEnding`

  Line ending used for breaking lines.

- **`initial_indent`**: `&'a str`

  Indentation used for the first line of output. See the
  [`Options::initial_indent`](#initial-indent) method.

- **`subsequent_indent`**: `&'a str`

  Indentation used for subsequent lines of output. See the
  [`Options::subsequent_indent`](#subsequent-indent) method.

- **`break_words`**: `bool`

  Allow long words to be broken if they cannot fit on a line.
  When set to `false`, some lines may be longer than
  `self.width`. See the [`Options::break_words`](#break-words) method.

- **`wrap_algorithm`**: `crate::WrapAlgorithm`

  Wrapping algorithm to use, see the implementations of the
  [`WrapAlgorithm`](textwrap/wrap_algorithms/index.md) trait for details.

- **`word_separator`**: `crate::WordSeparator`

  The line breaking algorithm to use, see the [`WordSeparator`](#wordseparator)
  trait for an overview and possible implementations.

- **`word_splitter`**: `crate::WordSplitter`

  The method for splitting words. This can be used to prohibit
  splitting words on hyphens, or it can be used to implement
  language-aware machine hyphenation.

#### Implementations

- `const fn new(width: usize) -> Self`
  Creates a new [`Options`] with the specified width.

- `fn line_ending(self: Self, line_ending: LineEnding) -> Self`
  Change [`self.line_ending`]. This specifies which of the

- `fn width(self: Self, width: usize) -> Self`
  Set [`self.width`] to the given value.

- `fn initial_indent(self: Self, initial_indent: &'a str) -> Self`
  Change [`self.initial_indent`]. The initial indentation is

- `fn subsequent_indent(self: Self, subsequent_indent: &'a str) -> Self`
  Change [`self.subsequent_indent`]. The subsequent indentation

- `fn break_words(self: Self, break_words: bool) -> Self`
  Change [`self.break_words`]. This controls if words longer

- `fn word_separator(self: Self, word_separator: WordSeparator) -> Options<'a>`
  Change [`self.word_separator`].

- `fn wrap_algorithm(self: Self, wrap_algorithm: WrapAlgorithm) -> Options<'a>`
  Change [`self.wrap_algorithm`].

- `fn word_splitter(self: Self, word_splitter: WordSplitter) -> Options<'a>`
  Change [`self.word_splitter`]. The [`WordSplitter`] is used to

#### Trait Implementations

##### `impl From`

- `fn from(width: usize) -> Self`

##### `impl From<'a>`

- `fn from(options: &'a Options<'a>) -> Self`

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

- `fn clone(self: &Self) -> Options<'a>`

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

## Enums

### `LineEnding`

```rust
enum LineEnding {
    CRLF,
    LF,
}
```

Supported line endings. Like in the Rust standard library, two line
endings are supported: `\r\n` and `\n`

#### Variants

- **`CRLF`**

  _Carriage return and line feed_ – a line ending sequence
  historically used in Windows. Corresponds to the sequence
  of ASCII control characters `0x0D 0x0A` or `\r\n`

- **`LF`**

  _Line feed_ – a line ending historically used in Unix.
   Corresponds to the ASCII control character `0x0A` or `\n`

#### Implementations

- `const fn as_str(self: &Self) -> &'static str`
  Turns this [`LineEnding`] value into its ASCII representation.

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

- `fn clone(self: &Self) -> LineEnding`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LineEnding) -> bool`

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

### `WordSeparator`

```rust
enum WordSeparator {
    AsciiSpace,
    UnicodeBreakProperties,
    Custom(fn(&str) -> Box<dyn Iterator<Item = crate::core::Word<'_>>>),
}
```

Describes where words occur in a line of text.

The simplest approach is say that words are separated by one or
more ASCII spaces (`' '`). This works for Western languages
without emojis. A more complex approach is to use the Unicode line
breaking algorithm, which finds break points in non-ASCII text.

The line breaks occur between words, please see
[`WordSplitter`](crate::WordSplitter) for options of how to handle
hyphenation of individual words.

# Examples

```
use textwrap::core::Word;
use textwrap::WordSeparator::AsciiSpace;

let words = AsciiSpace.find_words("Hello World!").collect::<Vec<_>>();
assert_eq!(words, vec![Word::from("Hello "), Word::from("World!")]);
```

#### Variants

- **`AsciiSpace`**

  Find words by splitting on runs of `' '` characters.
  
  # Examples
  
  ```
  use textwrap::core::Word;
  use textwrap::WordSeparator::AsciiSpace;
  
  let words = AsciiSpace.find_words("Hello   World!").collect::<Vec<_>>();
  assert_eq!(words, vec![Word::from("Hello   "),
                         Word::from("World!")]);
  ```

- **`UnicodeBreakProperties`**

  Split `line` into words using Unicode break properties.
  
  This word separator uses the Unicode line breaking algorithm
  described in [Unicode Standard Annex
  #14](https://www.unicode.org/reports/tr14/) to find legal places
  to break lines. There is a small difference in that the U+002D
  (Hyphen-Minus) and U+00AD (Soft Hyphen) don’t create a line break:
  to allow a line break at a hyphen, use
  [`WordSplitter::HyphenSplitter`](crate::WordSplitter::HyphenSplitter).
  Soft hyphens are not currently supported.
  
  # Examples
  
  Unlike [`WordSeparator::AsciiSpace`](#asciispace), the Unicode line
  breaking algorithm will find line break opportunities between
  some characters with no intervening whitespace:
  
  ```
  #[cfg(feature = "unicode-linebreak")] {
  use textwrap::core::Word;
  use textwrap::WordSeparator::UnicodeBreakProperties;
  
  assert_eq!(UnicodeBreakProperties.find_words("Emojis: 😂😍").collect::<Vec<_>>(),
             vec![Word::from("Emojis: "),
                  Word::from("😂"),
                  Word::from("😍")]);
  
  assert_eq!(UnicodeBreakProperties.find_words("CJK: 你好").collect::<Vec<_>>(),
             vec![Word::from("CJK: "),
                  Word::from("你"),
                  Word::from("好")]);
  }
  ```
  
  A U+2060 (Word Joiner) character can be inserted if you want to
  manually override the defaults and keep the characters together:
  
  ```
  #[cfg(feature = "unicode-linebreak")] {
  use textwrap::core::Word;
  use textwrap::WordSeparator::UnicodeBreakProperties;
  
  assert_eq!(UnicodeBreakProperties.find_words("Emojis: 😂\u{2060}😍").collect::<Vec<_>>(),
             vec![Word::from("Emojis: "),
                  Word::from("😂\u{2060}😍")]);
  }
  ```
  
  The Unicode line breaking algorithm will also automatically
  suppress break breaks around certain punctuation characters::
  
  ```
  #[cfg(feature = "unicode-linebreak")] {
  use textwrap::core::Word;
  use textwrap::WordSeparator::UnicodeBreakProperties;
  
  assert_eq!(UnicodeBreakProperties.find_words("[ foo ] bar !").collect::<Vec<_>>(),
             vec![Word::from("[ foo ] "),
                  Word::from("bar !")]);
  }
  ```

- **`Custom`**

  Find words using a custom word separator

#### Implementations

- `const fn new() -> Self`
  Create a new word separator.

- `fn find_words<'a>(self: &Self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>>>`
  Find all words in `line`.

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

- `fn clone(self: &Self) -> WordSeparator`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`
  Compare two word separators.

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

### `WordSplitter`

```rust
enum WordSplitter {
    NoHyphenation,
    HyphenSplitter,
    Custom(fn(&str) -> Vec<usize>),
}
```

The `WordSplitter` enum describes where words can be split.

If the textwrap crate has been compiled with the `hyphenation`
Cargo feature enabled, you will find a
[`WordSplitter::Hyphenation`](#hyphenation) variant. Use this struct for
language-aware hyphenation:

```
#[cfg(feature = "hyphenation")] {
    use hyphenation::{Language, Load, Standard};
    use textwrap::{wrap, Options, WordSplitter};

    let text = "Oxidation is the loss of electrons.";
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    let options = Options::new(8).word_splitter(WordSplitter::Hyphenation(dictionary));
    assert_eq!(wrap(text, &options), vec!["Oxida-",
                                          "tion is",
                                          "the loss",
                                          "of elec-",
                                          "trons."]);
}
```

Please see the documentation for the [hyphenation](#hyphenation)
 crate for more
details.

[hyphenation](#hyphenation)
: https://docs.rs/hyphenation/

#### Variants

- **`NoHyphenation`**

  Use this as a [`Options.word_splitter`](#optionsword-splitter) to avoid any kind of
  hyphenation:
  
  ```
  use textwrap::{wrap, Options, WordSplitter};
  
  let options = Options::new(8).word_splitter(WordSplitter::NoHyphenation);
  assert_eq!(wrap("foo bar-baz", &options),
             vec!["foo", "bar-baz"]);
  ```
  

- **`HyphenSplitter`**

  `HyphenSplitter` is the default `WordSplitter` used by
  [`Options::new`](super::Options::new). It will split words on
  existing hyphens in the word.
  
  It will only use hyphens that are surrounded by alphanumeric
  characters, which prevents a word like `"--foo-bar"` from
  being split into `"--"` and `"foo-bar"`.
  
  # Examples
  
  ```
  use textwrap::WordSplitter;
  
  assert_eq!(WordSplitter::HyphenSplitter.split_points("--foo-bar"),
             vec![6]);
  ```

- **`Custom`**

  Use a custom function as the word splitter.
  
  This variant lets you implement a custom word splitter using
  your own function.
  
  # Examples
  
  ```
  use textwrap::WordSplitter;
  
  fn split_at_underscore(word: &str) -> Vec<usize> {
      word.match_indices('_').map(|(idx, _)| idx + 1).collect()
  }
  
  let word_splitter = WordSplitter::Custom(split_at_underscore);
  assert_eq!(word_splitter.split_points("a_long_identifier"),
             vec![2, 7]);
  ```

#### Implementations

- `fn split_points(self: &Self, word: &str) -> Vec<usize>`
  Return all possible indices where `word` can be split.

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

- `fn clone(self: &Self) -> WordSplitter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &WordSplitter) -> bool`

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

### `WrapAlgorithm`

```rust
enum WrapAlgorithm {
    FirstFit,
    Custom(fn(&'b [crate::core::Word<'a>], &'b [usize]) -> Vec<&'b [crate::core::Word<'a>]>),
}
```

Describes how to wrap words into lines.

The simplest approach is to wrap words one word at a time and
accept the first way of wrapping which fit
([`WrapAlgorithm::FirstFit`](#firstfit)). If the `smawk` Cargo feature is
enabled, a more complex algorithm is available which will look at
an entire paragraph at a time in order to find optimal line breaks
([`WrapAlgorithm::OptimalFit`](#optimalfit)).

#### Variants

- **`FirstFit`**

  Wrap words using a fast and simple algorithm.
  
  This algorithm uses no look-ahead when finding line breaks.
  Implemented by [`wrap_first_fit()`](#wrap-first-fit), please see that function
  for details and examples.

- **`Custom`**

  Custom wrapping function.
  
  Use this if you want to implement your own wrapping algorithm.
  The function can freely decide how to turn a slice of
  [`Word`](textwrap/core/index.md)s into lines.
  
  # Example
  
  ```
  use textwrap::core::Word;
  use textwrap::{wrap, Options, WrapAlgorithm};
  
  fn stair<'a, 'b>(words: &'b [Word<'a>], _: &'b [usize](#usize)
  ) -> Vec<&'b [Word<'a>]> {
      let mut lines = Vec::new();
      let mut step = 1;
      let mut start_idx = 0;
      while start_idx + step <= words.len() {
        lines.push(&words[start_idx .. start_idx+step]);
        start_idx += step;
        step += 1;
      }
      lines
  }
  
  let options = Options::new(10).wrap_algorithm(WrapAlgorithm::Custom(stair));
  assert_eq!(wrap("First, second, third, fourth, fifth, sixth", options),
             vec!["First,",
                  "second, third,",
                  "fourth, fifth, sixth"]);
  ```

#### Implementations

- `const fn new() -> Self`
  Create new wrap algorithm.

- `fn wrap<'a, 'b>(self: &Self, words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]>`
  Wrap words according to line widths.

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

- `fn clone(self: &Self) -> WrapAlgorithm`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`
  Compare two wrap algorithms.

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

## Functions

