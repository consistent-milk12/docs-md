*[serde_json](../index.md) / [map](index.md)*

---

# Module `map`

A map of String to serde_json::Value.

By default the map is backed by a `BTreeMap`. Enable the `preserve_order`
feature of serde_json to use `IndexMap` instead.



## Contents

- [Structs](#structs)
  - [`Map`](#map)
  - [`VacantEntry`](#vacantentry)
  - [`OccupiedEntry`](#occupiedentry)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`IntoIter`](#intoiter)
  - [`Keys`](#keys)
  - [`Values`](#values)
  - [`ValuesMut`](#valuesmut)
  - [`IntoValues`](#intovalues)
- [Enums](#enums)
  - [`Entry`](#entry)
- [Type Aliases](#type-aliases)
  - [`MapImpl`](#mapimpl)
  - [`VacantEntryImpl`](#vacantentryimpl)
  - [`OccupiedEntryImpl`](#occupiedentryimpl)
  - [`IterImpl`](#iterimpl)
  - [`IterMutImpl`](#itermutimpl)
  - [`IntoIterImpl`](#intoiterimpl)
  - [`KeysImpl`](#keysimpl)
  - [`ValuesImpl`](#valuesimpl)
  - [`ValuesMutImpl`](#valuesmutimpl)
  - [`IntoValuesImpl`](#intovaluesimpl)
- [Macros](#macros)
  - [`delegate_iterator!`](#delegate_iterator)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Map`](#map) | struct | Represents a JSON key/value type. |
| [`VacantEntry`](#vacantentry) | struct | A vacant Entry. |
| [`OccupiedEntry`](#occupiedentry) | struct | An occupied Entry. |
| [`Iter`](#iter) | struct | An iterator over a serde_json::Map's entries. |
| [`IterMut`](#itermut) | struct | A mutable iterator over a serde_json::Map's entries. |
| [`IntoIter`](#intoiter) | struct | An owning iterator over a serde_json::Map's entries. |
| [`Keys`](#keys) | struct | An iterator over a serde_json::Map's keys. |
| [`Values`](#values) | struct | An iterator over a serde_json::Map's values. |
| [`ValuesMut`](#valuesmut) | struct | A mutable iterator over a serde_json::Map's values. |
| [`IntoValues`](#intovalues) | struct | An owning iterator over a serde_json::Map's values. |
| [`Entry`](#entry) | enum | A view into a single entry in a map, which may either be vacant or occupied. |
| [`MapImpl`](#mapimpl) | type |  |
| [`VacantEntryImpl`](#vacantentryimpl) | type |  |
| [`OccupiedEntryImpl`](#occupiedentryimpl) | type |  |
| [`IterImpl`](#iterimpl) | type |  |
| [`IterMutImpl`](#itermutimpl) | type |  |
| [`IntoIterImpl`](#intoiterimpl) | type |  |
| [`KeysImpl`](#keysimpl) | type |  |
| [`ValuesImpl`](#valuesimpl) | type |  |
| [`ValuesMutImpl`](#valuesmutimpl) | type |  |
| [`IntoValuesImpl`](#intovaluesimpl) | type |  |
| [`delegate_iterator!`](#delegate_iterator) | macro |  |

## Structs

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:29-31`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L29-L31)*

Represents a JSON key/value type.

#### Implementations

- <span id="map-new"></span>`fn new() -> Self`

- <span id="map-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="map-clear"></span>`fn clear(&mut self)`

- <span id="map-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&Value>` — [`Value`](../value/index.md)

- <span id="map-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

- <span id="map-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>` — [`Value`](../value/index.md)

- <span id="map-get-key-value"></span>`fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../value/index.md)

- <span id="map-insert"></span>`fn insert(&mut self, k: String, v: Value) -> Option<Value>` — [`Value`](../value/index.md)

- <span id="map-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<Value>` — [`Value`](../value/index.md)

- <span id="map-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>` — [`Value`](../value/index.md)

- <span id="map-append"></span>`fn append(&mut self, other: &mut Self)`

- <span id="map-entry"></span>`fn entry<S>(&mut self, key: S) -> Entry<'_>` — [`Entry`](#entry)

- <span id="map-len"></span>`fn len(&self) -> usize`

- <span id="map-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="map-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](#iter)

- <span id="map-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_>` — [`IterMut`](#itermut)

- <span id="map-keys"></span>`fn keys(&self) -> Keys<'_>` — [`Keys`](#keys)

- <span id="map-values"></span>`fn values(&self) -> Values<'_>` — [`Values`](#values)

- <span id="map-values-mut"></span>`fn values_mut(&mut self) -> ValuesMut<'_>` — [`ValuesMut`](#valuesmut)

- <span id="map-into-values"></span>`fn into_values(self) -> IntoValues` — [`IntoValues`](#intovalues)

- <span id="map-retain"></span>`fn retain<F>(&mut self, f: F)`

- <span id="map-sort-keys"></span>`fn sort_keys(&mut self)`

#### Trait Implementations

##### `impl Clone for Map<alloc::string::String, crate::value::Value>`

- <span id="map-clone"></span>`fn clone(&self) -> Self`

- <span id="map-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl Debug for Map<alloc::string::String, crate::value::Value>`

- <span id="map-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Map<alloc::string::String, crate::value::Value>`

- <span id="map-default"></span>`fn default() -> Self`

##### `impl Deserialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl<T> DeserializeOwned for Map<K, V>`

##### `impl Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-type-error"></span>`type Error = Error`

- <span id="cratemapmap-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Eq for Map<alloc::string::String, crate::value::Value>`

##### `impl Extend for Map<alloc::string::String, crate::value::Value>`

- <span id="map-extend"></span>`fn extend<T>(&mut self, iter: T)`

##### `impl FromIterator for Map<alloc::string::String, crate::value::Value>`

- <span id="map-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-type-err"></span>`type Err = Error`

- <span id="cratemapmap-from-str"></span>`fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../error/index.md)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- <span id="map-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl Index for Map<alloc::string::String, crate::value::Value>`

- <span id="map-type-output"></span>`type Output = Value`

- <span id="map-index"></span>`fn index(&self, index: &Q) -> &Value` — [`Value`](../value/index.md)

##### `impl IndexMut for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-mut"></span>`fn index_mut(&mut self, index: &Q) -> &mut Value` — [`Value`](../value/index.md)

##### `impl IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-type-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl IntoIterator for &'a Map<alloc::string::String, crate::value::Value>`

- <span id="a-map-type-item"></span>`type Item = (&'a String, &'a Value)`

- <span id="a-map-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-map-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for Map<alloc::string::String, crate::value::Value>`

- <span id="map-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Serialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

### `VacantEntry<'a>`

```rust
struct VacantEntry<'a> {
    vacant: btree_map::VacantEntry<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:631-633`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L631-L633)*

A vacant Entry. It is part of the [`Entry`](#entry) enum.

#### Implementations

- <span id="vacantentry-key"></span>`fn key(&self) -> &String`

- <span id="vacantentry-insert"></span>`fn insert(self, value: Value) -> &'a mut Value` — [`Value`](../value/index.md)

### `OccupiedEntry<'a>`

```rust
struct OccupiedEntry<'a> {
    occupied: btree_map::OccupiedEntry<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:636-638`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L636-L638)*

An occupied Entry. It is part of the [`Entry`](#entry) enum.

#### Implementations

- <span id="occupiedentry-key"></span>`fn key(&self) -> &String`

- <span id="occupiedentry-get"></span>`fn get(&self) -> &Value` — [`Value`](../value/index.md)

- <span id="occupiedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut Value` — [`Value`](../value/index.md)

- <span id="occupiedentry-into-mut"></span>`fn into_mut(self) -> &'a mut Value` — [`Value`](../value/index.md)

- <span id="occupiedentry-insert"></span>`fn insert(&mut self, value: Value) -> Value` — [`Value`](../value/index.md)

- <span id="occupiedentry-remove"></span>`fn remove(self) -> Value` — [`Value`](../value/index.md)

- <span id="occupiedentry-remove-entry"></span>`fn remove_entry(self) -> (String, Value)` — [`Value`](../value/index.md)

### `Iter<'a>`

```rust
struct Iter<'a> {
    iter: btree_map::Iter<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1060-1062`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1060-L1062)*

An iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Clone for Iter<'a>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl Debug for Iter<'a>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Iter<'a>`

- <span id="iter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Iter<'a>`

- <span id="iter-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for Iter<'a>`

##### `impl IntoIterator for Iter<'a>`

- <span id="iter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter<'a>`

- <span id="iter-type-item"></span>`type Item = (&'a String, &'a Value)`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterMut<'a>`

```rust
struct IterMut<'a> {
    iter: btree_map::IterMut<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1086-1088`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1086-L1088)*

A mutable iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Debug for IterMut<'a>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IterMut<'a>`

- <span id="itermut-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IterMut<'a>`

- <span id="itermut-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for IterMut<'a>`

##### `impl IntoIterator for IterMut<'a>`

- <span id="itermut-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IterMut<'a>`

- <span id="itermut-type-item"></span>`type Item = (&'a String, &'a mut Value)`

- <span id="itermut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IntoIter`

```rust
struct IntoIter {
    iter: btree_map::IntoIter<alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1112-1114`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1112-L1114)*

An owning iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Debug for IntoIter`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IntoIter`

- <span id="intoiter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IntoIter`

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for IntoIter`

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-type-item"></span>`type Item = (String, Value)`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Keys<'a>`

```rust
struct Keys<'a> {
    iter: btree_map::Keys<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1127-1129`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1127-L1129)*

An iterator over a serde_json::Map's keys.

#### Trait Implementations

##### `impl Clone for Keys<'a>`

- <span id="keys-clone"></span>`fn clone(&self) -> Keys<'a>` — [`Keys`](#keys)

##### `impl Debug for Keys<'a>`

- <span id="keys-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Keys<'a>`

- <span id="keys-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Keys<'a>`

- <span id="keys-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for Keys<'a>`

##### `impl IntoIterator for Keys<'a>`

- <span id="keys-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="keys-type-intoiter"></span>`type IntoIter = I`

- <span id="keys-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Keys<'a>`

- <span id="keys-type-item"></span>`type Item = &'a String`

- <span id="keys-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="keys-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Values<'a>`

```rust
struct Values<'a> {
    iter: btree_map::Values<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1142-1144`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1142-L1144)*

An iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl Clone for Values<'a>`

- <span id="values-clone"></span>`fn clone(&self) -> Values<'a>` — [`Values`](#values)

##### `impl Debug for Values<'a>`

- <span id="values-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Values<'a>`

- <span id="values-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Values<'a>`

- <span id="values-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for Values<'a>`

##### `impl IntoIterator for Values<'a>`

- <span id="values-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-type-intoiter"></span>`type IntoIter = I`

- <span id="values-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Values<'a>`

- <span id="values-type-item"></span>`type Item = &'a Value`

- <span id="values-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="values-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `ValuesMut<'a>`

```rust
struct ValuesMut<'a> {
    iter: btree_map::ValuesMut<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1157-1159`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1157-L1159)*

A mutable iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl Debug for ValuesMut<'a>`

- <span id="valuesmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for ValuesMut<'a>`

- <span id="valuesmut-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for ValuesMut<'a>`

- <span id="valuesmut-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for ValuesMut<'a>`

##### `impl IntoIterator for ValuesMut<'a>`

- <span id="valuesmut-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesmut-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesmut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ValuesMut<'a>`

- <span id="valuesmut-type-item"></span>`type Item = &'a mut Value`

- <span id="valuesmut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="valuesmut-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IntoValues`

```rust
struct IntoValues {
    iter: btree_map::IntoValues<alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1172-1174`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1172-L1174)*

An owning iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl Debug for IntoValues`

- <span id="intovalues-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IntoValues`

- <span id="intovalues-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IntoValues`

- <span id="intovalues-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for IntoValues`

##### `impl IntoIterator for IntoValues`

- <span id="intovalues-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intovalues-type-intoiter"></span>`type IntoIter = I`

- <span id="intovalues-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoValues`

- <span id="intovalues-type-item"></span>`type Item = Value`

- <span id="intovalues-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intovalues-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `Entry<'a>`

```rust
enum Entry<'a> {
    Vacant(VacantEntry<'a>),
    Occupied(OccupiedEntry<'a>),
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:623-628`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L623-L628)*

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the `entry` method on [`Map`](#map).


#### Variants

- **`Vacant`**

  A vacant Entry.

- **`Occupied`**

  An occupied Entry.

#### Implementations

- <span id="entry-key"></span>`fn key(&self) -> &String`

- <span id="entry-or-insert"></span>`fn or_insert(self, default: Value) -> &'a mut Value` — [`Value`](../value/index.md)

- <span id="entry-or-insert-with"></span>`fn or_insert_with<F>(self, default: F) -> &'a mut Value` — [`Value`](../value/index.md)

- <span id="entry-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

## Type Aliases

### `MapImpl<K, V>`

```rust
type MapImpl<K, V> = alloc::collections::BTreeMap<K, V>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:34`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L34)*

### `VacantEntryImpl<'a>`

```rust
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:641`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L641)*

### `OccupiedEntryImpl<'a>`

```rust
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:646`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L646)*

### `IterImpl<'a>`

```rust
type IterImpl<'a> = btree_map::Iter<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1065`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1065)*

### `IterMutImpl<'a>`

```rust
type IterMutImpl<'a> = btree_map::IterMut<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1091`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1091)*

### `IntoIterImpl`

```rust
type IntoIterImpl = btree_map::IntoIter<alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1117`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1117)*

### `KeysImpl<'a>`

```rust
type KeysImpl<'a> = btree_map::Keys<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1132`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1132)*

### `ValuesImpl<'a>`

```rust
type ValuesImpl<'a> = btree_map::Values<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1147`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1147)*

### `ValuesMutImpl<'a>`

```rust
type ValuesMutImpl<'a> = btree_map::ValuesMut<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1162`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1162)*

### `IntoValuesImpl`

```rust
type IntoValuesImpl = btree_map::IntoValues<alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1177`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L1177)*

## Macros

### `delegate_iterator!`

*Defined in [`serde_json-1.0.145/src/map.rs:569-599`](../../../.source_1765210505/serde_json-1.0.145/src/map.rs#L569-L599)*

