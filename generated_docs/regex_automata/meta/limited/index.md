*[regex_automata](../../index.md) / [meta](../index.md) / [limited](index.md)*

---

# Module `limited`

This module defines two bespoke reverse DFA searching routines. (One for the
lazy DFA and one for the fully compiled DFA.) These routines differ from the
usual ones by permitting the caller to specify a minimum starting position.
That is, the search will begin at `input.end()` and will usually stop at
`input.start()`, unless `min_start > input.start()`, in which case, the search
will stop at `min_start`.

In other words, this lets you say, "no, the search must not extend past this
point, even if it's within the bounds of the given `Input`." And if the search
*does* want to go past that point, it stops and returns a "may be quadratic"
error, which indicates that the caller should retry using some other technique.

These routines specifically exist to protect against quadratic behavior when
employing the "reverse suffix" and "reverse inner" optimizations. Without the
backstop these routines provide, it is possible for parts of the haystack to
get re-scanned over and over again. The backstop not only prevents this, but
*tells you when it is happening* so that you can change the strategy.

Why can't we just use the normal search routines? We could use the normal
search routines and just set the start bound on the provided `Input` to our
`min_start` position. The problem here is that it's impossible to distinguish
between "no match because we reached the end of input" and "determined there
was no match well before the end of input." The former case is what we care
about with respect to quadratic behavior. The latter case is totally fine.

Why don't we modify the normal search routines to report the position at which
the search stops? I considered this, and I still wonder if it is indeed the
right thing to do. However, I think the straight-forward thing to do there
would be to complicate the return type signature of almost every search routine
in this crate, which I really do not want to do. It therefore might make more
sense to provide a richer way for search routines to report meta data, but that
was beyond my bandwidth to work on at the time of writing.

See the 'opt/reverse-inner' and 'opt/reverse-suffix' benchmarks in rebar for a
real demonstration of how quadratic behavior is mitigated.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`hybrid_try_search_half_rev`](#hybrid_try_search_half_rev) | fn |  |
| [`hybrid_eoi_rev`](#hybrid_eoi_rev) | fn |  |

## Functions

### `hybrid_try_search_half_rev`

```rust
fn hybrid_try_search_half_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::Input<'_>, min_start: usize) -> Result<Option<crate::HalfMatch>, crate::meta::error::RetryError>
```

*Defined in [`regex-automata-0.4.13/src/meta/limited.rs:125-182`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/limited.rs#L125-L182)*

### `hybrid_eoi_rev`

```rust
fn hybrid_eoi_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::Input<'_>, sid: &mut crate::hybrid::LazyStateID, mat: &mut Option<crate::HalfMatch>) -> Result<(), crate::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/meta/limited.rs:219-251`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/limited.rs#L219-L251)*

