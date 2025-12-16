*[itertools](../index.md) / [iter_index](index.md)*

---

# Module `iter_index`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private_iter_index`](#private-iter-index) | mod |  |
| [`IteratorIndex`](#iteratorindex) | trait | Used by [`Itertools::get`] to know which iterator to turn different ranges into. |
| [`get`](#get) | fn |  |

## Modules

- [`private_iter_index`](private_iter_index/index.md)

## Traits

### `IteratorIndex<I>`

```rust
trait IteratorIndex<I>: private_iter_index::Sealed
where
    I: Iterator { ... }
```

*Defined in [`itertools-0.14.0/src/iter_index.rs:22-34`](../../../.source_1765894658/itertools-0.14.0/src/iter_index.rs#L22-L34)*

Used by `Itertools::get` to know which iterator
to turn different ranges into.

#### Associated Types

- `type Output: 1`

#### Required Methods

- `fn index(self, from: I) -> <Self as >::Output`

  Returns an adapted iterator for the current index.
  
  Prefer calling `Itertools::get` instead
  of calling this directly.

#### Implementors

- `core::ops::Range<usize>`
- `core::ops::RangeFrom<usize>`
- `core::ops::RangeFull`
- `core::ops::RangeInclusive<usize>`
- `core::ops::RangeTo<usize>`
- `core::ops::RangeToInclusive<usize>`

## Functions

### `get`

```rust
fn get<I, R>(iter: I, index: R) -> <R as >::Output
where
    I: IntoIterator,
    R: IteratorIndex<<I as >::IntoIter>
```

*Defined in [`itertools-0.14.0/src/iter_index.rs:110-116`](../../../.source_1765894658/itertools-0.14.0/src/iter_index.rs#L110-L116)*

