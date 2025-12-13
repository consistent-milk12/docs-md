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

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:18-22`](../../../.source_1765633015/unicode-normalization-0.1.25/src/stream_safe.rs#L18-L22)*

[UAX15-D4]: This iterator keeps track of how many non-starters there have been
since the last starter in *NFKD* and will emit a Combining Grapheme Joiner
(U+034F) if the count exceeds 30.


#### Implementations

- <span id="streamsafe-new"></span>`fn new(iter: I) -> Self`

  Create a new stream safe iterator.

  

  Note that this iterator can also be obtained by directly calling [`.stream_safe()`](crate::UnicodeNormalization::stream_safe)

  on the iterator.

#### Trait Implementations

##### `impl Any for StreamSafe<I>`

- <span id="streamsafe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StreamSafe<I>`

- <span id="streamsafe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StreamSafe<I>`

- <span id="streamsafe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StreamSafe<I>`

- <span id="streamsafe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I>`

##### `impl<U> Into for StreamSafe<I>`

- <span id="streamsafe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for StreamSafe<I>`

- <span id="streamsafe-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamsafe-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamsafe-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for StreamSafe<I>`

- <span id="streamsafe-iterator-type-item"></span>`type Item = char`

- <span id="streamsafe-iterator-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl<U> TryFrom for StreamSafe<I>`

- <span id="streamsafe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streamsafe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StreamSafe<I>`

- <span id="streamsafe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streamsafe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for StreamSafe<I>`

- <span id="streamsafe-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="streamsafe-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="streamsafe-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="streamsafe-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="streamsafe-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../replace/index.md#replacements)

- <span id="streamsafe-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](#streamsafe)

### `Decomposition`

```rust
struct Decomposition {
    leading_nonstarters: usize,
    trailing_nonstarters: usize,
    decomposition_len: usize,
}
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:71-75`](../../../.source_1765633015/unicode-normalization-0.1.25/src/stream_safe.rs#L71-L75)*

#### Trait Implementations

##### `impl Any for Decomposition`

- <span id="decomposition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Decomposition`

- <span id="decomposition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Decomposition`

- <span id="decomposition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Decomposition`

- <span id="decomposition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Decomposition`

- <span id="decomposition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Decomposition`

- <span id="decomposition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Decomposition`

- <span id="decomposition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decomposition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Decomposition`

- <span id="decomposition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decomposition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `classify_nonstarters`

```rust
fn classify_nonstarters(c: char) -> Decomposition
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:78-112`](../../../.source_1765633015/unicode-normalization-0.1.25/src/stream_safe.rs#L78-L112)*

## Constants

### `MAX_NONSTARTERS`
```rust
const MAX_NONSTARTERS: usize = 30usize;
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:10`](../../../.source_1765633015/unicode-normalization-0.1.25/src/stream_safe.rs#L10)*

### `COMBINING_GRAPHEME_JOINER`
```rust
const COMBINING_GRAPHEME_JOINER: char = '\u{34f}';
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:11`](../../../.source_1765633015/unicode-normalization-0.1.25/src/stream_safe.rs#L11)*

