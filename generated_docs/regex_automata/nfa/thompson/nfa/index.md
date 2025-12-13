*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [nfa](index.md)*

---

# Module `nfa`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NFA`](#nfa) | struct | A byte oriented Thompson non-deterministic finite automaton (NFA). |
| [`Inner`](#inner) | struct | The "inner" part of the NFA. |
| [`SparseTransitions`](#sparsetransitions) | struct | A sequence of transitions used to represent a sparse state. |
| [`DenseTransitions`](#densetransitions) | struct | A sequence of transitions used to represent a dense state. |
| [`Transition`](#transition) | struct | A single transition to another state. |
| [`PatternIter`](#patterniter) | struct | An iterator over all pattern IDs in an NFA. |
| [`State`](#state) | enum | A state in an NFA. |

## Structs

### `NFA`

```rust
struct NFA(alloc::sync::Arc<Inner>);
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:190-202`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L190-L202)*

A byte oriented Thompson non-deterministic finite automaton (NFA).

A Thompson NFA is a finite state machine that permits unconditional epsilon
transitions, but guarantees that there exists at most one non-epsilon
transition for each element in the alphabet for each state.

An NFA may be used directly for searching, for analysis or to build
a deterministic finite automaton (DFA).

# Cheap clones

Since an NFA is a core data type in this crate that many other regex
engines are based on top of, it is convenient to give ownership of an NFA
to said regex engines. Because of this, an NFA uses reference counting
internally. Therefore, it is cheap to clone and it is encouraged to do so.

# Capabilities

Using an NFA for searching via the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) provides the most amount
of "power" of any regex engine in this crate. Namely, it supports the
following in all cases:

1. Detection of a match.
2. Location of a match, including both the start and end offset, in a
single pass of the haystack.
3. Location of matching capturing groups.
4. Handles multiple patterns, including (1)-(3) when multiple patterns are
present.

# Capturing Groups

Groups refer to parenthesized expressions inside a regex pattern. They look
like this, where `exp` is an arbitrary regex:

* `(exp)` - An unnamed capturing group.
* `(?P<name>exp)` or `(?<name>exp)` - A named capturing group.
* `(?:exp)` - A non-capturing group.
* `(?i:exp)` - A non-capturing group that sets flags.

Only the first two forms are said to be _capturing_. Capturing
means that the last position at which they match is reportable. The
[`Captures`](crate::util::captures::Captures) type provides convenient
access to the match positions of capturing groups, which includes looking
up capturing groups by their name.

# Byte oriented

This NFA is byte oriented, which means that all of its transitions are
defined on bytes. In other words, the alphabet of an NFA consists of the
256 different byte values.

While DFAs nearly demand that they be byte oriented for performance
reasons, an NFA could conceivably be *Unicode codepoint* oriented. Indeed,
a previous version of this NFA supported both byte and codepoint oriented
modes. A codepoint oriented mode can work because an NFA fundamentally uses
a sparse representation of transitions, which works well with the large
sparse space of Unicode codepoints.

Nevertheless, this NFA is only byte oriented. This choice is primarily
driven by implementation simplicity, and also in part memory usage. In
practice, performance between the two is roughly comparable. However,
building a DFA (including a hybrid DFA) really wants a byte oriented NFA.
So if we do have a codepoint oriented NFA, then we also need to generate
byte oriented NFA in order to build an hybrid NFA/DFA. Thus, by only
generating byte oriented NFAs, we can produce one less NFA. In other words,
if we made our NFA codepoint oriented, we'd need to *also* make it support
a byte oriented mode, which is more complicated. But a byte oriented mode
can support everything.

# Differences with DFAs

At the theoretical level, the precise difference between an NFA and a DFA
is that, in a DFA, for every state, an input symbol unambiguously refers
to a single transition _and_ that an input symbol is required for each
transition. At a practical level, this permits DFA implementations to be
implemented at their core with a small constant number of CPU instructions
for each byte of input searched. In practice, this makes them quite a bit
faster than NFAs _in general_. Namely, in order to execute a search for any
Thompson NFA, one needs to keep track of a _set_ of states, and execute
the possible transitions on all of those states for each input symbol.
Overall, this results in much more overhead. To a first approximation, one
can expect DFA searches to be about an order of magnitude faster.

So why use an NFA at all? The main advantage of an NFA is that it takes
linear time (in the size of the pattern string after repetitions have been
expanded) to build and linear memory usage. A DFA, on the other hand, may
take exponential time and/or space to build. Even in non-pathological
cases, DFAs often take quite a bit more memory than their NFA counterparts,
_especially_ if large Unicode character classes are involved. Of course,
an NFA also provides additional capabilities. For example, it can match
Unicode word boundaries on non-ASCII text and resolve the positions of
capturing groups.

Note that a [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) strikes a
good balance between an NFA and a DFA. It avoids the exponential build time
of a DFA while maintaining its fast search time. The downside of a hybrid
NFA/DFA is that in some cases it can be slower at search time than the NFA.
(It also has less functionality than a pure NFA. It cannot handle Unicode
word boundaries on non-ASCII text and cannot resolve capturing groups.)

# Example

This shows how to build an NFA with the default configuration and execute a
search using the Pike VM.

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"foo[0-9]+")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let expected = Some(Match::must(0, 0..8));
re.captures(&mut cache, b"foo12345", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: resolving capturing groups

This example shows how to parse some simple dates and extract the
components of each date via capturing groups.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::captures::Captures,
};

let vm = PikeVM::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})")?;
let mut cache = vm.create_cache();

let haystack = "2012-03-14, 2013-01-01 and 2014-07-05";
let all: Vec<Captures> = vm.captures_iter(
    &mut cache, haystack.as_bytes()
).collect();
// There should be a total of 3 matches.
assert_eq!(3, all.len());
// The year from the second match is '2013'.
let span = all[1].get_group_by_name("y").unwrap();
assert_eq!("2013", &haystack[span]);

Ok::<(), Box<dyn std::error::Error>>(())
```

This example shows that only the last match of a capturing group is
reported, even if it had to match multiple times for an overall match
to occur.

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"([a-z]){4}")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let haystack = b"quux";
re.captures(&mut cache, haystack, &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(3..4)), caps.get_group(1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="nfa-new"></span>`fn new(pattern: &str) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../error/index.md#builderror)

  Parse the given regular expression using a default configuration and

  build an NFA from it.

  

  If you want a non-default configuration, then use the NFA

  [`Compiler`](../compiler/index.md) with a [`Config`](../compiler/index.md).

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re = PikeVM::new(r"foo[0-9]+")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(0, 0..8));

  re.captures(&mut cache, b"foo12345", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../error/index.md#builderror)

  Parse the given regular expressions using a default configuration and

  build a multi-NFA from them.

  

  If you want a non-default configuration, then use the NFA

  [`Compiler`](../compiler/index.md) with a [`Config`](../compiler/index.md).

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re = PikeVM::new_many(&["[0-9]+", "[a-z]+"])?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(1, 0..3));

  re.captures(&mut cache, b"foo12345bar", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-always-match"></span>`fn always_match() -> NFA` — [`NFA`](#nfa)

  Returns an NFA with a single regex pattern that always matches at every

  position.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  

  let re = PikeVM::new_from_nfa(NFA::always_match())?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(0, 0..0));

  re.captures(&mut cache, b"", &mut caps);

  assert_eq!(expected, caps.get_match());

  re.captures(&mut cache, b"foo", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-never-match"></span>`fn never_match() -> NFA` — [`NFA`](#nfa)

  Returns an NFA that never matches at any position.

  

  This is a convenience routine for creating an NFA with zero patterns.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, pikevm::PikeVM};

  

  let re = PikeVM::new_from_nfa(NFA::never_match())?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  re.captures(&mut cache, b"", &mut caps);

  assert!(!caps.is_match());

  re.captures(&mut cache, b"foo", &mut caps);

  assert!(!caps.is_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-config"></span>`fn config() -> Config` — [`Config`](../compiler/index.md#config)

  Return a default configuration for an `NFA`.

  

  This is a convenience routine to avoid needing to import the `Config`

  type when customizing the construction of an NFA.

  

  # Example

  

  This example shows how to build an NFA with a small size limit that

  results in a compilation error for any regex that tries to use more

  heap memory than the configured limit.

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, pikevm::PikeVM};

  

  let result = PikeVM::builder()

      .thompson(NFA::config().nfa_size_limit(Some(1_000)))

      // Remember, \w is Unicode-aware by default and thus huge.

      .build(r"\w+");

  assert!(result.is_err());

  ```

- <span id="nfa-compiler"></span>`fn compiler() -> Compiler` — [`Compiler`](../compiler/index.md#compiler)

  Return a compiler for configuring the construction of an `NFA`.

  

  This is a convenience routine to avoid needing to import the

  [`Compiler`](../compiler/index.md) type in common cases.

  

  # Example

  

  This example shows how to build an NFA that is permitted match invalid

  UTF-8. Without the additional syntax configuration here, compilation of

  `(?-u:.)` would fail because it is permitted to match invalid UTF-8.

  

  ```rust

  use regex_automata::{

      nfa::thompson::pikevm::PikeVM,

      util::syntax,

      Match,

  };

  

  let re = PikeVM::builder()

      .syntax(syntax::Config::new().utf8(false))

      .build(r"[a-z]+(?-u:.)")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(0, 1..5));

  re.captures(&mut cache, b"\xFFabc\xFF", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-patterns"></span>`fn patterns(&self) -> PatternIter<'_>` — [`PatternIter`](#patterniter)

  Returns an iterator over all pattern identifiers in this NFA.

  

  Pattern IDs are allocated in sequential order starting from zero,

  where the order corresponds to the order of patterns provided to the

  `NFA::new_many` constructor.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, PatternID};

  

  let nfa = NFA::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"])?;

  let pids: Vec<PatternID> = nfa.patterns().collect();

  assert_eq!(pids, vec![

      PatternID::must(0),

      PatternID::must(1),

      PatternID::must(2),

  ]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the total number of regex patterns in this NFA.

  

  This may return zero if the NFA was constructed with no patterns. In

  this case, the NFA can never produce a match for any input.

  

  This is guaranteed to be no bigger than `PatternID::LIMIT` because

  NFA construction will fail if too many patterns are added.

  

  It is always true that `nfa.patterns().count() == nfa.pattern_len()`.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let nfa = NFA::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"])?;

  assert_eq!(3, nfa.pattern_len());

  

  let nfa = NFA::never_match();

  assert_eq!(0, nfa.pattern_len());

  

  let nfa = NFA::always_match();

  assert_eq!(1, nfa.pattern_len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-start-anchored"></span>`fn start_anchored(&self) -> StateID` — [`StateID`](../../../util/primitives/index.md#stateid)

  Return the state identifier of the initial anchored state of this NFA.

  

  The returned identifier is guaranteed to be a valid index into the

  slice returned by `NFA::states`, and is also a valid argument to

  `NFA::state`.

  

  # Example

  

  This example shows a somewhat contrived example where we can easily

  predict the anchored starting state.

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, State, WhichCaptures};

  

  let nfa = NFA::compiler()

      .configure(NFA::config().which_captures(WhichCaptures::None))

      .build("a")?;

  let state = nfa.state(nfa.start_anchored());

  match *state {

      State::ByteRange { trans } => {

          assert_eq!(b'a', trans.start);

          assert_eq!(b'a', trans.end);

      }

      _ => unreachable!("unexpected state"),

  }

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-start-unanchored"></span>`fn start_unanchored(&self) -> StateID` — [`StateID`](../../../util/primitives/index.md#stateid)

  Return the state identifier of the initial unanchored state of this

  NFA.

  

  This is equivalent to the identifier returned by

  `NFA::start_anchored` when the NFA has no unanchored starting state.

  

  The returned identifier is guaranteed to be a valid index into the

  slice returned by `NFA::states`, and is also a valid argument to

  `NFA::state`.

  

  # Example

  

  This example shows that the anchored and unanchored starting states

  are equivalent when an anchored NFA is built.

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let nfa = NFA::new("^a")?;

  assert_eq!(nfa.start_anchored(), nfa.start_unanchored());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-start-pattern"></span>`fn start_pattern(&self, pid: PatternID) -> Option<StateID>` — [`PatternID`](../../../util/primitives/index.md#patternid), [`StateID`](../../../util/primitives/index.md#stateid)

  Return the state identifier of the initial anchored state for the given

  pattern, or `None` if there is no pattern corresponding to the given

  identifier.

  

  If one uses the starting state for a particular pattern, then the only

  match that can be returned is for the corresponding pattern.

  

  The returned identifier is guaranteed to be a valid index into the

  slice returned by `NFA::states`, and is also a valid argument to

  `NFA::state`.

  

  # Errors

  

  If the pattern doesn't exist in this NFA, then this returns an error.

  This occurs when `pid.as_usize() >= nfa.pattern_len()`.

  

  # Example

  

  This example shows that the anchored and unanchored starting states

  are equivalent when an anchored NFA is built.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, PatternID};

  

  let nfa = NFA::new_many(&["^a", "^b"])?;

  // The anchored and unanchored states for the entire NFA are the same,

  // since all of the patterns are anchored.

  assert_eq!(nfa.start_anchored(), nfa.start_unanchored());

  // But the anchored starting states for each pattern are distinct,

  // because these starting states can only lead to matches for the

  // corresponding pattern.

  let anchored = Some(nfa.start_anchored());

  assert_ne!(anchored, nfa.start_pattern(PatternID::must(0)));

  assert_ne!(anchored, nfa.start_pattern(PatternID::must(1)));

  // Requesting a pattern not in the NFA will result in None:

  assert_eq!(None, nfa.start_pattern(PatternID::must(2)));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-byte-class-set"></span>`fn byte_class_set(&self) -> &ByteClassSet` — [`ByteClassSet`](../../../util/alphabet/index.md#byteclassset)

  Get the byte class set for this NFA.

  

  A byte class set is a partitioning of this NFA's alphabet into

  equivalence classes. Any two bytes in the same equivalence class are

  guaranteed to never discriminate between a match or a non-match. (The

  partitioning may not be minimal.)

  

  Byte classes are used internally by this crate when building DFAs.

  Namely, among other optimizations, they enable a space optimization

  where the DFA's internal alphabet is defined over the equivalence

  classes of bytes instead of all possible byte values. The former is

  often quite a bit smaller than the latter, which permits the DFA to use

  less space for its transition table.

