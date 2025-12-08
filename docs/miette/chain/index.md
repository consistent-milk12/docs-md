*[miette](../index.md) / [chain](index.md)*

---

# Module `chain`

Iterate over error `.source()` chains.

NOTE: This module is taken wholesale from <https://crates.io/crates/eyre>.

## Structs

### `Chain<'a>`

```rust
struct Chain<'a> {
    state: crate::chain::ChainState<'a>,
}
```

Iterator of a chain of source errors.

This type is the iterator returned by `Report::chain`.

# Example

```rust
use miette::Report;
use std::io;

pub fn underlying_io_error_kind(error: &Report) -> Option<io::ErrorKind> {
    for cause in error.chain() {
        if let Some(io_error) = cause.downcast_ref::<io::Error>() {
            return Some(io_error.kind());
        }
    }
    None
}
```

#### Implementations

- `fn new(head: &'a dyn StdError) -> Self`

#### Trait Implementations

##### `impl<'a> Clone for Chain<'a>`

- `fn clone(self: &Self) -> Chain<'a>` — [`Chain`](#chain)

##### `impl Default for Chain<'_>`

- `fn default() -> Self`

##### `impl DoubleEndedIterator for Chain<'_>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Chain<'_>`

- `fn len(self: &Self) -> usize`

##### `impl<I> IntoIterator for Chain<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Chain<'a>`

- `type Item = &'a dyn Error`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<D> OwoColorize for Chain<'a>`

## Enums

### `ChainState<'a>`

```rust
enum ChainState<'a> {
    Linked {
        next: Option<&'a dyn StdError>,
    },
    Buffered {
        rest: vec::IntoIter<&'a dyn StdError>,
    },
}
```

#### Trait Implementations

##### `impl<'a> Clone for ChainState<'a>`

- `fn clone(self: &Self) -> ChainState<'a>` — [`ChainState`](#chainstate)

##### `impl<D> OwoColorize for ChainState<'a>`

