*[regex_automata](../../index.md) / [util](../index.md) / [iter](index.md)*

---

# Module `iter`

Generic helpers for iteration of matches from a regex engine in a haystack.

The principle type in this module is a [`Searcher`](#searcher). A `Searcher` provides
its own lower level iterator-like API in addition to methods for constructing
types that implement `Iterator`. The documentation for `Searcher` explains a
bit more about why these different APIs exist.

Currently, this module supports iteration over any regex engine that works
with the [`HalfMatch`](../../index.md), [`Match`](../../index.md) or [`Captures`](../captures/index.md) types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Searcher`](#searcher) | struct | A searcher for creating iterators and performing lower level iteration. |
| [`TryHalfMatchesIter`](#tryhalfmatchesiter) | struct | An iterator over all non-overlapping half matches for a fallible search. |
| [`HalfMatchesIter`](#halfmatchesiter) | struct | An iterator over all non-overlapping half matches for an infallible search. |
| [`TryMatchesIter`](#trymatchesiter) | struct | An iterator over all non-overlapping matches for a fallible search. |
| [`MatchesIter`](#matchesiter) | struct | An iterator over all non-overlapping matches for an infallible search. |
| [`TryCapturesIter`](#trycapturesiter) | struct | An iterator over all non-overlapping captures for a fallible search. |
| [`CapturesIter`](#capturesiter) | struct | An iterator over all non-overlapping captures for an infallible search. |

## Structs

### `Searcher<'h>`

```rust
struct Searcher<'h> {
    input: crate::util::search::Input<'h>,
    last_match_end: Option<usize>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:147-156`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/iter.rs#L147-L156)*

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
provides various routines, like `Searcher::into_matches_iter`, that
accept a closure (representing how a regex engine executes a search) and
returns a conventional iterator.

The lifetime parameters come from the [`Input`](../../index.md) type passed to
`Searcher::new`:

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
`Searcher::advance` for an example of using the "advance" API with the
PikeVM.)

What this boils down to is that there are "advance" APIs which require
handing a closure to it for every call, and there are also APIs to create
iterators from a closure. The former are useful for _implementing_
iterators or when you need more flexibility, while the latter are useful
for conveniently writing custom iterators on-the-fly.

# Example: iterating with captures

Several regex engines in this crate over convenient iterator APIs over
[`Captures`](../captures/index.md) values. To do so, this requires allocating a new `Captures`
value for each iteration step. This can perhaps be more costly than you
might want. Instead of implementing your own iterator to avoid that
cost (which can be a little subtle if you want to handle empty matches
correctly), you can use this `Searcher` to do it for you:

```rust
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

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`input`**: `crate::util::search::Input<'h>`

  The input parameters to give to each regex engine call.
  
  The start position of the search is mutated during iteration.

- **`last_match_end`**: `Option<usize>`

  Records the end offset of the most recent match. This is necessary to
  handle a corner case for preventing empty matches from overlapping with
  the ending bounds of a prior match.

#### Implementations

