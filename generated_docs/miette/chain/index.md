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

*Defined in [`miette-7.6.0/src/chain.rs:32-34`](../../../.source_1765633015/miette-7.6.0/src/chain.rs#L32-L34)*

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

##### `impl Any for Chain<'a>`

- <span id="chain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chain<'a>`

- <span id="chain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chain<'a>`

- <span id="chain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Chain<'a>`

- <span id="chain-clone"></span>`fn clone(&self) -> Chain<'a>` — [`Chain`](#chain)

##### `impl CloneToUninit for Chain<'a>`

- <span id="chain-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for Chain<'_>`

- <span id="chain-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for Chain<'_>`

- <span id="chain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Chain<'_>`

- <span id="chain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Chain<'a>`

- <span id="chain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Chain<'a>`

- <span id="chain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Chain<'a>`

- <span id="chain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Chain<'a>`

- <span id="chain-iterator-type-item"></span>`type Item = &'a dyn Error`

- <span id="chain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="chain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl OwoColorize for Chain<'a>`

##### `impl ToOwned for Chain<'a>`

- <span id="chain-toowned-type-owned"></span>`type Owned = T`

- <span id="chain-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chain-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chain<'a>`

- <span id="chain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chain<'a>`

- <span id="chain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miette-7.6.0/src/chain.rs:37-44`](../../../.source_1765633015/miette-7.6.0/src/chain.rs#L37-L44)*

#### Trait Implementations

##### `impl Any for ChainState<'a>`

- <span id="chainstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChainState<'a>`

- <span id="chainstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChainState<'a>`

- <span id="chainstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ChainState<'a>`

- <span id="chainstate-clone"></span>`fn clone(&self) -> ChainState<'a>` — [`ChainState`](#chainstate)

##### `impl CloneToUninit for ChainState<'a>`

- <span id="chainstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for ChainState<'a>`

- <span id="chainstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChainState<'a>`

- <span id="chainstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ChainState<'a>`

##### `impl ToOwned for ChainState<'a>`

- <span id="chainstate-toowned-type-owned"></span>`type Owned = T`

- <span id="chainstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chainstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ChainState<'a>`

- <span id="chainstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chainstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChainState<'a>`

- <span id="chainstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chainstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

