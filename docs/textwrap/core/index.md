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

## Structs

### `Word<'a>`

```rust
struct Word<'a> {
    pub word: &'a str,
    pub whitespace: &'a str,
    pub penalty: &'a str,
    // [REDACTED: Private Fields]
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

- `fn from(word: &str) -> Word<'_>`
  Construct a `Word` from a string.

- `fn break_apart<'b>(self: &'b Self, line_width: usize) -> impl Iterator<Item = Word<'a>> + 'b`
  Break this word into smaller words with a width of at most

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Word<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl Eq<'a>`

##### `impl Fragment`

- `fn width(self: &Self) -> f64`

- `fn whitespace_width(self: &Self) -> f64`

- `fn penalty_width(self: &Self) -> f64`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &Word<'a>) -> bool`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq<'a>`

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

##### `impl Deref`

- `type Target = str`

- `fn deref(self: &Self) -> &<Self as >::Target`

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

- `fn width(self: &Self) -> f64`

  Displayed width of word represented by this fragment.

- `fn whitespace_width(self: &Self) -> f64`

  Displayed width of the whitespace that must follow the word

- `fn penalty_width(self: &Self) -> f64`

  Displayed width of the penalty that must be inserted if the

## Functions

### `display_width`

```rust
fn display_width(text: &str) -> usize
```

Compute the display width of `text` while skipping over ANSI
escape sequences.

# Examples

```
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

```
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

```
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

```
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

[combining characters]: https://en.wikipedia.org/wiki/Combining_character
[Unicode equivalence]: https://en.wikipedia.org/wiki/Unicode_equivalence
[CJK characters]: https://en.wikipedia.org/wiki/CJK_characters
[emoji modifier sequences]: https://unicode.org/emoji/charts/full-emoji-modifiers.html

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

