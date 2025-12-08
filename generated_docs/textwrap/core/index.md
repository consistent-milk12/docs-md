*[textwrap](../index.md) / [core](index.md)*

---

# Module `core`

Building blocks for advanced wrapping functionality.

The functions and structs in this module can be used to implement
advanced wrapping functionality when [`wrap()`](crate::wrap())
[`fill()`](crate::fill()) don't do what you want.

In general, you want to follow these steps when wrapping
something:

1. Split your input into [`Fragment`](#fragment)s. These are abstract blocks
   of text or content which can be wrapped into lines. See
   [`WordSeparator`](crate::word_separators::WordSeparator) for
   how to do this for text.

2. Potentially split your fragments into smaller pieces. This
   allows you to implement things like hyphenation. If you use the
   `Word` type, you can use [`WordSplitter`](crate::WordSplitter)
   enum for this.

3. Potentially break apart fragments that are still too large to
   fit on a single line. This is implemented in [`break_words`](#break-words).

4. Finally take your fragments and put them into lines. There are
   two algorithms for this in the
   [`wrap_algorithms`](crate::wrap_algorithms) module:
   [`wrap_optimal_fit`](crate::wrap_algorithms::wrap_optimal_fit)
   and [`wrap_first_fit`](crate::wrap_algorithms::wrap_first_fit).
   The former produces better line breaks, the latter is faster.

5. Iterate through the slices returned by the wrapping functions
   and construct your lines of output.

Please [open an issue](https://github.com/mgeisler/textwrap/) if
the functionality here is not sufficient or if you have ideas for
improving it. We would love to hear from you!

## Contents

- [Structs](#structs)
  - [`Word`](#word)
- [Traits](#traits)
  - [`Fragment`](#fragment)
- [Functions](#functions)
  - [`skip_ansi_escape_sequence`](#skip_ansi_escape_sequence)
  - [`ch_width`](#ch_width)
  - [`display_width`](#display_width)
  - [`break_words`](#break_words)
- [Constants](#constants)
  - [`CSI`](#csi)
  - [`ANSI_FINAL_BYTE`](#ansi_final_byte)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Word`](#word) | struct | A piece of wrappable text, including any trailing whitespace. |
| [`Fragment`](#fragment) | trait | A (text) fragment denotes the unit which we wrap into lines. |
| [`skip_ansi_escape_sequence`](#skip_ansi_escape_sequence) | fn | Skip ANSI escape sequences. |
| [`ch_width`](#ch_width) | fn |  |
| [`display_width`](#display_width) | fn | Compute the display width of `text` while skipping over ANSI |
| [`break_words`](#break_words) | fn | Forcibly break words wider than `line_width` into smaller words. |
| [`CSI`](#csi) | const | The CSI or “Control Sequence Introducer” introduces an ANSI escape |
| [`ANSI_FINAL_BYTE`](#ansi_final_byte) | const | The final bytes of an ANSI escape sequence must be in this range. |

## Structs

### `Word<'a>`

```rust
struct Word<'a> {
    pub word: &'a str,
    pub whitespace: &'a str,
    pub penalty: &'a str,
    width: usize,
}
```

A piece of wrappable text, including any trailing whitespace.

A `Word` is an example of a [`Fragment`](#fragment), so it has a width,
trailing whitespace, and potentially a penalty item.

#### Fields

- **`word`**: `&'a str`

  Word content.

- **`whitespace`**: `&'a str`

  Whitespace to insert if the word does not fall at the end of a line.

- **`penalty`**: `&'a str`

  Penalty string to insert if the word falls at the end of a line.

#### Implementations

- <span id="word-from"></span>`fn from(word: &str) -> Word<'_>` — [`Word`](#word)

- <span id="word-break-apart"></span>`fn break_apart<'b>(self: &'b Self, line_width: usize) -> impl Iterator<Item = Word<'a>> + 'b` — [`Word`](#word)

#### Trait Implementations

##### `impl<'a> Clone for Word<'a>`

- <span id="word-clone"></span>`fn clone(&self) -> Word<'a>` — [`Word`](#word)

##### `impl<'a> Copy for Word<'a>`

##### `impl<'a> Debug for Word<'a>`

- <span id="word-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Word<'_>`

- <span id="word-target"></span>`type Target = str`

- <span id="word-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<'a> Eq for Word<'a>`

##### `impl Fragment for Word<'_>`

- <span id="word-width"></span>`fn width(&self) -> f64`

- <span id="word-whitespace-width"></span>`fn whitespace_width(&self) -> f64`

- <span id="word-penalty-width"></span>`fn penalty_width(&self) -> f64`

##### `impl<'a> PartialEq for Word<'a>`

- <span id="word-eq"></span>`fn eq(&self, other: &Word<'a>) -> bool` — [`Word`](#word)

##### `impl<P, T> Receiver for Word<'a>`

- <span id="word-target"></span>`type Target = T`

##### `impl<'a> StructuralPartialEq for Word<'a>`

## Traits

### `Fragment`

```rust
trait Fragment: std::fmt::Debug { ... }
```

A (text) fragment denotes the unit which we wrap into lines.

Fragments represent an abstract _word_ plus the _whitespace_
following the word. In case the word falls at the end of the line,
the whitespace is dropped and a so-called _penalty_ is inserted
instead (typically `"-"` if the word was hyphenated).

For wrapping purposes, the precise content of the word, the
whitespace, and the penalty is irrelevant. All we need to know is
the displayed width of each part, which this trait provides.

#### Required Methods

- `fn width(&self) -> f64`

  Displayed width of word represented by this fragment.

- `fn whitespace_width(&self) -> f64`

  Displayed width of the whitespace that must follow the word

- `fn penalty_width(&self) -> f64`

  Displayed width of the penalty that must be inserted if the

## Functions

### `skip_ansi_escape_sequence`

```rust
fn skip_ansi_escape_sequence<I: Iterator<Item = char>>(ch: char, chars: &mut I) -> bool
```

Skip ANSI escape sequences.

The `ch` is the current `char`, the `chars` provide the following
characters. The `chars` will be modified if `ch` is the start of
an ANSI escape sequence.

Returns `true` if one or more chars were skipped.

### `ch_width`

```rust
fn ch_width(ch: char) -> usize
```

### `display_width`

```rust
fn display_width(text: &str) -> usize
```

Compute the display width of `text` while skipping over ANSI
escape sequences.

# Examples

```rust
use textwrap::core::display_width;

assert_eq!(display_width("Café Plain"), 10);
assert_eq!(display_width("\u{1b}[31mCafé Rouge\u{1b}[0m"), 10);
assert_eq!(display_width("\x1b]8;;http://example.com\x1b\\This is a link\x1b]8;;\x1b\\"), 14);
```

**Note:** When the `unicode-width` Cargo feature is disabled, the
width of a `char` is determined by a crude approximation which
simply counts chars below U+1100 as 1 column wide, and all other
characters as 2 columns wide. With the feature enabled, function
will correctly deal with [combining characters] in their
decomposed form (see [Unicode equivalence]).

An example of a decomposed character is “é”, which can be
decomposed into: “e” followed by a combining acute accent: “◌́”.
Without the `unicode-width` Cargo feature, every `char` below
U+1100 has a width of 1. This includes the combining accent:

```rust
use textwrap::core::display_width;

assert_eq!(display_width("Cafe Plain"), 10);
#[cfg(feature = "unicode-width")]
assert_eq!(display_width("Cafe\u{301} Plain"), 10);
#[cfg(not(feature = "unicode-width"))]
assert_eq!(display_width("Cafe\u{301} Plain"), 11);
```

## Emojis and CJK Characters

Characters such as emojis and [CJK characters] used in the
Chinese, Japanese, and Korean languages are seen as double-width,
even if the `unicode-width` feature is disabled:

```rust
use textwrap::core::display_width;

assert_eq!(display_width("😂😭🥺🤣✨😍🙏🥰😊🔥"), 20);
assert_eq!(display_width("你好"), 4);  // “Nǐ hǎo” or “Hello” in Chinese
```

# Limitations

The displayed width of a string cannot always be computed from the
string alone. This is because the width depends on the rendering
engine used. This is particularly visible with [emoji modifier
sequences] where a base emoji is modified with, e.g., skin tone or
hair color modifiers. It is up to the rendering engine to detect
this and to produce a suitable emoji.

A simple example is “❤️”, which consists of “❤” (U+2764: Black
Heart Symbol) followed by U+FE0F (Variation Selector-16). By
itself, “❤” is a black heart, but if you follow it with the
variant selector, you may get a wider red heart.

A more complex example would be “👨‍🦰” which should depict a man
with red hair. Here the computed width is too large — and the
width differs depending on the use of the `unicode-width` feature:

```rust
use textwrap::core::display_width;

assert_eq!("👨‍🦰".chars().collect::<Vec<char>>(), ['\u{1f468}', '\u{200d}', '\u{1f9b0}']);
#[cfg(feature = "unicode-width")]
assert_eq!(display_width("👨‍🦰"), 4);
#[cfg(not(feature = "unicode-width"))]
assert_eq!(display_width("👨‍🦰"), 6);
```

This happens because the grapheme consists of three code points:
“👨” (U+1F468: Man), Zero Width Joiner (U+200D), and “🦰”
(U+1F9B0: Red Hair). You can see them above in the test. With
`unicode-width` enabled, the ZWJ is correctly seen as having zero
width, without it is counted as a double-width character.

## Terminal Support

Modern browsers typically do a great job at combining characters
as shown above, but terminals often struggle more. As an example,
Gnome Terminal version 3.38.1, shows “❤️” as a big red heart, but
shows "👨‍🦰" as “👨🦰”.





### `break_words`

```rust
fn break_words<'a, I>(words: I, line_width: usize) -> Vec<Word<'a>>
where
    I: IntoIterator<Item = Word<'a>>
```

Forcibly break words wider than `line_width` into smaller words.

This simply calls `Word::break_apart` on words that are too
wide. This means that no extra `'-'` is inserted, the word is
simply broken into smaller pieces.

## Constants

### `CSI`

```rust
const CSI: (char, char);
```

The CSI or “Control Sequence Introducer” introduces an ANSI escape
sequence. This is typically used for colored text and will be
ignored when computing the text width.

### `ANSI_FINAL_BYTE`

```rust
const ANSI_FINAL_BYTE: std::ops::RangeInclusive<char>;
```

The final bytes of an ANSI escape sequence must be in this range.

