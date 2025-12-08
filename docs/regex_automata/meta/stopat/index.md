*[regex_automata](../../index.md) / [meta](../index.md) / [stopat](index.md)*

---

# Module `stopat`

This module defines two bespoke forward DFA search routines. One for the lazy
DFA and one for the fully compiled DFA. These routines differ from the normal
ones by reporting the position at which the search terminates when a match
*isn't* found.

This position at which a search terminates is useful in contexts where the meta
regex engine runs optimizations that could go quadratic if we aren't careful.
Namely, a regex search *could* scan to the end of the haystack only to report a
non-match. If the caller doesn't know that the search scanned to the end of the
haystack, it might restart the search at the next literal candidate it finds
and repeat the process.

Providing the caller with the position at which the search stopped provides a
way for the caller to determine the point at which subsequent scans should not
pass. This is principally used in the "reverse inner" optimization, which works
like this:

1. Look for a match of an inner literal. Say, 'Z' in '\w+Z\d+'.
2. At the spot where 'Z' matches, do a reverse anchored search from there for
'\w+'.
3. If the reverse search matches, it corresponds to the start position of a
(possible) match. At this point, do a forward anchored search to find the end
position. If an end position is found, then we have a match and we know its
bounds.

If the forward anchored search in (3) searches the entire rest of the haystack
but reports a non-match, then a naive implementation of the above will continue
back at step 1 looking for more candidates. There might still be a match to be
found! It's possible. But we already scanned the whole haystack. So if we keep
repeating the process, then we might wind up taking quadratic time in the size
of the haystack, which is not great.

So if the forward anchored search in (3) reports the position at which it
stops, then we can detect whether quadratic behavior might be occurring in
steps (1) and (2). For (1), it occurs if the literal candidate found occurs
*before* the end of the previous search in (3), since that means we're now
going to look for another match in a place where the forward search has already
scanned. It is *correct* to do so, but our technique has become inefficient.
For (2), quadratic behavior occurs similarly when its reverse search extends
past the point where the previous forward search in (3) terminated. Indeed, to
implement (2), we use the sibling 'limited' module for ensuring our reverse
scan doesn't go further than we want.

See the 'opt/reverse-inner' benchmarks in rebar for a real demonstration of
how quadratic behavior is mitigated.

## Functions

### `hybrid_try_search_half_fwd`

```rust
fn hybrid_try_search_half_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::Input<'_>) -> Result<Result<crate::HalfMatch, usize>, crate::meta::error::RetryFailError>
```

### `hybrid_eoi_fwd`

```rust
fn hybrid_eoi_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::Input<'_>, sid: &mut crate::hybrid::LazyStateID, mat: &mut Option<crate::HalfMatch>) -> Result<(), crate::MatchError>
```

