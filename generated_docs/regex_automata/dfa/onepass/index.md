*[regex_automata](../../index.md) / [dfa](../index.md) / [onepass](index.md)*

---

# Module `onepass`

A DFA that can return spans for matching capturing groups.

This module is the home of a [one-pass DFA](DFA).

This module also contains a [`Builder`](#builder) and a [`Config`](#config) for building and
configuring a one-pass DFA.

## Contents

- [Structs](#structs)
  - [`Config`](#config)
  - [`Builder`](#builder)
  - [`InternalBuilder`](#internalbuilder)
  - [`DFA`](#dfa)
  - [`SparseTransitionIter`](#sparsetransitioniter)
  - [`Cache`](#cache)
  - [`Transition`](#transition)
  - [`PatternEpsilons`](#patternepsilons)
  - [`Epsilons`](#epsilons)
  - [`Slots`](#slots)
  - [`SlotsIter`](#slotsiter)
  - [`BuildError`](#builderror)
- [Enums](#enums)
  - [`BuildErrorKind`](#builderrorkind)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration used for building a [one-pass DFA](DFA). |
| [`Builder`](#builder) | struct | A builder for a [one-pass DFA](DFA). |
| [`InternalBuilder`](#internalbuilder) | struct | An internal builder for encapsulating the state necessary to build a one-pass DFA. |
| [`DFA`](#dfa) | struct | A one-pass DFA for executing a subset of anchored regex searches while resolving capturing groups. |
| [`SparseTransitionIter`](#sparsetransitioniter) | struct | An iterator over groups of consecutive equivalent transitions in a single state. |
| [`Cache`](#cache) | struct | A cache represents mutable state that a one-pass [`DFA`] requires during a search. |
| [`Transition`](#transition) | struct | Represents a single transition in a one-pass DFA. |
| [`PatternEpsilons`](#patternepsilons) | struct | A representation of a match state's pattern ID along with the epsilons for when a match occurs. |
| [`Epsilons`](#epsilons) | struct | Epsilons represents all of the NFA epsilons transitions that went into a single transition in a single DFA state. |
| [`Slots`](#slots) | struct | The set of epsilon transitions indicating that the current position in a search should be saved to a slot. |
| [`SlotsIter`](#slotsiter) | struct | An iterator over all of the bits set in a slot set. |
| [`BuildError`](#builderror) | struct | An error that occurred during the construction of a one-pass DFA. |
| [`BuildErrorKind`](#builderrorkind) | enum | The kind of error that occurred during the construction of a one-pass DFA. |

## Structs

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:67-72`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L67-L72)*

The configuration used for building a [one-pass DFA](DFA).

A one-pass DFA configuration is a simple data object that is typically used
with `Builder::configure`. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with `DFA::config`.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

  Return a new default one-pass DFA configuration.

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md#matchkind), [`Config`](#config)

  Set the desired match semantics.

  

  The default is [`MatchKind::LeftmostFirst`](../../index.md), which corresponds to the

  match semantics of Perl-like regex engines. That is, when multiple

  patterns would match at the same leftmost position, the pattern that

  appears first in the concrete syntax is chosen.

  

  Currently, the only other kind of match semantics supported is

  [`MatchKind::All`](../../index.md). This corresponds to "classical DFA" construction

  where all possible matches are visited.

  

  When it comes to the one-pass DFA, it is rarer for preference order and

  "longest match" to actually disagree. Since if they did disagree, then

  the regex typically isn't one-pass. For example, searching `Samwise`

  for `Sam|Samwise` will report `Sam` for leftmost-first matching and

  `Samwise` for "longest match" or "all" matching. However, this regex is

  not one-pass if taken literally. The equivalent regex, `Sam(?:|wise)`

  is one-pass and `Sam|Samwise` may be optimized to it.

  

  The other main difference is that "all" match semantics don't support

  non-greedy matches. "All" match semantics always try to match as much

  as possible.

- <span id="config-starts-for-each-pattern"></span>`fn starts_for_each_pattern(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to compile a separate start state for each pattern in the

  one-pass DFA.

  

  When enabled, a separate **anchored** start state is added for each

  pattern in the DFA. When this start state is used, then the DFA will

  only search for matches for the pattern specified, even if there are

  other patterns in the DFA.

  

  The main downside of this option is that it can potentially increase

  the size of the DFA and/or increase the time it takes to build the DFA.

  

  You might want to enable this option when you want to both search for

  anchored matches of any pattern or to search for anchored matches of

  one particular pattern while using the same DFA. (Otherwise, you would

  need to compile a new DFA for each pattern.)

  

  By default this is disabled.

  

  # Example

  

  This example shows how to build a multi-regex and then search for

  matches for a any of the patterns or matches for a specific pattern.

  

  ```rust

  use regex_automata::{

      dfa::onepass::DFA, Anchored, Input, Match, PatternID,

  };

  

  let re = DFA::builder()

      .configure(DFA::config().starts_for_each_pattern(true))

      .build_many(&["[a-z]+", "[0-9]+"])?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  let haystack = "123abc";

  let input = Input::new(haystack).anchored(Anchored::Yes);

  

  // A normal multi-pattern search will show pattern 1 matches.

  re.try_search(&mut cache, &input, &mut caps)?;

  assert_eq!(Some(Match::must(1, 0..3)), caps.get_match());

  

  // If we only want to report pattern 0 matches, then we'll get no

  // match here.

  let input = input.anchored(Anchored::Pattern(PatternID::must(0)));

  re.try_search(&mut cache, &input, &mut caps)?;

  assert_eq!(None, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-byte-classes"></span>`fn byte_classes(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to attempt to shrink the size of the DFA's alphabet or not.

  

  This option is enabled by default and should never be disabled unless

  one is debugging a one-pass DFA.

  

  When enabled, the DFA will use a map from all possible bytes to their

  corresponding equivalence class. Each equivalence class represents a

  set of bytes that does not discriminate between a match and a non-match

  in the DFA. For example, the pattern `[ab]+` has at least two

  equivalence classes: a set containing `a` and `b` and a set containing

  every byte except for `a` and `b`. `a` and `b` are in the same

  equivalence class because they never discriminate between a match and a

  non-match.

  

  The advantage of this map is that the size of the transition table

  can be reduced drastically from (approximately) `#states * 256 *

  sizeof(StateID)` to `#states * k * sizeof(StateID)` where `k` is the

  number of equivalence classes (rounded up to the nearest power of 2).

  As a result, total space usage can decrease substantially. Moreover,

  since a smaller alphabet is used, DFA compilation becomes faster as

  well.

  

  **WARNING:** This is only useful for debugging DFAs. Disabling this

  does not yield any speed advantages. Namely, even when this is

  disabled, a byte class map is still used while searching. The only

  difference is that every byte will be forced into its own distinct

  equivalence class. This is useful for debugging the actual generated

  transitions because it lets one see the transitions defined on actual

  bytes instead of the equivalence classes.

- <span id="config-size-limit"></span>`fn size_limit(self, limit: Option<usize>) -> Config` — [`Config`](#config)

  Set a size limit on the total heap used by a one-pass DFA.

  

  This size limit is expressed in bytes and is applied during

  construction of a one-pass DFA. If the DFA's heap usage exceeds

  this configured limit, then construction is stopped and an error is

  returned.

  

  The default is no limit.

  

  # Example

  

  This example shows a one-pass DFA that fails to build because of

  a configured size limit. This particular example also serves as a

  cautionary tale demonstrating just how big DFAs with large Unicode

  character classes can get.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{dfa::onepass::DFA, Match};

  

  // 6MB isn't enough!

  DFA::builder()

      .configure(DFA::config().size_limit(Some(6_000_000)))

      .build(r"\w{20}")

      .unwrap_err();

  

  // ... but 7MB probably is!

  // (Note that DFA sizes aren't necessarily stable between releases.)

  let re = DFA::builder()

      .configure(DFA::config().size_limit(Some(7_000_000)))

      .build(r"\w{20}")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  let haystack = "A".repeat(20);

  re.captures(&mut cache, &haystack, &mut caps);

  assert_eq!(Some(Match::must(0, 0..20)), caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  While one needs a little more than 3MB to represent `\w{20}`, it

  turns out that you only need a little more than 4KB to represent

  `(?-u:\w{20})`. So only use Unicode if you need it!

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../index.md#matchkind)

  Returns the match semantics set in this configuration.

- <span id="config-get-starts-for-each-pattern"></span>`fn get_starts_for_each_pattern(&self) -> bool`

  Returns whether this configuration has enabled anchored starting states

  for every pattern in the DFA.

- <span id="config-get-byte-classes"></span>`fn get_byte_classes(&self) -> bool`

  Returns whether this configuration has enabled byte classes or not.

  This is typically a debugging oriented option, as disabling it confers

  no speed benefit.

- <span id="config-get-size-limit"></span>`fn get_size_limit(&self) -> Option<usize>`

  Returns the DFA size limit of this configuration if one was set.

  The size limit is total number of bytes on the heap that a DFA is

  permitted to use. If the DFA exceeds this limit during construction,

  then construction is stopped and an error is returned.

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
    thompson: thompson::Compiler,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:335-339`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L335-L339)*

A builder for a [one-pass DFA](DFA).

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the DFA construction. This builder is different from a
general purpose regex builder in that it permits fine grain configuration
of the construction process. The trade off for this is complexity, and
the possibility of setting a configuration that might not make sense. For
example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* `thompson::Config::utf8` controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the NFA.
This is generally what you want for matching on arbitrary bytes.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    nfa::thompson,
    util::syntax,
    Match,
};

let re = DFA::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

let haystack = b"foo\xFFarzz\xE2\x98\xFF\n";
re.captures(&mut cache, haystack, &mut caps);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a one-pass DFA Config,
//  since that only impacts regexes that can
// produce matches of length 0.
assert_eq!(Some(Match::must(0, 0..8)), caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new one-pass DFA builder with the default configuration.

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

  Build a one-pass DFA from the given pattern.

  

  If there was a problem parsing or compiling the pattern, then an error

  is returned.

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

  Build a one-pass DFA from the given patterns.

  

  When matches are returned, the pattern ID corresponds to the index of

  the pattern in the slice given.

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: NFA) -> Result<DFA, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`DFA`](#dfa), [`BuildError`](#builderror)

  Build a DFA from the given NFA.

  

  # Example

  

  This example shows how to build a DFA if you already have an NFA in

  hand.

  

  ```rust

  use regex_automata::{dfa::onepass::DFA, nfa::thompson::NFA, Match};

  

  // This shows how to set non-default options for building an NFA.

  let nfa = NFA::compiler()

      .configure(NFA::config().shrink(true))

      .build(r"[a-z0-9]+")?;

  let re = DFA::builder().build_from_nfa(nfa)?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  re.captures(&mut cache, "foo123bar", &mut caps);

  assert_eq!(Some(Match::must(0, 0..9)), caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

  Apply the given one-pass DFA configuration options to this builder.

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md#config), [`Builder`](#builder)

  Set the syntax configuration for this builder using

  [`syntax::Config`](crate::util::syntax::Config).

  

  This permits setting things like case insensitivity, Unicode and multi

  line mode.

  

  These settings only apply when constructing a one-pass DFA directly

  from a pattern.

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/compiler/index.md#config), [`Builder`](#builder)

  Set the Thompson NFA configuration for this builder using

  [`nfa::thompson::Config`](crate::nfa::thompson::Config).

  

  This permits setting things like whether additional time should be

  spent shrinking the size of the NFA.

  

  These settings only apply when constructing a DFA directly from a

  pattern.

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

### `InternalBuilder<'a>`

```rust
struct InternalBuilder<'a> {
    dfa: DFA,
    uncompiled_nfa_ids: alloc::vec::Vec<crate::util::primitives::StateID>,
    nfa_to_dfa_id: alloc::vec::Vec<crate::util::primitives::StateID>,
    stack: alloc::vec::Vec<(crate::util::primitives::StateID, Epsilons)>,
    seen: crate::util::sparse_set::SparseSet,
    matched: bool,
    config: Config,
    nfa: &'a crate::nfa::thompson::NFA,
    classes: crate::util::alphabet::ByteClasses,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:475-520`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L475-L520)*

An internal builder for encapsulating the state necessary to build a
one-pass DFA. Typical use is just `InternalBuilder::new(..).build()`.

There is no separate pass for determining whether the NFA is one-pass or
not. We just try to build the DFA. If during construction we discover that
it is not one-pass, we bail out. This is likely to lead to some undesirable
expense in some cases, so it might make sense to try an identify common
patterns in the NFA that make it definitively not one-pass. That way, we
can avoid ever trying to build a one-pass DFA in the first place. For
example, '\w*\s' is not one-pass, and since '\w' is Unicode-aware by
default, it's probably not a trivial cost to try and build a one-pass DFA
for it and then fail.

Note that some (immutable) fields are duplicated here. For example, the
'nfa' and 'classes' fields are both in the 'DFA'. They are the same thing,
but we duplicate them because it makes composition easier below. Otherwise,
since the borrow checker can't see through method calls, the mutable borrow
we use to mutate the DFA winds up preventing borrowing from any other part
of the DFA, even though we aren't mutating those parts. We only do this
because the duplication is cheap.

#### Fields

- **`dfa`**: `DFA`

  The DFA we're building.

- **`uncompiled_nfa_ids`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  An unordered collection of NFA state IDs that we haven't yet tried to
  build into a DFA state yet.
  
  This collection does not ultimately wind up including every NFA state
  ID. Instead, each ID represents a "start" state for a sub-graph of the
  NFA. The set of NFA states we then use to build a DFA state consists
  of that "start" state and all states reachable from it via epsilon
  transitions.

- **`nfa_to_dfa_id`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  A map from NFA state ID to DFA state ID. This is useful for easily
  determining whether an NFA state has been used as a "starting" point
  to build a DFA state yet. If it hasn't, then it is mapped to DEAD,
  and since DEAD is specially added and never corresponds to any NFA
  state, it follows that a mapping to DEAD implies the NFA state has
  no corresponding DFA state yet.

- **`stack`**: `alloc::vec::Vec<(crate::util::primitives::StateID, Epsilons)>`

  A stack used to traverse the NFA states that make up a single DFA
  state. Traversal occurs until the stack is empty, and we only push to
  the stack when the state ID isn't in 'seen'. Actually, even more than
  that, if we try to push something on to this stack that is already in
  'seen', then we bail out on construction completely, since it implies
  that the NFA is not one-pass.

- **`seen`**: `crate::util::sparse_set::SparseSet`

  The set of NFA states that we've visited via 'stack'.

- **`matched`**: `bool`

  Whether a match NFA state has been observed while constructing a
  one-pass DFA state. Once a match state is seen, assuming we are using
  leftmost-first match semantics, then we don't add any more transitions
  to the DFA state we're building.

- **`config`**: `Config`

  The config passed to the builder.
  
  This is duplicated in dfa.config.

- **`nfa`**: `&'a crate::nfa::thompson::NFA`

  The NFA we're building a one-pass DFA from.
  
  This is duplicated in dfa.nfa.

- **`classes`**: `crate::util::alphabet::ByteClasses`

  The equivalence classes that make up the alphabet for this DFA>
  
  This is duplicated in dfa.classes.

#### Implementations

- <span id="internalbuilder-new"></span>`fn new(config: Config, nfa: &'a NFA) -> InternalBuilder<'a>` — [`Config`](#config), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`InternalBuilder`](#internalbuilder)

  Create a new builder with an initial empty DFA.

- <span id="internalbuilder-build"></span>`fn build(self) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

  Build the DFA from the NFA given to this builder. If the NFA is not

  one-pass, then return an error. An error may also be returned if a

  particular limit is exceeded. (Some limits, like the total heap memory

  used, are configurable. Others, like the total patterns or slots, are

  hard-coded based on representational limitations.)

- <span id="internalbuilder-shuffle-states"></span>`fn shuffle_states(&mut self)`

  Shuffle all match states to the end of the transition table and set

  'min_match_id' to the ID of the first such match state.

  

  The point of this is to make it extremely cheap to determine whether

  a state is a match state or not. We need to check on this on every

  transition during a search, so it being cheap is important. This

  permits us to check it by simply comparing two state identifiers, as

  opposed to looking for the pattern ID in the state's `PatternEpsilons`.

  (Which requires a memory load and some light arithmetic.)

- <span id="internalbuilder-compile-transition"></span>`fn compile_transition(&mut self, dfa_id: StateID, trans: &thompson::Transition, epsilons: Epsilons) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`Transition`](../../nfa/thompson/nfa/index.md#transition), [`Epsilons`](#epsilons), [`BuildError`](#builderror)

  Compile the given NFA transition into the DFA state given.

  

  'Epsilons' corresponds to any conditional epsilon transitions that need

  to be satisfied to follow this transition, and any slots that need to

  be saved if the transition is followed.

  

  If this transition indicates that the NFA is not one-pass, then

  this returns an error. (This occurs, for example, if the DFA state

  already has a transition defined for the same input symbols as the

  given transition, *and* the result of the old and new transitions is

  different.)

- <span id="internalbuilder-add-start-state"></span>`fn add_start_state(&mut self, pid: Option<PatternID>, nfa_id: StateID) -> Result<StateID, BuildError>` — [`PatternID`](../../util/primitives/index.md#patternid), [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](#builderror)

  Add a start state to the DFA corresponding to the given NFA starting

  state ID.

  

  If adding a state would blow any limits (configured or hard-coded),

  then an error is returned.

  

  If the starting state is an anchored state for a particular pattern,

  then callers must provide the pattern ID for that starting state.

  Callers must also ensure that the first starting state added is the

  start state for all patterns, and then each anchored starting state for

  each pattern (if necessary) added in order. Otherwise, this panics.

- <span id="internalbuilder-add-dfa-state-for-nfa-state"></span>`fn add_dfa_state_for_nfa_state(&mut self, nfa_id: StateID) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](#builderror)

  Add a new DFA state corresponding to the given NFA state. If adding a

  state would blow any limits (configured or hard-coded), then an error

  is returned. If a DFA state already exists for the given NFA state,

  then that DFA state's ID is returned and no new states are added.

  

  It is not expected that this routine is called for every NFA state.

  Instead, an NFA state ID will usually correspond to the "start" state

  for a sub-graph of the NFA, where all states in the sub-graph are

  reachable via epsilon transitions (conditional or unconditional). That

  sub-graph of NFA states is ultimately what produces a single DFA state.

- <span id="internalbuilder-add-empty-state"></span>`fn add_empty_state(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](#builderror)

  Unconditionally add a new empty DFA state. If adding it would exceed

  any limits (configured or hard-coded), then an error is returned. The

  ID of the new state is returned on success.

  

  The added state is *not* a match state.

- <span id="internalbuilder-stack-push"></span>`fn stack_push(&mut self, nfa_id: StateID, epsilons: Epsilons) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`Epsilons`](#epsilons), [`BuildError`](#builderror)

  Push the given NFA state ID and its corresponding epsilons (slots and

  conditional epsilon transitions) on to a stack for use in a depth first

  traversal of a sub-graph of the NFA.

  

  If the given NFA state ID has already been pushed on to the stack, then

  it indicates the regex is not one-pass and this correspondingly returns

  an error.

#### Trait Implementations

##### `impl Any for InternalBuilder<'a>`

- <span id="internalbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InternalBuilder<'a>`

- <span id="internalbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InternalBuilder<'a>`

- <span id="internalbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for InternalBuilder<'a>`

- <span id="internalbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for InternalBuilder<'a>`

- <span id="internalbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InternalBuilder<'a>`

- <span id="internalbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for InternalBuilder<'a>`

- <span id="internalbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="internalbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InternalBuilder<'a>`

- <span id="internalbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="internalbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DFA`

```rust
struct DFA {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
    table: alloc::vec::Vec<Transition>,
    starts: alloc::vec::Vec<crate::util::primitives::StateID>,
    min_match_id: crate::util::primitives::StateID,
    classes: crate::util::alphabet::ByteClasses,
    alphabet_len: usize,
    stride2: usize,
    pateps_offset: usize,
    explicit_slot_start: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:1077-1146`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L1077-L1146)*

A one-pass DFA for executing a subset of anchored regex searches while
resolving capturing groups.

A one-pass DFA can be built from an NFA that is one-pass. An NFA is
one-pass when there is never any ambiguity about how to continue a search.
For example, `a*a` is not one-pass because during a search, it's not
possible to know whether to continue matching the `a*` or to move on to
the single `a`. However, `a*b` is one-pass, because for every byte in the
input, it's always clear when to move on from `a*` to `b`.

# Only anchored searches are supported

In this crate, especially for DFAs, unanchored searches are implemented by
treating the pattern as if it had a `(?s-u:.)*?` prefix. While the prefix
is one-pass on its own, adding anything after it, e.g., `(?s-u:.)*?a` will
make the overall pattern not one-pass. Why? Because the `(?s-u:.)` matches
any byte, and there is therefore ambiguity as to when the prefix should
stop matching and something else should start matching.

Therefore, one-pass DFAs do not support unanchored searches. In addition
to many regexes simply not being one-pass, it implies that one-pass DFAs
have limited utility. With that said, when a one-pass DFA can be used, it
can potentially provide a dramatic speed up over alternatives like the
[`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)
and the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). In particular,
a one-pass DFA is the only DFA capable of reporting the spans of matching
capturing groups.

To clarify, when we say that unanchored searches are not supported, what
that actually means is:

* The high level routines, `DFA::is_match` and `DFA::captures`, always
do anchored searches.
* Since iterators are most useful in the context of unanchored searches,
there is no `DFA::captures_iter` method.
* For lower level routines like `DFA::try_search`, an error will be
returned if the given [`Input`](../../index.md) is configured to do an unanchored search or
search for an invalid pattern ID. (Note that an [`Input`](../../index.md) is configured to
do an unanchored search by default, so just giving a `Input::new` is
guaranteed to return an error.)

# Other limitations

In addition to the [configurable heap limit](Config::size_limit) and
the requirement that a regex pattern be one-pass, there are some other
limitations:

* There is an internal limit on the total number of explicit capturing
groups that appear across all patterns. It is somewhat small and there is
no way to configure it. If your pattern(s) exceed this limit, then building
a one-pass DFA will fail.
* If the number of patterns exceeds an internal unconfigurable limit, then
building a one-pass DFA will fail. This limit is quite large and you're
unlikely to hit it.
* If the total number of states exceeds an internal unconfigurable limit,
then building a one-pass DFA will fail. This limit is quite large and
you're unlikely to hit it.

# Other examples of regexes that aren't one-pass

One particularly unfortunate example is that enabling Unicode can cause
regexes that were one-pass to no longer be one-pass. Consider the regex
`(?-u)\w*\s` for example. It is one-pass because there is exactly no
overlap between the ASCII definitions of `\w` and `\s`. But `\w*\s`
(i.e., with Unicode enabled) is *not* one-pass because `\w` and `\s` get
translated to UTF-8 automatons. And while the *codepoints* in `\w` and `\s`
do not overlap, the underlying UTF-8 encodings do. Indeed, because of the
overlap between UTF-8 automata, the use of Unicode character classes will
tend to vastly increase the likelihood of a regex not being one-pass.

# How does one know if a regex is one-pass or not?

At the time of writing, the only way to know is to try and build a one-pass
DFA. The one-pass property is checked while constructing the DFA.

This does mean that you might potentially waste some CPU cycles and memory
by optimistically trying to build a one-pass DFA. But this is currently the
only way. In the future, building a one-pass DFA might be able to use some
heuristics to detect common violations of the one-pass property and bail
more quickly.

# Resource usage

Unlike a general DFA, a one-pass DFA has stricter bounds on its resource
usage. Namely, construction of a one-pass DFA has a time and space
complexity of `O(n)`, where `n ~ nfa.states().len()`. (A general DFA's time
and space complexity is `O(2^n)`.) This smaller time bound is achieved
because there is at most one DFA state created for each NFA state. If
additional DFA states would be required, then the pattern is not one-pass
and construction will fail.

Note though that currently, this DFA uses a fully dense representation.
This means that while its space complexity is no worse than an NFA, it may
in practice use more memory because of higher constant factors. The reason
for this trade off is two-fold. Firstly, a dense representation makes the
search faster. Secondly, the bigger an NFA, the more unlikely it is to be
one-pass. Therefore, most one-pass DFAs are usually pretty small.

# Example

This example shows that the one-pass DFA implements Unicode word boundaries
correctly while simultaneously reporting spans for capturing groups that
participate in a match. (This is the only DFA that implements full support
for Unicode word boundaries.)

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{dfa::onepass::DFA, Match, Span};

let re = DFA::new(r"\b(?P<first>\w+)[[:space:]]+(?P<last>\w+)\b")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "Шерлок Холмс", &mut caps);
assert_eq!(Some(Match::must(0, 0..23)), caps.get_match());
assert_eq!(Some(Span::from(0..12)), caps.get_group_by_name("first"));
assert_eq!(Some(Span::from(13..23)), caps.get_group_by_name("last"));
Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: iteration

Unlike other regex engines in this crate, this one does not provide
iterator search functions. This is because a one-pass DFA only supports
anchored searches, and so iterator functions are generally not applicable.

However, if you know that all of your matches are
directly adjacent, then an iterator can be used. The
[`util::iter::Searcher`](crate::util::iter::Searcher) type can be used for
this purpose:

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    util::iter::Searcher,
    Anchored, Input, Span,
};

let re = DFA::new(r"\w(\d)\w")?;
let (mut cache, caps) = (re.create_cache(), re.create_captures());
let input = Input::new("a1zb2yc3x").anchored(Anchored::Yes);

let mut it = Searcher::new(input).into_captures_iter(caps, |input, caps| {
    Ok(re.try_search(&mut cache, input, caps)?)
}).infallible();
let caps0 = it.next().unwrap();
assert_eq!(Some(Span::from(1..2)), caps0.get_group(1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`config`**: `Config`

  The configuration provided by the caller.

- **`nfa`**: `crate::nfa::thompson::NFA`

  The NFA used to build this DFA.
  
  NOTE: We probably don't need to store the NFA here, but we use enough
  bits from it that it's convenient to do so. And there really isn't much
  cost to doing so either, since an NFA is reference counted internally.

- **`table`**: `alloc::vec::Vec<Transition>`

  The transition table. Given a state ID 's' and a byte of haystack 'b',
  the next state is `table[sid + classes[byte]]`.
  
  The stride of this table (i.e., the number of columns) is always
  a power of 2, even if the alphabet length is smaller. This makes
  converting between state IDs and state indices very cheap.
  
  Note that the stride always includes room for one extra "transition"
  that isn't actually a transition. It is a 'PatternEpsilons' that is
  used for match states only. Because of this, the maximum number of
  active columns in the transition table is 257, which means the maximum
  stride is 512 (the next power of 2 greater than or equal to 257).

- **`starts`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The DFA state IDs of the starting states.
  
  `starts[0]` is always present and corresponds to the starting state
  when searching for matches of any pattern in the DFA.
  
  `starts[i]` where i>0 corresponds to the starting state for the pattern
  ID 'i-1'. These starting states are optional.

- **`min_match_id`**: `crate::util::primitives::StateID`

  Every state ID >= this value corresponds to a match state.
  
  This is what a search uses to detect whether a state is a match state
  or not. It requires only a simple comparison instead of bit-unpacking
  the PatternEpsilons from every state.

- **`classes`**: `crate::util::alphabet::ByteClasses`

  The alphabet of this DFA, split into equivalence classes. Bytes in the
  same equivalence class can never discriminate between a match and a
  non-match.

- **`alphabet_len`**: `usize`

  The number of elements in each state in the transition table. This may
  be less than the stride, since the stride is always a power of 2 and
  the alphabet length can be anything up to and including 256.

- **`stride2`**: `usize`

  The number of columns in the transition table, expressed as a power of
  2.

- **`pateps_offset`**: `usize`

  The offset at which the PatternEpsilons for a match state is stored in
  the transition table.
  
  PERF: One wonders whether it would be better to put this in a separate
  allocation, since only match states have a non-empty PatternEpsilons
  and the number of match states tends be dwarfed by the number of
  non-match states. So this would save '8*len(non_match_states)' for each
  DFA. The question is whether moving this to a different allocation will
  lead to a perf hit during searches. You might think dealing with match
  states is rare, but some regexes spend a lot of time in match states
  gobbling up input. But... match state handling is already somewhat
  expensive, so maybe this wouldn't do much? Either way, it's worth
  experimenting.

- **`explicit_slot_start`**: `usize`

  The first explicit slot index. This refers to the first slot appearing
  immediately after the last implicit slot. It is always 'patterns.len()
  * 2'.
  
  We record this because we only store the explicit slots in our DFA
  transition table that need to be saved. Implicit slots are handled
  automatically as part of the search.

#### Implementations

- <span id="dfa-new"></span>`fn new(pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

  Parse the given regular expression using the default configuration and

  return the corresponding one-pass DFA.

  

  If you want a non-default configuration, then use the [`Builder`](#builder) to

  set your own configuration.

  

  # Example

  

  ```rust

  use regex_automata::{dfa::onepass::DFA, Match};

  

  let re = DFA::new("foo[0-9]+bar")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  re.captures(&mut cache, "foo12345barzzz", &mut caps);

  assert_eq!(Some(Match::must(0, 0..11)), caps.get_match());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

  Like `new`, but parses multiple patterns into a single "multi regex."

  This similarly uses the default regex configuration.

  

  # Example

  

  ```rust

  use regex_automata::{dfa::onepass::DFA, Match};

  

  let re = DFA::new_many(&["[a-z]+", "[0-9]+"])?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  re.captures(&mut cache, "abc123", &mut caps);

  assert_eq!(Some(Match::must(0, 0..3)), caps.get_match());

  

  re.captures(&mut cache, "123abc", &mut caps);

  assert_eq!(Some(Match::must(1, 0..3)), caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-new-from-nfa"></span>`fn new_from_nfa(nfa: NFA) -> Result<DFA, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`DFA`](#dfa), [`BuildError`](#builderror)

  Like `new`, but builds a one-pass DFA directly from an NFA. This is

  useful if you already have an NFA, or even if you hand-assembled the

  NFA.

  

  # Example

  

  This shows how to hand assemble a regular expression via its HIR,

  compile an NFA from it and build a one-pass DFA from the NFA.

  

  ```rust

  use regex_automata::{

      dfa::onepass::DFA,

      nfa::thompson::NFA,

      Match,

  };

  use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

  

  let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![

      ClassBytesRange::new(b'0', b'9'),

      ClassBytesRange::new(b'A', b'Z'),

      ClassBytesRange::new(b'_', b'_'),

      ClassBytesRange::new(b'a', b'z'),

  ])));

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

  

  let re = DFA::new_from_nfa(nfa)?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  let expected = Some(Match::must(0, 0..1));

  re.captures(&mut cache, "A", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-always-match"></span>`fn always_match() -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

  Create a new one-pass DFA that matches every input.

  

  # Example

  

  ```rust

  use regex_automata::{dfa::onepass::DFA, Match};

  

  let dfa = DFA::always_match()?;

  let mut cache = dfa.create_cache();

  let mut caps = dfa.create_captures();

  

  let expected = Match::must(0, 0..0);

  dfa.captures(&mut cache, "", &mut caps);

  assert_eq!(Some(expected), caps.get_match());

  dfa.captures(&mut cache, "foo", &mut caps);

  assert_eq!(Some(expected), caps.get_match());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-never-match"></span>`fn never_match() -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

  Create a new one-pass DFA that never matches any input.

  

  # Example

  

  ```rust

  use regex_automata::dfa::onepass::DFA;

  

  let dfa = DFA::never_match()?;

  let mut cache = dfa.create_cache();

  let mut caps = dfa.create_captures();

  

  dfa.captures(&mut cache, "", &mut caps);

  assert_eq!(None, caps.get_match());

  dfa.captures(&mut cache, "foo", &mut caps);

  assert_eq!(None, caps.get_match());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-config"></span>`fn config() -> Config` — [`Config`](#config)

  Return a default configuration for a DFA.

  

  This is a convenience routine to avoid needing to import the `Config`

  type when customizing the construction of a DFA.

  

  # Example

  

  This example shows how to change the match semantics of this DFA from

  its default "leftmost first" to "all." When using "all," non-greediness

  doesn't apply and neither does preference order matching. Instead, the

  longest match possible is always returned. (Although, by construction,

  it's impossible for a one-pass DFA to have a different answer for

  "preference order" vs "longest match.")

  

  ```rust

  use regex_automata::{dfa::onepass::DFA, Match, MatchKind};

  

  let re = DFA::builder()

      .configure(DFA::config().match_kind(MatchKind::All))

      .build(r"(abc)+?")?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  

  re.captures(&mut cache, "abcabc", &mut caps);

  // Normally, the non-greedy repetition would give us a 0..3 match.

  assert_eq!(Some(Match::must(0, 0..6)), caps.get_match());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  Return a builder for configuring the construction of a DFA.

  

  This is a convenience routine to avoid needing to import the

  [`Builder`](#builder) type in common cases.

  

  # Example

  

  This example shows how to use the builder to disable UTF-8 mode.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      dfa::onepass::DFA,

      nfa::thompson,

      util::syntax,

      Match,

  };

  

  let re = DFA::builder()

      .syntax(syntax::Config::new().utf8(false))

      .thompson(thompson::Config::new().utf8(false))

      .build(r"foo(?-u:[^b])ar.*")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let haystack = b"foo\xFFarzz\xE2\x98\xFF\n";

  let expected = Some(Match::must(0, 0..8));

  re.captures(&mut cache, haystack, &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-create-captures"></span>`fn create_captures(&self) -> Captures` — [`Captures`](../../util/captures/index.md#captures)

  Create a new empty set of capturing groups that is guaranteed to be

  valid for the search APIs on this DFA.

  

  A `Captures` value created for a specific DFA cannot be used with any

  other DFA.

  

  This is a convenience function for `Captures::all`. See the

  [`Captures`](../../util/captures/index.md) documentation for an explanation of its alternative

  constructors that permit the DFA to do less work during a search, and

  thus might make it faster.

- <span id="dfa-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](#cache)

  Create a new cache for this DFA.

  

  The cache returned should only be used for searches for this

  DFA. If you want to reuse the cache for another DFA, then you

  must call `Cache::reset` with that DFA (or, equivalently,

  `DFA::reset_cache`).

- <span id="dfa-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](#cache)

  Reset the given cache such that it can be used for searching with the

  this DFA (and only this DFA).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different DFA.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different DFA.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{dfa::onepass::DFA, Match};

  

  let re1 = DFA::new(r"\w")?;

  let re2 = DFA::new(r"\W")?;

  let mut caps1 = re1.create_captures();

  let mut caps2 = re2.create_captures();

  

  let mut cache = re1.create_cache();

  assert_eq!(

      Some(Match::must(0, 0..2)),

      { re1.captures(&mut cache, "Δ", &mut caps1); caps1.get_match() },

  );

  

  // Using 'cache' with re2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the one-pass DFA we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 're1' is also not

  // allowed.

  re2.reset_cache(&mut cache);

  assert_eq!(

      Some(Match::must(0, 0..3)),

      { re2.captures(&mut cache, "☃", &mut caps2); caps2.get_match() },

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-get-config"></span>`fn get_config(&self) -> &Config` — [`Config`](#config)

  Return the config for this one-pass DFA.

- <span id="dfa-get-nfa"></span>`fn get_nfa(&self) -> &NFA` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa)

  Returns a reference to the underlying NFA.

- <span id="dfa-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the total number of patterns compiled into this DFA.

  

  In the case of a DFA that contains no patterns, this returns `0`.

- <span id="dfa-state-len"></span>`fn state_len(&self) -> usize`

  Returns the total number of states in this one-pass DFA.

  

  Note that unlike dense or sparse DFAs, a one-pass DFA does not expose

  a low level DFA API. Therefore, this routine has little use other than

  being informational.

- <span id="dfa-alphabet-len"></span>`fn alphabet_len(&self) -> usize`

  Returns the total number of elements in the alphabet for this DFA.

  

  That is, this returns the total number of transitions that each

  state in this DFA must have. The maximum alphabet size is 256, which

  corresponds to each possible byte value.

  

  The alphabet size may be less than 256 though, and unless

  `Config::byte_classes` is disabled, it is typically must less than

  256. Namely, bytes are grouped into equivalence classes such that no

  two bytes in the same class can distinguish a match from a non-match.

  For example, in the regex `^[a-z]+$`, the ASCII bytes `a-z` could

  all be in the same equivalence class. This leads to a massive space

  savings.

  

  Note though that the alphabet length does _not_ necessarily equal the

  total stride space taken up by a single DFA state in the transition

  table. Namely, for performance reasons, the stride is always the

  smallest power of two that is greater than or equal to the alphabet

  length. For this reason, `DFA::stride` or `DFA::stride2` are

  often more useful. The alphabet length is typically useful only for

  informational purposes.

  

  Note also that unlike dense or sparse DFAs, a one-pass DFA does

  not have a special end-of-input (EOI) transition. This is because

  a one-pass DFA handles look-around assertions explicitly (like the

  [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM)) and does not build

  them into the transitions of the DFA.

- <span id="dfa-stride2"></span>`fn stride2(&self) -> usize`

  Returns the total stride for every state in this DFA, expressed as the

  exponent of a power of 2. The stride is the amount of space each state

  takes up in the transition table, expressed as a number of transitions.

  (Unused transitions map to dead states.)

  

  The stride of a DFA is always equivalent to the smallest power of

  2 that is greater than or equal to the DFA's alphabet length. This

  definition uses extra space, but possibly permits faster translation

  between state identifiers and their corresponding offsets in this DFA's

  transition table.

  

  For example, if the DFA's stride is 16 transitions, then its `stride2`

  is `4` since `2^4 = 16`.

  

  The minimum `stride2` value is `1` (corresponding to a stride of `2`)

  while the maximum `stride2` value is `9` (corresponding to a stride

  of `512`). The maximum in theory should be `8`, but because of some

  implementation quirks that may be relaxed in the future, it is one more

  than `8`. (Do note that a maximal stride is incredibly rare, as it

  would imply that there is almost no redundant in the regex pattern.)

  

  Note that unlike dense or sparse DFAs, a one-pass DFA does not expose

  a low level DFA API. Therefore, this routine has little use other than

  being informational.

- <span id="dfa-stride"></span>`fn stride(&self) -> usize`

  Returns the total stride for every state in this DFA. This corresponds

  to the total number of transitions used by each state in this DFA's

  transition table.

  

  Please see `DFA::stride2` for more information. In particular, this

  returns the stride as the number of transitions, where as `stride2`

  returns it as the exponent of a power of 2.

  

  Note that unlike dense or sparse DFAs, a one-pass DFA does not expose

  a low level DFA API. Therefore, this routine has little use other than

  being informational.

- <span id="dfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the memory usage, in bytes, of this DFA.

  

  The memory usage is computed based on the number of bytes used to

  represent this DFA.

  

  This does **not** include the stack size used up by this DFA. To

  compute that, use `std::mem::size_of::<onepass::DFA>()`.

#### Trait Implementations

##### `impl Any for DFA`

- <span id="dfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DFA`

- <span id="dfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DFA`

- <span id="dfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DFA`

- <span id="dfa-clone"></span>`fn clone(&self) -> DFA` — [`DFA`](#dfa)

##### `impl CloneToUninit for DFA`

- <span id="dfa-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DFA`

- <span id="dfa-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for DFA`

- <span id="dfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DFA`

- <span id="dfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Remappable for crate::dfa::onepass::DFA`

- <span id="cratedfaonepassdfa-remappable-state-len"></span>`fn state_len(&self) -> usize`

- <span id="cratedfaonepassdfa-remappable-stride2"></span>`fn stride2(&self) -> usize`

- <span id="cratedfaonepassdfa-remappable-swap-states"></span>`fn swap_states(&mut self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="cratedfaonepassdfa-remappable-remap"></span>`fn remap(&mut self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

##### `impl ToOwned for DFA`

- <span id="dfa-toowned-type-owned"></span>`type Owned = T`

- <span id="dfa-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dfa-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DFA`

- <span id="dfa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dfa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DFA`

- <span id="dfa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dfa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SparseTransitionIter<'a>`

```rust
struct SparseTransitionIter<'a> {
    it: core::iter::Enumerate<core::slice::Iter<'a, Transition>>,
    cur: Option<(u8, u8, Transition)>,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2442-2445`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2442-L2445)*

An iterator over groups of consecutive equivalent transitions in a single
state.

#### Trait Implementations

##### `impl Any for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sparsetransitioniter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sparsetransitioniter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-iterator-type-item"></span>`type Item = (u8, u8, Transition)`

- <span id="sparsetransitioniter-iterator-next"></span>`fn next(&mut self) -> Option<(u8, u8, Transition)>` — [`Transition`](#transition)

##### `impl<U> TryFrom for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sparsetransitioniter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sparsetransitioniter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cache`

```rust
struct Cache {
    explicit_slots: alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>,
    explicit_slot_len: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2492-2504`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2492-L2504)*

A cache represents mutable state that a one-pass [`DFA`](#dfa) requires during a
search.

For a given one-pass DFA, its corresponding cache may be created either via
`DFA::create_cache`, or via `Cache::new`. They are equivalent in every
way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the one-pass DFA from which it was
created. It may only be used with that one-pass DFA. A cache and its
allocations may be re-purposed via `Cache::reset`, in which case, it can
only be used with the new one-pass DFA (and not the old one).

#### Fields

- **`explicit_slots`**: `alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>`

  Scratch space used to store slots during a search. Basically, we use
  the caller provided slots to store slots known when a match occurs.
  But after a match occurs, we might continue a search but ultimately
  fail to extend the match. When continuing the search, we need some
  place to store candidate capture offsets without overwriting the slot
  offsets recorded for the most recently seen match.

- **`explicit_slot_len`**: `usize`

  The number of slots in the caller-provided 'Captures' value for the
  current search. This is always at most 'explicit_slots.len()', but
  might be less than it, if the caller provided fewer slots to fill.

#### Implementations

- <span id="cache-new"></span>`fn new(re: &DFA) -> Cache` — [`DFA`](#dfa), [`Cache`](#cache)

  Create a new [`onepass::DFA`](DFA) cache.

  

  A potentially more convenient routine to create a cache is

  `DFA::create_cache`, as it does not require also importing the

  `Cache` type.

  

  If you want to reuse the returned `Cache` with some other one-pass DFA,

  then you must call `Cache::reset` with the desired one-pass DFA.

- <span id="cache-reset"></span>`fn reset(&mut self, re: &DFA)` — [`DFA`](#dfa)

  Reset this cache such that it can be used for searching with a

  different [`onepass::DFA`](DFA).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different one-pass DFA.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different one-pass

  DFA.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{dfa::onepass::DFA, Match};

  

  let re1 = DFA::new(r"\w")?;

  let re2 = DFA::new(r"\W")?;

  let mut caps1 = re1.create_captures();

  let mut caps2 = re2.create_captures();

  

  let mut cache = re1.create_cache();

  assert_eq!(

      Some(Match::must(0, 0..2)),

      { re1.captures(&mut cache, "Δ", &mut caps1); caps1.get_match() },

  );

  

  // Using 'cache' with re2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the one-pass DFA we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 're1' is also not

  // allowed.

  re2.reset_cache(&mut cache);

  assert_eq!(

      Some(Match::must(0, 0..3)),

      { re2.captures(&mut cache, "☃", &mut caps2); caps2.get_match() },

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, of this cache.

  

  This does **not** include the stack size used up by this cache. To

  compute that, use `std::mem::size_of::<Cache>()`.

- <span id="cache-explicit-slots"></span>`fn explicit_slots(&mut self) -> &mut [Option<NonMaxUsize>]` — [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize)

- <span id="cache-setup-search"></span>`fn setup_search(&mut self, explicit_slot_len: usize)`

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

### `Transition`

```rust
struct Transition(u64);
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2592`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2592)*

Represents a single transition in a one-pass DFA.

The high 21 bits corresponds to the state ID. The bit following corresponds
to the special "match wins" flag. The remaining low 42 bits corresponds to
the transition epsilons, which contains the slots that should be saved when
this transition is followed and the conditional epsilon transitions that
must be satisfied in order to follow this transition.

#### Implementations

- <span id="transition-const-state-id-bits"></span>`const STATE_ID_BITS: u64`

- <span id="transition-const-state-id-shift"></span>`const STATE_ID_SHIFT: u64`

- <span id="transition-const-state-id-limit"></span>`const STATE_ID_LIMIT: u64`

- <span id="transition-const-match-wins-shift"></span>`const MATCH_WINS_SHIFT: u64`

- <span id="transition-const-info-mask"></span>`const INFO_MASK: u64`

- <span id="transition-new"></span>`fn new(match_wins: bool, sid: StateID, epsilons: Epsilons) -> Transition` — [`StateID`](../../util/primitives/index.md#stateid), [`Epsilons`](#epsilons), [`Transition`](#transition)

  Return a new transition to the given state ID with the given epsilons.

- <span id="transition-is-dead"></span>`fn is_dead(self) -> bool`

  Returns true if and only if this transition points to the DEAD state.

- <span id="transition-match-wins"></span>`fn match_wins(&self) -> bool`

  Return whether this transition has a "match wins" property.

  

  When a transition has this property, it means that if a match has been

  found and the search uses leftmost-first semantics, then that match

  should be returned immediately instead of continuing on.

  

  The "match wins" name comes from RE2, which uses a pretty much

  identical mechanism for implementing leftmost-first semantics.

- <span id="transition-state-id"></span>`fn state_id(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Return the "next" state ID that this transition points to.

- <span id="transition-set-state-id"></span>`fn set_state_id(&mut self, sid: StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

  Set the "next" state ID in this transition.

- <span id="transition-epsilons"></span>`fn epsilons(&self) -> Epsilons` — [`Epsilons`](#epsilons)

  Return the epsilons embedded in this transition.

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

- <span id="transition-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Transition`

##### `impl<T> From for Transition`

- <span id="transition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

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

### `PatternEpsilons`

```rust
struct PatternEpsilons(u64);
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2679`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2679)*

A representation of a match state's pattern ID along with the epsilons for
when a match occurs.

A match state in a one-pass DFA, unlike in a more general DFA, has exactly
one pattern ID. If it had more, then the original NFA would not have been
one-pass.

The "epsilons" part of this corresponds to what was found in the epsilon
transitions between the transition taken in the last byte of input and the
ultimate match state. This might include saving slots and/or conditional
epsilon transitions that must be satisfied before one can report the match.

Technically, every state has room for a 'PatternEpsilons', but it is only
ever non-empty for match states.

#### Implementations

- <span id="patternepsilons-const-pattern-id-bits"></span>`const PATTERN_ID_BITS: u64`

- <span id="patternepsilons-const-pattern-id-shift"></span>`const PATTERN_ID_SHIFT: u64`

- <span id="patternepsilons-const-pattern-id-none"></span>`const PATTERN_ID_NONE: u64`

- <span id="patternepsilons-const-pattern-id-limit"></span>`const PATTERN_ID_LIMIT: u64`

- <span id="patternepsilons-const-pattern-id-mask"></span>`const PATTERN_ID_MASK: u64`

- <span id="patternepsilons-const-epsilons-mask"></span>`const EPSILONS_MASK: u64`

- <span id="patternepsilons-empty"></span>`fn empty() -> PatternEpsilons` — [`PatternEpsilons`](#patternepsilons)

  Return a new empty pattern epsilons that has no pattern ID and has no

  epsilons. This is suitable for non-match states.

- <span id="patternepsilons-is-empty"></span>`fn is_empty(self) -> bool`

  Whether this pattern epsilons is empty or not. It's empty when it has

  no pattern ID and an empty epsilons.

- <span id="patternepsilons-pattern-id"></span>`fn pattern_id(self) -> Option<PatternID>` — [`PatternID`](../../util/primitives/index.md#patternid)

  Return the pattern ID in this pattern epsilons if one exists.

- <span id="patternepsilons-pattern-id-unchecked"></span>`fn pattern_id_unchecked(self) -> PatternID` — [`PatternID`](../../util/primitives/index.md#patternid)

  Returns the pattern ID without checking whether it's valid. If this is

  called and there is no pattern ID in this `PatternEpsilons`, then this

  will likely produce an incorrect result or possibly even a panic or

  an overflow. But safety will not be violated.

  

  This is useful when you know a particular state is a match state. If

  it's a match state, then it must have a pattern ID.

- <span id="patternepsilons-set-pattern-id"></span>`fn set_pattern_id(self, pid: PatternID) -> PatternEpsilons` — [`PatternID`](../../util/primitives/index.md#patternid), [`PatternEpsilons`](#patternepsilons)

  Return a new pattern epsilons with the given pattern ID, but the same

  epsilons.

- <span id="patternepsilons-epsilons"></span>`fn epsilons(self) -> Epsilons` — [`Epsilons`](#epsilons)

  Return the epsilons part of this pattern epsilons.

- <span id="patternepsilons-set-epsilons"></span>`fn set_epsilons(self, epsilons: Epsilons) -> PatternEpsilons` — [`Epsilons`](#epsilons), [`PatternEpsilons`](#patternepsilons)

  Return a new pattern epsilons with the given epsilons, but the same

  pattern ID.

#### Trait Implementations

##### `impl Any for PatternEpsilons`

- <span id="patternepsilons-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternEpsilons`

- <span id="patternepsilons-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternEpsilons`

- <span id="patternepsilons-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PatternEpsilons`

- <span id="patternepsilons-clone"></span>`fn clone(&self) -> PatternEpsilons` — [`PatternEpsilons`](#patternepsilons)

##### `impl CloneToUninit for PatternEpsilons`

- <span id="patternepsilons-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PatternEpsilons`

##### `impl Debug for PatternEpsilons`

- <span id="patternepsilons-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for PatternEpsilons`

- <span id="patternepsilons-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PatternEpsilons`

- <span id="patternepsilons-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PatternEpsilons`

- <span id="patternepsilons-toowned-type-owned"></span>`type Owned = T`

- <span id="patternepsilons-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patternepsilons-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PatternEpsilons`

- <span id="patternepsilons-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patternepsilons-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternEpsilons`

- <span id="patternepsilons-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patternepsilons-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Epsilons`

```rust
struct Epsilons(u64);
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2787`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2787)*

Epsilons represents all of the NFA epsilons transitions that went into a
single transition in a single DFA state. In this case, it only represents
the epsilon transitions that have some kind of non-consuming side effect:
either the transition requires storing the current position of the search
into a slot, or the transition is conditional and requires the current
position in the input to satisfy an assertion before the transition may be
taken.

This folds the cumulative effect of a group of NFA states (all connected
by epsilon transitions) down into a single set of bits. While these bits
can represent all possible conditional epsilon transitions, it only permits
storing up to a somewhat small number of slots.

Epsilons is represented as a 42-bit integer. For example, it is packed into
the lower 42 bits of a `Transition`. (Where the high 22 bits contains a
`StateID` and a special "match wins" property.)

#### Implementations

- <span id="epsilons-const-slot-mask"></span>`const SLOT_MASK: u64`

- <span id="epsilons-const-slot-shift"></span>`const SLOT_SHIFT: u64`

- <span id="epsilons-const-look-mask"></span>`const LOOK_MASK: u64`

- <span id="epsilons-empty"></span>`fn empty() -> Epsilons` — [`Epsilons`](#epsilons)

  Create a new empty epsilons. It has no slots and no assertions that

  need to be satisfied.

- <span id="epsilons-is-empty"></span>`fn is_empty(self) -> bool`

  Returns true if this epsilons contains no slots and no assertions.

- <span id="epsilons-slots"></span>`fn slots(self) -> Slots` — [`Slots`](#slots)

  Returns the slot epsilon transitions.

- <span id="epsilons-set-slots"></span>`fn set_slots(self, slots: Slots) -> Epsilons` — [`Slots`](#slots), [`Epsilons`](#epsilons)

  Set the slot epsilon transitions.

- <span id="epsilons-looks"></span>`fn looks(self) -> LookSet` — [`LookSet`](../../util/look/index.md#lookset)

  Return the set of look-around assertions in these epsilon transitions.

- <span id="epsilons-set-looks"></span>`fn set_looks(self, look_set: LookSet) -> Epsilons` — [`LookSet`](../../util/look/index.md#lookset), [`Epsilons`](#epsilons)

  Set the look-around assertions on these epsilon transitions.

#### Trait Implementations

##### `impl Any for Epsilons`

- <span id="epsilons-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Epsilons`

- <span id="epsilons-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Epsilons`

- <span id="epsilons-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Epsilons`

- <span id="epsilons-clone"></span>`fn clone(&self) -> Epsilons` — [`Epsilons`](#epsilons)

##### `impl CloneToUninit for Epsilons`

- <span id="epsilons-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Epsilons`

##### `impl Debug for Epsilons`

- <span id="epsilons-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Epsilons`

- <span id="epsilons-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Epsilons`

- <span id="epsilons-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Epsilons`

- <span id="epsilons-toowned-type-owned"></span>`type Owned = T`

- <span id="epsilons-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="epsilons-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Epsilons`

- <span id="epsilons-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="epsilons-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Epsilons`

- <span id="epsilons-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="epsilons-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Slots`

```rust
struct Slots(u32);
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2886`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2886)*

The set of epsilon transitions indicating that the current position in a
search should be saved to a slot.

This *only* represents explicit slots. So for example, the pattern
`[a-z]+([0-9]+)([a-z]+)` has:

* 3 capturing groups, thus 6 slots.
* 1 implicit capturing group, thus 2 implicit slots.
* 2 explicit capturing groups, thus 4 explicit slots.

While implicit slots are represented by epsilon transitions in an NFA, we
do not explicitly represent them here. Instead, implicit slots are assumed
to be present and handled automatically in the search code. Therefore,
that means we only need to represent explicit slots in our epsilon
transitions.

Its representation is a bit set. The bit 'i' is set if and only if there
exists an explicit slot at index 'c', where 'c = (#patterns * 2) + i'. That
is, the bit 'i' corresponds to the first explicit slot and the first
explicit slot appears immediately following the last implicit slot. (If
this is confusing, see `GroupInfo` for more details on how slots works.)

A single `Slots` represents all the active slots in a sub-graph of an NFA,
where all the states are connected by epsilon transitions. In effect, when
traversing the one-pass DFA during a search, all slots set in a particular
transition must be captured by recording the current search position.

The API of `Slots` requires the caller to handle the explicit slot offset.
That is, a `Slots` doesn't know where the explicit slots start for a
particular NFA. Thus, if the callers see's the bit 'i' is set, then they
need to do the arithmetic above to find 'c', which is the real actual slot
index in the corresponding NFA.

#### Implementations

- <span id="slots-const-limit"></span>`const LIMIT: usize`

- <span id="slots-insert"></span>`fn insert(self, slot: usize) -> Slots` — [`Slots`](#slots)

  Insert the slot at the given bit index.

- <span id="slots-remove"></span>`fn remove(self, slot: usize) -> Slots` — [`Slots`](#slots)

  Remove the slot at the given bit index.

- <span id="slots-is-empty"></span>`fn is_empty(self) -> bool`

  Returns true if and only if this set contains no slots.

- <span id="slots-iter"></span>`fn iter(self) -> SlotsIter` — [`SlotsIter`](#slotsiter)

  Returns an iterator over all of the set bits in this set.

- <span id="slots-apply"></span>`fn apply(self, at: usize, caller_explicit_slots: &mut [Option<NonMaxUsize>])` — [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize)

  For the position `at` in the current haystack, copy it to

  `caller_explicit_slots` for all slots that are in this set.

  

  Callers may pass a slice of any length. Slots in this set bigger than

  the length of the given explicit slots are simply skipped.

  

  The slice *must* correspond only to the explicit slots and the first

  element of the slice must always correspond to the first explicit slot

  in the corresponding NFA.

#### Trait Implementations

##### `impl Any for Slots`

- <span id="slots-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Slots`

- <span id="slots-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Slots`

- <span id="slots-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Slots`

- <span id="slots-clone"></span>`fn clone(&self) -> Slots` — [`Slots`](#slots)

##### `impl CloneToUninit for Slots`

- <span id="slots-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Slots`

##### `impl Debug for Slots`

- <span id="slots-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Slots`

- <span id="slots-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Slots`

- <span id="slots-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Slots`

- <span id="slots-toowned-type-owned"></span>`type Owned = T`

- <span id="slots-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slots-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Slots`

- <span id="slots-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slots-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Slots`

- <span id="slots-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slots-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SlotsIter`

```rust
struct SlotsIter {
    slots: Slots,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2955-2957`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2955-L2957)*

An iterator over all of the bits set in a slot set.

This returns the bit index that is set, so callers may need to offset it
to get the actual NFA slot index.

#### Trait Implementations

##### `impl Any for SlotsIter`

- <span id="slotsiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SlotsIter`

- <span id="slotsiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SlotsIter`

- <span id="slotsiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SlotsIter`

- <span id="slotsiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SlotsIter`

- <span id="slotsiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SlotsIter`

- <span id="slotsiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SlotsIter`

- <span id="slotsiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="slotsiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="slotsiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SlotsIter`

- <span id="slotsiter-iterator-type-item"></span>`type Item = usize`

- <span id="slotsiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

##### `impl<U> TryFrom for SlotsIter`

- <span id="slotsiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slotsiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SlotsIter`

- <span id="slotsiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slotsiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2987-2989`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2987-L2989)*

An error that occurred during the construction of a one-pass DFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`thompson::BuildError`](../../nfa/thompson/error/index.md) type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building a one-pass DFA directly from a pattern
string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="builderror-nfa"></span>`fn nfa(err: crate::nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md#builderror)

- <span id="builderror-word"></span>`fn word(err: UnicodeWordBoundaryError) -> BuildError` — [`UnicodeWordBoundaryError`](../../util/look/index.md#unicodewordboundaryerror), [`BuildError`](#builderror)

- <span id="builderror-too-many-states"></span>`fn too_many_states(limit: u64) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-too-many-patterns"></span>`fn too_many_patterns(limit: u64) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-unsupported-look"></span>`fn unsupported_look(look: Look) -> BuildError` — [`Look`](../../util/look/index.md#look), [`BuildError`](#builderror)

- <span id="builderror-exceeded-size-limit"></span>`fn exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-not-one-pass"></span>`fn not_one_pass(msg: &'static str) -> BuildError` — [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Any for BuildError`

- <span id="builderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildError`

- <span id="builderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildError`

- <span id="builderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](#builderror)

##### `impl CloneToUninit for BuildError`

- <span id="builderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildError`

- <span id="builderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- <span id="builderror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> From for BuildError`

- <span id="builderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildError`

- <span id="builderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BuildError`

- <span id="builderror-toowned-type-owned"></span>`type Owned = T`

- <span id="builderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for BuildError`

- <span id="builderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BuildError`

- <span id="builderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildError`

- <span id="builderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `BuildErrorKind`

```rust
enum BuildErrorKind {
    NFA(crate::nfa::thompson::BuildError),
    Word(crate::util::look::UnicodeWordBoundaryError),
    TooManyStates {
        limit: u64,
    },
    TooManyPatterns {
        limit: u64,
    },
    UnsupportedLook {
        look: crate::util::look::Look,
    },
    ExceededSizeLimit {
        limit: usize,
    },
    NotOnePass {
        msg: &'static str,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/onepass.rs:2993-3001`](../../../../.source_1765633015/regex-automata-0.4.13/src/dfa/onepass.rs#L2993-L3001)*

The kind of error that occurred during the construction of a one-pass DFA.

#### Trait Implementations

##### `impl Any for BuildErrorKind`

- <span id="builderrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildErrorKind`

- <span id="builderrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildErrorKind`

- <span id="builderrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildErrorKind`

- <span id="builderrorkind-clone"></span>`fn clone(&self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl CloneToUninit for BuildErrorKind`

- <span id="builderrorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildErrorKind`

- <span id="builderrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BuildErrorKind`

- <span id="builderrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildErrorKind`

- <span id="builderrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BuildErrorKind`

- <span id="builderrorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="builderrorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builderrorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BuildErrorKind`

- <span id="builderrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builderrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildErrorKind`

- <span id="builderrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builderrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

