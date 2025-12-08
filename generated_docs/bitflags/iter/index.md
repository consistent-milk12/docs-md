*[bitflags](../index.md) / [iter](index.md)*

---

# Module `iter`

Yield the bits of a source flags value in a set of contained flags values.

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

- `fn new(flags: &B) -> Self`

#### Trait Implementations

##### `impl<I> IntoIterator for Iter<B>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<B: Flags> Iterator for Iter<B>`

- `type Item = B`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

- `fn new(flags: &B) -> Self`

#### Trait Implementations

##### `impl<I> IntoIterator for IterNames<B>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<B: Flags> Iterator for IterNames<B>`

- `type Item = (&'static str, B)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

- `fn new() -> Self`

#### Trait Implementations

##### `impl<I> IntoIterator for IterDefinedNames<B>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<B: Flags> Iterator for IterDefinedNames<B>`

- `type Item = (&'static str, B)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

