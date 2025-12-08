*[unicode_normalization](../index.md) / [replace](index.md)*

---

# Module `replace`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Replacements`](#replacements) | struct | External iterator for replacements for a string's characters. |

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

- <span id="replacements-new-cjk-compat-variants"></span>`fn new_cjk_compat_variants(iter: I) -> Replacements<I>` — [`Replacements`](../index.md)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Replacements<I>`

- <span id="replacements-clone"></span>`fn clone(&self) -> Replacements<I>` — [`Replacements`](../index.md)

##### `impl<I: Iterator<Item = char> + Clone> Display for Replacements<I>`

- <span id="replacements-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Replacements<I>`

##### `impl<I> IntoIterator for Replacements<I>`

- <span id="replacements-item"></span>`type Item = <I as Iterator>::Item`

- <span id="replacements-intoiter"></span>`type IntoIter = I`

- <span id="replacements-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Replacements<I>`

- <span id="replacements-item"></span>`type Item = char`

- <span id="replacements-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="replacements-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Replacements<I>`

- <span id="replacements-to-string"></span>`fn to_string(&self) -> String`

##### `impl<I> UnicodeNormalization for Replacements<I>`

- <span id="replacements-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- <span id="replacements-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](../index.md)

- <span id="replacements-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- <span id="replacements-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](../index.md)

- <span id="replacements-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../index.md)

- <span id="replacements-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](../index.md)

