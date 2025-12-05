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
    // [REDACTED: Private Fields]
}
```

**A punctuated sequence of syntax tree nodes of type `T` separated by
punctuation of type `P`.**

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Implementations

- `const fn new() -> Self`
  Creates an empty punctuated sequence.

- `fn is_empty(self: &Self) -> bool`
  Determines whether this punctuated sequence is empty, meaning it

- `fn len(self: &Self) -> usize`
  Returns the number of syntax tree nodes in this punctuated sequence.

- `fn first(self: &Self) -> Option<&T>`
  Borrows the first element in this sequence.

- `fn first_mut(self: &mut Self) -> Option<&mut T>`
  Mutably borrows the first element in this sequence.

- `fn last(self: &Self) -> Option<&T>`
  Borrows the last element in this sequence.

- `fn last_mut(self: &mut Self) -> Option<&mut T>`
  Mutably borrows the last element in this sequence.

- `fn get(self: &Self, index: usize) -> Option<&T>`
  Borrows the element at the given index.

- `fn get_mut(self: &mut Self, index: usize) -> Option<&mut T>`
  Mutably borrows the element at the given index.

- `fn iter(self: &Self) -> Iter<'_, T>`
  Returns an iterator over borrowed syntax tree nodes of type `&T`.

- `fn iter_mut(self: &mut Self) -> IterMut<'_, T>`
  Returns an iterator over mutably borrowed syntax tree nodes of type

- `fn pairs(self: &Self) -> Pairs<'_, T, P>`
  Returns an iterator over the contents of this sequence as borrowed

- `fn pairs_mut(self: &mut Self) -> PairsMut<'_, T, P>`
  Returns an iterator over the contents of this sequence as mutably

- `fn into_pairs(self: Self) -> IntoPairs<T, P>`
  Returns an iterator over the contents of this sequence as owned

- `fn push_value(self: &mut Self, value: T)`
  Appends a syntax tree node onto the end of this punctuated sequence. The

- `fn push_punct(self: &mut Self, punctuation: P)`
  Appends a trailing punctuation onto the end of this punctuated sequence.

- `fn pop(self: &mut Self) -> Option<Pair<T, P>>`
  Removes the last punctuated pair from this sequence, or `None` if the

- `fn pop_punct(self: &mut Self) -> Option<P>`
  Removes the trailing punctuation from this punctuated sequence, or

- `fn trailing_punct(self: &Self) -> bool`
  Determines whether this punctuated sequence ends with a trailing

- `fn empty_or_trailing(self: &Self) -> bool`
  Returns true if either this `Punctuated` is empty, or it has a trailing

- `fn push(self: &mut Self, value: T)`
  Appends a syntax tree node onto the end of this punctuated sequence.

- `fn insert(self: &mut Self, index: usize, value: T)`
  Inserts an element at position `index`.

- `fn clear(self: &mut Self)`
  Clears the sequence of all values and punctuation, making it empty.

- `fn parse_terminated(input: ParseStream<'_>) -> Result<Self>`
  Parses zero or more occurrences of `T` separated by punctuation of type

- `fn parse_terminated_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>`
  Parses zero or more occurrences of `T` using the given parse function,

- `fn parse_separated_nonempty(input: ParseStream<'_>) -> Result<Self>`
  Parses one or more occurrences of `T` separated by punctuation of type

- `fn parse_separated_nonempty_with<'a>(input: ParseStream<'a>, parser: fn(ParseStream<'a>) -> Result<T>) -> Result<Self>`
  Parses one or more occurrences of `T` using the given parse function,

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator<T, P>`

- `fn from_iter<I: IntoIterator<Item = Pair<T, P>>>(i: I) -> Self`

##### `impl FromIterator<T, P>`

- `fn from_iter<I: IntoIterator<Item = T>>(i: I) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<T, P>`

- `type Item = T`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T, P>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, other: &Self)`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Extend<T, P>`

- `fn extend<I: IntoIterator<Item = Pair<T, P>>>(self: &mut Self, i: I)`

##### `impl Extend<T, P>`

- `fn extend<I: IntoIterator<Item = T>>(self: &mut Self, i: I)`

##### `impl Index<T, P>`

- `type Output = T`

- `fn index(self: &Self, index: usize) -> &<Self as >::Output`

##### `impl IndexMut<T, P>`

- `fn index_mut(self: &mut Self, index: usize) -> &mut <Self as >::Output`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens<T, P>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default<T, P>`

- `fn default() -> Self`

### `Pairs<'a, T: 'a, P: 'a>`

```rust
struct Pairs<'a, T: 'a, P: 'a> {
    // [REDACTED: Private Fields]
}
```

An iterator over borrowed pairs of type `Pair<&T, &P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a, T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl Iterator<'a, T, P>`

- `type Item = Pair<&'a T, &'a P>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `PairsMut<'a, T: 'a, P: 'a>`

```rust
struct PairsMut<'a, T: 'a, P: 'a> {
    // [REDACTED: Private Fields]
}
```

An iterator over mutably borrowed pairs of type `Pair<&mut T, &mut P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl DoubleEndedIterator<'a, T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T, P>`

- `fn len(self: &Self) -> usize`

##### `impl Iterator<'a, T, P>`

- `type Item = Pair<&'a mut T, &'a mut P>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `IntoPairs<T, P>`

```rust
struct IntoPairs<T, P> {
    // [REDACTED: Private Fields]
}
```

An iterator over owned pairs of type `Pair<T, P>`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<T, P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<T, P>`

- `fn len(self: &Self) -> usize`

##### `impl Iterator<T, P>`

- `type Item = Pair<T, P>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    // [REDACTED: Private Fields]
}
```

An iterator over owned values of type `T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<T>`

- `fn len(self: &Self) -> usize`

##### `impl Iterator<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, T: 'a>`

```rust
struct Iter<'a, T: 'a> {
    // [REDACTED: Private Fields]
}
```

An iterator over borrowed values of type `&T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a, T>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T>`

- `fn len(self: &Self) -> usize`

##### `impl Iterator<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'a, T: 'a>`

```rust
struct IterMut<'a, T: 'a> {
    // [REDACTED: Private Fields]
}
```

An iterator over mutably borrowed values of type `&mut T`.

Refer to the [module documentation] for details about punctuated sequences.

[module documentation]: self

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl DoubleEndedIterator<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T>`

- `fn len(self: &Self) -> usize`

##### `impl Iterator<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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
  Extracts the syntax tree node from this punctuated pair, discarding the

- `fn value(self: &Self) -> &T`
  Borrows the syntax tree node from this punctuated pair.

- `fn value_mut(self: &mut Self) -> &mut T`
  Mutably borrows the syntax tree node from this punctuated pair.

- `fn punct(self: &Self) -> Option<&P>`
  Borrows the punctuation from this punctuated pair, unless this pair is

- `fn punct_mut(self: &mut Self) -> Option<&mut P>`
  Mutably borrows the punctuation from this punctuated pair, unless the

- `fn new(t: T, p: Option<P>) -> Self`
  Creates a punctuated pair out of a syntax tree node and an optional

- `fn into_tuple(self: Self) -> (T, Option<P>)`
  Produces this punctuated pair as a tuple of syntax tree node and

- `fn cloned(self: Self) -> Pair<T, P>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<T, P>`

##### `impl Spanned<T>`

- `fn span(self: &Self) -> Span`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToTokens<T, P>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

