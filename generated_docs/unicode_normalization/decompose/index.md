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

*Defined in [`unicode-normalization-0.1.25/src/decompose.rs:23-37`](../../../.source_1765521767/unicode-normalization-0.1.25/src/decompose.rs#L23-L37)*

External iterator for a string decomposition's characters.

#### Implementations

- <span id="decompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Decompositions<I>` — [`Decompositions`](#decompositions)

  Create a new decomposition iterator for canonical decompositions (NFD)

  

  Note that this iterator can also be obtained by directly calling [`.nfd()`](crate::UnicodeNormalization::nfd)

  on the iterator.

- <span id="decompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Decompositions<I>` — [`Decompositions`](#decompositions)

  Create a new decomposition iterator for compatability decompositions (NFkD)

  

  Note that this iterator can also be obtained by directly calling [`.nfkd()`](crate::UnicodeNormalization::nfkd)

  on the iterator.

#### Trait Implementations

##### `impl Any for Decompositions<I>`

- <span id="decompositions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Decompositions<I>`

- <span id="decompositions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Decompositions<I>`

- <span id="decompositions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Decompositions<I>`

- <span id="decompositions-clone"></span>`fn clone(&self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

##### `impl CloneToUninit for Decompositions<I>`

- <span id="decompositions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Iterator<Item = char> + Clone> Display for Decompositions<I>`

- <span id="decompositions-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Decompositions<I>`

- <span id="decompositions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Decompositions<I>`

##### `impl<U> Into for Decompositions<I>`

- <span id="decompositions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Decompositions<I>`

- <span id="decompositions-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="decompositions-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="decompositions-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Decompositions<I>`

- <span id="decompositions-iterator-type-item"></span>`type Item = char`

- <span id="decompositions-iterator-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="decompositions-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Decompositions<I>`

- <span id="decompositions-toowned-type-owned"></span>`type Owned = T`

- <span id="decompositions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="decompositions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Decompositions<I>`

- <span id="decompositions-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Decompositions<I>`

- <span id="decompositions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decompositions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Decompositions<I>`

- <span id="decompositions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decompositions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for Decompositions<I>`

- <span id="decompositions-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="decompositions-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md#recompositions)

- <span id="decompositions-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../replace/index.md#replacements)

- <span id="decompositions-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md#streamsafe)

## Enums

### `DecompositionType`

```rust
enum DecompositionType {
    Canonical,
    Compatible,
}
```

*Defined in [`unicode-normalization-0.1.25/src/decompose.rs:16-19`](../../../.source_1765521767/unicode-normalization-0.1.25/src/decompose.rs#L16-L19)*

#### Trait Implementations

##### `impl Any for DecompositionType`

- <span id="decompositiontype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DecompositionType`

- <span id="decompositiontype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DecompositionType`

- <span id="decompositiontype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DecompositionType`

- <span id="decompositiontype-clone"></span>`fn clone(&self) -> DecompositionType` — [`DecompositionType`](#decompositiontype)

##### `impl CloneToUninit for DecompositionType`

- <span id="decompositiontype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for DecompositionType`

- <span id="decompositiontype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DecompositionType`

- <span id="decompositiontype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DecompositionType`

- <span id="decompositiontype-toowned-type-owned"></span>`type Owned = T`

- <span id="decompositiontype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="decompositiontype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DecompositionType`

- <span id="decompositiontype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decompositiontype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DecompositionType`

- <span id="decompositiontype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decompositiontype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

