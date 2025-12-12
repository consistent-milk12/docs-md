*[textwrap](../index.md) / [word_splitters](index.md)*

---

# Module `word_splitters`

Word splitting functionality.

To wrap text into lines, long words sometimes need to be split
across lines. The [`WordSplitter`](#wordsplitter) enum defines this
functionality.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WordSplitter`](#wordsplitter) | enum | The `WordSplitter` enum describes where words can be split. |
| [`split_words`](#split-words) | fn | Split words into smaller words according to the split points given by `word_splitter`. |

## Enums

### `WordSplitter`

```rust
enum WordSplitter {
    NoHyphenation,
    HyphenSplitter,
    Custom(fn(&str) -> Vec<usize>),
}
```

*Defined in [`textwrap-0.16.2/src/word_splitters.rs:37-99`](../../../.source_1765210505/textwrap-0.16.2/src/word_splitters.rs#L37-L99)*

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

- <span id="wordsplitter-clone"></span>`fn clone(&self) -> WordSplitter` — [`WordSplitter`](#wordsplitter)

##### `impl Debug for WordSplitter`

- <span id="wordsplitter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for WordSplitter`

- <span id="wordsplitter-eq"></span>`fn eq(&self, other: &WordSplitter) -> bool` — [`WordSplitter`](#wordsplitter)

## Functions

### `split_words`

```rust
fn split_words<'a, I>(words: I, word_splitter: &'a WordSplitter) -> impl Iterator<Item = crate::core::Word<'a>>
where
    I: IntoIterator<Item = crate::core::Word<'a>>
```

*Defined in [`textwrap-0.16.2/src/word_splitters.rs:169-206`](../../../.source_1765210505/textwrap-0.16.2/src/word_splitters.rs#L169-L206)*

Split words into smaller words according to the split points given
by `word_splitter`.

Note that we split all words, regardless of their length. This is
to more cleanly separate the business of splitting (including
automatic hyphenation) from the business of word wrapping.

