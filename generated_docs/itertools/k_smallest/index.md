*[itertools](../index.md) / [k_smallest](index.md)*

---

# Module `k_smallest`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`k_smallest_general`](#k-smallest-general) | fn | Consumes a given iterator, returning the minimum elements in **ascending** order. |
| [`k_smallest_relaxed_general`](#k-smallest-relaxed-general) | fn |  |
| [`key_to_cmp`](#key-to-cmp) | fn |  |

## Functions

### `k_smallest_general`

```rust
fn k_smallest_general<I, F>(iter: I, k: usize, comparator: F) -> alloc::vec::Vec<<I as >::Item>
where
    I: Iterator,
    F: FnMut(&<I as >::Item, &<I as >::Item) -> core::cmp::Ordering
```

*Defined in [`itertools-0.14.0/src/k_smallest.rs:5-89`](../../../.source_1765900590/itertools-0.14.0/src/k_smallest.rs#L5-L89)*

Consumes a given iterator, returning the minimum elements in **ascending** order.

### `k_smallest_relaxed_general`

```rust
fn k_smallest_relaxed_general<I, F>(iter: I, k: usize, comparator: F) -> alloc::vec::Vec<<I as >::Item>
where
    I: Iterator,
    F: FnMut(&<I as >::Item, &<I as >::Item) -> core::cmp::Ordering
```

*Defined in [`itertools-0.14.0/src/k_smallest.rs:91-129`](../../../.source_1765900590/itertools-0.14.0/src/k_smallest.rs#L91-L129)*

### `key_to_cmp`

```rust
fn key_to_cmp<T, K, F>(key: F) -> impl FnMut(&T, &T) -> core::cmp::Ordering
where
    F: FnMut(&T) -> K,
    K: Ord
```

*Defined in [`itertools-0.14.0/src/k_smallest.rs:132-138`](../../../.source_1765900590/itertools-0.14.0/src/k_smallest.rs#L132-L138)*

