*[unicode_normalization](../index.md) / [replace](index.md)*

---

# Module `replace`

## Structs

### `Replacements<I>`

```rust
struct Replacements<I> {
    iter: I,
    buffer: Option<char>,
}
```

External iterator for replacements for a string's characters.

#### Implementations

- `fn new_cjk_compat_variants(iter: I) -> Replacements<I>` — [`Replacements`](#replacements)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Replacements<I>`

- `fn clone(self: &Self) -> Replacements<I>` — [`Replacements`](#replacements)

##### `impl<I: Iterator<Item = char> + Clone> Display for Replacements<I>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Replacements<I>`

##### `impl<I> IntoIterator for Replacements<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Replacements<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Replacements<I>`

- `fn to_string(self: &Self) -> String`

##### `impl<I> UnicodeNormalization for Replacements<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](#replacements)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md)

