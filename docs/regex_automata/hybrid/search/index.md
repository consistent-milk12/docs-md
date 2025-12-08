*[regex_automata](../../index.md) / [hybrid](../index.md) / [search](index.md)*

---

# Module `search`

## Functions

### `find_fwd`

```rust
fn find_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

### `find_fwd_imp`

```rust
fn find_fwd_imp(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, pre: Option<&crate::util::prefilter::Prefilter>, earliest: bool) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

### `find_rev`

```rust
fn find_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

### `find_rev_imp`

```rust
fn find_rev_imp(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, earliest: bool) -> Result<Option<crate::util::search::HalfMatch>, crate::util::search::MatchError>
```

### `find_overlapping_fwd`

```rust
fn find_overlapping_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, state: &mut crate::hybrid::dfa::OverlappingState) -> Result<(), crate::util::search::MatchError>
```

### `find_overlapping_fwd_imp`

```rust
fn find_overlapping_fwd_imp(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, pre: Option<&crate::util::prefilter::Prefilter>, state: &mut crate::hybrid::dfa::OverlappingState) -> Result<(), crate::util::search::MatchError>
```

### `find_overlapping_rev`

```rust
fn find_overlapping_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, state: &mut crate::hybrid::dfa::OverlappingState) -> Result<(), crate::util::search::MatchError>
```

### `init_fwd`

```rust
fn init_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<crate::hybrid::id::LazyStateID, crate::util::search::MatchError>
```

### `init_rev`

```rust
fn init_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>) -> Result<crate::hybrid::id::LazyStateID, crate::util::search::MatchError>
```

### `eoi_fwd`

```rust
fn eoi_fwd(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, sid: &mut crate::hybrid::id::LazyStateID, mat: &mut Option<crate::util::search::HalfMatch>) -> Result<(), crate::util::search::MatchError>
```

### `eoi_rev`

```rust
fn eoi_rev(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, sid: &mut crate::hybrid::id::LazyStateID, mat: &mut Option<crate::util::search::HalfMatch>) -> Result<(), crate::util::search::MatchError>
```

### `prefilter_restart`

```rust
fn prefilter_restart(dfa: &crate::hybrid::dfa::DFA, cache: &mut crate::hybrid::dfa::Cache, input: &crate::util::search::Input<'_>, at: usize) -> Result<crate::hybrid::id::LazyStateID, crate::util::search::MatchError>
```

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

A convenience routine for constructing a "gave up" match error.

