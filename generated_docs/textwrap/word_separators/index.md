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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WordSeparator`](#wordseparator) | enum | Describes where words occur in a line of text. |
| [`find_words_ascii_space`](#find-words-ascii-space) | fn |  |
| [`strip_ansi_escape_sequences`](#strip-ansi-escape-sequences) | fn |  |
| [`find_words_unicode_break_properties`](#find-words-unicode-break-properties) | fn | Find words in line. |
| [`SHY`](#shy) | const | Soft hyphen, also knows as a “shy hyphen”. |

## Enums

### `WordSeparator`

```rust
enum WordSeparator {
    AsciiSpace,
    UnicodeBreakProperties,
    Custom(fn(&str) -> Box<dyn Iterator<Item = crate::core::Word<'_>>>),
}
```

*Defined in [`textwrap-0.16.2/src/word_separators.rs:42-123`](../../../.source_1765633015/textwrap-0.16.2/src/word_separators.rs#L42-L123)*

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

- <span id="wordseparator-new"></span>`const fn new() -> Self`

  Create a new word separator.

  

  The best available algorithm is used by default, i.e.,

  [`WordSeparator::UnicodeBreakProperties`](../index.md) if available,

  otherwise [`WordSeparator::AsciiSpace`](../index.md).

- <span id="wordseparator-find-words"></span>`fn find_words<'a>(&self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>>>` — [`Word`](../core/index.md#word)

  Find all words in `line`.

#### Trait Implementations

##### `impl Any for WordSeparator`

- <span id="wordseparator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WordSeparator`

- <span id="wordseparator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WordSeparator`

- <span id="wordseparator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WordSeparator`

- <span id="wordseparator-clone"></span>`fn clone(&self) -> WordSeparator` — [`WordSeparator`](#wordseparator)

##### `impl CloneToUninit for WordSeparator`

- <span id="wordseparator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for WordSeparator`

##### `impl Debug for WordSeparator`

- <span id="wordseparator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WordSeparator`

- <span id="wordseparator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WordSeparator`

- <span id="wordseparator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for WordSeparator`

- <span id="wordseparator-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

  Compare two word separators.

  

  ```rust

  use textwrap::WordSeparator;

  

  assert_eq!(WordSeparator::AsciiSpace, WordSeparator::AsciiSpace);

  #[cfg(feature = "unicode-linebreak")] {

      assert_eq!(WordSeparator::UnicodeBreakProperties,

                 WordSeparator::UnicodeBreakProperties);

  }

  ```

  

  Note that `WordSeparator::Custom` values never compare equal:

  

  ```rust

  use textwrap::WordSeparator;

  use textwrap::core::Word;

  fn word_separator(line: &str) -> Box<dyn Iterator<Item = Word<'_>> + '_> {

      Box::new(line.split_inclusive(' ').map(Word::from))

  }

  assert_ne!(WordSeparator::Custom(word_separator),

             WordSeparator::Custom(word_separator));

  ```

##### `impl ToOwned for WordSeparator`

- <span id="wordseparator-toowned-type-owned"></span>`type Owned = T`

- <span id="wordseparator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="wordseparator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WordSeparator`

- <span id="wordseparator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wordseparator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WordSeparator`

- <span id="wordseparator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wordseparator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `find_words_ascii_space`

```rust
fn find_words_ascii_space<'a>(line: &'a str) -> Box<dyn Iterator<Item = crate::core::Word<'a>>>
```

*Defined in [`textwrap-0.16.2/src/word_separators.rs:191-216`](../../../.source_1765633015/textwrap-0.16.2/src/word_separators.rs#L191-L216)*

### `strip_ansi_escape_sequences`

```rust
fn strip_ansi_escape_sequences(text: &str) -> String
```

*Defined in [`textwrap-0.16.2/src/word_separators.rs:220-232`](../../../.source_1765633015/textwrap-0.16.2/src/word_separators.rs#L220-L232)*

### `find_words_unicode_break_properties`

```rust
fn find_words_unicode_break_properties<'a>(line: &'a str) -> Box<dyn Iterator<Item = crate::core::Word<'a>>>
```

*Defined in [`textwrap-0.16.2/src/word_separators.rs:243-305`](../../../.source_1765633015/textwrap-0.16.2/src/word_separators.rs#L243-L305)*

Find words in line. ANSI escape sequences are ignored in `line`.

## Constants

### `SHY`
```rust
const SHY: char = '\u{ad}';
```

*Defined in [`textwrap-0.16.2/src/word_separators.rs:239`](../../../.source_1765633015/textwrap-0.16.2/src/word_separators.rs#L239)*

Soft hyphen, also knows as a “shy hyphen”. Should show up as ‘-’
if a line is broken at this point, and otherwise be invisible.
Textwrap does not currently support breaking words at soft
hyphens.

