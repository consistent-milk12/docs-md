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



## Structs

### `Chars<'ch>`

```rust
struct Chars<'ch> {
    chars: &'ch str,
}
```

Parallel iterator over the characters of a string

#### Trait Implementations

##### `impl<'ch> Clone for Chars<'ch>`

- `fn clone(self: &Self) -> Chars<'ch>` — [`Chars`](#chars)

##### `impl<'ch> Debug for Chars<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Chars<'ch>`

##### `impl<T> IntoParallelIterator for Chars<'ch>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch> ParallelIterator for Chars<'ch>`

- `type Item = char`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Chars<'ch>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `CharIndices<'ch>`

```rust
struct CharIndices<'ch> {
    chars: &'ch str,
}
```

Parallel iterator over the characters of a string, with their positions

#### Trait Implementations

##### `impl<'ch> Clone for CharIndices<'ch>`

- `fn clone(self: &Self) -> CharIndices<'ch>` — [`CharIndices`](#charindices)

##### `impl<'ch> Debug for CharIndices<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for CharIndices<'ch>`

##### `impl<T> IntoParallelIterator for CharIndices<'ch>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch> ParallelIterator for CharIndices<'ch>`

- `type Item = (usize, char)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for CharIndices<'ch>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Bytes<'ch>`

```rust
struct Bytes<'ch> {
    chars: &'ch str,
}
```

Parallel iterator over the bytes of a string

#### Trait Implementations

##### `impl<'ch> Clone for Bytes<'ch>`

- `fn clone(self: &Self) -> Bytes<'ch>` — [`Bytes`](#bytes)

##### `impl<'ch> Debug for Bytes<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Bytes<'ch>`

##### `impl<T> IntoParallelIterator for Bytes<'ch>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch> ParallelIterator for Bytes<'ch>`

- `type Item = u8`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Bytes<'ch>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `EncodeUtf16<'ch>`

```rust
struct EncodeUtf16<'ch> {
    chars: &'ch str,
}
```

Parallel iterator over a string encoded as UTF-16

#### Trait Implementations

##### `impl<'ch> Clone for EncodeUtf16<'ch>`

- `fn clone(self: &Self) -> EncodeUtf16<'ch>` — [`EncodeUtf16`](#encodeutf16)

##### `impl<'ch> Debug for EncodeUtf16<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for EncodeUtf16<'ch>`

##### `impl<T> IntoParallelIterator for EncodeUtf16<'ch>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch> ParallelIterator for EncodeUtf16<'ch>`

