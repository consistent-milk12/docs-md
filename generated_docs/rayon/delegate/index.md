*[rayon](../index.md) / [delegate](index.md)*

---

# Module `delegate`

Macros for delegating newtype iterators to inner types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`delegate_iterator!`](#delegate-iterator) | macro | Creates a parallel iterator implementation which simply wraps an inner type and delegates all methods inward. |
| [`delegate_indexed_iterator!`](#delegate-indexed-iterator) | macro | Creates an indexed parallel iterator implementation which simply wraps an inner type and delegates all methods inward. |

## Macros

### `delegate_iterator!`

*Defined in [`rayon-1.11.0/src/delegate.rs:11-29`](../../../.source_1765210505/rayon-1.11.0/src/delegate.rs#L11-L29)*

Creates a parallel iterator implementation which simply wraps an inner type
and delegates all methods inward.  The actual struct must already be
declared with an `inner` field.

The implementation of `IntoParallelIterator` should be added separately.

### `delegate_indexed_iterator!`

*Defined in [`rayon-1.11.0/src/delegate.rs:34-61`](../../../.source_1765210505/rayon-1.11.0/src/delegate.rs#L34-L61)*

Creates an indexed parallel iterator implementation which simply wraps an
inner type and delegates all methods inward.  The actual struct must already
be declared with an `inner` field.

