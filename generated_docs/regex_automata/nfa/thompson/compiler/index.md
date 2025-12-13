*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [compiler](index.md)*

---

# Module `compiler`

## Contents

- [Structs](#structs)
  - [`Config`](#config)
  - [`Compiler`](#compiler)
  - [`ThompsonRef`](#thompsonref)
  - [`Utf8Compiler`](#utf8compiler)
  - [`Utf8State`](#utf8state)
  - [`Utf8Node`](#utf8node)
  - [`Utf8LastTransition`](#utf8lasttransition)
- [Enums](#enums)
  - [`WhichCaptures`](#whichcaptures)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration used for a Thompson NFA compiler. |
| [`Compiler`](#compiler) | struct | A builder for compiling an NFA from a regex's high-level intermediate representation (HIR). |
| [`ThompsonRef`](#thompsonref) | struct | A value that represents the result of compiling a sub-expression of a regex's HIR. |
| [`Utf8Compiler`](#utf8compiler) | struct | A UTF-8 compiler based on Daciuk's algorithm for compiling minimal DFAs from a lexicographically sorted sequence of strings in linear time. |
| [`Utf8State`](#utf8state) | struct |  |
| [`Utf8Node`](#utf8node) | struct |  |
| [`Utf8LastTransition`](#utf8lasttransition) | struct |  |
| [`WhichCaptures`](#whichcaptures) | enum | A configuration indicating which kinds of [`State::Capture`](crate::nfa::thompson::State::Capture) states to include. |

## Structs

### `Config`

```rust
struct Config {
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<crate::util::look::LookMatcher>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:28-37`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L28-L37)*

The configuration used for a Thompson NFA compiler.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

  Return a new default Thompson NFA compiler configuration.

- <span id="config-utf8"></span>`fn utf8(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to enable UTF-8 mode during search or not.

  

  A regex engine is said to be in UTF-8 mode when it guarantees that

  all matches returned by it have spans consisting of only valid UTF-8.

  That is, it is impossible for a match span to be returned that

  contains any invalid UTF-8.

  

  UTF-8 mode generally consists of two things:

  

  1. Whether the NFA's states are constructed such that all paths to a

  match state that consume at least one byte always correspond to valid

  UTF-8.

  2. Whether all paths to a match state that do _not_ consume any bytes

  should always correspond to valid UTF-8 boundaries.

  

  (1) is a guarantee made by whoever constructs the NFA.

  If you're parsing a regex from its concrete syntax, then

  [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) can make

  this guarantee for you. It does it by returning an error if the regex

  pattern could every report a non-empty match span that contains invalid

  UTF-8. So long as `syntax::Config::utf8` mode is enabled and your regex

  successfully parses, then you're guaranteed that the corresponding NFA

  will only ever report non-empty match spans containing valid UTF-8.

  

  (2) is a trickier guarantee because it cannot be enforced by the NFA

  state graph itself. Consider, for example, the regex `a*`. It matches

  the empty strings in `☃` at positions `0`, `1`, `2` and `3`, where

  positions `1` and `2` occur within the UTF-8 encoding of a codepoint,

  and thus correspond to invalid UTF-8 boundaries. Therefore, this

  guarantee must be made at a higher level than the NFA state graph

  itself. This crate deals with this case in each regex engine. Namely,

  when a zero-width match that splits a codepoint is found and UTF-8

  mode enabled, then it is ignored and the engine moves on looking for

  the next match.

  

  Thus, UTF-8 mode is both a promise that the NFA built only reports

  non-empty matches that are valid UTF-8, and an *instruction* to regex

  engines that empty matches that split codepoints should be banned.

  

  Because UTF-8 mode is fundamentally about avoiding invalid UTF-8 spans,

  it only makes sense to enable this option when you *know* your haystack

  is valid UTF-8. (For example, a `&str`.) Enabling UTF-8 mode and

  searching a haystack that contains invalid UTF-8 leads to **unspecified

  behavior**.

  

  Therefore, it may make sense to enable `syntax::Config::utf8` while

  simultaneously *disabling* this option. That would ensure all non-empty

  match spans are valid UTF-8, but that empty match spans may still split

  a codepoint or match at other places that aren't valid UTF-8.

  

  In general, this mode is only relevant if your regex can match the

  empty string. Most regexes don't.

  

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

- <span id="config-reverse"></span>`fn reverse(self, yes: bool) -> Config` — [`Config`](#config)

  Reverse the NFA.

  

  A NFA reversal is performed by reversing all of the concatenated

  sub-expressions in the original pattern, recursively. (Look around

  operators are also inverted.) The resulting NFA can be used to match

  the pattern starting from the end of a string instead of the beginning

  of a string.

  

  Reversing the NFA is useful for building a reverse DFA, which is most

  useful for finding the start of a match after its ending position has

  been found. NFA execution engines typically do not work on reverse

  NFAs. For example, currently, the Pike VM reports the starting location

  of matches without a reverse NFA.

  

  Currently, enabling this setting requires disabling the

  [`captures`](Config::captures) setting. If both are enabled, then the

  compiler will return an error. It is expected that this limitation will

  be lifted in the future.

  

  This is disabled by default.

  

  # Example

  

  This example shows how to build a DFA from a reverse NFA, and then use

  the DFA to search backwards.

  

  ```rust

  use regex_automata::{

      dfa::{self, Automaton},

      nfa::thompson::{NFA, WhichCaptures},

      HalfMatch, Input,

  };

  

  let dfa = dfa::dense::Builder::new()

      .thompson(NFA::config()

          .which_captures(WhichCaptures::None)

          .reverse(true)

      )

      .build("baz[0-9]+")?;

  let expected = Some(HalfMatch::must(0, 3));

  assert_eq!(

      expected,

      dfa.try_search_rev(&Input::new("foobaz12345bar"))?,

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-nfa-size-limit"></span>`fn nfa_size_limit(self, bytes: Option<usize>) -> Config` — [`Config`](#config)

  Sets an approximate size limit on the total heap used by the NFA being

  compiled.

  

  This permits imposing constraints on the size of a compiled NFA. This

  may be useful in contexts where the regex pattern is untrusted and one

  wants to avoid using too much memory.

  

  This size limit does not apply to auxiliary heap used during

  compilation that is not part of the built NFA.

  

  Note that this size limit is applied during compilation in order for

  the limit to prevent too much heap from being used. However, the

  implementation may use an intermediate NFA representation that is

  otherwise slightly bigger than the final public form. Since the size

  limit may be applied to an intermediate representation, there is not

  necessarily a precise correspondence between the configured size limit

  and the heap usage of the final NFA.

  

  There is no size limit by default.

  

  # Example

  

  This example demonstrates how Unicode mode can greatly increase the

  size of the NFA.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::NFA;

  

  // 300KB isn't enough!

  NFA::compiler()

      .configure(NFA::config().nfa_size_limit(Some(300_000)))

      .build(r"\w{20}")

      .unwrap_err();

  

  // ... but 500KB probably is.

  let nfa = NFA::compiler()

      .configure(NFA::config().nfa_size_limit(Some(500_000)))

      .build(r"\w{20}")?;

  

  assert_eq!(nfa.pattern_len(), 1);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-shrink"></span>`fn shrink(self, yes: bool) -> Config` — [`Config`](#config)

  Apply best effort heuristics to shrink the NFA at the expense of more

  time/memory.

  

  Generally speaking, if one is using an NFA to compile a DFA, then the

  extra time used to shrink the NFA will be more than made up for during

  DFA construction (potentially by a lot). In other words, enabling this

  can substantially decrease the overall amount of time it takes to build

  a DFA.

  

  A reason to keep this disabled is if you want to compile an NFA and

  start using it as quickly as possible without needing to build a DFA,

  and you don't mind using a bit of extra memory for the NFA. e.g., for

  an NFA simulation or for a lazy DFA.

  

  NFA shrinking is currently most useful when compiling a reverse

  NFA with large Unicode character classes. In particular, it trades

  additional CPU time during NFA compilation in favor of generating fewer

  NFA states.

  

  This is disabled by default because it can increase compile times

  quite a bit if you aren't building a full DFA.

  

  # Example

  

  This example shows that NFA shrinking can lead to substantial space

  savings in some cases. Notice that, as noted above, we build a reverse

  DFA and use a pattern with a large Unicode character class.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::{NFA, WhichCaptures};

  

  // Currently we have to disable captures when enabling reverse NFA.

  let config = NFA::config()

      .which_captures(WhichCaptures::None)

      .reverse(true);

  let not_shrunk = NFA::compiler()

      .configure(config.clone().shrink(false))

      .build(r"\w")?;

  let shrunk = NFA::compiler()

      .configure(config.clone().shrink(true))

      .build(r"\w")?;

  

  // While a specific shrink factor is not guaranteed, the savings can be

  // considerable in some cases.

  assert!(shrunk.states().len() * 2 < not_shrunk.states().len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-captures"></span>`fn captures(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to include 'Capture' states in the NFA.

  

  Currently, enabling this setting requires disabling the

  [`reverse`](Config::reverse) setting. If both are enabled, then the

  compiler will return an error. It is expected that this limitation will

  be lifted in the future.

  

  This is enabled by default.

  

  # Example

  

  This example demonstrates that some regex engines, like the Pike VM,

  require capturing states to be present in the NFA to report match

  offsets.

  

  (Note that since this method is deprecated, the example below uses

  `Config::which_captures` to disable capture states.)

  

  ```rust

  use regex_automata::nfa::thompson::{

      pikevm::PikeVM,

      NFA,

      WhichCaptures,

  };

  

  let re = PikeVM::builder()

      .thompson(NFA::config().which_captures(WhichCaptures::None))

      .build(r"[a-z]+")?;

  let mut cache = re.create_cache();

  

  assert!(re.is_match(&mut cache, "abc"));

  assert_eq!(None, re.find(&mut cache, "abc"));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-which-captures"></span>`fn which_captures(self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](#whichcaptures), [`Config`](#config)

  Configures what kinds of capture groups are compiled into

  [`State::Capture`](crate::nfa::thompson::State::Capture) states in a

  Thompson NFA.

  

  Currently, using any option except for [`WhichCaptures::None`](../../../index.md) requires

  disabling the [`reverse`](Config::reverse) setting. If both are

  enabled, then the compiler will return an error. It is expected that

  this limitation will be lifted in the future.

  

  This is set to [`WhichCaptures::All`](../../../index.md) by default. Callers may wish to

  use [`WhichCaptures::Implicit`](../../../index.md) in cases where one wants avoid the

  overhead of capture states for explicit groups. Usually this occurs

  when one wants to use the `PikeVM` only for determining the overall

  match. Otherwise, the `PikeVM` could use much more memory than is

  necessary.

  

  # Example

  

  This example demonstrates that some regex engines, like the Pike VM,

  require capturing states to be present in the NFA to report match

  offsets.

  

  ```rust

  use regex_automata::nfa::thompson::{

      pikevm::PikeVM,

      NFA,

      WhichCaptures,

  };

  

  let re = PikeVM::builder()

      .thompson(NFA::config().which_captures(WhichCaptures::None))

      .build(r"[a-z]+")?;

  let mut cache = re.create_cache();

  

  assert!(re.is_match(&mut cache, "abc"));

  assert_eq!(None, re.find(&mut cache, "abc"));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  The same applies to the bounded backtracker:

  

  ```rust

  use regex_automata::nfa::thompson::{

      backtrack::BoundedBacktracker,

      NFA,

      WhichCaptures,

  };

  

  let re = BoundedBacktracker::builder()

      .thompson(NFA::config().which_captures(WhichCaptures::None))

      .build(r"[a-z]+")?;

  let mut cache = re.create_cache();

  

  assert!(re.try_is_match(&mut cache, "abc")?);

  assert_eq!(None, re.try_find(&mut cache, "abc")?);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-look-matcher"></span>`fn look_matcher(self, m: LookMatcher) -> Config` — [`LookMatcher`](../../../util/look/index.md#lookmatcher), [`Config`](#config)

  Sets the look-around matcher that should be used with this NFA.

  

  A look-around matcher determines how to match look-around assertions.

  In particular, some assertions are configurable. For example, the

  `(?m:^)` and `(?m:$)` assertions can have their line terminator changed

  from the default of `\n` to any other byte.

  

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

- <span id="config-get-utf8"></span>`fn get_utf8(&self) -> bool`

  Returns whether this configuration has enabled UTF-8 mode.

- <span id="config-get-reverse"></span>`fn get_reverse(&self) -> bool`

  Returns whether this configuration has enabled reverse NFA compilation.

- <span id="config-get-nfa-size-limit"></span>`fn get_nfa_size_limit(&self) -> Option<usize>`

  Return the configured NFA size limit, if it exists, in the number of

  bytes of heap used.

- <span id="config-get-shrink"></span>`fn get_shrink(&self) -> bool`

  Return whether NFA shrinking is enabled.

- <span id="config-get-captures"></span>`fn get_captures(&self) -> bool`

  Return whether NFA compilation is configured to produce capture states.

- <span id="config-get-which-captures"></span>`fn get_which_captures(&self) -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

  Return what kinds of capture states will be compiled into an NFA.

- <span id="config-get-look-matcher"></span>`fn get_look_matcher(&self) -> LookMatcher` — [`LookMatcher`](../../../util/look/index.md#lookmatcher)

  Return the look-around matcher for this NFA.

- <span id="config-get-unanchored-prefix"></span>`fn get_unanchored_prefix(&self) -> bool`

  Return whether NFA compilation is configured to include an unanchored

  prefix.

  

  This is always false when not in test mode.

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

### `Compiler`

```rust
struct Compiler {
    parser: regex_syntax::ParserBuilder,
    config: Config,
    builder: core::cell::RefCell<crate::nfa::thompson::builder::Builder>,
    utf8_state: core::cell::RefCell<Utf8State>,
    trie_state: core::cell::RefCell<crate::nfa::thompson::range_trie::RangeTrie>,
    utf8_suffix: core::cell::RefCell<crate::nfa::thompson::map::Utf8SuffixMap>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:718-736`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L718-L736)*

A builder for compiling an NFA from a regex's high-level intermediate
representation (HIR).

This compiler provides a way to translate a parsed regex pattern into an
NFA state graph. The NFA state graph can either be used directly to execute
a search (e.g., with a Pike VM), or it can be further used to build a DFA.

This compiler provides APIs both for compiling regex patterns directly from
their concrete syntax, or via a [`regex_syntax::hir::Hir`](../../../../regex_syntax/hir/index.md).

This compiler has various options that may be configured via
[`thompson::Config`](Config).

Note that a compiler is not the same as a [`thompson::Builder`](Builder).
A `Builder` provides a lower level API that is uncoupled from a regex
pattern's concrete syntax or even its HIR. Instead, it permits stitching
together an NFA by hand. See its docs for examples.

# Example: compilation from concrete syntax

This shows how to compile an NFA from a pattern string while setting a size
limit on how big the NFA is allowed to be (in terms of bytes of heap used).

```rust
use regex_automata::{
    nfa::thompson::{NFA, pikevm::PikeVM},
    Match,
};

let config = NFA::config().nfa_size_limit(Some(1_000));
let nfa = NFA::compiler().configure(config).build(r"(?-u)\w")?;

let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 3..4));
re.captures(&mut cache, "!@#A#@!", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: compilation from HIR

This shows how to hand assemble a regular expression via its HIR, and then
compile an NFA directly from it.

```rust
use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};
use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
    ClassBytesRange::new(b'0', b'9'),
    ClassBytesRange::new(b'A', b'Z'),
    ClassBytesRange::new(b'_', b'_'),
    ClassBytesRange::new(b'a', b'z'),
])));

let config = NFA::config().nfa_size_limit(Some(1_000));
let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 3..4));
re.captures(&mut cache, "!@#A#@!", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`parser`**: `regex_syntax::ParserBuilder`

  A regex parser, used when compiling an NFA directly from a pattern
  string.

- **`config`**: `Config`

  The compiler configuration.

- **`builder`**: `core::cell::RefCell<crate::nfa::thompson::builder::Builder>`

  The builder for actually constructing an NFA. This provides a
  convenient abstraction for writing a compiler.

- **`utf8_state`**: `core::cell::RefCell<Utf8State>`

  State used for compiling character classes to UTF-8 byte automata.
  State is not retained between character class compilations. This just
  serves to amortize allocation to the extent possible.

- **`trie_state`**: `core::cell::RefCell<crate::nfa::thompson::range_trie::RangeTrie>`

  State used for arranging character classes in reverse into a trie.

- **`utf8_suffix`**: `core::cell::RefCell<crate::nfa::thompson::map::Utf8SuffixMap>`

  State used for caching common suffixes when compiling reverse UTF-8
  automata (for Unicode character classes).

#### Implementations

- <span id="compiler-new"></span>`fn new() -> Compiler` — [`Compiler`](#compiler)

  Create a new NFA builder with its default configuration.

- <span id="compiler-build"></span>`fn build(&self, pattern: &str) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BuildError`](../error/index.md#builderror)

  Compile the given regular expression pattern into an NFA.

  

  If there was a problem parsing the regex, then that error is returned.

  

  Otherwise, if there was a problem building the NFA, then an error is

  returned. The only error that can occur is if the compiled regex would

  exceed the size limits configured on this builder, or if any part of

  the NFA would exceed the integer representations used. (For example,

  too many states might plausibly occur on a 16-bit target.)

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build(r"(?-u)\w")?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(0, 3..4));

  re.captures(&mut cache, "!@#A#@!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BuildError`](../error/index.md#builderror)

  Compile the given regular expression patterns into a single NFA.

  

  When matches are returned, the pattern ID corresponds to the index of

  the pattern in the slice given.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_many(&[

      r"(?-u)\s",

      r"(?-u)\w",

  ])?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(1, 1..2));

  re.captures(&mut cache, "!A! !A!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-build-from-hir"></span>`fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BuildError`](../error/index.md#builderror)

  Compile the given high level intermediate representation of a regular

  expression into an NFA.

  

  If there was a problem building the NFA, then an error is returned. The

  only error that can occur is if the compiled regex would exceed the

  size limits configured on this builder, or if any part of the NFA would

  exceed the integer representations used. (For example, too many states

  might plausibly occur on a 16-bit target.)

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

  

  let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![

      ClassBytesRange::new(b'0', b'9'),

      ClassBytesRange::new(b'A', b'Z'),

      ClassBytesRange::new(b'_', b'_'),

      ClassBytesRange::new(b'a', b'z'),

  ])));

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(0, 3..4));

  re.captures(&mut cache, "!@#A#@!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-build-many-from-hir"></span>`fn build_many_from_hir<H: Borrow<Hir>>(&self, exprs: &[H]) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BuildError`](../error/index.md#builderror)

  Compile the given high level intermediate representations of regular

  expressions into a single NFA.

  

  When matches are returned, the pattern ID corresponds to the index of

  the pattern in the slice given.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

  

  let hirs = &[

      Hir::class(Class::Bytes(ClassBytes::new(vec![

          ClassBytesRange::new(b'\t', b'\r'),

          ClassBytesRange::new(b' ', b' '),

      ]))),

      Hir::class(Class::Bytes(ClassBytes::new(vec![

          ClassBytesRange::new(b'0', b'9'),

          ClassBytesRange::new(b'A', b'Z'),

          ClassBytesRange::new(b'_', b'_'),

          ClassBytesRange::new(b'a', b'z'),

      ]))),

  ];

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_many_from_hir(hirs)?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(1, 1..2));

  re.captures(&mut cache, "!A! !A!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-configure"></span>`fn configure(&mut self, config: Config) -> &mut Compiler` — [`Config`](#config), [`Compiler`](#compiler)

  Apply the given NFA configuration options to this builder.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build(r"(?-u)\w")?;

  assert_eq!(nfa.pattern_len(), 1);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Compiler` — [`Config`](../../../util/syntax/index.md#config), [`Compiler`](#compiler)

  Set the syntax configuration for this builder using

  [`syntax::Config`](crate::util::syntax::Config).

  

  This permits setting things like case insensitivity, Unicode and multi

  line mode.

  

  This syntax configuration only applies when an NFA is built directly

  from a pattern string. If an NFA is built from an HIR, then all syntax

  settings are ignored.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::syntax};

  

  let syntax_config = syntax::Config::new().unicode(false);

  let nfa = NFA::compiler().syntax(syntax_config).build(r"\w")?;

  // If Unicode were enabled, the number of states would be much bigger.

  assert!(nfa.states().len() < 15);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

#### Trait Implementations

##### `impl Any for Compiler`

- <span id="compiler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Compiler`

- <span id="compiler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Compiler`

- <span id="compiler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Compiler`

- <span id="compiler-clone"></span>`fn clone(&self) -> Compiler` — [`Compiler`](#compiler)

##### `impl CloneToUninit for Compiler`

- <span id="compiler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Compiler`

- <span id="compiler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Compiler`

- <span id="compiler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Compiler`

- <span id="compiler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Compiler`

- <span id="compiler-toowned-type-owned"></span>`type Owned = T`

- <span id="compiler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="compiler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Compiler`

- <span id="compiler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compiler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Compiler`

- <span id="compiler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compiler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThompsonRef`

```rust
struct ThompsonRef {
    start: crate::util::primitives::StateID,
    end: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1722-1725`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1722-L1725)*

A value that represents the result of compiling a sub-expression of a
regex's HIR. Specifically, this represents a sub-graph of the NFA that
has an initial state at `start` and a final state at `end`.

#### Trait Implementations

##### `impl Any for ThompsonRef`

- <span id="thompsonref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThompsonRef`

- <span id="thompsonref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThompsonRef`

- <span id="thompsonref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ThompsonRef`

- <span id="thompsonref-clone"></span>`fn clone(&self) -> ThompsonRef` — [`ThompsonRef`](#thompsonref)

##### `impl CloneToUninit for ThompsonRef`

- <span id="thompsonref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ThompsonRef`

##### `impl Debug for ThompsonRef`

- <span id="thompsonref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ThompsonRef`

- <span id="thompsonref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThompsonRef`

- <span id="thompsonref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ThompsonRef`

- <span id="thompsonref-toowned-type-owned"></span>`type Owned = T`

- <span id="thompsonref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="thompsonref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ThompsonRef`

- <span id="thompsonref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="thompsonref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThompsonRef`

- <span id="thompsonref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="thompsonref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8Compiler<'a>`

```rust
struct Utf8Compiler<'a> {
    builder: &'a mut crate::nfa::thompson::builder::Builder,
    state: &'a mut Utf8State,
    target: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1748-1752`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1748-L1752)*

A UTF-8 compiler based on Daciuk's algorithm for compiling minimal DFAs
from a lexicographically sorted sequence of strings in linear time.

The trick here is that any Unicode codepoint range can be converted to
a sequence of byte ranges that form a UTF-8 automaton. Connecting them
together via an alternation is trivial, and indeed, it works. However,
there is a lot of redundant structure in many UTF-8 automatons. Since our
UTF-8 ranges are in lexicographic order, we can use Daciuk's algorithm
to build nearly minimal DFAs in linear time. (They are guaranteed to be
minimal because we use a bounded cache of previously build DFA states.)

The drawback is that this sadly doesn't work for reverse automata, since
the ranges are no longer in lexicographic order. For that, we invented the
range trie (which gets its own module). Once a range trie is built, we then
use this same Utf8Compiler to build a reverse UTF-8 automaton.

The high level idea is described here:
https://blog.burntsushi.net/transducers/#finite-state-machines-as-data-structures

There is also another implementation of this in the `fst` crate.

#### Implementations

- <span id="utf8compiler-new"></span>`fn new(builder: &'a mut Builder, state: &'a mut Utf8State) -> Result<Utf8Compiler<'a>, BuildError>` — [`Builder`](../builder/index.md#builder), [`Utf8State`](#utf8state), [`Utf8Compiler`](#utf8compiler), [`BuildError`](../error/index.md#builderror)

- <span id="utf8compiler-finish"></span>`fn finish(&mut self) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../error/index.md#builderror)

- <span id="utf8compiler-add"></span>`fn add(&mut self, ranges: &[Utf8Range]) -> Result<(), BuildError>` — [`BuildError`](../error/index.md#builderror)

- <span id="utf8compiler-compile-from"></span>`fn compile_from(&mut self, from: usize) -> Result<(), BuildError>` — [`BuildError`](../error/index.md#builderror)

- <span id="utf8compiler-compile"></span>`fn compile(&mut self, node: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](../nfa/index.md#transition), [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

- <span id="utf8compiler-add-suffix"></span>`fn add_suffix(&mut self, ranges: &[Utf8Range])`

- <span id="utf8compiler-add-empty"></span>`fn add_empty(&mut self)`

- <span id="utf8compiler-pop-freeze"></span>`fn pop_freeze(&mut self, next: StateID) -> Vec<Transition>` — [`StateID`](../../../util/primitives/index.md#stateid), [`Transition`](../nfa/index.md#transition)

- <span id="utf8compiler-pop-root"></span>`fn pop_root(&mut self) -> Vec<Transition>` — [`Transition`](../nfa/index.md#transition)

- <span id="utf8compiler-top-last-freeze"></span>`fn top_last_freeze(&mut self, next: StateID)` — [`StateID`](../../../util/primitives/index.md#stateid)

#### Trait Implementations

##### `impl Any for Utf8Compiler<'a>`

- <span id="utf8compiler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8Compiler<'a>`

- <span id="utf8compiler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Compiler<'a>`

- <span id="utf8compiler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Utf8Compiler<'a>`

- <span id="utf8compiler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8Compiler<'a>`

- <span id="utf8compiler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8Compiler<'a>`

- <span id="utf8compiler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Utf8Compiler<'a>`

- <span id="utf8compiler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8compiler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Compiler<'a>`

- <span id="utf8compiler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8compiler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8State`

```rust
struct Utf8State {
    compiled: crate::nfa::thompson::map::Utf8BoundedMap,
    uncompiled: alloc::vec::Vec<Utf8Node>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1755-1758`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1755-L1758)*

#### Implementations

- <span id="utf8state-new"></span>`fn new() -> Utf8State` — [`Utf8State`](#utf8state)

- <span id="utf8state-clear"></span>`fn clear(&mut self)`

#### Trait Implementations

##### `impl Any for Utf8State`

- <span id="utf8state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8State`

- <span id="utf8state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8State`

- <span id="utf8state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8State`

- <span id="utf8state-clone"></span>`fn clone(&self) -> Utf8State` — [`Utf8State`](#utf8state)

##### `impl CloneToUninit for Utf8State`

- <span id="utf8state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8State`

- <span id="utf8state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8State`

- <span id="utf8state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8State`

- <span id="utf8state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Utf8State`

- <span id="utf8state-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8State`

- <span id="utf8state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8State`

- <span id="utf8state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8Node`

```rust
struct Utf8Node {
    trans: alloc::vec::Vec<crate::nfa::thompson::nfa::Transition>,
    last: Option<Utf8LastTransition>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1761-1764`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1761-L1764)*

#### Implementations

- <span id="utf8node-set-last-transition"></span>`fn set_last_transition(&mut self, next: StateID)` — [`StateID`](../../../util/primitives/index.md#stateid)

#### Trait Implementations

##### `impl Any for Utf8Node`

- <span id="utf8node-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8Node`

- <span id="utf8node-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Node`

- <span id="utf8node-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8Node`

- <span id="utf8node-clone"></span>`fn clone(&self) -> Utf8Node` — [`Utf8Node`](#utf8node)

##### `impl CloneToUninit for Utf8Node`

- <span id="utf8node-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8Node`

- <span id="utf8node-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8Node`

- <span id="utf8node-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8Node`

- <span id="utf8node-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Utf8Node`

- <span id="utf8node-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8node-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8node-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8Node`

- <span id="utf8node-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8node-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Node`

- <span id="utf8node-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8node-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8LastTransition`

```rust
struct Utf8LastTransition {
    start: u8,
    end: u8,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1767-1770`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1767-L1770)*

#### Trait Implementations

##### `impl Any for Utf8LastTransition`

- <span id="utf8lasttransition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8LastTransition`

- <span id="utf8lasttransition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8LastTransition`

- <span id="utf8lasttransition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8LastTransition`

- <span id="utf8lasttransition-clone"></span>`fn clone(&self) -> Utf8LastTransition` — [`Utf8LastTransition`](#utf8lasttransition)

##### `impl CloneToUninit for Utf8LastTransition`

- <span id="utf8lasttransition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8LastTransition`

- <span id="utf8lasttransition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8LastTransition`

- <span id="utf8lasttransition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8LastTransition`

- <span id="utf8lasttransition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Utf8LastTransition`

- <span id="utf8lasttransition-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8lasttransition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8lasttransition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8LastTransition`

- <span id="utf8lasttransition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8lasttransition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8LastTransition`

- <span id="utf8lasttransition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8lasttransition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `WhichCaptures`

```rust
enum WhichCaptures {
    All,
    Implicit,
    None,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:547-589`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L547-L589)*

A configuration indicating which kinds of
[`State::Capture`](crate::nfa::thompson::State::Capture) states to include.

This configuration can be used with `Config::which_captures` to control
which capture states are compiled into a Thompson NFA.

The default configuration is [`WhichCaptures::All`](../../../index.md).

#### Variants

- **`All`**

  All capture states, including those corresponding to both implicit and
  explicit capture groups, are included in the Thompson NFA.

- **`Implicit`**

  Only capture states corresponding to implicit capture groups are
  included. Implicit capture groups appear in every pattern implicitly
  and correspond to the overall match of a pattern.
  
  This is useful when one only cares about the overall match of a
  pattern. By excluding capture states from explicit capture groups,
  one might be able to reduce the memory usage of a multi-pattern regex
  substantially if it was otherwise written to have many explicit capture
  groups.

- **`None`**

  No capture states are compiled into the Thompson NFA.
  
  This is useful when capture states are either not needed (for example,
  if one is only trying to build a DFA) or if they aren't supported (for
  example, a reverse NFA).
  
  # Warning
  
  Callers must be exceedingly careful when using this
  option. In particular, not all regex engines support
  reporting match spans when using this option (for example,
  [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) or
  [`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)).
  
  Perhaps more confusingly, using this option with such an
  engine means that an `is_match` routine could report `true`
  when `find` reports `None`. This is generally not something
  that _should_ happen, but the low level control provided by
  this crate makes it possible.
  
  Similarly, any regex engines (like [`meta::Regex`](crate::meta::Regex))
  should always return `None` from `find` routines when this option is
  used, even if _some_ of its internal engines could find the match
  boundaries. This is because inputs from user data could influence
  engine selection, and thus influence whether a match is found or not.
  Indeed, `meta::Regex::find` will always return `None` when configured
  with this option.

#### Implementations

- <span id="whichcaptures-is-none"></span>`fn is_none(&self) -> bool`

  Returns true if this configuration indicates that no capture states

  should be produced in an NFA.

- <span id="whichcaptures-is-any"></span>`fn is_any(&self) -> bool`

  Returns true if this configuration indicates that some capture states

  should be added to an NFA. Note that this might only include capture

  states for implicit capture groups.

#### Trait Implementations

##### `impl Any for WhichCaptures`

- <span id="whichcaptures-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhichCaptures`

- <span id="whichcaptures-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhichCaptures`

- <span id="whichcaptures-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WhichCaptures`

- <span id="whichcaptures-clone"></span>`fn clone(&self) -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

##### `impl CloneToUninit for WhichCaptures`

- <span id="whichcaptures-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for WhichCaptures`

##### `impl Debug for WhichCaptures`

- <span id="whichcaptures-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WhichCaptures`

- <span id="whichcaptures-default"></span>`fn default() -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

##### `impl<T> From for WhichCaptures`

- <span id="whichcaptures-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhichCaptures`

- <span id="whichcaptures-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for WhichCaptures`

- <span id="whichcaptures-toowned-type-owned"></span>`type Owned = T`

- <span id="whichcaptures-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whichcaptures-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WhichCaptures`

- <span id="whichcaptures-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whichcaptures-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhichCaptures`

- <span id="whichcaptures-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whichcaptures-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

