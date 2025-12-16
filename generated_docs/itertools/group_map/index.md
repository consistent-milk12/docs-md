*[itertools](../index.md) / [group_map](index.md)*

---

# Module `group_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`into_group_map`](#into-group-map) | fn | Return a `HashMap` of keys mapped to a list of their corresponding values. |
| [`into_group_map_by`](#into-group-map-by) | fn |  |

## Functions

### `into_group_map`

```rust
fn into_group_map<I, K, V>(iter: I) -> std::collections::HashMap<K, Vec<V>>
where
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq
```

*Defined in [`itertools-0.14.0/src/group_map.rs:11-23`](../../../.source_1765900590/itertools-0.14.0/src/group_map.rs#L11-L23)*

Return a `HashMap` of keys mapped to a list of their corresponding values.

See [`.into_group_map()`](crate::Itertools::into_group_map)
for more information.

### `into_group_map_by`

```rust
fn into_group_map_by<I, K, V, F>(iter: I, f: F) -> std::collections::HashMap<K, Vec<V>>
where
    I: Iterator<Item = V>,
    K: Hash + Eq,
    F: FnMut(&V) -> K
```

*Defined in [`itertools-0.14.0/src/group_map.rs:25-32`](../../../.source_1765900590/itertools-0.14.0/src/group_map.rs#L25-L32)*

