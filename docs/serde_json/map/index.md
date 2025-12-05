*[serde_json](../index.md) / [map](index.md)*

---

# Module `map`

A map of String to serde_json::Value.

By default the map is backed by a [`BTreeMap`](#btreemap). Enable the `preserve_order`
feature of serde_json to use [`IndexMap`](#indexmap) instead.



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

- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>` — [`Value`](../../value/index.md)

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`

- `fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>` — [`Value`](../../value/index.md)

- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../../value/index.md)

- `fn insert(self: &mut Self, k: String, v: Value) -> Option<Value>` — [`Value`](../../value/index.md)

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>` — [`Value`](../../value/index.md)

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>` — [`Value`](../../value/index.md)

- `fn append(self: &mut Self, other: &mut Self)`

- `fn entry<S>(self: &mut Self, key: S) -> Entry<'_>` — [`Entry`](../../map/index.md)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> Iter<'_>` — [`Iter`](../../map/index.md)

- `fn iter_mut(self: &mut Self) -> IterMut<'_>` — [`IterMut`](../../map/index.md)

- `fn keys(self: &Self) -> Keys<'_>` — [`Keys`](../../map/index.md)

- `fn values(self: &Self) -> Values<'_>` — [`Values`](../../map/index.md)

- `fn values_mut(self: &mut Self) -> ValuesMut<'_>` — [`ValuesMut`](../../map/index.md)

- `fn into_values(self: Self) -> IntoValues` — [`IntoValues`](../../map/index.md)

- `fn retain<F>(self: &mut Self, f: F)`

- `fn sort_keys(self: &mut Self)`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

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

##### `impl Eq`

##### `impl Extend`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl FromIterator`

- `fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../../error/index.md)

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl Index<Q>`

- `type Output = Value`

- `fn index(self: &Self, index: &Q) -> &Value` — [`Value`](../../value/index.md)

##### `impl IndexMut<Q>`

- `fn index_mut(self: &mut Self, index: &Q) -> &mut Value` — [`Value`](../../value/index.md)

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Map<String, Value>`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl IntoIterator`

- `type Item = (String, Value)`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Serialize`

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

- `fn insert(self: Self, value: Value) -> &'a mut Value` — [`Value`](../../value/index.md)

### `OccupiedEntry<'a>`

```rust
struct OccupiedEntry<'a> {
    occupied: btree_map::OccupiedEntry<'a, alloc::string::String, crate::value::Value>,
}
```

An occupied Entry. It is part of the [`Entry`](#entry) enum.

#### Implementations

- `fn key(self: &Self) -> &String`

- `fn get(self: &Self) -> &Value` — [`Value`](../../value/index.md)

- `fn get_mut(self: &mut Self) -> &mut Value` — [`Value`](../../value/index.md)

- `fn into_mut(self: Self) -> &'a mut Value` — [`Value`](../../value/index.md)

- `fn insert(self: &mut Self, value: Value) -> Value` — [`Value`](../../value/index.md)

- `fn remove(self: Self) -> Value` — [`Value`](../../value/index.md)

- `fn remove_entry(self: Self) -> (String, Value)` — [`Value`](../../value/index.md)

### `Iter<'a>`

```rust
struct Iter<'a> {
    iter: btree_map::Iter<'a, alloc::string::String, crate::value::Value>,
}
```

An iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Iter<'a>` — [`Iter`](../../map/index.md)

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a>`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator<'a>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a>`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator<'a>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator`

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Keys<'a>` — [`Keys`](../../map/index.md)

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a>`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator<'a>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Values<'a>` — [`Values`](../../map/index.md)

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a>`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator<'a>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a>`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator<'a>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator`

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
This enum is constructed from the [`entry`](#entry) method on [`Map`](../index.md).


#### Variants

- **`Vacant`**

  A vacant Entry.

- **`Occupied`**

  An occupied Entry.

#### Implementations

- `fn key(self: &Self) -> &String`

- `fn or_insert(self: Self, default: Value) -> &'a mut Value` — [`Value`](../../value/index.md)

- `fn or_insert_with<F>(self: Self, default: F) -> &'a mut Value` — [`Value`](../../value/index.md)

- `fn and_modify<F>(self: Self, f: F) -> Self`

