*[unicode_normalization](../index.md) / [stream_safe](index.md)*

---

# Module `stream_safe`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StreamSafe`](#streamsafe) | struct | [UAX15-D4]: This iterator keeps track of how many non-starters there have been since the last starter in *NFKD* and will emit a Combining Grapheme Joiner (U+034F) if the count exceeds 30. |
| [`Decomposition`](#decomposition) | struct |  |
| [`classify_nonstarters`](#classify-nonstarters) | fn |  |
| [`MAX_NONSTARTERS`](#max-nonstarters) | const |  |
| [`COMBINING_GRAPHEME_JOINER`](#combining-grapheme-joiner) | const |  |

## Structs

### `StreamSafe<I>`

```rust
struct StreamSafe<I> {
    iter: I,
    nonstarter_count: usize,
    buffer: Option<char>,
}
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:18-22`](../../../.source_1765210505/unicode-normalization-0.1.25/src/stream_safe.rs#L18-L22)*

[UAX15-D4]: This iterator keeps track of how many non-starters there have been
since the last starter in *NFKD* and will emit a Combining Grapheme Joiner
(U+034F) if the count exceeds 30.


#### Implementations

- <span id="streamsafe-new"></span>`fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I>`

##### `impl<I> IntoIterator for StreamSafe<I>`

- <span id="streamsafe-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamsafe-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamsafe-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for StreamSafe<I>`

- <span id="streamsafe-iterator-type-item"></span>`type Item = char`

- <span id="streamsafe-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl<I> UnicodeNormalization for StreamSafe<I>`

- <span id="streamsafe-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="streamsafe-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="streamsafe-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="streamsafe-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="streamsafe-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../replace/index.md#replacements)

- <span id="streamsafe-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](#streamsafe)

### `Decomposition`

```rust
struct Decomposition {
    leading_nonstarters: usize,
    trailing_nonstarters: usize,
    decomposition_len: usize,
}
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:71-75`](../../../.source_1765210505/unicode-normalization-0.1.25/src/stream_safe.rs#L71-L75)*

#### Trait Implementations

##### `impl Debug for Decomposition`

- <span id="decomposition-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `classify_nonstarters`

```rust
fn classify_nonstarters(c: char) -> Decomposition
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:78-112`](../../../.source_1765210505/unicode-normalization-0.1.25/src/stream_safe.rs#L78-L112)*

## Constants

### `MAX_NONSTARTERS`
```rust
const MAX_NONSTARTERS: usize = 30usize;
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:10`](../../../.source_1765210505/unicode-normalization-0.1.25/src/stream_safe.rs#L10)*

### `COMBINING_GRAPHEME_JOINER`
```rust
const COMBINING_GRAPHEME_JOINER: char = '\u{34f}';
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:11`](../../../.source_1765210505/unicode-normalization-0.1.25/src/stream_safe.rs#L11)*

