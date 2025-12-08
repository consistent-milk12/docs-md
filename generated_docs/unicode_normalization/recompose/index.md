*[unicode_normalization](../index.md) / [recompose](index.md)*

---

# Module `recompose`

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

External iterator for a string recomposition's characters.

#### Implementations

- `fn new_canonical(iter: I) -> Self`

- `fn new_compatible(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Recompositions<I>`

- `fn clone(self: &Self) -> Recompositions<I>` — [`Recompositions`](../index.md)

##### `impl<I: Iterator<Item = char> + Clone> Display for Recompositions<I>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Recompositions<I>`

##### `impl<I> IntoIterator for Recompositions<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Recompositions<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

##### `impl<T> ToString for Recompositions<I>`

- `fn to_string(self: &Self) -> String`

##### `impl<I> UnicodeNormalization for Recompositions<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](../index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../index.md)

## Enums

### `RecompositionState`

```rust
enum RecompositionState {
    Composing,
    Purging(usize),
    Finished(usize),
}
```

#### Trait Implementations

##### `impl Clone for RecompositionState`

- `fn clone(self: &Self) -> RecompositionState` — [`RecompositionState`](#recompositionstate)

