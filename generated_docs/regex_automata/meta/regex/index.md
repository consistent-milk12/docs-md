*[regex_automata](../../index.md) / [meta](../index.md) / [regex](index.md)*

---

# Module `regex`

## Contents

- [Structs](#structs)
  - [`Regex`](#regex)
  - [`RegexI`](#regexi)
  - [`RegexInfo`](#regexinfo)
  - [`RegexInfoI`](#regexinfoi)
  - [`FindMatches`](#findmatches)
  - [`CapturesMatches`](#capturesmatches)
  - [`Split`](#split)
  - [`SplitN`](#splitn)
  - [`Cache`](#cache)
  - [`Config`](#config)
  - [`Builder`](#builder)
- [Type Aliases](#type-aliases)
  - [`CachePool`](#cachepool)
  - [`CachePoolGuard`](#cachepoolguard)
  - [`CachePoolFn`](#cachepoolfn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Regex`](#regex) | struct | A regex matcher that works by composing several other regex matchers |
| [`RegexI`](#regexi) | struct | The internal implementation of `Regex`, split out so that it can be wrapped |
| [`RegexInfo`](#regexinfo) | struct |  |
| [`RegexInfoI`](#regexinfoi) | struct |  |
| [`FindMatches`](#findmatches) | struct | An iterator over all non-overlapping matches. |
| [`CapturesMatches`](#capturesmatches) | struct | An iterator over all non-overlapping leftmost matches with their capturing |
| [`Split`](#split) | struct | Yields all substrings delimited by a regular expression match. |
| [`SplitN`](#splitn) | struct | Yields at most `N` spans delimited by a regular expression match. |
| [`Cache`](#cache) | struct | Represents mutable scratch space used by regex engines during a search. |
| [`Config`](#config) | struct | An object describing the configuration of a `Regex`. |
| [`Builder`](#builder) | struct | A builder for configuring and constructing a `Regex`. |
| [`CachePool`](#cachepool) | type | A type alias for our pool of meta::Cache that fixes the type parameters to |
| [`CachePoolGuard`](#cachepoolguard) | type | Same as above, but for the guard returned by a pool. |
| [`CachePoolFn`](#cachepoolfn) | type | The type of the closure we use to create new caches. |

## Structs

### `Regex`

```rust
struct Regex {
    imp: alloc::sync::Arc<RegexI>,
    pool: crate::util::pool::Pool<Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
}
```

A regex matcher that works by composing several other regex matchers
automatically.

In effect, a meta regex papers over a lot of the quirks or performance
problems in each of the regex engines in this crate. Its goal is to provide
an infallible and simple API that "just does the right thing" in the common
case.

A meta regex is the implementation of a `Regex` in the `regex` crate.
Indeed, the `regex` crate API is essentially just a light wrapper over
this type. This includes the `regex` crate's `RegexSet` API!

# Composition

This is called a "meta" matcher precisely because it uses other regex
matchers to provide a convenient high level regex API. Here are some
examples of how other regex matchers are composed:

* When calling `Regex::captures`, instead of immediately
running a slower but more capable regex engine like the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM), the meta regex engine
will usually first look for the bounds of a match with a higher throughput
regex engine like a [lazy DFA](crate::hybrid). Only when a match is found
is a slower engine like `PikeVM` used to find the matching span for each
capture group.
* While higher throughout engines like the lazy DFA cannot handle
Unicode word boundaries in general, they can still be used on pure ASCII
haystacks by pretending that Unicode word boundaries are just plain ASCII
word boundaries. However, if a haystack is not ASCII, the meta regex engine
will automatically switch to a (possibly slower) regex engine that supports
Unicode word boundaries in general.
* In some cases where a regex pattern is just a simple literal or a small
set of literals, an actual regex engine won't be used at all. Instead,
substring or multi-substring search algorithms will be employed.

There are many other forms of composition happening too, but the above
should give a general idea. In particular, it may perhaps be surprising
that *multiple* regex engines might get executed for a single search. That
is, the decision of what regex engine to use is not _just_ based on the
pattern, but also based on the dynamic execution of the search itself.

The primary reason for this composition is performance. The fundamental
tension is that the faster engines tend to be less capable, and the more
capable engines tend to be slower.

Note that the forms of composition that are allowed are determined by
compile time crate features and configuration. For example, if the `hybrid`
feature isn't enabled, or if `Config::hybrid` has been disabled, then the
meta regex engine will never use a lazy DFA.

# Synchronization and cloning

Most of the regex engines in this crate require some kind of mutable
"scratch" space to read and write from while performing a search. Since
a meta regex composes these regex engines, a meta regex also requires
mutable scratch space. This scratch space is called a [`Cache`](../index.md).

Most regex engines _also_ usually have a read-only component, typically
a [Thompson `NFA`](crate::nfa::thompson::NFA).

In order to make the `Regex` API convenient, most of the routines hide
the fact that a `Cache` is needed at all. To achieve this, a [memory
pool](crate::util::pool::Pool) is used internally to retrieve `Cache`
values in a thread safe way that also permits reuse. This in turn implies
that every such search call requires some form of synchronization. Usually
this synchronization is fast enough to not notice, but in some cases, it
can be a bottleneck. This typically occurs when all of the following are
true:

* The same `Regex` is shared across multiple threads simultaneously,
usually via a [`util::lazy::Lazy`](crate::util::lazy::Lazy) or something
similar from the `once_cell` or `lazy_static` crates.
* The primary unit of work in each thread is a regex search.
* Searches are run on very short haystacks.

This particular case can lead to high contention on the pool used by a
`Regex` internally, which can in turn increase latency to a noticeable
effect. This cost can be mitigated in one of the following ways:

* Use a distinct copy of a `Regex` in each thread, usually by cloning it.
Cloning a `Regex` _does not_ do a deep copy of its read-only component.
But it does lead to each `Regex` having its own memory pool, which in
turn eliminates the problem of contention. In general, this technique should
not result in any additional memory usage when compared to sharing the same
`Regex` across multiple threads simultaneously.
* Use lower level APIs, like `Regex::search_with`, which permit passing
a `Cache` explicitly. In this case, it is up to you to determine how best
to provide a `Cache`. For example, you might put a `Cache` in thread-local
storage if your use case allows for it.

Overall, this is an issue that happens rarely in practice, but it can
happen.

# Warning: spin-locks may be used in alloc-only mode

When this crate is built without the `std` feature and the high level APIs
on a `Regex` are used, then a spin-lock will be used to synchronize access
to an internal pool of `Cache` values. This may be undesirable because
a spin-lock is [effectively impossible to implement correctly in user
space][spinlocks-are-bad]. That is, more concretely, the spin-lock could
result in a deadlock.

If one wants to avoid the use of spin-locks when the `std` feature is
disabled, then you must use APIs that accept a `Cache` value explicitly.
For example, `Regex::search_with`.

# Example

```rust
use regex_automata::meta::Regex;

let re = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$")?;
assert!(re.is_match("2010-03-14"));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: anchored search

This example shows how to use `Input::anchored` to run an anchored
search, even when the regex pattern itself isn't anchored. An anchored
search guarantees that if a match is found, then the start offset of the
match corresponds to the offset at which the search was started.

```rust
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"\bfoo\b")?;
let input = Input::new("xx foo xx").range(3..).anchored(Anchored::Yes);
// The offsets are in terms of the original haystack.
assert_eq!(Some(Match::must(0, 3..6)), re.find(input));

// Notice that no match occurs here, because \b still takes the
// surrounding context into account, even if it means looking back
// before the start of your search.
let hay = "xxfoo xx";
let input = Input::new(hay).range(2..).anchored(Anchored::Yes);
assert_eq!(None, re.find(input));
// Indeed, you cannot achieve the above by simply slicing the
// haystack itself, since the regex engine can't see the
// surrounding context. This is why 'Input' permits setting
// the bounds of a search!
let input = Input::new(&hay[2..]).anchored(Anchored::Yes);
// WRONG!
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: earliest search

This example shows how to use `Input::earliest` to run a search that
might stop before finding the typical leftmost match.

```rust
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"[a-z]{3}|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 1..2)), re.find(input));

// Note that "earliest" isn't really a match semantic unto itself.
// Instead, it is merely an instruction to whatever regex engine
// gets used internally to quit as soon as it can. For example,
// this regex uses a different search technique, and winds up
// producing a different (but valid) match!
let re = Regex::new(r"abc|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change
the line terminator to the NUL byte:

```rust
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`imp`**: `alloc::sync::Arc<RegexI>`

  The actual regex implementation.

- **`pool`**: `crate::util::pool::Pool<Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>`

  A thread safe pool of caches.
  
  For the higher level search APIs, a `Cache` is automatically plucked
  from this pool before running a search. The lower level `with` methods
  permit the caller to provide their own cache, thereby bypassing
  accesses to this pool.
  
  Note that we put this outside the `Arc` so that cloning a `Regex`
  results in creating a fresh `CachePool`. This in turn permits callers
  to clone regexes into separate threads where each such regex gets
  the pool's "thread owner" optimization. Otherwise, if one shares the
  `Regex` directly, then the pool will go through a slower mutex path for
  all threads except for the "owner."

#### Implementations

- <span id="regex-search-with"></span>`fn search_with(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- <span id="regex-search-half-with"></span>`fn search_half_with(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- <span id="regex-search-captures-with"></span>`fn search_captures_with(&self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures)` — [`Cache`](../index.md), [`Input`](../../index.md), [`Captures`](../../util/captures/index.md)

- <span id="regex-search-slots-with"></span>`fn search_slots_with(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../index.md)

- <span id="regex-which-overlapping-matches-with"></span>`fn which_overlapping_matches_with(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

#### Trait Implementations

##### `impl Clone for Regex`

- <span id="regex-clone"></span>`fn clone(&self) -> Regex` — [`Regex`](../index.md)

##### `impl Debug for Regex`

- <span id="regex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RegexI`

```rust
struct RegexI {
    strat: alloc::sync::Arc<dyn Strategy>,
    info: RegexInfo,
}
```

The internal implementation of `Regex`, split out so that it can be wrapped
in an `Arc`.

#### Fields

- **`strat`**: `alloc::sync::Arc<dyn Strategy>`

  The core matching engine.
  
  Why is this reference counted when RegexI is already wrapped in an Arc?
  Well, we need to capture this in a closure to our `Pool` below in order
  to create new `Cache` values when needed. So since it needs to be in
  two places, we make it reference counted.
  
  We make `RegexI` itself reference counted too so that `Regex` itself
  stays extremely small and very cheap to clone.

- **`info`**: `RegexInfo`

  Metadata about the regexes driving the strategy. The metadata is also
  usually stored inside the strategy too, but we put it here as well
  so that we can get quick access to it (without virtual calls) before
  executing the regex engine. For example, we use this metadata to
  detect a subset of cases where we know a match is impossible, and can
  thus avoid calling into the strategy at all.
  
  Since `RegexInfo` is stored in multiple places, it is also reference
  counted.

#### Trait Implementations

##### `impl Debug for RegexI`

- <span id="regexi-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RegexInfo`

```rust
struct RegexInfo(alloc::sync::Arc<RegexInfoI>);
```

#### Implementations

- <span id="regexinfo-new"></span>`fn new(config: Config, hirs: &[&Hir]) -> RegexInfo` — [`Config`](../index.md), [`RegexInfo`](#regexinfo)

- <span id="regexinfo-config"></span>`fn config(&self) -> &Config` — [`Config`](../index.md)

- <span id="regexinfo-props"></span>`fn props(&self) -> &[hir::Properties]`

- <span id="regexinfo-props-union"></span>`fn props_union(&self) -> &hir::Properties`

- <span id="regexinfo-pattern-len"></span>`fn pattern_len(&self) -> usize`

- <span id="regexinfo-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="regexinfo-is-anchored-start"></span>`fn is_anchored_start(&self, input: &Input<'_>) -> bool` — [`Input`](../../index.md)

- <span id="regexinfo-is-always-anchored-start"></span>`fn is_always_anchored_start(&self) -> bool`

- <span id="regexinfo-is-always-anchored-end"></span>`fn is_always_anchored_end(&self) -> bool`

- <span id="regexinfo-captures-disabled"></span>`fn captures_disabled(&self) -> bool`

- <span id="regexinfo-is-impossible"></span>`fn is_impossible(&self, input: &Input<'_>) -> bool` — [`Input`](../../index.md)

#### Trait Implementations

##### `impl Clone for RegexInfo`

- <span id="regexinfo-clone"></span>`fn clone(&self) -> RegexInfo` — [`RegexInfo`](#regexinfo)

##### `impl Debug for RegexInfo`

- <span id="regexinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RegexInfoI`

```rust
struct RegexInfoI {
    config: Config,
    props: alloc::vec::Vec<hir::Properties>,
    props_union: hir::Properties,
}
```

#### Trait Implementations

##### `impl Clone for RegexInfoI`

- <span id="regexinfoi-clone"></span>`fn clone(&self) -> RegexInfoI` — [`RegexInfoI`](#regexinfoi)

##### `impl Debug for RegexInfoI`

- <span id="regexinfoi-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FindMatches<'r, 'h>`

```rust
struct FindMatches<'r, 'h> {
    re: &'r Regex,
    cache: crate::util::pool::PoolGuard<'r, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
    it: iter::Searcher<'h>,
}
```

An iterator over all non-overlapping matches.

The iterator yields a [`Match`](../../index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::find_iter` method.

#### Implementations

- <span id="findmatches-regex"></span>`fn regex(&self) -> &'r Regex` — [`Regex`](../index.md)

- <span id="findmatches-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md)

#### Trait Implementations

##### `impl<'r, 'h> Debug for FindMatches<'r, 'h>`

- <span id="findmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'r, 'h> FusedIterator for FindMatches<'r, 'h>`

##### `impl<I> IntoIterator for FindMatches<'r, 'h>`

- <span id="findmatches-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findmatches-intoiter"></span>`type IntoIter = I`

- <span id="findmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'r, 'h> Iterator for FindMatches<'r, 'h>`

- <span id="findmatches-item"></span>`type Item = Match`

- <span id="findmatches-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../index.md)

- <span id="findmatches-count"></span>`fn count(self) -> usize`

### `CapturesMatches<'r, 'h>`

```rust
struct CapturesMatches<'r, 'h> {
    re: &'r Regex,
    cache: crate::util::pool::PoolGuard<'r, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

An iterator over all non-overlapping leftmost matches with their capturing
groups.

The iterator yields a [`Captures`](../../util/captures/index.md) value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::captures_iter` method.

#### Implementations

- <span id="capturesmatches-regex"></span>`fn regex(&self) -> &'r Regex` — [`Regex`](../index.md)

- <span id="capturesmatches-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md)

#### Trait Implementations

##### `impl<'r, 'h> Debug for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'r, 'h> FusedIterator for CapturesMatches<'r, 'h>`

##### `impl<I> IntoIterator for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturesmatches-intoiter"></span>`type IntoIter = I`

- <span id="capturesmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'r, 'h> Iterator for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-item"></span>`type Item = Captures`

- <span id="capturesmatches-next"></span>`fn next(&mut self) -> Option<Captures>` — [`Captures`](../../util/captures/index.md)

- <span id="capturesmatches-count"></span>`fn count(self) -> usize`

### `Split<'r, 'h>`

```rust
struct Split<'r, 'h> {
    finder: FindMatches<'r, 'h>,
    last: usize,
}
```

Yields all substrings delimited by a regular expression match.

The spans correspond to the offsets between matches.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::split` method.

#### Implementations

- <span id="split-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md)

#### Trait Implementations

##### `impl<'r, 'h> Debug for Split<'r, 'h>`

- <span id="split-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'r, 'h> FusedIterator for Split<'r, 'h>`

##### `impl<I> IntoIterator for Split<'r, 'h>`

- <span id="split-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-intoiter"></span>`type IntoIter = I`

- <span id="split-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'r, 'h> Iterator for Split<'r, 'h>`

- <span id="split-item"></span>`type Item = Span`

- <span id="split-next"></span>`fn next(&mut self) -> Option<Span>` — [`Span`](../../index.md)

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    splits: Split<'r, 'h>,
    limit: usize,
}
```

Yields at most `N` spans delimited by a regular expression match.

The spans correspond to the offsets between matches. The last span will be
whatever remains after splitting.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::splitn` method.

#### Implementations

- <span id="splitn-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md)

#### Trait Implementations

##### `impl<'r, 'h> Debug for SplitN<'r, 'h>`

- <span id="splitn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'r, 'h> FusedIterator for SplitN<'r, 'h>`

##### `impl<I> IntoIterator for SplitN<'r, 'h>`

- <span id="splitn-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splitn-intoiter"></span>`type IntoIter = I`

- <span id="splitn-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'r, 'h> Iterator for SplitN<'r, 'h>`

- <span id="splitn-item"></span>`type Item = Span`

- <span id="splitn-next"></span>`fn next(&mut self) -> Option<Span>` — [`Span`](../../index.md)

- <span id="splitn-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Cache`

```rust
struct Cache {
    capmatches: crate::util::captures::Captures,
    pikevm: wrappers::PikeVMCache,
    backtrack: wrappers::BoundedBacktrackerCache,
    onepass: wrappers::OnePassCache,
    hybrid: wrappers::HybridCache,
    revhybrid: wrappers::ReverseHybridCache,
}
```

Represents mutable scratch space used by regex engines during a search.

Most of the regex engines in this crate require some kind of
mutable state in order to execute a search. This mutable state is
explicitly separated from the core regex object (such as a
[`thompson::NFA`](crate::nfa::thompson::NFA)) so that the read-only regex
object can be shared across multiple threads simultaneously without any
synchronization. Conversely, a `Cache` must either be duplicated if using
the same `Regex` from multiple threads, or else there must be some kind of
synchronization that guarantees exclusive access while it's in use by one
thread.

A `Regex` attempts to do this synchronization for you by using a thread
pool internally. Its size scales roughly with the number of simultaneous
regex searches.

For cases where one does not want to rely on a `Regex`'s internal thread
pool, lower level routines such as `Regex::search_with` are provided
that permit callers to pass a `Cache` into the search routine explicitly.

General advice is that the thread pool is often more than good enough.
However, it may be possible to observe the effects of its latency,
especially when searching many small haystacks from many threads
simultaneously.

Caches can be created from their corresponding `Regex` via
`Regex::create_cache`. A cache can only be used with either the `Regex`
that created it, or the `Regex` that was most recently used to reset it
with `Cache::reset`. Using a cache with any other `Regex` may result in
panics or incorrect results.

# Example

```rust
use regex_automata::{meta::Regex, Input, Match};

let re = Regex::new(r"(?-u)m\w+\s+m\w+")?;
let mut cache = re.create_cache();
let input = Input::new("crazy janey and her mission man");
assert_eq!(
    Some(Match::must(0, 20..31)),
    re.search_with(&mut cache, &input),
);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="cache-new"></span>`fn new(re: &Regex) -> Cache` — [`Regex`](../index.md), [`Cache`](../index.md)

- <span id="cache-reset"></span>`fn reset(&mut self, re: &Regex)` — [`Regex`](../index.md)

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](../index.md)

##### `impl Debug for Cache`

- <span id="cache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    utf8_empty: Option<bool>,
    autopre: Option<bool>,
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
    which_captures: Option<crate::nfa::thompson::WhichCaptures>,
    nfa_size_limit: Option<Option<usize>>,
    onepass_size_limit: Option<Option<usize>>,
    hybrid_cache_capacity: Option<usize>,
    hybrid: Option<bool>,
    dfa: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    dfa_state_limit: Option<Option<usize>>,
    onepass: Option<bool>,
    backtrack: Option<bool>,
    byte_classes: Option<bool>,
    line_terminator: Option<u8>,
}
```

An object describing the configuration of a `Regex`.

This configuration only includes options for the
non-syntax behavior of a `Regex`, and can be applied via the
`Builder::configure` method. For configuring the syntax options, see
[`util::syntax::Config`](crate::util::syntax::Config).

# Example: lower the NFA size limit

In some cases, the default size limit might be too big. The size limit can
be lowered, which will prevent large regex patterns from compiling.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::meta::Regex;

let result = Regex::builder()
    .configure(Regex::config().nfa_size_limit(Some(20 * (1<<10))))
    // Not even 20KB is enough to build a single large Unicode class!
    .build(r"\pL");
assert!(result.is_err());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](../index.md)

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md), [`Config`](../index.md)

- <span id="config-utf8-empty"></span>`fn utf8_empty(self, yes: bool) -> Config` — [`Config`](../index.md)

- <span id="config-auto-prefilter"></span>`fn auto_prefilter(self, yes: bool) -> Config` — [`Config`](../index.md)

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../util/prefilter/index.md), [`Config`](../index.md)

- <span id="config-which-captures"></span>`fn which_captures(self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](../../nfa/thompson/index.md), [`Config`](../index.md)

- <span id="config-nfa-size-limit"></span>`fn nfa_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](../index.md)

- <span id="config-onepass-size-limit"></span>`fn onepass_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](../index.md)

- <span id="config-hybrid-cache-capacity"></span>`fn hybrid_cache_capacity(self, limit: usize) -> Config` — [`Config`](../index.md)

- <span id="config-dfa-size-limit"></span>`fn dfa_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](../index.md)

- <span id="config-dfa-state-limit"></span>`fn dfa_state_limit(self, limit: Option<usize>) -> Config` — [`Config`](../index.md)

- <span id="config-byte-classes"></span>`fn byte_classes(self, yes: bool) -> Config` — [`Config`](../index.md)

- <span id="config-line-terminator"></span>`fn line_terminator(self, byte: u8) -> Config` — [`Config`](../index.md)

- <span id="config-hybrid"></span>`fn hybrid(self, yes: bool) -> Config` — [`Config`](../index.md)

- <span id="config-dfa"></span>`fn dfa(self, yes: bool) -> Config` — [`Config`](../index.md)

- <span id="config-onepass"></span>`fn onepass(self, yes: bool) -> Config` — [`Config`](../index.md)

- <span id="config-backtrack"></span>`fn backtrack(self, yes: bool) -> Config` — [`Config`](../index.md)

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../index.md)

- <span id="config-get-utf8-empty"></span>`fn get_utf8_empty(&self) -> bool`

- <span id="config-get-auto-prefilter"></span>`fn get_auto_prefilter(&self) -> bool`

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md)

- <span id="config-get-which-captures"></span>`fn get_which_captures(&self) -> WhichCaptures` — [`WhichCaptures`](../../nfa/thompson/index.md)

- <span id="config-get-nfa-size-limit"></span>`fn get_nfa_size_limit(&self) -> Option<usize>`

- <span id="config-get-onepass-size-limit"></span>`fn get_onepass_size_limit(&self) -> Option<usize>`

- <span id="config-get-hybrid-cache-capacity"></span>`fn get_hybrid_cache_capacity(&self) -> usize`

- <span id="config-get-dfa-size-limit"></span>`fn get_dfa_size_limit(&self) -> Option<usize>`

- <span id="config-get-dfa-state-limit"></span>`fn get_dfa_state_limit(&self) -> Option<usize>`

- <span id="config-get-byte-classes"></span>`fn get_byte_classes(&self) -> bool`

- <span id="config-get-line-terminator"></span>`fn get_line_terminator(&self) -> u8`

- <span id="config-get-hybrid"></span>`fn get_hybrid(&self) -> bool`

- <span id="config-get-dfa"></span>`fn get_dfa(&self) -> bool`

- <span id="config-get-onepass"></span>`fn get_onepass(&self) -> bool`

- <span id="config-get-backtrack"></span>`fn get_backtrack(&self) -> bool`

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](../index.md)

#### Trait Implementations

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](../index.md)

##### `impl Debug for Config`

- <span id="config-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](../index.md)

### `Builder`

```rust
struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
```

A builder for configuring and constructing a `Regex`.

The builder permits configuring two different aspects of a `Regex`:

* `Builder::configure` will set high-level configuration options as
described by a [`Config`](../index.md).
* `Builder::syntax` will set the syntax level configuration options
as described by a [`util::syntax::Config`](crate::util::syntax::Config).
This only applies when building a `Regex` from pattern strings.

Once configured, the builder can then be used to construct a `Regex` from
one of 4 different inputs:

* `Builder::build` creates a regex from a single pattern string.
* `Builder::build_many` creates a regex from many pattern strings.
* `Builder::build_from_hir` creates a regex from a
[`regex-syntax::Hir`](Hir) expression.
* `Builder::build_many_from_hir` creates a regex from many
[`regex-syntax::Hir`](Hir) expressions.

The latter two methods in particular provide a way to construct a fully
feature regular expression matcher directly from an `Hir` expression
without having to first convert it to a string. (This is in contrast to the
top-level `regex` crate which intentionally provides no such API in order
to avoid making `regex-syntax` a public dependency.)

As a convenience, this builder may be created via `Regex::builder`, which
may help avoid an extra import.

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change the
line terminator to the NUL byte:

```rust
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: disable UTF-8 requirement

By default, regex patterns are required to match UTF-8. This includes
regex patterns that can produce matches of length zero. In the case of an
empty match, by default, matches will not appear between the code units of
a UTF-8 encoded codepoint.

However, it can be useful to disable this requirement, particularly if
you're searching things like `&[u8]` that are not known to be valid UTF-8.

```rust
use regex_automata::{meta::Regex, util::syntax, Match};

let mut builder = Regex::builder();
// Disables the requirement that non-empty matches match UTF-8.
builder.syntax(syntax::Config::new().utf8(false));
// Disables the requirement that empty matches match UTF-8 boundaries.
builder.configure(Regex::config().utf8_empty(false));

// We can match raw bytes via \xZZ syntax, but we need to disable
// Unicode mode to do that. We could disable it everywhere, or just
// selectively, as shown here.
let re = builder.build(r"(?-u:\xFF)foo(?-u:\xFF)")?;
let hay = b"\xFFfoo\xFF";
assert_eq!(Some(Match::must(0, 0..5)), re.find(hay));

// We can also match between code units.
let re = builder.build(r"")?;
let hay = "☃";
assert_eq!(re.find_iter(hay).collect::<Vec<Match>>(), vec![
    Match::must(0, 0..0),
    Match::must(0, 1..1),
    Match::must(0, 2..2),
    Match::must(0, 3..3),
]);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](../index.md)

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](../index.md), [`BuildError`](../index.md)

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](../index.md), [`BuildError`](../index.md)

- <span id="builder-build-from-hir"></span>`fn build_from_hir(&self, hir: &Hir) -> Result<Regex, BuildError>` — [`Regex`](../index.md), [`BuildError`](../index.md)

- <span id="builder-build-many-from-hir"></span>`fn build_many_from_hir<H: Borrow<Hir>>(&self, hirs: &[H]) -> Result<Regex, BuildError>` — [`Regex`](../index.md), [`BuildError`](../index.md)

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](../index.md), [`Builder`](../index.md)

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md), [`Builder`](../index.md)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](../index.md)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `CachePool`

```rust
type CachePool = crate::util::pool::Pool<Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>;
```

A type alias for our pool of meta::Cache that fixes the type parameters to
what we use for the meta regex below.

### `CachePoolGuard<'a>`

```rust
type CachePoolGuard<'a> = crate::util::pool::PoolGuard<'a, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>;
```

Same as above, but for the guard returned by a pool.

### `CachePoolFn`

```rust
type CachePoolFn = alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>;
```

The type of the closure we use to create new caches. We need to spell out
all of the marker traits or else we risk leaking !MARKER impls.