- `type Item = u16`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for EncodeUtf16<'ch>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Split<'ch, P: Pattern>`

```rust
struct Split<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}
```

Parallel iterator over substrings separated by a pattern

#### Implementations

- `fn new(chars: &'ch str, separator: P) -> Self`

#### Trait Implementations

##### `impl<'ch, P: $crate::clone::Clone + Pattern> Clone for Split<'ch, P>`

- `fn clone(self: &Self) -> Split<'ch, P>` — [`Split`](#split)

##### `impl<'ch, P: $crate::fmt::Debug + Pattern> Debug for Split<'ch, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Split<'ch, P>`

##### `impl<T> IntoParallelIterator for Split<'ch, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for Split<'ch, P>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Split<'ch, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SplitInclusive<'ch, P: Pattern>`

```rust
struct SplitInclusive<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}
```

Parallel iterator over substrings separated by a pattern

#### Implementations

- `fn new(chars: &'ch str, separator: P) -> Self`

#### Trait Implementations

##### `impl<'ch, P: $crate::clone::Clone + Pattern> Clone for SplitInclusive<'ch, P>`

- `fn clone(self: &Self) -> SplitInclusive<'ch, P>` — [`SplitInclusive`](#splitinclusive)

##### `impl<'ch, P: $crate::fmt::Debug + Pattern> Debug for SplitInclusive<'ch, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for SplitInclusive<'ch, P>`

##### `impl<T> IntoParallelIterator for SplitInclusive<'ch, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for SplitInclusive<'ch, P>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitInclusive<'ch, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SplitTerminator<'ch, P: Pattern>`

```rust
struct SplitTerminator<'ch, P: Pattern> {
    chars: &'ch str,
    terminator: P,
}
```

Parallel iterator over substrings separated by a terminator pattern

#### Implementations

- `fn new(chars: &'ch str, terminator: P) -> Self`

#### Trait Implementations

##### `impl<'ch, P: $crate::clone::Clone + Pattern> Clone for SplitTerminator<'ch, P>`

- `fn clone(self: &Self) -> SplitTerminator<'ch, P>` — [`SplitTerminator`](#splitterminator)

##### `impl<'ch, P: $crate::fmt::Debug + Pattern> Debug for SplitTerminator<'ch, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for SplitTerminator<'ch, P>`

##### `impl<T> IntoParallelIterator for SplitTerminator<'ch, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for SplitTerminator<'ch, P>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitTerminator<'ch, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Lines<'ch>`

```rust
struct Lines<'ch>(&'ch str);
```

Parallel iterator over lines in a string

#### Trait Implementations

##### `impl<'ch> Clone for Lines<'ch>`

- `fn clone(self: &Self) -> Lines<'ch>` — [`Lines`](#lines)

##### `impl<'ch> Debug for Lines<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Lines<'ch>`

##### `impl<T> IntoParallelIterator for Lines<'ch>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch> ParallelIterator for Lines<'ch>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Lines<'ch>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SplitWhitespace<'ch>`

```rust
struct SplitWhitespace<'ch>(&'ch str);
```

Parallel iterator over substrings separated by whitespace

#### Trait Implementations

##### `impl<'ch> Clone for SplitWhitespace<'ch>`

- `fn clone(self: &Self) -> SplitWhitespace<'ch>` — [`SplitWhitespace`](#splitwhitespace)

##### `impl<'ch> Debug for SplitWhitespace<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for SplitWhitespace<'ch>`

##### `impl<T> IntoParallelIterator for SplitWhitespace<'ch>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch> ParallelIterator for SplitWhitespace<'ch>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitWhitespace<'ch>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SplitAsciiWhitespace<'ch>`

```rust
struct SplitAsciiWhitespace<'ch>(&'ch str);
```

Parallel iterator over substrings separated by ASCII whitespace

#### Trait Implementations

##### `impl<'ch> Clone for SplitAsciiWhitespace<'ch>`

- `fn clone(self: &Self) -> SplitAsciiWhitespace<'ch>` — [`SplitAsciiWhitespace`](#splitasciiwhitespace)

##### `impl<'ch> Debug for SplitAsciiWhitespace<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for SplitAsciiWhitespace<'ch>`

##### `impl<T> IntoParallelIterator for SplitAsciiWhitespace<'ch>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch> ParallelIterator for SplitAsciiWhitespace<'ch>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitAsciiWhitespace<'ch>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Matches<'ch, P: Pattern>`

```rust
struct Matches<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}
```

Parallel iterator over substrings that match a pattern

#### Trait Implementations

##### `impl<'ch, P: $crate::clone::Clone + Pattern> Clone for Matches<'ch, P>`

- `fn clone(self: &Self) -> Matches<'ch, P>` — [`Matches`](#matches)

##### `impl<'ch, P: $crate::fmt::Debug + Pattern> Debug for Matches<'ch, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Matches<'ch, P>`

##### `impl<T> IntoParallelIterator for Matches<'ch, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for Matches<'ch, P>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Matches<'ch, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MatchIndices<'ch, P: Pattern>`

```rust
struct MatchIndices<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}
```

Parallel iterator over substrings that match a pattern, with their positions

#### Trait Implementations

##### `impl<'ch, P: $crate::clone::Clone + Pattern> Clone for MatchIndices<'ch, P>`

- `fn clone(self: &Self) -> MatchIndices<'ch, P>` — [`MatchIndices`](#matchindices)

##### `impl<'ch, P: $crate::fmt::Debug + Pattern> Debug for MatchIndices<'ch, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for MatchIndices<'ch, P>`

##### `impl<T> IntoParallelIterator for MatchIndices<'ch, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'ch, P: Pattern> ParallelIterator for MatchIndices<'ch, P>`

- `type Item = (usize, &'ch str)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for MatchIndices<'ch, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `ParallelString`

```rust
trait ParallelString { ... }
```

Parallel extensions for strings.

#### Required Methods

- `fn as_parallel_string(self: &Self) -> &str`

  Returns a plain string slice, which is used to implement the rest of

- `fn par_chars(self: &Self) -> Chars<'_>`

  Returns a parallel iterator over the characters of a string.

- `fn par_char_indices(self: &Self) -> CharIndices<'_>`

  Returns a parallel iterator over the characters of a string, with their positions.

- `fn par_bytes(self: &Self) -> Bytes<'_>`

  Returns a parallel iterator over the bytes of a string.

- `fn par_encode_utf16(self: &Self) -> EncodeUtf16<'_>`

  Returns a parallel iterator over a string encoded as UTF-16.

- `fn par_split<P: Pattern>(self: &Self, separator: P) -> Split<'_, P>`

  Returns a parallel iterator over substrings separated by a

- `fn par_split_inclusive<P: Pattern>(self: &Self, separator: P) -> SplitInclusive<'_, P>`

  Returns a parallel iterator over substrings separated by a

- `fn par_split_terminator<P: Pattern>(self: &Self, terminator: P) -> SplitTerminator<'_, P>`

  Returns a parallel iterator over substrings terminated by a

- `fn par_lines(self: &Self) -> Lines<'_>`

  Returns a parallel iterator over the lines of a string, ending with an

- `fn par_split_whitespace(self: &Self) -> SplitWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are

- `fn par_split_ascii_whitespace(self: &Self) -> SplitAsciiWhitespace<'_>`

  Returns a parallel iterator over the sub-slices of a string that are

- `fn par_matches<P: Pattern>(self: &Self, pattern: P) -> Matches<'_, P>`

  Returns a parallel iterator over substrings that match a

- `fn par_match_indices<P: Pattern>(self: &Self, pattern: P) -> MatchIndices<'_, P>`

  Returns a parallel iterator over substrings that match a given character

