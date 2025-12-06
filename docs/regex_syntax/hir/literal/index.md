*[regex_syntax](../../index.md) / [hir](../index.md) / [literal](index.md)*

---

# Module `literal`

Provides literal extraction from `Hir` expressions.

An [`Extractor`](#extractor) pulls literals out of [`Hir`](../index.md) expressions and returns a
[`Seq`](#seq) of [`Literal`](../../index.md)s.

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

* Stick to small [`Seq`](#seq)s. If you search for too many literals, it's likely
to lead to substring search that is only a little faster than a regex search,
and thus the overhead of using literal optimizations in the first place might
make things slower overall.
* The literals in your [`Seq`](#seq) shouldn't be too short. In general, longer is
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

You're encouraged to explore the methods on [`Seq`](#seq), which permit shrinking
the size of sequences in a preference-order preserving fashion.

Finally, note that it isn't strictly necessary to use an [`Extractor`](#extractor). Namely,
an `Extractor` only uses public APIs of the [`Seq`](#seq) and [`Literal`](../../index.md) types,
so it is possible to implement your own extractor. For example, for n-grams
or "inner" literals (i.e., not prefix or suffix literals). The `Extractor`
is mostly responsible for the case analysis over `Hir` expressions. Much of
the "trickier" parts are how to combine literal sequences, and that is all
implemented on [`Seq`](#seq).

## Structs

### `Extractor`

```rust
struct Extractor {
    kind: ExtractKind,
    limit_class: usize,
    limit_repeat: usize,
    limit_literal_len: usize,
    limit_total: usize,
}
```

Extracts prefix or suffix literal sequences from [`Hir`](../index.md) expressions.

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

```rust
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

Ok::<(), Box<dyn std::error::Error>>(())
```

This shows how to extract suffixes:

```rust
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

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Extractor` — [`Extractor`](#extractor)

- `fn extract(self: &Self, hir: &Hir) -> Seq` — [`Hir`](../index.md), [`Seq`](#seq)

- `fn kind(self: &mut Self, kind: ExtractKind) -> &mut Extractor` — [`ExtractKind`](#extractkind), [`Extractor`](#extractor)

- `fn limit_class(self: &mut Self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- `fn limit_repeat(self: &mut Self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- `fn limit_literal_len(self: &mut Self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- `fn limit_total(self: &mut Self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- `fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(self: &Self, it: I) -> Seq` — [`Seq`](#seq)

- `fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>(self: &Self, it: I) -> Seq` — [`Seq`](#seq)

- `fn extract_repetition(self: &Self, rep: &hir::Repetition) -> Seq` — [`Repetition`](../index.md), [`Seq`](#seq)

- `fn extract_class_unicode(self: &Self, cls: &hir::ClassUnicode) -> Seq` — [`ClassUnicode`](../index.md), [`Seq`](#seq)

- `fn extract_class_bytes(self: &Self, cls: &hir::ClassBytes) -> Seq` — [`ClassBytes`](../index.md), [`Seq`](#seq)

- `fn class_over_limit_unicode(self: &Self, cls: &hir::ClassUnicode) -> bool` — [`ClassUnicode`](../index.md)

- `fn class_over_limit_bytes(self: &Self, cls: &hir::ClassBytes) -> bool` — [`ClassBytes`](../index.md)

- `fn cross(self: &Self, seq1: Seq, seq2: &mut Seq) -> Seq` — [`Seq`](#seq)

- `fn union(self: &Self, seq1: Seq, seq2: &mut Seq) -> Seq` — [`Seq`](#seq)

- `fn enforce_literal_len(self: &Self, seq: &mut Seq)` — [`Seq`](#seq)

#### Trait Implementations

##### `impl Clone for Extractor`

- `fn clone(self: &Self) -> Extractor` — [`Extractor`](#extractor)

##### `impl Debug for Extractor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Extractor`

- `fn default() -> Extractor` — [`Extractor`](#extractor)

### `Seq`

```rust
struct Seq {
    literals: Option<alloc::vec::Vec<Literal>>,
}
```

A sequence of literals.

A `Seq` is very much like a set in that it represents a union of its
members. That is, it corresponds to a set of literals where at least one
must match in order for a particular [`Hir`](../index.md) expression to match. (Whether
this corresponds to the entire `Hir` expression, a prefix of it or a suffix
of it depends on how the `Seq` was extracted from the `Hir`.)

It is also unlike a set in that multiple identical literals may appear,
and that the order of the literals in the `Seq` matters. For example, if
the sequence is `[sam, samwise]` and leftmost-first matching is used, then
`samwise` can never match and the sequence is equivalent to `[sam]`.

# States of a sequence

A `Seq` has a few different logical states to consider:

* The sequence can represent "any" literal. When this happens, the set does
not have a finite size. The purpose of this state is to inhibit callers
from making assumptions about what literals are required in order to match
a particular [`Hir`](../index.md) expression. Generally speaking, when a set is in this
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

```rust
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

#### Fields

- **`literals`**: `Option<alloc::vec::Vec<Literal>>`

  The members of this seq.
  
  When `None`, the seq represents all possible literals. That is, it
  prevents one from making assumptions about specific literals in the
  seq, and forces one to treat it as if any literal might be in the seq.
  
  Note that `Some(vec![])` is valid and corresponds to the empty seq of
  literals, i.e., a regex that can never match. For example, `[a&&b]`.
  It is distinct from `Some(vec![""])`, which corresponds to the seq
  containing an empty string, which matches at every position.

#### Implementations

- `fn empty() -> Seq` — [`Seq`](#seq)

- `fn infinite() -> Seq` — [`Seq`](#seq)

- `fn singleton(lit: Literal) -> Seq` — [`Literal`](#literal), [`Seq`](#seq)

- `fn new<I, B>(it: I) -> Seq` — [`Seq`](#seq)

- `fn literals(self: &Self) -> Option<&[Literal]>` — [`Literal`](#literal)

- `fn push(self: &mut Self, lit: Literal)` — [`Literal`](#literal)

- `fn make_inexact(self: &mut Self)`

- `fn make_infinite(self: &mut Self)`

- `fn cross_forward(self: &mut Self, other: &mut Seq)` — [`Seq`](#seq)

- `fn cross_reverse(self: &mut Self, other: &mut Seq)` — [`Seq`](#seq)

- `fn cross_preamble<'a>(self: &'a mut Self, other: &'a mut Seq) -> Option<(&'a mut Vec<Literal>, &'a mut Vec<Literal>)>` — [`Seq`](#seq), [`Literal`](#literal)

- `fn union(self: &mut Self, other: &mut Seq)` — [`Seq`](#seq)

- `fn union_into_empty(self: &mut Self, other: &mut Seq)` — [`Seq`](#seq)

- `fn dedup(self: &mut Self)`

- `fn sort(self: &mut Self)`

- `fn reverse_literals(self: &mut Self)`

- `fn minimize_by_preference(self: &mut Self)`

- `fn keep_first_bytes(self: &mut Self, len: usize)`

- `fn keep_last_bytes(self: &mut Self, len: usize)`

- `fn is_finite(self: &Self) -> bool`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> Option<usize>`

- `fn is_exact(self: &Self) -> bool`

- `fn is_inexact(self: &Self) -> bool`

- `fn max_union_len(self: &Self, other: &Seq) -> Option<usize>` — [`Seq`](#seq)

- `fn max_cross_len(self: &Self, other: &Seq) -> Option<usize>` — [`Seq`](#seq)

- `fn min_literal_len(self: &Self) -> Option<usize>`

- `fn max_literal_len(self: &Self) -> Option<usize>`

- `fn longest_common_prefix(self: &Self) -> Option<&[u8]>`

- `fn longest_common_suffix(self: &Self) -> Option<&[u8]>`

- `fn optimize_for_prefix_by_preference(self: &mut Self)`

- `fn optimize_for_suffix_by_preference(self: &mut Self)`

- `fn optimize_by_preference(self: &mut Self, prefix: bool)`

#### Trait Implementations

##### `impl Clone for Seq`

- `fn clone(self: &Self) -> Seq` — [`Seq`](#seq)

##### `impl Debug for Seq`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Seq`

##### `impl FromIterator for Seq`

- `fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq` — [`Seq`](#seq)

##### `impl PartialEq for Seq`

- `fn eq(self: &Self, other: &Seq) -> bool` — [`Seq`](#seq)

##### `impl StructuralPartialEq for Seq`

### `Literal`

```rust
struct Literal {
    bytes: alloc::vec::Vec<u8>,
    exact: bool,
}
```

A single literal extracted from an [`Hir`](../index.md) expression.

A literal is composed of two things:

* A sequence of bytes. No guarantees with respect to UTF-8 are provided.
In particular, even if the regex a literal is extracted from is UTF-8, the
literal extracted may not be valid UTF-8. (For example, if an [`Extractor`](#extractor)
limit resulted in trimming a literal in a way that splits a codepoint.)
* Whether the literal is "exact" or not. An "exact" literal means that it
has not been trimmed, and may continue to be extended. If a literal is
"exact" after visiting the entire `Hir` expression, then this implies that
the literal leads to a match state. (Although it doesn't necessarily imply
all occurrences of the literal correspond to a match of the regex, since
literal extraction ignores look-around assertions.)

#### Implementations

- `fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal` — [`Literal`](#literal)

- `fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal` — [`Literal`](#literal)

- `fn as_bytes(self: &Self) -> &[u8]`

- `fn into_bytes(self: Self) -> Vec<u8>`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn is_exact(self: &Self) -> bool`

- `fn make_inexact(self: &mut Self)`

- `fn reverse(self: &mut Self)`

- `fn extend(self: &mut Self, lit: &Literal)` — [`Literal`](#literal)

- `fn keep_first_bytes(self: &mut Self, len: usize)`

- `fn keep_last_bytes(self: &mut Self, len: usize)`

- `fn is_poisonous(self: &Self) -> bool`

#### Trait Implementations

##### `impl AsRef for Literal`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Clone for Literal`

- `fn clone(self: &Self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Literal`

##### `impl Ord for Literal`

- `fn cmp(self: &Self, other: &Literal) -> $crate::cmp::Ordering` — [`Literal`](#literal)

##### `impl PartialEq for Literal`

- `fn eq(self: &Self, other: &Literal) -> bool` — [`Literal`](#literal)

##### `impl PartialOrd for Literal`

- `fn partial_cmp(self: &Self, other: &Literal) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Literal`](#literal)

##### `impl StructuralPartialEq for Literal`

## Enums

### `ExtractKind`

```rust
enum ExtractKind {
    Prefix,
    Suffix,
}
```

The kind of literals to extract from an [`Hir`](../index.md) expression.

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

- `fn is_suffix(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for ExtractKind`

- `fn clone(self: &Self) -> ExtractKind` — [`ExtractKind`](#extractkind)

##### `impl Debug for ExtractKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ExtractKind`

- `fn default() -> ExtractKind` — [`ExtractKind`](#extractkind)

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