- <span id="nfa-byte-classes"></span>`fn byte_classes(&self) -> &ByteClasses` — [`ByteClasses`](../../../util/alphabet/index.md#byteclasses)

  Get the byte classes for this NFA.

  

  Byte classes represent a partitioning of this NFA's alphabet into

  equivalence classes. Any two bytes in the same equivalence class are

  guaranteed to never discriminate between a match or a non-match. (The

  partitioning may not be minimal.)

  

  Byte classes are used internally by this crate when building DFAs.

  Namely, among other optimizations, they enable a space optimization

  where the DFA's internal alphabet is defined over the equivalence

  classes of bytes instead of all possible byte values. The former is

  often quite a bit smaller than the latter, which permits the DFA to use

  less space for its transition table.

  

  # Example

  

  This example shows how to query the class of various bytes.

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let nfa = NFA::new("[a-z]+")?;

  let classes = nfa.byte_classes();

  // 'a' and 'z' are in the same class for this regex.

  assert_eq!(classes.get(b'a'), classes.get(b'z'));

  // But 'a' and 'A' are not.

  assert_ne!(classes.get(b'a'), classes.get(b'A'));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-state"></span>`fn state(&self, id: StateID) -> &State` — [`StateID`](../../../util/primitives/index.md#stateid), [`State`](#state)

  Return a reference to the NFA state corresponding to the given ID.

  

  This is a convenience routine for `nfa.states()[id]`.

  

  # Panics

  

  This panics when the given identifier does not reference a valid state.

  That is, when `id.as_usize() >= nfa.states().len()`.

  

  # Example

  

  The anchored state for a pattern will typically correspond to a

  capturing state for that pattern. (Although, this is not an API

  guarantee!)

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, State}, PatternID};

  

  let nfa = NFA::new("a")?;

  let state = nfa.state(nfa.start_pattern(PatternID::ZERO).unwrap());

  match *state {

      State::Capture { slot, .. } => {

          assert_eq!(0, slot.as_usize());

      }

      _ => unreachable!("unexpected state"),

  }

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-states"></span>`fn states(&self) -> &[State]` — [`State`](#state)

  Returns a slice of all states in this NFA.

  

  The slice returned is indexed by `StateID`. This provides a convenient

  way to access states while following transitions among those states.

  

  # Example

  

  This demonstrates that disabling UTF-8 mode can shrink the size of the

  NFA considerably in some cases, especially when using Unicode character

  classes.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::NFA;

  

  let nfa_unicode = NFA::new(r"\w")?;

  let nfa_ascii = NFA::new(r"(?-u)\w")?;

  // Yes, a factor of 45 difference. No lie.

  assert!(40 * nfa_ascii.states().len() < nfa_unicode.states().len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../../util/captures/index.md#groupinfo)

  Returns the capturing group info for this NFA.

  

  The [`GroupInfo`](../../../util/captures/index.md) provides a way to map to and from capture index

  and capture name for each pattern. It also provides a mapping from

  each of the capturing groups in every pattern to their corresponding

  slot offsets encoded in [`State::Capture`](../../../index.md) states.

  

  Note that `GroupInfo` uses reference counting internally, such that

  cloning a `GroupInfo` is very cheap.

  

  # Example

  

  This example shows how to get a list of all capture group names for

  a particular pattern.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, PatternID};

  

  let nfa = NFA::new(r"(a)(?P<foo>b)(c)(d)(?P<bar>e)")?;

  // The first is the implicit group that is always unnamed. The next

  // 5 groups are the explicit groups found in the concrete syntax above.

  let expected = vec![None, None, Some("foo"), None, None, Some("bar")];

  let got: Vec<Option<&str>> =

      nfa.group_info().pattern_names(PatternID::ZERO).collect();

  assert_eq!(expected, got);

  

  // Using an invalid pattern ID will result in nothing yielded.

  let got = nfa.group_info().pattern_names(PatternID::must(999)).count();

  assert_eq!(0, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-has-capture"></span>`fn has_capture(&self) -> bool`

  Returns true if and only if this NFA has at least one

  [`Capture`](State::Capture) in its sequence of states.

  

  This is useful as a way to perform a quick test before attempting

  something that does or does not require capture states. For example,

  some regex engines (like the PikeVM) require capture states in order to

  work at all.

  

  # Example

  

  This example shows a few different NFAs and whether they have captures

  or not.

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, WhichCaptures};

  

  // Obviously has capture states.

  let nfa = NFA::new("(a)")?;

  assert!(nfa.has_capture());

  

  // Less obviously has capture states, because every pattern has at

  // least one anonymous capture group corresponding to the match for the

  // entire pattern.

  let nfa = NFA::new("a")?;

  assert!(nfa.has_capture());

  

  // Other than hand building your own NFA, this is the only way to build

  // an NFA without capturing groups. In general, you should only do this

  // if you don't intend to use any of the NFA-oriented regex engines.

  // Overall, capturing groups don't have many downsides. Although they

  // can add a bit of noise to simple NFAs, so it can be nice to disable

  // them for debugging purposes.

  //

  // Notice that 'has_capture' is false here even when we have an

  // explicit capture group in the pattern.

  let nfa = NFA::compiler()

      .configure(NFA::config().which_captures(WhichCaptures::None))

      .build("(a)")?;

  assert!(!nfa.has_capture());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-has-empty"></span>`fn has_empty(&self) -> bool`

  Returns true if and only if this NFA can match the empty string.

  When it returns false, all possible matches are guaranteed to have a

  non-zero length.

  

  This is useful as cheap way to know whether code needs to handle the

  case of a zero length match. This is particularly important when UTF-8

  modes are enabled, as when UTF-8 mode is enabled, empty matches that

  split a codepoint must never be reported. This extra handling can

  sometimes be costly, and since regexes matching an empty string are

  somewhat rare, it can be beneficial to treat such regexes specially.

  

  # Example

  

  This example shows a few different NFAs and whether they match the

  empty string or not. Notice the empty string isn't merely a matter

  of a string of length literally `0`, but rather, whether a match can

  occur between specific pairs of bytes.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::syntax};

  

  // The empty regex matches the empty string.

  let nfa = NFA::new("")?;

  assert!(nfa.has_empty(), "empty matches empty");

  // The '+' repetition operator requires at least one match, and so

  // does not match the empty string.

  let nfa = NFA::new("a+")?;

  assert!(!nfa.has_empty(), "+ does not match empty");

  // But the '*' repetition operator does.

  let nfa = NFA::new("a*")?;

  assert!(nfa.has_empty(), "* does match empty");

  // And wrapping '+' in an operator that can match an empty string also

  // causes it to match the empty string too.

  let nfa = NFA::new("(a+)*")?;

  assert!(nfa.has_empty(), "+ inside of * matches empty");

  

  // If a regex is just made of a look-around assertion, even if the

  // assertion requires some kind of non-empty string around it (such as

  // \b), then it is still treated as if it matches the empty string.

  // Namely, if a match occurs of just a look-around assertion, then the

  // match returned is empty.

  let nfa = NFA::compiler()

      .syntax(syntax::Config::new().utf8(false))

      .build(r"^$\A\z\b\B(?-u:\b\B)")?;

  assert!(nfa.has_empty(), "assertions match empty");

  // Even when an assertion is wrapped in a '+', it still matches the

  // empty string.

  let nfa = NFA::new(r"\b+")?;

  assert!(nfa.has_empty(), "+ of an assertion matches empty");

  

  // An alternation with even one branch that can match the empty string

  // is also said to match the empty string overall.

  let nfa = NFA::new("foo|(bar)?|quux")?;

  assert!(nfa.has_empty(), "alternations can match empty");

  

  // An NFA that matches nothing does not match the empty string.

  let nfa = NFA::new("[a&&b]")?;

  assert!(!nfa.has_empty(), "never matching means not matching empty");

  // But if it's wrapped in something that doesn't require a match at

  // all, then it can match the empty string!

  let nfa = NFA::new("[a&&b]*")?;

  assert!(nfa.has_empty(), "* on never-match still matches empty");

  // Since a '+' requires a match, using it on something that can never

  // match will itself produce a regex that can never match anything,

  // and thus does not match the empty string.

  let nfa = NFA::new("[a&&b]+")?;

  assert!(!nfa.has_empty(), "+ on never-match still matches nothing");

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-is-utf8"></span>`fn is_utf8(&self) -> bool`

  Whether UTF-8 mode is enabled for this NFA or not.

  

  When UTF-8 mode is enabled, all matches reported by a regex engine

  derived from this NFA are guaranteed to correspond to spans of valid

  UTF-8. This includes zero-width matches. For example, the regex engine

  must guarantee that the empty regex will not match at the positions

  between code units in the UTF-8 encoding of a single codepoint.

  

  See `Config::utf8` for more information.

  

  This is enabled by default.

  

  # Example

  

  This example shows how UTF-8 mode can impact the match spans that may

  be reported in certain cases.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, pikevm::PikeVM},

      Match, Input,

  };

  

  let re = PikeVM::new("")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  // UTF-8 mode is enabled by default.

  let mut input = Input::new("☃");

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 0..0)), caps.get_match());

  

  // Even though an empty regex matches at 1..1, our next match is

  // 3..3 because 1..1 and 2..2 split the snowman codepoint (which is

  // three bytes long).

  input.set_start(1);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 3..3)), caps.get_match());

  

  // But if we disable UTF-8, then we'll get matches at 1..1 and 2..2:

  let re = PikeVM::builder()

      .thompson(thompson::Config::new().utf8(false))

      .build("")?;

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 1..1)), caps.get_match());

  

  input.set_start(2);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 2..2)), caps.get_match());

  

  input.set_start(3);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 3..3)), caps.get_match());

  

  input.set_start(4);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(None, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-is-reverse"></span>`fn is_reverse(&self) -> bool`

  Returns true when this NFA is meant to be matched in reverse.

  

  Generally speaking, when this is true, it means the NFA is supposed to

  be used in conjunction with moving backwards through the haystack. That

  is, from a higher memory address to a lower memory address.

  

  It is often the case that lower level routines dealing with an NFA

  don't need to care about whether it is "meant" to be matched in reverse

  or not. However, there are some specific cases where it matters. For

  example, the implementation of CRLF-aware `^` and `$` line anchors

  needs to know whether the search is in the forward or reverse

  direction. In the forward direction, neither `^` nor `$` should match

  when a `\r` has been seen previously and a `\n` is next. However, in

  the reverse direction, neither `^` nor `$` should match when a `\n`

  has been seen previously and a `\r` is next. This fundamentally changes

  how the state machine is constructed, and thus needs to be altered

  based on the direction of the search.

  

  This is automatically set when using a [`Compiler`](../compiler/index.md) with a configuration

  where `Config::reverse` is enabled. If you're building your own NFA

  by hand via a [`Builder`](../builder/index.md)

- <span id="nfa-is-always-start-anchored"></span>`fn is_always_start_anchored(&self) -> bool`

  Returns true if and only if all starting states for this NFA correspond

  to the beginning of an anchored search.

  

  Typically, an NFA will have both an anchored and an unanchored starting

  state. Namely, because it tends to be useful to have both and the cost

  of having an unanchored starting state is almost zero (for an NFA).

  However, if all patterns in the NFA are themselves anchored, then even

  the unanchored starting state will correspond to an anchored search

  since the pattern doesn't permit anything else.

  

  # Example

  

  This example shows a few different scenarios where this method's

  return value varies.

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  // The unanchored starting state permits matching this pattern anywhere

  // in a haystack, instead of just at the beginning.

  let nfa = NFA::new("a")?;

  assert!(!nfa.is_always_start_anchored());

  

  // In this case, the pattern is itself anchored, so there is no way

  // to run an unanchored search.

  let nfa = NFA::new("^a")?;

  assert!(nfa.is_always_start_anchored());

  

  // When multiline mode is enabled, '^' can match at the start of a line

  // in addition to the start of a haystack, so an unanchored search is

  // actually possible.

  let nfa = NFA::new("(?m)^a")?;

  assert!(!nfa.is_always_start_anchored());

  

  // Weird cases also work. A pattern is only considered anchored if all

  // matches may only occur at the start of a haystack.

  let nfa = NFA::new("(^a)|a")?;

  assert!(!nfa.is_always_start_anchored());

  

  // When multiple patterns are present, if they are all anchored, then

  // the NFA is always anchored too.

  let nfa = NFA::new_many(&["^a", "^b", "^c"])?;

  assert!(nfa.is_always_start_anchored());

  

  // But if one pattern is unanchored, then the NFA must permit an

  // unanchored search.

  let nfa = NFA::new_many(&["^a", "b", "^c"])?;

  assert!(!nfa.is_always_start_anchored());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-look-matcher"></span>`fn look_matcher(&self) -> &LookMatcher` — [`LookMatcher`](../../../util/look/index.md#lookmatcher)

  Returns the look-around matcher associated with this NFA.

  

  A look-around matcher determines how to match look-around assertions.

  In particular, some assertions are configurable. For example, the

  `(?m:^)` and `(?m:$)` assertions can have their line terminator changed

  from the default of `\n` to any other byte.

  

  If the NFA was built using a [`Compiler`](../compiler/index.md), then this matcher

  can be set via the `Config::look_matcher` configuration

  knob. Otherwise, if you've built an NFA by hand, it is set via

  `Builder::set_look_matcher`.

  

  # Example

  

  This shows how to change the line terminator for multi-line assertions.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, pikevm::PikeVM},

      util::look::LookMatcher,

      Match, Input,

  };

  

  let mut lookm = LookMatcher::new();

  lookm.set_line_terminator(b'\x00');

  

  let re = PikeVM::builder()

      .thompson(thompson::Config::new().look_matcher(lookm))

      .build(r"(?m)^[a-z]+$")?;

  let mut cache = re.create_cache();

  

  // Multi-line assertions now use NUL as a terminator.

  assert_eq!(

      Some(Match::must(0, 1..4)),

      re.find(&mut cache, b"\x00abc\x00"),

  );

  // ... and \n is no longer recognized as a terminator.

  assert_eq!(

      None,

      re.find(&mut cache, b"\nabc\n"),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-look-set-any"></span>`fn look_set_any(&self) -> LookSet` — [`LookSet`](../../../util/look/index.md#lookset)

  Returns the union of all look-around assertions used throughout this

  NFA. When the returned set is empty, it implies that the NFA has no

  look-around assertions and thus zero conditional epsilon transitions.

  

  This is useful in some cases enabling optimizations. It is not

  unusual, for example, for optimizations to be of the form, "for any

  regex with zero conditional epsilon transitions, do ..." where "..."

  is some kind of optimization.

  

  This isn't only helpful for optimizations either. Sometimes look-around

  assertions are difficult to support. For example, many of the DFAs in

  this crate don't support Unicode word boundaries or handle them using

  heuristics. Handling that correctly typically requires some kind of

  cheap check of whether the NFA has a Unicode word boundary in the first

  place.

  

  # Example

  

  This example shows how this routine varies based on the regex pattern:

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::look::Look};

  

  // No look-around at all.

  let nfa = NFA::new("a")?;

  assert!(nfa.look_set_any().is_empty());

  

  // When multiple patterns are present, since this returns the union,

  // it will include look-around assertions that only appear in one

  // pattern.

  let nfa = NFA::new_many(&["a", "b", "a^b", "c"])?;

  assert!(nfa.look_set_any().contains(Look::Start));

  

  // Some groups of assertions have various shortcuts. For example:

  let nfa = NFA::new(r"(?-u:\b)")?;

  assert!(nfa.look_set_any().contains_word());

  assert!(!nfa.look_set_any().contains_word_unicode());

  assert!(nfa.look_set_any().contains_word_ascii());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-look-set-prefix-any"></span>`fn look_set_prefix_any(&self) -> LookSet` — [`LookSet`](../../../util/look/index.md#lookset)

  Returns the union of all prefix look-around assertions for every

  pattern in this NFA. When the returned set is empty, it implies none of

  the patterns require moving through a conditional epsilon transition

  before inspecting the first byte in the haystack.

  

  This can be useful for determining what kinds of assertions need to be

  satisfied at the beginning of a search. For example, typically DFAs

  in this crate will build a distinct starting state for each possible

  starting configuration that might result in look-around assertions

  being satisfied differently. However, if the set returned here is

  empty, then you know that the start state is invariant because there

  are no conditional epsilon transitions to consider.

  

  # Example

  

  This example shows how this routine varies based on the regex pattern:

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::look::Look};

  

  // No look-around at all.

  let nfa = NFA::new("a")?;

  assert!(nfa.look_set_prefix_any().is_empty());

  

  // When multiple patterns are present, since this returns the union,

  // it will include look-around assertions that only appear in one

  // pattern. But it will only include assertions that are in the prefix

  // of a pattern. For example, this includes '^' but not '$' even though

  // '$' does appear.

  let nfa = NFA::new_many(&["a", "b", "^ab$", "c"])?;

  assert!(nfa.look_set_prefix_any().contains(Look::Start));

  assert!(!nfa.look_set_prefix_any().contains(Look::End));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the memory usage, in bytes, of this NFA.

  

  This does **not** include the stack size used up by this NFA. To

  compute that, use `std::mem::size_of::<NFA>()`.

  

  # Example

  

  This example shows that large Unicode character classes can use quite

  a bit of memory.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::NFA;

  

  let nfa_unicode = NFA::new(r"\w")?;

  let nfa_ascii = NFA::new(r"(?-u:\w)")?;

  

  assert!(10 * nfa_ascii.memory_usage() < nfa_unicode.memory_usage());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

