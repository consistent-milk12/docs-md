*[unicode_normalization](../index.md) / [recompose](index.md)*

---

# Module `recompose`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Recompositions`](#recompositions) | struct | External iterator for a string recomposition's characters. |
| [`RecompositionState`](#recompositionstate) | enum |  |

## Structs

### `Recompositions<I>`

```rust
struct Recompositions<I> {
    iter: crate::decompose::Decompositions<I>,
    state: RecompositionState,
    buffer: tinyvec::TinyVec<[char; 4]>,
    composee: Option<char>,
    last_ccc: Option<u8>,
}
```

*Defined in [`unicode-normalization-0.1.25/src/recompose.rs:27-33`](../../../.source_1765210505/unicode-normalization-0.1.25/src/recompose.rs#L27-L33)*

External iterator for a string recomposition's characters.

#### Implementations

- <span id="recompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Self`

- <span id="recompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Recompositions<I>`

- <span id="recompositions-clone"></span>`fn clone(&self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

##### `impl<I: Iterator<Item = char> + Clone> Display for Recompositions<I>`

- <span id="recompositions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Recompositions<I>`

##### `impl<I> IntoIterator for Recompositions<I>`

- <span id="recompositions-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="recompositions-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="recompositions-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Recompositions<I>`

- <span id="recompositions-iterator-type-item"></span>`type Item = char`

- <span id="recompositions-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl ToString for Recompositions<I>`

- <span id="recompositions-to-string"></span>`fn to_string(&self) -> String`

##### `impl<I> UnicodeNormalization for Recompositions<I>`

- <span id="recompositions-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="recompositions-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="recompositions-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="recompositions-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="recompositions-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../replace/index.md#replacements)

- <span id="recompositions-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md#streamsafe)

## Enums

### `RecompositionState`

```rust
enum RecompositionState {
    Composing,
    Purging(usize),
    Finished(usize),
}
```

*Defined in [`unicode-normalization-0.1.25/src/recompose.rs:19-23`](../../../.source_1765210505/unicode-normalization-0.1.25/src/recompose.rs#L19-L23)*

#### Trait Implementations

##### `impl Clone for RecompositionState`

- <span id="recompositionstate-clone"></span>`fn clone(&self) -> RecompositionState` — [`RecompositionState`](#recompositionstate)

