*[regex_automata](../../index.md) / [meta](../index.md) / [reverse_inner](index.md)*

---

# Module `reverse_inner`

A module dedicated to plucking inner literals out of a regex pattern, and
then constructing a prefilter for them. We also include a regex pattern
"prefix" that corresponds to the bits of the regex that need to match before
the literals do. The reverse inner optimization then proceeds by looking for
matches of the inner literal(s), and then doing a reverse search of the prefix
from the start of the literal match to find the overall start position of the
match.

The essential invariant we want to uphold here is that the literals we return
reflect a set where *at least* one of them must match in order for the overall
regex to match. We also need to maintain the invariant that the regex prefix
returned corresponds to the entirety of the regex up until the literals we
return.

This somewhat limits what we can do. That is, if we a regex like
`\w+(@!|%%)\w+`, then we can pluck the `{@!, %%}` out and build a prefilter
from it. Then we just need to compile `\w+` in reverse. No fuss no muss. But if
we have a regex like \d+@!|\w+%%`, then we get kind of stymied. Technically,
we could still extract `{@!, %%}`, and it is true that at least of them must
match. But then, what is our regex prefix? Again, in theory, that could be
`\d+|\w+`, but that's not quite right, because the `\d+` only matches when `@!`
matches, and `\w+` only matches when `%%` matches.

All of that is technically possible to do, but it seemingly requires a lot of
sophistication and machinery. Probably the way to tackle that is with some kind
of formalism and approach this problem more generally.

For now, the code below basically just looks for a top-level concatenation.
And if it can find one, it looks for literals in each of the direct child
sub-expressions of that concatenation. If some good ones are found, we return
those and a concatenation of the Hir expressions seen up to that point.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`extract`](#extract) | fn | Attempts to extract an "inner" prefilter from the given HIR expressions. |
| [`prefilter`](#prefilter) | fn | Attempt to extract a prefilter from an HIR expression. |
| [`top_concat`](#top-concat) | fn | Looks for a "top level" HirKind::Concat item in the given HIR. |
| [`flatten`](#flatten) | fn | Returns a copy of the given HIR but with all capturing groups removed. |

## Functions

### `extract`

```rust
fn extract(hirs: &[&regex_syntax::hir::Hir]) -> Option<(regex_syntax::hir::Hir, crate::util::prefilter::Prefilter)>
```

*Defined in [`regex-automata-0.4.13/src/meta/reverse_inner.rs:53-116`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/reverse_inner.rs#L53-L116)*

Attempts to extract an "inner" prefilter from the given HIR expressions. If
one was found, then a concatenation of the HIR expressions that precede it
is returned.

The idea here is that the prefilter returned can be used to find candidate
matches. And then the HIR returned can be used to build a reverse regex
matcher, which will find the start of the candidate match. Finally, the
match still has to be confirmed with a normal anchored forward scan to find
the end position of the match.

Note that this assumes leftmost-first match semantics, so callers must
not call this otherwise.

### `prefilter`

```rust
fn prefilter(hir: &regex_syntax::hir::Hir) -> Option<crate::util::prefilter::Prefilter>
```

*Defined in [`regex-automata-0.4.13/src/meta/reverse_inner.rs:127-154`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/reverse_inner.rs#L127-L154)*

Attempt to extract a prefilter from an HIR expression.

We do a little massaging here to do our best that the prefilter we get out
of this is *probably* fast. Basically, the false positive rate has a much
higher impact for things like the reverse inner optimization because more
work needs to potentially be done for each candidate match.

Note that this assumes leftmost-first match semantics, so callers must
not call this otherwise.

### `top_concat`

```rust
fn top_concat(hir: &regex_syntax::hir::Hir) -> Option<alloc::vec::Vec<regex_syntax::hir::Hir>>
```

*Defined in [`regex-automata-0.4.13/src/meta/reverse_inner.rs:166-200`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/reverse_inner.rs#L166-L200)*

Looks for a "top level" HirKind::Concat item in the given HIR. This will
try to return one even if it's embedded in a capturing group, but is
otherwise pretty conservative in what is returned.

The HIR returned is a complete copy of the concat with all capturing
groups removed. In effect, the concat returned is "flattened" with respect
to capturing groups. This makes the detection logic above for prefixes
a bit simpler, and it works because 1) capturing groups never influence
whether a match occurs or not and 2) capturing groups are not used when
doing the reverse inner search to find the start of the match.

### `flatten`

```rust
fn flatten(hir: &regex_syntax::hir::Hir) -> regex_syntax::hir::Hir
```

*Defined in [`regex-automata-0.4.13/src/meta/reverse_inner.rs:203-220`](../../../../.source_1765521767/regex-automata-0.4.13/src/meta/reverse_inner.rs#L203-L220)*

Returns a copy of the given HIR but with all capturing groups removed.