#### Trait Implementations

##### `impl Any for NFA`

- <span id="nfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NFA`

- <span id="nfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NFA`

- <span id="nfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NFA`

- <span id="nfa-clone"></span>`fn clone(&self) -> NFA` — [`NFA`](#nfa)

##### `impl CloneToUninit for NFA`

- <span id="nfa-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NFA`

- <span id="nfa-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NFA`

- <span id="nfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NFA`

- <span id="nfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for NFA`

- <span id="nfa-toowned-type-owned"></span>`type Owned = T`

- <span id="nfa-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nfa-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NFA`

- <span id="nfa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nfa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NFA`

- <span id="nfa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nfa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Inner`

```rust
struct Inner {
    states: alloc::vec::Vec<State>,
    start_anchored: crate::util::primitives::StateID,
    start_unanchored: crate::util::primitives::StateID,
    start_pattern: alloc::vec::Vec<crate::util::primitives::StateID>,
    group_info: crate::util::captures::GroupInfo,
    byte_class_set: crate::util::alphabet::ByteClassSet,
    byte_classes: crate::util::alphabet::ByteClasses,
    has_capture: bool,
    has_empty: bool,
    utf8: bool,
    reverse: bool,
    look_matcher: crate::util::look::LookMatcher,
    look_set_any: crate::util::look::LookSet,
    look_set_prefix_any: crate::util::look::LookSet,
    memory_extra: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1195-1268`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1195-L1268)*

