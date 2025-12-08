*[unicode_normalization](../index.md) / [stream_safe](index.md)*

---

# Module `stream_safe`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StreamSafe`](#streamsafe) | struct | [UAX15-D4]: This iterator keeps track of how many non-starters there have been |
| [`Decomposition`](#decomposition) | struct |  |
| [`classify_nonstarters`](#classify_nonstarters) | fn |  |
| [`MAX_NONSTARTERS`](#max_nonstarters) | const |  |
| [`COMBINING_GRAPHEME_JOINER`](#combining_grapheme_joiner) | const |  |

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

- <span id="streamsafe-new"></span>`fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I>`

##### `impl<I> IntoIterator for StreamSafe<I>`

- <span id="streamsafe-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamsafe-intoiter"></span>`type IntoIter = I`

- <span id="streamsafe-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for StreamSafe<I>`

- <span id="streamsafe-item"></span>`type Item = char`

- <span id="streamsafe-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl<I> UnicodeNormalization for StreamSafe<I>`

- <span id="streamsafe-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- <span id="streamsafe-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- <span id="streamsafe-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- <span id="streamsafe-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- <span id="streamsafe-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../index.md)

- <span id="streamsafe-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](../index.md)

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

- <span id="decomposition-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

