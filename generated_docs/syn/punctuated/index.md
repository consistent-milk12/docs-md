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

## Contents

- [Modules](#modules)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Punctuated`](#punctuated)
  - [`Pairs`](#pairs)
  - [`PairsMut`](#pairsmut)
  - [`IntoPairs`](#intopairs)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
  - [`PrivateIter`](#privateiter)
  - [`IterMut`](#itermut)
  - [`PrivateIterMut`](#privateitermut)
- [Enums](#enums)
  - [`Pair`](#pair)
- [Traits](#traits)
  - [`IterTrait`](#itertrait)
  - [`IterMutTrait`](#itermuttrait)
- [Functions](#functions)
  - [`do_extend`](#do_extend)
  - [`empty_punctuated_iter`](#empty_punctuated_iter)
  - [`empty_punctuated_iter_mut`](#empty_punctuated_iter_mut)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`printing`](#printing) | mod |  |
| [`Punctuated`](#punctuated) | struct | **A punctuated sequence of syntax tree nodes of type `T` separated by |
| [`Pairs`](#pairs) | struct | An iterator over borrowed pairs of type `Pair<&T, &P>`. |
| [`PairsMut`](#pairsmut) | struct | An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`. |
| [`IntoPairs`](#intopairs) | struct | An iterator over owned pairs of type `Pair<T, P>`. |
| [`IntoIter`](#intoiter) | struct | An iterator over owned values of type `T`. |
| [`Iter`](#iter) | struct | An iterator over borrowed values of type `&T`. |
| [`PrivateIter`](#privateiter) | struct |  |
| [`IterMut`](#itermut) | struct | An iterator over mutably borrowed values of type `&mut T`. |
| [`PrivateIterMut`](#privateitermut) | struct |  |
| [`Pair`](#pair) | enum | A single syntax tree node of type `T` followed by its trailing punctuation |
| [`IterTrait`](#itertrait) | trait |  |
| [`IterMutTrait`](#itermuttrait) | trait |  |
| [`do_extend`](#do_extend) | fn |  |
| [`empty_punctuated_iter`](#empty_punctuated_iter) | fn |  |
| [`empty_punctuated_iter_mut`](#empty_punctuated_iter_mut) | fn |  |

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

- <span id="punctuated-new"></span>`const fn new() -> Self`

- <span id="punctuated-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="punctuated-len"></span>`fn len(&self) -> usize`

- <span id="punctuated-first"></span>`fn first(&self) -> Option<&T>`

- <span id="punctuated-first-mut"></span>`fn first_mut(&mut self) -> Option<&mut T>`

- <span id="punctuated-last"></span>`fn last(&self) -> Option<&T>`

- <span id="punctuated-last-mut"></span>`fn last_mut(&mut self) -> Option<&mut T>`

- <span id="punctuated-get"></span>`fn get(&self, index: usize) -> Option<&T>`

- <span id="punctuated-get-mut"></span>`fn get_mut(&mut self, index: usize) -> Option<&mut T>`

- <span id="punctuated-iter"></span>`fn iter(&self) -> Iter<'_, T>` — [`Iter`](#iter)

- <span id="punctuated-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, T>` — [`IterMut`](#itermut)

- <span id="punctuated-pairs"></span>`fn pairs(&self) -> Pairs<'_, T, P>` — [`Pairs`](#pairs)

- <span id="punctuated-pairs-mut"></span>`fn pairs_mut(&mut self) -> PairsMut<'_, T, P>` — [`PairsMut`](#pairsmut)

- <span id="punctuated-into-pairs"></span>`fn into_pairs(self) -> IntoPairs<T, P>` — [`IntoPairs`](#intopairs)

- <span id="punctuated-push-value"></span>`fn push_value(&mut self, value: T)`

- <span id="punctuated-push-punct"></span>`fn push_punct(&mut self, punctuation: P)`

- <span id="punctuated-pop"></span>`fn pop(&mut self) -> Option<Pair<T, P>>` — [`Pair`](#pair)

- <span id="punctuated-pop-punct"></span>`fn pop_punct(&mut self) -> Option<P>`

- <span id="punctuated-trailing-punct"></span>`fn trailing_punct(&self) -> bool`

- <span id="punctuated-empty-or-trailing"></span>`fn empty_or_trailing(&self) -> bool`

- <span id="punctuated-push"></span>`fn push(&mut self, value: T)`

- <span id="punctuated-insert"></span>`fn insert(&mut self, index: usize, value: T)`

- <span id="punctuated-clear"></span>`fn clear(&mut self)`

- <span id="punctuated-parse-terminated"></span>`fn parse_terminated(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="punctuated-parse-terminated-with"></span>`fn parse_terminated_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="punctuated-parse-separated-nonempty"></span>`fn parse_separated_nonempty(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="punctuated-parse-separated-nonempty-with"></span>`fn parse_separated_nonempty_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl<T, P> Clone for Punctuated<T, P>`

- <span id="punctuated-clone"></span>`fn clone(&self) -> Self`

- <span id="punctuated-clone-from"></span>`fn clone_from(&mut self, other: &Self)`

##### `impl<T: Debug, P: Debug> Debug for Punctuated<T, P>`

- <span id="punctuated-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, P> Default for Punctuated<T, P>`

- <span id="punctuated-default"></span>`fn default() -> Self`

##### `impl<T, P> Eq for Punctuated<T, P>`

##### `impl<T, P> Extend for Punctuated<T, P>`

- <span id="punctuated-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, i: I)`

##### `impl<T, P> FromIterator for Punctuated<T, P>`

- <span id="punctuated-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(i: I) -> Self`

##### `impl<T, P> Hash for Punctuated<T, P>`

- <span id="punctuated-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T, P> Index for Punctuated<T, P>`

- <span id="punctuated-output"></span>`type Output = T`

- <span id="punctuated-index"></span>`fn index(&self, index: usize) -> &<Self as >::Output`

##### `impl<T, P> IndexMut for Punctuated<T, P>`

- <span id="punctuated-index-mut"></span>`fn index_mut(&mut self, index: usize) -> &mut <Self as >::Output`

##### `impl<T, P> IntoIterator for Punctuated<T, P>`

- <span id="punctuated-item"></span>`type Item = T`

- <span id="punctuated-intoiter"></span>`type IntoIter = IntoIter<T>`

- <span id="punctuated-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, P> PartialEq for Punctuated<T, P>`

- <span id="punctuated-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Punctuated<T, P>`

##### `impl<T> Spanned for Punctuated<T, P>`

- <span id="punctuated-span"></span>`fn span(&self) -> Span`

##### `impl<T, P> ToTokens for crate::punctuated::Punctuated<T, P>`

- <span id="cratepunctuatedpunctuated-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="pairs-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, T, P> DoubleEndedIterator for Pairs<'a, T, P>`

- <span id="pairs-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for Pairs<'a, T, P>`

- <span id="pairs-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for Pairs<'a, T, P>`

- <span id="pairs-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pairs-intoiter"></span>`type IntoIter = I`

- <span id="pairs-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T, P> Iterator for Pairs<'a, T, P>`

- <span id="pairs-item"></span>`type Item = Pair<&'a T, &'a P>`

- <span id="pairs-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="pairs-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="pairsmut-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for PairsMut<'a, T, P>`

- <span id="pairsmut-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for PairsMut<'a, T, P>`

- <span id="pairsmut-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pairsmut-intoiter"></span>`type IntoIter = I`

- <span id="pairsmut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T, P> Iterator for PairsMut<'a, T, P>`

- <span id="pairsmut-item"></span>`type Item = Pair<&'a mut T, &'a mut P>`

- <span id="pairsmut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="pairsmut-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="intopairs-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, P> DoubleEndedIterator for IntoPairs<T, P>`

- <span id="intopairs-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, P> ExactSizeIterator for IntoPairs<T, P>`

- <span id="intopairs-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for IntoPairs<T, P>`

- <span id="intopairs-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intopairs-intoiter"></span>`type IntoIter = I`

- <span id="intopairs-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, P> Iterator for IntoPairs<T, P>`

- <span id="intopairs-item"></span>`type Item = Pair<T, P>`

- <span id="intopairs-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intopairs-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> DoubleEndedIterator for IntoIter<T>`

- <span id="intoiter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IntoIter<T>`

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for IntoIter<T>`

- <span id="intoiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IntoIter<T>`

- <span id="intoiter-item"></span>`type Item = T`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, T> DoubleEndedIterator for Iter<'a, T>`

- <span id="iter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for Iter<'a, T>`

- <span id="iter-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for Iter<'a, T>`

- <span id="iter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for Iter<'a, T>`

- <span id="iter-item"></span>`type Item = &'a T`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `PrivateIter<'a, T: 'a, P: 'a>`

```rust
struct PrivateIter<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: option::IntoIter<&'a T>,
}
```

#### Trait Implementations

##### `impl<'a, T, P> Clone for PrivateIter<'a, T, P>`

- <span id="privateiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, T, P> DoubleEndedIterator for PrivateIter<'a, T, P>`

- <span id="privateiter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for PrivateIter<'a, T, P>`

- <span id="privateiter-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for PrivateIter<'a, T, P>`

- <span id="privateiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="privateiter-intoiter"></span>`type IntoIter = I`

- <span id="privateiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T, P> Iterator for PrivateIter<'a, T, P>`

- <span id="privateiter-item"></span>`type Item = &'a T`

- <span id="privateiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="itermut-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for IterMut<'a, T>`

- <span id="itermut-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for IterMut<'a, T>`

- <span id="itermut-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiter"></span>`type IntoIter = I`

- <span id="itermut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for IterMut<'a, T>`

- <span id="itermut-item"></span>`type Item = &'a mut T`

- <span id="itermut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `PrivateIterMut<'a, T: 'a, P: 'a>`

```rust
struct PrivateIterMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: option::IntoIter<&'a mut T>,
}
```

#### Trait Implementations

##### `impl<'a, T, P> DoubleEndedIterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T, P> ExactSizeIterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-len"></span>`fn len(&self) -> usize`

##### `impl<I> IntoIterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-item"></span>`type Item = <I as Iterator>::Item`

- <span id="privateitermut-intoiter"></span>`type IntoIter = I`

- <span id="privateitermut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T, P> Iterator for PrivateIterMut<'a, T, P>`

- <span id="privateitermut-item"></span>`type Item = &'a mut T`

- <span id="privateitermut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="pair-cloned"></span>`fn cloned(self) -> Pair<T, P>` — [`Pair`](#pair)

#### Trait Implementations

##### `impl<T, P> Clone for Pair<T, P>`

- <span id="pair-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, P> Copy for Pair<T, P>`

##### `impl<T> Sealed for Pair<T, P>`

##### `impl<T> Spanned for Pair<T, P>`

- <span id="pair-span"></span>`fn span(&self) -> Span`

##### `impl<T, P> ToTokens for crate::punctuated::Pair<T, P>`

- <span id="cratepunctuatedpair-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Traits

### `IterTrait<'a, T: 'a>`

```rust
trait IterTrait<'a, T: 'a>: Iterator<Item = &'a T> + DoubleEndedIterator + ExactSizeIterator { ... }
```

#### Required Methods

- `fn clone_box(&self) -> Box<NoDrop<dyn IterTrait<'a, T>>>`

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

