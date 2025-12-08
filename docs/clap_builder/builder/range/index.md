*[clap_builder](../../index.md) / [builder](../index.md) / [range](index.md)*

---

# Module `range`

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

- `const EMPTY: Self`

- `const SINGLE: Self`

- `const OPTIONAL: Self`

- `const FULL: Self`

- `fn new(range: impl Into<Self>) -> Self`

- `fn raw(start_inclusive: usize, end_inclusive: usize) -> Self`

- `fn min_values(self: &Self) -> usize`

- `fn max_values(self: &Self) -> usize`

- `fn takes_values(self: &Self) -> bool`

- `fn is_unbounded(self: &Self) -> bool`

- `fn is_fixed(self: &Self) -> bool`

- `fn is_multiple(self: &Self) -> bool`

- `fn num_values(self: &Self) -> Option<usize>`

- `fn accepts_more(self: &Self, current: usize) -> bool`

#### Trait Implementations

##### `impl Clone for ValueRange`

- `fn clone(self: &Self) -> ValueRange` — [`ValueRange`](#valuerange)

##### `impl Copy for ValueRange`

##### `impl Debug for ValueRange`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ValueRange`

- `fn default() -> Self`

##### `impl Display for ValueRange`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ValueRange`

##### `impl Hash for ValueRange`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for ValueRange`

- `fn into_resettable(self: Self) -> Resettable<ValueRange>` — [`Resettable`](../resettable/index.md), [`ValueRange`](#valuerange)

##### `impl PartialEq for ValueRange`

- `fn eq(self: &Self, other: &ValueRange) -> bool` — [`ValueRange`](#valuerange)

##### `impl RangeBounds for ValueRange`

- `fn start_bound(self: &Self) -> std::ops::Bound<&usize>`

- `fn end_bound(self: &Self) -> std::ops::Bound<&usize>`

##### `impl StructuralPartialEq for ValueRange`

##### `impl<T> ToString for ValueRange`

- `fn to_string(self: &Self) -> String`

