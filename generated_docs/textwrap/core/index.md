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
  - [`skip_ansi_escape_sequence`](#skip-ansi-escape-sequence)
  - [`ch_width`](#ch-width)
  - [`display_width`](#display-width)
  - [`break_words`](#break-words)
- [Constants](#constants)
  - [`CSI`](#csi)
  - [`ANSI_FINAL_BYTE`](#ansi-final-byte)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Word`](#word) | struct | A piece of wrappable text, including any trailing whitespace. |
| [`Fragment`](#fragment) | trait | A (text) fragment denotes the unit which we wrap into lines. |
| [`skip_ansi_escape_sequence`](#skip-ansi-escape-sequence) | fn | Skip ANSI escape sequences. |
| [`ch_width`](#ch-width) | fn |  |
| [`display_width`](#display-width) | fn | Compute the display width of `text` while skipping over ANSI escape sequences. |
| [`break_words`](#break-words) | fn | Forcibly break words wider than `line_width` into smaller words. |
| [`CSI`](#csi) | const | The CSI or “Control Sequence Introducer” introduces an ANSI escape sequence. |
| [`ANSI_FINAL_BYTE`](#ansi-final-byte) | const | The final bytes of an ANSI escape sequence must be in this range. |

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

*Defined in [`textwrap-0.16.2/src/core.rs:239-248`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L239-L248)*

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

  Construct a `Word` from a string.

  

  A trailing stretch of `' '` is automatically taken to be the

  whitespace part of the word.

- <span id="word-break-apart"></span>`fn break_apart<'b>(self: &'b Self, line_width: usize) -> impl Iterator<Item = Word<'a>> + 'b` — [`Word`](#word)

  Break this word into smaller words with a width of at most

  `line_width`. The whitespace and penalty from this `Word` is

  added to the last piece.

  

  # Examples

  

  ```rust

  use textwrap::core::Word;

  assert_eq!(

      Word::from("Hello!  ").break_apart(3).collect::<Vec<_>>(),

      vec![Word::from("Hel"), Word::from("lo!  ")]

  );

  ```

#### Trait Implementations

##### `impl Any for Word<'a>`

- <span id="word-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Word<'a>`

- <span id="word-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Word<'a>`

- <span id="word-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Word<'a>`

- <span id="word-clone"></span>`fn clone(&self) -> Word<'a>` — [`Word`](#word)

##### `impl CloneToUninit for Word<'a>`

- <span id="word-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Word<'a>`

##### `impl Debug for Word<'a>`

- <span id="word-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Word<'_>`

- <span id="word-deref-type-target"></span>`type Target = str`

- <span id="word-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for Word<'a>`

##### `impl Fragment for Word<'_>`

- <span id="word-fragment-width"></span>`fn width(&self) -> f64`

- <span id="word-fragment-whitespace-width"></span>`fn whitespace_width(&self) -> f64`

- <span id="word-fragment-penalty-width"></span>`fn penalty_width(&self) -> f64`

##### `impl<T> From for Word<'a>`

- <span id="word-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Word<'a>`

- <span id="word-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Word<'a>`

- <span id="word-partialeq-eq"></span>`fn eq(&self, other: &Word<'a>) -> bool` — [`Word`](#word)

##### `impl Receiver for Word<'a>`

- <span id="word-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Word<'a>`

##### `impl ToOwned for Word<'a>`

- <span id="word-toowned-type-owned"></span>`type Owned = T`

- <span id="word-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="word-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Word<'a>`

- <span id="word-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="word-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Word<'a>`

- <span id="word-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="word-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Fragment`

```rust
trait Fragment: std::fmt::Debug { ... }
```

*Defined in [`textwrap-0.16.2/src/core.rs:221-232`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L221-L232)*

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

#### Implementors

- [`Word`](#word)

## Functions

### `skip_ansi_escape_sequence`

```rust
fn skip_ansi_escape_sequence<I: Iterator<Item = char>>(ch: char, chars: &mut I) -> bool
```

*Defined in [`textwrap-0.16.2/src/core.rs:52-83`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L52-L83)*

Skip ANSI escape sequences.

The `ch` is the current `char`, the `chars` provide the following
characters. The `chars` will be modified if `ch` is the start of
an ANSI escape sequence.

Returns `true` if one or more chars were skipped.

### `ch_width`

```rust
fn ch_width(ch: char) -> usize
```

*Defined in [`textwrap-0.16.2/src/core.rs:87-89`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L87-L89)*

### `display_width`

```rust
fn display_width(text: &str) -> usize
```

*Defined in [`textwrap-0.16.2/src/core.rs:199-209`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L199-L209)*

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

*Defined in [`textwrap-0.16.2/src/core.rs:354-367`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L354-L367)*

Forcibly break words wider than `line_width` into smaller words.

This simply calls `Word::break_apart` on words that are too
wide. This means that no extra `'-'` is inserted, the word is
simply broken into smaller pieces.

## Constants

### `CSI`
```rust
const CSI: (char, char);
```

*Defined in [`textwrap-0.16.2/src/core.rs:40`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L40)*

The CSI or “Control Sequence Introducer” introduces an ANSI escape
sequence. This is typically used for colored text and will be
ignored when computing the text width.

### `ANSI_FINAL_BYTE`
```rust
const ANSI_FINAL_BYTE: std::ops::RangeInclusive<char>;
```

*Defined in [`textwrap-0.16.2/src/core.rs:42`](../../../.source_1765633015/textwrap-0.16.2/src/core.rs#L42)*

The final bytes of an ANSI escape sequence must be in this range.

