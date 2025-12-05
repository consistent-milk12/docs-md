*[rayon](../index.md) / [str](index.md)*

---

# Module `str`

Parallel iterator types for [strings](#strings)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

Note: `ParallelString::par_split()` and [`par_split_terminator()`](#par-split-terminator)
reference a `Pattern` trait which is not visible outside this crate.
This trait is intentionally kept private, for use only by Rayon itself.
It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
and any function or closure `F: Fn(char) -> bool + Sync + Send`.

[strings](#strings): std::str

## Structs

### `Chars<'ch>`

```rust
struct Chars<'ch> {
    chars: &'ch str,
}
```

Parallel iterator over the characters of a string

#### Trait Implementations

##### `impl Clone<'ch>`

- `fn clone(self: &Self) -> Chars<'ch>` — [`Chars`](../../str/index.md)

##### `impl Debug<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch>`

- `type Item = char`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch>`

- `fn clone(self: &Self) -> CharIndices<'ch>` — [`CharIndices`](../../str/index.md)

##### `impl Debug<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch>`

- `type Item = (usize, char)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch>`

- `fn clone(self: &Self) -> Bytes<'ch>` — [`Bytes`](../../str/index.md)

##### `impl Debug<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch>`

- `type Item = u8`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch>`

- `fn clone(self: &Self) -> EncodeUtf16<'ch>` — [`EncodeUtf16`](../../str/index.md)

##### `impl Debug<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch>`

- `type Item = u16`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch, P: $crate::clone::Clone + Pattern>`

- `fn clone(self: &Self) -> Split<'ch, P>` — [`Split`](../../str/index.md)

##### `impl Debug<'ch, P: $crate::fmt::Debug + Pattern>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch, P: Pattern>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch, P: $crate::clone::Clone + Pattern>`

- `fn clone(self: &Self) -> SplitInclusive<'ch, P>` — [`SplitInclusive`](../../str/index.md)

##### `impl Debug<'ch, P: $crate::fmt::Debug + Pattern>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch, P: Pattern>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch, P: $crate::clone::Clone + Pattern>`

- `fn clone(self: &Self) -> SplitTerminator<'ch, P>` — [`SplitTerminator`](../../str/index.md)

##### `impl Debug<'ch, P: $crate::fmt::Debug + Pattern>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch, P: Pattern>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch>`

- `fn clone(self: &Self) -> Lines<'ch>` — [`Lines`](../../str/index.md)

##### `impl Debug<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch>`

- `fn clone(self: &Self) -> SplitWhitespace<'ch>` — [`SplitWhitespace`](../../str/index.md)

##### `impl Debug<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch>`

- `fn clone(self: &Self) -> SplitAsciiWhitespace<'ch>` — [`SplitAsciiWhitespace`](../../str/index.md)

##### `impl Debug<'ch>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch, P: $crate::clone::Clone + Pattern>`

- `fn clone(self: &Self) -> Matches<'ch, P>` — [`Matches`](../../str/index.md)

##### `impl Debug<'ch, P: $crate::fmt::Debug + Pattern>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch, P: Pattern>`

- `type Item = &'ch str`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<'ch, P: $crate::clone::Clone + Pattern>`

- `fn clone(self: &Self) -> MatchIndices<'ch, P>` — [`MatchIndices`](../../str/index.md)

##### `impl Debug<'ch, P: $crate::fmt::Debug + Pattern>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'ch, P: Pattern>`

- `type Item = (usize, &'ch str)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