The "inner" part of the NFA. We split this part out so that we can easily
wrap it in an `Arc` above in the definition of `NFA`.

See builder.rs for the code that actually builds this type. This module
does provide (internal) mutable methods for adding things to this
NFA before finalizing it, but the high level construction process is
controlled by the builder abstraction. (Which is complicated enough to
get its own module.)

#### Fields

- **`states`**: `alloc::vec::Vec<State>`

  The state sequence. This sequence is guaranteed to be indexable by all
  starting state IDs, and it is also guaranteed to contain at most one
  `Match` state for each pattern compiled into this NFA. (A pattern may
  not have a corresponding `Match` state if a `Match` state is impossible
  to reach.)

- **`start_anchored`**: `crate::util::primitives::StateID`

  The anchored starting state of this NFA.

- **`start_unanchored`**: `crate::util::primitives::StateID`

  The unanchored starting state of this NFA.

- **`start_pattern`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The starting states for each individual pattern. Starting at any
  of these states will result in only an anchored search for the
  corresponding pattern. The vec is indexed by pattern ID. When the NFA
  contains a single regex, then `start_pattern[0]` and `start_anchored`
  are always equivalent.

- **`group_info`**: `crate::util::captures::GroupInfo`

  Info about the capturing groups in this NFA. This is responsible for
  mapping groups to slots, mapping groups to names and names to groups.

