*[regex_automata](../../index.md) / [util](../index.md) / [iter](index.md)*

---

# Module `iter`

Generic helpers for iteration of matches from a regex engine in a haystack.

The principle type in this module is a [`Searcher`](regex_automata/util/iter/index.md). A `Searcher` provides
its own lower level iterator-like API in addition to methods for constructing
types that implement `Iterator`. The documentation for `Searcher` explains a
bit more about why these different APIs exist.

Currently, this module supports iteration over any regex engine that works
with the [`HalfMatch`](#halfmatch), [`Match`](../../../syn/syn/token/index.md) or [`Captures`](regex_automata/util/captures/index.md) types.

## Structs

### `Searcher<'h>`

```rust
struct Searcher<'h> {
}
```

A searcher for creating iterators and performing lower level iteration.

This searcher encapsulates the logic required for finding all successive
non-overlapping matches in a haystack. In theory, iteration would look
something like this:

1. Setting the start position to `0`.
2. Execute a regex search. If no match, end iteration.
3. Report the match and set the start position to the end of the match.
4. Go back to (2).

And if this were indeed the case, it's likely that `Searcher` wouldn't
exist. Unfortunately, because a regex may match the empty string, the above
logic won't work for all possible regexes. Namely, if an empty match is
found, then step (3) would set the start position of the search to the
position it was at. Thus, iteration would never end.

Instead, a `Searcher` knows how to detect these cases and forcefully
advance iteration in the case of an empty match that overlaps with a
previous match.

If you know that your regex cannot match any empty string, then the simple
algorithm described above will work correctly.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

In particular, a `Searcher` is not itself an iterator. Instead, it provides
`advance` routines that permit moving the search along explicitly. It also
provides various routines, like [`Searcher::into_matches_iter`](#into-matches-iter), that
accept a closure (representing how a regex engine executes a search) and
returns a conventional iterator.

The lifetime parameters come from the [`Input`](#input) type passed to
[`Searcher::new`](#new):

* `'h` is the lifetime of the underlying haystack.

# Searcher vs Iterator

Why does a search type with "advance" APIs exist at all when we also have
iterators? Unfortunately, the reasoning behind this split is a complex
combination of the following things:

1. While many of the regex engines expose their own iterators, it is also
nice to expose this lower level iteration helper because it permits callers
to provide their own `Input` configuration. Moreover, a `Searcher` can work
with _any_ regex engine instead of only the ones defined in this crate.
This way, everyone benefits from a shared iteration implementation.
2. There are many different regex engines that, while they have the same
match semantics, they have slightly different APIs. Iteration is just
complex enough to want to share code, and so we need a way of abstracting
over those different regex engines. While we could define a new trait that
describes any regex engine search API, it would wind up looking very close
to a closure. While there may still be reasons for the more generic trait
to exist, for now and for the purposes of iteration, we use a closure.
Closures also provide a lot of easy flexibility at the call site, in that
they permit the caller to borrow any kind of state they want for use during
each search call.
3. As a result of using closures, and because closures are anonymous types
that cannot be named, it is difficult to encapsulate them without both
costs to speed and added complexity to the public API. For example, in
defining an iterator type like
[`dfa::regex::FindMatches`](crate::dfa::regex::FindMatches),
if we use a closure internally, it's not possible to name this type in the
return type of the iterator constructor. Thus, the only way around it is
to erase the type by boxing it and turning it into a `Box<dyn FnMut ...>`.
This boxed closure is unlikely to be inlined _and_ it infects the public
API in subtle ways. Namely, unless you declare the closure as implementing
`Send` and `Sync`, then the resulting iterator type won't implement it
either. But there are practical issues with requiring the closure to
implement `Send` and `Sync` that result in other API complexities that
are beyond the scope of this already long exposition.
4. Some regex engines expose more complex match information than just
"which pattern matched" and "at what offsets." For example, the PikeVM
exposes match spans for each capturing group that participated in the
match. In such cases, it can be quite beneficial to reuse the capturing
group allocation on subsequent searches. A proper iterator doesn't permit
this API due to its interface, so it's useful to have something a bit lower
level that permits callers to amortize allocations while also reusing a
shared implementation of iteration. (See the documentation for
[`Searcher::advance`](#advance) for an example of using the "advance" API with the
PikeVM.)

What this boils down to is that there are "advance" APIs which require
handing a closure to it for every call, and there are also APIs to create
iterators from a closure. The former are useful for _implementing_
iterators or when you need more flexibility, while the latter are useful
for conveniently writing custom iterators on-the-fly.

# Example: iterating with captures

Several regex engines in this crate over convenient iterator APIs over
[`Captures`](regex_automata/util/captures/index.md) values. To do so, this requires allocating a new `Captures`
value for each iteration step. This can perhaps be more costly than you
might want. Instead of implementing your own iterator to avoid that
cost (which can be a little subtle if you want to handle empty matches
correctly), you can use this `Searcher` to do it for you:

```
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::iter::Searcher,
    Input, Span,
};

let re = PikeVM::new("foo(?P<numbers>[0-9]+)")?;
let haystack = "foo1 foo12 foo123";

let mut caps = re.create_captures();
let mut cache = re.create_cache();
let mut matches = vec![];
let mut searcher = Searcher::new(Input::new(haystack));
while let Some(_) = searcher.advance(|input| {
    re.search(&mut cache, input, &mut caps);
    Ok(caps.get_match())
}) {
    // The unwrap is OK since 'numbers' matches if the pattern matches.
    matches.push(caps.get_group_by_name("numbers").unwrap());
}
assert_eq!(matches, vec![
    Span::from(3..4),
    Span::from(8..10),
    Span::from(14..17),
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new(input: Input<'h>) -> Searcher<'h>`
  Create a new fallible non-overlapping matches iterator.

- `fn input<'s>(self: &'s Self) -> &'s Input<'h>`
  Returns the current `Input` used by this searcher.

- `fn advance_half<F>(self: &mut Self, finder: F) -> Option<HalfMatch>`
  Return the next half match for an infallible search if one exists, and

- `fn advance<F>(self: &mut Self, finder: F) -> Option<Match>`
  Return the next match for an infallible search if one exists, and

- `fn try_advance_half<F>(self: &mut Self, finder: F) -> Result<Option<HalfMatch>, MatchError>`
  Return the next half match for a fallible search if one exists, and

- `fn try_advance<F>(self: &mut Self, finder: F) -> Result<Option<Match>, MatchError>`
  Return the next match for a fallible search if one exists, and advance

- `fn into_half_matches_iter<F>(self: Self, finder: F) -> TryHalfMatchesIter<'h, F>`
  Given a closure that executes a single search, return an iterator over

- `fn into_matches_iter<F>(self: Self, finder: F) -> TryMatchesIter<'h, F>`
  Given a closure that executes a single search, return an iterator over

- `fn into_captures_iter<F>(self: Self, caps: Captures, finder: F) -> TryCapturesIter<'h, F>`
  Given a closure that executes a single search, return an iterator over

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

##### `impl Clone<'h>`

- `fn clone(self: &Self) -> Searcher<'h>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Debug<'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TryHalfMatchesIter<'h, F>`

```rust
struct TryHalfMatchesIter<'h, F> {
}
```

An iterator over all non-overlapping half matches for a fallible search.

The iterator yields a `Result<HalfMatch, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](#input) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_half_matches_iter`](#into-half-matches-iter).

#### Implementations

- `fn infallible(self: Self) -> HalfMatchesIter<'h, F>`
  Return an infallible version of this iterator.

- `fn input<'i>(self: &'i Self) -> &'i Input<'h>`
  Returns the current `Input` used by this iterator.

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

##### `impl Iterator<'h, F>`

- `type Item = Result<HalfMatch, MatchError>`

- `fn next(self: &mut Self) -> Option<Result<HalfMatch, MatchError>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, F>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `HalfMatchesIter<'h, F>`

```rust
struct HalfMatchesIter<'h, F>();
```

An iterator over all non-overlapping half matches for an infallible search.

The iterator yields a [`HalfMatch`](#halfmatch) value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](#input) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_half_matches_iter`](#into-half-matches-iter) and
then calling [`TryHalfMatchesIter::infallible`](#infallible).

#### Implementations

- `fn input<'i>(self: &'i Self) -> &'i Input<'h>`
  Returns the current `Input` used by this iterator.

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

##### `impl Iterator<'h, F>`

- `type Item = HalfMatch`

- `fn next(self: &mut Self) -> Option<HalfMatch>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, F: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TryMatchesIter<'h, F>`

```rust
struct TryMatchesIter<'h, F> {
}
```

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](#input) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_matches_iter`](#into-matches-iter).

#### Implementations

- `fn infallible(self: Self) -> MatchesIter<'h, F>`
  Return an infallible version of this iterator.

- `fn input<'i>(self: &'i Self) -> &'i Input<'h>`
  Returns the current `Input` used by this iterator.

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

##### `impl Iterator<'h, F>`

- `type Item = Result<Match, MatchError>`

- `fn next(self: &mut Self) -> Option<Result<Match, MatchError>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, F>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `MatchesIter<'h, F>`

```rust
struct MatchesIter<'h, F>();
```

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`](../../../syn/syn/token/index.md) value until no more matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](#input) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_matches_iter`](#into-matches-iter) and
then calling [`TryMatchesIter::infallible`](#infallible).

#### Implementations

- `fn input<'i>(self: &'i Self) -> &'i Input<'h>`
  Returns the current `Input` used by this iterator.

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

##### `impl Iterator<'h, F>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, F: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TryCapturesIter<'h, F>`

```rust
struct TryCapturesIter<'h, F> {
}
```

An iterator over all non-overlapping captures for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](#input) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_captures_iter`](#into-captures-iter).

#### Implementations

- `fn infallible(self: Self) -> CapturesIter<'h, F>`
  Return an infallible version of this iterator.

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

##### `impl Iterator<'h, F>`

- `type Item = Result<Captures, MatchError>`

- `fn next(self: &mut Self) -> Option<Result<Captures, MatchError>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, F>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CapturesIter<'h, F>`

```rust
struct CapturesIter<'h, F>();
```

An iterator over all non-overlapping captures for an infallible search.

The iterator yields a [`Captures`](regex_automata/util/captures/index.md) value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](#input) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by [`Searcher::into_captures_iter`](#into-captures-iter) and then
calling [`TryCapturesIter::infallible`](#infallible).

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

##### `impl Iterator<'h, F>`

- `type Item = Captures`

- `fn next(self: &mut Self) -> Option<Captures>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, F: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

