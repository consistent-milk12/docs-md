*[regex_syntax](../../index.md) / [hir](../index.md) / [literal](index.md)*

---

# Module `literal`

Provides literal extraction from `Hir` expressions.

An [`Extractor`](regex_syntax/hir/literal/index.md) pulls literals out of [`Hir`](regex_syntax/hir/index.md) expressions and returns a
[`Seq`](regex_syntax/hir/literal/index.md) of [`Literal`](regex_syntax/ast/index.md)s.

The purpose of literal extraction is generally to provide avenues for
optimizing regex searches. The main idea is that substring searches can be an
order of magnitude faster than a regex search. Therefore, if one can execute
a substring search to find candidate match locations and only run the regex
search at those locations, then it is possible for huge improvements in
performance to be realized.

With that said, literal optimizations are generally a black art because even
though substring search is generally faster, if the number of candidates
produced is high, then it can create a lot of overhead by ping-ponging between
the substring search and the regex search.

Here are some heuristics that might be used to help increase the chances of
effective literal optimizations:

* Stick to small [`Seq`](regex_syntax/hir/literal/index.md)s. If you search for too many literals, it's likely
to lead to substring search that is only a little faster than a regex search,
and thus the overhead of using literal optimizations in the first place might
make things slower overall.
* The literals in your [`Seq`](regex_syntax/hir/literal/index.md) shouldn't be too short. In general, longer is
better. A sequence corresponding to single bytes that occur frequently in the
haystack, for example, is probably a bad literal optimization because it's
likely to produce many false positive candidates. Longer literals are less
likely to match, and thus probably produce fewer false positives.
* If it's possible to estimate the approximate frequency of each byte according
to some pre-computed background distribution, it is possible to compute a score
of how "good" a `Seq` is. If a `Seq` isn't good enough, you might consider
skipping the literal optimization and just use the regex engine.

(It should be noted that there are always pathological cases that can make
any kind of literal optimization be a net slower result. This is why it
might be a good idea to be conservative, or to even provide a means for
literal optimizations to be dynamically disabled if they are determined to be
ineffective according to some measure.)

You're encouraged to explore the methods on [`Seq`](regex_syntax/hir/literal/index.md), which permit shrinking
the size of sequences in a preference-order preserving fashion.

Finally, note that it isn't strictly necessary to use an [`Extractor`](regex_syntax/hir/literal/index.md). Namely,
an `Extractor` only uses public APIs of the [`Seq`](regex_syntax/hir/literal/index.md) and [`Literal`](regex_syntax/ast/index.md) types,
so it is possible to implement your own extractor. For example, for n-grams
or "inner" literals (i.e., not prefix or suffix literals). The `Extractor`
is mostly responsible for the case analysis over `Hir` expressions. Much of
the "trickier" parts are how to combine literal sequences, and that is all
implemented on [`Seq`](regex_syntax/hir/literal/index.md).

## Structs

### `Extractor`

```rust
struct Extractor {
}
```

Extracts prefix or suffix literal sequences from [`Hir`](regex_syntax/hir/index.md) expressions.

Literal extraction is based on the following observations:

* Many regexes start with one or a small number of literals.
* Substring search for literals is often much faster (sometimes by an order
of magnitude) than a regex search.

Thus, in many cases, one can search for literals to find candidate starting
locations of a match, and then only run the full regex engine at each such
location instead of over the full haystack.

The main downside of literal extraction is that it can wind up causing a
search to be slower overall. For example, if there are many matches or if
there are many candidates that don't ultimately lead to a match, then a
lot of overhead will be spent in shuffling back-and-forth between substring
search and the regex engine. This is the fundamental reason why literal
optimizations for regex patterns is sometimes considered a "black art."

# Look-around assertions

Literal extraction treats all look-around assertions as-if they match every
empty string. So for example, the regex `\bquux\b` will yield a sequence
containing a single exact literal `quux`. However, not all occurrences
of `quux` correspond to a match a of the regex. For example, `\bquux\b`
does not match `ZquuxZ` anywhere because `quux` does not fall on a word
boundary.

