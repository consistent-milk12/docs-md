*[rayon](../../index.md) / [iter](../index.md) / [from_par_iter](index.md)*

---

# Module `from_par_iter`

## Functions

### `collect_extended`

```rust
fn collect_extended<C, I>(par_iter: I) -> C
where
    I: IntoParallelIterator,
    C: ParallelExtend<<I as >::Item> + Default
```

Creates an empty default collection and extends it.

## Macros

### `collect_string!`

