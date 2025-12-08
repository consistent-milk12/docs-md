*[regex_automata](../../index.md) / [hybrid](../index.md) / [id](index.md)*

---

# Module `id`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LazyStateID`](#lazystateid) | struct | A state identifier specifically tailored for lazy DFAs. |
| [`LazyStateIDError`](#lazystateiderror) | struct | This error occurs when a lazy state ID could not be constructed. |

## Structs

### `LazyStateID`

```rust
struct LazyStateID(u32);
```

A state identifier specifically tailored for lazy DFAs.

A lazy state ID logically represents a pointer to a DFA state. In practice,
by limiting the number of DFA states it can address, it reserves some
bits of its representation to encode some additional information. That
additional information is called a "tag." That tag is used to record
whether the state it points to is an unknown, dead, quit, start or match
state.

When implementing a low level search routine with a lazy DFA, it is
necessary to query the type of the current state to know what to do:

* **Unknown** - The state has not yet been computed. The
parameters used to get this state ID must be re-passed to
[`DFA::next_state`](crate::hybrid::dfa::DFA::next_state), which will never
return an unknown state ID.
* **Dead** - A dead state only has transitions to itself. It indicates that
the search cannot do anything else and should stop with whatever result it
has.
* **Quit** - A quit state indicates that the automaton could not answer
whether a match exists or not. Correct search implementations must return a
[`MatchError::quit`](crate::MatchError::quit) when a DFA enters a quit
state.
* **Start** - A start state is a state in which a search can begin.
Lazy DFAs usually have more than one start state. Branching on
this isn't required for correctness, but a common optimization is
to run a prefilter when a search enters a start state. Note that
start states are *not* tagged automatically, and one must enable the
[`Config::specialize_start_states`](crate::hybrid::dfa::Config::specialize_start_states)
setting for start states to be tagged. The reason for this is
that a DFA search loop is usually written to execute a prefilter once it
enters a start state. But if there is no prefilter, this handling can be
quite disastrous as the DFA may ping-pong between the special handling code
and a possible optimized hot path for handling untagged states. When start
states aren't specialized, then they are untagged and remain in the hot
path.
* **Match** - A match state indicates that a match has been found.
Depending on the semantics of your search implementation, it may either
continue until the end of the haystack or a dead state, or it might quit
and return the match immediately.

As an optimization, the [`is_tagged`](LazyStateID::is_tagged) predicate
can be used to determine if a tag exists at all. This is useful to avoid
branching on all of the above types for every byte searched.

# Example

This example shows how `LazyStateID` can be used to implement a correct
search routine with minimal branching. In particular, this search routine
implements "leftmost" matching, which means that it doesn't immediately
stop once a match is found. Instead, it continues until it reaches a dead
state.

Notice also how a correct search implementation deals with
[`CacheError`](crate::hybrid::CacheError)s returned by some of
the lazy DFA routines. When a `CacheError` occurs, it returns
[`MatchError::gave_up`](crate::MatchError::gave_up).

```rust
use regex_automata::{
    hybrid::dfa::{Cache, DFA},
    HalfMatch, MatchError, Input,
};

fn find_leftmost_first(
    dfa: &DFA,
    cache: &mut Cache,
    haystack: &[u8],
) -> Result<Option<HalfMatch>, MatchError> {
    // The start state is determined by inspecting the position and the
    // initial bytes of the haystack. Note that start states can never
    // be match states (since DFAs in this crate delay matches by 1
    // byte), so we don't need to check if the start state is a match.
    let mut sid = dfa.start_state_forward(
        cache,
        &Input::new(haystack),
    )?;
    let mut last_match = None;
    // Walk all the bytes in the haystack. We can quit early if we see
    // a dead or a quit state. The former means the automaton will
    // never transition to any other state. The latter means that the
    // automaton entered a condition in which its search failed.
    for (i, &b) in haystack.iter().enumerate() {
        sid = dfa
            .next_state(cache, sid, b)
            .map_err(|_| MatchError::gave_up(i))?;
        if sid.is_tagged() {
            if sid.is_match() {
                last_match = Some(HalfMatch::new(
                    dfa.match_pattern(cache, sid, 0),
                    i,
                ));
            } else if sid.is_dead() {
                return Ok(last_match);
            } else if sid.is_quit() {
                // It is possible to enter into a quit state after
                // observing a match has occurred. In that case, we
                // should return the match instead of an error.
                if last_match.is_some() {
                    return Ok(last_match);
                }
                return Err(MatchError::quit(b, i));
            }
            // Implementors may also want to check for start states and
            // handle them differently for performance reasons. But it is
            // not necessary for correctness. Note that in order to check
            // for start states, you'll need to enable the
            // 'specialize_start_states' config knob, otherwise start
            // states will not be tagged.
        }
    }
    // Matches are always delayed by 1 byte, so we must explicitly walk
    // the special "EOI" transition at the end of the search.
    sid = dfa
        .next_eoi_state(cache, sid)
        .map_err(|_| MatchError::gave_up(haystack.len()))?;
    if sid.is_match() {
        last_match = Some(HalfMatch::new(
            dfa.match_pattern(cache, sid, 0),
            haystack.len(),
        ));
    }
    Ok(last_match)
}

// We use a greedy '+' operator to show how the search doesn't just stop
// once a match is detected. It continues extending the match. Using
// '[a-z]+?' would also work as expected and stop the search early.
// Greediness is built into the automaton.
let dfa = DFA::new(r"[a-z]+")?;
let mut cache = dfa.create_cache();
let haystack = "123 foobar 4567".as_bytes();
let mat = find_leftmost_first(&dfa, &mut cache, haystack)?.unwrap();
assert_eq!(mat.pattern().as_usize(), 0);
assert_eq!(mat.offset(), 10);

// Here's another example that tests our handling of the special
// EOI transition. This will fail to find a match if we don't call
// 'next_eoi_state' at the end of the search since the match isn't found
// until the final byte in the haystack.
let dfa = DFA::new(r"[0-9]{4}")?;
let mut cache = dfa.create_cache();
let haystack = "123 foobar 4567".as_bytes();
let mat = find_leftmost_first(&dfa, &mut cache, haystack)?.unwrap();
assert_eq!(mat.pattern().as_usize(), 0);
assert_eq!(mat.offset(), 15);

// And note that our search implementation above automatically works
// with multi-DFAs. Namely, `dfa.match_pattern(match_state, 0)` selects
// the appropriate pattern ID for us.
let dfa = DFA::new_many(&[r"[a-z]+", r"[0-9]+"])?;
let mut cache = dfa.create_cache();
let haystack = "123 foobar 4567".as_bytes();
let mat = find_leftmost_first(&dfa, &mut cache, haystack)?.unwrap();
assert_eq!(mat.pattern().as_usize(), 1);
assert_eq!(mat.offset(), 3);
let mat = find_leftmost_first(&dfa, &mut cache, &haystack[3..])?.unwrap();
assert_eq!(mat.pattern().as_usize(), 0);
assert_eq!(mat.offset(), 7);
let mat = find_leftmost_first(&dfa, &mut cache, &haystack[10..])?.unwrap();
assert_eq!(mat.pattern().as_usize(), 1);
assert_eq!(mat.offset(), 5);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="lazystateid-max-bit"></span>`const MAX_BIT: usize`

- <span id="lazystateid-mask-unknown"></span>`const MASK_UNKNOWN: usize`

- <span id="lazystateid-mask-dead"></span>`const MASK_DEAD: usize`

- <span id="lazystateid-mask-quit"></span>`const MASK_QUIT: usize`

- <span id="lazystateid-mask-start"></span>`const MASK_START: usize`

- <span id="lazystateid-mask-match"></span>`const MASK_MATCH: usize`

- <span id="lazystateid-max"></span>`const MAX: usize`

- <span id="lazystateid-new"></span>`fn new(id: usize) -> Result<LazyStateID, LazyStateIDError>` — [`LazyStateID`](../index.md), [`LazyStateIDError`](#lazystateiderror)

- <span id="lazystateid-new-unchecked"></span>`const fn new_unchecked(id: usize) -> LazyStateID` — [`LazyStateID`](../index.md)

- <span id="lazystateid-as-usize-untagged"></span>`fn as_usize_untagged(&self) -> usize`

- <span id="lazystateid-as-usize-unchecked"></span>`const fn as_usize_unchecked(&self) -> usize`

- <span id="lazystateid-to-unknown"></span>`const fn to_unknown(&self) -> LazyStateID` — [`LazyStateID`](../index.md)

- <span id="lazystateid-to-dead"></span>`const fn to_dead(&self) -> LazyStateID` — [`LazyStateID`](../index.md)

- <span id="lazystateid-to-quit"></span>`const fn to_quit(&self) -> LazyStateID` — [`LazyStateID`](../index.md)

- <span id="lazystateid-to-start"></span>`const fn to_start(&self) -> LazyStateID` — [`LazyStateID`](../index.md)

- <span id="lazystateid-to-match"></span>`const fn to_match(&self) -> LazyStateID` — [`LazyStateID`](../index.md)

- <span id="lazystateid-is-tagged"></span>`const fn is_tagged(&self) -> bool`

- <span id="lazystateid-is-unknown"></span>`const fn is_unknown(&self) -> bool`

- <span id="lazystateid-is-dead"></span>`const fn is_dead(&self) -> bool`

- <span id="lazystateid-is-quit"></span>`const fn is_quit(&self) -> bool`

- <span id="lazystateid-is-start"></span>`const fn is_start(&self) -> bool`

- <span id="lazystateid-is-match"></span>`const fn is_match(&self) -> bool`

#### Trait Implementations

##### `impl Clone for LazyStateID`

- <span id="lazystateid-clone"></span>`fn clone(&self) -> LazyStateID` — [`LazyStateID`](../index.md)

##### `impl Copy for LazyStateID`

##### `impl Debug for LazyStateID`

- <span id="lazystateid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LazyStateID`

- <span id="lazystateid-default"></span>`fn default() -> LazyStateID` — [`LazyStateID`](../index.md)

##### `impl Eq for LazyStateID`

##### `impl Hash for LazyStateID`

- <span id="lazystateid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LazyStateID`

- <span id="lazystateid-cmp"></span>`fn cmp(&self, other: &LazyStateID) -> cmp::Ordering` — [`LazyStateID`](../index.md)

##### `impl PartialEq for LazyStateID`

- <span id="lazystateid-eq"></span>`fn eq(&self, other: &LazyStateID) -> bool` — [`LazyStateID`](../index.md)

##### `impl PartialOrd for LazyStateID`

- <span id="lazystateid-partial-cmp"></span>`fn partial_cmp(&self, other: &LazyStateID) -> option::Option<cmp::Ordering>` — [`LazyStateID`](../index.md)

##### `impl StructuralPartialEq for LazyStateID`

### `LazyStateIDError`

```rust
struct LazyStateIDError {
    attempted: u64,
}
```

This error occurs when a lazy state ID could not be constructed.

This occurs when given an integer exceeding the maximum lazy state ID
value.

When the `std` feature is enabled, this implements the `Error` trait.

#### Implementations

- <span id="lazystateiderror-attempted"></span>`fn attempted(&self) -> u64`

#### Trait Implementations

##### `impl Clone for LazyStateIDError`

- <span id="lazystateiderror-clone"></span>`fn clone(&self) -> LazyStateIDError` — [`LazyStateIDError`](#lazystateiderror)

##### `impl Debug for LazyStateIDError`

- <span id="lazystateiderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LazyStateIDError`

- <span id="lazystateiderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for LazyStateIDError`

##### `impl Error for LazyStateIDError`

##### `impl PartialEq for LazyStateIDError`

- <span id="lazystateiderror-eq"></span>`fn eq(&self, other: &LazyStateIDError) -> bool` — [`LazyStateIDError`](#lazystateiderror)

##### `impl StructuralPartialEq for LazyStateIDError`

##### `impl<T> ToString for LazyStateIDError`

- <span id="lazystateiderror-to-string"></span>`fn to_string(&self) -> String`

