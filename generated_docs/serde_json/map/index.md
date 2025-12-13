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
  - [`delegate_iterator!`](#delegate-iterator)

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
| [`delegate_iterator!`](#delegate-iterator) | macro |  |

## Structs

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:29-31`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L29-L31)*

Represents a JSON key/value type.

#### Implementations

- <span id="map-new"></span>`fn new() -> Self`

  Makes a new empty Map.

- <span id="map-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Makes a new empty Map with the given initial capacity.

- <span id="map-clear"></span>`fn clear(&mut self)`

  Clears the map, removing all values.

- <span id="map-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&Value>` — [`Value`](../value/index.md#value)

  Returns a reference to the value corresponding to the key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

  Returns true if the map contains a value for the specified key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>` — [`Value`](../value/index.md#value)

  Returns a mutable reference to the value corresponding to the key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-get-key-value"></span>`fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../value/index.md#value)

  Returns the key-value pair matching the given key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-insert"></span>`fn insert(&mut self, k: String, v: Value) -> Option<Value>` — [`Value`](../value/index.md#value)

  Inserts a key-value pair into the map.

  

  If the map did not have this key present, `None` is returned.

  

  If the map did have this key present, the value is updated, and the old

  value is returned.

- <span id="map-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<Value>` — [`Value`](../value/index.md#value)

  Removes a key from the map, returning the value at the key if the key

  was previously in the map.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

  

  If serde_json's "preserve_order" is enabled, `.remove(key)` is

  equivalent to `.swap_remove(key)`, replacing this

  entry's position with the last element. If you need to preserve the

  relative order of the keys in the map, use

  `.shift_remove(key)` instead.

- <span id="map-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>` — [`Value`](../value/index.md#value)

  Removes a key from the map, returning the stored key and value if the

  key was previously in the map.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

  

  If serde_json's "preserve_order" is enabled, `.remove_entry(key)` is

  equivalent to `.swap_remove_entry(key)`,

  replacing this entry's position with the last element. If you need to

  preserve the relative order of the keys in the map, use

  `.shift_remove_entry(key)` instead.

- <span id="map-append"></span>`fn append(&mut self, other: &mut Self)`

  Moves all elements from other into self, leaving other empty.

- <span id="map-entry"></span>`fn entry<S>(&mut self, key: S) -> Entry<'_>` — [`Entry`](#entry)

  Gets the given key's corresponding entry in the map for in-place

  manipulation.

- <span id="map-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in the map.

- <span id="map-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if the map contains no elements.

- <span id="map-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](#iter)

  Gets an iterator over the entries of the map.

- <span id="map-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_>` — [`IterMut`](#itermut)

  Gets a mutable iterator over the entries of the map.

- <span id="map-keys"></span>`fn keys(&self) -> Keys<'_>` — [`Keys`](#keys)

  Gets an iterator over the keys of the map.

- <span id="map-values"></span>`fn values(&self) -> Values<'_>` — [`Values`](#values)

  Gets an iterator over the values of the map.

- <span id="map-values-mut"></span>`fn values_mut(&mut self) -> ValuesMut<'_>` — [`ValuesMut`](#valuesmut)

  Gets an iterator over mutable values of the map.

- <span id="map-into-values"></span>`fn into_values(self) -> IntoValues` — [`IntoValues`](#intovalues)

  Gets an iterator over the values of the map.

- <span id="map-retain"></span>`fn retain<F>(&mut self, f: F)`

  Retains only the elements specified by the predicate.

  

  In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)`

  returns `false`.

- <span id="map-sort-keys"></span>`fn sort_keys(&mut self)`

  Sorts this map's entries in-place using `str`'s usual ordering.

  

  If serde_json's "preserve_order" feature is not enabled, this method

  does no work because all JSON maps are always kept in a sorted state.

  

  If serde_json's "preserve_order" feature is enabled, this method

  destroys the original source order or insertion order of this map in

  favor of an alphanumerical order that matches how a BTreeMap with the

  same contents would be ordered. This takes **O(n log n + c)** time where

  _n_ is the length of the map and _c_ is the capacity.

  

  Other maps nested within the values of this map are not sorted. If you

  need the entire data structure to be sorted at all levels, you must also

  call

  <code>map.[values_mut]\().for_each([Value::sort_all_objects])</code>.

#### Trait Implementations

##### `impl Any for Map<K, V>`

- <span id="map-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Map<K, V>`

- <span id="map-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Map<K, V>`

- <span id="map-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Map<alloc::string::String, crate::value::Value>`

- <span id="map-clone"></span>`fn clone(&self) -> Self`

- <span id="map-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl CloneToUninit for Map<K, V>`

- <span id="map-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Map<alloc::string::String, crate::value::Value>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Map<alloc::string::String, crate::value::Value>`

- <span id="map-default"></span>`fn default() -> Self`

##### `impl Deserialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Map<K, V>`

##### `impl Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-deserializer-type-error"></span>`type Error = Error`

- <span id="cratemapmap-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Eq for Map<alloc::string::String, crate::value::Value>`

##### `impl Extend for Map<alloc::string::String, crate::value::Value>`

- <span id="map-extend"></span>`fn extend<T>(&mut self, iter: T)`

##### `impl<T> From for Map<K, V>`

- <span id="map-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for Map<alloc::string::String, crate::value::Value>`

- <span id="map-fromiterator-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-fromstr-type-err"></span>`type Err = Error`

- <span id="cratemapmap-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../error/index.md#error)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- <span id="map-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-type-output"></span>`type Output = Value`

- <span id="map-index"></span>`fn index(&self, index: &Q) -> &Value` — [`Value`](../value/index.md#value)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- <span id="map-indexmut-index-mut"></span>`fn index_mut(&mut self, index: &Q) -> &mut Value` — [`Value`](../value/index.md#value)

##### `impl<U> Into for Map<K, V>`

- <span id="map-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-intodeserializer-type-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl IntoIterator for &'a Map<alloc::string::String, crate::value::Value>`

- <span id="a-map-intoiterator-type-item"></span>`type Item = (&'a String, &'a Value)`

- <span id="a-map-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-map-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for Map<alloc::string::String, crate::value::Value>`

- <span id="map-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Serialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl ToOwned for Map<K, V>`

- <span id="map-toowned-type-owned"></span>`type Owned = T`

- <span id="map-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="map-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Map<K, V>`

- <span id="map-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="map-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Map<K, V>`

- <span id="map-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="map-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VacantEntry<'a>`

```rust
struct VacantEntry<'a> {
    vacant: btree_map::VacantEntry<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:631-633`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L631-L633)*

A vacant Entry. It is part of the [`Entry`](#entry) enum.

#### Implementations

- <span id="vacantentry-key"></span>`fn key(&self) -> &String`

  Gets a reference to the key that would be used when inserting a value

  through the VacantEntry.

  

  # Examples

  

  ```rust

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  

  match map.entry("serde") {

      Entry::Vacant(vacant) => {

          assert_eq!(vacant.key(), &"serde");

      }

      Entry::Occupied(_) => unimplemented!(),

  }

  ```

- <span id="vacantentry-insert"></span>`fn insert(self, value: Value) -> &'a mut Value` — [`Value`](../value/index.md#value)

  Sets the value of the entry with the VacantEntry's key, and returns a

  mutable reference to it.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  

  match map.entry("serde") {

      Entry::Vacant(vacant) => {

          vacant.insert(json!("hoho"));

      }

      Entry::Occupied(_) => unimplemented!(),

  }

  ```

#### Trait Implementations

##### `impl Any for VacantEntry<'a>`

- <span id="vacantentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VacantEntry<'a>`

- <span id="vacantentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VacantEntry<'a>`

- <span id="vacantentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for VacantEntry<'a>`

- <span id="vacantentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VacantEntry<'a>`

- <span id="vacantentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for VacantEntry<'a>`

- <span id="vacantentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vacantentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VacantEntry<'a>`

- <span id="vacantentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vacantentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OccupiedEntry<'a>`

```rust
struct OccupiedEntry<'a> {
    occupied: btree_map::OccupiedEntry<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:636-638`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L636-L638)*

An occupied Entry. It is part of the [`Entry`](#entry) enum.

#### Implementations

- <span id="occupiedentry-key"></span>`fn key(&self) -> &String`

  Gets a reference to the key in the entry.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  map.insert("serde".to_owned(), json!(12));

  

  match map.entry("serde") {

      Entry::Occupied(occupied) => {

          assert_eq!(occupied.key(), &"serde");

      }

      Entry::Vacant(_) => unimplemented!(),

  }

  ```

- <span id="occupiedentry-get"></span>`fn get(&self) -> &Value` — [`Value`](../value/index.md#value)

  Gets a reference to the value in the entry.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  map.insert("serde".to_owned(), json!(12));

  

  match map.entry("serde") {

      Entry::Occupied(occupied) => {

          assert_eq!(occupied.get(), 12);

      }

      Entry::Vacant(_) => unimplemented!(),

  }

  ```

- <span id="occupiedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut Value` — [`Value`](../value/index.md#value)

  Gets a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  map.insert("serde".to_owned(), json!([1, 2, 3]));

  

  match map.entry("serde") {

      Entry::Occupied(mut occupied) => {

          occupied.get_mut().as_array_mut().unwrap().push(json!(4));

      }

      Entry::Vacant(_) => unimplemented!(),

  }

  

  assert_eq!(map["serde"].as_array().unwrap().len(), 4);

  ```

- <span id="occupiedentry-into-mut"></span>`fn into_mut(self) -> &'a mut Value` — [`Value`](../value/index.md#value)

  Converts the entry into a mutable reference to its value.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  map.insert("serde".to_owned(), json!([1, 2, 3]));

  

  match map.entry("serde") {

      Entry::Occupied(mut occupied) => {

          occupied.into_mut().as_array_mut().unwrap().push(json!(4));

      }

      Entry::Vacant(_) => unimplemented!(),

  }

  

  assert_eq!(map["serde"].as_array().unwrap().len(), 4);

  ```

- <span id="occupiedentry-insert"></span>`fn insert(&mut self, value: Value) -> Value` — [`Value`](../value/index.md#value)

  Sets the value of the entry with the `OccupiedEntry`'s key, and returns

  the entry's old value.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  map.insert("serde".to_owned(), json!(12));

  

  match map.entry("serde") {

      Entry::Occupied(mut occupied) => {

          assert_eq!(occupied.insert(json!(13)), 12);

          assert_eq!(occupied.get(), 13);

      }

      Entry::Vacant(_) => unimplemented!(),

  }

  ```

- <span id="occupiedentry-remove"></span>`fn remove(self) -> Value` — [`Value`](../value/index.md#value)

  Takes the value of the entry out of the map, and returns it.

  

  If serde_json's "preserve_order" is enabled, `.remove()` is

  equivalent to `.swap_remove()`, replacing this

  entry's position with the last element. If you need to preserve the

  relative order of the keys in the map, use

  `.shift_remove()` instead.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  map.insert("serde".to_owned(), json!(12));

  

  match map.entry("serde") {

      Entry::Occupied(occupied) => {

          assert_eq!(occupied.remove(), 12);

      }

      Entry::Vacant(_) => unimplemented!(),

  }

  ```

- <span id="occupiedentry-remove-entry"></span>`fn remove_entry(self) -> (String, Value)` — [`Value`](../value/index.md#value)

  Removes the entry from the map, returning the stored key and value.

  

  If serde_json's "preserve_order" is enabled, `.remove_entry()` is

  equivalent to `.swap_remove_entry()`,

  replacing this entry's position with the last element. If you need to

  preserve the relative order of the keys in the map, use

  `.shift_remove_entry()` instead.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  use serde_json::map::Entry;

  

  let mut map = serde_json::Map::new();

  map.insert("serde".to_owned(), json!(12));

  

  match map.entry("serde") {

      Entry::Occupied(occupied) => {

          let (key, value) = occupied.remove_entry();

          assert_eq!(key, "serde");

          assert_eq!(value, 12);

      }

      Entry::Vacant(_) => unimplemented!(),

  }

  ```

#### Trait Implementations

##### `impl Any for OccupiedEntry<'a>`

- <span id="occupiedentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OccupiedEntry<'a>`

- <span id="occupiedentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OccupiedEntry<'a>`

- <span id="occupiedentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for OccupiedEntry<'a>`

- <span id="occupiedentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OccupiedEntry<'a>`

- <span id="occupiedentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OccupiedEntry<'a>`

- <span id="occupiedentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="occupiedentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OccupiedEntry<'a>`

- <span id="occupiedentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="occupiedentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a>`

```rust
struct Iter<'a> {
    iter: btree_map::Iter<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1060-1062`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1060-L1062)*

An iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Any for Iter<'a>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Iter<'a>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl CloneToUninit for Iter<'a>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Iter<'a>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Iter<'a>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Iter<'a>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Iter<'a>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Iter<'a>`

##### `impl<U> Into for Iter<'a>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'a>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter<'a>`

- <span id="iter-iterator-type-item"></span>`type Item = (&'a String, &'a Value)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Iter<'a>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Iter<'a>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<'a>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'a>`

```rust
struct IterMut<'a> {
    iter: btree_map::IterMut<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1086-1088`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1086-L1088)*

A mutable iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Any for IterMut<'a>`

- <span id="itermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMut<'a>`

- <span id="itermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMut<'a>`

- <span id="itermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for IterMut<'a>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IterMut<'a>`

- <span id="itermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IterMut<'a>`

- <span id="itermut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IterMut<'a>`

- <span id="itermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for IterMut<'a>`

##### `impl<U> Into for IterMut<'a>`

- <span id="itermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterMut<'a>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IterMut<'a>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a String, &'a mut Value)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for IterMut<'a>`

- <span id="itermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterMut<'a>`

- <span id="itermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter`

```rust
struct IntoIter {
    iter: btree_map::IntoIter<alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1112-1114`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1112-L1114)*

An owning iterator over a serde_json::Map's entries.

#### Trait Implementations

##### `impl Any for IntoIter`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for IntoIter`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IntoIter`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IntoIter`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntoIter`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for IntoIter`

##### `impl<U> Into for IntoIter`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-iterator-type-item"></span>`type Item = (String, Value)`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for IntoIter`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoIter`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Keys<'a>`

```rust
struct Keys<'a> {
    iter: btree_map::Keys<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1127-1129`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1127-L1129)*

An iterator over a serde_json::Map's keys.

#### Trait Implementations

##### `impl Any for Keys<'a>`

- <span id="keys-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Keys<'a>`

- <span id="keys-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Keys<'a>`

- <span id="keys-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Keys<'a>`

- <span id="keys-clone"></span>`fn clone(&self) -> Keys<'a>` — [`Keys`](#keys)

##### `impl CloneToUninit for Keys<'a>`

- <span id="keys-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Keys<'a>`

- <span id="keys-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Keys<'a>`

- <span id="keys-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Keys<'a>`

- <span id="keys-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Keys<'a>`

- <span id="keys-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Keys<'a>`

##### `impl<U> Into for Keys<'a>`

- <span id="keys-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Keys<'a>`

- <span id="keys-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="keys-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="keys-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Keys<'a>`

- <span id="keys-iterator-type-item"></span>`type Item = &'a String`

- <span id="keys-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="keys-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Keys<'a>`

- <span id="keys-toowned-type-owned"></span>`type Owned = T`

- <span id="keys-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="keys-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Keys<'a>`

- <span id="keys-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="keys-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Keys<'a>`

- <span id="keys-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="keys-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Values<'a>`

```rust
struct Values<'a> {
    iter: btree_map::Values<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1142-1144`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1142-L1144)*

An iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl Any for Values<'a>`

- <span id="values-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Values<'a>`

- <span id="values-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Values<'a>`

- <span id="values-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Values<'a>`

- <span id="values-clone"></span>`fn clone(&self) -> Values<'a>` — [`Values`](#values)

##### `impl CloneToUninit for Values<'a>`

- <span id="values-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Values<'a>`

- <span id="values-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Values<'a>`

- <span id="values-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for Values<'a>`

- <span id="values-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Values<'a>`

- <span id="values-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Values<'a>`

##### `impl<U> Into for Values<'a>`

- <span id="values-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Values<'a>`

- <span id="values-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="values-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Values<'a>`

- <span id="values-iterator-type-item"></span>`type Item = &'a Value`

- <span id="values-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="values-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Values<'a>`

- <span id="values-toowned-type-owned"></span>`type Owned = T`

- <span id="values-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="values-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Values<'a>`

- <span id="values-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="values-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Values<'a>`

- <span id="values-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="values-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ValuesMut<'a>`

```rust
struct ValuesMut<'a> {
    iter: btree_map::ValuesMut<'a, alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1157-1159`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1157-L1159)*

A mutable iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl Any for ValuesMut<'a>`

- <span id="valuesmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValuesMut<'a>`

- <span id="valuesmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValuesMut<'a>`

- <span id="valuesmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ValuesMut<'a>`

- <span id="valuesmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for ValuesMut<'a>`

- <span id="valuesmut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for ValuesMut<'a>`

- <span id="valuesmut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for ValuesMut<'a>`

- <span id="valuesmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for ValuesMut<'a>`

##### `impl<U> Into for ValuesMut<'a>`

- <span id="valuesmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ValuesMut<'a>`

- <span id="valuesmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ValuesMut<'a>`

- <span id="valuesmut-iterator-type-item"></span>`type Item = &'a mut Value`

- <span id="valuesmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="valuesmut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for ValuesMut<'a>`

- <span id="valuesmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuesmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValuesMut<'a>`

- <span id="valuesmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuesmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoValues`

```rust
struct IntoValues {
    iter: btree_map::IntoValues<alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:1172-1174`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1172-L1174)*

An owning iterator over a serde_json::Map's values.

#### Trait Implementations

##### `impl Any for IntoValues`

- <span id="intovalues-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoValues`

- <span id="intovalues-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoValues`

- <span id="intovalues-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for IntoValues`

- <span id="intovalues-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IntoValues`

- <span id="intovalues-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for IntoValues`

- <span id="intovalues-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntoValues`

- <span id="intovalues-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for IntoValues`

##### `impl<U> Into for IntoValues`

- <span id="intovalues-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoValues`

- <span id="intovalues-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intovalues-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intovalues-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoValues`

- <span id="intovalues-iterator-type-item"></span>`type Item = Value`

- <span id="intovalues-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intovalues-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for IntoValues`

- <span id="intovalues-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intovalues-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoValues`

- <span id="intovalues-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intovalues-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Entry<'a>`

```rust
enum Entry<'a> {
    Vacant(VacantEntry<'a>),
    Occupied(OccupiedEntry<'a>),
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:623-628`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L623-L628)*

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the `entry` method on [`Map`](#map).


#### Variants

- **`Vacant`**

  A vacant Entry.

- **`Occupied`**

  An occupied Entry.

#### Implementations

- <span id="entry-key"></span>`fn key(&self) -> &String`

  Returns a reference to this entry's key.

  

  # Examples

  

  ```rust

  let mut map = serde_json::Map::new();

  assert_eq!(map.entry("serde").key(), &"serde");

  ```

- <span id="entry-or-insert"></span>`fn or_insert(self, default: Value) -> &'a mut Value` — [`Value`](../value/index.md#value)

  Ensures a value is in the entry by inserting the default if empty, and

  returns a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  let mut map = serde_json::Map::new();

  map.entry("serde").or_insert(json!(12));

  

  assert_eq!(map["serde"], 12);

  ```

- <span id="entry-or-insert-with"></span>`fn or_insert_with<F>(self, default: F) -> &'a mut Value` — [`Value`](../value/index.md#value)

  Ensures a value is in the entry by inserting the result of the default

  function if empty, and returns a mutable reference to the value in the

  entry.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  let mut map = serde_json::Map::new();

  map.entry("serde").or_insert_with(|| json!("hoho"));

  

  assert_eq!(map["serde"], "hoho".to_owned());

  ```

- <span id="entry-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

  Provides in-place mutable access to an occupied entry before any

  potential inserts into the map.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  let mut map = serde_json::Map::new();

  map.entry("serde")

      .and_modify(|e| *e = json!("rust"))

      .or_insert(json!("cpp"));

  

  assert_eq!(map["serde"], "cpp");

  

  map.entry("serde")

      .and_modify(|e| *e = json!("rust"))

      .or_insert(json!("cpp"));

  

  assert_eq!(map["serde"], "rust");

  ```

#### Trait Implementations

##### `impl Any for Entry<'a>`

- <span id="entry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entry<'a>`

- <span id="entry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entry<'a>`

- <span id="entry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Entry<'a>`

- <span id="entry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Entry<'a>`

- <span id="entry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Entry<'a>`

- <span id="entry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Entry<'a>`

- <span id="entry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `MapImpl<K, V>`

```rust
type MapImpl<K, V> = alloc::collections::BTreeMap<K, V>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:34`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L34)*

### `VacantEntryImpl<'a>`

```rust
type VacantEntryImpl<'a> = btree_map::VacantEntry<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:641`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L641)*

### `OccupiedEntryImpl<'a>`

```rust
type OccupiedEntryImpl<'a> = btree_map::OccupiedEntry<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:646`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L646)*

### `IterImpl<'a>`

```rust
type IterImpl<'a> = btree_map::Iter<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1065`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1065)*

### `IterMutImpl<'a>`

```rust
type IterMutImpl<'a> = btree_map::IterMut<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1091`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1091)*

### `IntoIterImpl`

```rust
type IntoIterImpl = btree_map::IntoIter<alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1117`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1117)*

### `KeysImpl<'a>`

```rust
type KeysImpl<'a> = btree_map::Keys<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1132`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1132)*

### `ValuesImpl<'a>`

```rust
type ValuesImpl<'a> = btree_map::Values<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1147`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1147)*

### `ValuesMutImpl<'a>`

```rust
type ValuesMutImpl<'a> = btree_map::ValuesMut<'a, alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1162`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1162)*

### `IntoValuesImpl`

```rust
type IntoValuesImpl = btree_map::IntoValues<alloc::string::String, crate::value::Value>;
```

*Defined in [`serde_json-1.0.145/src/map.rs:1177`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L1177)*

## Macros

### `delegate_iterator!`

*Defined in [`serde_json-1.0.145/src/map.rs:569-599`](../../../.source_1765633015/serde_json-1.0.145/src/map.rs#L569-L599)*

