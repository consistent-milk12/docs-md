# Crate `regex_automata`

This crate exposes a variety of regex engines used by the `regex` crate.
It provides a vast, sprawling and "expert" level API to each regex engine.
The regex engines provided by this crate focus heavily on finite automata
implementations and specifically guarantee worst case `O(m * n)` time
complexity for all searches. (Where `m ~ len(regex)` and `n ~ len(haystack)`.)

The primary goal of this crate is to serve as an implementation detail for the
`regex` crate. A secondary goal is to make its internals available for use by
others.

# Table of contents

* [Should I be using this crate?](#should-i-be-using-this-crate) gives some
reasons for and against using this crate.
* [Examples](#examples) provides a small selection of things you can do with
this crate.
* [Available regex engines](#available-regex-engines) provides a hyperlinked
list of all regex engines in this crate.
* [API themes](#api-themes) discusses common elements used throughout this
crate.
* [Crate features](#crate-features) documents the extensive list of Cargo
features available.

# Should I be using this crate?

If you find yourself here because you just want to use regexes, then you should
first check out whether the [`regex` crate](https://docs.rs/regex) meets
your needs. It provides a streamlined and difficult-to-misuse API for regex
searching.

If you're here because there is something specific you want to do that can't
be easily done with `regex` crate, then you are perhaps in the right place.
It's most likely that the first stop you'll want to make is to explore the
[`meta` regex APIs](meta). Namely, the `regex` crate is just a light wrapper
over a `meta::Regex`, so its API will probably be the easiest to transition
to. In contrast to the `regex` crate, the `meta::Regex` API supports more
search parameters and does multi-pattern searches. However, it isn't quite as
ergonomic.

Otherwise, the following is an inexhaustive list of reasons to use this crate:

* You want to analyze or use a [Thompson `NFA`](nfa::thompson::NFA) directly.
* You want more powerful multi-pattern search than what is provided by
`RegexSet` in the `regex` crate. All regex engines in this crate support
multi-pattern searches.
* You want to use one of the `regex` crate's internal engines directly because
of some interesting configuration that isn't possible via the `regex` crate.
For example, a [lazy DFA's configuration](hybrid::dfa::Config) exposes a
dizzying number of options for controlling its execution.
* You want to use the lower level search APIs. For example, both the [lazy
DFA](hybrid::dfa) and [fully compiled DFAs](dfa) support searching by exploring
the automaton one state at a time. This might be useful, for example, for
stream searches or searches of strings stored in non-contiguous in memory.
* You want to build a fully compiled DFA and then [use zero-copy
deserialization](dfa::dense::DFA::from_bytes) to load it into memory and use
it for searching. This use case is supported in core-only no-std/no-alloc
environments.
* You want to run [anchored searches](Input::anchored) without using the `^`
anchor in your regex pattern.
* You need to work-around contention issues with
sharing a regex across multiple threads. The
[`meta::Regex::search_with`](meta::Regex::search_with) API permits bypassing
any kind of synchronization at all by requiring the caller to provide the
mutable scratch spaced needed during a search.
* You want to build your own regex engine on top of the `regex` crate's
infrastructure.

# Examples

This section tries to identify a few interesting things you can do with this
crate and demonstrates them.

### Multi-pattern searches with capture groups

One of the more frustrating limitations of `RegexSet` in the `regex` crate
(at the time of writing) is that it doesn't report match positions. With this
crate, multi-pattern support was intentionally designed in from the beginning,
which means it works in all regex engines and even for capture groups as well.

This example shows how to search for matches of multiple regexes, where each
regex uses the same capture group names to parse different key-value formats.

```rust
use regex_automata::{meta::Regex, PatternID};

let re = Regex::new_many(&[
    r#"(?m)^(?<key>[[:word:]]+)=(?<val>[[:word:]]+)$"#,
    r#"(?m)^(?<key>[[:word:]]+)="(?<val>[^"]+)"$"#,
    r#"(?m)^(?<key>[[:word:]]+)='(?<val>[^']+)'$"#,
    r#"(?m)^(?<key>[[:word:]]+):\s*(?<val>[[:word:]]+)$"#,
])?;
let hay = r#"
best_album="Blow Your Face Out"
best_quote='"then as it was, then again it will be"'
best_year=1973
best_simpsons_episode: HOMR
"#;
let mut kvs = vec![];
for caps in re.captures_iter(hay) {
    // N.B. One could use capture indices '1' and '2' here
    // as well. Capture indices are local to each pattern.
    // (Just like names are.)
    let key = &hay[caps.get_group_by_name("key").unwrap()];
    let val = &hay[caps.get_group_by_name("val").unwrap()];
    kvs.push((key, val));
}
assert_eq!(kvs, vec![
    ("best_album", "Blow Your Face Out"),
    ("best_quote", "\"then as it was, then again it will be\""),
    ("best_year", "1973"),
    ("best_simpsons_episode", "HOMR"),
]);

Ok::<(), Box<dyn std::error::Error>>(())
```

### Build a full DFA and walk it manually

One of the regex engines in this crate is a fully compiled DFA. It takes worst
case exponential time to build, but once built, it can be easily explored and
used for searches. Here's a simple example that uses its lower level APIs to
implement a simple anchored search by hand.

```rust
use regex_automata::{dfa::{Automaton, dense}, Input};

let dfa = dense::DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let haystack = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&Input::new(haystack))?;
// Walk all the bytes in the haystack.
for &b in haystack.as_bytes().iter() {
    state = dfa.next_state(state, b);
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(state);
assert!(dfa.is_match_state(state));

Ok::<(), Box<dyn std::error::Error>>(())
```

Or do the same with a lazy DFA that avoids exponential worst case compile time,
but requires mutable scratch space to lazily build the DFA during the search.

```rust
use regex_automata::{hybrid::dfa::DFA, Input};

let dfa = DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let mut cache = dfa.create_cache();
let hay = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&mut cache, &Input::new(hay))?;
// Walk all the bytes in the haystack.
for &b in hay.as_bytes().iter() {
    state = dfa.next_state(&mut cache, state, b)?;
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(&mut cache, state)?;
assert!(state.is_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

### Find all overlapping matches

This example shows how to build a DFA and use it to find all possible matches,
including overlapping matches. A similar example will work with a lazy DFA as
well. This also works with multiple patterns and will report all matches at the
same position where multiple patterns match.

```rust
use regex_automata::{
    dfa::{dense, Automaton, OverlappingState},
    Input, MatchKind,
};

let dfa = dense::DFA::builder()
    .configure(dense::DFA::config().match_kind(MatchKind::All))
    .build(r"(?-u)\w{3,}")?;
let input = Input::new("homer marge bart lisa maggie");
let mut state = OverlappingState::start();

let mut matches = vec![];
while let Some(hm) = {
    dfa.try_search_overlapping_fwd(&input, &mut state)?;
    state.get_match()
} {
    matches.push(hm.offset());
}
assert_eq!(matches, vec![
    3, 4, 5,        // hom, home, homer
    9, 10, 11,      // mar, marg, marge
    15, 16,         // bar, bart
    20, 21,         // lis, lisa
    25, 26, 27, 28, // mag, magg, maggi, maggie
]);

Ok::<(), Box<dyn std::error::Error>>(())
```

# Available regex engines

The following is a complete list of all regex engines provided by this crate,
along with a very brief description of it and why you might want to use it.

* `dfa::regex::Regex` is a regex engine that works on top of either
[dense](dfa::dense) or [sparse](dfa::sparse) fully compiled DFAs. You might
use a DFA if you need the fastest possible regex engine in this crate and can
afford the exorbitant memory usage usually required by DFAs. Low level APIs on
fully compiled DFAs are provided by the [`Automaton` trait](dfa::Automaton).
Fully compiled dense DFAs can handle all regexes except for searching a regex
with a Unicode word boundary on non-ASCII haystacks. A fully compiled DFA based
regex can only report the start and end of each match.
* `hybrid::regex::Regex` is a regex engine that works on top of a lazily
built DFA. Its performance profile is very similar to that of fully compiled
DFAs, but can be slower in some pathological cases. Fully compiled DFAs are
also amenable to more optimizations, such as state acceleration, that aren't
available in a lazy DFA. You might use this lazy DFA if you can't abide the
worst case exponential compile time of a full DFA, but still want the DFA
search performance in the vast majority of cases. A lazy DFA based regex can
only report the start and end of each match.
* `dfa::onepass::DFA` is a regex engine that is implemented as a DFA, but
can report the matches of each capture group in addition to the start and end
of each match. The catch is that it only works on a somewhat small subset of
regexes known as "one-pass." You'll want to use this for cases when you need
capture group matches and the regex is one-pass since it is likely to be faster
than any alternative. A one-pass DFA can handle all types of regexes, but does
have some reasonable limits on the number of capture groups it can handle.
* `nfa::thompson::backtrack::BoundedBacktracker` is a regex engine that uses
backtracking, but keeps track of the work it has done to avoid catastrophic
backtracking. Like the one-pass DFA, it provides the matches of each capture
group. It retains the `O(m * n)` worst case time bound. This tends to be slower
than the one-pass DFA regex engine, but faster than the PikeVM. It can handle
all types of regexes, but usually only works well with small haystacks and
small regexes due to the memory required to avoid redoing work.
* `nfa::thompson::pikevm::PikeVM` is a regex engine that can handle all
regexes, of all sizes and provides capture group matches. It tends to be a tool
of last resort because it is also usually the slowest regex engine.
* `meta::Regex` is the meta regex engine that combines *all* of the above
engines into one. The reason for this is that each of the engines above have
their own caveats such as, "only handles a subset of regexes" or "is generally
slow." The meta regex engine accounts for all of these caveats and composes
the engines in a way that attempts to mitigate each engine's weaknesses while
emphasizing its strengths. For example, it will attempt to run a lazy DFA even
if it might fail. In which case, it will restart the search with a likely
slower but more capable regex engine. The meta regex engine is what you should
default to. Use one of the above engines directly only if you have a specific
reason to.

# API themes

While each regex engine has its own APIs and configuration options, there are
some general themes followed by all of them.

### The `Input` abstraction

Most search routines in this crate accept anything that implements
`Into<Input>`. Both `&str` and `&[u8]` haystacks satisfy this constraint, which
means that things like `engine.search("foo")` will work as you would expect.

By virtue of accepting an `Into<Input>` though, callers can provide more than
just a haystack. Indeed, the [`Input`](#input) type has more details, but briefly,
callers can use it to configure various aspects of the search:

* The span of the haystack to search via `Input::span` or `Input::range`,
which might be a substring of the haystack.
* Whether to run an anchored search or not via `Input::anchored`. This
permits one to require matches to start at the same offset that the search
started.
* Whether to ask the regex engine to stop as soon as a match is seen via
`Input::earliest`. This can be used to find the offset of a match as soon
as it is known without waiting for the full leftmost-first match to be found.
This can also be used to avoid the worst case `O(m * n^2)` time complexity
of iteration.

Some lower level search routines accept an `&Input` for performance reasons.
In which case, `&Input::new("haystack")` can be used for a simple search.

### Error reporting

Most, but not all, regex engines in this crate can fail to execute a search.
When a search fails, callers cannot determine whether or not a match exists.
That is, the result is indeterminate.

Search failure, in all cases in this crate, is represented by a [`MatchError`](#matcherror).
Routines that can fail start with the `try_` prefix in their name. For example,
`hybrid::regex::Regex::try_search` can fail for a number of reasons.
Conversely, routines that either can't fail or can panic on failure lack the
`try_` prefix. For example, `hybrid::regex::Regex::find` will panic in
cases where `hybrid::regex::Regex::try_search` would return an error, and
`meta::Regex::find` will never panic. Therefore, callers need to pay close
attention to the panicking conditions in the documentation.

In most cases, the reasons that a search fails are either predictable or
configurable, albeit at some additional cost.

An example of predictable failure is
[`BoundedBacktracker::try_search`](nfa::thompson::backtrack::BoundedBacktracker::try_search).
Namely, it fails whenever the multiplication of the haystack, the regex and some
constant exceeds the
[configured visited capacity](nfa::thompson::backtrack::Config::visited_capacity).
Callers can predict the failure in terms of haystack length via the
[`BoundedBacktracker::max_haystack_len`](nfa::thompson::backtrack::BoundedBacktracker::max_haystack_len)
method. While this form of failure is technically avoidable by increasing the
visited capacity, it isn't practical to do so for all inputs because the
memory usage required for larger haystacks becomes impractically large. So in
practice, if one is using the bounded backtracker, you really do have to deal
with the failure.

An example of configurable failure happens when one enables heuristic support
for Unicode word boundaries in a DFA. Namely, since the DFAs in this crate
(except for the one-pass DFA) do not support Unicode word boundaries on
non-ASCII haystacks, building a DFA from an NFA that contains a Unicode word
boundary will itself fail. However, one can configure DFAs to still be built in
this case by
[configuring heuristic support for Unicode word boundaries](hybrid::dfa::Config::unicode_word_boundary).
If the NFA the DFA is built from contains a Unicode word boundary, then the
DFA will still be built, but special transitions will be added to every state
that cause the DFA to fail if any non-ASCII byte is seen. This failure happens
at search time and it requires the caller to opt into this.

There are other ways for regex engines to fail in this crate, but the above
two should represent the general theme of failures one can find. Dealing
with these failures is, in part, one the responsibilities of the [meta regex
engine](meta). Notice, for example, that the meta regex engine exposes an API
that never returns an error nor panics. It carefully manages all of the ways
in which the regex engines can fail and either avoids the predictable ones
entirely (e.g., the bounded backtracker) or reacts to configured failures by
falling back to a different engine (e.g., the lazy DFA quitting because it saw
a non-ASCII byte).

### Configuration and Builders

Most of the regex engines in this crate come with two types to facilitate
building the regex engine: a `Config` and a `Builder`. A `Config` is usually
specific to that particular regex engine, but other objects such as parsing and
NFA compilation have `Config` types too. A `Builder` is the thing responsible
for taking inputs (either pattern strings or already-parsed patterns or even
NFAs directly) and turning them into an actual regex engine that can be used
for searching.

The main reason why building a regex engine is a bit complicated is because
of the desire to permit composition with de-coupled components. For example,
you might want to [manually construct a Thompson NFA](nfa::thompson::Builder)
and then build a regex engine from it without ever using a regex parser
at all. On the other hand, you might also want to build a regex engine directly
from the concrete syntax. This demonstrates why regex engine construction is
so flexible: it needs to support not just convenient construction, but also
construction from parts built elsewhere.

This is also in turn why there are many different `Config` structs in this
crate. Let's look more closely at an example: `hybrid::regex::Builder`. It
accepts three different `Config` types for configuring construction of a lazy
DFA regex:

* `hybrid::regex::Builder::syntax` accepts a
`util::syntax::Config` for configuring the options found in the
[`regex-syntax`](regex_syntax) crate. For example, whether to match
case insensitively.
* `hybrid::regex::Builder::thompson` accepts a `nfa::thompson::Config` for
configuring construction of a [Thompson NFA](nfa::thompson::NFA). For example,
whether to build an NFA that matches the reverse language described by the
regex.
* `hybrid::regex::Builder::dfa` accept a [`hybrid::dfa::Config`](hybrid/dfa/index.md) for
configuring construction of the pair of underlying lazy DFAs that make up the
lazy DFA regex engine. For example, changing the capacity of the cache used to
store the transition table.

The lazy DFA regex engine uses all three of those configuration objects for
methods like `hybrid::regex::Builder::build`, which accepts a pattern
string containing the concrete syntax of your regex. It uses the syntax
configuration to parse it into an AST and translate it into an HIR. Then the
NFA configuration when compiling the HIR into an NFA. And then finally the DFA
configuration when lazily determinizing the NFA into a DFA.

Notice though that the builder also has a
`hybrid::regex::Builder::build_from_dfas` constructor. This permits callers
to build the underlying pair of lazy DFAs themselves (one for the forward
searching to find the end of a match and one for the reverse searching to find
the start of a match), and then build the regex engine from them. The lazy
DFAs, in turn, have their own builder that permits [construction directly from
a Thompson NFA](hybrid::dfa::Builder::build_from_nfa). Continuing down the
rabbit hole, a Thompson NFA has its own compiler that permits [construction
directly from an HIR](nfa::thompson::Compiler::build_from_hir). The lazy DFA
regex engine builder lets you follow this rabbit hole all the way down, but
also provides convenience routines that do it for you when you don't need
precise control over every component.

The [meta regex engine](meta) is a good example of something that utilizes the
full flexibility of these builders. It often needs not only precise control
over each component, but also shares them across multiple regex engines.
(Most sharing is done by internal reference accounting. For example, an
[`NFA`](nfa::thompson::NFA) is reference counted internally which makes cloning
cheap.)

### Size limits

Unlike the `regex` crate, the `regex-automata` crate specifically does not
enable any size limits by default. That means users of this crate need to
be quite careful when using untrusted patterns. Namely, because bounded
repetitions can grow exponentially by stacking them, it is possible to build a
very large internal regex object from just a small pattern string. For example,
the NFA built from the pattern `a{10}{10}{10}{10}{10}{10}{10}` is over 240MB.

There are multiple size limit options in this crate. If one or more size limits
are relevant for the object you're building, they will be configurable via
methods on a corresponding `Config` type.

# Crate features

This crate has a dizzying number of features. The main idea is to be able to
control how much stuff you pull in for your specific use case, since the full
crate is quite large and can dramatically increase compile times and binary
size.

The most barebones but useful configuration is to disable all default features
and enable only `dfa-search`. This will bring in just the DFA deserialization
and search routines without any dependency on `std` or `alloc`. This does
require generating and serializing a DFA, and then storing it somewhere, but
it permits regex searches in freestanding or embedded environments.

Because there are so many features, they are split into a few groups.

The default set of features is: `std`, `syntax`, `perf`, `unicode`, `meta`,
`nfa`, `dfa` and `hybrid`. Basically, the default is to enable everything
except for development related features like `logging`.

### Ecosystem features

* **std** - Enables use of the standard library. In terms of APIs, this usually
just means that error types implement the `std::error::Error` trait. Otherwise,
`std` sometimes enables the code to be faster, for example, using a `HashMap`
instead of a `BTreeMap`. (The `std` feature matters more for dependencies like
`aho-corasick` and `memchr`, where `std` is required to enable certain classes
of SIMD optimizations.) Enabling `std` automatically enables `alloc`.
* **alloc** - Enables use of the `alloc` library. This is required for most
APIs in this crate. The main exception is deserializing and searching with
fully compiled DFAs.
* **logging** - Adds a dependency on the `log` crate and makes this crate emit
log messages of varying degrees of utility. The log messages are especially
useful in trying to understand what the meta regex engine is doing.

### Performance features

**Note**:
  To get performance benefits offered by the SIMD, `std` must be enabled.
  None of the `perf-*` features will enable `std` implicitly.

* **perf** - Enables all of the below features.
* **perf-inline** - When enabled, `inline(always)` is used in (many) strategic
locations to help performance at the expense of longer compile times and
increased binary size.
* **perf-literal** - Enables all literal related optimizations.
    * **perf-literal-substring** - Enables all single substring literal
    optimizations. This includes adding a dependency on the `memchr` crate.
    * **perf-literal-multisubstring** - Enables all multiple substring literal
    optimizations. This includes adding a dependency on the `aho-corasick`
    crate.

### Unicode features

* **unicode** -
  Enables all Unicode features. This feature is enabled by default, and will
  always cover all Unicode features, even if more are added in the future.
* **unicode-age** -
  Provide the data for the
  [Unicode `Age` property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
  This makes it possible to use classes like `\p{Age:6.0}` to refer to all
  codepoints first introduced in Unicode 6.0
* **unicode-bool** -
  Provide the data for numerous Unicode boolean properties. The full list
  is not included here, but contains properties like `Alphabetic`, `Emoji`,
  `Lowercase`, `Math`, `Uppercase` and `White_Space`.
* **unicode-case** -
  Provide the data for case insensitive matching using
  [Unicode's "simple loose matches" specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
* **unicode-gencat** -
  Provide the data for
  [Unicode general categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
  This includes, but is not limited to, `Decimal_Number`, `Letter`,
  `Math_Symbol`, `Number` and `Punctuation`.
* **unicode-perl** -
  Provide the data for supporting the Unicode-aware Perl character classes,
  corresponding to `\w`, `\s` and `\d`. This is also necessary for using
  Unicode-aware word boundary assertions. Note that if this feature is
  disabled, the `\s` and `\d` character classes are still available if the
  `unicode-bool` and `unicode-gencat` features are enabled, respectively.
* **unicode-script** -
  Provide the data for
  [Unicode scripts and script extensions](https://www.unicode.org/reports/tr24/).
  This includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
  `Latin` and `Thai`.
* **unicode-segment** -
  Provide the data necessary to provide the properties used to implement the
  [Unicode text segmentation algorithms](https://www.unicode.org/reports/tr29/).
  This enables using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
  `\p{sb=ATerm}`.
* **unicode-word-boundary** -
  Enables support for Unicode word boundaries, i.e., `\b`, in regexes. When
  this and `unicode-perl` are enabled, then data tables from `regex-syntax` are
  used to implement Unicode word boundaries. However, if `regex-syntax` isn't
  enabled as a dependency then one can still enable this feature. It will
  cause `regex-automata` to bundle its own data table that would otherwise be
  redundant with `regex-syntax`'s table.

### Regex engine features

* **syntax** - Enables a dependency on `regex-syntax`. This makes APIs
for building regex engines from pattern strings available. Without the
`regex-syntax` dependency, the only way to build a regex engine is generally
to deserialize a previously built DFA or to hand assemble an NFA using its
[builder API](nfa::thompson::Builder). Once you have an NFA, you can build any
of the regex engines in this crate. The `syntax` feature also enables `alloc`.
* **meta** - Enables the meta regex engine. This also enables the `syntax` and
`nfa-pikevm` features, as both are the minimal requirements needed. The meta
regex engine benefits from enabling any of the other regex engines and will
use them automatically when appropriate.
* **nfa** - Enables all NFA related features below.
    * **nfa-thompson** - Enables the Thompson NFA APIs. This enables `alloc`.
    * **nfa-pikevm** - Enables the PikeVM regex engine. This enables
    `nfa-thompson`.
    * **nfa-backtrack** - Enables the bounded backtracker regex engine. This
    enables `nfa-thompson`.
* **dfa** - Enables all DFA related features below.
    * **dfa-build** - Enables APIs for determinizing DFAs from NFAs. This
    enables `nfa-thompson` and `dfa-search`.
    * **dfa-search** - Enables APIs for searching with DFAs.
    * **dfa-onepass** - Enables the one-pass DFA API. This enables
    `nfa-thompson`.
* **hybrid** - Enables the hybrid NFA/DFA or "lazy DFA" regex engine. This
enables `alloc` and `nfa-thompson`.

## Modules

- [`dfa`](dfa/index.md) - A module for building and searching with deterministic finite automata (DFAs).
- [`hybrid`](hybrid/index.md) - A module for building and searching with lazy deterministic finite automata
- [`meta`](meta/index.md) - Provides a regex matcher that composes several other regex matchers
- [`nfa`](nfa/index.md) - Provides non-deterministic finite automata (NFA) and regex engines that use
- [`util`](util/index.md) - A collection of modules that provide APIs that are useful across many regex

## Structs

### `PatternID`

```rust
struct PatternID(SmallIndex);
```

The identifier of a regex pattern, represented by a [`SmallIndex`](util/primitives/index.md).

The identifier for a pattern corresponds to its relative position among
other patterns in a single finite state machine. Namely, when building
a multi-pattern regex engine, one must supply a sequence of patterns to
match. The position (starting at 0) of each pattern in that sequence
represents its identifier. This identifier is in turn used to identify and
report matches of that pattern in various APIs.

See the [`SmallIndex`](util/primitives/index.md) type for more information about what it means for
a pattern ID to be a "small index."

Note that this type is defined in the
[`util::primitives`](crate::util::primitives) module, but it is also
re-exported at the crate root due to how common it is.

#### Implementations

- `const MAX: PatternID`

- `const LIMIT: usize`

- `const ZERO: PatternID`

- `const SIZE: usize`

- `fn new(value: usize) -> Result<PatternID, PatternIDError>` — [`PatternID`](util/primitives/index.md), [`PatternIDError`](util/primitives/index.md)

- `const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](util/primitives/index.md)

- `fn must(value: usize) -> PatternID` — [`PatternID`](util/primitives/index.md)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](util/primitives/index.md), [`PatternIDError`](util/primitives/index.md)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](util/primitives/index.md)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

- `fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for PatternID`

- `fn clone(self: &Self) -> PatternID` — [`PatternID`](util/primitives/index.md)

##### `impl Copy for PatternID`

##### `impl Debug for PatternID`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for PatternID`

- `fn default() -> PatternID` — [`PatternID`](util/primitives/index.md)

##### `impl Eq for PatternID`

##### `impl Hash for PatternID`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for PatternID`

- `fn cmp(self: &Self, other: &PatternID) -> $crate::cmp::Ordering` — [`PatternID`](util/primitives/index.md)

##### `impl PartialEq for PatternID`

- `fn eq(self: &Self, other: &PatternID) -> bool` — [`PatternID`](util/primitives/index.md)

##### `impl PartialOrd for PatternID`

- `fn partial_cmp(self: &Self, other: &PatternID) -> $crate::option::Option<$crate::cmp::Ordering>` — [`PatternID`](util/primitives/index.md)

##### `impl StructuralPartialEq for PatternID`

### `Input<'h>`

```rust
struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
```

The parameters for a regex search including the haystack to search.

It turns out that regex searches have a few parameters, and in most cases,
those parameters have defaults that work in the vast majority of cases.
This `Input` type exists to make that common case seamless while also
providing an avenue for changing the parameters of a search. In particular,
this type enables doing so without a combinatorial explosion of different
methods and/or superfluous parameters in the common cases.

An `Input` permits configuring the following things:

* Search only a substring of a haystack, while taking the broader context
into account for resolving look-around assertions.
* Indicating whether to search for all patterns in a regex, or to
only search for one pattern in particular.
* Whether to perform an anchored on unanchored search.
* Whether to report a match as early as possible.

All of these parameters, except for the haystack, have sensible default
values. This means that the minimal search configuration is simply a call
to `Input::new` with your haystack. Setting any other parameter is
optional.

Moreover, for any `H` that implements `AsRef<[u8]>`, there exists a
`From<H> for Input` implementation. This is useful because many of the
search APIs in this crate accept an `Into<Input>`. This means you can
provide string or byte strings to these routines directly, and they'll
automatically get converted into an `Input` for you.

The lifetime parameter `'h` refers to the lifetime of the haystack.

# Organization

The API of `Input` is split into a few different parts:

* A builder-like API that transforms a `Input` by value. Examples:
`Input::span` and `Input::anchored`.
* A setter API that permits mutating parameters in place. Examples:
`Input::set_span` and `Input::set_anchored`.
* A getter API that permits retrieving any of the search parameters.
Examples: `Input::get_span` and `Input::get_anchored`.
* A few convenience getter routines that don't conform to the above naming
pattern due to how common they are. Examples: `Input::haystack`,
`Input::start` and `Input::end`.
* Miscellaneous predicates and other helper routines that are useful
in some contexts. Examples: `Input::is_char_boundary`.

A `Input` exposes so much because it is meant to be used by both callers of
regex engines _and_ implementors of regex engines. A constraining factor is
that regex engines should accept a `&Input` as its lowest level API, which
means that implementors should only use the "getter" APIs of a `Input`.

# Valid bounds and search termination

An `Input` permits setting the bounds of a search via either
`Input::span` or `Input::range`. The bounds set must be valid, or
else a panic will occur. Bounds are valid if and only if:

* The bounds represent a valid range into the input's haystack.
* **or** the end bound is a valid ending bound for the haystack *and*
the start bound is exactly one greater than the start bound.

In the latter case, `Input::is_done` will return true and indicates any
search receiving such an input should immediately return with no match.

Note that while `Input` is used for reverse searches in this crate, the
`Input::is_done` predicate assumes a forward search. Because unsigned
offsets are used internally, there is no way to tell from only the offsets
whether a reverse search is done or not.

# Regex engine support

Any regex engine accepting an `Input` must support at least the following
things:

* Searching a `&[u8]` for matches.
* Searching a substring of `&[u8]` for a match, such that any match
reported must appear entirely within that substring.
* For a forwards search, a match should never be reported when
`Input::is_done` returns true. (For reverse searches, termination should
be handled outside of `Input`.)

Supporting other aspects of an `Input` are optional, but regex engines
should handle aspects they don't support gracefully. How this is done is
generally up to the regex engine. This crate generally treats unsupported
anchored modes as an error to report for example, but for simplicity, in
the meta regex engine, trying to search with an invalid pattern ID just
results in no match being reported.

#### Implementations

- `fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h>` — [`Input`](#input)

- `fn span<S: Into<Span>>(self: Self, span: S) -> Input<'h>` — [`Input`](#input)

- `fn range<R: RangeBounds<usize>>(self: Self, range: R) -> Input<'h>` — [`Input`](#input)

- `fn anchored(self: Self, mode: Anchored) -> Input<'h>` — [`Anchored`](#anchored), [`Input`](#input)

- `fn earliest(self: Self, yes: bool) -> Input<'h>` — [`Input`](#input)

- `fn set_span<S: Into<Span>>(self: &mut Self, span: S)`

- `fn set_range<R: RangeBounds<usize>>(self: &mut Self, range: R)`

- `fn set_start(self: &mut Self, start: usize)`

- `fn set_end(self: &mut Self, end: usize)`

- `fn set_anchored(self: &mut Self, mode: Anchored)` — [`Anchored`](#anchored)

- `fn set_earliest(self: &mut Self, yes: bool)`

- `fn haystack(self: &Self) -> &'h [u8]`

- `fn start(self: &Self) -> usize`

- `fn end(self: &Self) -> usize`

- `fn get_span(self: &Self) -> Span` — [`Span`](#span)

- `fn get_range(self: &Self) -> Range<usize>`

- `fn get_anchored(self: &Self) -> Anchored` — [`Anchored`](#anchored)

- `fn get_earliest(self: &Self) -> bool`

- `fn is_done(self: &Self) -> bool`

- `fn is_char_boundary(self: &Self, offset: usize) -> bool`

#### Trait Implementations

##### `impl<'h> Clone for Input<'h>`

- `fn clone(self: &Self) -> Input<'h>` — [`Input`](#input)

##### `impl<'h> Debug for Input<'h>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Span`

```rust
struct Span {
    pub start: usize,
    pub end: usize,
}
```

A representation of a span reported by a regex engine.

A span corresponds to the starting and ending _byte offsets_ of a
contiguous region of bytes. The starting offset is inclusive while the
ending offset is exclusive. That is, a span is a half-open interval.

A span is used to report the offsets of a match, but it is also used to
convey which region of a haystack should be searched via routines like
`Input::span`.

This is basically equivalent to a `std::ops::Range<usize>`, except this
type implements `Copy` which makes it more ergonomic to use in the context
of this crate. Like a range, this implements `Index` for `[u8]` and `str`,
and `IndexMut` for `[u8]`. For convenience, this also impls `From<Range>`,
which means things like `Span::from(5..10)` work.

#### Fields

- **`start`**: `usize`

  The start offset of the span, inclusive.

- **`end`**: `usize`

  The end offset of the span, exclusive.

#### Implementations

- `fn range(self: &Self) -> Range<usize>`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn contains(self: &Self, offset: usize) -> bool`

- `fn offset(self: &Self, offset: usize) -> Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for Span`

- `fn clone(self: &Self) -> Span` — [`Span`](#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Span`

##### `impl Hash for Span`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Span`

- `fn eq(self: &Self, other: &Span) -> bool` — [`Span`](#span)

##### `impl StructuralPartialEq for Span`

### `HalfMatch`

```rust
struct HalfMatch {
    pattern: crate::util::primitives::PatternID,
    offset: usize,
}
```

A representation of "half" of a match reported by a DFA.

This is called a "half" match because it only includes the end location (or
start location for a reverse search) of a match. This corresponds to the
information that a single DFA scan can report. Getting the other half of
the match requires a second scan with a reversed DFA.

A half match also includes the pattern that matched. The pattern is
identified by an ID, which corresponds to its position (starting from `0`)
relative to other patterns used to construct the corresponding DFA. If only
a single pattern is provided to the DFA, then all matches are guaranteed to
have a pattern ID of `0`.

#### Fields

- **`pattern`**: `crate::util::primitives::PatternID`

  The pattern ID.

- **`offset`**: `usize`

  The offset of the match.
  
  For forward searches, the offset is exclusive. For reverse searches,
  the offset is inclusive.

#### Implementations

- `fn new(pattern: PatternID, offset: usize) -> HalfMatch` — [`PatternID`](util/primitives/index.md), [`HalfMatch`](#halfmatch)

- `fn must(pattern: usize, offset: usize) -> HalfMatch` — [`HalfMatch`](#halfmatch)

- `fn pattern(self: &Self) -> PatternID` — [`PatternID`](util/primitives/index.md)

- `fn offset(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for HalfMatch`

- `fn clone(self: &Self) -> HalfMatch` — [`HalfMatch`](#halfmatch)

##### `impl Copy for HalfMatch`

##### `impl Debug for HalfMatch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for HalfMatch`

##### `impl Hash for HalfMatch`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for HalfMatch`

- `fn eq(self: &Self, other: &HalfMatch) -> bool` — [`HalfMatch`](#halfmatch)

##### `impl StructuralPartialEq for HalfMatch`

### `Match`

```rust
struct Match {
    pattern: crate::util::primitives::PatternID,
    span: Span,
}
```

A representation of a match reported by a regex engine.

A match has two essential pieces of information: the [`PatternID`](util/primitives/index.md) that
matches, and the [`Span`](#span) of the match in a haystack.

The pattern is identified by an ID, which corresponds to its position
(starting from `0`) relative to other patterns used to construct the
corresponding regex engine. If only a single pattern is provided, then all
matches are guaranteed to have a pattern ID of `0`.

Every match reported by a regex engine guarantees that its span has its
start offset as less than or equal to its end offset.

#### Fields

- **`pattern`**: `crate::util::primitives::PatternID`

  The pattern ID.

- **`span`**: `Span`

  The underlying match span.

#### Implementations

- `fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match` — [`PatternID`](util/primitives/index.md), [`Match`](#match)

- `fn must<S: Into<Span>>(pattern: usize, span: S) -> Match` — [`Match`](#match)

- `fn pattern(self: &Self) -> PatternID` — [`PatternID`](util/primitives/index.md)

- `fn start(self: &Self) -> usize`

- `fn end(self: &Self) -> usize`

- `fn range(self: &Self) -> core::ops::Range<usize>`

- `fn span(self: &Self) -> Span` — [`Span`](#span)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for Match`

- `fn clone(self: &Self) -> Match` — [`Match`](#match)

##### `impl Copy for Match`

##### `impl Debug for Match`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Match`

##### `impl Hash for Match`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Match`

- `fn eq(self: &Self, other: &Match) -> bool` — [`Match`](#match)

##### `impl StructuralPartialEq for Match`

### `PatternSet`

```rust
struct PatternSet {
    len: usize,
    which: alloc::boxed::Box<[bool]>,
}
```

A set of `PatternID`s.

A set of pattern identifiers is useful for recording which patterns have
matched a particular haystack. A pattern set _only_ includes pattern
identifiers. It does not include offset information.

# Example

This shows basic usage of a set.

```rust
use regex_automata::{PatternID, PatternSet};

let pid1 = PatternID::must(5);
let pid2 = PatternID::must(8);
// Create a new empty set.
let mut set = PatternSet::new(10);
// Insert pattern IDs.
set.insert(pid1);
set.insert(pid2);
// Test membership.
assert!(set.contains(pid1));
assert!(set.contains(pid2));
// Get all members.
assert_eq!(
    vec![5, 8],
    set.iter().map(|p| p.as_usize()).collect::<Vec<usize>>(),
);
// Clear the set.
set.clear();
// Test that it is indeed empty.
assert!(set.is_empty());
```

#### Fields

- **`len`**: `usize`

  The number of patterns set to 'true' in this set.

- **`which`**: `alloc::boxed::Box<[bool]>`

  A map from PatternID to boolean of whether a pattern matches or not.
  
  This should probably be a bitset, but it's probably unlikely to matter
  much in practice.
  
  The main downside of this representation (and similarly for a bitset)
  is that iteration scales with the capacity of the set instead of
  the length of the set. This doesn't seem likely to be a problem in
  practice.
  
  Another alternative is to just use a 'SparseSet' for this. It does use
  more memory (quite a bit more), but that seems fine I think compared
  to the memory being used by the regex engine. The real hiccup with
  it is that it yields pattern IDs in the order they were inserted.
  Which is actually kind of nice, but at the time of writing, pattern
  IDs are yielded in ascending order in the regex crate RegexSet API.
  If we did change to 'SparseSet', we could provide an additional
  'iter_match_order' iterator, but keep the ascending order one for
  compatibility.

#### Implementations

- `fn new(capacity: usize) -> PatternSet` — [`PatternSet`](#patternset)

- `fn clear(self: &mut Self)`

- `fn contains(self: &Self, pid: PatternID) -> bool` — [`PatternID`](util/primitives/index.md)

- `fn insert(self: &mut Self, pid: PatternID) -> bool` — [`PatternID`](util/primitives/index.md)

- `fn try_insert(self: &mut Self, pid: PatternID) -> Result<bool, PatternSetInsertError>` — [`PatternID`](util/primitives/index.md), [`PatternSetInsertError`](#patternsetinserterror)

- `fn is_empty(self: &Self) -> bool`

- `fn is_full(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn capacity(self: &Self) -> usize`

- `fn iter(self: &Self) -> PatternSetIter<'_>` — [`PatternSetIter`](#patternsetiter)

#### Trait Implementations

##### `impl Clone for PatternSet`

- `fn clone(self: &Self) -> PatternSet` — [`PatternSet`](#patternset)

##### `impl Debug for PatternSet`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for PatternSet`

##### `impl PartialEq for PatternSet`

- `fn eq(self: &Self, other: &PatternSet) -> bool` — [`PatternSet`](#patternset)

##### `impl StructuralPartialEq for PatternSet`

### `PatternSetInsertError`

```rust
struct PatternSetInsertError {
    attempted: crate::util::primitives::PatternID,
    capacity: usize,
}
```

An error that occurs when a `PatternID` failed to insert into a
`PatternSet`.

An insert fails when the given `PatternID` exceeds the configured capacity
of the `PatternSet`.

This error is created by the `PatternSet::try_insert` routine.

#### Trait Implementations

##### `impl Clone for PatternSetInsertError`

- `fn clone(self: &Self) -> PatternSetInsertError` — [`PatternSetInsertError`](#patternsetinserterror)

##### `impl Debug for PatternSetInsertError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for PatternSetInsertError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for PatternSetInsertError`

##### `impl<T> ToString for PatternSetInsertError`

- `fn to_string(self: &Self) -> String`

### `PatternSetIter<'a>`

```rust
struct PatternSetIter<'a> {
    it: core::iter::Enumerate<core::slice::Iter<'a, bool>>,
}
```

An iterator over all pattern identifiers in a [`PatternSet`](#patternset).

The lifetime parameter `'a` refers to the lifetime of the pattern set being
iterated over.

This iterator is created by the `PatternSet::iter` method.

#### Trait Implementations

##### `impl<'a> Clone for PatternSetIter<'a>`

- `fn clone(self: &Self) -> PatternSetIter<'a>` — [`PatternSetIter`](#patternsetiter)

##### `impl<'a> Debug for PatternSetIter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for PatternSetIter<'a>`

- `fn next_back(self: &mut Self) -> Option<PatternID>` — [`PatternID`](util/primitives/index.md)

##### `impl<I> IntoIterator for PatternSetIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for PatternSetIter<'a>`

- `type Item = PatternID`

- `fn next(self: &mut Self) -> Option<PatternID>` — [`PatternID`](util/primitives/index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `MatchError`

```rust
struct MatchError(alloc::boxed::Box<MatchErrorKind>);
```

An error indicating that a search stopped before reporting whether a
match exists or not.

To be very clear, this error type implies that one cannot assume that no
matches occur, since the search stopped before completing. That is, if
you're looking for information about where a search determined that no
match can occur, then this error type does *not* give you that. (Indeed, at
the time of writing, if you need such a thing, you have to write your own
search routine.)

Normally, when one searches for something, the response is either an
affirmative "it was found at this location" or a negative "not found at
all." However, in some cases, a regex engine can be configured to stop its
search before concluding whether a match exists or not. When this happens,
it may be important for the caller to know why the regex engine gave up and
where in the input it gave up at. This error type exposes the 'why' and the
'where.'

For example, the DFAs provided by this library generally cannot correctly
implement Unicode word boundaries. Instead, they provide an option to
eagerly support them on ASCII text (since Unicode word boundaries are
equivalent to ASCII word boundaries when searching ASCII text), but will
"give up" if a non-ASCII byte is seen. In such cases, one is usually
required to either report the failure to the caller (unergonomic) or
otherwise fall back to some other regex engine (ergonomic, but potentially
costly).

More generally, some regex engines offer the ability for callers to specify
certain bytes that will trigger the regex engine to automatically quit if
they are seen.

Still yet, there may be other reasons for a failed match. For example,
the hybrid DFA provided by this crate can be configured to give up if it
believes that it is not efficient. This in turn permits callers to choose a
different regex engine.

(Note that DFAs are configured by default to never quit or give up in this
fashion. For example, by default, a DFA will fail to build if the regex
pattern contains a Unicode word boundary. One needs to opt into the "quit"
behavior via options, like
[`hybrid::dfa::Config::unicode_word_boundary`](crate::hybrid::dfa::Config::unicode_word_boundary).)

There are a couple other ways a search
can fail. For example, when using the
[`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)
with a haystack that is too long, or trying to run an unanchored search
with a [one-pass DFA](crate::dfa::onepass).

#### Implementations

- `fn new(kind: MatchErrorKind) -> MatchError` — [`MatchErrorKind`](#matcherrorkind), [`MatchError`](#matcherror)

- `fn kind(self: &Self) -> &MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

- `fn quit(byte: u8, offset: usize) -> MatchError` — [`MatchError`](#matcherror)

- `fn gave_up(offset: usize) -> MatchError` — [`MatchError`](#matcherror)

- `fn haystack_too_long(len: usize) -> MatchError` — [`MatchError`](#matcherror)

- `fn unsupported_anchored(mode: Anchored) -> MatchError` — [`Anchored`](#anchored), [`MatchError`](#matcherror)

#### Trait Implementations

##### `impl Clone for MatchError`

- `fn clone(self: &Self) -> MatchError` — [`MatchError`](#matcherror)

##### `impl Debug for MatchError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for MatchError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl PartialEq for MatchError`

- `fn eq(self: &Self, other: &MatchError) -> bool` — [`MatchError`](#matcherror)

##### `impl StructuralPartialEq for MatchError`

##### `impl<T> ToString for MatchError`

- `fn to_string(self: &Self) -> String`

## Enums

### `Anchored`

```rust
enum Anchored {
    No,
    Yes,
    Pattern(crate::util::primitives::PatternID),
}
```

The type of anchored search to perform.

This is *almost* a boolean option. That is, you can either do an unanchored
search for any pattern in a regex, or you can do an anchored search for any
pattern in a regex.

A third option exists that, assuming the regex engine supports it, permits
you to do an anchored search for a specific pattern.

Note that there is no way to run an unanchored search for a specific
pattern. If you need that, you'll need to build separate regexes for each
pattern.

# Errors

If a regex engine does not support the anchored mode selected, then the
regex engine will return an error. While any non-trivial regex engine
should support at least one of the available anchored modes, there is no
singular mode that is guaranteed to be universally supported. Some regex
engines might only support unanchored searches (DFAs compiled without
anchored starting states) and some regex engines might only support
anchored searches (like the one-pass DFA).

The specific error returned is a [`MatchError`](#matcherror) with a
`MatchErrorKind::UnsupportedAnchored` kind. The kind includes the
`Anchored` value given that is unsupported.

Note that regex engines should report "no match" if, for example, an
`Anchored::Pattern` is provided with an invalid pattern ID _but_ where
anchored searches for a specific pattern are supported. This is smooths out
behavior such that it's possible to guarantee that an error never occurs
based on how the regex engine is configured. All regex engines in this
crate report "no match" when searching for an invalid pattern ID, but where
searching for a valid pattern ID is otherwise supported.

# Example

This example shows how to use the various `Anchored` modes to run a
search. We use the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM)
because it supports all modes unconditionally. Some regex engines, like
the [`onepass::DFA`](crate::dfa::onepass::DFA) cannot support unanchored
searches.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    Anchored, Input, Match, PatternID,
};

let re = PikeVM::new_many(&[
    r"Mrs. \w+",
    r"Miss \w+",
    r"Mr. \w+",
    r"Ms. \w+",
])?;
let mut cache = re.create_cache();
let hay = "Hello Mr. Springsteen!";

// The default is to do an unanchored search.
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, hay));
// Explicitly ask for an unanchored search. Same as above.
let input = Input::new(hay).anchored(Anchored::No);
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, hay));

// Now try an anchored search. Since the match doesn't start at the
// beginning of the haystack, no match is found!
let input = Input::new(hay).anchored(Anchored::Yes);
assert_eq!(None, re.find(&mut cache, input));

// We can try an anchored search again, but move the location of where
// we start the search. Note that the offsets reported are still in
// terms of the overall haystack and not relative to where we started
// the search.
let input = Input::new(hay).anchored(Anchored::Yes).range(6..);
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, input));

// Now try an anchored search for a specific pattern. We specifically
// choose a pattern that we know doesn't match to prove that the search
// only looks for the pattern we provide.
let input = Input::new(hay)
    .anchored(Anchored::Pattern(PatternID::must(1)))
    .range(6..);
assert_eq!(None, re.find(&mut cache, input));

// But if we switch it to the pattern that we know matches, then we find
// the match.
let input = Input::new(hay)
    .anchored(Anchored::Pattern(PatternID::must(2)))
    .range(6..);
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, input));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Variants

- **`No`**

  Run an unanchored search. This means a match may occur anywhere at or
  after the start position of the search.
  
  This search can return a match for any pattern in the regex.

- **`Yes`**

  Run an anchored search. This means that a match must begin at the
  start position of the search.
  
  This search can return a match for any pattern in the regex.

- **`Pattern`**

  Run an anchored search for a specific pattern. This means that a match
  must be for the given pattern and must begin at the start position of
  the search.

#### Implementations

- `fn is_anchored(self: &Self) -> bool`

- `fn pattern(self: &Self) -> Option<PatternID>` — [`PatternID`](util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for Anchored`

- `fn clone(self: &Self) -> Anchored` — [`Anchored`](#anchored)

##### `impl Copy for Anchored`

##### `impl Debug for Anchored`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Anchored`

##### `impl PartialEq for Anchored`

- `fn eq(self: &Self, other: &Anchored) -> bool` — [`Anchored`](#anchored)

##### `impl StructuralPartialEq for Anchored`

### `MatchKind`

```rust
enum MatchKind {
    All,
    LeftmostFirst,
}
```

The kind of match semantics to use for a regex pattern.

The default match kind is `LeftmostFirst`, and this corresponds to the
match semantics used by most backtracking engines, such as Perl.

# Leftmost first or "preference order" match semantics

Leftmost-first semantics determine which match to report when there are
multiple paths through a regex that match at the same position. The tie is
essentially broken by how a backtracker would behave. For example, consider
running the regex `foofoofoo|foofoo|foo` on the haystack `foofoo`. In this
case, both the `foofoo` and `foo` branches match at position `0`. So should
the end of the match be `3` or `6`?

A backtracker will conceptually work by trying `foofoofoo` and failing.
Then it will try `foofoo`, find the match and stop there. Thus, the
leftmost-first match position is `6`. This is called "leftmost-first" or
"preference order" because the order of the branches as written in the
regex pattern is what determines how to break the tie.

(Note that leftmost-longest match semantics, which break ties by always
taking the longest matching string, are not currently supported by this
crate. These match semantics tend to be found in POSIX regex engines.)

This example shows how leftmost-first semantics work, and how it even
applies to multi-pattern regexes:

```rust
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    Match,
};

let re = PikeVM::new_many(&[
    r"foofoofoo",
    r"foofoo",
    r"foo",
])?;
let mut cache = re.create_cache();
let got: Vec<Match> = re.find_iter(&mut cache, "foofoo").collect();
let expected = vec![Match::must(1, 0..6)];
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

# All matches

The `All` match semantics report any and all matches, and generally will
attempt to match as much as possible. It doesn't respect any sort of match
priority at all, so things like non-greedy matching don't work in this
mode.

The fact that non-greedy matching doesn't work generally makes most forms
of unanchored non-overlapping searches have unintuitive behavior. Namely,
unanchored searches behave as if there is a `(?s-u:.)*?` prefix at the
beginning of the pattern, which is specifically non-greedy. Since it will
be treated as greedy in `All` match semantics, this generally means that
it will first attempt to consume all of the haystack and is likely to wind
up skipping matches.

Generally speaking, `All` should only be used in two circumstances:

* When running an anchored search and there is a desire to match as much as
possible. For example, when building a reverse regex matcher to find the
start of a match after finding the end. In this case, the reverse search
is anchored to the end of the match found by the forward search.
* When running overlapping searches. Since `All` encodes all possible
matches, this is generally what you want for an overlapping search. If you
try to use leftmost-first in an overlapping search, it is likely to produce
counter-intuitive results since leftmost-first specifically excludes some
matches from its underlying finite state machine.

This example demonstrates the counter-intuitive behavior of `All` semantics
when using a standard leftmost unanchored search:

```rust
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    Match, MatchKind,
};

let re = PikeVM::builder()
    .configure(PikeVM::config().match_kind(MatchKind::All))
    .build("foo")?;
let hay = "first foo second foo wat";
let mut cache = re.create_cache();
let got: Vec<Match> = re.find_iter(&mut cache, hay).collect();
// Notice that it completely skips the first 'foo'!
let expected = vec![Match::must(0, 17..20)];
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

This second example shows how `All` semantics are useful for an overlapping
search. Note that we use lower level lazy DFA APIs here since the NFA
engines only currently support a very limited form of overlapping search.

```rust
use regex_automata::{
    hybrid::dfa::{DFA, OverlappingState},
    HalfMatch, Input, MatchKind,
};

let re = DFA::builder()
    // If we didn't set 'All' semantics here, then the regex would only
    // match 'foo' at offset 3 and nothing else. Why? Because the state
    // machine implements preference order and knows that the 'foofoo' and
    // 'foofoofoo' branches can never match since 'foo' will always match
    // when they match and take priority.
    .configure(DFA::config().match_kind(MatchKind::All))
    .build(r"foo|foofoo|foofoofoo")?;
let mut cache = re.create_cache();
let mut state = OverlappingState::start();
let input = Input::new("foofoofoo");
let mut got = vec![];
loop {
    re.try_search_overlapping_fwd(&mut cache, &input, &mut state)?;
    let m = match state.get_match() {
        None => break,
        Some(m) => m,
    };
    got.push(m);
}
let expected = vec![
    HalfMatch::must(0, 3),
    HalfMatch::must(0, 6),
    HalfMatch::must(0, 9),
];
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Variants

- **`All`**

  Report all possible matches.

- **`LeftmostFirst`**

  Report only the leftmost matches. When multiple leftmost matches exist,
  report the match corresponding to the part of the regex that appears
  first in the syntax.

#### Implementations

- `fn continue_past_first_match(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for MatchKind`

- `fn clone(self: &Self) -> MatchKind` — [`MatchKind`](#matchkind)

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MatchKind`

- `fn default() -> MatchKind` — [`MatchKind`](#matchkind)

##### `impl Eq for MatchKind`

##### `impl PartialEq for MatchKind`

- `fn eq(self: &Self, other: &MatchKind) -> bool` — [`MatchKind`](#matchkind)

##### `impl StructuralPartialEq for MatchKind`

### `MatchErrorKind`

```rust
enum MatchErrorKind {
    Quit {
        byte: u8,
        offset: usize,
    },
    GaveUp {
        offset: usize,
    },
    HaystackTooLong {
        len: usize,
    },
    UnsupportedAnchored {
        mode: Anchored,
    },
}
```

The underlying kind of a [`MatchError`](#matcherror).

This is a **non-exhaustive** enum. That means new variants may be added in
a semver-compatible release.

#### Variants

- **`Quit`**

  The search saw a "quit" byte at which it was instructed to stop
  searching.

- **`GaveUp`**

  The search, based on heuristics, determined that it would be better
  to stop, typically to provide the caller an opportunity to use an
  alternative regex engine.
  
  Currently, the only way for this to occur is via the lazy DFA and
  only when it is configured to do so (it will not return this error by
  default).

- **`HaystackTooLong`**

  This error occurs if the haystack given to the regex engine was too
  long to be searched. This occurs, for example, with regex engines
  like the bounded backtracker that have a configurable fixed amount of
  capacity that is tied to the length of the haystack. Anything beyond
  that configured limit will result in an error at search time.

- **`UnsupportedAnchored`**

  An error indicating that a particular type of anchored search was
  requested, but that the regex engine does not support it.
  
  Note that this error should not be returned by a regex engine simply
  because the pattern ID is invalid (i.e., equal to or exceeds the number
  of patterns in the regex). In that case, the regex engine should report
  a non-match.

#### Trait Implementations

##### `impl Clone for MatchErrorKind`

- `fn clone(self: &Self) -> MatchErrorKind` — [`MatchErrorKind`](#matcherrorkind)

##### `impl Debug for MatchErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl PartialEq for MatchErrorKind`

- `fn eq(self: &Self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](#matcherrorkind)

##### `impl StructuralPartialEq for MatchErrorKind`