In effect, if your regex contains look-around assertions, then a match of
an exact literal does not necessarily mean the regex overall matches. So
you may still need to run the regex engine in such cases to confirm the
match.

The precise guarantee you get from a literal sequence is: if every literal
in the sequence is exact and the original regex contains zero look-around
assertions, then a preference-order multi-substring search of those
literals will precisely match a preference-order search of the original
regex.

# Example

This shows how to extract prefixes:

```
use regex_syntax::{hir::literal::{Extractor, Literal, Seq}, parse};

let hir = parse(r"(a|b|c)(x|y|z)[A-Z]+foo")?;
let got = Extractor::new().extract(&hir);
// All literals returned are "inexact" because none of them reach the
// match state.
let expected = Seq::from_iter([
    Literal::inexact("ax"),
    Literal::inexact("ay"),
    Literal::inexact("az"),
    Literal::inexact("bx"),
    Literal::inexact("by"),
    Literal::inexact("bz"),
    Literal::inexact("cx"),
    Literal::inexact("cy"),
    Literal::inexact("cz"),
]);
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

This shows how to extract suffixes:

```
use regex_syntax::{
    hir::literal::{Extractor, ExtractKind, Literal, Seq},
    parse,
};

let hir = parse(r"foo|[A-Z]+bar")?;
let got = Extractor::new().kind(ExtractKind::Suffix).extract(&hir);
// Since 'foo' gets to a match state, it is considered exact. But 'bar'
// does not because of the '[A-Z]+', and thus is marked inexact.
let expected = Seq::from_iter([
    Literal::exact("foo"),
    Literal::inexact("bar"),
]);
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Extractor`
  Create a new extractor with a default configuration.

- `fn extract(self: &Self, hir: &Hir) -> Seq`
  Execute the extractor and return a sequence of literals.

- `fn kind(self: &mut Self, kind: ExtractKind) -> &mut Extractor`
  Set the kind of literal sequence to extract from an [`Hir`] expression.

- `fn limit_class(self: &mut Self, limit: usize) -> &mut Extractor`
  Configure a limit on the length of the sequence that is permitted for

- `fn limit_repeat(self: &mut Self, limit: usize) -> &mut Extractor`
  Configure a limit on the total number of repetitions that is permitted

- `fn limit_literal_len(self: &mut Self, limit: usize) -> &mut Extractor`
  Configure a limit on the maximum length of any literal in a sequence.

- `fn limit_total(self: &mut Self, limit: usize) -> &mut Extractor`
  Configure a limit on the total number of literals that will be

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

##### `impl Clone`

- `fn clone(self: &Self) -> Extractor`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Extractor`

### `Seq`

```rust
struct Seq {
}
```

A sequence of literals.

A `Seq` is very much like a set in that it represents a union of its
members. That is, it corresponds to a set of literals where at least one
must match in order for a particular [`Hir`](regex_syntax/hir/index.md) expression to match. (Whether
this corresponds to the entire `Hir` expression, a prefix of it or a suffix
of it depends on how the `Seq` was extracted from the `Hir`.)

It is also unlike a set in that multiple identical literals may appear,
and that the order of the literals in the `Seq` matters. For example, if
the sequence is `[sam, samwise]` and leftmost-first matching is used, then
`samwise` can never match and the sequence is equivalent to `[sam](#sam)`.

# States of a sequence

A `Seq` has a few different logical states to consider:

* The sequence can represent "any" literal. When this happens, the set does
not have a finite size. The purpose of this state is to inhibit callers
from making assumptions about what literals are required in order to match
a particular [`Hir`](regex_syntax/hir/index.md) expression. Generally speaking, when a set is in this
state, literal optimizations are inhibited. A good example of a regex that
will cause this sort of set to appear is `[A-Za-z]`. The character class
is just too big (and also too narrow) to be usefully expanded into 52
different literals. (Note that the decision for when a seq should become
infinite is determined by the caller. A seq itself has no hard-coded
limits.)
* The sequence can be empty, in which case, it is an affirmative statement
that there are no literals that can match the corresponding `Hir`.
Consequently, the `Hir` never matches any input. For example, `[a&&b]`.
* The sequence can be non-empty, in which case, at least one of the
literals must match in order for the corresponding `Hir` to match.

