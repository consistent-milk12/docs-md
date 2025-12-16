*[itertools](../index.md) / [extrema_set](index.md)*

---

# Module `extrema_set`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`min_set_impl`](#min-set-impl) | fn | Implementation guts for `min_set`, `min_set_by`, and `min_set_by_key`. |
| [`max_set_impl`](#max-set-impl) | fn | Implementation guts for `ax_set`, `max_set_by`, and `max_set_by_key`. |

## Functions

### `min_set_impl`

```rust
fn min_set_impl<I, K, F, Compare>(it: I, key_for: F, compare: Compare) -> alloc::vec::Vec<<I as >::Item>
where
    I: Iterator,
    F: FnMut(&<I as >::Item) -> K,
    Compare: FnMut(&<I as >::Item, &<I as >::Item, &K, &K) -> std::cmp::Ordering
```

*Defined in [`itertools-0.14.0/src/extrema_set.rs:5-37`](../../../.source_1765900590/itertools-0.14.0/src/extrema_set.rs#L5-L37)*

Implementation guts for `min_set`, `min_set_by`, and `min_set_by_key`.

### `max_set_impl`

```rust
fn max_set_impl<I, K, F, Compare>(it: I, key_for: F, compare: Compare) -> alloc::vec::Vec<<I as >::Item>
where
    I: Iterator,
    F: FnMut(&<I as >::Item) -> K,
    Compare: FnMut(&<I as >::Item, &<I as >::Item, &K, &K) -> std::cmp::Ordering
```

*Defined in [`itertools-0.14.0/src/extrema_set.rs:40-49`](../../../.source_1765900590/itertools-0.14.0/src/extrema_set.rs#L40-L49)*

Implementation guts for `ax_set`, `max_set_by`, and `max_set_by_key`.

