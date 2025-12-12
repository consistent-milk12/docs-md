*[regex_automata](../../index.md) / [hybrid](../index.md) / [search](index.md)*

---

# Module `search`

## Contents

- [Functions](#functions)
  - [`find_fwd`](#find-fwd)
  - [`find_fwd_imp`](#find-fwd-imp)
  - [`find_rev`](#find-rev)
  - [`find_rev_imp`](#find-rev-imp)
  - [`find_overlapping_fwd`](#find-overlapping-fwd)
  - [`find_overlapping_fwd_imp`](#find-overlapping-fwd-imp)
  - [`find_overlapping_rev`](#find-overlapping-rev)
  - [`init_fwd`](#init-fwd)
  - [`init_rev`](#init-rev)
  - [`eoi_fwd`](#eoi-fwd)
  - [`eoi_rev`](#eoi-rev)
  - [`prefilter_restart`](#prefilter-restart)
  - [`gave_up`](#gave-up)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`find_fwd`](#find-fwd) | fn |  |
| [`find_fwd_imp`](#find-fwd-imp) | fn |  |
| [`find_rev`](#find-rev) | fn |  |
| [`find_rev_imp`](#find-rev-imp) | fn |  |
| [`find_overlapping_fwd`](#find-overlapping-fwd) | fn |  |
| [`find_overlapping_fwd_imp`](#find-overlapping-fwd-imp) | fn |  |
| [`find_overlapping_rev`](#find-overlapping-rev) | fn |  |
| [`init_fwd`](#init-fwd) | fn |  |
| [`init_rev`](#init-rev) | fn |  |
| [`eoi_fwd`](#eoi-fwd) | fn |  |
| [`eoi_rev`](#eoi-rev) | fn |  |
| [`prefilter_restart`](#prefilter-restart) | fn | Re-compute the starting state that a DFA should be in after finding a prefilter candidate match at the position `at`. |
| [`gave_up`](#gave-up) | fn | A convenience routine for constructing a "gave up" match error. |

## Functions

### `find_fwd`

```rust
fn find_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:13-47`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L13-L47)*

### `find_fwd_imp`

```rust
fn find_fwd_imp(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, pre: Option<&crate::util::prefilter::Prefilter>, earliest: bool) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:50-293`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L50-L293)*

### `find_rev`

```rust
fn find_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:296-309`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L296-L309)*

### `find_rev_imp`

```rust
fn find_rev_imp(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, earliest: bool) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:312-440`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L312-L440)*

### `find_overlapping_fwd`

```rust
fn find_overlapping_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, state: &mut crate::hybrid::dfa::OverlappingState) -> Result<(), crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:443-463`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L443-L463)*

### `find_overlapping_fwd_imp`

```rust
fn find_overlapping_fwd_imp(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, pre: Option<&crate::util::prefilter::Prefilter>, state: &mut crate::hybrid::dfa::OverlappingState) -> Result<(), crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:466-564`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L466-L564)*

### `find_overlapping_rev`

```rust
fn find_overlapping_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, state: &mut crate::hybrid::dfa::OverlappingState) -> Result<(), crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:567-664`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L567-L664)*

### `init_fwd`

```rust
fn init_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<crate::hybrid::id::LazyStateID, crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:667-677`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L667-L677)*

### `init_rev`

```rust
fn init_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<crate::hybrid::id::LazyStateID, crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:680-690`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L680-L690)*

### `eoi_fwd`

```rust
fn eoi_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, sid: &mut crate::hybrid::id::LazyStateID, mat: &mut Option<crate::util::search::HalfMatch>) -> Result<(), crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:693-726`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L693-L726)*

### `eoi_rev`

```rust
fn eoi_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, sid: &mut crate::hybrid::id::LazyStateID, mat: &mut Option<crate::util::search::HalfMatch>) -> Result<(), crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:729-760`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L729-L760)*

### `prefilter_restart`

```rust
fn prefilter_restart(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, at: usize) -> Result<crate::hybrid::id::LazyStateID, crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:787-796`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L787-L796)*

Re-compute the starting state that a DFA should be in after finding a
prefilter candidate match at the position `at`.

It is always correct to call this, but not always necessary. Namely,
whenever the DFA has a universal start state, the DFA can remain in the
start state that it was in when it ran the prefilter. Why? Because in that
case, there is only one start state.

When does a DFA have a universal start state? In precisely cases where
it has no look-around assertions in its prefix. So for example, `\bfoo`
does not have a universal start state because the start state depends on
whether the byte immediately before the start position is a word byte or
not. However, `foo\b` does have a universal start state because the word
boundary does not appear in the pattern's prefix.

So... most cases don't need this, but when a pattern doesn't have a
universal start state, then after a prefilter candidate has been found, the
current state *must* be re-litigated as if computing the start state at the
beginning of the search because it might change. That is, not all start
states are created equal.

Why avoid it? Because while it's not super expensive, it isn't a trivial
operation to compute the start state. It is much better to avoid it and
just state in the current state if you know it to be correct.

### `gave_up`

```rust
fn gave_up(offset: usize) -> crate::util::search::MatchError
```

*Defined in [`regex-automata-0.4.13/src/hybrid/search.rs:800-802`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/search.rs#L800-L802)*

A convenience routine for constructing a "gave up" match error.

