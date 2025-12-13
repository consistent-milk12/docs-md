*[rayon](../index.md) / [str](index.md)*

---

# Module `str`

Parallel iterator types for [strings]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

Note: `ParallelString::par_split()` and `par_split_terminator()`
reference a `Pattern` trait which is not visible outside this crate.
This trait is intentionally kept private, for use only by Rayon itself.
It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
and any function or closure `F: Fn(char) -> bool + Sync + Send`.



## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Chars`](#chars)
  - [`CharsProducer`](#charsproducer)
  - [`CharIndices`](#charindices)
  - [`CharIndicesProducer`](#charindicesproducer)
  - [`Bytes`](#bytes)
  - [`BytesProducer`](#bytesproducer)
  - [`EncodeUtf16`](#encodeutf16)
  - [`EncodeUtf16Producer`](#encodeutf16producer)
  - [`Split`](#split)
  - [`SplitInclusive`](#splitinclusive)
  - [`SplitTerminator`](#splitterminator)
  - [`SplitTerminatorProducer`](#splitterminatorproducer)
  - [`Lines`](#lines)
  - [`SplitWhitespace`](#splitwhitespace)
  - [`SplitAsciiWhitespace`](#splitasciiwhitespace)
  - [`Matches`](#matches)
  - [`MatchesProducer`](#matchesproducer)
  - [`MatchIndices`](#matchindices)
  - [`MatchIndicesProducer`](#matchindicesproducer)
- [Traits](#traits)
  - [`ParallelString`](#parallelstring)
- [Functions](#functions)
  - [`is_char_boundary`](#is-char-boundary)
  - [`find_char_midpoint`](#find-char-midpoint)
  - [`split`](#split)
  - [`offset`](#offset)
  - [`no_carriage_return`](#no-carriage-return)
  - [`not_empty`](#not-empty)
  - [`is_ascii_whitespace`](#is-ascii-whitespace)
- [Macros](#macros)
  - [`impl_pattern!`](#impl-pattern)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | We hide the `Pattern` trait in a private module, as its API is not meant for general consumption. |
| [`Chars`](#chars) | struct | Parallel iterator over the characters of a string |
| [`CharsProducer`](#charsproducer) | struct |  |
| [`CharIndices`](#charindices) | struct | Parallel iterator over the characters of a string, with their positions |
| [`CharIndicesProducer`](#charindicesproducer) | struct |  |
| [`Bytes`](#bytes) | struct | Parallel iterator over the bytes of a string |
| [`BytesProducer`](#bytesproducer) | struct |  |
| [`EncodeUtf16`](#encodeutf16) | struct | Parallel iterator over a string encoded as UTF-16 |
| [`EncodeUtf16Producer`](#encodeutf16producer) | struct |  |
| [`Split`](#split) | struct | Parallel iterator over substrings separated by a pattern |
| [`SplitInclusive`](#splitinclusive) | struct | Parallel iterator over substrings separated by a pattern |
| [`SplitTerminator`](#splitterminator) | struct | Parallel iterator over substrings separated by a terminator pattern |
| [`SplitTerminatorProducer`](#splitterminatorproducer) | struct |  |
| [`Lines`](#lines) | struct | Parallel iterator over lines in a string |
| [`SplitWhitespace`](#splitwhitespace) | struct | Parallel iterator over substrings separated by whitespace |
| [`SplitAsciiWhitespace`](#splitasciiwhitespace) | struct | Parallel iterator over substrings separated by ASCII whitespace |
| [`Matches`](#matches) | struct | Parallel iterator over substrings that match a pattern |
| [`MatchesProducer`](#matchesproducer) | struct |  |
| [`MatchIndices`](#matchindices) | struct | Parallel iterator over substrings that match a pattern, with their positions |
| [`MatchIndicesProducer`](#matchindicesproducer) | struct |  |
| [`ParallelString`](#parallelstring) | trait | Parallel extensions for strings. |
| [`is_char_boundary`](#is-char-boundary) | fn | Test if a byte is the start of a UTF-8 character. |
| [`find_char_midpoint`](#find-char-midpoint) | fn | Find the index of a character boundary near the midpoint. |
| [`split`](#split) | fn | Try to split a string near the midpoint. |
| [`offset`](#offset) | fn |  |
| [`no_carriage_return`](#no-carriage-return) | fn |  |
| [`not_empty`](#not-empty) | fn |  |
| [`is_ascii_whitespace`](#is-ascii-whitespace) | fn |  |
| [`impl_pattern!`](#impl-pattern) | macro |  |

## Modules

- [`private`](private/index.md) — We hide the `Pattern` trait in a private module, as its API is not meant

## Structs

### `Chars<'ch>`

```rust
struct Chars<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:467-469`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L467-L469)*

Parallel iterator over the characters of a string

#### Trait Implementations

##### `impl Any for Chars<'ch>`

- <span id="chars-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chars<'ch>`

- <span id="chars-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chars<'ch>`

- <span id="chars-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Chars<'ch>`

- <span id="chars-clone"></span>`fn clone(&self) -> Chars<'ch>` — [`Chars`](#chars)

##### `impl CloneToUninit for Chars<'ch>`

- <span id="chars-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Chars<'ch>`

- <span id="chars-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Chars<'ch>`

- <span id="chars-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Chars<'ch>`

- <span id="chars-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chars<'ch>`

##### `impl IntoParallelIterator for Chars<'ch>`

- <span id="chars-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chars-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chars-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Chars<'ch>`

- <span id="chars-paralleliterator-type-item"></span>`type Item = char`

- <span id="chars-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Chars<'ch>`

- <span id="chars-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chars-pointable-type-init"></span>`type Init = T`

- <span id="chars-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chars-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chars-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chars-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Chars<'ch>`

- <span id="chars-toowned-type-owned"></span>`type Owned = T`

- <span id="chars-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chars-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chars<'ch>`

- <span id="chars-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chars-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chars<'ch>`

- <span id="chars-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chars-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CharsProducer<'ch>`

```rust
struct CharsProducer<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:471-473`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L471-L473)*

#### Trait Implementations

##### `impl Any for CharsProducer<'ch>`

- <span id="charsproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CharsProducer<'ch>`

- <span id="charsproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CharsProducer<'ch>`

- <span id="charsproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for CharsProducer<'ch>`

- <span id="charsproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CharsProducer<'ch>`

- <span id="charsproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CharsProducer<'ch>`

##### `impl Pointable for CharsProducer<'ch>`

- <span id="charsproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="charsproducer-pointable-type-init"></span>`type Init = T`

- <span id="charsproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="charsproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="charsproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="charsproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CharsProducer<'ch>`

- <span id="charsproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="charsproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CharsProducer<'ch>`

- <span id="charsproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="charsproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnindexedProducer for CharsProducer<'ch>`

- <span id="charsproducer-unindexedproducer-type-item"></span>`type Item = char`

- <span id="charsproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="charsproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `CharIndices<'ch>`

```rust
struct CharIndices<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:511-513`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L511-L513)*

Parallel iterator over the characters of a string, with their positions

#### Trait Implementations

##### `impl Any for CharIndices<'ch>`

- <span id="charindices-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CharIndices<'ch>`

- <span id="charindices-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CharIndices<'ch>`

- <span id="charindices-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CharIndices<'ch>`

- <span id="charindices-clone"></span>`fn clone(&self) -> CharIndices<'ch>` — [`CharIndices`](#charindices)

##### `impl CloneToUninit for CharIndices<'ch>`

- <span id="charindices-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CharIndices<'ch>`

- <span id="charindices-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CharIndices<'ch>`

- <span id="charindices-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CharIndices<'ch>`

- <span id="charindices-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CharIndices<'ch>`

##### `impl IntoParallelIterator for CharIndices<'ch>`

- <span id="charindices-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="charindices-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="charindices-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for CharIndices<'ch>`

- <span id="charindices-paralleliterator-type-item"></span>`type Item = (usize, char)`

- <span id="charindices-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for CharIndices<'ch>`

- <span id="charindices-pointable-const-align"></span>`const ALIGN: usize`

- <span id="charindices-pointable-type-init"></span>`type Init = T`

- <span id="charindices-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="charindices-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="charindices-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="charindices-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for CharIndices<'ch>`

- <span id="charindices-toowned-type-owned"></span>`type Owned = T`

- <span id="charindices-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="charindices-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CharIndices<'ch>`

- <span id="charindices-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="charindices-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CharIndices<'ch>`

- <span id="charindices-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="charindices-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CharIndicesProducer<'ch>`

```rust
struct CharIndicesProducer<'ch> {
    index: usize,
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:515-518`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L515-L518)*

#### Trait Implementations

##### `impl Any for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CharIndicesProducer<'ch>`

##### `impl Pointable for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="charindicesproducer-pointable-type-init"></span>`type Init = T`

- <span id="charindicesproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="charindicesproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="charindicesproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="charindicesproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="charindicesproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="charindicesproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnindexedProducer for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-unindexedproducer-type-item"></span>`type Item = (usize, char)`

- <span id="charindicesproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="charindicesproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `Bytes<'ch>`

```rust
struct Bytes<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:567-569`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L567-L569)*

Parallel iterator over the bytes of a string

#### Trait Implementations

##### `impl Any for Bytes<'ch>`

- <span id="bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bytes<'ch>`

- <span id="bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bytes<'ch>`

- <span id="bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Bytes<'ch>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'ch>` — [`Bytes`](#bytes)

##### `impl CloneToUninit for Bytes<'ch>`

- <span id="bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Bytes<'ch>`

- <span id="bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Bytes<'ch>`

- <span id="bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bytes<'ch>`

- <span id="bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Bytes<'ch>`

##### `impl IntoParallelIterator for Bytes<'ch>`

- <span id="bytes-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="bytes-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="bytes-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Bytes<'ch>`

- <span id="bytes-paralleliterator-type-item"></span>`type Item = u8`

- <span id="bytes-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Bytes<'ch>`

- <span id="bytes-pointable-const-align"></span>`const ALIGN: usize`

- <span id="bytes-pointable-type-init"></span>`type Init = T`

- <span id="bytes-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="bytes-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="bytes-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="bytes-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Bytes<'ch>`

- <span id="bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Bytes<'ch>`

- <span id="bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bytes<'ch>`

- <span id="bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BytesProducer<'ch>`

```rust
struct BytesProducer<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:571-573`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L571-L573)*

#### Trait Implementations

##### `impl Any for BytesProducer<'ch>`

- <span id="bytesproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BytesProducer<'ch>`

- <span id="bytesproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BytesProducer<'ch>`

- <span id="bytesproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for BytesProducer<'ch>`

- <span id="bytesproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BytesProducer<'ch>`

- <span id="bytesproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for BytesProducer<'ch>`

##### `impl Pointable for BytesProducer<'ch>`

- <span id="bytesproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="bytesproducer-pointable-type-init"></span>`type Init = T`

- <span id="bytesproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="bytesproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="bytesproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="bytesproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for BytesProducer<'ch>`

- <span id="bytesproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytesproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BytesProducer<'ch>`

- <span id="bytesproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytesproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnindexedProducer for BytesProducer<'ch>`

- <span id="bytesproducer-unindexedproducer-type-item"></span>`type Item = u8`

- <span id="bytesproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="bytesproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `EncodeUtf16<'ch>`

```rust
struct EncodeUtf16<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:611-613`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L611-L613)*

Parallel iterator over a string encoded as UTF-16

#### Trait Implementations

##### `impl Any for EncodeUtf16<'ch>`

- <span id="encodeutf16-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EncodeUtf16<'ch>`

- <span id="encodeutf16-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EncodeUtf16<'ch>`

- <span id="encodeutf16-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EncodeUtf16<'ch>`

- <span id="encodeutf16-clone"></span>`fn clone(&self) -> EncodeUtf16<'ch>` — [`EncodeUtf16`](#encodeutf16)

##### `impl CloneToUninit for EncodeUtf16<'ch>`

- <span id="encodeutf16-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for EncodeUtf16<'ch>`

- <span id="encodeutf16-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EncodeUtf16<'ch>`

- <span id="encodeutf16-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EncodeUtf16<'ch>`

- <span id="encodeutf16-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for EncodeUtf16<'ch>`

##### `impl IntoParallelIterator for EncodeUtf16<'ch>`

- <span id="encodeutf16-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="encodeutf16-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="encodeutf16-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for EncodeUtf16<'ch>`

- <span id="encodeutf16-paralleliterator-type-item"></span>`type Item = u16`

- <span id="encodeutf16-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for EncodeUtf16<'ch>`

- <span id="encodeutf16-pointable-const-align"></span>`const ALIGN: usize`

- <span id="encodeutf16-pointable-type-init"></span>`type Init = T`

- <span id="encodeutf16-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="encodeutf16-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="encodeutf16-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="encodeutf16-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for EncodeUtf16<'ch>`

- <span id="encodeutf16-toowned-type-owned"></span>`type Owned = T`

- <span id="encodeutf16-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="encodeutf16-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EncodeUtf16<'ch>`

- <span id="encodeutf16-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="encodeutf16-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EncodeUtf16<'ch>`

- <span id="encodeutf16-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="encodeutf16-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EncodeUtf16Producer<'ch>`

```rust
struct EncodeUtf16Producer<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:615-617`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L615-L617)*

#### Trait Implementations

##### `impl Any for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for EncodeUtf16Producer<'ch>`

##### `impl Pointable for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="encodeutf16producer-pointable-type-init"></span>`type Init = T`

- <span id="encodeutf16producer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="encodeutf16producer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="encodeutf16producer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="encodeutf16producer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="encodeutf16producer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="encodeutf16producer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnindexedProducer for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-unindexedproducer-type-item"></span>`type Item = u16`

- <span id="encodeutf16producer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="encodeutf16producer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `Split<'ch, P: Pattern>`

```rust
struct Split<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:655-658`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L655-L658)*

Parallel iterator over substrings separated by a pattern

#### Implementations

- <span id="split-new"></span>`fn new(chars: &'ch str, separator: P) -> Self`

#### Trait Implementations

##### `impl Any for Split<'ch, P>`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split<'ch, P>`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split<'ch, P>`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone + Pattern> Clone for Split<'ch, P>`

- <span id="split-clone"></span>`fn clone(&self) -> Split<'ch, P>` — [`Split`](#split)

##### `impl CloneToUninit for Split<'ch, P>`

- <span id="split-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug + Pattern> Debug for Split<'ch, P>`

- <span id="split-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Split<'ch, P>`

- <span id="split-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Split<'ch, P>`

- <span id="split-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Split<'ch, P>`

##### `impl IntoParallelIterator for Split<'ch, P>`

- <span id="split-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="split-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="split-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<P: Pattern> ParallelIterator for Split<'ch, P>`

- <span id="split-paralleliterator-type-item"></span>`type Item = &'ch str`

- <span id="split-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Split<'ch, P>`

- <span id="split-pointable-const-align"></span>`const ALIGN: usize`

- <span id="split-pointable-type-init"></span>`type Init = T`

- <span id="split-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="split-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="split-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="split-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Split<'ch, P>`

- <span id="split-toowned-type-owned"></span>`type Owned = T`

- <span id="split-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="split-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Split<'ch, P>`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Split<'ch, P>`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitInclusive<'ch, P: Pattern>`

```rust
struct SplitInclusive<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:727-730`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L727-L730)*

Parallel iterator over substrings separated by a pattern

#### Implementations

- <span id="splitinclusive-new"></span>`fn new(chars: &'ch str, separator: P) -> Self`

#### Trait Implementations

##### `impl Any for SplitInclusive<'ch, P>`

- <span id="splitinclusive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitInclusive<'ch, P>`

- <span id="splitinclusive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitInclusive<'ch, P>`

- <span id="splitinclusive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone + Pattern> Clone for SplitInclusive<'ch, P>`

- <span id="splitinclusive-clone"></span>`fn clone(&self) -> SplitInclusive<'ch, P>` — [`SplitInclusive`](#splitinclusive)

##### `impl CloneToUninit for SplitInclusive<'ch, P>`

- <span id="splitinclusive-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug + Pattern> Debug for SplitInclusive<'ch, P>`

- <span id="splitinclusive-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitInclusive<'ch, P>`

- <span id="splitinclusive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitInclusive<'ch, P>`

- <span id="splitinclusive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SplitInclusive<'ch, P>`

##### `impl IntoParallelIterator for SplitInclusive<'ch, P>`

- <span id="splitinclusive-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitinclusive-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusive-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<P: Pattern> ParallelIterator for SplitInclusive<'ch, P>`

- <span id="splitinclusive-paralleliterator-type-item"></span>`type Item = &'ch str`

- <span id="splitinclusive-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for SplitInclusive<'ch, P>`

- <span id="splitinclusive-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitinclusive-pointable-type-init"></span>`type Init = T`

- <span id="splitinclusive-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitinclusive-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitinclusive-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitinclusive-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SplitInclusive<'ch, P>`

- <span id="splitinclusive-toowned-type-owned"></span>`type Owned = T`

- <span id="splitinclusive-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="splitinclusive-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SplitInclusive<'ch, P>`

- <span id="splitinclusive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitinclusive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitInclusive<'ch, P>`

- <span id="splitinclusive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitinclusive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitTerminator<'ch, P: Pattern>`

```rust
struct SplitTerminator<'ch, P: Pattern> {
    chars: &'ch str,
    terminator: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:754-757`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L754-L757)*

Parallel iterator over substrings separated by a terminator pattern

#### Implementations

- <span id="splitterminator-new"></span>`fn new(chars: &'ch str, terminator: P) -> Self`

#### Trait Implementations

##### `impl Any for SplitTerminator<'ch, P>`

- <span id="splitterminator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitTerminator<'ch, P>`

- <span id="splitterminator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitTerminator<'ch, P>`

- <span id="splitterminator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone + Pattern> Clone for SplitTerminator<'ch, P>`

- <span id="splitterminator-clone"></span>`fn clone(&self) -> SplitTerminator<'ch, P>` — [`SplitTerminator`](#splitterminator)

##### `impl CloneToUninit for SplitTerminator<'ch, P>`

- <span id="splitterminator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug + Pattern> Debug for SplitTerminator<'ch, P>`

- <span id="splitterminator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitTerminator<'ch, P>`

- <span id="splitterminator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitTerminator<'ch, P>`

- <span id="splitterminator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SplitTerminator<'ch, P>`

##### `impl IntoParallelIterator for SplitTerminator<'ch, P>`

- <span id="splitterminator-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitterminator-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitterminator-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<P: Pattern> ParallelIterator for SplitTerminator<'ch, P>`

- <span id="splitterminator-paralleliterator-type-item"></span>`type Item = &'ch str`

- <span id="splitterminator-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for SplitTerminator<'ch, P>`

- <span id="splitterminator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitterminator-pointable-type-init"></span>`type Init = T`

- <span id="splitterminator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitterminator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitterminator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitterminator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SplitTerminator<'ch, P>`

- <span id="splitterminator-toowned-type-owned"></span>`type Owned = T`

- <span id="splitterminator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="splitterminator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SplitTerminator<'ch, P>`

- <span id="splitterminator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitterminator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitTerminator<'ch, P>`

- <span id="splitterminator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitterminator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitTerminatorProducer<'ch, 'sep, P: Pattern>`

```rust
struct SplitTerminatorProducer<'ch, 'sep, P: Pattern> {
    splitter: SplitProducer<'sep, P, &'ch str>,
    skip_last: bool,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:759-762`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L759-L762)*

#### Implementations

- <span id="splitterminatorproducer-new"></span>`fn new(chars: &'ch str, terminator: &'sep P) -> Self`

#### Trait Implementations

##### `impl Any for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SplitTerminatorProducer<'ch, 'sep, P>`

##### `impl Pointable for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitterminatorproducer-pointable-type-init"></span>`type Init = T`

- <span id="splitterminatorproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitterminatorproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitterminatorproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitterminatorproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitterminatorproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitterminatorproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P: Pattern + 'sep> UnindexedProducer for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-unindexedproducer-type-item"></span>`type Item = &'ch str`

- <span id="splitterminatorproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="splitterminatorproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `Lines<'ch>`

```rust
struct Lines<'ch>(&'ch str);
```

*Defined in [`rayon-1.11.0/src/str.rs:820`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L820)*

Parallel iterator over lines in a string

#### Trait Implementations

##### `impl Any for Lines<'ch>`

- <span id="lines-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lines<'ch>`

- <span id="lines-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lines<'ch>`

- <span id="lines-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Lines<'ch>`

- <span id="lines-clone"></span>`fn clone(&self) -> Lines<'ch>` — [`Lines`](#lines)

##### `impl CloneToUninit for Lines<'ch>`

- <span id="lines-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Lines<'ch>`

- <span id="lines-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Lines<'ch>`

- <span id="lines-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lines<'ch>`

- <span id="lines-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Lines<'ch>`

##### `impl IntoParallelIterator for Lines<'ch>`

- <span id="lines-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="lines-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="lines-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Lines<'ch>`

- <span id="lines-paralleliterator-type-item"></span>`type Item = &'ch str`

- <span id="lines-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Lines<'ch>`

- <span id="lines-pointable-const-align"></span>`const ALIGN: usize`

- <span id="lines-pointable-type-init"></span>`type Init = T`

- <span id="lines-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="lines-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="lines-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="lines-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Lines<'ch>`

- <span id="lines-toowned-type-owned"></span>`type Owned = T`

- <span id="lines-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lines-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Lines<'ch>`

- <span id="lines-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lines-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lines<'ch>`

- <span id="lines-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lines-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitWhitespace<'ch>`

```rust
struct SplitWhitespace<'ch>(&'ch str);
```

*Defined in [`rayon-1.11.0/src/str.rs:845`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L845)*

Parallel iterator over substrings separated by whitespace

#### Trait Implementations

##### `impl Any for SplitWhitespace<'ch>`

- <span id="splitwhitespace-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitWhitespace<'ch>`

- <span id="splitwhitespace-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitWhitespace<'ch>`

- <span id="splitwhitespace-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SplitWhitespace<'ch>`

- <span id="splitwhitespace-clone"></span>`fn clone(&self) -> SplitWhitespace<'ch>` — [`SplitWhitespace`](#splitwhitespace)

##### `impl CloneToUninit for SplitWhitespace<'ch>`

- <span id="splitwhitespace-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SplitWhitespace<'ch>`

- <span id="splitwhitespace-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitWhitespace<'ch>`

- <span id="splitwhitespace-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitWhitespace<'ch>`

- <span id="splitwhitespace-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SplitWhitespace<'ch>`

##### `impl IntoParallelIterator for SplitWhitespace<'ch>`

- <span id="splitwhitespace-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitwhitespace-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitwhitespace-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for SplitWhitespace<'ch>`

- <span id="splitwhitespace-paralleliterator-type-item"></span>`type Item = &'ch str`

- <span id="splitwhitespace-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for SplitWhitespace<'ch>`

- <span id="splitwhitespace-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitwhitespace-pointable-type-init"></span>`type Init = T`

- <span id="splitwhitespace-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitwhitespace-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitwhitespace-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitwhitespace-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SplitWhitespace<'ch>`

- <span id="splitwhitespace-toowned-type-owned"></span>`type Owned = T`

- <span id="splitwhitespace-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="splitwhitespace-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SplitWhitespace<'ch>`

- <span id="splitwhitespace-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitwhitespace-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitWhitespace<'ch>`

- <span id="splitwhitespace-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitwhitespace-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitAsciiWhitespace<'ch>`

```rust
struct SplitAsciiWhitespace<'ch>(&'ch str);
```

*Defined in [`rayon-1.11.0/src/str.rs:870`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L870)*

Parallel iterator over substrings separated by ASCII whitespace

#### Trait Implementations

##### `impl Any for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-clone"></span>`fn clone(&self) -> SplitAsciiWhitespace<'ch>` — [`SplitAsciiWhitespace`](#splitasciiwhitespace)

##### `impl CloneToUninit for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SplitAsciiWhitespace<'ch>`

##### `impl IntoParallelIterator for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitasciiwhitespace-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitasciiwhitespace-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-paralleliterator-type-item"></span>`type Item = &'ch str`

- <span id="splitasciiwhitespace-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitasciiwhitespace-pointable-type-init"></span>`type Init = T`

- <span id="splitasciiwhitespace-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitasciiwhitespace-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitasciiwhitespace-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitasciiwhitespace-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-toowned-type-owned"></span>`type Owned = T`

- <span id="splitasciiwhitespace-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="splitasciiwhitespace-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitasciiwhitespace-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitasciiwhitespace-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Matches<'ch, P: Pattern>`

```rust
struct Matches<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:895-898`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L895-L898)*

Parallel iterator over substrings that match a pattern

#### Trait Implementations

##### `impl Any for Matches<'ch, P>`

- <span id="matches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Matches<'ch, P>`

- <span id="matches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Matches<'ch, P>`

- <span id="matches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone + Pattern> Clone for Matches<'ch, P>`

- <span id="matches-clone"></span>`fn clone(&self) -> Matches<'ch, P>` — [`Matches`](#matches)

##### `impl CloneToUninit for Matches<'ch, P>`

- <span id="matches-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug + Pattern> Debug for Matches<'ch, P>`

- <span id="matches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Matches<'ch, P>`

- <span id="matches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Matches<'ch, P>`

- <span id="matches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Matches<'ch, P>`

##### `impl IntoParallelIterator for Matches<'ch, P>`

- <span id="matches-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="matches-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="matches-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<P: Pattern> ParallelIterator for Matches<'ch, P>`

- <span id="matches-paralleliterator-type-item"></span>`type Item = &'ch str`

- <span id="matches-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Matches<'ch, P>`

- <span id="matches-pointable-const-align"></span>`const ALIGN: usize`

- <span id="matches-pointable-type-init"></span>`type Init = T`

- <span id="matches-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matches-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matches-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matches-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Matches<'ch, P>`

- <span id="matches-toowned-type-owned"></span>`type Owned = T`

- <span id="matches-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matches-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Matches<'ch, P>`

- <span id="matches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Matches<'ch, P>`

- <span id="matches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MatchesProducer<'ch, 'pat, P: Pattern>`

```rust
struct MatchesProducer<'ch, 'pat, P: Pattern> {
    chars: &'ch str,
    pattern: &'pat P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:900-903`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L900-L903)*

#### Trait Implementations

##### `impl Any for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MatchesProducer<'ch, 'pat, P>`

##### `impl Pointable for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="matchesproducer-pointable-type-init"></span>`type Init = T`

- <span id="matchesproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchesproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchesproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchesproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matchesproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matchesproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P: Pattern> UnindexedProducer for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-unindexedproducer-type-item"></span>`type Item = &'ch str`

- <span id="matchesproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="matchesproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `MatchIndices<'ch, P: Pattern>`

```rust
struct MatchIndices<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:951-954`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L951-L954)*

Parallel iterator over substrings that match a pattern, with their positions

#### Trait Implementations

##### `impl Any for MatchIndices<'ch, P>`

- <span id="matchindices-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchIndices<'ch, P>`

- <span id="matchindices-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchIndices<'ch, P>`

- <span id="matchindices-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone + Pattern> Clone for MatchIndices<'ch, P>`

- <span id="matchindices-clone"></span>`fn clone(&self) -> MatchIndices<'ch, P>` — [`MatchIndices`](#matchindices)

##### `impl CloneToUninit for MatchIndices<'ch, P>`

- <span id="matchindices-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug + Pattern> Debug for MatchIndices<'ch, P>`

- <span id="matchindices-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MatchIndices<'ch, P>`

- <span id="matchindices-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchIndices<'ch, P>`

- <span id="matchindices-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MatchIndices<'ch, P>`

##### `impl IntoParallelIterator for MatchIndices<'ch, P>`

- <span id="matchindices-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="matchindices-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="matchindices-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<P: Pattern> ParallelIterator for MatchIndices<'ch, P>`

- <span id="matchindices-paralleliterator-type-item"></span>`type Item = (usize, &'ch str)`

- <span id="matchindices-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for MatchIndices<'ch, P>`

- <span id="matchindices-pointable-const-align"></span>`const ALIGN: usize`

- <span id="matchindices-pointable-type-init"></span>`type Init = T`

- <span id="matchindices-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchindices-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchindices-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchindices-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MatchIndices<'ch, P>`

- <span id="matchindices-toowned-type-owned"></span>`type Owned = T`

- <span id="matchindices-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matchindices-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MatchIndices<'ch, P>`

- <span id="matchindices-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matchindices-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchIndices<'ch, P>`

- <span id="matchindices-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matchindices-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MatchIndicesProducer<'ch, 'pat, P: Pattern>`

```rust
struct MatchIndicesProducer<'ch, 'pat, P: Pattern> {
    index: usize,
    chars: &'ch str,
    pattern: &'pat P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:956-960`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L956-L960)*

#### Trait Implementations

##### `impl Any for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MatchIndicesProducer<'ch, 'pat, P>`

##### `impl Pointable for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="matchindicesproducer-pointable-type-init"></span>`type Init = T`

- <span id="matchindicesproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchindicesproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchindicesproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchindicesproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matchindicesproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matchindicesproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P: Pattern> UnindexedProducer for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-unindexedproducer-type-item"></span>`type Item = (usize, &'ch str)`

- <span id="matchindicesproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="matchindicesproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Traits

### `ParallelString`

```rust
trait ParallelString { ... }
```

*Defined in [`rayon-1.11.0/src/str.rs:58-342`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L58-L342)*

Parallel extensions for strings.

#### Required Methods

- `fn as_parallel_string(&self) -> &str`

  Returns a plain string slice, which is used to implement the rest of

#### Provided Methods

- `fn par_chars(&self) -> Chars<'_>`

  Returns a parallel iterator over the characters of a string.

- `fn par_char_indices(&self) -> CharIndices<'_>`

  Returns a parallel iterator over the characters of a string, with their positions.

- `fn par_bytes(&self) -> Bytes<'_>`

  Returns a parallel iterator over the bytes of a string.

- `fn par_encode_utf16(&self) -> EncodeUtf16<'_>`

  Returns a parallel iterator over a string encoded as UTF-16.

- `fn par_split<P: Pattern>(&self, separator: P) -> Split<'_, P>`

  Returns a parallel iterator over substrings separated by a

- `fn par_split_inclusive<P: Pattern>(&self, separator: P) -> SplitInclusive<'_, P>`

  Returns a parallel iterator over substrings separated by a

- `fn par_split_terminator<P: Pattern>(&self, terminator: P) -> SplitTerminator<'_, P>`

  Returns a parallel iterator over substrings terminated by a

- `fn par_lines(&self) -> Lines<'_>`

  Returns a parallel iterator over the lines of a string, ending with an

- `fn par_split_whitespace(&self) -> SplitWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are

- `fn par_split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are

- `fn par_matches<P: Pattern>(&self, pattern: P) -> Matches<'_, P>`

  Returns a parallel iterator over substrings that match a

- `fn par_match_indices<P: Pattern>(&self, pattern: P) -> MatchIndices<'_, P>`

  Returns a parallel iterator over substrings that match a given character

#### Implementors

- `str`

## Functions

### `is_char_boundary`

```rust
fn is_char_boundary(b: u8) -> bool
```

*Defined in [`rayon-1.11.0/src/str.rs:22-25`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L22-L25)*

Test if a byte is the start of a UTF-8 character.
(extracted from `str::is_char_boundary`)

### `find_char_midpoint`

```rust
fn find_char_midpoint(chars: &str) -> usize
```

*Defined in [`rayon-1.11.0/src/str.rs:29-44`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L29-L44)*

Find the index of a character boundary near the midpoint.

### `split`

```rust
fn split(chars: &str) -> Option<(&str, &str)>
```

*Defined in [`rayon-1.11.0/src/str.rs:48-55`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L48-L55)*

Try to split a string near the midpoint.

### `offset`

```rust
fn offset<T>(base: usize) -> impl Fn((usize, T)) -> (usize, T)
```

*Defined in [`rayon-1.11.0/src/str.rs:386-388`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L386-L388)*

### `no_carriage_return`

```rust
fn no_carriage_return(line: &str) -> &str
```

*Defined in [`rayon-1.11.0/src/str.rs:823-825`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L823-L825)*

### `not_empty`

```rust
fn not_empty(s: &&str) -> bool
```

*Defined in [`rayon-1.11.0/src/str.rs:848-850`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L848-L850)*

### `is_ascii_whitespace`

```rust
fn is_ascii_whitespace(c: char) -> bool
```

*Defined in [`rayon-1.11.0/src/str.rs:873-875`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L873-L875)*

## Macros

### `impl_pattern!`

*Defined in [`rayon-1.11.0/src/str.rs:390-441`](../../../.source_1765521767/rayon-1.11.0/src/str.rs#L390-L441)*

