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
| [`Regex`](#regex) | struct | A regex matcher that works by composing several other regex matchers automatically. |
| [`RegexI`](#regexi) | struct | The internal implementation of `Regex`, split out so that it can be wrapped in an `Arc`. |
| [`RegexInfo`](#regexinfo) | struct |  |
| [`RegexInfoI`](#regexinfoi) | struct |  |
| [`FindMatches`](#findmatches) | struct | An iterator over all non-overlapping matches. |
| [`CapturesMatches`](#capturesmatches) | struct | An iterator over all non-overlapping leftmost matches with their capturing groups. |
| [`Split`](#split) | struct | Yields all substrings delimited by a regular expression match. |
| [`SplitN`](#splitn) | struct | Yields at most `N` spans delimited by a regular expression match. |
| [`Cache`](#cache) | struct | Represents mutable scratch space used by regex engines during a search. |
| [`Config`](#config) | struct | An object describing the configuration of a `Regex`. |
| [`Builder`](#builder) | struct | A builder for configuring and constructing a `Regex`. |
| [`CachePool`](#cachepool) | type | A type alias for our pool of meta::Cache that fixes the type parameters to what we use for the meta regex below. |
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

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:235-252`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L235-L252)*

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
mutable scratch space. This scratch space is called a [`Cache`](#cache).

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

- <span id="regex-new"></span>`fn new(pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

  Builds a `Regex` from a single pattern string using the default

  configuration.

  

  If there was a problem parsing the pattern or a problem turning it into

  a regex matcher, then an error is returned.

  

  If you want to change the configuration of a `Regex`, use a [`Builder`](#builder)

  with a [`Config`](#config).

  

  # Example

  

  ```rust

  use regex_automata::{meta::Regex, Match};

  

  let re = Regex::new(r"(?Rm)^foo$")?;

  let hay = "\r\nfoo\r\n";

  assert_eq!(Some(Match::must(0, 2..5)), re.find(hay));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="regex-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

  Builds a `Regex` from many pattern strings using the default

  configuration.

  

  If there was a problem parsing any of the patterns or a problem turning

  them into a regex matcher, then an error is returned.

  

  If you want to change the configuration of a `Regex`, use a [`Builder`](#builder)

  with a [`Config`](#config).

  

  # Example: simple lexer

  

  This simplistic example leverages the multi-pattern support to build a

  simple little lexer. The pattern ID in the match tells you which regex

  matched, which in turn might be used to map back to the "type" of the

  token returned by the lexer.

  

  ```rust

  use regex_automata::{meta::Regex, Match};

  

  let re = Regex::new_many(&[

      r"[[:space:]]",

      r"[A-Za-z0-9][A-Za-z0-9_]+",

      r"->",

      r".",

  ])?;

  let haystack = "fn is_boss(bruce: i32, springsteen: String) -> bool;";

  let matches: Vec<Match> = re.find_iter(haystack).collect();

  assert_eq!(matches, vec![

      Match::must(1, 0..2),   // 'fn'

      Match::must(0, 2..3),   // ' '

      Match::must(1, 3..10),  // 'is_boss'

      Match::must(3, 10..11), // '('

      Match::must(1, 11..16), // 'bruce'

      Match::must(3, 16..17), // ':'

      Match::must(0, 17..18), // ' '

      Match::must(1, 18..21), // 'i32'

      Match::must(3, 21..22), // ','

      Match::must(0, 22..23), // ' '

      Match::must(1, 23..34), // 'springsteen'

      Match::must(3, 34..35), // ':'

      Match::must(0, 35..36), // ' '

      Match::must(1, 36..42), // 'String'

      Match::must(3, 42..43), // ')'

      Match::must(0, 43..44), // ' '

      Match::must(2, 44..46), // '->'

      Match::must(0, 46..47), // ' '

      Match::must(1, 47..51), // 'bool'

      Match::must(3, 51..52), // ';'

  ]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  One can write a lexer like the above using a regex like

  `(?P<space>[[:space:]])|(?P<ident>[A-Za-z0-9][A-Za-z0-9_]+)|...`,

  but then you need to ask whether capture group matched to determine

  which branch in the regex matched, and thus, which token the match

  corresponds to. In contrast, the above example includes the pattern ID

  in the match. There's no need to use capture groups at all.

  

  # Example: finding the pattern that caused an error

  

  When a syntax error occurs, it is possible to ask which pattern

  caused the syntax error.

  

  ```rust

  use regex_automata::{meta::Regex, PatternID};

  

  let err = Regex::new_many(&["a", "b", r"\p{Foo}", "c"]).unwrap_err();

  assert_eq!(Some(PatternID::must(2)), err.pattern());

  ```

  

  # Example: zero patterns is valid

  

  Building a regex with zero patterns results in a regex that never

  matches anything. Because this routine is generic, passing an empty

  slice usually requires a turbo-fish (or something else to help type

  inference).

  

  ```rust

  use regex_automata::{meta::Regex, util::syntax, Match};

  

  let re = Regex::new_many::<&str>(&[])?;

  assert_eq!(None, re.find(""));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="regex-config"></span>`fn config() -> Config` — [`Config`](#config)

  Return a default configuration for a `Regex`.

  

  This is a convenience routine to avoid needing to import the [`Config`](#config)

  type when customizing the construction of a `Regex`.

  

  # Example: lower the NFA size limit

  

  In some cases, the default size limit might be too big. The size limit

  can be lowered, which will prevent large regex patterns from compiling.

  

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

- <span id="regex-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  Return a builder for configuring the construction of a `Regex`.

  

  This is a convenience routine to avoid needing to import the

  [`Builder`](#builder) type in common cases.

  

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

#### Trait Implementations

##### `impl Any for Regex`

- <span id="regex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Regex`

- <span id="regex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Regex`

- <span id="regex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Regex`

- <span id="regex-clone"></span>`fn clone(&self) -> Regex` — [`Regex`](#regex)

##### `impl CloneToUninit for Regex`

- <span id="regex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Regex`

- <span id="regex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Regex`

- <span id="regex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Regex`

- <span id="regex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Regex`

- <span id="regex-toowned-type-owned"></span>`type Owned = T`

- <span id="regex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="regex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Regex`

- <span id="regex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Regex`

- <span id="regex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegexI`

```rust
struct RegexI {
    strat: alloc::sync::Arc<dyn Strategy>,
    info: RegexInfo,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:257-278`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L257-L278)*

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

##### `impl Any for RegexI`

- <span id="regexi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegexI`

- <span id="regexi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegexI`

- <span id="regexi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RegexI`

- <span id="regexi-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RegexI`

- <span id="regexi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RegexI`

- <span id="regexi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RegexI`

- <span id="regexi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regexi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RegexI`

- <span id="regexi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regexi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegexInfo`

```rust
struct RegexInfo(alloc::sync::Arc<RegexInfoI>);
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:1924`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L1924)*

#### Implementations

- <span id="regexinfo-new"></span>`fn new(config: Config, hirs: &[&Hir]) -> RegexInfo` — [`Config`](#config), [`RegexInfo`](#regexinfo)

- <span id="regexinfo-config"></span>`fn config(&self) -> &Config` — [`Config`](#config)

- <span id="regexinfo-props"></span>`fn props(&self) -> &[hir::Properties]`

- <span id="regexinfo-props-union"></span>`fn props_union(&self) -> &hir::Properties`

- <span id="regexinfo-pattern-len"></span>`fn pattern_len(&self) -> usize`

- <span id="regexinfo-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="regexinfo-is-anchored-start"></span>`fn is_anchored_start(&self, input: &Input<'_>) -> bool` — [`Input`](../../index.md#input)

  Returns true when the search is guaranteed to be anchored. That is,

  when a match is reported, its offset is guaranteed to correspond to

  the start of the search.

  

  This includes returning true when `input` _isn't_ anchored but the

  underlying regex is.

- <span id="regexinfo-is-always-anchored-start"></span>`fn is_always_anchored_start(&self) -> bool`

  Returns true when this regex is always anchored to the start of a

  search. And in particular, that regardless of an `Input` configuration,

  if any match is reported it must start at `0`.

- <span id="regexinfo-is-always-anchored-end"></span>`fn is_always_anchored_end(&self) -> bool`

  Returns true when this regex is always anchored to the end of a

  search. And in particular, that regardless of an `Input` configuration,

  if any match is reported it must end at the end of the haystack.

- <span id="regexinfo-captures-disabled"></span>`fn captures_disabled(&self) -> bool`

  Returns true when the regex's NFA lacks capture states.

  

  In this case, some regex engines (like the PikeVM) are unable to report

  match offsets, while some (like the lazy DFA can). To avoid whether a

  match or not is reported based on engine selection, routines that

  return match offsets will _always_ report `None` when this is true.

  

  Yes, this is a weird case and it's a little fucked up. But

  `WhichCaptures::None` comes with an appropriate warning.

- <span id="regexinfo-is-impossible"></span>`fn is_impossible(&self, input: &Input<'_>) -> bool` — [`Input`](../../index.md#input)

  Returns true if and only if it is known that a match is impossible

  for the given input. This is useful for short-circuiting and avoiding

  running the regex engine if it's known no match can be reported.

  

  Note that this doesn't necessarily detect every possible case. For

  example, when `pattern_len() == 0`, a match is impossible, but that

  case is so rare that it's fine to be handled by the regex engine

  itself. That is, it's not worth the cost of adding it here in order to

  make it a little faster. The reason is that this is called for every

  search. so there is some cost to adding checks here. Arguably, some of

  the checks that are here already probably shouldn't be here...

#### Trait Implementations

##### `impl Any for RegexInfo`

- <span id="regexinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegexInfo`

- <span id="regexinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegexInfo`

- <span id="regexinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RegexInfo`

- <span id="regexinfo-clone"></span>`fn clone(&self) -> RegexInfo` — [`RegexInfo`](#regexinfo)

##### `impl CloneToUninit for RegexInfo`

- <span id="regexinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RegexInfo`

- <span id="regexinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RegexInfo`

- <span id="regexinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RegexInfo`

- <span id="regexinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RegexInfo`

- <span id="regexinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="regexinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="regexinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RegexInfo`

- <span id="regexinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regexinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RegexInfo`

- <span id="regexinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regexinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegexInfoI`

```rust
struct RegexInfoI {
    config: Config,
    props: alloc::vec::Vec<hir::Properties>,
    props_union: hir::Properties,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:1927-1931`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L1927-L1931)*

#### Trait Implementations

##### `impl Any for RegexInfoI`

- <span id="regexinfoi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegexInfoI`

- <span id="regexinfoi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegexInfoI`

- <span id="regexinfoi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RegexInfoI`

- <span id="regexinfoi-clone"></span>`fn clone(&self) -> RegexInfoI` — [`RegexInfoI`](#regexinfoi)

##### `impl CloneToUninit for RegexInfoI`

- <span id="regexinfoi-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RegexInfoI`

- <span id="regexinfoi-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RegexInfoI`

- <span id="regexinfoi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RegexInfoI`

- <span id="regexinfoi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RegexInfoI`

- <span id="regexinfoi-toowned-type-owned"></span>`type Owned = T`

- <span id="regexinfoi-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="regexinfoi-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RegexInfoI`

- <span id="regexinfoi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regexinfoi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RegexInfoI`

- <span id="regexinfoi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regexinfoi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FindMatches<'r, 'h>`

```rust
struct FindMatches<'r, 'h> {
    re: &'r Regex,
    cache: crate::util::pool::PoolGuard<'r, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2075-2079`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2075-L2079)*

An iterator over all non-overlapping matches.

The iterator yields a [`Match`](../../index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::find_iter` method.

#### Implementations

- <span id="findmatches-regex"></span>`fn regex(&self) -> &'r Regex` — [`Regex`](#regex)

  Returns the `Regex` value that created this iterator.

- <span id="findmatches-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` associated with this iterator.

  

  The `start` position on the given `Input` may change during iteration,

  but all other values are guaranteed to remain invariant.

#### Trait Implementations

##### `impl Any for FindMatches<'r, 'h>`

- <span id="findmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindMatches<'r, 'h>`

- <span id="findmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindMatches<'r, 'h>`

- <span id="findmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FindMatches<'r, 'h>`

- <span id="findmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FindMatches<'r, 'h>`

- <span id="findmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for FindMatches<'r, 'h>`

##### `impl<U> Into for FindMatches<'r, 'h>`

- <span id="findmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for FindMatches<'r, 'h>`

- <span id="findmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="findmatches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindMatches<'r, 'h>`

- <span id="findmatches-iterator-type-item"></span>`type Item = Match`

- <span id="findmatches-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../index.md#match)

- <span id="findmatches-iterator-count"></span>`fn count(self) -> usize`

##### `impl<U> TryFrom for FindMatches<'r, 'h>`

- <span id="findmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindMatches<'r, 'h>`

- <span id="findmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CapturesMatches<'r, 'h>`

```rust
struct CapturesMatches<'r, 'h> {
    re: &'r Regex,
    cache: crate::util::pool::PoolGuard<'r, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2138-2143`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2138-L2143)*

An iterator over all non-overlapping leftmost matches with their capturing
groups.

The iterator yields a [`Captures`](../../util/captures/index.md) value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::captures_iter` method.

#### Implementations

- <span id="capturesmatches-regex"></span>`fn regex(&self) -> &'r Regex` — [`Regex`](#regex)

  Returns the `Regex` value that created this iterator.

- <span id="capturesmatches-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` associated with this iterator.

  

  The `start` position on the given `Input` may change during iteration,

  but all other values are guaranteed to remain invariant.

#### Trait Implementations

##### `impl Any for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for CapturesMatches<'r, 'h>`

##### `impl<U> Into for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturesmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturesmatches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-iterator-type-item"></span>`type Item = Captures`

- <span id="capturesmatches-iterator-next"></span>`fn next(&mut self) -> Option<Captures>` — [`Captures`](../../util/captures/index.md#captures)

- <span id="capturesmatches-iterator-count"></span>`fn count(self) -> usize`

##### `impl<U> TryFrom for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturesmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturesmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Split<'r, 'h>`

```rust
struct Split<'r, 'h> {
    finder: FindMatches<'r, 'h>,
    last: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2206-2209`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2206-L2209)*

Yields all substrings delimited by a regular expression match.

The spans correspond to the offsets between matches.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::split` method.

#### Implementations

- <span id="split-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` associated with this iterator.

  

  The `start` position on the given `Input` may change during iteration,

  but all other values are guaranteed to remain invariant.

#### Trait Implementations

##### `impl Any for Split<'r, 'h>`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split<'r, 'h>`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split<'r, 'h>`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Split<'r, 'h>`

- <span id="split-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Split<'r, 'h>`

- <span id="split-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Split<'r, 'h>`

##### `impl<U> Into for Split<'r, 'h>`

- <span id="split-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Split<'r, 'h>`

- <span id="split-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="split-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Split<'r, 'h>`

- <span id="split-iterator-type-item"></span>`type Item = Span`

- <span id="split-iterator-next"></span>`fn next(&mut self) -> Option<Span>` — [`Span`](../../index.md#span)

##### `impl<U> TryFrom for Split<'r, 'h>`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Split<'r, 'h>`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    splits: Split<'r, 'h>,
    limit: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2260-2263`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2260-L2263)*

Yields at most `N` spans delimited by a regular expression match.

The spans correspond to the offsets between matches. The last span will be
whatever remains after splitting.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::splitn` method.

#### Implementations

- <span id="splitn-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md#input)

  Returns the current `Input` associated with this iterator.

  

  The `start` position on the given `Input` may change during iteration,

  but all other values are guaranteed to remain invariant.

#### Trait Implementations

##### `impl Any for SplitN<'r, 'h>`

- <span id="splitn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitN<'r, 'h>`

- <span id="splitn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitN<'r, 'h>`

- <span id="splitn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SplitN<'r, 'h>`

- <span id="splitn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitN<'r, 'h>`

- <span id="splitn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for SplitN<'r, 'h>`

##### `impl<U> Into for SplitN<'r, 'h>`

- <span id="splitn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SplitN<'r, 'h>`

- <span id="splitn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splitn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splitn-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SplitN<'r, 'h>`

- <span id="splitn-iterator-type-item"></span>`type Item = Span`

- <span id="splitn-iterator-next"></span>`fn next(&mut self) -> Option<Span>` — [`Span`](../../index.md#span)

- <span id="splitn-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for SplitN<'r, 'h>`

- <span id="splitn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitN<'r, 'h>`

- <span id="splitn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2353-2360`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2353-L2360)*

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

- <span id="cache-new"></span>`fn new(re: &Regex) -> Cache` — [`Regex`](#regex), [`Cache`](#cache)

  Creates a new `Cache` for use with this regex.

  

  The cache returned should only be used for searches for the given

  `Regex`. If you want to reuse the cache for another `Regex`, then you

  must call `Cache::reset` with that `Regex`.

- <span id="cache-reset"></span>`fn reset(&mut self, re: &Regex)` — [`Regex`](#regex)

  Reset this cache such that it can be used for searching with the given

  `Regex` (and only that `Regex`).

  

  A cache reset permits potentially reusing memory already allocated in

  this cache with a different `Regex`.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different `Regex`.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{meta::Regex, Match, Input};

  

  let re1 = Regex::new(r"\w")?;

  let re2 = Regex::new(r"\W")?;

  

  let mut cache = re1.create_cache();

  assert_eq!(

      Some(Match::must(0, 0..2)),

      re1.search_with(&mut cache, &Input::new("Δ")),

  );

  

  // Using 'cache' with re2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the Regex we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 're1' is also not

  // allowed.

  cache.reset(&re2);

  assert_eq!(

      Some(Match::must(0, 0..3)),

      re2.search_with(&mut cache, &Input::new("☃")),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, of this cache.

  

  This does **not** include the stack size used up by this cache. To

  compute that, use `std::mem::size_of::<Cache>()`.

#### Trait Implementations

##### `impl Any for Cache`

- <span id="cache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cache`

- <span id="cache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cache`

- <span id="cache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl CloneToUninit for Cache`

- <span id="cache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Cache`

- <span id="cache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Cache`

- <span id="cache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cache`

- <span id="cache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Cache`

- <span id="cache-toowned-type-owned"></span>`type Owned = T`

- <span id="cache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Cache`

- <span id="cache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cache`

- <span id="cache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2453-2477`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2453-L2477)*

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

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

  Create a new configuration object for a `Regex`.

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md#matchkind), [`Config`](#config)

  Set the match semantics for a `Regex`.

  

  The default value is [`MatchKind::LeftmostFirst`](../../index.md).

  

  # Example

  

  ```rust

  use regex_automata::{meta::Regex, Match, MatchKind};

  

  // By default, leftmost-first semantics are used, which

  // disambiguates matches at the same position by selecting

  // the one that corresponds earlier in the pattern.

  let re = Regex::new("sam|samwise")?;

  assert_eq!(Some(Match::must(0, 0..3)), re.find("samwise"));

  

  // But with 'all' semantics, match priority is ignored

  // and all match states are included. When coupled with

  // a leftmost search, the search will report the last

  // possible match.

  let re = Regex::builder()

      .configure(Regex::config().match_kind(MatchKind::All))

      .build("sam|samwise")?;

  assert_eq!(Some(Match::must(0, 0..7)), re.find("samwise"));

  // Beware that this can lead to skipping matches!

  // Usually 'all' is used for anchored reverse searches

  // only, or for overlapping searches.

  assert_eq!(Some(Match::must(0, 4..11)), re.find("sam samwise"));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-utf8-empty"></span>`fn utf8_empty(self, yes: bool) -> Config` — [`Config`](#config)

  Toggles whether empty matches are permitted to occur between the code

  units of a UTF-8 encoded codepoint.

  

  This should generally be enabled when search a `&str` or anything that

  you otherwise know is valid UTF-8. It should be disabled in all other

  cases. Namely, if the haystack is not valid UTF-8 and this is enabled,

  then behavior is unspecified.

  

  By default, this is enabled.

  

  # Example

  

  ```rust

  use regex_automata::{meta::Regex, Match};

  

  let re = Regex::new("")?;

  let got: Vec<Match> = re.find_iter("☃").collect();

  // Matches only occur at the beginning and end of the snowman.

  assert_eq!(got, vec![

      Match::must(0, 0..0),

      Match::must(0, 3..3),

  ]);

  

  let re = Regex::builder()

      .configure(Regex::config().utf8_empty(false))

      .build("")?;

  let got: Vec<Match> = re.find_iter("☃").collect();

  // Matches now occur at every position!

  assert_eq!(got, vec![

      Match::must(0, 0..0),

      Match::must(0, 1..1),

      Match::must(0, 2..2),

      Match::must(0, 3..3),

  ]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-auto-prefilter"></span>`fn auto_prefilter(self, yes: bool) -> Config` — [`Config`](#config)

  Toggles whether automatic prefilter support is enabled.

  

  If this is disabled and `Config::prefilter` is not set, then the

  meta regex engine will not use any prefilters. This can sometimes

  be beneficial in cases where you know (or have measured) that the

  prefilter leads to overall worse search performance.

  

  By default, this is enabled.

  

  # Example

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{meta::Regex, Match};

  

  let re = Regex::builder()

      .configure(Regex::config().auto_prefilter(false))

      .build(r"Bruce \w+")?;

  let hay = "Hello Bruce Springsteen!";

  assert_eq!(Some(Match::must(0, 6..23)), re.find(hay));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../util/prefilter/index.md#prefilter), [`Config`](#config)

  Overrides and sets the prefilter to use inside a `Regex`.

  

  This permits one to forcefully set a prefilter in cases where the

  caller knows better than whatever the automatic prefilter logic is

  capable of.

  

  By default, this is set to `None` and an automatic prefilter will be

  used if one could be built. (Assuming `Config::auto_prefilter` is

  enabled, which it is by default.)

  

  # Example

  

  This example shows how to set your own prefilter. In the case of a

  pattern like `Bruce \w+`, the automatic prefilter is likely to be

  constructed in a way that it will look for occurrences of `Bruce `.

  In most cases, this is the best choice. But in some cases, it may be

  the case that running `memchr` on `B` is the best choice. One can

  achieve that behavior by overriding the automatic prefilter logic

  and providing a prefilter that just matches `B`.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      meta::Regex,

      util::prefilter::Prefilter,

      Match, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["B"])

      .expect("a prefilter");

  let re = Regex::builder()

      .configure(Regex::config().prefilter(Some(pre)))

      .build(r"Bruce \w+")?;

  let hay = "Hello Bruce Springsteen!";

  assert_eq!(Some(Match::must(0, 6..23)), re.find(hay));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  # Example: incorrect prefilters can lead to incorrect results!

  

  Be warned that setting an incorrect prefilter can lead to missed

  matches. So if you use this option, ensure your prefilter can _never_

  report false negatives. (A false positive is, on the other hand, quite

  okay and generally unavoidable.)

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      meta::Regex,

      util::prefilter::Prefilter,

      Match, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["Z"])

      .expect("a prefilter");

  let re = Regex::builder()

      .configure(Regex::config().prefilter(Some(pre)))

      .build(r"Bruce \w+")?;

  let hay = "Hello Bruce Springsteen!";

  // Oops! No match found, but there should be one!

  assert_eq!(None, re.find(hay));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-which-captures"></span>`fn which_captures(self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](../../nfa/thompson/compiler/index.md#whichcaptures), [`Config`](#config)

  Configures what kinds of groups are compiled as "capturing" in the

  underlying regex engine.

  

  This is set to [`WhichCaptures::All`](../../index.md) by default. Callers may wish to

  use [`WhichCaptures::Implicit`](../../index.md) in cases where one wants avoid the

  overhead of capture states for explicit groups.

  

  Note that another approach to avoiding the overhead of capture groups

  is by using non-capturing groups in the regex pattern. That is,

  `(?:a)` instead of `(a)`. This option is useful when you can't control

  the concrete syntax but know that you don't need the underlying capture

  states. For example, using `WhichCaptures::Implicit` will behave as if

  all explicit capturing groups in the pattern were non-capturing.

  

  Setting this to `WhichCaptures::None` is usually not the right thing to

  do. When no capture states are compiled, some regex engines (such as

  the `PikeVM`) won't be able to report match offsets. This will manifest

  as no match being found. Indeed, in order to enforce consistent

  behavior, the meta regex engine will always report `None` for routines

  that return match offsets even if one of its regex engines could

  service the request. This avoids "match or not" behavior from being

  influenced by user input (since user input can influence the selection

  of the regex engine).

  

  # Example

  

  This example demonstrates how the results of capture groups can change

  based on this option. First we show the default (all capture groups in

  the pattern are capturing):

  

  ```rust

  use regex_automata::{meta::Regex, Match, Span};

  

  let re = Regex::new(r"foo([0-9]+)bar")?;

  let hay = "foo123bar";

  

  let mut caps = re.create_captures();

  re.captures(hay, &mut caps);

  assert_eq!(Some(Span::from(0..9)), caps.get_group(0));

  assert_eq!(Some(Span::from(3..6)), caps.get_group(1));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  And now we show the behavior when we only include implicit capture

  groups. In this case, we can only find the overall match span, but the

  spans of any other explicit group don't exist because they are treated

  as non-capturing. (In effect, when `WhichCaptures::Implicit` is used,

  there is no real point in using `Regex::captures` since it will never

  be able to report more information than `Regex::find`.)

  

  ```rust

  use regex_automata::{

      meta::Regex,

      nfa::thompson::WhichCaptures,

      Match,

      Span,

  };

  

  let re = Regex::builder()

      .configure(Regex::config().which_captures(WhichCaptures::Implicit))

      .build(r"foo([0-9]+)bar")?;

  let hay = "foo123bar";

  

  let mut caps = re.create_captures();

  re.captures(hay, &mut caps);

  assert_eq!(Some(Span::from(0..9)), caps.get_group(0));

  assert_eq!(None, caps.get_group(1));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  # Example: strange `Regex::find` behavior

  

  As noted above, when using [`WhichCaptures::None`](../../index.md), this means that

  `Regex::is_match` could return `true` while `Regex::find` returns

  `None`:

  

  ```rust

  use regex_automata::{

      meta::Regex,

      nfa::thompson::WhichCaptures,

      Input,

      Match,

      Span,

  };

  

  let re = Regex::builder()

      .configure(Regex::config().which_captures(WhichCaptures::None))

      .build(r"foo([0-9]+)bar")?;

  let hay = "foo123bar";

  

  assert!(re.is_match(hay));

  assert_eq!(re.find(hay), None);

  assert_eq!(re.search_half(&Input::new(hay)), None);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-nfa-size-limit"></span>`fn nfa_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](#config)

  Sets the size limit, in bytes, to enforce on the construction of every

  NFA build by the meta regex engine.

  

  Setting it to `None` disables the limit. This is not recommended if

  you're compiling untrusted patterns.

  

  Note that this limit is applied to _each_ NFA built, and if any of

  them exceed the limit, then construction will fail. This limit does

  _not_ correspond to the total memory used by all NFAs in the meta regex

  engine.

  

  This defaults to some reasonable number that permits most reasonable

  patterns.

  

  # Example

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::meta::Regex;

  

  let result = Regex::builder()

      .configure(Regex::config().nfa_size_limit(Some(20 * (1<<10))))

      // Not even 20KB is enough to build a single large Unicode class!

      .build(r"\pL");

  assert!(result.is_err());

  

  // But notice that building such a regex with the exact same limit

  // can succeed depending on other aspects of the configuration. For

  // example, a single *forward* NFA will (at time of writing) fit into

  // the 20KB limit, but a *reverse* NFA of the same pattern will not.

  // So if one configures a meta regex such that a reverse NFA is never

  // needed and thus never built, then the 20KB limit will be enough for

  // a pattern like \pL!

  let result = Regex::builder()

      .configure(Regex::config()

          .nfa_size_limit(Some(20 * (1<<10)))

          // The DFAs are the only thing that (currently) need a reverse

          // NFA. So if both are disabled, the meta regex engine will

          // skip building the reverse NFA. Note that this isn't an API

          // guarantee. A future semver compatible version may introduce

          // new use cases for a reverse NFA.

          .hybrid(false)

          .dfa(false)

      )

      // Not even 20KB is enough to build a single large Unicode class!

      .build(r"\pL");

  assert!(result.is_ok());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-onepass-size-limit"></span>`fn onepass_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](#config)

  Sets the size limit, in bytes, for the one-pass DFA.

  

  Setting it to `None` disables the limit. Disabling the limit is

  strongly discouraged when compiling untrusted patterns. Even if the

  patterns are trusted, it still may not be a good idea, since a one-pass

  DFA can use a lot of memory. With that said, as the size of a regex

  increases, the likelihood of it being one-pass likely decreases.

  

  This defaults to some reasonable number that permits most reasonable

  one-pass patterns.

  

  # Example

  

  This shows how to set the one-pass DFA size limit. Note that since

  a one-pass DFA is an optional component of the meta regex engine,

  this size limit only impacts what is built internally and will never

  determine whether a `Regex` itself fails to build.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::meta::Regex;

  

  let result = Regex::builder()

      .configure(Regex::config().onepass_size_limit(Some(2 * (1<<20))))

      .build(r"\pL{5}");

  assert!(result.is_ok());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-hybrid-cache-capacity"></span>`fn hybrid_cache_capacity(self, limit: usize) -> Config` — [`Config`](#config)

  Set the cache capacity, in bytes, for the lazy DFA.

  

  The cache capacity of the lazy DFA determines approximately how much

  heap memory it is allowed to use to store its state transitions. The

  state transitions are computed at search time, and if the cache fills

  up it, it is cleared. At this point, any previously generated state

  transitions are lost and are re-generated if they're needed again.

  

  This sort of cache filling and clearing works quite well _so long as

  cache clearing happens infrequently_. If it happens too often, then the

  meta regex engine will stop using the lazy DFA and switch over to a

  different regex engine.

  

  In cases where the cache is cleared too often, it may be possible to

  give the cache more space and reduce (or eliminate) how often it is

  cleared. Similarly, sometimes a regex is so big that the lazy DFA isn't

  used at all if its cache capacity isn't big enough.

  

  The capacity set here is a _limit_ on how much memory is used. The

  actual memory used is only allocated as it's needed.

  

  Determining the right value for this is a little tricky and will likely

  required some profiling. Enabling the `logging` feature and setting the

  log level to `trace` will also tell you how often the cache is being

  cleared.

  

  # Example

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::meta::Regex;

  

  let result = Regex::builder()

      .configure(Regex::config().hybrid_cache_capacity(20 * (1<<20)))

      .build(r"\pL{5}");

  assert!(result.is_ok());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-dfa-size-limit"></span>`fn dfa_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](#config)

  Sets the size limit, in bytes, for heap memory used for a fully

  compiled DFA.

  

  **NOTE:** If you increase this, you'll likely also need to increase

  `Config::dfa_state_limit`.

  

  In contrast to the lazy DFA, building a full DFA requires computing

  all of its state transitions up front. This can be a very expensive

  process, and runs in worst case `2^n` time and space (where `n` is

  proportional to the size of the regex). However, a full DFA unlocks

  some additional optimization opportunities.

  

  Because full DFAs can be so expensive, the default limits for them are

  incredibly small. Generally speaking, if your regex is moderately big

  or if you're using Unicode features (`\w` is Unicode-aware by default

  for example), then you can expect that the meta regex engine won't even

  attempt to build a DFA for it.

  

  If this and `Config::dfa_state_limit` are set to `None`, then the

  meta regex will not use any sort of limits when deciding whether to

  build a DFA. This in turn makes construction of a `Regex` take

  worst case exponential time and space. Even short patterns can result

  in huge space blow ups. So it is strongly recommended to keep some kind

  of limit set!

  

  The default is set to a small number that permits some simple regexes

  to get compiled into DFAs in reasonable time.

  

  # Example

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::meta::Regex;

  

  let result = Regex::builder()

      // 100MB is much bigger than the default.

      .configure(Regex::config()

          .dfa_size_limit(Some(100 * (1<<20)))

          // We don't care about size too much here, so just

          // remove the NFA state limit altogether.

          .dfa_state_limit(None))

      .build(r"\pL{5}");

  assert!(result.is_ok());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-dfa-state-limit"></span>`fn dfa_state_limit(self, limit: Option<usize>) -> Config` — [`Config`](#config)

  Sets a limit on the total number of NFA states, beyond which, a full

  DFA is not attempted to be compiled.

  

  This limit works in concert with `Config::dfa_size_limit`. Namely,

  where as `Config::dfa_size_limit` is applied by attempting to construct

  a DFA, this limit is used to avoid the attempt in the first place. This

  is useful to avoid hefty initialization costs associated with building

  a DFA for cases where it is obvious the DFA will ultimately be too big.

  

  By default, this is set to a very small number.

  

  # Example

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::meta::Regex;

  

  let result = Regex::builder()

      .configure(Regex::config()

          // Sometimes the default state limit rejects DFAs even

          // if they would fit in the size limit. Here, we disable

          // the check on the number of NFA states and just rely on

          // the size limit.

          .dfa_state_limit(None))

      .build(r"(?-u)\w{30}");

  assert!(result.is_ok());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-byte-classes"></span>`fn byte_classes(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to attempt to shrink the size of the alphabet for the regex

  pattern or not. When enabled, the alphabet is shrunk into a set of

  equivalence classes, where every byte in the same equivalence class

  cannot discriminate between a match or non-match.

  

  **WARNING:** This is only useful for debugging DFAs. Disabling this

  does not yield any speed advantages. Indeed, disabling it can result

  in much higher memory usage. Disabling byte classes is useful for

  debugging the actual generated transitions because it lets one see the

  transitions defined on actual bytes instead of the equivalence classes.

  

  This option is enabled by default and should never be disabled unless

  one is debugging the meta regex engine's internals.

  

  # Example

  

  ```rust

  use regex_automata::{meta::Regex, Match};

  

  let re = Regex::builder()

      .configure(Regex::config().byte_classes(false))

      .build(r"[a-z]+")?;

  let hay = "!!quux!!";

  assert_eq!(Some(Match::must(0, 2..6)), re.find(hay));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-line-terminator"></span>`fn line_terminator(self, byte: u8) -> Config` — [`Config`](#config)

  Set the line terminator to be used by the `^` and `$` anchors in

  multi-line mode.

  

  This option has no effect when CRLF mode is enabled. That is,

  regardless of this setting, `(?Rm:^)` and `(?Rm:$)` will always treat

  `\r` and `\n` as line terminators (and will never match between a `\r`

  and a `\n`).

  

  By default, `\n` is the line terminator.

  

  **Warning**: This does not change the behavior of `.`. To do that,

  you'll need to configure the syntax option

  [`syntax::Config::line_terminator`](crate::util::syntax::Config::line_terminator)

  in addition to this. Otherwise, `.` will continue to match any

  character other than `\n`.

  

  # Example

  

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

- <span id="config-hybrid"></span>`fn hybrid(self, yes: bool) -> Config` — [`Config`](#config)

  Toggle whether the hybrid NFA/DFA (also known as the "lazy DFA") should

  be available for use by the meta regex engine.

  

  Enabling this does not necessarily mean that the lazy DFA will

  definitely be used. It just means that it will be _available_ for use

  if the meta regex engine thinks it will be useful.

  

  When the `hybrid` crate feature is enabled, then this is enabled by

  default. Otherwise, if the crate feature is disabled, then this is

  always disabled, regardless of its setting by the caller.

- <span id="config-dfa"></span>`fn dfa(self, yes: bool) -> Config` — [`Config`](#config)

  Toggle whether a fully compiled DFA should be available for use by the

  meta regex engine.

  

  Enabling this does not necessarily mean that a DFA will definitely be

  used. It just means that it will be _available_ for use if the meta

  regex engine thinks it will be useful.

  

  When the `dfa-build` crate feature is enabled, then this is enabled by

  default. Otherwise, if the crate feature is disabled, then this is

  always disabled, regardless of its setting by the caller.

- <span id="config-onepass"></span>`fn onepass(self, yes: bool) -> Config` — [`Config`](#config)

  Toggle whether a one-pass DFA should be available for use by the meta

  regex engine.

  

  Enabling this does not necessarily mean that a one-pass DFA will

  definitely be used. It just means that it will be _available_ for

  use if the meta regex engine thinks it will be useful. (Indeed, a

  one-pass DFA can only be used when the regex is one-pass. See the

  [`dfa::onepass`](crate::dfa::onepass) module for more details.)

  

  When the `dfa-onepass` crate feature is enabled, then this is enabled

  by default. Otherwise, if the crate feature is disabled, then this is

  always disabled, regardless of its setting by the caller.

- <span id="config-backtrack"></span>`fn backtrack(self, yes: bool) -> Config` — [`Config`](#config)

  Toggle whether a bounded backtracking regex engine should be available

  for use by the meta regex engine.

  

  Enabling this does not necessarily mean that a bounded backtracker will

  definitely be used. It just means that it will be _available_ for use

  if the meta regex engine thinks it will be useful.

  

  When the `nfa-backtrack` crate feature is enabled, then this is enabled

  by default. Otherwise, if the crate feature is disabled, then this is

  always disabled, regardless of its setting by the caller.

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../index.md#matchkind)

  Returns the match kind on this configuration, as set by

  `Config::match_kind`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-utf8-empty"></span>`fn get_utf8_empty(&self) -> bool`

  Returns whether empty matches must fall on valid UTF-8 boundaries, as

  set by `Config::utf8_empty`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-auto-prefilter"></span>`fn get_auto_prefilter(&self) -> bool`

  Returns whether automatic prefilters are enabled, as set by

  `Config::auto_prefilter`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md#prefilter)

  Returns a manually set prefilter, if one was set by

  `Config::prefilter`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-which-captures"></span>`fn get_which_captures(&self) -> WhichCaptures` — [`WhichCaptures`](../../nfa/thompson/compiler/index.md#whichcaptures)

  Returns the capture configuration, as set by

  `Config::which_captures`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-nfa-size-limit"></span>`fn get_nfa_size_limit(&self) -> Option<usize>`

  Returns NFA size limit, as set by `Config::nfa_size_limit`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-onepass-size-limit"></span>`fn get_onepass_size_limit(&self) -> Option<usize>`

  Returns one-pass DFA size limit, as set by

  `Config::onepass_size_limit`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-hybrid-cache-capacity"></span>`fn get_hybrid_cache_capacity(&self) -> usize`

  Returns hybrid NFA/DFA cache capacity, as set by

  `Config::hybrid_cache_capacity`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-dfa-size-limit"></span>`fn get_dfa_size_limit(&self) -> Option<usize>`

  Returns DFA size limit, as set by `Config::dfa_size_limit`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-dfa-state-limit"></span>`fn get_dfa_state_limit(&self) -> Option<usize>`

  Returns DFA size limit in terms of the number of states in the NFA, as

  set by `Config::dfa_state_limit`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-byte-classes"></span>`fn get_byte_classes(&self) -> bool`

  Returns whether byte classes are enabled, as set by

  `Config::byte_classes`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-line-terminator"></span>`fn get_line_terminator(&self) -> u8`

  Returns the line terminator for this configuration, as set by

  `Config::line_terminator`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-hybrid"></span>`fn get_hybrid(&self) -> bool`

  Returns whether the hybrid NFA/DFA regex engine may be used, as set by

  `Config::hybrid`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-dfa"></span>`fn get_dfa(&self) -> bool`

  Returns whether the DFA regex engine may be used, as set by

  `Config::dfa`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-onepass"></span>`fn get_onepass(&self) -> bool`

  Returns whether the one-pass DFA regex engine may be used, as set by

  `Config::onepass`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-get-backtrack"></span>`fn get_backtrack(&self) -> bool`

  Returns whether the bounded backtracking regex engine may be used, as

  set by `Config::backtrack`.

  

  If it was not explicitly set, then a default value is returned.

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](#config)

  Overwrite the default configuration such that the options in `o` are

  always used. If an option in `o` is not set, then the corresponding

  option in `self` is used. If it's not set in `self` either, then it

  remains not set.

#### Trait Implementations

##### `impl Any for Config`

- <span id="config-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Config`

- <span id="config-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Config`

- <span id="config-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl CloneToUninit for Config`

- <span id="config-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Config`

- <span id="config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

##### `impl<T> From for Config`

- <span id="config-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Config`

- <span id="config-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Config`

- <span id="config-toowned-type-owned"></span>`type Owned = T`

- <span id="config-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="config-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Config`

- <span id="config-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="config-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Config`

- <span id="config-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="config-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Builder`

```rust
struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:3380-3384`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L3380-L3384)*

A builder for configuring and constructing a `Regex`.

The builder permits configuring two different aspects of a `Regex`:

* `Builder::configure` will set high-level configuration options as
described by a [`Config`](#config).
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

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Creates a new builder for configuring and constructing a [`Regex`](#regex).

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

  Builds a `Regex` from a single pattern string.

  

  If there was a problem parsing the pattern or a problem turning it into

  a regex matcher, then an error is returned.

  

  # Example

  

  This example shows how to configure syntax options.

  

  ```rust

  use regex_automata::{meta::Regex, util::syntax, Match};

  

  let re = Regex::builder()

      .syntax(syntax::Config::new().crlf(true).multi_line(true))

      .build(r"^foo$")?;

  let hay = "\r\nfoo\r\n";

  assert_eq!(Some(Match::must(0, 2..5)), re.find(hay));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

  Builds a `Regex` from many pattern strings.

  

  If there was a problem parsing any of the patterns or a problem turning

  them into a regex matcher, then an error is returned.

  

  # Example: finding the pattern that caused an error

  

  When a syntax error occurs, it is possible to ask which pattern

  caused the syntax error.

  

  ```rust

  use regex_automata::{meta::Regex, PatternID};

  

  let err = Regex::builder()

      .build_many(&["a", "b", r"\p{Foo}", "c"])

      .unwrap_err();

  assert_eq!(Some(PatternID::must(2)), err.pattern());

  ```

  

  # Example: zero patterns is valid

  

  Building a regex with zero patterns results in a regex that never

  matches anything. Because this routine is generic, passing an empty

  slice usually requires a turbo-fish (or something else to help type

  inference).

  

  ```rust

  use regex_automata::{meta::Regex, util::syntax, Match};

  

  let re = Regex::builder()

      .build_many::<&str>(&[])?;

  assert_eq!(None, re.find(""));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-build-from-hir"></span>`fn build_from_hir(&self, hir: &Hir) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

  Builds a `Regex` directly from an `Hir` expression.

  

  This is useful if you needed to parse a pattern string into an `Hir`

  for other reasons (such as analysis or transformations). This routine

  permits building a `Regex` directly from the `Hir` expression instead

  of first converting the `Hir` back to a pattern string.

  

  When using this method, any options set via `Builder::syntax` are

  ignored. Namely, the syntax options only apply when parsing a pattern

  string, which isn't relevant here.

  

  If there was a problem building the underlying regex matcher for the

  given `Hir`, then an error is returned.

  

  # Example

  

  This example shows how one can hand-construct an `Hir` expression and

  build a regex from it without doing any parsing at all.

  

  ```rust

  use {

      regex_automata::{meta::Regex, Match},

      regex_syntax::hir::{Hir, Look},

  };

  

  // (?Rm)^foo$

  let hir = Hir::concat(vec![

      Hir::look(Look::StartCRLF),

      Hir::literal("foo".as_bytes()),

      Hir::look(Look::EndCRLF),

  ]);

  let re = Regex::builder()

      .build_from_hir(&hir)?;

  let hay = "\r\nfoo\r\n";

  assert_eq!(Some(Match::must(0, 2..5)), re.find(hay));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-build-many-from-hir"></span>`fn build_many_from_hir<H: Borrow<Hir>>(&self, hirs: &[H]) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

  Builds a `Regex` directly from many `Hir` expressions.

  

  This is useful if you needed to parse pattern strings into `Hir`

  expressions for other reasons (such as analysis or transformations).

  This routine permits building a `Regex` directly from the `Hir`

  expressions instead of first converting the `Hir` expressions back to

  pattern strings.

  

  When using this method, any options set via `Builder::syntax` are

  ignored. Namely, the syntax options only apply when parsing a pattern

  string, which isn't relevant here.

  

  If there was a problem building the underlying regex matcher for the

  given `Hir` expressions, then an error is returned.

  

  Note that unlike `Builder::build_many`, this can only fail as a

  result of building the underlying matcher. In that case, there is

  no single `Hir` expression that can be isolated as a reason for the

  failure. So if this routine fails, it's not possible to determine which

  `Hir` expression caused the failure.

  

  # Example

  

  This example shows how one can hand-construct multiple `Hir`

  expressions and build a single regex from them without doing any

  parsing at all.

  

  ```rust

  use {

      regex_automata::{meta::Regex, Match},

      regex_syntax::hir::{Hir, Look},

  };

  

  // (?Rm)^foo$

  let hir1 = Hir::concat(vec![

      Hir::look(Look::StartCRLF),

      Hir::literal("foo".as_bytes()),

      Hir::look(Look::EndCRLF),

  ]);

  // (?Rm)^bar$

  let hir2 = Hir::concat(vec![

      Hir::look(Look::StartCRLF),

      Hir::literal("bar".as_bytes()),

      Hir::look(Look::EndCRLF),

  ]);

  let re = Regex::builder()

      .build_many_from_hir(&[&hir1, &hir2])?;

  let hay = "\r\nfoo\r\nbar";

  let got: Vec<Match> = re.find_iter(hay).collect();

  let expected = vec![

      Match::must(0, 2..5),

      Match::must(1, 7..10),

  ];

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

  Configure the behavior of a `Regex`.

  

  This configuration controls non-syntax options related to the behavior

  of a `Regex`. This includes things like whether empty matches can split

  a codepoint, prefilters, line terminators and a long list of options

  for configuring which regex engines the meta regex engine will be able

  to use internally.

  

  # Example

  

  This example shows how to disable UTF-8 empty mode. This will permit

  empty matches to occur between the UTF-8 encoding of a codepoint.

  

  ```rust

  use regex_automata::{meta::Regex, Match};

  

  let re = Regex::new("")?;

  let got: Vec<Match> = re.find_iter("☃").collect();

  // Matches only occur at the beginning and end of the snowman.

  assert_eq!(got, vec![

      Match::must(0, 0..0),

      Match::must(0, 3..3),

  ]);

  

  let re = Regex::builder()

      .configure(Regex::config().utf8_empty(false))

      .build("")?;

  let got: Vec<Match> = re.find_iter("☃").collect();

  // Matches now occur at every position!

  assert_eq!(got, vec![

      Match::must(0, 0..0),

      Match::must(0, 1..1),

      Match::must(0, 2..2),

      Match::must(0, 3..3),

  ]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md#config), [`Builder`](#builder)

  Configure the syntax options when parsing a pattern string while

  building a `Regex`.

  

  These options _only_ apply when `Builder::build` or `Builder::build_many`

  are used. The other build methods accept `Hir` values, which have

  already been parsed.

  

  # Example

  

  This example shows how to enable case insensitive mode.

  

  ```rust

  use regex_automata::{meta::Regex, util::syntax, Match};

  

  let re = Regex::builder()

      .syntax(syntax::Config::new().case_insensitive(true))

      .build(r"δ")?;

  assert_eq!(Some(Match::must(0, 0..2)), re.find(r"Δ"));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

#### Trait Implementations

##### `impl Any for Builder`

- <span id="builder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Builder`

- <span id="builder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Builder`

- <span id="builder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl CloneToUninit for Builder`

- <span id="builder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Builder`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Builder`

- <span id="builder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Builder`

- <span id="builder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Builder`

- <span id="builder-toowned-type-owned"></span>`type Owned = T`

- <span id="builder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Builder`

- <span id="builder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Builder`

- <span id="builder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `CachePool`

```rust
type CachePool = crate::util::pool::Pool<Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>;
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:32`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L32)*

A type alias for our pool of meta::Cache that fixes the type parameters to
what we use for the meta regex below.

### `CachePoolGuard<'a>`

```rust
type CachePoolGuard<'a> = crate::util::pool::PoolGuard<'a, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>;
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:35`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L35)*

Same as above, but for the guard returned by a pool.

### `CachePoolFn`

```rust
type CachePoolFn = alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>;
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:39-40`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L39-L40)*

The type of the closure we use to create new caches. We need to spell out
all of the marker traits or else we risk leaking !MARKER impls.

