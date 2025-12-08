*[unicode_normalization](../index.md) / [stream_safe](index.md)*

---

# Module `stream_safe`

## Structs

### `StreamSafe<I>`

```rust
struct StreamSafe<I> {
    iter: I,
    nonstarter_count: usize,
    buffer: Option<char>,
}
```

[UAX15-D4]: This iterator keeps track of how many non-starters there have been
since the last starter in *NFKD* and will emit a Combining Grapheme Joiner
(U+034F) if the count exceeds 30.


#### Implementations

- `fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I>`

##### `impl<I> IntoIterator for StreamSafe<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for StreamSafe<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

##### `impl<I> UnicodeNormalization for StreamSafe<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](../index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../index.md)

### `Decomposition`

```rust
struct Decomposition {
    leading_nonstarters: usize,
    trailing_nonstarters: usize,
    decomposition_len: usize,
}
```

#### Trait Implementations

##### `impl Debug for Decomposition`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `classify_nonstarters`

```rust
fn classify_nonstarters(c: char) -> Decomposition
```

## Constants

### `MAX_NONSTARTERS`

```rust
const MAX_NONSTARTERS: usize = 30usize;
```

### `COMBINING_GRAPHEME_JOINER`

```rust
const COMBINING_GRAPHEME_JOINER: char = '\u{34f}';
```

