*[textwrap](../index.md) / [options](index.md)*

---

# Module `options`

Options for wrapping text.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Options`](#options) | struct | Holds configuration options for wrapping and filling text. |

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

*Defined in [`textwrap-0.16.2/src/options.rs:8-33`](../../../.source_1765521767/textwrap-0.16.2/src/options.rs#L8-L33)*

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
  [`WrapAlgorithm`](../wrap_algorithms/index.md) trait for details.

- **`word_separator`**: `crate::WordSeparator`

  The line breaking algorithm to use, see the [`WordSeparator`](../word_separators/index.md)
  trait for an overview and possible implementations.

- **`word_splitter`**: `crate::WordSplitter`

  The method for splitting words. This can be used to prohibit
  splitting words on hyphens, or it can be used to implement
  language-aware machine hyphenation.

#### Implementations

- <span id="options-new"></span>`const fn new(width: usize) -> Self`

  Creates a new [`Options`](#options) with the specified width.

  

  The other fields are given default values as follows:

  

  ```rust

  use textwrap::{LineEnding, Options, WordSplitter, WordSeparator, WrapAlgorithm};

  let width = 80;

  let options = Options::new(width);

  assert_eq!(options.line_ending, LineEnding::LF);

  assert_eq!(options.initial_indent, "");

  assert_eq!(options.subsequent_indent, "");

  assert_eq!(options.break_words, true);

  

  #[cfg(feature = "unicode-linebreak")]

  assert_eq!(options.word_separator, WordSeparator::UnicodeBreakProperties);

  #[cfg(not(feature = "unicode-linebreak"))]

  assert_eq!(options.word_separator, WordSeparator::AsciiSpace);

  

  #[cfg(feature = "smawk")]

  assert_eq!(options.wrap_algorithm, WrapAlgorithm::new_optimal_fit());

  #[cfg(not(feature = "smawk"))]

  assert_eq!(options.wrap_algorithm, WrapAlgorithm::FirstFit);

  

  assert_eq!(options.word_splitter, WordSplitter::HyphenSplitter);

  ```

  

  Note that the default word separator and wrap algorithms

  changes based on the available Cargo features. The best

  available algorithms are used by default.

- <span id="options-line-ending"></span>`fn line_ending(self, line_ending: LineEnding) -> Self` — [`LineEnding`](../line_ending/index.md#lineending)

  Change `self.line_ending`. This specifies which of the

  supported line endings should be used to break the lines of the

  input text.

  

  # Examples

  

  ```rust

  use textwrap::{refill, LineEnding, Options};

  

  let options = Options::new(15).line_ending(LineEnding::CRLF);

  assert_eq!(refill("This is a little example.", options),

             "This is a\r\nlittle example.");

  ```

- <span id="options-width"></span>`fn width(self, width: usize) -> Self`

  Set `self.width` to the given value.

- <span id="options-initial-indent"></span>`fn initial_indent(self, initial_indent: &'a str) -> Self`

  Change `self.initial_indent`. The initial indentation is

  used on the very first line of output.

  

  # Examples

  

  Classic paragraph indentation can be achieved by specifying an

  initial indentation and wrapping each paragraph by itself:

  

  ```rust

  use textwrap::{wrap, Options};

  

  let options = Options::new(16).initial_indent("    ");

  assert_eq!(wrap("This is a little example.", options),

             vec!["    This is a",

                  "little example."]);

  ```

- <span id="options-subsequent-indent"></span>`fn subsequent_indent(self, subsequent_indent: &'a str) -> Self`

  Change `self.subsequent_indent`. The subsequent indentation

  is used on lines following the first line of output.

  

  # Examples

  

  Combining initial and subsequent indentation lets you format a

  single paragraph as a bullet list:

  

  ```rust

  use textwrap::{wrap, Options};

  

  let options = Options::new(12)

      .initial_indent("* ")

      .subsequent_indent("  ");

  #[cfg(feature = "smawk")]

  assert_eq!(wrap("This is a little example.", options),

             vec!["* This is",

                  "  a little",

                  "  example."]);

  

  // Without the `smawk` feature, the wrapping is a little different:

  #[cfg(not(feature = "smawk"))]

  assert_eq!(wrap("This is a little example.", options),

             vec!["* This is a",

                  "  little",

                  "  example."]);

  ```

- <span id="options-break-words"></span>`fn break_words(self, break_words: bool) -> Self`

  Change `self.break_words`. This controls if words longer

  than `self.width` can be broken, or if they will be left

  sticking out into the right margin.

  

  See `Options::word_splitter` instead if you want to control

  hyphenation.

  

  # Examples

  

  ```rust

  use textwrap::{wrap, Options};

  

  let options = Options::new(4).break_words(true);

  assert_eq!(wrap("This is a little example.", options),

             vec!["This",

                  "is a",

                  "litt",

                  "le",

                  "exam",

                  "ple."]);

  ```

- <span id="options-word-separator"></span>`fn word_separator(self, word_separator: WordSeparator) -> Options<'a>` — [`WordSeparator`](../word_separators/index.md#wordseparator), [`Options`](#options)

  Change `self.word_separator`.

  

  See the [`WordSeparator`](../word_separators/index.md) trait for details on the choices.

- <span id="options-wrap-algorithm"></span>`fn wrap_algorithm(self, wrap_algorithm: WrapAlgorithm) -> Options<'a>` — [`WrapAlgorithm`](../wrap_algorithms/index.md#wrapalgorithm), [`Options`](#options)

  Change `self.wrap_algorithm`.

  

  See the [`WrapAlgorithm`](../wrap_algorithms/index.md) trait for details on the choices.

- <span id="options-word-splitter"></span>`fn word_splitter(self, word_splitter: WordSplitter) -> Options<'a>` — [`WordSplitter`](../word_splitters/index.md#wordsplitter), [`Options`](#options)

  Change `self.word_splitter`. The [`WordSplitter`](../word_splitters/index.md) is used to

  fit part of a word into the current line when wrapping text.

  

  See `Options::break_words` instead if you want to control the

  handling of words longer than the line width.

  

  # Examples

  

  ```rust

  use textwrap::{wrap, Options, WordSplitter};

  

  // The default is WordSplitter::HyphenSplitter.

  let options = Options::new(5);

  assert_eq!(wrap("foo-bar-baz", &options),

             vec!["foo-", "bar-", "baz"]);

  

  // The word is now so long that break_words kick in:

  let options = Options::new(5)

      .word_splitter(WordSplitter::NoHyphenation);

  assert_eq!(wrap("foo-bar-baz", &options),

             vec!["foo-b", "ar-ba", "z"]);

  

  // If you want to breaks at all, disable both:

  let options = Options::new(5)

      .break_words(false)

      .word_splitter(WordSplitter::NoHyphenation);

  assert_eq!(wrap("foo-bar-baz", &options),

             vec!["foo-bar-baz"]);

  ```

#### Trait Implementations

##### `impl Any for Options<'a>`

- <span id="options-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Options<'a>`

- <span id="options-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Options<'a>`

- <span id="options-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Options<'a>`

- <span id="options-clone"></span>`fn clone(&self) -> Options<'a>` — [`Options`](#options)

##### `impl CloneToUninit for Options<'a>`

- <span id="options-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Options<'a>`

- <span id="options-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Options<'a>`

- <span id="options-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Options<'a>`

- <span id="options-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Options<'a>`

- <span id="options-toowned-type-owned"></span>`type Owned = T`

- <span id="options-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="options-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Options<'a>`

- <span id="options-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="options-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Options<'a>`

- <span id="options-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="options-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

