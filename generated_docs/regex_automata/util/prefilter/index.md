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

## Contents

- [Modules](#modules)
  - [`aho_corasick`](#aho-corasick)
  - [`byteset`](#byteset)
  - [`memchr`](#memchr)
  - [`memmem`](#memmem)
  - [`teddy`](#teddy)
- [Structs](#structs)
  - [`Prefilter`](#prefilter)
- [Enums](#enums)
  - [`Choice`](#choice)
- [Traits](#traits)
  - [`PrefilterI`](#prefilteri)
- [Functions](#functions)
  - [`prefixes`](#prefixes)
  - [`suffixes`](#suffixes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`aho_corasick`](#aho-corasick) | mod |  |
| [`byteset`](#byteset) | mod |  |
| [`memchr`](#memchr) | mod |  |
| [`memmem`](#memmem) | mod |  |
| [`teddy`](#teddy) | mod |  |
| [`Prefilter`](#prefilter) | struct | A prefilter for accelerating regex searches. |
| [`Choice`](#choice) | enum | A type that encapsulates the selection of a prefilter algorithm from a sequence of needles. |
| [`PrefilterI`](#prefilteri) | trait | A trait for abstracting over prefilters. |
| [`prefixes`](#prefixes) | fn | Extracts all of the prefix literals from the given HIR expressions into a single `Seq`. |
| [`suffixes`](#suffixes) | fn | Like `prefixes`, but for all suffixes of all matches for the given HIRs. |

## Modules

- [`aho_corasick`](aho_corasick/index.md)
- [`byteset`](byteset/index.md)
- [`memchr`](memchr/index.md)
- [`memmem`](memmem/index.md)
- [`teddy`](teddy/index.md)

## Structs

### `Prefilter`

```rust
struct Prefilter {
    pre: alloc::sync::Arc<dyn PrefilterI>,
    is_fast: bool,
    max_needle_len: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:142-151`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/prefilter/mod.rs#L142-L151)*

A prefilter for accelerating regex searches.

If you already have your literals that you want to search with,
then the vanilla `Prefilter::new` constructor is for you. But
if you have an [`Hir`](../../../regex_syntax/hir/index.md) value from the `regex-syntax` crate, then
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

- <span id="prefilter-new"></span>`fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter>` — [`MatchKind`](../../index.md#matchkind), [`Prefilter`](#prefilter)

- <span id="prefilter-from-choice"></span>`fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter>` — [`Choice`](#choice), [`Prefilter`](#prefilter)

- <span id="prefilter-from-hir-prefix"></span>`fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter>` — [`MatchKind`](../../index.md#matchkind), [`Prefilter`](#prefilter)

- <span id="prefilter-from-hirs-prefix"></span>`fn from_hirs_prefix<H: Borrow<Hir>>(kind: MatchKind, hirs: &[H]) -> Option<Prefilter>` — [`MatchKind`](../../index.md#matchkind), [`Prefilter`](#prefilter)

- <span id="prefilter-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../index.md#span)

- <span id="prefilter-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../index.md#span)

- <span id="prefilter-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="prefilter-max-needle-len"></span>`fn max_needle_len(&self) -> usize`

- <span id="prefilter-is-fast"></span>`fn is_fast(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Prefilter`

- <span id="prefilter-clone"></span>`fn clone(&self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl Debug for Prefilter`

- <span id="prefilter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Choice`

```rust
enum Choice {
    Memchr(crate::util::prefilter::memchr::Memchr),
    Memchr2(crate::util::prefilter::memchr::Memchr2),
    Memchr3(crate::util::prefilter::memchr::Memchr3),
    Memmem(crate::util::prefilter::memmem::Memmem),
    Teddy(crate::util::prefilter::teddy::Teddy),
    ByteSet(crate::util::prefilter::byteset::ByteSet),
    AhoCorasick(crate::util::prefilter::aho_corasick::AhoCorasick),
}
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:546-554`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/prefilter/mod.rs#L546-L554)*

A type that encapsulates the selection of a prefilter algorithm from a
sequence of needles.

The existence of this type is a little tricky, because we don't (currently)
use it for performing a search. Instead, we really only consume it by
converting the underlying prefilter into a trait object, whether that be
`dyn PrefilterI` or `dyn Strategy` (for the meta regex engine). In order
to avoid re-copying the prefilter selection logic, we isolate it here, and
then force anything downstream that wants to convert it to a trait object
to do trivial case analysis on it.

One wonders whether we *should* use an enum instead of a trait object.
At time of writing, I chose trait objects based on instinct because 1) I
knew I wasn't going to inline anything and 2) there would potentially be
many different choices. However, as of time of writing, I haven't actually
compared the trait object approach to the enum approach. That probably
should be litigated, but I ran out of steam.

Note that if the `alloc` feature is disabled, then values of this type
are (and should) never be constructed. Also, in practice, for any of the
prefilters to be selected, you'll need at least one of the `perf-literal-*`
features enabled.

#### Implementations

- <span id="choice-new"></span>`fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Choice>` — [`MatchKind`](../../index.md#matchkind), [`Choice`](#choice)

#### Trait Implementations

##### `impl Clone for Choice`

- <span id="choice-clone"></span>`fn clone(&self) -> Choice` — [`Choice`](#choice)

##### `impl Debug for Choice`

- <span id="choice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `PrefilterI`

```rust
trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static { ... }
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:474-498`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/prefilter/mod.rs#L474-L498)*

A trait for abstracting over prefilters. Basically, a prefilter is
something that do an unanchored *and* an anchored search in a haystack
within a given span.

This exists pretty much only so that we can use prefilters as a trait
object (which is what `Prefilter` is). If we ever move off of trait objects
and to an enum, then it's likely this trait could be removed.

#### Required Methods

- `fn find(&self, haystack: &[u8], span: Span) -> Option<Span>`

  Run this prefilter on `haystack[span.start..end]` and return a matching

- `fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>`

  Returns the span of a prefix of `haystack[span.start..span.end]` if

- `fn memory_usage(&self) -> usize`

  Returns the heap memory, in bytes, used by the underlying prefilter.

- `fn is_fast(&self) -> bool`

  Implementations might return true here if they believe themselves to

#### Implementors

- [`AhoCorasick`](aho_corasick/index.md#ahocorasick)
- [`ByteSet`](byteset/index.md#byteset)
- [`Memchr2`](memchr/index.md#memchr2)
- [`Memchr3`](memchr/index.md#memchr3)
- [`Memchr`](memchr/index.md#memchr)
- [`Memmem`](memmem/index.md#memmem)
- [`Teddy`](teddy/index.md#teddy)
- `alloc::sync::Arc<P>`

## Functions

### `prefixes`

```rust
fn prefixes<H>(kind: crate::util::search::MatchKind, hirs: &[H]) -> literal::Seq
where
    H: core::borrow::Borrow<regex_syntax::hir::Hir>
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:649-682`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/prefilter/mod.rs#L649-L682)*

Extracts all of the prefix literals from the given HIR expressions into a
single `Seq`. The literals in the sequence are ordered with respect to the
order of the given HIR expressions and consistent with the match semantics
given.

The sequence returned is "optimized." That is, they may be shrunk or even
truncated according to heuristics with the intent of making them more
useful as a prefilter. (Which translates to both using faster algorithms
and minimizing the false positive rate.)

Note that this erases any connection between the literals and which pattern
(or patterns) they came from.

The match kind given must correspond to the match semantics of the regex
that is represented by the HIRs given. The match semantics may change the
literal sequence returned.

### `suffixes`

```rust
fn suffixes<H>(kind: crate::util::search::MatchKind, hirs: &[H]) -> literal::Seq
where
    H: core::borrow::Borrow<regex_syntax::hir::Hir>
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:686-719`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/prefilter/mod.rs#L686-L719)*

Like `prefixes`, but for all suffixes of all matches for the given HIRs.

