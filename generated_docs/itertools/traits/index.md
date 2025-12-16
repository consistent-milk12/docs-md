*[itertools](../index.md) / [traits](index.md)*

---

# Module `traits`

Traits helpful for using certain `Itertools` methods in generic contexts.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IteratorIndex`](#iteratorindex) | trait |  |
| [`HomogeneousTuple`](#homogeneoustuple) | trait |  |

## Traits

### `IteratorIndex<I>`

```rust
trait IteratorIndex<I>: private_iter_index::Sealed
where
    I: Iterator { ... }
```

*Defined in [`itertools-0.14.0/src/iter_index.rs:22-34`](../../../.source_1765900590/itertools-0.14.0/src/iter_index.rs#L22-L34)*

Used by `Itertools::get` to know which iterator
to turn different ranges into.

#### Associated Types

- `type Output: 1`

#### Required Methods

- `fn IteratorIndex::index(self, from: I) -> <Self as >::Output`

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

### `HomogeneousTuple`

```rust
trait HomogeneousTuple: TupleCollect { ... }
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:15`](../../../.source_1765900590/itertools-0.14.0/src/tuple_impl.rs#L15)*

Implemented for homogeneous tuples of size up to 12.

#### Implementors

- `T`

