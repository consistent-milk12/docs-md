*[clap_builder](../../index.md) / [builder](../index.md) / [range](index.md)*

---

# Module `range`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValueRange`](#valuerange) | struct | Values per occurrence for an argument |

## Structs

### `ValueRange`

```rust
struct ValueRange {
    start_inclusive: usize,
    end_inclusive: usize,
}
```

Values per occurrence for an argument

#### Implementations

- <span id="valuerange-empty"></span>`const EMPTY: Self`

- <span id="valuerange-single"></span>`const SINGLE: Self`

- <span id="valuerange-optional"></span>`const OPTIONAL: Self`

- <span id="valuerange-full"></span>`const FULL: Self`

- <span id="valuerange-new"></span>`fn new(range: impl Into<Self>) -> Self`

- <span id="valuerange-raw"></span>`fn raw(start_inclusive: usize, end_inclusive: usize) -> Self`

- <span id="valuerange-min-values"></span>`fn min_values(&self) -> usize`

- <span id="valuerange-max-values"></span>`fn max_values(&self) -> usize`

- <span id="valuerange-takes-values"></span>`fn takes_values(&self) -> bool`

- <span id="valuerange-is-unbounded"></span>`fn is_unbounded(&self) -> bool`

- <span id="valuerange-is-fixed"></span>`fn is_fixed(&self) -> bool`

- <span id="valuerange-is-multiple"></span>`fn is_multiple(&self) -> bool`

- <span id="valuerange-num-values"></span>`fn num_values(&self) -> Option<usize>`

- <span id="valuerange-accepts-more"></span>`fn accepts_more(&self, current: usize) -> bool`

#### Trait Implementations

##### `impl Clone for ValueRange`

- <span id="valuerange-clone"></span>`fn clone(&self) -> ValueRange` — [`ValueRange`](../index.md)

##### `impl Copy for ValueRange`

##### `impl Debug for ValueRange`

- <span id="valuerange-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ValueRange`

- <span id="valuerange-default"></span>`fn default() -> Self`

##### `impl Display for ValueRange`

- <span id="valuerange-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ValueRange`

##### `impl Hash for ValueRange`

- <span id="valuerange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> IntoResettable for ValueRange`

- <span id="valuerange-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueRange>` — [`Resettable`](../index.md), [`ValueRange`](../index.md)

##### `impl PartialEq for ValueRange`

- <span id="valuerange-eq"></span>`fn eq(&self, other: &ValueRange) -> bool` — [`ValueRange`](../index.md)

##### `impl RangeBounds for ValueRange`

- <span id="valuerange-start-bound"></span>`fn start_bound(&self) -> std::ops::Bound<&usize>`

- <span id="valuerange-end-bound"></span>`fn end_bound(&self) -> std::ops::Bound<&usize>`

##### `impl StructuralPartialEq for ValueRange`

##### `impl<T> ToString for ValueRange`

- <span id="valuerange-to-string"></span>`fn to_string(&self) -> String`