- **`byte_class_set`**: `crate::util::alphabet::ByteClassSet`

  A representation of equivalence classes over the transitions in this
  NFA. Two bytes in the same equivalence class must not discriminate
  between a match or a non-match. This map can be used to shrink the
  total size of a DFA's transition table with a small match-time cost.
  
  Note that the NFA's transitions are *not* defined in terms of these
  equivalence classes. The NFA's transitions are defined on the original
  byte values. For the most part, this is because they wouldn't really
  help the NFA much since the NFA already uses a sparse representation
  to represent transitions. Byte classes are most effective in a dense
  representation.

- **`byte_classes`**: `crate::util::alphabet::ByteClasses`

  This is generated from `byte_class_set`, and essentially represents the
  same thing but supports different access patterns. Namely, this permits
  looking up the equivalence class of a byte very cheaply.
  
  Ideally we would just store this, but because of annoying code
  structure reasons, we keep both this and `byte_class_set` around for
  now. I think I would prefer that `byte_class_set` were computed in the
  `Builder`, but right now, we compute it as states are added to the
  `NFA`.

- **`has_capture`**: `bool`

  Whether this NFA has a `Capture` state anywhere.

- **`has_empty`**: `bool`

  When the empty string is in the language matched by this NFA.

- **`utf8`**: `bool`

  Whether UTF-8 mode is enabled for this NFA. Briefly, this means that
  all non-empty matches produced by this NFA correspond to spans of valid
  UTF-8, and any empty matches produced by this NFA that split a UTF-8
  encoded codepoint should be filtered out by the corresponding regex
  engine.

