*[bitflags](../index.md) / [iter](index.md)*

---

# Module `iter`

Yield the bits of a source flags value in a set of contained flags values.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Iter`](#iter) | struct | An iterator over flags values. |
| [`IterNames`](#iternames) | struct | An iterator over flags values. |
| [`IterDefinedNames`](#iterdefinednames) | struct | An iterator over all defined named flags. |

## Structs

### `Iter<B: 'static>`

```rust
struct Iter<B: 'static> {
    inner: IterNames<B>,
    done: bool,
}
```

An iterator over flags values.

This iterator will yield flags values for contained, defined flags first, with any remaining bits yielded
as a final flags value.

#### Implementations

- <span id="iter-new"></span>`fn new(flags: &B) -> Self`

#### Trait Implementations

##### `impl<I> IntoIterator for Iter<B>`

- <span id="iter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<B: Flags> Iterator for Iter<B>`

- <span id="iter-item"></span>`type Item = B`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `IterNames<B: 'static>`

```rust
struct IterNames<B: 'static> {
    flags: &'static [crate::Flag<B>],
    idx: usize,
    source: B,
    remaining: B,
}
```

An iterator over flags values.

This iterator only yields flags values for contained, defined, named flags. Any remaining bits
won't be yielded, but can be found with the `IterNames::remaining` method.

#### Implementations

- <span id="iternames-remaining"></span>`fn remaining(&self) -> &B`

#### Trait Implementations

##### `impl<I> IntoIterator for IterNames<B>`

- <span id="iternames-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iternames-intoiter"></span>`type IntoIter = I`

- <span id="iternames-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<B: Flags> Iterator for IterNames<B>`

- <span id="iternames-item"></span>`type Item = (&'static str, B)`

- <span id="iternames-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `IterDefinedNames<B: 'static>`

```rust
struct IterDefinedNames<B: 'static> {
    flags: &'static [crate::Flag<B>],
    idx: usize,
}
```

An iterator over all defined named flags.

This iterator will yield flags values for all defined named flags, regardless of
whether they are contained in a particular flags value.

#### Implementations

- <span id="iterdefinednames-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<I> IntoIterator for IterDefinedNames<B>`

- <span id="iterdefinednames-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterdefinednames-intoiter"></span>`type IntoIter = I`

- <span id="iterdefinednames-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<B: Flags> Iterator for IterDefinedNames<B>`

- <span id="iterdefinednames-item"></span>`type Item = (&'static str, B)`

- <span id="iterdefinednames-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

