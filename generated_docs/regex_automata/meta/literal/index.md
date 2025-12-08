*[regex_automata](../../index.md) / [meta](../index.md) / [literal](index.md)*

---

# Module `literal`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alternation_literals`](#alternation_literals) | fn | Pull out an alternation of literals from the given sequence of HIR |

## Functions

### `alternation_literals`

```rust
fn alternation_literals(info: &crate::meta::regex::RegexInfo, hirs: &[&regex_syntax::hir::Hir]) -> Option<alloc::vec::Vec<alloc::vec::Vec<u8>>>
```

Pull out an alternation of literals from the given sequence of HIR
expressions.

There are numerous ways for this to fail. Generally, this only applies
to regexes of the form 'foo|bar|baz|...|quux'. It can also fail if there
are "too few" alternates, in which case, the regex engine is likely faster.

And currently, this only returns something when 'hirs.len() == 1'.