- **`reverse`**: `bool`

  Whether this NFA is meant to be matched in reverse or not.

- **`look_matcher`**: `crate::util::look::LookMatcher`

  The matcher to be used for look-around assertions.

- **`look_set_any`**: `crate::util::look::LookSet`

  The union of all look-around assertions that occur anywhere within
  this NFA. If this set is empty, then it means there are precisely zero
  conditional epsilon transitions in the NFA.

- **`look_set_prefix_any`**: `crate::util::look::LookSet`

  The union of all look-around assertions that occur as a zero-length
  prefix for any of the patterns in this NFA.

- **`memory_extra`**: `usize`

  Heap memory used indirectly by NFA states and other things (like the
  various capturing group representations above). Since each state
  might use a different amount of heap, we need to keep track of this
  incrementally.

#### Implementations

- <span id="inner-into-nfa"></span>`fn into_nfa(self) -> NFA` — [`NFA`](#nfa)

  Runs any last finalization bits and turns this into a full NFA.

- <span id="inner-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../../util/captures/index.md#groupinfo)

  Returns the capturing group info for this NFA.

- <span id="inner-add"></span>`fn add(&mut self, state: State) -> StateID` — [`State`](#state), [`StateID`](../../../util/primitives/index.md#stateid)

  Add the given state to this NFA after allocating a fresh identifier for

  it.

  

  This panics if too many states are added such that a fresh identifier

  could not be created. (Currently, the only caller of this routine is

  a `Builder`, and it upholds this invariant.)

- <span id="inner-set-starts"></span>`fn set_starts(&mut self, start_anchored: StateID, start_unanchored: StateID, start_pattern: &[StateID])` — [`StateID`](../../../util/primitives/index.md#stateid)

  Set the starting state identifiers for this NFA.

  

  `start_anchored` and `start_unanchored` may be equivalent. When they

  are, then the NFA can only execute anchored searches. This might

  occur, for example, for patterns that are unconditionally anchored.

  e.g., `^foo`.

- <span id="inner-set-utf8"></span>`fn set_utf8(&mut self, yes: bool)`

  Sets the UTF-8 mode of this NFA.

- <span id="inner-set-reverse"></span>`fn set_reverse(&mut self, yes: bool)`

  Sets the reverse mode of this NFA.

- <span id="inner-set-look-matcher"></span>`fn set_look_matcher(&mut self, m: LookMatcher)` — [`LookMatcher`](../../../util/look/index.md#lookmatcher)

  Sets the look-around assertion matcher for this NFA.

- <span id="inner-set-captures"></span>`fn set_captures(&mut self, captures: &[Vec<Option<Arc<str>>>]) -> Result<(), GroupInfoError>` — [`GroupInfoError`](../../../util/captures/index.md#groupinfoerror)

  Set the capturing groups for this NFA.

  

  The given slice should contain the capturing groups for each pattern,

  The capturing groups in turn should correspond to the total number of

  capturing groups in the pattern, including the anonymous first capture

  group for each pattern. If a capturing group does have a name, then it

  should be provided as a Arc<str>.

  

  This returns an error if a corresponding `GroupInfo` could not be

  built.

- <span id="inner-remap"></span>`fn remap(&mut self, old_to_new: &[StateID])` — [`StateID`](../../../util/primitives/index.md#stateid)

  Remap the transitions in every state of this NFA using the given map.

  The given map should be indexed according to state ID namespace used by

  the transitions of the states currently in this NFA.

  

  This is particularly useful to the NFA builder, since it is convenient

  to add NFA states in order to produce their final IDs. Then, after all

  of the intermediate "empty" states (unconditional epsilon transitions)

  have been removed from the builder's representation, we can re-map all

  of the transitions in the states already added to their final IDs.

#### Trait Implementations

##### `impl Any for Inner`

- <span id="inner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inner`

- <span id="inner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inner`

- <span id="inner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Inner`

- <span id="inner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Inner`

- <span id="inner-default"></span>`fn default() -> Inner` — [`Inner`](#inner)

##### `impl<T> From for Inner`

- <span id="inner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Inner`

- <span id="inner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Inner`

- <span id="inner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Inner`

- <span id="inner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SparseTransitions`

```rust
struct SparseTransitions {
    pub transitions: alloc::boxed::Box<[Transition]>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1795-1798`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1795-L1798)*

A sequence of transitions used to represent a sparse state.

This is the primary representation of a [`Sparse`](State::Sparse) state.
It corresponds to a sorted sequence of transitions with non-overlapping
byte ranges. If the byte at the current position in the haystack matches
one of the byte ranges, then the finite state machine should take the
corresponding transition.

#### Fields

- **`transitions`**: `alloc::boxed::Box<[Transition]>`

  The sorted sequence of non-overlapping transitions.

#### Implementations

- <span id="sparsetransitions-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a matching byte

  range (there is at most one) corresponding to the position `at` in

  `haystack`.

  

  If `at >= haystack.len()`, then this returns `None`.

- <span id="sparsetransitions-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../../util/alphabet/index.md#unit), [`StateID`](../../../util/primitives/index.md#stateid)

  This follows the matching transition for any member of the alphabet.

  

  The matching transition is found by looking for a matching byte

  range (there is at most one) corresponding to the position `at` in

  `haystack`. If the given alphabet unit is [`EOI`](alphabet::Unit::eoi),

  then this always returns `None`.

- <span id="sparsetransitions-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a matching byte range

  (there is at most one) corresponding to the byte given.

#### Trait Implementations

##### `impl Any for SparseTransitions`

- <span id="sparsetransitions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SparseTransitions`

- <span id="sparsetransitions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SparseTransitions`

- <span id="sparsetransitions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SparseTransitions`

- <span id="sparsetransitions-clone"></span>`fn clone(&self) -> SparseTransitions` — [`SparseTransitions`](#sparsetransitions)

##### `impl CloneToUninit for SparseTransitions`

- <span id="sparsetransitions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SparseTransitions`

- <span id="sparsetransitions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SparseTransitions`

##### `impl<T> From for SparseTransitions`

- <span id="sparsetransitions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SparseTransitions`

- <span id="sparsetransitions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SparseTransitions`

- <span id="sparsetransitions-partialeq-eq"></span>`fn eq(&self, other: &SparseTransitions) -> bool` — [`SparseTransitions`](#sparsetransitions)

##### `impl StructuralPartialEq for SparseTransitions`

##### `impl ToOwned for SparseTransitions`

- <span id="sparsetransitions-toowned-type-owned"></span>`type Owned = T`

- <span id="sparsetransitions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sparsetransitions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SparseTransitions`

- <span id="sparsetransitions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sparsetransitions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SparseTransitions`

- <span id="sparsetransitions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sparsetransitions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DenseTransitions`

```rust
struct DenseTransitions {
    pub transitions: alloc::boxed::Box<[crate::util::primitives::StateID]>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1882-1886`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1882-L1886)*

A sequence of transitions used to represent a dense state.

This is the primary representation of a [`Dense`](State::Dense) state. It
provides constant time matching. That is, given a byte in a haystack and
a `DenseTransitions`, one can determine if the state matches in constant
time.

This is in contrast to `SparseTransitions`, whose time complexity is
necessarily bigger than constant time. Also in contrast, `DenseTransitions`
usually requires (much) more heap memory.

#### Fields

- **`transitions`**: `alloc::boxed::Box<[crate::util::primitives::StateID]>`

  A dense representation of this state's transitions on the heap. This
  always has length 256.

#### Implementations

- <span id="densetransitions-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a transition that

  doesn't correspond to `StateID::ZERO` for the byte `at` the given

  position in `haystack`.

  

  If `at >= haystack.len()`, then this returns `None`.

- <span id="densetransitions-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../../util/alphabet/index.md#unit), [`StateID`](../../../util/primitives/index.md#stateid)

  This follows the matching transition for any member of the alphabet.

  

  The matching transition is found by looking for a transition that

  doesn't correspond to `StateID::ZERO` for the given alphabet `unit`.

  

  If the given alphabet unit is [`EOI`](alphabet::Unit::eoi), then

  this returns `None`.

- <span id="densetransitions-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a transition that

  doesn't correspond to `StateID::ZERO` for the given `byte`.

- <span id="densetransitions-iter"></span>`fn iter(&self) -> impl Iterator<Item = Transition> + '_` — [`Transition`](#transition)

  Returns an iterator over all transitions that don't point to

  `StateID::ZERO`.

#### Trait Implementations

##### `impl Any for DenseTransitions`

- <span id="densetransitions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DenseTransitions`

- <span id="densetransitions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DenseTransitions`

- <span id="densetransitions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DenseTransitions`

- <span id="densetransitions-clone"></span>`fn clone(&self) -> DenseTransitions` — [`DenseTransitions`](#densetransitions)

##### `impl CloneToUninit for DenseTransitions`

- <span id="densetransitions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DenseTransitions`

- <span id="densetransitions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DenseTransitions`

##### `impl<T> From for DenseTransitions`

- <span id="densetransitions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DenseTransitions`

- <span id="densetransitions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DenseTransitions`

- <span id="densetransitions-partialeq-eq"></span>`fn eq(&self, other: &DenseTransitions) -> bool` — [`DenseTransitions`](#densetransitions)

##### `impl StructuralPartialEq for DenseTransitions`

##### `impl ToOwned for DenseTransitions`

- <span id="densetransitions-toowned-type-owned"></span>`type Owned = T`

- <span id="densetransitions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="densetransitions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DenseTransitions`

- <span id="densetransitions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="densetransitions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DenseTransitions`

- <span id="densetransitions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="densetransitions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Transition`

```rust
struct Transition {
    pub start: u8,
    pub end: u8,
    pub next: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1965-1972`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1965-L1972)*

A single transition to another state.

This transition may only be followed if the current byte in the haystack
falls in the inclusive range of bytes specified.

#### Fields

- **`start`**: `u8`

  The inclusive start of the byte range.

- **`end`**: `u8`

  The inclusive end of the byte range.

- **`next`**: `crate::util::primitives::StateID`

  The identifier of the state to transition to.

#### Implementations

- <span id="transition-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> bool`

  Returns true if the position `at` in `haystack` falls in this

  transition's range of bytes.

  

  If `at >= haystack.len()`, then this returns `false`.

- <span id="transition-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> bool` — [`Unit`](../../../util/alphabet/index.md#unit)

  Returns true if the given alphabet unit falls in this transition's

  range of bytes. If the given unit is [`EOI`](alphabet::Unit::eoi), then

  this returns `false`.

- <span id="transition-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> bool`

  Returns true if the given byte falls in this transition's range of

  bytes.

#### Trait Implementations

##### `impl Any for Transition`

- <span id="transition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Transition`

- <span id="transition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Transition`

- <span id="transition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Transition`

- <span id="transition-clone"></span>`fn clone(&self) -> Transition` — [`Transition`](#transition)

##### `impl CloneToUninit for Transition`

- <span id="transition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- <span id="transition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Transition`

##### `impl<T> From for Transition`

- <span id="transition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Transition`

- <span id="transition-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Transition`

- <span id="transition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Transition`

- <span id="transition-partialeq-eq"></span>`fn eq(&self, other: &Transition) -> bool` — [`Transition`](#transition)

##### `impl StructuralPartialEq for Transition`

##### `impl ToOwned for Transition`

- <span id="transition-toowned-type-owned"></span>`type Owned = T`

- <span id="transition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="transition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Transition`

- <span id="transition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="transition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Transition`

- <span id="transition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="transition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternIter<'a>`

```rust
struct PatternIter<'a> {
    it: crate::util::primitives::PatternIDIter,
    _marker: core::marker::PhantomData<&'a ()>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:2023-2031`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L2023-L2031)*

An iterator over all pattern IDs in an NFA.

This iterator is created by `NFA::patterns`.

The lifetime parameter `'a` refers to the lifetime of the NFA from which
this pattern iterator was created.

#### Fields

- **`_marker`**: `core::marker::PhantomData<&'a ()>`

  We explicitly associate a lifetime with this iterator even though we
  don't actually borrow anything from the NFA. We do this for backward
  compatibility purposes. If we ever do need to borrow something from
  the NFA, then we can and just get rid of this marker without breaking
  the public API.

#### Trait Implementations

##### `impl Any for PatternIter<'a>`

- <span id="patterniter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternIter<'a>`

- <span id="patterniter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternIter<'a>`

- <span id="patterniter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PatternIter<'a>`

- <span id="patterniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PatternIter<'a>`

- <span id="patterniter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PatternIter<'a>`

- <span id="patterniter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PatternIter<'a>`

- <span id="patterniter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patterniter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="patterniter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternIter<'a>`

- <span id="patterniter-iterator-type-item"></span>`type Item = PatternID`

- <span id="patterniter-iterator-next"></span>`fn next(&mut self) -> Option<PatternID>` — [`PatternID`](../../../util/primitives/index.md#patternid)

##### `impl<U> TryFrom for PatternIter<'a>`

- <span id="patterniter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patterniter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternIter<'a>`

- <span id="patterniter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterniter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `State`

```rust
enum State {
    ByteRange {
        trans: Transition,
    },
    Sparse(SparseTransitions),
    Dense(DenseTransitions),
    Look {
        look: crate::util::look::Look,
        next: crate::util::primitives::StateID,
    },
    Union {
        alternates: alloc::boxed::Box<[crate::util::primitives::StateID]>,
    },
    BinaryUnion {
        alt1: crate::util::primitives::StateID,
        alt2: crate::util::primitives::StateID,
    },
    Capture {
        next: crate::util::primitives::StateID,
        pattern_id: crate::util::primitives::PatternID,
        group_index: crate::util::primitives::SmallIndex,
        slot: crate::util::primitives::SmallIndex,
    },
    Fail,
    Match {
        pattern_id: crate::util::primitives::PatternID,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1514-1621`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1514-L1621)*

A state in an NFA.

In theory, it can help to conceptualize an `NFA` as a graph consisting of
`State`s. Each `State` contains its complete set of outgoing transitions.

In practice, it can help to conceptualize an `NFA` as a sequence of
instructions for a virtual machine. Each `State` says what to do and where
to go next.

Strictly speaking, the practical interpretation is the most correct one,
because of the [`Capture`](State::Capture) state. Namely, a `Capture`
state always forwards execution to another state unconditionally. Its only
purpose is to cause a side effect: the recording of the current input
position at a particular location in memory. In this sense, an `NFA`
has more power than a theoretical non-deterministic finite automaton.

For most uses of this crate, it is likely that one may never even need to
be aware of this type at all. The main use cases for looking at `State`s
directly are if you need to write your own search implementation or if you
need to do some kind of analysis on the NFA.

#### Variants

- **`ByteRange`**

  A state with a single transition that can only be taken if the current
  input symbol is in a particular range of bytes.

- **`Sparse`**

  A state with possibly many transitions represented in a sparse fashion.
  Transitions are non-overlapping and ordered lexicographically by input
  range.
  
  In practice, this is used for encoding UTF-8 automata. Its presence is
  primarily an optimization that avoids many additional unconditional
  epsilon transitions (via [`Union`](State::Union) states), and thus
  decreases the overhead of traversing the NFA. This can improve both
  matching time and DFA construction time.

- **`Dense`**

  A dense representation of a state with multiple transitions.

- **`Look`**

  A conditional epsilon transition satisfied via some sort of
  look-around. Look-around is limited to anchor and word boundary
  assertions.
  
  Look-around states are meant to be evaluated while performing epsilon
  closure (computing the set of states reachable from a particular state
  via only epsilon transitions). If the current position in the haystack
  satisfies the look-around assertion, then you're permitted to follow
  that epsilon transition.

- **`Union`**

  An alternation such that there exists an epsilon transition to all
  states in `alternates`, where matches found via earlier transitions
  are preferred over later transitions.

- **`BinaryUnion`**

  An alternation such that there exists precisely two unconditional
  epsilon transitions, where matches found via `alt1` are preferred over
  matches found via `alt2`.
  
  This state exists as a common special case of Union where there are
  only two alternates. In this case, we don't need any allocations to
  represent the state. This saves a bit of memory and also saves an
  additional memory access when traversing the NFA.

- **`Capture`**

  An empty state that records a capture location.
  
  From the perspective of finite automata, this is precisely equivalent
  to an unconditional epsilon transition, but serves the purpose of
  instructing NFA simulations to record additional state when the finite
  state machine passes through this epsilon transition.
  
  `slot` in this context refers to the specific capture group slot
  offset that is being recorded. Each capturing group has two slots
  corresponding to the start and end of the matching portion of that
  group.
  
  The pattern ID and capture group index are also included in this state
  in case they are useful. But mostly, all you'll need is `next` and
  `slot`.

- **`Fail`**

  A state that cannot be transitioned out of. This is useful for cases
  where you want to prevent matching from occurring. For example, if your
  regex parser permits empty character classes, then one could choose
  a `Fail` state to represent them. (An empty character class can be
  thought of as an empty set. Since nothing is in an empty set, they can
  never match anything.)

- **`Match`**

  A match state. There is at least one such occurrence of this state for
  each regex that can match that is in this NFA.

#### Implementations

- <span id="state-is-epsilon"></span>`fn is_epsilon(&self) -> bool`

  Returns true if and only if this state contains one or more epsilon

  transitions.

  

  In practice, a state has no outgoing transitions (like `Match`), has

  only non-epsilon transitions (like `ByteRange`) or has only epsilon

  transitions (like `Union`).

  

  # Example

  

  ```rust

  use regex_automata::{

      nfa::thompson::{State, Transition},

      util::primitives::{PatternID, StateID, SmallIndex},

  };

  

  // Capture states are epsilon transitions.

  let state = State::Capture {

      next: StateID::ZERO,

      pattern_id: PatternID::ZERO,

      group_index: SmallIndex::ZERO,

      slot: SmallIndex::ZERO,

  };

  assert!(state.is_epsilon());

  

  // ByteRange states are not.

  let state = State::ByteRange {

      trans: Transition { start: b'a', end: b'z', next: StateID::ZERO },

  };

  assert!(!state.is_epsilon());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="state-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage of this NFA state in bytes.

- <span id="state-remap"></span>`fn remap(&mut self, remap: &[StateID])` — [`StateID`](../../../util/primitives/index.md#stateid)

  Remap the transitions in this state using the given map. Namely, the

  given map should be indexed according to the transitions currently

  in this state.

  

  This is used during the final phase of the NFA compiler, which turns

  its intermediate NFA into the final NFA.

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for State`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for State`

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

##### `impl ToOwned for State`

- <span id="state-toowned-type-owned"></span>`type Owned = T`

- <span id="state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

