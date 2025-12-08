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
in the form of the `Punctuated<T, P>` type. We store a vector of pairs of
syntax tree node + punctuation, where every node in the sequence is followed
by punctuation except for possibly the final one.

```text
a_function_call(arg1, arg2, arg3);
                ~~~~^ ~~~~^ ~~~~
```

## Modules

- [`printing`](printing/index.md) - 

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

- `fn iter(self: &Self) -> Iter<'_, T>` — [`Iter`](#iter)

- `fn iter_mut(self: &mut Self) -> IterMut<'_, T>` — [`IterMut`](#itermut)

- `fn pairs(self: &Self) -> Pairs<'_, T, P>` — [`Pairs`](#pairs)

- `fn pairs_mut(self: &mut Self) -> PairsMut<'_, T, P>` — [`PairsMut`](#pairsmut)

- `fn into_pairs(self: Self) -> IntoPairs<T, P>` — [`IntoPairs`](#intopairs)

- `fn push_value(self: &mut Self, value: T)`

- `fn push_punct(self: &mut Self, punctuation: P)`

- `fn pop(self: &mut Self) -> Option<Pair<T, P>>` — [`Pair`](#pair)

- `fn pop_punct(self: &mut Self) -> Option<P>`

- `fn trailing_punct(self: &Self) -> bool`

- `fn empty_or_trailing(self: &Self) -> bool`

- `fn push(self: &mut Self, value: T)`

- `fn insert(self: &mut Self, index: usize, value: T)`

- `fn clear(self: &mut Self)`

- `fn parse_terminated(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- `fn parse_terminated_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- `fn parse_separated_nonempty(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- `fn parse_separated_nonempty_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl<T, P> Clone for Punctuated<T, P>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, other: &Self)`

##### `impl<T: Debug, P: Debug> Debug for Punctuated<T, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, P> Default for Punctuated<T, P>`

- `fn default() -> Self`

##### `impl<T, P> Eq for Punctuated<T, P>`

##### `impl<T, P> Extend for Punctuated<T, P>`

- `fn extend<I: IntoIterator<Item = T>>(self: &mut Self, i: I)`

##### `impl<T, P> FromIterator for Punctuated<T, P>`

- `fn from_iter<I: IntoIterator<Item = Pair<T, P>>>(i: I) -> Self`

##### `impl<T, P> Hash for Punctuated<T, P>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<T, P> Index for Punctuated<T, P>`

- `type Output = T`

- `fn index(self: &Self, index: usize) -> &<Self as >::Output`

##### `impl<T, P> IndexMut for Punctuated<T, P>`

- `fn index_mut(self: &mut Self, index: usize) -> &mut <Self as >::Output`

##### `impl<T, P> IntoIterator for Punctuated<T, P>`

- `type Item = T`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<T, P> PartialEq for Punctuated<T, P>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Punctuated<T, P>`

##### `impl<T> Spanned for Punctuated<T, P>`

- `fn span(self: &Self) -> Span`

##### `impl<T, P> ToTokens for crate::punctuated::Punctuated<T, P>`

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


#### Trait Implementations

##### `impl<'a, T, P> Clone for Pairs<'a, T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, T, P> DoubleEndedIterator for Pairs<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for Pairs<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for Pairs<'a, T, P>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, P> Iterator for Pairs<'a, T, P>`

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


#### Trait Implementations

##### `impl<'a, T, P> DoubleEndedIterator for PairsMut<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for PairsMut<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for PairsMut<'a, T, P>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, P> Iterator for PairsMut<'a, T, P>`

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


#### Trait Implementations

##### `impl<T, P> Clone for IntoPairs<T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, P> DoubleEndedIterator for IntoPairs<T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T, P> ExactSizeIterator for IntoPairs<T, P>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for IntoPairs<T, P>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, P> Iterator for IntoPairs<T, P>`

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


#### Trait Implementations

##### `impl<T> Clone for IntoIter<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> DoubleEndedIterator for IntoIter<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IntoIter<T>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for IntoIter<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for IntoIter<T>`

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


#### Trait Implementations

##### `impl<'a, T> Clone for Iter<'a, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, T> DoubleEndedIterator for Iter<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for Iter<'a, T>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for Iter<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for Iter<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `PrivateIter<'a, T: 'a, P: 'a>`

```rust
struct PrivateIter<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: option::IntoIter<&'a T>,
}
```

#### Trait Implementations

##### `impl<'a, T, P> Clone for PrivateIter<'a, T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, T, P> DoubleEndedIterator for PrivateIter<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for PrivateIter<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for PrivateIter<'a, T, P>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, P> Iterator for PrivateIter<'a, T, P>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> TrivialDrop for PrivateIter<'a, T, P>`

### `IterMut<'a, T: 'a>`

```rust
struct IterMut<'a, T: 'a> {
    inner: Box<crate::drops::NoDrop<dyn IterMutTrait<'a, T, Item = &'a mut T>>>,
}
```

An iterator over mutably borrowed values of type `&mut T`.

Refer to the [module documentation] for details about punctuated sequences.


#### Trait Implementations

##### `impl<'a, T> DoubleEndedIterator for IterMut<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for IterMut<'a, T>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for IterMut<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for IterMut<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `PrivateIterMut<'a, T: 'a, P: 'a>`

```rust
struct PrivateIterMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: option::IntoIter<&'a mut T>,
}
```

#### Trait Implementations

##### `impl<'a, T, P> DoubleEndedIterator for PrivateIterMut<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for PrivateIterMut<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for PrivateIterMut<'a, T, P>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, P> Iterator for PrivateIterMut<'a, T, P>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> TrivialDrop for PrivateIterMut<'a, T, P>`

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


#### Implementations

- `fn cloned(self: Self) -> Pair<T, P>` — [`Pair`](#pair)

#### Trait Implementations

##### `impl<T, P> Clone for Pair<T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, P> Copy for Pair<T, P>`

##### `impl<T> Sealed for Pair<T, P>`

##### `impl<T> Spanned for Pair<T, P>`

- `fn span(self: &Self) -> Span`

##### `impl<T, P> ToTokens for crate::punctuated::Pair<T, P>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Traits

### `IterTrait<'a, T: 'a>`

```rust
trait IterTrait<'a, T: 'a>: Iterator<Item = &'a T> + DoubleEndedIterator + ExactSizeIterator { ... }
```

#### Required Methods

- `fn clone_box(self: &Self) -> Box<NoDrop<dyn IterTrait<'a, T>>>`

### `IterMutTrait<'a, T: 'a>`

```rust
trait IterMutTrait<'a, T: 'a>: DoubleEndedIterator<Item = &'a mut T> + ExactSizeIterator<Item = &'a mut T> { ... }
```

## Functions

### `do_extend`

```rust
fn do_extend<T, P, I>(punctuated: &mut Punctuated<T, P>, i: I)
where
    I: Iterator<Item = Pair<T, P>>
```

### `empty_punctuated_iter`

```rust
fn empty_punctuated_iter<'a, T>() -> Iter<'a, T>
```

### `empty_punctuated_iter_mut`

```rust
fn empty_punctuated_iter_mut<'a, T>() -> IterMut<'a, T>
```

