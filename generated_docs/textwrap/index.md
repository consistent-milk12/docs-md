# Crate `textwrap`

The textwrap library provides functions for word wrapping and
indenting text.

# Wrapping Text

Wrapping text can be very useful in command-line programs where
you want to format dynamic output nicely so it looks good in a
terminal. A quick example:

```rust
#[cfg(feature = "smawk")] {
let text = "textwrap: a small library for wrapping text.";
assert_eq!(textwrap::wrap(text, 18),
           vec!["textwrap: a",
                "small library for",
                "wrapping text."]);
}
```

The [`wrap()`](wrap/index.md) function returns the individual lines, use
[`fill()`](fill/index.md) is you want the lines joined with `'\n'` to form a
`String`.

If you enable the `hyphenation` Cargo feature, you can get
automatic hyphenation for a number of languages:

```rust
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

See also the [`unfill()`](refill/index.md) and [`refill()`](refill/index.md) functions which allow
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
example, [`indent()`](indentation/index.md) allows you to turn lines of text into a
bullet list:

```rust
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

Removing leading whitespace is done with [`dedent()`](indentation/index.md):

```rust
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
  to enable this feature. See [`WordSeparator`](word_separators/index.md) for details.

* `unicode-width`: enables correct width computation of non-ASCII
  characters via the [unicode-width] crate. Without this feature,
  every [`char`](../unicode_normalization/char/index.md) is 1 column wide, except for emojis which are 2
  columns wide. See [`core::display_width()`](core/index.md) for details.

  This feature can be disabled if you only need to wrap ASCII
  text, or if the functions in [`core`](core/index.md) are used directly with
  [`core::Fragment`](core/index.md)s for which the widths have been computed in
  other ways.

* `smawk`: enables linear-time wrapping of the whole paragraph via
  the [smawk] crate. See `wrap_algorithms::wrap_optimal_fit()`
  for details on the optimal-fit algorithm.

  This feature can be disabled if you only ever intend to use
  [`wrap_algorithms::wrap_first_fit()`](wrap_algorithms/index.md).

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
  width via the [`terminal_size`](../terminal_size/index.md) crate. See
  `Options::with_termwidth()` for details.

* `hyphenation`: enables language-sensitive hyphenation via the
  [hyphenation] crate. See the [`word_splitters::WordSplitter`](word_splitters/index.md)
  trait for details.








## Contents

- [Modules](#modules)
  - [`core`](#core)
  - [`word_splitters`](#word_splitters)
  - [`wrap_algorithms`](#wrap_algorithms)
  - [`columns`](#columns)
  - [`fill`](#fill)
  - [`indentation`](#indentation)
  - [`line_ending`](#line_ending)
  - [`options`](#options)
  - [`refill`](#refill)
  - [`word_separators`](#word_separators)
  - [`wrap`](#wrap)
- [Structs](#structs)
  - [`Options`](#options)
- [Enums](#enums)
  - [`LineEnding`](#lineending)
  - [`WordSeparator`](#wordseparator)
  - [`WordSplitter`](#wordsplitter)
  - [`WrapAlgorithm`](#wrapalgorithm)
- [Functions](#functions)
  - [`wrap_columns`](#wrap_columns)
  - [`fill`](#fill)
  - [`fill_inplace`](#fill_inplace)
  - [`dedent`](#dedent)
  - [`indent`](#indent)
  - [`refill`](#refill)
  - [`unfill`](#unfill)
  - [`wrap`](#wrap)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`core`](#core) | mod | Building blocks for advanced wrapping functionality. |
| [`word_splitters`](#word_splitters) | mod | Word splitting functionality. |
| [`wrap_algorithms`](#wrap_algorithms) | mod | Word wrapping algorithms. |
| [`columns`](#columns) | mod | Functionality for wrapping text into columns. |
| [`fill`](#fill) | mod | Functions for filling text. |
| [`indentation`](#indentation) | mod | Functions related to adding and removing indentation from lines of text. |
| [`line_ending`](#line_ending) | mod | Line ending detection and conversion. |
| [`options`](#options) | mod | Options for wrapping text. |
| [`refill`](#refill) | mod | Functionality for unfilling and refilling text. |
| [`word_separators`](#word_separators) | mod | Functionality for finding words. |
| [`wrap`](#wrap) | mod | Functions for wrapping text. |
| [`Options`](#options) | struct |  |
| [`LineEnding`](#lineending) | enum |  |
| [`WordSeparator`](#wordseparator) | enum |  |
| [`WordSplitter`](#wordsplitter) | enum |  |
| [`WrapAlgorithm`](#wrapalgorithm) | enum |  |
| [`wrap_columns`](#wrap_columns) | fn |  |
| [`fill`](#fill) | fn |  |
| [`fill_inplace`](#fill_inplace) | fn |  |
| [`dedent`](#dedent) | fn |  |
| [`indent`](#indent) | fn |  |
| [`refill`](#refill) | fn |  |
| [`unfill`](#unfill) | fn |  |
| [`wrap`](#wrap) | fn |  |

## Modules

- [`core`](core/index.md) — Building blocks for advanced wrapping functionality.
- [`word_splitters`](word_splitters/index.md) — Word splitting functionality.
- [`wrap_algorithms`](wrap_algorithms/index.md) — Word wrapping algorithms.
- [`columns`](columns/index.md) — Functionality for wrapping text into columns.
- [`fill`](fill/index.md) — Functions for filling text.
- [`indentation`](indentation/index.md) — Functions related to adding and removing indentation from lines of
- [`line_ending`](line_ending/index.md) — Line ending detection and conversion.
- [`options`](options/index.md) — Options for wrapping text.
- [`refill`](refill/index.md) — Functionality for unfilling and refilling text.
- [`word_separators`](word_separators/index.md) — Functionality for finding words.
- [`wrap`](wrap/index.md) — Functions for wrapping text.

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

*Defined in [`textwrap-0.16.2/src/options.rs:8-33`](../../.source_1765210505/textwrap-0.16.2/src/options.rs#L8-L33)*

Holds configuration options for wrapping and filling text.

#### Fields

- **`width`**: `usize`

  The width in columns at which the text will be wrapped.

- **`line_ending`**: `crate::LineEnding`

  Line ending used for breaking lines.

- **`initial_indent`**: `&'a str`

  Indentation used for the first line of output. See the
  `Options::initial_indent` method.

