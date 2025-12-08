*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [compiler](index.md)*

---

# Module `compiler`

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

The configuration used for a Thompson NFA compiler.

#### Implementations

- `fn new() -> Config` — [`Config`](../index.md)

- `fn utf8(self: Self, yes: bool) -> Config` — [`Config`](../index.md)

- `fn reverse(self: Self, yes: bool) -> Config` — [`Config`](../index.md)

- `fn nfa_size_limit(self: Self, bytes: Option<usize>) -> Config` — [`Config`](../index.md)

- `fn shrink(self: Self, yes: bool) -> Config` — [`Config`](../index.md)

- `fn captures(self: Self, yes: bool) -> Config` — [`Config`](../index.md)

- `fn which_captures(self: Self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](../index.md), [`Config`](../index.md)

- `fn look_matcher(self: Self, m: LookMatcher) -> Config` — [`LookMatcher`](../../../util/look/index.md), [`Config`](../index.md)

- `fn get_utf8(self: &Self) -> bool`

- `fn get_reverse(self: &Self) -> bool`

- `fn get_nfa_size_limit(self: &Self) -> Option<usize>`

- `fn get_shrink(self: &Self) -> bool`

- `fn get_captures(self: &Self) -> bool`

- `fn get_which_captures(self: &Self) -> WhichCaptures` — [`WhichCaptures`](../index.md)

- `fn get_look_matcher(self: &Self) -> LookMatcher` — [`LookMatcher`](../../../util/look/index.md)

- `fn get_unanchored_prefix(self: &Self) -> bool`

- `fn overwrite(self: &Self, o: Config) -> Config` — [`Config`](../index.md)

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](../index.md)

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Config`

- `fn default() -> Config` — [`Config`](../index.md)

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

- `fn compile<H: Borrow<Hir>>(self: &Self, exprs: &[H]) -> Result<NFA, BuildError>` — [`NFA`](../index.md), [`BuildError`](../index.md)

- `fn c(self: &Self, expr: &Hir) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_concat<I>(self: &Self, it: I) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_alt_slice(self: &Self, exprs: &[Hir]) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_alt_iter<I>(self: &Self, it: I) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_cap(self: &Self, index: u32, name: Option<&str>, expr: &Hir) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_repetition(self: &Self, rep: &hir::Repetition) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_bounded(self: &Self, expr: &Hir, greedy: bool, min: u32, max: u32) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_at_least(self: &Self, expr: &Hir, greedy: bool, n: u32) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_zero_or_one(self: &Self, expr: &Hir, greedy: bool) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_exactly(self: &Self, expr: &Hir, n: u32) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_byte_class(self: &Self, cls: &hir::ClassBytes) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_unicode_class(self: &Self, cls: &hir::ClassUnicode) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_unicode_class_reverse_with_suffix(self: &Self, cls: &hir::ClassUnicode) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_look(self: &Self, anchor: &hir::Look) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_literal(self: &Self, bytes: &[u8]) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_range(self: &Self, start: u8, end: u8) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_empty(self: &Self) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn c_fail(self: &Self) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn patch(self: &Self, from: StateID, to: StateID) -> Result<(), BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn start_pattern(self: &Self) -> Result<PatternID, BuildError>` — [`PatternID`](../../../index.md), [`BuildError`](../index.md)

