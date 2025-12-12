*[miette](../index.md) / [chain](index.md)*

---

# Module `chain`

Iterate over error `.source()` chains.

NOTE: This module is taken wholesale from <https://crates.io/crates/eyre>.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Chain`](#chain) | struct | Iterator of a chain of source errors. |
| [`ChainState`](#chainstate) | enum |  |

## Structs

### `Chain<'a>`

```rust
struct Chain<'a> {
    state: crate::chain::ChainState<'a>,
}
```

*Defined in [`miette-7.6.0/src/chain.rs:32-34`](../../../.source_1765521767/miette-7.6.0/src/chain.rs#L32-L34)*

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

- <span id="chain-new"></span>`fn new(head: &'a dyn StdError) -> Self`

#### Trait Implementations

##### `impl Clone for Chain<'a>`

- <span id="chain-clone"></span>`fn clone(&self) -> Chain<'a>` — [`Chain`](#chain)

##### `impl Default for Chain<'_>`

- <span id="chain-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for Chain<'_>`

- <span id="chain-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Chain<'_>`

- <span id="chain-len"></span>`fn len(&self) -> usize`

##### `impl IntoIterator for Chain<'a>`

- <span id="chain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Chain<'a>`

- <span id="chain-iterator-type-item"></span>`type Item = &'a dyn Error`

- <span id="chain-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="chain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl OwoColorize for Chain<'a>`

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

*Defined in [`miette-7.6.0/src/chain.rs:37-44`](../../../.source_1765521767/miette-7.6.0/src/chain.rs#L37-L44)*

#### Trait Implementations

##### `impl Clone for ChainState<'a>`

- <span id="chainstate-clone"></span>`fn clone(&self) -> ChainState<'a>` — [`ChainState`](#chainstate)

##### `impl OwoColorize for ChainState<'a>`