- **`subsequent_indent`**: `&'a str`

  Indentation used for subsequent lines of output. See the
  `Options::subsequent_indent` method.

- **`break_words`**: `bool`

  Allow long words to be broken if they cannot fit on a line.
  When set to `false`, some lines may be longer than
  `self.width`. See the `Options::break_words` method.

- **`wrap_algorithm`**: `crate::WrapAlgorithm`

  Wrapping algorithm to use, see the implementations of the
  [`WrapAlgorithm`](wrap_algorithms/index.md) trait for details.

- **`word_separator`**: `crate::WordSeparator`

  The line breaking algorithm to use, see the [`WordSeparator`](word_separators/index.md)
  trait for an overview and possible implementations.

- **`word_splitter`**: `crate::WordSplitter`

  The method for splitting words. This can be used to prohibit
  splitting words on hyphens, or it can be used to implement
  language-aware machine hyphenation.

#### Implementations

- <span id="options-new"></span>`const fn new(width: usize) -> Self`

- <span id="options-line-ending"></span>`fn line_ending(self, line_ending: LineEnding) -> Self` — [`LineEnding`](line_ending/index.md#lineending)

- <span id="options-width"></span>`fn width(self, width: usize) -> Self`

- <span id="options-initial-indent"></span>`fn initial_indent(self, initial_indent: &'a str) -> Self`

- <span id="options-subsequent-indent"></span>`fn subsequent_indent(self, subsequent_indent: &'a str) -> Self`

- <span id="options-break-words"></span>`fn break_words(self, break_words: bool) -> Self`

- <span id="options-word-separator"></span>`fn word_separator(self, word_separator: WordSeparator) -> Options<'a>` — [`WordSeparator`](word_separators/index.md#wordseparator), [`Options`](options/index.md#options)

- <span id="options-wrap-algorithm"></span>`fn wrap_algorithm(self, wrap_algorithm: WrapAlgorithm) -> Options<'a>` — [`WrapAlgorithm`](wrap_algorithms/index.md#wrapalgorithm), [`Options`](options/index.md#options)

- <span id="options-word-splitter"></span>`fn word_splitter(self, word_splitter: WordSplitter) -> Options<'a>` — [`WordSplitter`](word_splitters/index.md#wordsplitter), [`Options`](options/index.md#options)

#### Trait Implementations

##### `impl Clone for Options<'a>`

- <span id="options-clone"></span>`fn clone(&self) -> Options<'a>` — [`Options`](options/index.md#options)

##### `impl Debug for Options<'a>`

- <span id="options-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `LineEnding`

```rust
enum LineEnding {
    CRLF,
    LF,
}
```

*Defined in [`textwrap-0.16.2/src/line_ending.rs:8-16`](../../.source_1765210505/textwrap-0.16.2/src/line_ending.rs#L8-L16)*

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

- <span id="lineending-as-str"></span>`const fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Clone for LineEnding`

- <span id="lineending-clone"></span>`fn clone(&self) -> LineEnding` — [`LineEnding`](line_ending/index.md#lineending)

##### `impl Copy for LineEnding`

##### `impl Debug for LineEnding`

- <span id="lineending-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineEnding`

##### `impl PartialEq for LineEnding`

- <span id="lineending-eq"></span>`fn eq(&self, other: &LineEnding) -> bool` — [`LineEnding`](line_ending/index.md#lineending)

##### `impl StructuralPartialEq for LineEnding`

### `WordSeparator`

```rust
enum WordSeparator {
    AsciiSpace,
    UnicodeBreakProperties,
    Custom(fn(&str) -> Box<dyn Iterator<Item = crate::core::Word<'_>>>),
}
```

*Defined in [`textwrap-0.16.2/src/word_separators.rs:42-123`](../../.source_1765210505/textwrap-0.16.2/src/word_separators.rs#L42-L123)*

Describes where words occur in a line of text.

The simplest approach is say that words are separated by one or
more ASCII spaces (`' '`). This works for Western languages
without emojis. A more complex approach is to use the Unicode line
breaking algorithm, which finds break points in non-ASCII text.

The line breaks occur between words, please see
[`WordSplitter`](crate::WordSplitter) for options of how to handle
hyphenation of individual words.

# Examples

```rust
use textwrap::core::Word;
use textwrap::WordSeparator::AsciiSpace;

let words = AsciiSpace.find_words("Hello World!").collect::<Vec<_>>();
assert_eq!(words, vec![Word::from("Hello "), Word::from("World!")]);
```

#### Variants

- **`AsciiSpace`**

  Find words by splitting on runs of `' '` characters.
  
  # Examples
  
  ```rust
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
  
  Unlike [`WordSeparator::AsciiSpace`](#wordseparatorasciispace), the Unicode line
  breaking algorithm will find line break opportunities between
  some characters with no intervening whitespace:
  
  ```rust
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
  
  ```rust
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
  
  ```rust
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

- <span id="wordseparator-new"></span>`const fn new() -> Self`

- <span id="wordseparator-find-words"></span>`fn find_words<'a>(&self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>>>` — [`Word`](core/index.md#word)

#### Trait Implementations

##### `impl Clone for WordSeparator`

- <span id="wordseparator-clone"></span>`fn clone(&self) -> WordSeparator` — [`WordSeparator`](word_separators/index.md#wordseparator)

##### `impl Copy for WordSeparator`

##### `impl Debug for WordSeparator`

- <span id="wordseparator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for WordSeparator`

- <span id="wordseparator-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `WordSplitter`

```rust
enum WordSplitter {
    NoHyphenation,
    HyphenSplitter,
    Custom(fn(&str) -> Vec<usize>),
}
```

*Defined in [`textwrap-0.16.2/src/word_splitters.rs:37-99`](../../.source_1765210505/textwrap-0.16.2/src/word_splitters.rs#L37-L99)*

The `WordSplitter` enum describes where words can be split.

If the textwrap crate has been compiled with the `hyphenation`
Cargo feature enabled, you will find a
`WordSplitter::Hyphenation` variant. Use this struct for
language-aware hyphenation:

```rust
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

Please see the documentation for the [hyphenation] crate for more
details.


#### Variants

- **`NoHyphenation`**

  Use this as a `Options.word_splitter` to avoid any kind of
  hyphenation:
  
  ```rust
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
  
  ```rust
  use textwrap::WordSplitter;
  
  assert_eq!(WordSplitter::HyphenSplitter.split_points("--foo-bar"),
             vec![6]);
  ```

- **`Custom`**

  Use a custom function as the word splitter.
  
  This variant lets you implement a custom word splitter using
  your own function.
  
  # Examples
  
  ```rust
  use textwrap::WordSplitter;
  
  fn split_at_underscore(word: &str) -> Vec<usize> {
      word.match_indices('_').map(|(idx, _)| idx + 1).collect()
  }
  
  let word_splitter = WordSplitter::Custom(split_at_underscore);
  assert_eq!(word_splitter.split_points("a_long_identifier"),
             vec![2, 7]);
  ```

#### Implementations

- <span id="wordsplitter-split-points"></span>`fn split_points(&self, word: &str) -> Vec<usize>`

#### Trait Implementations

##### `impl Clone for WordSplitter`

- <span id="wordsplitter-clone"></span>`fn clone(&self) -> WordSplitter` — [`WordSplitter`](word_splitters/index.md#wordsplitter)

##### `impl Debug for WordSplitter`

- <span id="wordsplitter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for WordSplitter`

- <span id="wordsplitter-eq"></span>`fn eq(&self, other: &WordSplitter) -> bool` — [`WordSplitter`](word_splitters/index.md#wordsplitter)

### `WrapAlgorithm`

```rust
enum WrapAlgorithm {
    FirstFit,
    Custom(fn(&'b [crate::core::Word<'a>], &'b [usize]) -> Vec<&'b [crate::core::Word<'a>]>),
}
```

*Defined in [`textwrap-0.16.2/src/wrap_algorithms.rs:36-90`](../../.source_1765210505/textwrap-0.16.2/src/wrap_algorithms.rs#L36-L90)*

Describes how to wrap words into lines.

The simplest approach is to wrap words one word at a time and
accept the first way of wrapping which fit
([`WrapAlgorithm::FirstFit`](#wrapalgorithmfirstfit)). If the `smawk` Cargo feature is
enabled, a more complex algorithm is available which will look at
an entire paragraph at a time in order to find optimal line breaks
(`WrapAlgorithm::OptimalFit`).

#### Variants

- **`FirstFit`**

  Wrap words using a fast and simple algorithm.
  
  This algorithm uses no look-ahead when finding line breaks.
  Implemented by [`wrap_first_fit()`](wrap_algorithms/index.md), please see that function
  for details and examples.

- **`Custom`**

  Custom wrapping function.
  
  Use this if you want to implement your own wrapping algorithm.
  The function can freely decide how to turn a slice of
  [`Word`](core/index.md)s into lines.
  
  # Example
  
  ```rust
  use textwrap::core::Word;
  use textwrap::{wrap, Options, WrapAlgorithm};
  
  fn stair<'a, 'b>(words: &'b [Word<'a>], _: &'b [usize]) -> Vec<&'b [Word<'a>]> {
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

- <span id="wrapalgorithm-new"></span>`const fn new() -> Self`

- <span id="wrapalgorithm-wrap"></span>`fn wrap<'a, 'b>(&self, words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]>` — [`Word`](core/index.md#word)

#### Trait Implementations

##### `impl Clone for WrapAlgorithm`

- <span id="wrapalgorithm-clone"></span>`fn clone(&self) -> WrapAlgorithm` — [`WrapAlgorithm`](wrap_algorithms/index.md#wrapalgorithm)

##### `impl Copy for WrapAlgorithm`

##### `impl Debug for WrapAlgorithm`

- <span id="wrapalgorithm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WrapAlgorithm`

- <span id="wrapalgorithm-default"></span>`fn default() -> Self`

##### `impl PartialEq for WrapAlgorithm`

- <span id="wrapalgorithm-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Functions

*Defined in [`textwrap-0.16.2/src/lib.rs:224`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L224)*

*Defined in [`textwrap-0.16.2/src/lib.rs:225`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L225)*

*Defined in [`textwrap-0.16.2/src/lib.rs:225`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L225)*

*Defined in [`textwrap-0.16.2/src/lib.rs:226`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L226)*

*Defined in [`textwrap-0.16.2/src/lib.rs:226`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L226)*

*Defined in [`textwrap-0.16.2/src/lib.rs:229`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L229)*

*Defined in [`textwrap-0.16.2/src/lib.rs:229`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L229)*

*Defined in [`textwrap-0.16.2/src/lib.rs:234`](../../.source_1765210505/textwrap-0.16.2/src/lib.rs#L234)*

