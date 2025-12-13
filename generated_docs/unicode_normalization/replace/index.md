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

*Defined in [`unicode-normalization-0.1.25/src/replace.rs:18-23`](../../../.source_1765521767/unicode-normalization-0.1.25/src/replace.rs#L18-L23)*

External iterator for replacements for a string's characters.

#### Implementations

- <span id="replacements-new-cjk-compat-variants"></span>`fn new_cjk_compat_variants(iter: I) -> Replacements<I>` — [`Replacements`](#replacements)

  Create a new iterator that replaces [CJK Compatibility Ideograph] codepoints with normal forms using [Standardized Variation Sequences].

  

  Note that this iterator can also be obtained by directly calling `.cjk_compat_variants()` on the iterator.

  

  

#### Trait Implementations

##### `impl Any for Replacements<I>`

- <span id="replacements-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Replacements<I>`

- <span id="replacements-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Replacements<I>`

- <span id="replacements-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Replacements<I>`

- <span id="replacements-clone"></span>`fn clone(&self) -> Replacements<I>` — [`Replacements`](#replacements)

##### `impl CloneToUninit for Replacements<I>`

- <span id="replacements-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Iterator<Item = char> + Clone> Display for Replacements<I>`

- <span id="replacements-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Replacements<I>`

- <span id="replacements-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Replacements<I>`

##### `impl<U> Into for Replacements<I>`

- <span id="replacements-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Replacements<I>`

- <span id="replacements-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="replacements-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="replacements-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Replacements<I>`

- <span id="replacements-iterator-type-item"></span>`type Item = char`

- <span id="replacements-iterator-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="replacements-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Replacements<I>`

- <span id="replacements-toowned-type-owned"></span>`type Owned = T`

- <span id="replacements-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="replacements-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Replacements<I>`

- <span id="replacements-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Replacements<I>`

- <span id="replacements-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="replacements-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Replacements<I>`

- <span id="replacements-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="replacements-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for Replacements<I>`

- <span id="replacements-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="replacements-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="replacements-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="replacements-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="replacements-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](#replacements)

- <span id="replacements-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md#streamsafe)

