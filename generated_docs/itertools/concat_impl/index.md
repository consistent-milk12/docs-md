*[itertools](../index.md) / [concat_impl](index.md)*

---

# Module `concat_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`concat`](#concat) | fn | Combine all an iterator's elements into one element by using [`Extend`]. |

## Functions

### `concat`

```rust
fn concat<I>(iterable: I) -> <I as >::Item
where
    I: IntoIterator,
    <I as >::Item: Extend<<<I as IntoIterator>::Item as IntoIterator>::Item> + IntoIterator + Default
```

*Defined in [`itertools-0.14.0/src/concat_impl.rs:15-27`](../../../.source_1765900590/itertools-0.14.0/src/concat_impl.rs#L15-L27)*

Combine all an iterator's elements into one element by using `Extend`.

`IntoIterator`-enabled version of [`Itertools::concat`](crate::Itertools::concat).

This combinator will extend the first item with each of the rest of the
items of the iterator. If the iterator is empty, the default value of
`I::Item` is returned.

```rust
use itertools::concat;

let input = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
assert_eq!(concat(input), vec![1, 2, 3, 4, 5, 6]);
```

