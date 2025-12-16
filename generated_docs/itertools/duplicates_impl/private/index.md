*[itertools](../../index.md) / [duplicates_impl](../index.md) / [private](index.md)*

---

# Module `private`

## Contents

- [Structs](#structs)
  - [`DuplicatesBy`](#duplicatesby)
  - [`Meta`](#meta)
  - [`ById`](#byid)
  - [`ByFn`](#byfn)
  - [`KeyValue`](#keyvalue)
  - [`JustValue`](#justvalue)
- [Traits](#traits)
  - [`KeyMethod`](#keymethod)
  - [`KeyXorValue`](#keyxorvalue)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DuplicatesBy`](#duplicatesby) | struct |  |
| [`Meta`](#meta) | struct |  |
| [`ById`](#byid) | struct | Apply the identity function to elements before checking them for equality. |
| [`ByFn`](#byfn) | struct | Apply a user-supplied function to elements before checking them for equality. |
| [`KeyValue`](#keyvalue) | struct |  |
| [`JustValue`](#justvalue) | struct |  |
| [`KeyMethod`](#keymethod) | trait | A keying method for use with `DuplicatesBy` |
| [`KeyXorValue`](#keyxorvalue) | trait |  |

## Structs

### `DuplicatesBy<I: Iterator, Key, F>`

```rust
struct DuplicatesBy<I: Iterator, Key, F> {
    iter: I,
    meta: Meta<Key, F>,
}
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:10-13`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L10-L13)*

#### Implementations

- <span id="duplicatesby-new"></span>`fn new(iter: I, key_method: F) -> Self`

#### Trait Implementations

##### `impl Any for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator, Key: clone::Clone, F: clone::Clone> Clone for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-clone"></span>`fn clone(&self) -> DuplicatesBy<I, Key, F>` — [`DuplicatesBy`](#duplicatesby)

##### `impl CloneToUninit for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, V, F> Debug for DuplicatesBy<I, V, F>`

- <span id="duplicatesby-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, Key, F> DoubleEndedIterator for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> From for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for DuplicatesBy<I, Key, F>`

##### `impl<I> IntoIterator for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="duplicatesby-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="duplicatesby-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, Key, F> Iterator for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="duplicatesby-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="duplicatesby-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for DuplicatesBy<I, Key, F>`

##### `impl MultiUnzip for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-toowned-type-owned"></span>`type Owned = T`

- <span id="duplicatesby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="duplicatesby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="duplicatesby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DuplicatesBy<I, Key, F>`

- <span id="duplicatesby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="duplicatesby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Meta<Key, F>`

```rust
struct Meta<Key, F> {
    used: std::collections::HashMap<Key, bool>,
    pending: usize,
    key_method: F,
}
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:37-41`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L37-L41)*

#### Implementations

- <span id="meta-filter"></span>`fn filter<I>(&mut self, item: I) -> Option<I>`

  Takes an item and returns it back to the caller if it's the second time we see it.
  Otherwise the item is consumed and None is returned

#### Trait Implementations

##### `impl Any for Meta<Key, F>`

- <span id="meta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Meta<Key, F>`

- <span id="meta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Meta<Key, F>`

- <span id="meta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Key: clone::Clone, F: clone::Clone> Clone for Meta<Key, F>`

- <span id="meta-clone"></span>`fn clone(&self) -> Meta<Key, F>` — [`Meta`](#meta)

##### `impl CloneToUninit for Meta<Key, F>`

- <span id="meta-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Meta<Key, F>`

- <span id="meta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Meta<Key, F>`

- <span id="meta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Meta<Key, F>`

##### `impl ToOwned for Meta<Key, F>`

- <span id="meta-toowned-type-owned"></span>`type Owned = T`

- <span id="meta-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="meta-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Meta<Key, F>`

- <span id="meta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="meta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Meta<Key, F>`

- <span id="meta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="meta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ById`

```rust
struct ById;
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:126`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L126)*

Apply the identity function to elements before checking them for equality.

#### Trait Implementations

##### `impl Any for ById`

- <span id="byid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ById`

- <span id="byid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ById`

- <span id="byid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ById`

- <span id="byid-clone"></span>`fn clone(&self) -> ById` — [`ById`](#byid)

##### `impl CloneToUninit for ById`

- <span id="byid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ById`

- <span id="byid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ById`

- <span id="byid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ById`

- <span id="byid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ById`

##### `impl<V> KeyMethod for ById`

- <span id="byid-keymethod-type-container"></span>`type Container = JustValue<V>`

- <span id="byid-keymethod-make"></span>`fn make(&mut self, v: V) -> <Self as >::Container` — [`KeyMethod`](#keymethod)

##### `impl ToOwned for ById`

- <span id="byid-toowned-type-owned"></span>`type Owned = T`

- <span id="byid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="byid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ById`

- <span id="byid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ById`

- <span id="byid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByFn<F>`

```rust
struct ByFn<F>(F);
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:137`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L137)*

Apply a user-supplied function to elements before checking them for equality.

#### Trait Implementations

##### `impl Any for ByFn<F>`

- <span id="byfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByFn<F>`

- <span id="byfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByFn<F>`

- <span id="byfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: clone::Clone> Clone for ByFn<F>`

- <span id="byfn-clone"></span>`fn clone(&self) -> ByFn<F>` — [`ByFn`](#byfn)

##### `impl CloneToUninit for ByFn<F>`

- <span id="byfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<F> Debug for ByFn<F>`

- <span id="byfn-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for ByFn<F>`

- <span id="byfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByFn<F>`

- <span id="byfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ByFn<F>`

##### `impl<K, V, F> KeyMethod for ByFn<F>`

- <span id="byfn-keymethod-type-container"></span>`type Container = KeyValue<K, V>`

- <span id="byfn-keymethod-make"></span>`fn make(&mut self, v: V) -> <Self as >::Container` — [`KeyMethod`](#keymethod)

##### `impl ToOwned for ByFn<F>`

- <span id="byfn-toowned-type-owned"></span>`type Owned = T`

- <span id="byfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="byfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ByFn<F>`

- <span id="byfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByFn<F>`

- <span id="byfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `KeyValue<K, V>`

```rust
struct KeyValue<K, V>(K, V);
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:161`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L161)*

#### Trait Implementations

##### `impl Any for KeyValue<K, V>`

- <span id="keyvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KeyValue<K, V>`

- <span id="keyvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KeyValue<K, V>`

- <span id="keyvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for KeyValue<K, V>`

- <span id="keyvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for KeyValue<K, V>`

- <span id="keyvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for KeyValue<K, V>`

- <span id="keyvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for KeyValue<K, V>`

##### `impl<K, V> KeyXorValue for KeyValue<K, V>`

- <span id="keyvalue-keyxorvalue-key-ref"></span>`fn key_ref(&self) -> &K`

- <span id="keyvalue-keyxorvalue-key"></span>`fn key(self) -> K`

- <span id="keyvalue-keyxorvalue-value"></span>`fn value(self) -> V`

##### `impl<U> TryFrom for KeyValue<K, V>`

- <span id="keyvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="keyvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KeyValue<K, V>`

- <span id="keyvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="keyvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `JustValue<V>`

```rust
struct JustValue<V>(V);
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:175`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L175)*

#### Trait Implementations

##### `impl Any for JustValue<V>`

- <span id="justvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JustValue<V>`

- <span id="justvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JustValue<V>`

- <span id="justvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<V: fmt::Debug> Debug for JustValue<V>`

- <span id="justvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for JustValue<V>`

- <span id="justvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JustValue<V>`

- <span id="justvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for JustValue<V>`

##### `impl<V> KeyXorValue for JustValue<V>`

- <span id="justvalue-keyxorvalue-key-ref"></span>`fn key_ref(&self) -> &V`

- <span id="justvalue-keyxorvalue-key"></span>`fn key(self) -> V`

- <span id="justvalue-keyxorvalue-value"></span>`fn value(self) -> V`

##### `impl<U> TryFrom for JustValue<V>`

- <span id="justvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="justvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JustValue<V>`

- <span id="justvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="justvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `KeyMethod<K, V>`

```rust
trait KeyMethod<K, V> { ... }
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:118-122`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L118-L122)*

A keying method for use with `DuplicatesBy`

#### Associated Types

- `type Container: 1`

#### Required Methods

- `fn KeyMethod::make(&mut self, value: V) -> <Self as >::Container`

#### Implementors

- [`ByFn`](#byfn)
- [`ById`](#byid)

### `KeyXorValue<K, V>`

```rust
trait KeyXorValue<K, V> { ... }
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:154-158`](../../../../.source_1765900590/itertools-0.14.0/src/duplicates_impl.rs#L154-L158)*

#### Required Methods

- `fn KeyXorValue::key_ref(&self) -> &K`

- `fn KeyXorValue::key(self) -> K`

- `fn KeyXorValue::value(self) -> V`

#### Implementors

- [`JustValue`](#justvalue)
- [`KeyValue`](#keyvalue)

