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

- <span id="options-line-ending"></span>`fn line_ending(self, line_ending: LineEnding) -> Self` — [`LineEnding`](../line_ending/index.md#lineending)

- <span id="options-width"></span>`fn width(self, width: usize) -> Self`

- <span id="options-initial-indent"></span>`fn initial_indent(self, initial_indent: &'a str) -> Self`

- <span id="options-subsequent-indent"></span>`fn subsequent_indent(self, subsequent_indent: &'a str) -> Self`

- <span id="options-break-words"></span>`fn break_words(self, break_words: bool) -> Self`

- <span id="options-word-separator"></span>`fn word_separator(self, word_separator: WordSeparator) -> Options<'a>` — [`WordSeparator`](../word_separators/index.md#wordseparator), [`Options`](#options)

- <span id="options-wrap-algorithm"></span>`fn wrap_algorithm(self, wrap_algorithm: WrapAlgorithm) -> Options<'a>` — [`WrapAlgorithm`](../wrap_algorithms/index.md#wrapalgorithm), [`Options`](#options)

- <span id="options-word-splitter"></span>`fn word_splitter(self, word_splitter: WordSplitter) -> Options<'a>` — [`WordSplitter`](../word_splitters/index.md#wordsplitter), [`Options`](#options)

#### Trait Implementations

##### `impl Clone for Options<'a>`

- <span id="options-clone"></span>`fn clone(&self) -> Options<'a>` — [`Options`](#options)

##### `impl Debug for Options<'a>`

- <span id="options-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