# Example

This example shows how literal sequences can be simplified by stripping
suffixes and minimizing while maintaining preference order.

```
use regex_syntax::hir::literal::{Literal, Seq};

let mut seq = Seq::new(&[
    "farm",
    "appliance",
    "faraway",
    "apple",
    "fare",
    "gap",
    "applicant",
    "applaud",
]);
seq.keep_first_bytes(3);
seq.minimize_by_preference();
// Notice that 'far' comes before 'app', which matches the order in the
// original sequence. This guarantees that leftmost-first semantics are
// not altered by simplifying the set.
let expected = Seq::from_iter([
    Literal::inexact("far"),
    Literal::inexact("app"),
    Literal::exact("gap"),
]);
assert_eq!(expected, seq);
```

#### Implementations

- `fn empty() -> Seq`
  Returns an empty sequence.

- `fn infinite() -> Seq`
  Returns a sequence of literals without a finite size and may contain

- `fn singleton(lit: Literal) -> Seq`
  Returns a sequence containing a single literal.

- `fn new<I, B>(it: I) -> Seq`
  Returns a sequence of exact literals from the given byte strings.

- `fn literals(self: &Self) -> Option<&[Literal]>`
  If this is a finite sequence, return its members as a slice of

- `fn push(self: &mut Self, lit: Literal)`
  Push a literal to the end of this sequence.

- `fn make_inexact(self: &mut Self)`
  Make all of the literals in this sequence inexact.

- `fn make_infinite(self: &mut Self)`
  Converts this sequence to an infinite sequence.

- `fn cross_forward(self: &mut Self, other: &mut Seq)`
  Modify this sequence to contain the cross product between it and the

- `fn cross_reverse(self: &mut Self, other: &mut Seq)`
  Modify this sequence to contain the cross product between it and

- `fn union(self: &mut Self, other: &mut Seq)`
  Unions the `other` sequence into this one.

- `fn union_into_empty(self: &mut Self, other: &mut Seq)`
  Unions the `other` sequence into this one by splice the `other`

- `fn dedup(self: &mut Self)`
  Deduplicate adjacent equivalent literals in this sequence.

- `fn sort(self: &mut Self)`
  Sorts this sequence of literals lexicographically.

- `fn reverse_literals(self: &mut Self)`
  Reverses all of the literals in this sequence.

- `fn minimize_by_preference(self: &mut Self)`
  Shrinks this seq to its minimal size while respecting the preference

- `fn keep_first_bytes(self: &mut Self, len: usize)`
  Trims all literals in this seq such that only the first `len` bytes

- `fn keep_last_bytes(self: &mut Self, len: usize)`
  Trims all literals in this seq such that only the last `len` bytes

- `fn is_finite(self: &Self) -> bool`
  Returns true if this sequence is finite.

- `fn is_empty(self: &Self) -> bool`
  Returns true if and only if this sequence is finite and empty.

- `fn len(self: &Self) -> Option<usize>`
  Returns the number of literals in this sequence if the sequence is

- `fn is_exact(self: &Self) -> bool`
  Returns true if and only if all literals in this sequence are exact.

- `fn is_inexact(self: &Self) -> bool`
  Returns true if and only if all literals in this sequence are inexact.

- `fn max_union_len(self: &Self, other: &Seq) -> Option<usize>`
  Return the maximum length of the sequence that would result from

- `fn max_cross_len(self: &Self, other: &Seq) -> Option<usize>`
  Return the maximum length of the sequence that would result from the

- `fn min_literal_len(self: &Self) -> Option<usize>`
  Returns the length of the shortest literal in this sequence.

- `fn max_literal_len(self: &Self) -> Option<usize>`
  Returns the length of the longest literal in this sequence.

