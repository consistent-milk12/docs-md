*[rayon](../index.md) / [delegate](index.md)*

---

# Module `delegate`

Macros for delegating newtype iterators to inner types.

## Macros

### `delegate_iterator!`

Creates a parallel iterator implementation which simply wraps an inner type
and delegates all methods inward.  The actual struct must already be
declared with an `inner` field.

The implementation of `IntoParallelIterator` should be added separately.

### `delegate_indexed_iterator!`

Creates an indexed parallel iterator implementation which simply wraps an
inner type and delegates all methods inward.  The actual struct must already
be declared with an `inner` field.