- <span id="searcher-new"></span>`fn new(input: Input<'h>) -> Searcher<'h>` — [`Input`](../../index.md#input), [`Searcher`](#searcher)

  Create a new fallible non-overlapping matches iterator.

  

  The given `input` provides the parameters (including the haystack),

  while the `finder` represents a closure that calls the underlying regex

  engine. The closure may borrow any additional state that is needed,

  such as a prefilter scanner.

- <span id="searcher-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` used by this searcher.

  

  The `Input` returned is generally equivalent to the one given to

  `Searcher::new`, but its start position may be different to reflect

  the start of the next search to be executed.

- <span id="searcher-advance-half"></span>`fn advance_half<F>(&mut self, finder: F) -> Option<HalfMatch>` — [`HalfMatch`](../../index.md#halfmatch)

  Return the next half match for an infallible search if one exists, and

  advance to the next position.

  

  This is like `try_advance_half`, except errors are converted into

  panics.

  

  # Panics

  

  If the given closure returns an error, then this panics. This is useful

  when you know your underlying regex engine has been configured to not

  return an error.

  

  # Example

  

  This example shows how to use a `Searcher` to iterate over all matches

  when using a DFA, which only provides "half" matches.

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      util::iter::Searcher,

      HalfMatch, Input,

  };

  

  let re = DFA::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;

  let mut cache = re.create_cache();

  

  let input = Input::new("2010-03-14 2016-10-08 2020-10-22");

  let mut it = Searcher::new(input);

  

  let expected = Some(HalfMatch::must(0, 10));

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = Some(HalfMatch::must(0, 21));

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = Some(HalfMatch::must(0, 32));

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = None;

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  This correctly moves iteration forward even when an empty match occurs:

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      util::iter::Searcher,

      HalfMatch, Input,

  };

  

  let re = DFA::new(r"a|")?;

  let mut cache = re.create_cache();

  

  let input = Input::new("abba");

  let mut it = Searcher::new(input);

  

  let expected = Some(HalfMatch::must(0, 1));

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = Some(HalfMatch::must(0, 2));

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = Some(HalfMatch::must(0, 4));

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = None;

  let got = it.advance_half(|input| re.try_search_fwd(&mut cache, input));

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="searcher-advance"></span>`fn advance<F>(&mut self, finder: F) -> Option<Match>` — [`Match`](../../index.md#match)

  Return the next match for an infallible search if one exists, and

  advance to the next position.

  

  The search is advanced even in the presence of empty matches by

  forbidding empty matches from overlapping with any other match.

  

  This is like `try_advance`, except errors are converted into panics.

  

  # Panics

  

  If the given closure returns an error, then this panics. This is useful

  when you know your underlying regex engine has been configured to not

  return an error.

  

  # Example

  

  This example shows how to use a `Searcher` to iterate over all matches

  when using a regex based on lazy DFAs:

  

  ```rust

  use regex_automata::{

      hybrid::regex::Regex,

      util::iter::Searcher,

      Match, Input,

  };

  

  let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;

  let mut cache = re.create_cache();

  

  let input = Input::new("2010-03-14 2016-10-08 2020-10-22");

  let mut it = Searcher::new(input);

  

  let expected = Some(Match::must(0, 0..10));

  let got = it.advance(|input| re.try_search(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = Some(Match::must(0, 11..21));

  let got = it.advance(|input| re.try_search(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = Some(Match::must(0, 22..32));

  let got = it.advance(|input| re.try_search(&mut cache, input));

  assert_eq!(expected, got);

  

  let expected = None;

  let got = it.advance(|input| re.try_search(&mut cache, input));

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  This example shows the same as above, but with the PikeVM. This example

  is useful because it shows how to use this API even when the regex

  engine doesn't directly return a `Match`.

  

  ```rust

  use regex_automata::{

      nfa::thompson::pikevm::PikeVM,

      util::iter::Searcher,

      Match, Input,

  };

  

  let re = PikeVM::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let input = Input::new("2010-03-14 2016-10-08 2020-10-22");

  let mut it = Searcher::new(input);

  

  let expected = Some(Match::must(0, 0..10));

  let got = it.advance(|input| {

      re.search(&mut cache, input, &mut caps);

      Ok(caps.get_match())

  });

  // Note that if we wanted to extract capturing group spans, we could

  // do that here with 'caps'.

  assert_eq!(expected, got);

  

  let expected = Some(Match::must(0, 11..21));

  let got = it.advance(|input| {

      re.search(&mut cache, input, &mut caps);

      Ok(caps.get_match())

  });

  assert_eq!(expected, got);

  

  let expected = Some(Match::must(0, 22..32));

  let got = it.advance(|input| {

      re.search(&mut cache, input, &mut caps);

      Ok(caps.get_match())

  });

  assert_eq!(expected, got);

  

  let expected = None;

  let got = it.advance(|input| {

      re.search(&mut cache, input, &mut caps);

      Ok(caps.get_match())

  });

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="searcher-try-advance-half"></span>`fn try_advance_half<F>(&mut self, finder: F) -> Result<Option<HalfMatch>, MatchError>` — [`HalfMatch`](../../index.md#halfmatch), [`MatchError`](../../index.md#matcherror)

  Return the next half match for a fallible search if one exists, and

  advance to the next position.

  

  This is like `advance_half`, except it permits callers to handle errors

  during iteration.

- <span id="searcher-try-advance"></span>`fn try_advance<F>(&mut self, finder: F) -> Result<Option<Match>, MatchError>` — [`Match`](../../index.md#match), [`MatchError`](../../index.md#matcherror)

  Return the next match for a fallible search if one exists, and advance

  to the next position.

  

  This is like `advance`, except it permits callers to handle errors

  during iteration.

- <span id="searcher-into-half-matches-iter"></span>`fn into_half_matches_iter<F>(self, finder: F) -> TryHalfMatchesIter<'h, F>` — [`TryHalfMatchesIter`](#tryhalfmatchesiter)

  Given a closure that executes a single search, return an iterator over

  all successive non-overlapping half matches.

  

  The iterator returned yields result values. If the underlying regex

  engine is configured to never return an error, consider calling

  `TryHalfMatchesIter::infallible` to convert errors into panics.

  

  # Example

  

  This example shows how to use a `Searcher` to create a proper

  iterator over half matches.

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      util::iter::Searcher,

      HalfMatch, Input,

  };

  

  let re = DFA::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;

  let mut cache = re.create_cache();

  

  let input = Input::new("2010-03-14 2016-10-08 2020-10-22");

  let mut it = Searcher::new(input).into_half_matches_iter(|input| {

      re.try_search_fwd(&mut cache, input)

  });

  

  let expected = Some(Ok(HalfMatch::must(0, 10)));

  assert_eq!(expected, it.next());

  

  let expected = Some(Ok(HalfMatch::must(0, 21)));

  assert_eq!(expected, it.next());

  

  let expected = Some(Ok(HalfMatch::must(0, 32)));

  assert_eq!(expected, it.next());

  

  let expected = None;

  assert_eq!(expected, it.next());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="searcher-into-matches-iter"></span>`fn into_matches_iter<F>(self, finder: F) -> TryMatchesIter<'h, F>` — [`TryMatchesIter`](#trymatchesiter)

  Given a closure that executes a single search, return an iterator over

  all successive non-overlapping matches.

  

  The iterator returned yields result values. If the underlying regex

  engine is configured to never return an error, consider calling

  `TryMatchesIter::infallible` to convert errors into panics.

  

  # Example

  

  This example shows how to use a `Searcher` to create a proper

  iterator over matches.

  

  ```rust

  use regex_automata::{

      hybrid::regex::Regex,

      util::iter::Searcher,

      Match, Input,

  };

  

  let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;

  let mut cache = re.create_cache();

  

  let input = Input::new("2010-03-14 2016-10-08 2020-10-22");

  let mut it = Searcher::new(input).into_matches_iter(|input| {

      re.try_search(&mut cache, input)

  });

  

  let expected = Some(Ok(Match::must(0, 0..10)));

  assert_eq!(expected, it.next());

  

  let expected = Some(Ok(Match::must(0, 11..21)));

  assert_eq!(expected, it.next());

  

  let expected = Some(Ok(Match::must(0, 22..32)));

  assert_eq!(expected, it.next());

  

  let expected = None;

  assert_eq!(expected, it.next());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="searcher-into-captures-iter"></span>`fn into_captures_iter<F>(self, caps: Captures, finder: F) -> TryCapturesIter<'h, F>` — [`Captures`](../captures/index.md#captures), [`TryCapturesIter`](#trycapturesiter)

  Given a closure that executes a single search, return an iterator over

  all successive non-overlapping `Captures` values.

  

  The iterator returned yields result values. If the underlying regex

  engine is configured to never return an error, consider calling

  `TryCapturesIter::infallible` to convert errors into panics.

  

  Unlike the other iterator constructors, this accepts an initial

  `Captures` value. This `Captures` value is reused for each search, and

  the iterator implementation clones it before returning it. The caller

  must provide this value because the iterator is purposely ignorant

  of the underlying regex engine and thus doesn't know how to create

  one itself. More to the point, a `Captures` value itself has a few

  different constructors, which change which kind of information is

  available to query in exchange for search performance.

  

  # Example

  

  This example shows how to use a `Searcher` to create a proper iterator

  over `Captures` values, which provides access to all capturing group

  spans for each match.

  

  ```rust

  use regex_automata::{

      nfa::thompson::pikevm::PikeVM,

      util::iter::Searcher,

      Input,

  };

  

  let re = PikeVM::new(

      r"(?P<y>[0-9]{4})-(?P<m>[0-9]{2})-(?P<d>[0-9]{2})",

  )?;

  let (mut cache, caps) = (re.create_cache(), re.create_captures());

  

  let haystack = "2010-03-14 2016-10-08 2020-10-22";

  let input = Input::new(haystack);

  let mut it = Searcher::new(input)

      .into_captures_iter(caps, |input, caps| {

          re.search(&mut cache, input, caps);

          Ok(())

      });

  

  let got = it.next().expect("first date")?;

  let year = got.get_group_by_name("y").expect("must match");

  assert_eq!("2010", &haystack[year]);

  

  let got = it.next().expect("second date")?;

  let month = got.get_group_by_name("m").expect("must match");

  assert_eq!("10", &haystack[month]);

  

  let got = it.next().expect("third date")?;

  let day = got.get_group_by_name("d").expect("must match");

  assert_eq!("22", &haystack[day]);

  

  assert!(it.next().is_none());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="searcher-handle-overlapping-empty-half-match"></span>`fn handle_overlapping_empty_half_match<F>(&mut self, _: HalfMatch, finder: F) -> Result<Option<HalfMatch>, MatchError>` — [`HalfMatch`](../../index.md#halfmatch), [`MatchError`](../../index.md#matcherror)

  Handles the special case of a match that begins where the previous

  match ended. Without this special handling, it'd be possible to get

  stuck where an empty match never results in forward progress. This

  also makes it more consistent with how presiding general purpose regex

  engines work.

- <span id="searcher-handle-overlapping-empty-match"></span>`fn handle_overlapping_empty_match<F>(&mut self, m: Match, finder: F) -> Result<Option<Match>, MatchError>` — [`Match`](../../index.md#match), [`MatchError`](../../index.md#matcherror)

  Handles the special case of an empty match by ensuring that 1) the

  iterator always advances and 2) empty matches never overlap with other

  matches.

  

  (1) is necessary because we principally make progress by setting the

  starting location of the next search to the ending location of the last

  match. But if a match is empty, then this results in a search that does

  not advance and thus does not terminate.

  

  (2) is not strictly necessary, but makes intuitive sense and matches

  the presiding behavior of most general purpose regex engines. The

  "intuitive sense" here is that we want to report NON-overlapping

  matches. So for example, given the regex 'a|(?:)' against the haystack

  'a', without the special handling, you'd get the matches [0, 1) and [1,

  1), where the latter overlaps with the end bounds of the former.

  

  Note that we mark this cold and forcefully prevent inlining because

  handling empty matches like this is extremely rare and does require

  quite a bit of code, comparatively. Keeping this code out of the main

  iterator function keeps it smaller and more amenable to inlining

  itself.

#### Trait Implementations

##### `impl Any for Searcher<'h>`

- <span id="searcher-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Searcher<'h>`

- <span id="searcher-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Searcher<'h>`

- <span id="searcher-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Searcher<'h>`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher<'h>` — [`Searcher`](#searcher)

##### `impl CloneToUninit for Searcher<'h>`

- <span id="searcher-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Searcher<'h>`

- <span id="searcher-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Searcher<'h>`

- <span id="searcher-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Searcher<'h>`

- <span id="searcher-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Searcher<'h>`

- <span id="searcher-toowned-type-owned"></span>`type Owned = T`

- <span id="searcher-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searcher-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Searcher<'h>`

- <span id="searcher-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searcher-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Searcher<'h>`

- <span id="searcher-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searcher-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryHalfMatchesIter<'h, F>`

```rust
struct TryHalfMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:699-702`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/iter.rs#L699-L702)*

An iterator over all non-overlapping half matches for a fallible search.

The iterator yields a `Result<HalfMatch, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_half_matches_iter`.

#### Implementations

- <span id="tryhalfmatchesiter-infallible"></span>`fn infallible(self) -> HalfMatchesIter<'h, F>` — [`HalfMatchesIter`](#halfmatchesiter)

  Return an infallible version of this iterator.

  

  Any item yielded that corresponds to an error results in a panic. This

  is useful if your underlying regex engine is configured in a way that

  it is guaranteed to never return an error.

- <span id="tryhalfmatchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` used by this iterator.

  

  The `Input` returned is generally equivalent to the one used to

  construct this iterator, but its start position may be different to

  reflect the start of the next search to be executed.

#### Trait Implementations

##### `impl Any for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F> Debug for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tryhalfmatchesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tryhalfmatchesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<F> Iterator for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-iterator-type-item"></span>`type Item = Result<HalfMatch, MatchError>`

- <span id="tryhalfmatchesiter-iterator-next"></span>`fn next(&mut self) -> Option<Result<HalfMatch, MatchError>>` — [`HalfMatch`](../../index.md#halfmatch), [`MatchError`](../../index.md#matcherror)

##### `impl<U> TryFrom for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryhalfmatchesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryhalfmatchesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HalfMatchesIter<'h, F>`

```rust
struct HalfMatchesIter<'h, F>(TryHalfMatchesIter<'h, F>);
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:765`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/iter.rs#L765)*

An iterator over all non-overlapping half matches for an infallible search.

The iterator yields a [`HalfMatch`](../../index.md) value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_half_matches_iter` and
then calling `TryHalfMatchesIter::infallible`.

#### Implementations

- <span id="halfmatchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` used by this iterator.

  

  The `Input` returned is generally equivalent to the one used to

  construct this iterator, but its start position may be different to

  reflect the start of the next search to be executed.

#### Trait Implementations

##### `impl Any for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: fmt::Debug> Debug for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="halfmatchesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="halfmatchesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<F> Iterator for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-iterator-type-item"></span>`type Item = HalfMatch`

- <span id="halfmatchesiter-iterator-next"></span>`fn next(&mut self) -> Option<HalfMatch>` — [`HalfMatch`](../../index.md#halfmatch)

##### `impl<U> TryFrom for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="halfmatchesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="halfmatchesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryMatchesIter<'h, F>`

```rust
struct TryMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:814-817`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/iter.rs#L814-L817)*

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_matches_iter`.

#### Implementations

- <span id="trymatchesiter-infallible"></span>`fn infallible(self) -> MatchesIter<'h, F>` — [`MatchesIter`](#matchesiter)

  Return an infallible version of this iterator.

  

  Any item yielded that corresponds to an error results in a panic. This

  is useful if your underlying regex engine is configured in a way that

  it is guaranteed to never return an error.

- <span id="trymatchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` used by this iterator.

  

  The `Input` returned is generally equivalent to the one used to

  construct this iterator, but its start position may be different to

  reflect the start of the next search to be executed.

#### Trait Implementations

##### `impl Any for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F> Debug for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="trymatchesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="trymatchesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<F> Iterator for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-iterator-type-item"></span>`type Item = Result<Match, MatchError>`

- <span id="trymatchesiter-iterator-next"></span>`fn next(&mut self) -> Option<Result<Match, MatchError>>` — [`Match`](../../index.md#match), [`MatchError`](../../index.md#matcherror)

##### `impl<U> TryFrom for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trymatchesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trymatchesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MatchesIter<'h, F>`

```rust
struct MatchesIter<'h, F>(TryMatchesIter<'h, F>);
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:879`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/iter.rs#L879)*

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`](../../index.md) value until no more matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_matches_iter` and
then calling `TryMatchesIter::infallible`.

#### Implementations

- <span id="matchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` used by this iterator.

  

  The `Input` returned is generally equivalent to the one used to

  construct this iterator, but its start position may be different to

  reflect the start of the next search to be executed.

#### Trait Implementations

##### `impl Any for MatchesIter<'h, F>`

- <span id="matchesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchesIter<'h, F>`

- <span id="matchesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchesIter<'h, F>`

- <span id="matchesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: fmt::Debug> Debug for MatchesIter<'h, F>`

- <span id="matchesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MatchesIter<'h, F>`

- <span id="matchesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchesIter<'h, F>`

- <span id="matchesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MatchesIter<'h, F>`

- <span id="matchesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matchesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="matchesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<F> Iterator for MatchesIter<'h, F>`

- <span id="matchesiter-iterator-type-item"></span>`type Item = Match`

- <span id="matchesiter-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../index.md#match)

##### `impl<U> TryFrom for MatchesIter<'h, F>`

- <span id="matchesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matchesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchesIter<'h, F>`

- <span id="matchesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matchesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryCapturesIter<'h, F>`

```rust
struct TryCapturesIter<'h, F> {
    it: Searcher<'h>,
    caps: crate::util::captures::Captures,
    finder: F,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:929-933`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/iter.rs#L929-L933)*

An iterator over all non-overlapping captures for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_captures_iter`.

#### Implementations

- <span id="trycapturesiter-infallible"></span>`fn infallible(self) -> CapturesIter<'h, F>` — [`CapturesIter`](#capturesiter)

  Return an infallible version of this iterator.

  

  Any item yielded that corresponds to an error results in a panic. This

  is useful if your underlying regex engine is configured in a way that

  it is guaranteed to never return an error.

#### Trait Implementations

##### `impl Any for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F> Debug for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="trycapturesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="trycapturesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<F> Iterator for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-iterator-type-item"></span>`type Item = Result<Captures, MatchError>`

- <span id="trycapturesiter-iterator-next"></span>`fn next(&mut self) -> Option<Result<Captures, MatchError>>` — [`Captures`](../captures/index.md#captures), [`MatchError`](../../index.md#matcherror)

##### `impl<U> TryFrom for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trycapturesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trycapturesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CapturesIter<'h, F>`

```rust
struct CapturesIter<'h, F>(TryCapturesIter<'h, F>);
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:1003`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/iter.rs#L1003)*

An iterator over all non-overlapping captures for an infallible search.

The iterator yields a [`Captures`](../captures/index.md) value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_captures_iter` and then
calling `TryCapturesIter::infallible`.

#### Trait Implementations

##### `impl Any for CapturesIter<'h, F>`

- <span id="capturesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CapturesIter<'h, F>`

- <span id="capturesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CapturesIter<'h, F>`

- <span id="capturesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: fmt::Debug> Debug for CapturesIter<'h, F>`

- <span id="capturesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CapturesIter<'h, F>`

- <span id="capturesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CapturesIter<'h, F>`

- <span id="capturesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CapturesIter<'h, F>`

- <span id="capturesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<F> Iterator for CapturesIter<'h, F>`

- <span id="capturesiter-iterator-type-item"></span>`type Item = Captures`

- <span id="capturesiter-iterator-next"></span>`fn next(&mut self) -> Option<Captures>` — [`Captures`](../captures/index.md#captures)

##### `impl<U> TryFrom for CapturesIter<'h, F>`

- <span id="capturesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CapturesIter<'h, F>`

- <span id="capturesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

