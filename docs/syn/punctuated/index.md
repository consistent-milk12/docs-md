*[syn](../index.md) / [punctuated](index.md)*

---

# Module `punctuated`

A punctuated sequence of syntax tree nodes separated by punctuation.

Lots of things in Rust are punctuated sequences.

- The fields of a struct are `Punctuated<Field, Token![,]>`.
- The segments of a path are `Punctuated<PathSegment, Token![::]>`.
- The bounds on a generic parameter are `Punctuated<TypeParamBound,
  Token![+]>`.
- The arguments to a function call are `Punctuated<Expr, Token![,]>`.

This module provides a common representation for these punctuated sequences
in the form of the [`Punctuated<T, P>`](#punctuated) type. We store a vector of pairs of
syntax tree node + punctuation, where every node in the sequence is followed
by punctuation except for possibly the final one.

```text
a_function_call(arg1, arg2, arg3);
                ~~~~^ ~~~~^ ~~~~
```

## Structs

### `Punctuated<T, P>`

```rust
struct Punctuated<T, P> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}
```

**A punctuated sequence of syntax tree nodes of type `T` separated by
punctuation of type `P`.**

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Implementations

- `const fn new() -> Self`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn first(self: &Self) -> Option<&T>`

- `fn first_mut(self: &mut Self) -> Option<&mut T>`

- `fn last(self: &Self) -> Option<&T>`

- `fn last_mut(self: &mut Self) -> Option<&mut T>`

- `fn get(self: &Self, index: usize) -> Option<&T>`

- `fn get_mut(self: &mut Self, index: usize) -> Option<&mut T>`

- `fn iter(self: &Self) -> Iter<'_, T>` — [`Iter`](../../punctuated/index.md)

- `fn iter_mut(self: &mut Self) -> IterMut<'_, T>` — [`IterMut`](../../punctuated/index.md)

- `fn pairs(self: &Self) -> Pairs<'_, T, P>` — [`Pairs`](../../punctuated/index.md)

- `fn pairs_mut(self: &mut Self) -> PairsMut<'_, T, P>` — [`PairsMut`](../../punctuated/index.md)

- `fn into_pairs(self: Self) -> IntoPairs<T, P>` — [`IntoPairs`](../../punctuated/index.md)

- `fn push_value(self: &mut Self, value: T)`

- `fn push_punct(self: &mut Self, punctuation: P)`

- `fn pop(self: &mut Self) -> Option<Pair<T, P>>` — [`Pair`](../../punctuated/index.md)

- `fn pop_punct(self: &mut Self) -> Option<P>`

- `fn trailing_punct(self: &Self) -> bool`

- `fn empty_or_trailing(self: &Self) -> bool`

- `fn push(self: &mut Self, value: T)`

- `fn insert(self: &mut Self, index: usize, value: T)`

- `fn clear(self: &mut Self)`

- `fn parse_terminated(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

- `fn parse_terminated_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

- `fn parse_separated_nonempty(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

- `fn parse_separated_nonempty_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../../parse/index.md), [`Result`](../../error/index.md)

#### Trait Implementations

##### `impl Clone<T, P>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, other: &Self)`

##### `impl Default<T, P>`

- `fn default() -> Self`

##### `impl Extend<T, P>`

- `fn extend<I: IntoIterator<Item = Pair<T, P>>>(self: &mut Self, i: I)`

##### `impl FromIterator<T, P>`

- `fn from_iter<I: IntoIterator<Item = Pair<T, P>>>(i: I) -> Self`

##### `impl Index<T, P>`

- `type Output = T`

- `fn index(self: &Self, index: usize) -> &<Self as >::Output`

##### `impl IndexMut<T, P>`

- `fn index_mut(self: &mut Self, index: usize) -> &mut <Self as >::Output`

##### `impl IntoIterator<T, P>`

- `type Item = T`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens<T, P>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Pairs<'a, T: 'a, P: 'a>`

```rust
struct Pairs<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: option::IntoIter<&'a T>,
}
```

An iterator over borrowed pairs of type `Pair<&T, &P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl Clone<'a, T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl DoubleEndedIterator<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, T, P>`

- `type Item = Pair<&'a T, &'a P>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `PairsMut<'a, T: 'a, P: 'a>`

```rust
struct PairsMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: option::IntoIter<&'a mut T>,
}
```

An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl DoubleEndedIterator<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, T, P>`

- `type Item = Pair<&'a mut T, &'a mut P>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IntoPairs<T, P>`

```rust
struct IntoPairs<T, P> {
    inner: vec::IntoIter<(T, P)>,
    last: option::IntoIter<T>,
}
```

An iterator over owned pairs of type `Pair<T, P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl Clone<T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl DoubleEndedIterator<T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<T, P>`

- `fn len(self: &Self) -> usize`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<T, P>`

- `type Item = Pair<T, P>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

An iterator over owned values of type `T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl DoubleEndedIterator<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<T>`

- `fn len(self: &Self) -> usize`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Iter<'a, T: 'a>`

```rust
struct Iter<'a, T: 'a> {
    inner: Box<crate::drops::NoDrop<dyn IterTrait<'a, T>>>,
}
```

An iterator over borrowed values of type `&T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl Clone<'a, T>`

- `fn clone(self: &Self) -> Self`

##### `impl DoubleEndedIterator<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T>`

- `fn len(self: &Self) -> usize`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IterMut<'a, T: 'a>`

```rust
struct IterMut<'a, T: 'a> {
    inner: Box<crate::drops::NoDrop<dyn IterMutTrait<'a, T, Item = &'a mut T>>>,
}
```

An iterator over mutably borrowed values of type `&mut T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl DoubleEndedIterator<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T>`

- `fn len(self: &Self) -> usize`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

## Enums

### `Pair<T, P>`

```rust
enum Pair<T, P> {
    Punctuated(T, P),
    End(T),
}
```

A single syntax tree node of type `T` followed by its trailing punctuation
of type `P` if any.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Implementations

- `fn into_value(self: Self) -> T`

- `fn value(self: &Self) -> &T`

- `fn value_mut(self: &mut Self) -> &mut T`

- `fn punct(self: &Self) -> Option<&P>`

- `fn punct_mut(self: &mut Self) -> Option<&mut P>`

- `fn new(t: T, p: Option<P>) -> Self`

- `fn into_tuple(self: Self) -> (T, Option<P>)`

#### Trait Implementations

##### `impl Clone<T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl Copy<T, P>`

##### `impl Sealed<T>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens<T, P>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

