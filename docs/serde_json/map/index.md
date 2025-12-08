*[serde_json](../index.md) / [map](index.md)*

---

# Module `map`

A map of String to serde_json::Value.

By default the map is backed by a `BTreeMap`. Enable the `preserve_order`
feature of serde_json to use `IndexMap` instead.



## Structs

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

Represents a JSON key/value type.

#### Implementations

- `fn new() -> Self`

- `fn with_capacity(capacity: usize) -> Self`

- `fn clear(self: &mut Self)`

- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>` — [`Value`](../value/index.md)

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`

- `fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>` — [`Value`](../value/index.md)

- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../value/index.md)

- `fn insert(self: &mut Self, k: String, v: Value) -> Option<Value>` — [`Value`](../value/index.md)

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>` — [`Value`](../value/index.md)

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>` — [`Value`](../value/index.md)

- `fn append(self: &mut Self, other: &mut Self)`

- `fn entry<S>(self: &mut Self, key: S) -> Entry<'_>` — [`Entry`](#entry)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> Iter<'_>` — [`Iter`](#iter)

- `fn iter_mut(self: &mut Self) -> IterMut<'_>` — [`IterMut`](#itermut)

- `fn keys(self: &Self) -> Keys<'_>` — [`Keys`](#keys)

- `fn values(self: &Self) -> Values<'_>` — [`Values`](#values)

- `fn values_mut(self: &mut Self) -> ValuesMut<'_>` — [`ValuesMut`](#valuesmut)

- `fn into_values(self: Self) -> IntoValues` — [`IntoValues`](#intovalues)

- `fn retain<F>(self: &mut Self, f: F)`

- `fn sort_keys(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for Map<alloc::string::String, crate::value::Value>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl Debug for Map<alloc::string::String, crate::value::Value>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Map<alloc::string::String, crate::value::Value>`

- `fn default() -> Self`

##### `impl<'de> Deserialize for Map<alloc::string::String, crate::value::Value>`

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl<T> DeserializeOwned for Map<K, V>`

##### `impl<'de> Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- `fn deserialize_enum<V>(self: Self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

##### `impl Eq for Map<alloc::string::String, crate::value::Value>`

##### `impl Extend for Map<alloc::string::String, crate::value::Value>`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl FromIterator for Map<alloc::string::String, crate::value::Value>`

- `fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr for crate::map::Map<alloc::string::String, crate::value::Value>`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../error/index.md)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- `type Output = Value`

- `fn index(self: &Self, index: &Q) -> &Value` — [`Value`](../value/index.md)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- `fn index_mut(self: &mut Self, index: &Q) -> &mut Value` — [`Value`](../value/index.md)

##### `impl<'de> IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- `type Deserializer = Map<String, Value>`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl IntoIterator for Map<alloc::string::String, crate::value::Value>`

- `type Item = (String, Value)`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl PartialEq for Map<alloc::string::String, crate::value::Value>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Serialize for Map<alloc::string::String, crate::value::Value>`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

### `VacantEntry<'a>`

```rust
struct VacantEntry<'a> {
    vacant: btree_map::VacantEntry<'a, alloc::string::String, crate::value::Value>,
}
```

A vacant Entry. It is part of the [`Entry`](#entry) enum.

#### Implementations

- `fn key(self: &Self) -> &String`

- `fn insert(self: Self, value: Value) -> &'a mut Value` — [`Value`](../value/index.md)

### `OccupiedEntry<'a>`

```rust
struct OccupiedEntry<'a> {
    occupied: btree_map::OccupiedEntry<'a, alloc::string::String, crate::value::Value>,
}
```

An occupied Entry. It is part of the [`Entry`](#entry) enum.

#### Implementations

- `fn key(self: &Self) -> &String`

- `fn get(self: &Self) -> &Value` — [`Value`](../value/index.md)

- `fn get_mut(self: &mut Self) -> &mut Value` — [`Value`](../value/index.md)

- `fn into_mut(self: Self) -> &'a mut Value` — [`Value`](../value/index.md)

- `fn insert(self: &mut Self, value: Value) -> Value` — [`Value`](../value/index.md)

- `fn remove(self: Self) -> Value` — [`Value`](../value/index.md)

- `fn remove_entry(self: Self) -> (String, Value)` — [`Value`](../value/index.md)

### `Iter<'a>`

```rust
struct Iter<'a> {
    iter: btree_map::Iter<'a, alloc::string::String, crate::value::Value>,
}
```

An iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl<'a> Clone for Iter<'a>`

- `fn clone(self: &Self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl<'a> Debug for Iter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for Iter<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a> ExactSizeIterator for Iter<'a>`

- `fn len(self: &Self) -> usize`

##### `impl<'a> FusedIterator for Iter<'a>`

##### `impl<I> IntoIterator for Iter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Iter<'a>`

- `type Item = (&'a String, &'a Value)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IterMut<'a>`

```rust
struct IterMut<'a> {
    iter: btree_map::IterMut<'a, alloc::string::String, crate::value::Value>,
}
```

A mutable iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl<'a> Debug for IterMut<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for IterMut<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a> ExactSizeIterator for IterMut<'a>`

- `fn len(self: &Self) -> usize`

##### `impl<'a> FusedIterator for IterMut<'a>`

##### `impl<I> IntoIterator for IterMut<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for IterMut<'a>`

- `type Item = (&'a String, &'a mut Value)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IntoIter`

```rust
struct IntoIter {
    iter: btree_map::IntoIter<alloc::string::String, crate::value::Value>,
}
```

An owning iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Debug for IntoIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator for IntoIter`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IntoIter`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator for IntoIter`

##### `impl<I> IntoIterator for IntoIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for IntoIter`

- `type Item = (String, Value)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Keys<'a>`

```rust
struct Keys<'a> {
    iter: btree_map::Keys<'a, alloc::string::String, crate::value::Value>,
}
```

An iterator over a serde_json::Map's keys.

#### Trait Implementations

##### `impl<'a> Clone for Keys<'a>`

- `fn clone(self: &Self) -> Keys<'a>` — [`Keys`](#keys)

##### `impl<'a> Debug for Keys<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for Keys<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a> ExactSizeIterator for Keys<'a>`

- `fn len(self: &Self) -> usize`

##### `impl<'a> FusedIterator for Keys<'a>`

##### `impl<I> IntoIterator for Keys<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Keys<'a>`

- `type Item = &'a String`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Values<'a>`

```rust
struct Values<'a> {
    iter: btree_map::Values<'a, alloc::string::String, crate::value::Value>,
}
```

An iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl<'a> Clone for Values<'a>`

- `fn clone(self: &Self) -> Values<'a>` — [`Values`](#values)

##### `impl<'a> Debug for Values<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for Values<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a> ExactSizeIterator for Values<'a>`

- `fn len(self: &Self) -> usize`

##### `impl<'a> FusedIterator for Values<'a>`

##### `impl<I> IntoIterator for Values<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Values<'a>`

- `type Item = &'a Value`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `ValuesMut<'a>`

```rust
struct ValuesMut<'a> {
    iter: btree_map::ValuesMut<'a, alloc::string::String, crate::value::Value>,
}
```

A mutable iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl<'a> Debug for ValuesMut<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for ValuesMut<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a> ExactSizeIterator for ValuesMut<'a>`

- `fn len(self: &Self) -> usize`

##### `impl<'a> FusedIterator for ValuesMut<'a>`

##### `impl<I> IntoIterator for ValuesMut<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for ValuesMut<'a>`

- `type Item = &'a mut Value`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IntoValues`

```rust
struct IntoValues {
    iter: btree_map::IntoValues<alloc::string::String, crate::value::Value>,
}
```

An owning iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl Debug for IntoValues`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator for IntoValues`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IntoValues`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator for IntoValues`

##### `impl<I> IntoIterator for IntoValues`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for IntoValues`

- `type Item = Value`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

## Enums

### `Entry<'a>`

```rust
enum Entry<'a> {
    Vacant(VacantEntry<'a>),
    Occupied(OccupiedEntry<'a>),
}
```

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the `entry` method on [`Map`](#map).


#### Variants

- **`Vacant`**

  A vacant Entry.

- **`Occupied`**

  An occupied Entry.

#### Implementations

- `fn key(self: &Self) -> &String`

- `fn or_insert(self: Self, default: Value) -> &'a mut Value` — [`Value`](../value/index.md)

- `fn or_insert_with<F>(self: Self, default: F) -> &'a mut Value` — [`Value`](../value/index.md)

- `fn and_modify<F>(self: Self, f: F) -> Self`

## Type Aliases

### `MapImpl<K, V>`

```rust
type MapImpl<K, V> = alloc::collections::BTreeMap<K, V>;
```

### `VacantEntryImpl<'a>`

```rust
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, alloc::string::String, crate::value::Value>;
```

### `OccupiedEntryImpl<'a>`

```rust
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, alloc::string::String, crate::value::Value>;
```

### `IterImpl<'a>`

```rust
type IterImpl<'a> = btree_map::Iter<'a, alloc::string::String, crate::value::Value>;
```

### `IterMutImpl<'a>`

```rust
type IterMutImpl<'a> = btree_map::IterMut<'a, alloc::string::String, crate::value::Value>;
```

### `IntoIterImpl`

```rust
type IntoIterImpl = btree_map::IntoIter<alloc::string::String, crate::value::Value>;
```

### `KeysImpl<'a>`

```rust
type KeysImpl<'a> = btree_map::Keys<'a, alloc::string::String, crate::value::Value>;
```

### `ValuesImpl<'a>`

```rust
type ValuesImpl<'a> = btree_map::Values<'a, alloc::string::String, crate::value::Value>;
```

### `ValuesMutImpl<'a>`

```rust
type ValuesMutImpl<'a> = btree_map::ValuesMut<'a, alloc::string::String, crate::value::Value>;
```

### `IntoValuesImpl`

```rust
type IntoValuesImpl = btree_map::IntoValues<alloc::string::String, crate::value::Value>;
```

## Macros

### `delegate_iterator!`