- `fn finish_pattern(self: &Self, start_id: StateID) -> Result<PatternID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`PatternID`](../../../index.md), [`BuildError`](../index.md)

- `fn add_empty(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_range(self: &Self, start: u8, end: u8) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_sparse(self: &Self, ranges: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](../index.md), [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_look(self: &Self, look: Look) -> Result<StateID, BuildError>` — [`Look`](../../../util/look/index.md), [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_union(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_union_reverse(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_capture_start(self: &Self, capture_index: u32, name: Option<&str>) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_capture_end(self: &Self, capture_index: u32) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_fail(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_match(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn is_reverse(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for Compiler`

- `fn clone(self: &Self) -> Compiler` — [`Compiler`](../index.md)

##### `impl Debug for Compiler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ThompsonRef`

```rust
struct ThompsonRef {
    start: crate::util::primitives::StateID,
    end: crate::util::primitives::StateID,
}
```

A value that represents the result of compiling a sub-expression of a
regex's HIR. Specifically, this represents a sub-graph of the NFA that
has an initial state at `start` and a final state at `end`.

#### Trait Implementations

##### `impl Clone for ThompsonRef`

- `fn clone(self: &Self) -> ThompsonRef` — [`ThompsonRef`](#thompsonref)

##### `impl Copy for ThompsonRef`

##### `impl Debug for ThompsonRef`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Utf8Compiler<'a>`

```rust
struct Utf8Compiler<'a> {
    builder: &'a mut crate::nfa::thompson::builder::Builder,
    state: &'a mut Utf8State,
    target: crate::util::primitives::StateID,
}
```

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

- `fn new(builder: &'a mut Builder, state: &'a mut Utf8State) -> Result<Utf8Compiler<'a>, BuildError>` — [`Builder`](../index.md), [`Utf8State`](#utf8state), [`Utf8Compiler`](#utf8compiler), [`BuildError`](../index.md)

- `fn finish(self: &mut Self) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](#thompsonref), [`BuildError`](../index.md)

- `fn add(self: &mut Self, ranges: &[Utf8Range]) -> Result<(), BuildError>` — [`BuildError`](../index.md)

- `fn compile_from(self: &mut Self, from: usize) -> Result<(), BuildError>` — [`BuildError`](../index.md)

- `fn compile(self: &mut Self, node: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](../index.md), [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_suffix(self: &mut Self, ranges: &[Utf8Range])`

- `fn add_empty(self: &mut Self)`

- `fn pop_freeze(self: &mut Self, next: StateID) -> Vec<Transition>` — [`StateID`](../../../util/primitives/index.md), [`Transition`](../index.md)

- `fn pop_root(self: &mut Self) -> Vec<Transition>` — [`Transition`](../index.md)

- `fn top_last_freeze(self: &mut Self, next: StateID)` — [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl<'a> Debug for Utf8Compiler<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Utf8State`

```rust
struct Utf8State {
    compiled: crate::nfa::thompson::map::Utf8BoundedMap,
    uncompiled: alloc::vec::Vec<Utf8Node>,
}
```

#### Implementations

- `fn new() -> Utf8State` — [`Utf8State`](#utf8state)

- `fn clear(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for Utf8State`

- `fn clone(self: &Self) -> Utf8State` — [`Utf8State`](#utf8state)

##### `impl Debug for Utf8State`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Utf8Node`

```rust
struct Utf8Node {
    trans: alloc::vec::Vec<crate::nfa::thompson::nfa::Transition>,
    last: Option<Utf8LastTransition>,
}
```

#### Implementations

- `fn set_last_transition(self: &mut Self, next: StateID)` — [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for Utf8Node`

- `fn clone(self: &Self) -> Utf8Node` — [`Utf8Node`](#utf8node)

##### `impl Debug for Utf8Node`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Utf8LastTransition`

```rust
struct Utf8LastTransition {
    start: u8,
    end: u8,
}
```

#### Trait Implementations

##### `impl Clone for Utf8LastTransition`

- `fn clone(self: &Self) -> Utf8LastTransition` — [`Utf8LastTransition`](#utf8lasttransition)

##### `impl Debug for Utf8LastTransition`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `WhichCaptures`

```rust
enum WhichCaptures {
    All,
    Implicit,
    None,
}
```

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

- `fn is_none(self: &Self) -> bool`

- `fn is_any(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for WhichCaptures`

- `fn clone(self: &Self) -> WhichCaptures` — [`WhichCaptures`](../index.md)

##### `impl Copy for WhichCaptures`

##### `impl Debug for WhichCaptures`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for WhichCaptures`

- `fn default() -> WhichCaptures` — [`WhichCaptures`](../index.md)

