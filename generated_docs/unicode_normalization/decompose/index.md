*[unicode_normalization](../index.md) / [decompose](index.md)*

---

# Module `decompose`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Decompositions`](#decompositions) | struct | External iterator for a string decomposition's characters. |
| [`DecompositionType`](#decompositiontype) | enum |  |

## Structs

### `Decompositions<I>`

```rust
struct Decompositions<I> {
    kind: DecompositionType,
    iter: core::iter::Fuse<I>,
    buffer: tinyvec::TinyVec<[(u8, char); 4]>,
    ready: core::ops::Range<usize>,
}
```

*Defined in [`unicode-normalization-0.1.25/src/decompose.rs:23-37`](../../../.source_1765210505/unicode-normalization-0.1.25/src/decompose.rs#L23-L37)*

External iterator for a string decomposition's characters.

#### Implementations

- <span id="decompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Decompositions<I>` — [`Decompositions`](#decompositions)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Decompositions<I>`

- <span id="decompositions-clone"></span>`fn clone(&self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

##### `impl<I: Iterator<Item = char> + Clone> Display for Decompositions<I>`

- <span id="decompositions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Decompositions<I>`

##### `impl<I> IntoIterator for Decompositions<I>`

- <span id="decompositions-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="decompositions-type-intoiter"></span>`type IntoIter = I`

- <span id="decompositions-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Decompositions<I>`

- <span id="decompositions-type-item"></span>`type Item = char`

- <span id="decompositions-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="decompositions-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Decompositions<I>`

- <span id="decompositions-to-string"></span>`fn to_string(&self) -> String`

##### `impl<I> UnicodeNormalization for Decompositions<I>`

- <span id="decompositions-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="decompositions-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="decompositions-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../replace/index.md#replacements)

- <span id="decompositions-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md#streamsafe)

## Enums

### `DecompositionType`

```rust
enum DecompositionType {
    Canonical,
    Compatible,
}
```

*Defined in [`unicode-normalization-0.1.25/src/decompose.rs:16-19`](../../../.source_1765210505/unicode-normalization-0.1.25/src/decompose.rs#L16-L19)*

#### Trait Implementations

##### `impl Clone for DecompositionType`

- <span id="decompositiontype-clone"></span>`fn clone(&self) -> DecompositionType` — [`DecompositionType`](#decompositiontype)

