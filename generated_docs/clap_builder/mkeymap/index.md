*[clap_builder](../index.md) / [mkeymap](index.md)*

---

# Module `mkeymap`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Key`](#key) | struct |  |
| [`MKeyMap`](#mkeymap) | struct |  |
| [`KeyType`](#keytype) | enum |  |
| [`append_keys`](#append-keys) | fn | Generate key types for an specific Arg. |

## Structs

### `Key`

```rust
struct Key {
    key: KeyType,
    index: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/mkeymap.rs:9-12`](../../../.source_1765633015/clap_builder-4.5.53/src/mkeymap.rs#L9-L12)*

#### Trait Implementations

##### `impl Any for Key`

- <span id="key-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Key`

- <span id="key-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Key`

- <span id="key-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

##### `impl CloneToUninit for Key`

- <span id="key-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Key`

- <span id="key-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Key`

##### `impl<T> From for Key`

- <span id="key-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Key`

- <span id="key-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Key`

- <span id="key-partialeq-eq"></span>`fn eq(&self, other: &Key) -> bool` — [`Key`](#key)

##### `impl StructuralPartialEq for Key`

##### `impl ToOwned for Key`

- <span id="key-toowned-type-owned"></span>`type Owned = T`

- <span id="key-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="key-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Key`

- <span id="key-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="key-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Key`

- <span id="key-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="key-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MKeyMap`

```rust
struct MKeyMap {
    args: Vec<crate::Arg>,
    keys: Vec<Key>,
}
```

*Defined in [`clap_builder-4.5.53/src/mkeymap.rs:15-22`](../../../.source_1765633015/clap_builder-4.5.53/src/mkeymap.rs#L15-L22)*

#### Fields

- **`args`**: `Vec<crate::Arg>`

  All of the arguments.

- **`keys`**: `Vec<Key>`

  Will be set after `_build()`.

#### Implementations

- <span id="mkeymap-contains"></span>`fn contains<K>(&self, key: K) -> bool`

  If any arg has corresponding key in this map, we can search the key with

  `u64` (for positional argument), `char` (for short flag), `&str` and `OsString`

  (for long flag)

- <span id="mkeymap-push"></span>`fn push(&mut self, new_arg: Arg)` — [`Arg`](../builder/arg/index.md#arg)

  Push an argument in the map.

- <span id="mkeymap-get"></span>`fn get<K: ?Sized>(&self, key: &K) -> Option<&Arg>` — [`Arg`](../builder/arg/index.md#arg)

  Find the arg have corresponding key in this map, we can search the key

  with `u64` (for positional argument), `char` (for short flag), `&str` and

  `OsString` (for long flag)

- <span id="mkeymap-keys"></span>`fn keys(&self) -> impl Iterator<Item = &KeyType>` — [`KeyType`](#keytype)

  Return iterators of all keys.

- <span id="mkeymap-args"></span>`fn args(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](../builder/arg/index.md#arg)

  Return iterators of all args.

- <span id="mkeymap-args-mut"></span>`fn args_mut(&mut self) -> impl Iterator<Item = &mut Arg>` — [`Arg`](../builder/arg/index.md#arg)

  Return mutable iterators of all args.

- <span id="mkeymap-mut-args"></span>`fn mut_args<F>(&mut self, f: F)`

  Mutate every argument.

- <span id="mkeymap-build"></span>`fn _build(&mut self)`

  We need a lazy build here since some we may change args after creating

  the map, you can checkout who uses `args_mut`.

- <span id="mkeymap-remove-by-name"></span>`fn remove_by_name(&mut self, name: &str) -> Option<Arg>` — [`Arg`](../builder/arg/index.md#arg)

  Remove an arg in the graph by Id, usually used by `mut_arg`. Return

  `Some(arg)` if removed.

#### Trait Implementations

##### `impl Any for MKeyMap`

- <span id="mkeymap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MKeyMap`

- <span id="mkeymap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MKeyMap`

- <span id="mkeymap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MKeyMap`

- <span id="mkeymap-clone"></span>`fn clone(&self) -> MKeyMap` — [`MKeyMap`](#mkeymap)

##### `impl CloneToUninit for MKeyMap`

- <span id="mkeymap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MKeyMap`

- <span id="mkeymap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MKeyMap`

- <span id="mkeymap-default"></span>`fn default() -> MKeyMap` — [`MKeyMap`](#mkeymap)

##### `impl Eq for MKeyMap`

##### `impl<T> From for MKeyMap`

- <span id="mkeymap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Index for MKeyMap`

- <span id="mkeymap-index-type-output"></span>`type Output = Arg`

- <span id="mkeymap-index"></span>`fn index(&self, key: &KeyType) -> &<Self as >::Output` — [`KeyType`](#keytype)

##### `impl<U> Into for MKeyMap`

- <span id="mkeymap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MKeyMap`

- <span id="mkeymap-partialeq-eq"></span>`fn eq(&self, other: &MKeyMap) -> bool` — [`MKeyMap`](#mkeymap)

##### `impl StructuralPartialEq for MKeyMap`

##### `impl ToOwned for MKeyMap`

- <span id="mkeymap-toowned-type-owned"></span>`type Owned = T`

- <span id="mkeymap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mkeymap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MKeyMap`

- <span id="mkeymap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mkeymap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MKeyMap`

- <span id="mkeymap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mkeymap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `KeyType`

```rust
enum KeyType {
    Short(char),
    Long(crate::builder::OsStr),
    Position(usize),
}
```

*Defined in [`clap_builder-4.5.53/src/mkeymap.rs:25-29`](../../../.source_1765633015/clap_builder-4.5.53/src/mkeymap.rs#L25-L29)*

#### Implementations

- <span id="keytype-is-position"></span>`fn is_position(&self) -> bool`

#### Trait Implementations

##### `impl Any for KeyType`

- <span id="keytype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KeyType`

- <span id="keytype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KeyType`

- <span id="keytype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for KeyType`

- <span id="keytype-clone"></span>`fn clone(&self) -> KeyType` — [`KeyType`](#keytype)

##### `impl CloneToUninit for KeyType`

- <span id="keytype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for KeyType`

- <span id="keytype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for KeyType`

##### `impl<T> From for KeyType`

- <span id="keytype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for KeyType`

- <span id="keytype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for MKeyMap`

- <span id="mkeymap-index-type-output"></span>`type Output = Arg`

- <span id="mkeymap-index"></span>`fn index(&self, key: &KeyType) -> &<Self as >::Output` — [`KeyType`](#keytype)

##### `impl<U> Into for KeyType`

- <span id="keytype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for KeyType`

- <span id="keytype-partialeq-eq"></span>`fn eq(&self, other: &KeyType) -> bool` — [`KeyType`](#keytype)

##### `impl StructuralPartialEq for KeyType`

##### `impl ToOwned for KeyType`

- <span id="keytype-toowned-type-owned"></span>`type Owned = T`

- <span id="keytype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="keytype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for KeyType`

- <span id="keytype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="keytype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KeyType`

- <span id="keytype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="keytype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `append_keys`

```rust
fn append_keys(keys: &mut Vec<Key>, arg: &crate::Arg, index: usize)
```

*Defined in [`clap_builder-4.5.53/src/mkeymap.rs:165-188`](../../../.source_1765633015/clap_builder-4.5.53/src/mkeymap.rs#L165-L188)*

Generate key types for an specific Arg.