- `fn longest_common_prefix(self: &Self) -> Option<&[u8]>`
  Returns the longest common prefix from this seq.

- `fn longest_common_suffix(self: &Self) -> Option<&[u8]>`
  Returns the longest common suffix from this seq.

- `fn optimize_for_prefix_by_preference(self: &mut Self)`
  Optimizes this seq while treating its literals as prefixes and

- `fn optimize_for_suffix_by_preference(self: &mut Self)`
  Optimizes this seq while treating its literals as suffixes and

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator`

- `fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Seq`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Seq) -> bool`

##### `impl StructuralPartialEq`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Literal`

```rust
struct Literal {
}
```

A single literal extracted from an [`Hir`](regex_syntax/hir/index.md) expression.

A literal is composed of two things:

* A sequence of bytes. No guarantees with respect to UTF-8 are provided.
In particular, even if the regex a literal is extracted from is UTF-8, the
literal extracted may not be valid UTF-8. (For example, if an [`Extractor`](regex_syntax/hir/literal/index.md)
limit resulted in trimming a literal in a way that splits a codepoint.)
* Whether the literal is "exact" or not. An "exact" literal means that it
has not been trimmed, and may continue to be extended. If a literal is
"exact" after visiting the entire `Hir` expression, then this implies that
the literal leads to a match state. (Although it doesn't necessarily imply
all occurrences of the literal correspond to a match of the regex, since
literal extraction ignores look-around assertions.)

#### Implementations

- `fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal`
  Returns a new exact literal containing the bytes given.

- `fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal`
  Returns a new inexact literal containing the bytes given.

- `fn as_bytes(self: &Self) -> &[u8]`
  Returns the bytes in this literal.

- `fn into_bytes(self: Self) -> Vec<u8>`
  Yields ownership of the bytes inside this literal.

- `fn len(self: &Self) -> usize`
  Returns the length of this literal in bytes.

- `fn is_empty(self: &Self) -> bool`
  Returns true if and only if this literal has zero bytes.

- `fn is_exact(self: &Self) -> bool`
  Returns true if and only if this literal is exact.

- `fn make_inexact(self: &mut Self)`
  Marks this literal as inexact.

- `fn reverse(self: &mut Self)`
  Reverse the bytes in this literal.

- `fn extend(self: &mut Self, lit: &Literal)`
  Extend this literal with the literal given.

- `fn keep_first_bytes(self: &mut Self, len: usize)`
  Trims this literal such that only the first `len` bytes remain. If

- `fn keep_last_bytes(self: &mut Self, len: usize)`
  Trims this literal such that only the last `len` bytes remain. If this

#### Trait Implementations

##### `impl From`

- `fn from(byte: u8) -> Literal`

##### `impl From`

- `fn from(ch: char) -> Literal`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Literal`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Literal) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Literal) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Literal) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Enums

### `ExtractKind`

```rust
enum ExtractKind {
    Prefix,
    Suffix,
}
```

The kind of literals to extract from an [`Hir`](regex_syntax/hir/index.md) expression.

The default extraction kind is `Prefix`.

#### Variants

- **`Prefix`**

  Extracts only prefix literals from a regex.

- **`Suffix`**

  Extracts only suffix literals from a regex.
  
  Note that the sequence returned by suffix literals currently may
  not correctly represent leftmost-first or "preference" order match
  semantics.

#### Implementations

- `fn is_prefix(self: &Self) -> bool`
  Returns true if this kind is the `Prefix` variant.

- `fn is_suffix(self: &Self) -> bool`
  Returns true if this kind is the `Suffix` variant.

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

##### `impl Clone`

- `fn clone(self: &Self) -> ExtractKind`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ExtractKind`

## Functions

### `rank`

```rust
fn rank(byte: u8) -> u8
```

Returns the "rank" of the given byte.

The minimum rank value is `0` and the maximum rank value is `255`.

The rank of a byte is derived from a heuristic background distribution of
relative frequencies of bytes. The heuristic says that lower the rank of a
byte, the less likely that byte is to appear in any arbitrary haystack.

