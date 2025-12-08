*[clap_builder](../index.md) / [mkeymap](index.md)*

---

# Module `mkeymap`

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

- `fn clone(self: &Self) -> Key` — [`Key`](#key)

##### `impl Debug for Key`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Key`

##### `impl PartialEq for Key`

- `fn eq(self: &Self, other: &Key) -> bool` — [`Key`](#key)

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

- `fn contains<K>(self: &Self, key: K) -> bool`

- `fn push(self: &mut Self, new_arg: Arg)` — [`Arg`](../index.md)

- `fn get<K: ?Sized>(self: &Self, key: &K) -> Option<&Arg>` — [`Arg`](../index.md)

- `fn keys(self: &Self) -> impl Iterator<Item = &KeyType>` — [`KeyType`](#keytype)

- `fn args(self: &Self) -> impl Iterator<Item = &Arg>` — [`Arg`](../index.md)

- `fn args_mut(self: &mut Self) -> impl Iterator<Item = &mut Arg>` — [`Arg`](../index.md)

- `fn mut_args<F>(self: &mut Self, f: F)`

- `fn _build(self: &mut Self)`

- `fn remove_by_name(self: &mut Self, name: &str) -> Option<Arg>` — [`Arg`](../index.md)

#### Trait Implementations

##### `impl Clone for MKeyMap`

- `fn clone(self: &Self) -> MKeyMap` — [`MKeyMap`](#mkeymap)

##### `impl Debug for MKeyMap`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MKeyMap`

- `fn default() -> MKeyMap` — [`MKeyMap`](#mkeymap)

##### `impl Eq for MKeyMap`

##### `impl Index for MKeyMap`

- `type Output = Arg`

- `fn index(self: &Self, key: &KeyType) -> &<Self as >::Output` — [`KeyType`](#keytype)

##### `impl PartialEq for MKeyMap`

- `fn eq(self: &Self, other: &MKeyMap) -> bool` — [`MKeyMap`](#mkeymap)

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

- `fn is_position(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for KeyType`

- `fn clone(self: &Self) -> KeyType` — [`KeyType`](#keytype)

##### `impl Debug for KeyType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for KeyType`

##### `impl Hash for KeyType`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for KeyType`

- `fn eq(self: &Self, rhs: &str) -> bool`

##### `impl StructuralPartialEq for KeyType`

## Functions

### `append_keys`

```rust
fn append_keys(keys: &mut Vec<Key>, arg: &crate::Arg, index: usize)
```

Generate key types for an specific Arg.

