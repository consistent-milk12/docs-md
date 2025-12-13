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

*Defined in [`bitflags-2.10.0/src/iter.rs:13-16`](../../../.source_1765633015/bitflags-2.10.0/src/iter.rs#L13-L16)*

An iterator over flags values.

This iterator will yield flags values for contained, defined flags first, with any remaining bits yielded
as a final flags value.

#### Implementations

- <span id="iter-new"></span>`fn new(flags: &B) -> Self`

#### Trait Implementations

##### `impl Any for Iter<B>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<B>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<B>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Iter<B>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Iter<B>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<B>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<B: Flags> Iterator for Iter<B>`

- <span id="iter-iterator-type-item"></span>`type Item = B`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for Iter<B>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<B>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterNames<B: 'static>`

```rust
struct IterNames<B: 'static> {
    flags: &'static [crate::Flag<B>],
    idx: usize,
    source: B,
    remaining: B,
}
```

*Defined in [`bitflags-2.10.0/src/iter.rs:67-72`](../../../.source_1765633015/bitflags-2.10.0/src/iter.rs#L67-L72)*

An iterator over flags values.

This iterator only yields flags values for contained, defined, named flags. Any remaining bits
won't be yielded, but can be found with the `IterNames::remaining` method.

#### Implementations

- <span id="iternames-new"></span>`fn new(flags: &B) -> Self`

#### Trait Implementations

##### `impl Any for IterNames<B>`

- <span id="iternames-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterNames<B>`

- <span id="iternames-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterNames<B>`

- <span id="iternames-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IterNames<B>`

- <span id="iternames-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterNames<B>`

- <span id="iternames-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterNames<B>`

- <span id="iternames-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iternames-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iternames-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<B: Flags> Iterator for IterNames<B>`

- <span id="iternames-iterator-type-item"></span>`type Item = (&'static str, B)`

- <span id="iternames-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for IterNames<B>`

- <span id="iternames-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iternames-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterNames<B>`

- <span id="iternames-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iternames-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterDefinedNames<B: 'static>`

```rust
struct IterDefinedNames<B: 'static> {
    flags: &'static [crate::Flag<B>],
    idx: usize,
}
```

*Defined in [`bitflags-2.10.0/src/iter.rs:153-156`](../../../.source_1765633015/bitflags-2.10.0/src/iter.rs#L153-L156)*

An iterator over all defined named flags.

This iterator will yield flags values for all defined named flags, regardless of
whether they are contained in a particular flags value.

#### Implementations

- <span id="iterdefinednames-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Any for IterDefinedNames<B>`

- <span id="iterdefinednames-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterDefinedNames<B>`

- <span id="iterdefinednames-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterDefinedNames<B>`

- <span id="iterdefinednames-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IterDefinedNames<B>`

- <span id="iterdefinednames-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterDefinedNames<B>`

- <span id="iterdefinednames-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterDefinedNames<B>`

- <span id="iterdefinednames-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterdefinednames-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterdefinednames-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<B: Flags> Iterator for IterDefinedNames<B>`

- <span id="iterdefinednames-iterator-type-item"></span>`type Item = (&'static str, B)`

- <span id="iterdefinednames-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for IterDefinedNames<B>`

- <span id="iterdefinednames-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterdefinednames-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterDefinedNames<B>`

- <span id="iterdefinednames-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterdefinednames-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

