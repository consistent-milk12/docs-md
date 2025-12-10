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
  - [`is_char_boundary`](#is_char_boundary)
  - [`find_char_midpoint`](#find_char_midpoint)
  - [`split`](#split)
  - [`offset`](#offset)
  - [`no_carriage_return`](#no_carriage_return)
  - [`not_empty`](#not_empty)
  - [`is_ascii_whitespace`](#is_ascii_whitespace)
- [Macros](#macros)
  - [`impl_pattern!`](#impl_pattern)

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
| [`is_char_boundary`](#is_char_boundary) | fn | Test if a byte is the start of a UTF-8 character. |
| [`find_char_midpoint`](#find_char_midpoint) | fn | Find the index of a character boundary near the midpoint. |
| [`split`](#split) | fn | Try to split a string near the midpoint. |
| [`offset`](#offset) | fn |  |
| [`no_carriage_return`](#no_carriage_return) | fn |  |
| [`not_empty`](#not_empty) | fn |  |
| [`is_ascii_whitespace`](#is_ascii_whitespace) | fn |  |
| [`impl_pattern!`](#impl_pattern) | macro |  |

## Modules

- [`private`](private/index.md) — We hide the `Pattern` trait in a private module, as its API is not meant

## Structs

### `Chars<'ch>`

```rust
struct Chars<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:467-469`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L467-L469)*

Parallel iterator over the characters of a string

#### Trait Implementations

##### `impl Clone for Chars<'ch>`

- <span id="chars-clone"></span>`fn clone(&self) -> Chars<'ch>` — [`Chars`](#chars)

##### `impl Debug for Chars<'ch>`

- <span id="chars-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for Chars<'ch>`

##### `impl IntoParallelIterator for Chars<'ch>`

- <span id="chars-type-iter"></span>`type Iter = T`

- <span id="chars-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chars-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Chars<'ch>`

- <span id="chars-type-item"></span>`type Item = char`

- <span id="chars-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Chars<'ch>`

- <span id="chars-const-align"></span>`const ALIGN: usize`

- <span id="chars-type-init"></span>`type Init = T`

- <span id="chars-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chars-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chars-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chars-drop"></span>`unsafe fn drop(ptr: usize)`

### `CharsProducer<'ch>`

```rust
struct CharsProducer<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:471-473`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L471-L473)*

#### Trait Implementations

##### `impl IntoEither for CharsProducer<'ch>`

##### `impl Pointable for CharsProducer<'ch>`

- <span id="charsproducer-const-align"></span>`const ALIGN: usize`

- <span id="charsproducer-type-init"></span>`type Init = T`

- <span id="charsproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="charsproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="charsproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="charsproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl UnindexedProducer for CharsProducer<'ch>`

- <span id="charsproducer-type-item"></span>`type Item = char`

- <span id="charsproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="charsproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `CharIndices<'ch>`

```rust
struct CharIndices<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:511-513`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L511-L513)*

Parallel iterator over the characters of a string, with their positions

#### Trait Implementations

##### `impl Clone for CharIndices<'ch>`

- <span id="charindices-clone"></span>`fn clone(&self) -> CharIndices<'ch>` — [`CharIndices`](#charindices)

##### `impl Debug for CharIndices<'ch>`

- <span id="charindices-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for CharIndices<'ch>`

##### `impl IntoParallelIterator for CharIndices<'ch>`

- <span id="charindices-type-iter"></span>`type Iter = T`

- <span id="charindices-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="charindices-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for CharIndices<'ch>`

- <span id="charindices-type-item"></span>`type Item = (usize, char)`

- <span id="charindices-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for CharIndices<'ch>`

- <span id="charindices-const-align"></span>`const ALIGN: usize`

- <span id="charindices-type-init"></span>`type Init = T`

- <span id="charindices-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="charindices-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="charindices-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="charindices-drop"></span>`unsafe fn drop(ptr: usize)`

### `CharIndicesProducer<'ch>`

```rust
struct CharIndicesProducer<'ch> {
    index: usize,
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:515-518`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L515-L518)*

#### Trait Implementations

##### `impl IntoEither for CharIndicesProducer<'ch>`

##### `impl Pointable for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-const-align"></span>`const ALIGN: usize`

- <span id="charindicesproducer-type-init"></span>`type Init = T`

- <span id="charindicesproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="charindicesproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="charindicesproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="charindicesproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl UnindexedProducer for CharIndicesProducer<'ch>`

- <span id="charindicesproducer-type-item"></span>`type Item = (usize, char)`

- <span id="charindicesproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="charindicesproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `Bytes<'ch>`

```rust
struct Bytes<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:567-569`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L567-L569)*

Parallel iterator over the bytes of a string

#### Trait Implementations

##### `impl Clone for Bytes<'ch>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'ch>` — [`Bytes`](#bytes)

##### `impl Debug for Bytes<'ch>`

- <span id="bytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for Bytes<'ch>`

##### `impl IntoParallelIterator for Bytes<'ch>`

- <span id="bytes-type-iter"></span>`type Iter = T`

- <span id="bytes-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="bytes-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Bytes<'ch>`

- <span id="bytes-type-item"></span>`type Item = u8`

- <span id="bytes-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Bytes<'ch>`

- <span id="bytes-const-align"></span>`const ALIGN: usize`

- <span id="bytes-type-init"></span>`type Init = T`

- <span id="bytes-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="bytes-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="bytes-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="bytes-drop"></span>`unsafe fn drop(ptr: usize)`

### `BytesProducer<'ch>`

```rust
struct BytesProducer<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:571-573`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L571-L573)*

#### Trait Implementations

##### `impl IntoEither for BytesProducer<'ch>`

##### `impl Pointable for BytesProducer<'ch>`

- <span id="bytesproducer-const-align"></span>`const ALIGN: usize`

- <span id="bytesproducer-type-init"></span>`type Init = T`

- <span id="bytesproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="bytesproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="bytesproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="bytesproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl UnindexedProducer for BytesProducer<'ch>`

- <span id="bytesproducer-type-item"></span>`type Item = u8`

- <span id="bytesproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="bytesproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `EncodeUtf16<'ch>`

```rust
struct EncodeUtf16<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:611-613`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L611-L613)*

Parallel iterator over a string encoded as UTF-16

#### Trait Implementations

##### `impl Clone for EncodeUtf16<'ch>`

- <span id="encodeutf16-clone"></span>`fn clone(&self) -> EncodeUtf16<'ch>` — [`EncodeUtf16`](#encodeutf16)

##### `impl Debug for EncodeUtf16<'ch>`

- <span id="encodeutf16-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for EncodeUtf16<'ch>`

##### `impl IntoParallelIterator for EncodeUtf16<'ch>`

- <span id="encodeutf16-type-iter"></span>`type Iter = T`

- <span id="encodeutf16-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="encodeutf16-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for EncodeUtf16<'ch>`

- <span id="encodeutf16-type-item"></span>`type Item = u16`

- <span id="encodeutf16-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for EncodeUtf16<'ch>`

- <span id="encodeutf16-const-align"></span>`const ALIGN: usize`

- <span id="encodeutf16-type-init"></span>`type Init = T`

- <span id="encodeutf16-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="encodeutf16-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="encodeutf16-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="encodeutf16-drop"></span>`unsafe fn drop(ptr: usize)`

### `EncodeUtf16Producer<'ch>`

```rust
struct EncodeUtf16Producer<'ch> {
    chars: &'ch str,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:615-617`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L615-L617)*

#### Trait Implementations

##### `impl IntoEither for EncodeUtf16Producer<'ch>`

##### `impl Pointable for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-const-align"></span>`const ALIGN: usize`

- <span id="encodeutf16producer-type-init"></span>`type Init = T`

- <span id="encodeutf16producer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="encodeutf16producer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="encodeutf16producer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="encodeutf16producer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl UnindexedProducer for EncodeUtf16Producer<'ch>`

- <span id="encodeutf16producer-type-item"></span>`type Item = u16`

- <span id="encodeutf16producer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="encodeutf16producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `Split<'ch, P: Pattern>`

```rust
struct Split<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:655-658`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L655-L658)*

Parallel iterator over substrings separated by a pattern

#### Implementations

- <span id="split-new"></span>`fn new(chars: &'ch str, separator: P) -> Self`

#### Trait Implementations

##### `impl<'ch, P: clone::Clone + Pattern> Clone for Split<'ch, P>`

- <span id="split-clone"></span>`fn clone(&self) -> Split<'ch, P>` — [`Split`](#split)

##### `impl<'ch, P: fmt::Debug + Pattern> Debug for Split<'ch, P>`

- <span id="split-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Split<'ch, P>`

##### `impl<T> IntoParallelIterator for Split<'ch, P>`

- <span id="split-type-iter"></span>`type Iter = T`

- <span id="split-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="split-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for Split<'ch, P>`

- <span id="split-type-item"></span>`type Item = &'ch str`

- <span id="split-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for Split<'ch, P>`

- <span id="split-const-align"></span>`const ALIGN: usize`

- <span id="split-type-init"></span>`type Init = T`

- <span id="split-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="split-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="split-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="split-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitInclusive<'ch, P: Pattern>`

```rust
struct SplitInclusive<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:727-730`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L727-L730)*

Parallel iterator over substrings separated by a pattern

#### Implementations

- <span id="splitinclusive-new"></span>`fn new(chars: &'ch str, separator: P) -> Self`

#### Trait Implementations

##### `impl<'ch, P: clone::Clone + Pattern> Clone for SplitInclusive<'ch, P>`

- <span id="splitinclusive-clone"></span>`fn clone(&self) -> SplitInclusive<'ch, P>` — [`SplitInclusive`](#splitinclusive)

##### `impl<'ch, P: fmt::Debug + Pattern> Debug for SplitInclusive<'ch, P>`

- <span id="splitinclusive-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitInclusive<'ch, P>`

##### `impl<T> IntoParallelIterator for SplitInclusive<'ch, P>`

- <span id="splitinclusive-type-iter"></span>`type Iter = T`

- <span id="splitinclusive-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusive-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for SplitInclusive<'ch, P>`

- <span id="splitinclusive-type-item"></span>`type Item = &'ch str`

- <span id="splitinclusive-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitInclusive<'ch, P>`

- <span id="splitinclusive-const-align"></span>`const ALIGN: usize`

- <span id="splitinclusive-type-init"></span>`type Init = T`

- <span id="splitinclusive-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitinclusive-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitinclusive-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitinclusive-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitTerminator<'ch, P: Pattern>`

```rust
struct SplitTerminator<'ch, P: Pattern> {
    chars: &'ch str,
    terminator: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:754-757`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L754-L757)*

Parallel iterator over substrings separated by a terminator pattern

#### Implementations

- <span id="splitterminator-new"></span>`fn new(chars: &'ch str, terminator: P) -> Self`

#### Trait Implementations

##### `impl<'ch, P: clone::Clone + Pattern> Clone for SplitTerminator<'ch, P>`

- <span id="splitterminator-clone"></span>`fn clone(&self) -> SplitTerminator<'ch, P>` — [`SplitTerminator`](#splitterminator)

##### `impl<'ch, P: fmt::Debug + Pattern> Debug for SplitTerminator<'ch, P>`

- <span id="splitterminator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitTerminator<'ch, P>`

##### `impl<T> IntoParallelIterator for SplitTerminator<'ch, P>`

- <span id="splitterminator-type-iter"></span>`type Iter = T`

- <span id="splitterminator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitterminator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for SplitTerminator<'ch, P>`

- <span id="splitterminator-type-item"></span>`type Item = &'ch str`

- <span id="splitterminator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitTerminator<'ch, P>`

- <span id="splitterminator-const-align"></span>`const ALIGN: usize`

- <span id="splitterminator-type-init"></span>`type Init = T`

- <span id="splitterminator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitterminator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitterminator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitterminator-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitTerminatorProducer<'ch, 'sep, P: Pattern>`

```rust
struct SplitTerminatorProducer<'ch, 'sep, P: Pattern> {
    splitter: SplitProducer<'sep, P, &'ch str>,
    skip_last: bool,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:759-762`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L759-L762)*

#### Implementations

- <span id="splitterminatorproducer-new"></span>`fn new(chars: &'ch str, terminator: &'sep P) -> Self`

#### Trait Implementations

##### `impl<T> IntoEither for SplitTerminatorProducer<'ch, 'sep, P>`

##### `impl<T> Pointable for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-const-align"></span>`const ALIGN: usize`

- <span id="splitterminatorproducer-type-init"></span>`type Init = T`

- <span id="splitterminatorproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitterminatorproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitterminatorproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitterminatorproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'ch, 'sep, P: Pattern + 'sep> UnindexedProducer for SplitTerminatorProducer<'ch, 'sep, P>`

- <span id="splitterminatorproducer-type-item"></span>`type Item = &'ch str`

- <span id="splitterminatorproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="splitterminatorproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `Lines<'ch>`

```rust
struct Lines<'ch>(&'ch str);
```

*Defined in [`rayon-1.11.0/src/str.rs:820`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L820)*

Parallel iterator over lines in a string

#### Trait Implementations

##### `impl Clone for Lines<'ch>`

- <span id="lines-clone"></span>`fn clone(&self) -> Lines<'ch>` — [`Lines`](#lines)

##### `impl Debug for Lines<'ch>`

- <span id="lines-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for Lines<'ch>`

##### `impl IntoParallelIterator for Lines<'ch>`

- <span id="lines-type-iter"></span>`type Iter = T`

- <span id="lines-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="lines-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Lines<'ch>`

- <span id="lines-type-item"></span>`type Item = &'ch str`

- <span id="lines-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Lines<'ch>`

- <span id="lines-const-align"></span>`const ALIGN: usize`

- <span id="lines-type-init"></span>`type Init = T`

- <span id="lines-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="lines-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="lines-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="lines-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitWhitespace<'ch>`

```rust
struct SplitWhitespace<'ch>(&'ch str);
```

*Defined in [`rayon-1.11.0/src/str.rs:845`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L845)*

Parallel iterator over substrings separated by whitespace

#### Trait Implementations

##### `impl Clone for SplitWhitespace<'ch>`

- <span id="splitwhitespace-clone"></span>`fn clone(&self) -> SplitWhitespace<'ch>` — [`SplitWhitespace`](#splitwhitespace)

##### `impl Debug for SplitWhitespace<'ch>`

- <span id="splitwhitespace-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for SplitWhitespace<'ch>`

##### `impl IntoParallelIterator for SplitWhitespace<'ch>`

- <span id="splitwhitespace-type-iter"></span>`type Iter = T`

- <span id="splitwhitespace-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitwhitespace-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for SplitWhitespace<'ch>`

- <span id="splitwhitespace-type-item"></span>`type Item = &'ch str`

- <span id="splitwhitespace-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for SplitWhitespace<'ch>`

- <span id="splitwhitespace-const-align"></span>`const ALIGN: usize`

- <span id="splitwhitespace-type-init"></span>`type Init = T`

- <span id="splitwhitespace-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitwhitespace-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitwhitespace-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitwhitespace-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitAsciiWhitespace<'ch>`

```rust
struct SplitAsciiWhitespace<'ch>(&'ch str);
```

*Defined in [`rayon-1.11.0/src/str.rs:870`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L870)*

Parallel iterator over substrings separated by ASCII whitespace

#### Trait Implementations

##### `impl Clone for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-clone"></span>`fn clone(&self) -> SplitAsciiWhitespace<'ch>` — [`SplitAsciiWhitespace`](#splitasciiwhitespace)

##### `impl Debug for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for SplitAsciiWhitespace<'ch>`

##### `impl IntoParallelIterator for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-type-iter"></span>`type Iter = T`

- <span id="splitasciiwhitespace-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitasciiwhitespace-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-type-item"></span>`type Item = &'ch str`

- <span id="splitasciiwhitespace-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for SplitAsciiWhitespace<'ch>`

- <span id="splitasciiwhitespace-const-align"></span>`const ALIGN: usize`

- <span id="splitasciiwhitespace-type-init"></span>`type Init = T`

- <span id="splitasciiwhitespace-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitasciiwhitespace-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitasciiwhitespace-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitasciiwhitespace-drop"></span>`unsafe fn drop(ptr: usize)`

### `Matches<'ch, P: Pattern>`

```rust
struct Matches<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:895-898`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L895-L898)*

Parallel iterator over substrings that match a pattern

#### Trait Implementations

##### `impl<'ch, P: clone::Clone + Pattern> Clone for Matches<'ch, P>`

- <span id="matches-clone"></span>`fn clone(&self) -> Matches<'ch, P>` — [`Matches`](#matches)

##### `impl<'ch, P: fmt::Debug + Pattern> Debug for Matches<'ch, P>`

- <span id="matches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Matches<'ch, P>`

##### `impl<T> IntoParallelIterator for Matches<'ch, P>`

- <span id="matches-type-iter"></span>`type Iter = T`

- <span id="matches-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="matches-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for Matches<'ch, P>`

- <span id="matches-type-item"></span>`type Item = &'ch str`

- <span id="matches-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for Matches<'ch, P>`

- <span id="matches-const-align"></span>`const ALIGN: usize`

- <span id="matches-type-init"></span>`type Init = T`

- <span id="matches-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matches-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matches-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matches-drop"></span>`unsafe fn drop(ptr: usize)`

### `MatchesProducer<'ch, 'pat, P: Pattern>`

```rust
struct MatchesProducer<'ch, 'pat, P: Pattern> {
    chars: &'ch str,
    pattern: &'pat P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:900-903`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L900-L903)*

#### Trait Implementations

##### `impl<T> IntoEither for MatchesProducer<'ch, 'pat, P>`

##### `impl<T> Pointable for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-const-align"></span>`const ALIGN: usize`

- <span id="matchesproducer-type-init"></span>`type Init = T`

- <span id="matchesproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchesproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchesproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchesproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'ch, 'pat, P: Pattern> UnindexedProducer for MatchesProducer<'ch, 'pat, P>`

- <span id="matchesproducer-type-item"></span>`type Item = &'ch str`

- <span id="matchesproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="matchesproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `MatchIndices<'ch, P: Pattern>`

```rust
struct MatchIndices<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:951-954`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L951-L954)*

Parallel iterator over substrings that match a pattern, with their positions

#### Trait Implementations

##### `impl<'ch, P: clone::Clone + Pattern> Clone for MatchIndices<'ch, P>`

- <span id="matchindices-clone"></span>`fn clone(&self) -> MatchIndices<'ch, P>` — [`MatchIndices`](#matchindices)

##### `impl<'ch, P: fmt::Debug + Pattern> Debug for MatchIndices<'ch, P>`

- <span id="matchindices-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for MatchIndices<'ch, P>`

##### `impl<T> IntoParallelIterator for MatchIndices<'ch, P>`

- <span id="matchindices-type-iter"></span>`type Iter = T`

- <span id="matchindices-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="matchindices-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for MatchIndices<'ch, P>`

- <span id="matchindices-type-item"></span>`type Item = (usize, &'ch str)`

- <span id="matchindices-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for MatchIndices<'ch, P>`

- <span id="matchindices-const-align"></span>`const ALIGN: usize`

- <span id="matchindices-type-init"></span>`type Init = T`

- <span id="matchindices-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchindices-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchindices-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchindices-drop"></span>`unsafe fn drop(ptr: usize)`

### `MatchIndicesProducer<'ch, 'pat, P: Pattern>`

```rust
struct MatchIndicesProducer<'ch, 'pat, P: Pattern> {
    index: usize,
    chars: &'ch str,
    pattern: &'pat P,
}
```

*Defined in [`rayon-1.11.0/src/str.rs:956-960`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L956-L960)*

#### Trait Implementations

##### `impl<T> IntoEither for MatchIndicesProducer<'ch, 'pat, P>`

##### `impl<T> Pointable for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-const-align"></span>`const ALIGN: usize`

- <span id="matchindicesproducer-type-init"></span>`type Init = T`

- <span id="matchindicesproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchindicesproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchindicesproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchindicesproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'ch, 'pat, P: Pattern> UnindexedProducer for MatchIndicesProducer<'ch, 'pat, P>`

- <span id="matchindicesproducer-type-item"></span>`type Item = (usize, &'ch str)`

- <span id="matchindicesproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="matchindicesproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Traits

### `ParallelString`

```rust
trait ParallelString { ... }
```

*Defined in [`rayon-1.11.0/src/str.rs:58-342`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L58-L342)*

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

*Defined in [`rayon-1.11.0/src/str.rs:22-25`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L22-L25)*

Test if a byte is the start of a UTF-8 character.
(extracted from `str::is_char_boundary`)

### `find_char_midpoint`

```rust
fn find_char_midpoint(chars: &str) -> usize
```

*Defined in [`rayon-1.11.0/src/str.rs:29-44`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L29-L44)*

Find the index of a character boundary near the midpoint.

### `split`

```rust
fn split(chars: &str) -> Option<(&str, &str)>
```

*Defined in [`rayon-1.11.0/src/str.rs:48-55`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L48-L55)*

Try to split a string near the midpoint.

### `offset`

```rust
fn offset<T>(base: usize) -> impl Fn((usize, T)) -> (usize, T)
```

*Defined in [`rayon-1.11.0/src/str.rs:386-388`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L386-L388)*

### `no_carriage_return`

```rust
fn no_carriage_return(line: &str) -> &str
```

*Defined in [`rayon-1.11.0/src/str.rs:823-825`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L823-L825)*

### `not_empty`

```rust
fn not_empty(s: &&str) -> bool
```

*Defined in [`rayon-1.11.0/src/str.rs:848-850`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L848-L850)*

### `is_ascii_whitespace`

```rust
fn is_ascii_whitespace(c: char) -> bool
```

*Defined in [`rayon-1.11.0/src/str.rs:873-875`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L873-L875)*

## Macros

### `impl_pattern!`

*Defined in [`rayon-1.11.0/src/str.rs:390-441`](../../../.source_1765210505/rayon-1.11.0/src/str.rs#L390-L441)*

