*[clap_builder](../index.md) / [mkeymap](index.md)*

---

# Module `mkeymap`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Key`](#key) | struct |  |
| [`MKeyMap`](#mkeymap) | struct |  |
| [`KeyType`](#keytype) | enum |  |
| [`append_keys`](#append_keys) | fn | Generate key types for an specific Arg. |

## Structs

### `Key`

```rust
struct Key {
    key: KeyType,
    index: usize,
}
```

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

##### `impl Debug for Key`

- <span id="key-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Key`

##### `impl PartialEq for Key`

- <span id="key-eq"></span>`fn eq(&self, other: &Key) -> bool` — [`Key`](#key)

##### `impl StructuralPartialEq for Key`

### `MKeyMap`

```rust
struct MKeyMap {
    args: Vec<crate::Arg>,
    keys: Vec<Key>,
}
```

#### Fields

- **`args`**: `Vec<crate::Arg>`

  All of the arguments.

- **`keys`**: `Vec<Key>`

  Will be set after `_build()`.

#### Implementations

- <span id="mkeymap-contains"></span>`fn contains<K>(&self, key: K) -> bool`

- <span id="mkeymap-push"></span>`fn push(&mut self, new_arg: Arg)` — [`Arg`](../index.md)

- <span id="mkeymap-get"></span>`fn get<K: ?Sized>(&self, key: &K) -> Option<&Arg>` — [`Arg`](../index.md)

- <span id="mkeymap-keys"></span>`fn keys(&self) -> impl Iterator<Item = &KeyType>` — [`KeyType`](#keytype)

- <span id="mkeymap-args"></span>`fn args(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](../index.md)

- <span id="mkeymap-args-mut"></span>`fn args_mut(&mut self) -> impl Iterator<Item = &mut Arg>` — [`Arg`](../index.md)

- <span id="mkeymap-mut-args"></span>`fn mut_args<F>(&mut self, f: F)`

- <span id="mkeymap-build"></span>`fn _build(&mut self)`

- <span id="mkeymap-remove-by-name"></span>`fn remove_by_name(&mut self, name: &str) -> Option<Arg>` — [`Arg`](../index.md)

#### Trait Implementations

##### `impl Clone for MKeyMap`

- <span id="mkeymap-clone"></span>`fn clone(&self) -> MKeyMap` — [`MKeyMap`](#mkeymap)

##### `impl Debug for MKeyMap`

- <span id="mkeymap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MKeyMap`

- <span id="mkeymap-default"></span>`fn default() -> MKeyMap` — [`MKeyMap`](#mkeymap)

##### `impl Eq for MKeyMap`

##### `impl Index for MKeyMap`

- <span id="mkeymap-output"></span>`type Output = Arg`

- <span id="mkeymap-index"></span>`fn index(&self, key: &KeyType) -> &<Self as >::Output` — [`KeyType`](#keytype)

##### `impl PartialEq for MKeyMap`

- <span id="mkeymap-eq"></span>`fn eq(&self, other: &MKeyMap) -> bool` — [`MKeyMap`](#mkeymap)

##### `impl StructuralPartialEq for MKeyMap`

## Enums

### `KeyType`

```rust
enum KeyType {
    Short(char),
    Long(crate::builder::OsStr),
    Position(usize),
}
```

#### Implementations

- <span id="keytype-is-position"></span>`fn is_position(&self) -> bool`

#### Trait Implementations

##### `impl Clone for KeyType`

- <span id="keytype-clone"></span>`fn clone(&self) -> KeyType` — [`KeyType`](#keytype)

##### `impl Debug for KeyType`

- <span id="keytype-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for KeyType`

##### `impl Hash for KeyType`

- <span id="keytype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for KeyType`

- <span id="keytype-eq"></span>`fn eq(&self, rhs: &usize) -> bool`

##### `impl StructuralPartialEq for KeyType`

## Functions

### `append_keys`

```rust
fn append_keys(keys: &mut Vec<Key>, arg: &crate::Arg, index: usize)
```

Generate key types for an specific Arg.

