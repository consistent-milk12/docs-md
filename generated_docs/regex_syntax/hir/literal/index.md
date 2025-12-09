*[regex_syntax](../../index.md) / [hir](../index.md) / [literal](index.md)*

---

# Module `literal`

Provides literal extraction from `Hir` expressions.

An [`Extractor`](#extractor) pulls literals out of [`Hir`](../index.md) expressions and returns a
[`Seq`](#seq) of [`Literal`](#literal)s.

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
an `Extractor` only uses public APIs of the [`Seq`](#seq) and [`Literal`](#literal) types,
so it is possible to implement your own extractor. For example, for n-grams
or "inner" literals (i.e., not prefix or suffix literals). The `Extractor`
is mostly responsible for the case analysis over `Hir` expressions. Much of
the "trickier" parts are how to combine literal sequences, and that is all
implemented on [`Seq`](#seq).

## Contents

- [Structs](#structs)
  - [`Extractor`](#extractor)
  - [`Seq`](#seq)
  - [`Literal`](#literal)
  - [`PreferenceTrie`](#preferencetrie)
  - [`State`](#state)
- [Enums](#enums)
  - [`ExtractKind`](#extractkind)
- [Functions](#functions)
  - [`rank`](#rank)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Extractor`](#extractor) | struct | Extracts prefix or suffix literal sequences from [`Hir`] expressions. |
| [`Seq`](#seq) | struct | A sequence of literals. |
| [`Literal`](#literal) | struct | A single literal extracted from an [`Hir`] expression. |
| [`PreferenceTrie`](#preferencetrie) | struct | A "preference" trie that rejects literals that will never match when executing a leftmost first or "preference" search. |
| [`State`](#state) | struct | A single state in a trie. |
| [`ExtractKind`](#extractkind) | enum | The kind of literals to extract from an [`Hir`] expression. |
| [`rank`](#rank) | fn | Returns the "rank" of the given byte. |

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

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:147-153`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/literal.rs#L147-L153)*

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

- <span id="extractor-new"></span>`fn new() -> Extractor` — [`Extractor`](#extractor)

- <span id="extractor-extract"></span>`fn extract(&self, hir: &Hir) -> Seq` — [`Hir`](../index.md), [`Seq`](#seq)

- <span id="extractor-kind"></span>`fn kind(&mut self, kind: ExtractKind) -> &mut Extractor` — [`ExtractKind`](#extractkind), [`Extractor`](#extractor)

- <span id="extractor-limit-class"></span>`fn limit_class(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- <span id="extractor-limit-repeat"></span>`fn limit_repeat(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- <span id="extractor-limit-literal-len"></span>`fn limit_literal_len(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- <span id="extractor-limit-total"></span>`fn limit_total(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

- <span id="extractor-extract-concat"></span>`fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq` — [`Seq`](#seq)

- <span id="extractor-extract-alternation"></span>`fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq` — [`Seq`](#seq)

- <span id="extractor-extract-repetition"></span>`fn extract_repetition(&self, rep: &hir::Repetition) -> Seq` — [`Repetition`](../index.md), [`Seq`](#seq)

- <span id="extractor-extract-class-unicode"></span>`fn extract_class_unicode(&self, cls: &hir::ClassUnicode) -> Seq` — [`ClassUnicode`](../index.md), [`Seq`](#seq)

- <span id="extractor-extract-class-bytes"></span>`fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> Seq` — [`ClassBytes`](../index.md), [`Seq`](#seq)

- <span id="extractor-class-over-limit-unicode"></span>`fn class_over_limit_unicode(&self, cls: &hir::ClassUnicode) -> bool` — [`ClassUnicode`](../index.md)

- <span id="extractor-class-over-limit-bytes"></span>`fn class_over_limit_bytes(&self, cls: &hir::ClassBytes) -> bool` — [`ClassBytes`](../index.md)

- <span id="extractor-cross"></span>`fn cross(&self, seq1: Seq, seq2: &mut Seq) -> Seq` — [`Seq`](#seq)

- <span id="extractor-union"></span>`fn union(&self, seq1: Seq, seq2: &mut Seq) -> Seq` — [`Seq`](#seq)

- <span id="extractor-enforce-literal-len"></span>`fn enforce_literal_len(&self, seq: &mut Seq)` — [`Seq`](#seq)

#### Trait Implementations

##### `impl Clone for Extractor`

- <span id="extractor-clone"></span>`fn clone(&self) -> Extractor` — [`Extractor`](#extractor)

##### `impl Debug for Extractor`

- <span id="extractor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Extractor`

- <span id="extractor-default"></span>`fn default() -> Extractor` — [`Extractor`](#extractor)

### `Seq`

```rust
struct Seq {
    literals: Option<alloc::vec::Vec<Literal>>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:733-745`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/literal.rs#L733-L745)*

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

- <span id="seq-empty"></span>`fn empty() -> Seq` — [`Seq`](#seq)

- <span id="seq-infinite"></span>`fn infinite() -> Seq` — [`Seq`](#seq)

- <span id="seq-singleton"></span>`fn singleton(lit: Literal) -> Seq` — [`Literal`](#literal), [`Seq`](#seq)

- <span id="seq-new"></span>`fn new<I, B>(it: I) -> Seq` — [`Seq`](#seq)

- <span id="seq-literals"></span>`fn literals(&self) -> Option<&[Literal]>` — [`Literal`](#literal)

- <span id="seq-push"></span>`fn push(&mut self, lit: Literal)` — [`Literal`](#literal)

- <span id="seq-make-inexact"></span>`fn make_inexact(&mut self)`

- <span id="seq-make-infinite"></span>`fn make_infinite(&mut self)`

- <span id="seq-cross-forward"></span>`fn cross_forward(&mut self, other: &mut Seq)` — [`Seq`](#seq)

- <span id="seq-cross-reverse"></span>`fn cross_reverse(&mut self, other: &mut Seq)` — [`Seq`](#seq)

- <span id="seq-cross-preamble"></span>`fn cross_preamble<'a>(self: &'a mut Self, other: &'a mut Seq) -> Option<(&'a mut Vec<Literal>, &'a mut Vec<Literal>)>` — [`Seq`](#seq), [`Literal`](#literal)

- <span id="seq-union"></span>`fn union(&mut self, other: &mut Seq)` — [`Seq`](#seq)

- <span id="seq-union-into-empty"></span>`fn union_into_empty(&mut self, other: &mut Seq)` — [`Seq`](#seq)

- <span id="seq-dedup"></span>`fn dedup(&mut self)`

- <span id="seq-sort"></span>`fn sort(&mut self)`

- <span id="seq-reverse-literals"></span>`fn reverse_literals(&mut self)`

- <span id="seq-minimize-by-preference"></span>`fn minimize_by_preference(&mut self)`

- <span id="seq-keep-first-bytes"></span>`fn keep_first_bytes(&mut self, len: usize)`

- <span id="seq-keep-last-bytes"></span>`fn keep_last_bytes(&mut self, len: usize)`

- <span id="seq-is-finite"></span>`fn is_finite(&self) -> bool`

- <span id="seq-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="seq-len"></span>`fn len(&self) -> Option<usize>`

- <span id="seq-is-exact"></span>`fn is_exact(&self) -> bool`

- <span id="seq-is-inexact"></span>`fn is_inexact(&self) -> bool`

- <span id="seq-max-union-len"></span>`fn max_union_len(&self, other: &Seq) -> Option<usize>` — [`Seq`](#seq)

- <span id="seq-max-cross-len"></span>`fn max_cross_len(&self, other: &Seq) -> Option<usize>` — [`Seq`](#seq)

- <span id="seq-min-literal-len"></span>`fn min_literal_len(&self) -> Option<usize>`

- <span id="seq-max-literal-len"></span>`fn max_literal_len(&self) -> Option<usize>`

- <span id="seq-longest-common-prefix"></span>`fn longest_common_prefix(&self) -> Option<&[u8]>`

- <span id="seq-longest-common-suffix"></span>`fn longest_common_suffix(&self) -> Option<&[u8]>`

- <span id="seq-optimize-for-prefix-by-preference"></span>`fn optimize_for_prefix_by_preference(&mut self)`

- <span id="seq-optimize-for-suffix-by-preference"></span>`fn optimize_for_suffix_by_preference(&mut self)`

- <span id="seq-optimize-by-preference"></span>`fn optimize_by_preference(&mut self, prefix: bool)`

#### Trait Implementations

##### `impl Clone for Seq`

- <span id="seq-clone"></span>`fn clone(&self) -> Seq` — [`Seq`](#seq)

##### `impl Debug for Seq`

- <span id="seq-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Seq`

##### `impl FromIterator for Seq`

- <span id="seq-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq` — [`Seq`](#seq)

##### `impl PartialEq for Seq`

- <span id="seq-eq"></span>`fn eq(&self, other: &Seq) -> bool` — [`Seq`](#seq)

##### `impl StructuralPartialEq for Seq`

### `Literal`

```rust
struct Literal {
    bytes: alloc::vec::Vec<u8>,
    exact: bool,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2049-2052`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/literal.rs#L2049-L2052)*

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

- <span id="literal-exact"></span>`fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal` — [`Literal`](#literal)

- <span id="literal-inexact"></span>`fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal` — [`Literal`](#literal)

- <span id="literal-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

- <span id="literal-into-bytes"></span>`fn into_bytes(self) -> Vec<u8>`

- <span id="literal-len"></span>`fn len(&self) -> usize`

- <span id="literal-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="literal-is-exact"></span>`fn is_exact(&self) -> bool`

- <span id="literal-make-inexact"></span>`fn make_inexact(&mut self)`

- <span id="literal-reverse"></span>`fn reverse(&mut self)`

- <span id="literal-extend"></span>`fn extend(&mut self, lit: &Literal)` — [`Literal`](#literal)

- <span id="literal-keep-first-bytes"></span>`fn keep_first_bytes(&mut self, len: usize)`

- <span id="literal-keep-last-bytes"></span>`fn keep_last_bytes(&mut self, len: usize)`

- <span id="literal-is-poisonous"></span>`fn is_poisonous(&self) -> bool`

#### Trait Implementations

##### `impl AsRef for Literal`

- <span id="literal-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- <span id="literal-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Literal`

##### `impl FromIterator for Seq`

- <span id="seq-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq` — [`Seq`](#seq)

##### `impl Ord for Literal`

- <span id="literal-cmp"></span>`fn cmp(&self, other: &Literal) -> cmp::Ordering` — [`Literal`](#literal)

##### `impl PartialEq for Literal`

- <span id="literal-eq"></span>`fn eq(&self, other: &Literal) -> bool` — [`Literal`](#literal)

##### `impl PartialOrd for Literal`

- <span id="literal-partial-cmp"></span>`fn partial_cmp(&self, other: &Literal) -> option::Option<cmp::Ordering>` — [`Literal`](#literal)

##### `impl StructuralPartialEq for Literal`

### `PreferenceTrie`

```rust
struct PreferenceTrie {
    states: alloc::vec::Vec<State>,
    matches: alloc::vec::Vec<Option<core::num::NonZeroUsize>>,
    next_literal_index: usize,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2203-2216`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/literal.rs#L2203-L2216)*

A "preference" trie that rejects literals that will never match when
executing a leftmost first or "preference" search.

For example, if 'sam' is inserted, then trying to insert 'samwise' will be
rejected because 'samwise' can never match since 'sam' will always take
priority. However, if 'samwise' is inserted first, then inserting 'sam'
after it is accepted. In this case, either 'samwise' or 'sam' can match in
a "preference" search.

Note that we only use this trie as a "set." That is, given a sequence of
literals, we insert each one in order. An `insert` will reject a literal
if a prefix of that literal already exists in the trie. Thus, to rebuild
the "minimal" sequence, we simply only keep literals that were successfully
inserted. (Since we don't need traversal, one wonders whether we can make
some simplifications here, but I haven't given it a ton of thought and I've
never seen this show up on a profile. Because of the heuristic limits
imposed on literal extractions, the size of the inputs here is usually
very small.)

#### Fields

- **`states`**: `alloc::vec::Vec<State>`

  The states in this trie. The index of a state in this vector is its ID.

- **`matches`**: `alloc::vec::Vec<Option<core::num::NonZeroUsize>>`

  This vec indicates which states are match states. It always has
  the same length as `states` and is indexed by the same state ID.
  A state with identifier `sid` is a match state if and only if
  `matches[sid].is_some()`. The option contains the index of the literal
  corresponding to the match. The index is offset by 1 so that it fits in
  a NonZeroUsize.

- **`next_literal_index`**: `usize`

  The index to allocate to the next literal added to this trie. Starts at
  1 and increments by 1 for every literal successfully added to the trie.

#### Implementations

- <span id="preferencetrie-minimize"></span>`fn minimize(literals: &mut Vec<Literal>, keep_exact: bool)` — [`Literal`](#literal)

- <span id="preferencetrie-insert"></span>`fn insert(&mut self, bytes: &[u8]) -> Result<usize, usize>`

- <span id="preferencetrie-root"></span>`fn root(&mut self) -> usize`

- <span id="preferencetrie-create-state"></span>`fn create_state(&mut self) -> usize`

#### Trait Implementations

##### `impl Debug for PreferenceTrie`

- <span id="preferencetrie-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `State`

```rust
struct State {
    trans: alloc::vec::Vec<(u8, usize)>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2220-2225`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/literal.rs#L2220-L2225)*

A single state in a trie. Uses a sparse representation for its transitions.

#### Fields

- **`trans`**: `alloc::vec::Vec<(u8, usize)>`

  Sparse representation of the transitions out of this state. Transitions
  are sorted by byte. There is at most one such transition for any
  particular byte.

#### Trait Implementations

##### `impl Debug for State`

- <span id="state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> State` — [`State`](#state)

## Enums

### `ExtractKind`

```rust
enum ExtractKind {
    Prefix,
    Suffix,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:640-649`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/literal.rs#L640-L649)*

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

- <span id="extractkind-is-prefix"></span>`fn is_prefix(&self) -> bool`

- <span id="extractkind-is-suffix"></span>`fn is_suffix(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ExtractKind`

- <span id="extractkind-clone"></span>`fn clone(&self) -> ExtractKind` — [`ExtractKind`](#extractkind)

##### `impl Debug for ExtractKind`

- <span id="extractkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ExtractKind`

- <span id="extractkind-default"></span>`fn default() -> ExtractKind` — [`ExtractKind`](#extractkind)

## Functions

### `rank`

```rust
fn rank(byte: u8) -> u8
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2319-2321`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/literal.rs#L2319-L2321)*

Returns the "rank" of the given byte.

The minimum rank value is `0` and the maximum rank value is `255`.

The rank of a byte is derived from a heuristic background distribution of
relative frequencies of bytes. The heuristic says that lower the rank of a
byte, the less likely that byte is to appear in any arbitrary haystack.

