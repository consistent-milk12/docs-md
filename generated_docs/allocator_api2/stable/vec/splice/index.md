*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [splice](index.md)*

---

# Module `splice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Splice`](#splice) | struct | A splicing iterator for `Vec`. |

## Structs

### `Splice<'a, I: Iterator + 'a, A: Allocator + 'a>`

```rust
struct Splice<'a, I: Iterator + 'a, A: Allocator + 'a> {
    drain: super::Drain<'a, <I as >::Item, A>,
    replace_with: I,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/splice.rs:21-24`](../../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/splice.rs#L21-L24)*

A splicing iterator for `Vec`.

This struct is created by `Vec::splice()`.
See its documentation for more.

# Example

```rust
let mut v = vec![0, 1, 2];
let new = [7, 8];
let iter: std::vec::Splice<_> = v.splice(1.., new);
```

#### Trait Implementations

##### `impl<'a, I: fmt::Debug + Iterator + 'a, A: fmt::Debug + Allocator + 'a> Debug for Splice<'a, I, A>`

- <span id="splice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator, A: Allocator> DoubleEndedIterator for Splice<'_, I, A>`

- <span id="splice-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I: Iterator, A: Allocator> Drop for Splice<'_, I, A>`

- <span id="splice-drop"></span>`fn drop(&mut self)`

##### `impl<I: Iterator, A: Allocator> ExactSizeIterator for Splice<'_, I, A>`

##### `impl<I> IntoIterator for Splice<'a, I, A>`

- <span id="splice-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-type-intoiter"></span>`type IntoIter = I`

- <span id="splice-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator, A: Allocator> Iterator for Splice<'_, I, A>`

- <span id="splice-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="splice-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

