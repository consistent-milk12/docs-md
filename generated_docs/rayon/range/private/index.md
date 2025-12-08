*[rayon](../../index.md) / [range](../index.md) / [private](index.md)*

---

# Module `private`

These traits help drive integer type inference. Without them, an unknown `{integer}` type only
has constraints on `Iter<{integer}>`, which will probably give up and use `i32`. By adding
these traits on the item type, the compiler can see a more direct constraint to infer like
`{integer}: RangeInteger`, which works better. See `test_issue_833` for an example.

They have to be `pub` since they're seen in the public `impl ParallelIterator` constraints, but
we put them in a private modules so they're not actually reachable in our public API.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeInteger`](#rangeinteger) | trait | Implementation details of `ParallelIterator for Iter<Self>` |
| [`IndexedRangeInteger`](#indexedrangeinteger) | trait | Implementation details of `IndexedParallelIterator for Iter<Self>` |

## Traits

### `RangeInteger`

```rust
trait RangeInteger: Sized + Send { ... }
```

Implementation details of `ParallelIterator for Iter<Self>`

#### Required Methods

- `fn drive_unindexed<C>(iter: Iter<Self>, consumer: C) -> <C as >::Result`

- `fn opt_len(iter: &Iter<Self>) -> Option<usize>`

### `IndexedRangeInteger`

```rust
trait IndexedRangeInteger: RangeInteger { ... }
```

Implementation details of `IndexedParallelIterator for Iter<Self>`

#### Required Methods

- `fn drive<C>(iter: Iter<Self>, consumer: C) -> <C as >::Result`

- `fn len(iter: &Iter<Self>) -> usize`

- `fn with_producer<CB>(iter: Iter<Self>, callback: CB) -> <CB as >::Output`

