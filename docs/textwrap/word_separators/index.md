*[textwrap](../index.md) / [word_separators](index.md)*

---

# Module `word_separators`

Functionality for finding words.

In order to wrap text, we need to know where the legal break
points are, i.e., where the words of the text are. This means that
we need to define what a "word" is.

A simple approach is to simply split the text on whitespace, but
this does not work for East-Asian languages such as Chinese or
Japanese where there are no spaces between words. Breaking a long
sequence of emojis is another example where line breaks might be
wanted even if there are no whitespace to be found.

The [`WordSeparator`](#wordseparator) enum is responsible for determining where
there words are in a line of text. Please refer to the enum and
its variants for more information.

## Enums

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
  
  Unlike [`WordSeparator::AsciiSpace`](../index.md), the Unicode line
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

- `const fn new() -> Self`

- `fn find_words<'a>(self: &Self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>>>` — [`Word`](../core/index.md)

#### Trait Implementations

##### `impl Clone for WordSeparator`

- `fn clone(self: &Self) -> WordSeparator` — [`WordSeparator`](#wordseparator)

##### `impl Copy for WordSeparator`

##### `impl Debug for WordSeparator`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for WordSeparator`

- `fn eq(self: &Self, other: &Self) -> bool`

## Functions

### `find_words_ascii_space`

```rust
fn find_words_ascii_space<'a>(line: &'a str) -> Box<dyn Iterator<Item = crate::core::Word<'a>>>
```

### `strip_ansi_escape_sequences`

```rust
fn strip_ansi_escape_sequences(text: &str) -> String
```

### `find_words_unicode_break_properties`

```rust
fn find_words_unicode_break_properties<'a>(line: &'a str) -> Box<dyn Iterator<Item = crate::core::Word<'a>>>
```

Find words in line. ANSI escape sequences are ignored in `line`.

## Constants

### `SHY`

```rust
const SHY: char = '\u{ad}';
```

Soft hyphen, also knows as a “shy hyphen”. Should show up as ‘-’
if a line is broken at this point, and otherwise be invisible.
Textwrap does not currently support breaking words at soft
hyphens.

