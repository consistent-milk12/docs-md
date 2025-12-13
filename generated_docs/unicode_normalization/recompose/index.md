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

*Defined in [`unicode-normalization-0.1.25/src/recompose.rs:27-33`](../../../.source_1765521767/unicode-normalization-0.1.25/src/recompose.rs#L27-L33)*

External iterator for a string recomposition's characters.

#### Implementations

- <span id="recompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Self`

  Create a new recomposition iterator for canonical compositions (NFC)

  

  Note that this iterator can also be obtained by directly calling [`.nfc()`](crate::UnicodeNormalization::nfc)

  on the iterator.

- <span id="recompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Self`

  Create a new recomposition iterator for compatability compositions (NFkC)

  

  Note that this iterator can also be obtained by directly calling [`.nfkc()`](crate::UnicodeNormalization::nfkc)

  on the iterator.

#### Trait Implementations

##### `impl Any for Recompositions<I>`

- <span id="recompositions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Recompositions<I>`

- <span id="recompositions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Recompositions<I>`

- <span id="recompositions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Recompositions<I>`

- <span id="recompositions-clone"></span>`fn clone(&self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

##### `impl CloneToUninit for Recompositions<I>`

- <span id="recompositions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Iterator<Item = char> + Clone> Display for Recompositions<I>`

- <span id="recompositions-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Recompositions<I>`

- <span id="recompositions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Recompositions<I>`

##### `impl<U> Into for Recompositions<I>`

- <span id="recompositions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Recompositions<I>`

- <span id="recompositions-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="recompositions-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="recompositions-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Recompositions<I>`

- <span id="recompositions-iterator-type-item"></span>`type Item = char`

- <span id="recompositions-iterator-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl ToOwned for Recompositions<I>`

- <span id="recompositions-toowned-type-owned"></span>`type Owned = T`

- <span id="recompositions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="recompositions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Recompositions<I>`

- <span id="recompositions-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Recompositions<I>`

- <span id="recompositions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="recompositions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Recompositions<I>`

- <span id="recompositions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="recompositions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for Recompositions<I>`

- <span id="recompositions-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="recompositions-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md#decompositions)

- <span id="recompositions-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="recompositions-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="recompositions-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](../replace/index.md#replacements)

- <span id="recompositions-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md#streamsafe)

## Enums

### `RecompositionState`

```rust
enum RecompositionState {
    Composing,
    Purging(usize),
    Finished(usize),
}
```

*Defined in [`unicode-normalization-0.1.25/src/recompose.rs:19-23`](../../../.source_1765521767/unicode-normalization-0.1.25/src/recompose.rs#L19-L23)*

#### Trait Implementations

##### `impl Any for RecompositionState`

- <span id="recompositionstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RecompositionState`

- <span id="recompositionstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RecompositionState`

- <span id="recompositionstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RecompositionState`

- <span id="recompositionstate-clone"></span>`fn clone(&self) -> RecompositionState` — [`RecompositionState`](#recompositionstate)

##### `impl CloneToUninit for RecompositionState`

- <span id="recompositionstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for RecompositionState`

- <span id="recompositionstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RecompositionState`

- <span id="recompositionstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RecompositionState`

- <span id="recompositionstate-toowned-type-owned"></span>`type Owned = T`

- <span id="recompositionstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="recompositionstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RecompositionState`

- <span id="recompositionstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="recompositionstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RecompositionState`

- <span id="recompositionstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="recompositionstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

