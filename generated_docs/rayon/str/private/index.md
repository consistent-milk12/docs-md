*[rayon](../../index.md) / [str](../index.md) / [private](index.md)*

---

# Module `private`

We hide the `Pattern` trait in a private module, as its API is not meant
for general consumption.  If we could have privacy on trait items, then it
would be nicer to have its basic existence and implementors public while
keeping all of the methods private.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pattern`](#pattern) | trait | Pattern-matching trait for `ParallelString`, somewhat like a mix of `std::str::pattern::{Pattern, Searcher}`. |

## Traits

### `Pattern`

```rust
trait Pattern: Sized + Sync + Send { ... }
```

*Defined in [`rayon-1.11.0/src/str.rs:364-381`](../../../../.source_1765521767/rayon-1.11.0/src/str.rs#L364-L381)*

Pattern-matching trait for `ParallelString`, somewhat like a mix of
`std::str::pattern::{Pattern, Searcher}`.

Implementing this trait is not permitted outside of `rayon`.

#### Required Methods

- `fn find_in(&self, haystack: &str) -> Option<usize>`

- `fn rfind_in(&self, haystack: &str) -> Option<usize>`

- `fn is_suffix_of(&self, haystack: &str) -> bool`

- `fn fold_splits<'ch, F>(&self, haystack: &'ch str, folder: F, skip_last: bool) -> F`

- `fn fold_inclusive_splits<'ch, F>(&self, haystack: &'ch str, folder: F) -> F`

- `fn fold_matches<'ch, F>(&self, haystack: &'ch str, folder: F) -> F`

- `fn fold_match_indices<'ch, F>(&self, haystack: &'ch str, folder: F, base: usize) -> F`

#### Implementors

- `&[char; N]`
- `&[char]`
- `FN`
- `[char; N]`
- `char`

