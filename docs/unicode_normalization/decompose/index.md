*[unicode_normalization](../index.md) / [decompose](index.md)*

---

# Module `decompose`

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

External iterator for a string decomposition's characters.

#### Implementations

- `fn new_canonical(iter: I) -> Decompositions<I>` — [`Decompositions`](../index.md)

- `fn new_compatible(iter: I) -> Decompositions<I>` — [`Decompositions`](../index.md)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Decompositions<I>`

- `fn clone(self: &Self) -> Decompositions<I>` — [`Decompositions`](../index.md)

##### `impl<I: Iterator<Item = char> + Clone> Display for Decompositions<I>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Decompositions<I>`

##### `impl<I> IntoIterator for Decompositions<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Decompositions<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Decompositions<I>`

- `fn to_string(self: &Self) -> String`

##### `impl<I> UnicodeNormalization for Decompositions<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](../index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../index.md)

## Enums

### `DecompositionType`

```rust
enum DecompositionType {
    Canonical,
    Compatible,
}
```

#### Trait Implementations

##### `impl Clone for DecompositionType`

- `fn clone(self: &Self) -> DecompositionType` — [`DecompositionType`](#decompositiontype)

