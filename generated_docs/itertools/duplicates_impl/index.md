*[itertools](../index.md) / [duplicates_impl](index.md)*

---

# Module `duplicates_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`duplicates_by`](#duplicates-by) | fn | Create a new `DuplicatesBy` iterator. |
| [`duplicates`](#duplicates) | fn | Create a new `Duplicates` iterator. |
| [`DuplicatesBy`](#duplicatesby) | type | An iterator adapter to filter for duplicate elements. |
| [`Duplicates`](#duplicates) | type | An iterator adapter to filter out duplicate elements. |

## Modules

- [`private`](private/index.md)

## Functions

### `duplicates_by`

```rust
fn duplicates_by<I, Key, F>(iter: I, f: F) -> DuplicatesBy<I, Key, F>
where
    Key: Eq + Hash,
    F: FnMut(&<I as >::Item) -> Key,
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:195-202`](../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L195-L202)*

Create a new `DuplicatesBy` iterator.

### `duplicates`

```rust
fn duplicates<I>(iter: I) -> Duplicates<I>
where
    I: Iterator,
    <I as >::Item: Eq + Hash
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:210-216`](../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L210-L216)*

Create a new `Duplicates` iterator.

## Type Aliases

### `DuplicatesBy<I, V, F>`

```rust
type DuplicatesBy<I, V, F> = private::DuplicatesBy<I, V, private::ByFn<F>>;
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:192`](../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L192)*

An iterator adapter to filter for duplicate elements.

See [`.duplicates_by()`](crate::Itertools::duplicates_by) for more information.

### `Duplicates<I>`

```rust
type Duplicates<I> = private::DuplicatesBy<I, <I as Iterator>::Item, private::ById>;
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:207`](../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L207)*

An iterator adapter to filter out duplicate elements.

See [`.duplicates()`](crate::Itertools::duplicates) for more information.

