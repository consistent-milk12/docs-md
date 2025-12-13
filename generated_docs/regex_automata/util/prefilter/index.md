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

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:142-151`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/prefilter/mod.rs#L142-L151)*

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

  Create a new prefilter from a sequence of needles and a corresponding

  match semantics.

  

  This may return `None` for a variety of reasons, for example, if

  a suitable prefilter could not be constructed. That might occur

  if they are unavailable (e.g., the `perf-literal-substring` and

  `perf-literal-multisubstring` features aren't enabled), or it might

  occur because of heuristics or other artifacts of how the prefilter

  works.

  

  Note that if you have an [`Hir`](../../../regex_syntax/hir/index.md) expression, it may be more convenient

  to use `Prefilter::from_hir_prefix`. It will automatically handle the

  task of extracting prefix literals for you.

  

  # Example

  

  This example shows how match semantics can impact the matching

  algorithm used by the prefilter. For this reason, it is important to

  ensure that the match semantics given here are consistent with the

  match semantics intended for the regular expression that the literals

  were extracted from.

  

  ```rust

  use regex_automata::{

      util::{prefilter::Prefilter, syntax},

      MatchKind, Span,

  };

  

  let hay = "Hello samwise";

  

  // With leftmost-first, we find 'samwise' here because it comes

  // before 'sam' in the sequence we give it..

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["samwise", "sam"])

      .expect("a prefilter");

  assert_eq!(

      Some(Span::from(6..13)),

      pre.find(hay.as_bytes(), Span::from(0..hay.len())),

  );

  // Still with leftmost-first but with the literals reverse, now 'sam'

  // will match instead!

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["sam", "samwise"])

      .expect("a prefilter");

  assert_eq!(

      Some(Span::from(6..9)),

      pre.find(hay.as_bytes(), Span::from(0..hay.len())),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="prefilter-from-choice"></span>`fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter>` — [`Choice`](#choice), [`Prefilter`](#prefilter)

  This turns a prefilter selection into a `Prefilter`. That is, in turns

  the enum given into a trait object.

- <span id="prefilter-from-hir-prefix"></span>`fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter>` — [`MatchKind`](../../index.md#matchkind), [`Prefilter`](#prefilter)

  This attempts to extract prefixes from the given `Hir` expression for

  the given match semantics, and if possible, builds a prefilter for

  them.

  

  # Example

  

  This example shows how to build a prefilter directly from an [`Hir`](../../../regex_syntax/hir/index.md)

  expression, and use to find an occurrence of a prefix from the regex

  pattern.

  

  ```rust

  use regex_automata::{

      util::{prefilter::Prefilter, syntax},

      MatchKind, Span,

  };

  

  let hir = syntax::parse(r"(Bruce|Patti) \w+")?;

  let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &hir)

      .expect("a prefilter");

  let hay = "Hello Patti Scialfa!";

  assert_eq!(

      Some(Span::from(6..12)),

      pre.find(hay.as_bytes(), Span::from(0..hay.len())),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="prefilter-from-hirs-prefix"></span>`fn from_hirs_prefix<H: Borrow<Hir>>(kind: MatchKind, hirs: &[H]) -> Option<Prefilter>` — [`MatchKind`](../../index.md#matchkind), [`Prefilter`](#prefilter)

  This attempts to extract prefixes from the given `Hir` expressions for

  the given match semantics, and if possible, builds a prefilter for

  them.

  

  Note that as of now, prefilters throw away information about which

  pattern each literal comes from. In other words, when a prefilter finds

  a match, there's no way to know which pattern (or patterns) it came

  from. Therefore, in order to confirm a match, you'll have to check all

  of the patterns by running the full regex engine.

  

  # Example

  

  This example shows how to build a prefilter directly from multiple

  `Hir` expressions expression, and use it to find an occurrence of a

  prefix from the regex patterns.

  

  ```rust

  use regex_automata::{

      util::{prefilter::Prefilter, syntax},

      MatchKind, Span,

  };

  

  let hirs = syntax::parse_many(&[

      r"(Bruce|Patti) \w+",

      r"Mrs?\. Doubtfire",

  ])?;

  let pre = Prefilter::from_hirs_prefix(MatchKind::LeftmostFirst, &hirs)

      .expect("a prefilter");

  let hay = "Hello Mrs. Doubtfire";

  assert_eq!(

      Some(Span::from(6..20)),

      pre.find(hay.as_bytes(), Span::from(0..hay.len())),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="prefilter-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../index.md#span)

  Run this prefilter on `haystack[span.start..end]` and return a matching

  span if one exists.

  

  The span returned is guaranteed to have a start position greater than

  or equal to the one given, and an end position less than or equal to

  the one given.

  

  # Example

  

  This example shows how to build a prefilter directly from an [`Hir`](../../../regex_syntax/hir/index.md)

  expression, and use it to find an occurrence of a prefix from the regex

  pattern.

  

  ```rust

  use regex_automata::{

      util::{prefilter::Prefilter, syntax},

      MatchKind, Span,

  };

  

  let hir = syntax::parse(r"Bruce \w+")?;

  let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &hir)

      .expect("a prefilter");

  let hay = "Hello Bruce Springsteen!";

  assert_eq!(

      Some(Span::from(6..12)),

      pre.find(hay.as_bytes(), Span::from(0..hay.len())),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="prefilter-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../index.md#span)

  Returns the span of a prefix of `haystack[span.start..span.end]` if

  the prefilter matches.

  

  The span returned is guaranteed to have a start position equivalent to

  the one given, and an end position less than or equal to the one given.

  

  # Example

  

  This example shows how to build a prefilter directly from an [`Hir`](../../../regex_syntax/hir/index.md)

  expression, and use it to find an occurrence of a prefix from the regex

  pattern that begins at the start of a haystack only.

  

  ```rust

  use regex_automata::{

      util::{prefilter::Prefilter, syntax},

      MatchKind, Span,

  };

  

  let hir = syntax::parse(r"Bruce \w+")?;

  let pre = Prefilter::from_hir_prefix(MatchKind::LeftmostFirst, &hir)

      .expect("a prefilter");

  let hay = "Hello Bruce Springsteen!";

  // Nothing is found here because 'Bruce' does

  // not occur at the beginning of our search.

  assert_eq!(

      None,

      pre.prefix(hay.as_bytes(), Span::from(0..hay.len())),

  );

  // But if we change where we start the search

  // to begin where 'Bruce ' begins, then a

  // match will be found.

  assert_eq!(

      Some(Span::from(6..12)),

      pre.prefix(hay.as_bytes(), Span::from(6..hay.len())),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="prefilter-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory, in bytes, used by the underlying prefilter.

- <span id="prefilter-max-needle-len"></span>`fn max_needle_len(&self) -> usize`

  Return the length of the longest needle

  in this Prefilter

- <span id="prefilter-is-fast"></span>`fn is_fast(&self) -> bool`

  Implementations might return true here if they believe themselves to

  be "fast." The concept of "fast" is deliberately left vague, but in

  practice this usually corresponds to whether it's believed that SIMD

  will be used.

  

  Why do we care about this? Well, some prefilter tricks tend to come

  with their own bits of overhead, and so might only make sense if we

  know that a scan will be *much* faster than the regex engine itself.

  Otherwise, the trick may not be worth doing. Whether something is

  "much" faster than the regex engine generally boils down to whether

  SIMD is used. (But not always. Even a SIMD matcher with a high false

  positive rate can become quite slow.)

  

  Even if this returns true, it is still possible for the prefilter to

  be "slow." Remember, prefilters are just heuristics. We can't really

  *know* a prefilter will be fast without actually trying the prefilter.

  (Which of course we cannot afford to do.)

#### Trait Implementations

##### `impl Any for Prefilter`

- <span id="prefilter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Prefilter`

- <span id="prefilter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Prefilter`

- <span id="prefilter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Prefilter`

- <span id="prefilter-clone"></span>`fn clone(&self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl CloneToUninit for Prefilter`

- <span id="prefilter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Prefilter`

- <span id="prefilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Prefilter`

- <span id="prefilter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Prefilter`

- <span id="prefilter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Prefilter`

- <span id="prefilter-toowned-type-owned"></span>`type Owned = T`

- <span id="prefilter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prefilter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Prefilter`

- <span id="prefilter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prefilter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Prefilter`

- <span id="prefilter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prefilter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:546-554`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/prefilter/mod.rs#L546-L554)*

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

  Select what is believed to be the best prefilter algorithm for the

  match semantics and sequence of needles given.

  

  This selection algorithm uses the needles as given without any

  modification. For example, if `[bar]` is given, then this doesn't

  try to select `memchr` for `b`. Instead, it would select `memmem`

  for `bar`. If callers would want `memchr` selected for `[bar]`, then

  callers should massages the literals themselves. That is, callers are

  responsible for heuristics surrounding which sequence of literals is

  best.

  

  What this selection algorithm does is attempt to use the fastest

  prefilter that works for the literals given. So if `[a, b]`, is given,

  then `memchr2` is selected.

  

  Of course, which prefilter is selected is also subject to what

  is available. For example, if `alloc` isn't enabled, then

  that limits which prefilters can be selected. Similarly, if

  `perf-literal-substring` isn't enabled, then nothing from the `memchr`

  crate can be returned.

#### Trait Implementations

##### `impl Any for Choice`

- <span id="choice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Choice`

- <span id="choice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Choice`

- <span id="choice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Choice`

- <span id="choice-clone"></span>`fn clone(&self) -> Choice` — [`Choice`](#choice)

##### `impl CloneToUninit for Choice`

- <span id="choice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Choice`

- <span id="choice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Choice`

- <span id="choice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Choice`

- <span id="choice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Choice`

- <span id="choice-toowned-type-owned"></span>`type Owned = T`

- <span id="choice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="choice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Choice`

- <span id="choice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="choice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Choice`

- <span id="choice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="choice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `PrefilterI`

```rust
trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static { ... }
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:474-498`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/prefilter/mod.rs#L474-L498)*

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

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:649-682`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/prefilter/mod.rs#L649-L682)*

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

*Defined in [`regex-automata-0.4.13/src/util/prefilter/mod.rs:686-719`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/prefilter/mod.rs#L686-L719)*

Like `prefixes`, but for all suffixes of all matches for the given HIRs.

