*[textwrap](../index.md) / [word_splitters](index.md)*

---

# Module `word_splitters`

Word splitting functionality.

To wrap text into lines, long words sometimes need to be split
across lines. The [`WordSplitter`](word_splitters/index.md) enum defines this
functionality.

## Enums

### `WordSplitter`

```rust
enum WordSplitter {
    NoHyphenation,
    HyphenSplitter,
    Custom(fn(&str) -> Vec<usize>),
}
```

The `WordSplitter` enum describes where words can be split.

If the textwrap crate has been compiled with the `hyphenation`
Cargo feature enabled, you will find a
`WordSplitter::Hyphenation` variant. Use this struct for
language-aware hyphenation:

```
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

Please see the documentation for the [hyphenation](#hyphenation)
 crate for more
details.

[hyphenation](#hyphenation)
: https://docs.rs/hyphenation/

#### Variants

- **`NoHyphenation`**

  Use this as a [`Options.word_splitter`](#optionsword-splitter) to avoid any kind of
  hyphenation:
  
  ```
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
  
  ```
  use textwrap::WordSplitter;
  
  assert_eq!(WordSplitter::HyphenSplitter.split_points("--foo-bar"),
             vec![6]);
  ```

- **`Custom`**

  Use a custom function as the word splitter.
  
  This variant lets you implement a custom word splitter using
  your own function.
  
  # Examples
  
  ```
  use textwrap::WordSplitter;
  
  fn split_at_underscore(word: &str) -> Vec<usize> {
      word.match_indices('_').map(|(idx, _)| idx + 1).collect()
  }
  
  let word_splitter = WordSplitter::Custom(split_at_underscore);
  assert_eq!(word_splitter.split_points("a_long_identifier"),
             vec![2, 7]);
  ```

#### Implementations

- `fn split_points(self: &Self, word: &str) -> Vec<usize>`
  Return all possible indices where `word` can be split.

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

##### `impl Clone`

- `fn clone(self: &Self) -> WordSplitter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &WordSplitter) -> bool`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `split_words`

```rust
fn split_words<'a, I>(words: I, word_splitter: &'a WordSplitter) -> impl Iterator<Item = crate::core::Word<'a>>
where
    I: IntoIterator<Item = crate::core::Word<'a>>
```

Split words into smaller words according to the split points given
by `word_splitter`.

Note that we split all words, regardless of their length. This is
to more cleanly separate the business of splitting (including
automatic hyphenation) from the business of word wrapping.

