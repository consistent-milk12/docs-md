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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:28-37`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L28-L37)*

The configuration used for a Thompson NFA compiler.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

- <span id="config-utf8"></span>`fn utf8(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-reverse"></span>`fn reverse(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-nfa-size-limit"></span>`fn nfa_size_limit(self, bytes: Option<usize>) -> Config` — [`Config`](#config)

- <span id="config-shrink"></span>`fn shrink(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-captures"></span>`fn captures(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-which-captures"></span>`fn which_captures(self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](#whichcaptures), [`Config`](#config)

- <span id="config-look-matcher"></span>`fn look_matcher(self, m: LookMatcher) -> Config` — [`LookMatcher`](../../../util/look/index.md), [`Config`](#config)

- <span id="config-get-utf8"></span>`fn get_utf8(&self) -> bool`

- <span id="config-get-reverse"></span>`fn get_reverse(&self) -> bool`

- <span id="config-get-nfa-size-limit"></span>`fn get_nfa_size_limit(&self) -> Option<usize>`

- <span id="config-get-shrink"></span>`fn get_shrink(&self) -> bool`

- <span id="config-get-captures"></span>`fn get_captures(&self) -> bool`

- <span id="config-get-which-captures"></span>`fn get_which_captures(&self) -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

- <span id="config-get-look-matcher"></span>`fn get_look_matcher(&self) -> LookMatcher` — [`LookMatcher`](../../../util/look/index.md)

- <span id="config-get-unanchored-prefix"></span>`fn get_unanchored_prefix(&self) -> bool`

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](#config)

#### Trait Implementations

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- <span id="config-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:718-736`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L718-L736)*

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

- <span id="compiler-build"></span>`fn build(&self, pattern: &str) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md), [`BuildError`](../error/index.md)

- <span id="compiler-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md), [`BuildError`](../error/index.md)

- <span id="compiler-build-from-hir"></span>`fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md), [`BuildError`](../error/index.md)

- <span id="compiler-build-many-from-hir"></span>`fn build_many_from_hir<H: Borrow<Hir>>(&self, exprs: &[H]) -> Result<NFA, BuildError>` — [`NFA`](../nfa/index.md), [`BuildError`](../error/index.md)

- <span id="compiler-configure"></span>`fn configure(&mut self, config: Config) -> &mut Compiler` — [`Config`](#config), [`Compiler`](#compiler)

- <span id="compiler-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Compiler` — [`Config`](../../../util/syntax/index.md), [`Compiler`](#compiler)

#### Trait Implementations

##### `impl Clone for Compiler`

- <span id="compiler-clone"></span>`fn clone(&self) -> Compiler` — [`Compiler`](#compiler)

##### `impl Debug for Compiler`

- <span id="compiler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ThompsonRef`

```rust
struct ThompsonRef {
    start: crate::util::primitives::StateID,
    end: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1722-1725`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1722-L1725)*

A value that represents the result of compiling a sub-expression of a
regex's HIR. Specifically, this represents a sub-graph of the NFA that
has an initial state at `start` and a final state at `end`.

#### Trait Implementations

##### `impl Clone for ThompsonRef`

- <span id="thompsonref-clone"></span>`fn clone(&self) -> ThompsonRef` — [`ThompsonRef`](#thompsonref)

##### `impl Copy for ThompsonRef`

##### `impl Debug for ThompsonRef`

- <span id="thompsonref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Utf8Compiler<'a>`

```rust
struct Utf8Compiler<'a> {
    builder: &'a mut crate::nfa::thompson::builder::Builder,
    state: &'a mut Utf8State,
    target: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1748-1752`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1748-L1752)*

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

- <span id="utf8compiler-new"></span>`fn new(builder: &'a mut Builder, state: &'a mut Utf8State) -> Result<Utf8Compiler<'a>, BuildError>` — [`Builder`](../builder/index.md), [`Utf8State`](#utf8state), [`Utf8Compiler`](#utf8compiler), [`BuildError`](../error/index.md)

- <span id="utf8compiler-finish"></span>`fn finish(&mut self) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../error/index.md)

- <span id="utf8compiler-add"></span>`fn add(&mut self, ranges: &[Utf8Range]) -> Result<(), BuildError>` — [`BuildError`](../error/index.md)

- <span id="utf8compiler-compile-from"></span>`fn compile_from(&mut self, from: usize) -> Result<(), BuildError>` — [`BuildError`](../error/index.md)

- <span id="utf8compiler-compile"></span>`fn compile(&mut self, node: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](../nfa/index.md), [`StateID`](../../../util/primitives/index.md), [`BuildError`](../error/index.md)

- <span id="utf8compiler-add-suffix"></span>`fn add_suffix(&mut self, ranges: &[Utf8Range])`

- <span id="utf8compiler-add-empty"></span>`fn add_empty(&mut self)`

- <span id="utf8compiler-pop-freeze"></span>`fn pop_freeze(&mut self, next: StateID) -> Vec<Transition>` — [`StateID`](../../../util/primitives/index.md), [`Transition`](../nfa/index.md)

- <span id="utf8compiler-pop-root"></span>`fn pop_root(&mut self) -> Vec<Transition>` — [`Transition`](../nfa/index.md)

- <span id="utf8compiler-top-last-freeze"></span>`fn top_last_freeze(&mut self, next: StateID)` — [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Debug for Utf8Compiler<'a>`

- <span id="utf8compiler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Utf8State`

```rust
struct Utf8State {
    compiled: crate::nfa::thompson::map::Utf8BoundedMap,
    uncompiled: alloc::vec::Vec<Utf8Node>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1755-1758`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1755-L1758)*

#### Implementations

- <span id="utf8state-new"></span>`fn new() -> Utf8State` — [`Utf8State`](#utf8state)

- <span id="utf8state-clear"></span>`fn clear(&mut self)`

#### Trait Implementations

##### `impl Clone for Utf8State`

- <span id="utf8state-clone"></span>`fn clone(&self) -> Utf8State` — [`Utf8State`](#utf8state)

##### `impl Debug for Utf8State`

- <span id="utf8state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Utf8Node`

```rust
struct Utf8Node {
    trans: alloc::vec::Vec<crate::nfa::thompson::nfa::Transition>,
    last: Option<Utf8LastTransition>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1761-1764`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1761-L1764)*

#### Implementations

- <span id="utf8node-set-last-transition"></span>`fn set_last_transition(&mut self, next: StateID)` — [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for Utf8Node`

- <span id="utf8node-clone"></span>`fn clone(&self) -> Utf8Node` — [`Utf8Node`](#utf8node)

##### `impl Debug for Utf8Node`

- <span id="utf8node-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Utf8LastTransition`

```rust
struct Utf8LastTransition {
    start: u8,
    end: u8,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:1767-1770`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L1767-L1770)*

#### Trait Implementations

##### `impl Clone for Utf8LastTransition`

- <span id="utf8lasttransition-clone"></span>`fn clone(&self) -> Utf8LastTransition` — [`Utf8LastTransition`](#utf8lasttransition)

##### `impl Debug for Utf8LastTransition`

- <span id="utf8lasttransition-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `WhichCaptures`

```rust
enum WhichCaptures {
    All,
    Implicit,
    None,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:547-589`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L547-L589)*

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

- <span id="whichcaptures-is-any"></span>`fn is_any(&self) -> bool`

#### Trait Implementations

##### `impl Clone for WhichCaptures`

- <span id="whichcaptures-clone"></span>`fn clone(&self) -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

##### `impl Copy for WhichCaptures`

##### `impl Debug for WhichCaptures`

- <span id="whichcaptures-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WhichCaptures`

- <span id="whichcaptures-default"></span>`fn default() -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

