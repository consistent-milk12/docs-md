*[textwrap](../index.md) / [options](index.md)*

---

# Module `options`

Options for wrapping text.

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
  [`WrapAlgorithm`](../index.md) trait for details.

- **`word_separator`**: `crate::WordSeparator`

  The line breaking algorithm to use, see the [`WordSeparator`](../index.md)
  trait for an overview and possible implementations.

- **`word_splitter`**: `crate::WordSplitter`

  The method for splitting words. This can be used to prohibit
  splitting words on hyphens, or it can be used to implement
  language-aware machine hyphenation.

#### Implementations

- `const fn new(width: usize) -> Self`

- `fn line_ending(self: Self, line_ending: LineEnding) -> Self` — [`LineEnding`](../index.md)

- `fn width(self: Self, width: usize) -> Self`

- `fn initial_indent(self: Self, initial_indent: &'a str) -> Self`

- `fn subsequent_indent(self: Self, subsequent_indent: &'a str) -> Self`

- `fn break_words(self: Self, break_words: bool) -> Self`

- `fn word_separator(self: Self, word_separator: WordSeparator) -> Options<'a>` — [`WordSeparator`](../index.md), [`Options`](../index.md)

- `fn wrap_algorithm(self: Self, wrap_algorithm: WrapAlgorithm) -> Options<'a>` — [`WrapAlgorithm`](../index.md), [`Options`](../index.md)

- `fn word_splitter(self: Self, word_splitter: WordSplitter) -> Options<'a>` — [`WordSplitter`](../index.md), [`Options`](../index.md)

#### Trait Implementations

##### `impl<'a> Clone for Options<'a>`

- `fn clone(self: &Self) -> Options<'a>` — [`Options`](../index.md)

##### `impl<'a> Debug for Options<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

