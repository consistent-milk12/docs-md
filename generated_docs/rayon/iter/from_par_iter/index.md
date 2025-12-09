*[rayon](../../index.md) / [iter](../index.md) / [from_par_iter](index.md)*

---

# Module `from_par_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`collect_extended`](#collect_extended) | fn | Creates an empty default collection and extends it. |
| [`collect_string!`](#collect_string) | macro |  |

## Functions

### `collect_extended`

```rust
fn collect_extended<C, I>(par_iter: I) -> C
where
    I: IntoParallelIterator,
    C: ParallelExtend<<I as >::Item> + Default
```

*Defined in [`rayon-1.11.0/src/iter/from_par_iter.rs:14-22`](../../../../.source_1765210505/rayon-1.11.0/src/iter/from_par_iter.rs#L14-L22)*

Creates an empty default collection and extends it.

## Macros

### `collect_string!`

*Defined in [`rayon-1.11.0/src/iter/from_par_iter.rs:179-201`](../../../../.source_1765210505/rayon-1.11.0/src/iter/from_par_iter.rs#L179-L201)*

