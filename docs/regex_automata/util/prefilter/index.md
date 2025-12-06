*[regex_automata](../../index.md) / [util](../index.md) / [prefilter](index.md)*

---

# Module `prefilter`

Defines a prefilter for accelerating regex searches.

A prefilter can be created by building a [`Prefilter`](#prefilter) value.

A prefilter represents one of the most important optimizations available for
accelerating regex searches. The idea of a prefilter is to very quickly find
candidate locations in a haystack where a regex _could_ match. Once a candidate
is found, it is then intended for the regex engine to run at that position to
determine whether the candidate is a match or a false positive.

In the aforementioned description of the prefilter optimization also lay its
demise. Namely, if a prefilter has a high false positive rate and it produces
lots of candidates, then a prefilter can overall make a regex search slower.
It can run more slowly because more time is spent ping-ponging between the
prefilter search and the regex engine attempting to confirm each candidate as
a match. This ping-ponging has overhead that adds up, and is exacerbated by
a high false positive rate.

Nevertheless, the optimization is still generally worth performing in most
cases. Particularly given just how much throughput can be improved. (It is not
uncommon for prefilter optimizations to improve throughput by one or two orders
of magnitude.)

Typically a prefilter is used to find occurrences of literal prefixes from a
regex pattern, but this isn't required. A prefilter can be used to look for
suffixes or even inner literals.

Note that as of now, prefilters throw away information about which pattern
each literal comes from. In other words, when a prefilter finds a match,
there's no way to know which pattern (or patterns) it came from. Therefore,
in order to confirm a match, you'll have to check all of the patterns by
running the full regex engine.

## Structs

### `Prefilter`

```rust
struct Prefilter {
    pre: alloc::sync::Arc<dyn PrefilterI>,
    is_fast: bool,
    max_needle_len: usize,
}
```

A prefilter for accelerating regex searches.

If you already have your literals that you want to search with,
then the vanilla `Prefilter::new` constructor is for you. But
if you have an [`Hir`](#hir) value from the `regex-syntax` crate, then
`Prefilter::from_hir_prefix` might be more convenient. Namely, it uses
the [`regex-syntax::hir::literal`](regex_syntax::hir::literal) module to
extract literal prefixes for you, optimize them and then select and build a
prefilter matcher.

A prefilter must have **zero false negatives**. However, by its very
nature, it may produce false positives. That is, a prefilter will never
skip over a position in the haystack that corresponds to a match of the
original regex pattern, but it *may* produce a match for a position
in the haystack that does *not* correspond to a match of the original
regex pattern. If you use either the `Prefilter::from_hir_prefix` or
`Prefilter::from_hirs_prefix` constructors, then this guarantee is
upheld for you automatically. This guarantee is not preserved if you use
`Prefilter::new` though, since it is up to the caller to provide correct
literal strings with respect to the original regex pattern.

# Cloning

It is an API guarantee that cloning a prefilter is cheap. That is, cloning
it will not duplicate whatever heap memory is used to represent the
underlying matcher.

# Example

This example shows how to attach a `Prefilter` to the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) in order to accelerate
searches.

```rust
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::prefilter::Prefilter,
    Match, MatchKind,
};

let pre = Prefilter::new(MatchKind::LeftmostFirst, &["Bruce "])
    .expect("a prefilter");
let re = PikeVM::builder()
    .configure(PikeVM::config().prefilter(Some(pre)))
    .build(r"Bruce \w+")?;
let mut cache = re.create_cache();
assert_eq!(
    Some(Match::must(0, 6..23)),
    re.find(&mut cache, "Hello Bruce Springsteen!"),
);
Ok::<(), Box<dyn std::error::Error>>(())
```

But note that if you get your prefilter incorrect, it could lead to an
incorrect result!

```rust
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::prefilter::Prefilter,
    Match, MatchKind,
};

// This prefilter is wrong!
let pre = Prefilter::new(MatchKind::LeftmostFirst, &["Patti "])
    .expect("a prefilter");
let re = PikeVM::builder()
    .configure(PikeVM::config().prefilter(Some(pre)))
    .build(r"Bruce \w+")?;
let mut cache = re.create_cache();
// We find no match even though the regex does match.
assert_eq!(
    None,
    re.find(&mut cache, "Hello Bruce Springsteen!"),
);
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter>` — [`MatchKind`](../../index.md), [`Prefilter`](#prefilter)

- `fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter>` — [`Choice`](#choice), [`Prefilter`](#prefilter)

- `fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter>` — [`MatchKind`](../../index.md), [`Prefilter`](#prefilter)

- `fn from_hirs_prefix<H: Borrow<Hir>>(kind: MatchKind, hirs: &[H]) -> Option<Prefilter>` — [`MatchKind`](../../index.md), [`Prefilter`](#prefilter)

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn max_needle_len(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for Prefilter`

- `fn clone(self: &Self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl Debug for Prefilter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

