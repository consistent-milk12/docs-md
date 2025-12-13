*[hashbrown](../index.md) / [raw_entry](index.md)*

---

# Module `raw_entry`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RawEntryBuilderMut`](#rawentrybuildermut) | struct | A builder for computing where in a [`HashMap`] a key-value pair would be stored. |
| [`RawOccupiedEntryMut`](#rawoccupiedentrymut) | struct | A view into an occupied entry in a `HashMap`. |
| [`RawVacantEntryMut`](#rawvacantentrymut) | struct | A view into a vacant entry in a `HashMap`. |
| [`RawEntryBuilder`](#rawentrybuilder) | struct | A builder for computing where in a [`HashMap`] a key-value pair would be stored. |
| [`RawEntryMut`](#rawentrymut) | enum | A view into a single entry in a map, which may either be vacant or occupied. |

## Structs

### `RawEntryBuilderMut<'a, K, V, S, A: Allocator>`

```rust
struct RawEntryBuilderMut<'a, K, V, S, A: Allocator> {
    map: &'a mut crate::HashMap<K, V, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:216-218`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L216-L218)*

A builder for computing where in a [`HashMap`](../hash_map/index.md) a key-value pair would be stored.

See the `HashMap::raw_entry_mut` docs for usage examples.

# Examples

```rust
use hashbrown::hash_map::{RawEntryBuilderMut, RawEntryMut::Vacant, RawEntryMut::Occupied};
use hashbrown::HashMap;
use core::hash::{BuildHasher, Hash};

let mut map = HashMap::new();
map.extend([(1, 11), (2, 12), (3, 13), (4, 14), (5, 15), (6, 16)]);
assert_eq!(map.len(), 6);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

let builder: RawEntryBuilderMut<_, _, _> = map.raw_entry_mut();

// Existing key
match builder.from_key(&6) {
    Vacant(_) => unreachable!(),
    Occupied(view) => assert_eq!(view.get(), &16),
}

for key in 0..12 {
    let hash = compute_hash(map.hasher(), &key);
    let value = map.get(&key).cloned();
    let key_value = value.as_ref().map(|v| (&key, v));

    println!("Key: {} and value: {:?}", key, value);

    match map.raw_entry_mut().from_key(&key) {
        Occupied(mut o) => assert_eq!(Some(o.get_key_value()), key_value),
        Vacant(_) => assert_eq!(value, None),
    }
    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        Occupied(mut o) => assert_eq!(Some(o.get_key_value()), key_value),
        Vacant(_) => assert_eq!(value, None),
    }
    match map.raw_entry_mut().from_hash(hash, |q| *q == key) {
        Occupied(mut o) => assert_eq!(Some(o.get_key_value()), key_value),
        Vacant(_) => assert_eq!(value, None),
    }
}

assert_eq!(map.len(), 6);
```

#### Implementations

- <span id="rawentrybuildermut-from-key"></span>`fn from_key<Q>(self, k: &Q) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](#rawentrymut)

  Creates a `RawEntryMut` from the given key.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let key = "a";

  let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key(&key);

  entry.insert(key, 100);

  assert_eq!(map[&"a"], 100);

  ```

- <span id="rawentrybuildermut-from-key-hashed-nocheck"></span>`fn from_key_hashed_nocheck<Q>(self, hash: u64, k: &Q) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](#rawentrymut)

  Creates a `RawEntryMut` from the given key and its hash.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let key = "a";

  let hash = compute_hash(map.hasher(), &key);

  let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);

  entry.insert(key, 100);

  assert_eq!(map[&"a"], 100);

  ```

#### Trait Implementations

##### `impl Any for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V, S, A: Allocator> Debug for RawEntryBuilderMut<'_, K, V, S, A>`

- <span id="rawentrybuildermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawentrybuildermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawentrybuildermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawOccupiedEntryMut<'a, K, V, S, A: Allocator>`

```rust
struct RawOccupiedEntryMut<'a, K, V, S, A: Allocator> {
    elem: crate::raw::Bucket<(K, V)>,
    table: &'a mut crate::raw::RawTable<(K, V), A>,
    hash_builder: &'a S,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:395-399`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L395-L399)*

A view into an occupied entry in a `HashMap`.
It is part of the [`RawEntryMut`](#rawentrymut) enum.

# Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::{HashMap, RawEntryMut, RawOccupiedEntryMut};

let mut map = HashMap::new();
map.extend([("a", 10), ("b", 20), ("c", 30)]);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

let _raw_o: RawOccupiedEntryMut<_, _, _> = map.raw_entry_mut().from_key(&"a").insert("a", 100);
assert_eq!(map.len(), 3);

// Existing key (insert and update)
match map.raw_entry_mut().from_key(&"a") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(mut view) => {
        assert_eq!(view.get(), &100);
        let v = view.get_mut();
        let new_v = (*v) * 10;
        *v = new_v;
        assert_eq!(view.insert(1111), 1000);
    }
}

assert_eq!(map[&"a"], 1111);
assert_eq!(map.len(), 3);

// Existing key (take)
let hash = compute_hash(map.hasher(), &"c");
match map.raw_entry_mut().from_key_hashed_nocheck(hash, &"c") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(view) => {
        assert_eq!(view.remove_entry(), ("c", 30));
    }
}
assert_eq!(map.raw_entry().from_key(&"c"), None);
assert_eq!(map.len(), 2);

let hash = compute_hash(map.hasher(), &"b");
match map.raw_entry_mut().from_hash(hash, |q| *q == "b") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(view) => {
        assert_eq!(view.remove_entry(), ("b", 20));
    }
}
assert_eq!(map.get(&"b"), None);
assert_eq!(map.len(), 1);
```

#### Implementations

- <span id="rawoccupiedentrymut-key"></span>`fn key(&self) -> &K`

  Gets a reference to the key in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.key(), &"a")

  }

  ```

- <span id="rawoccupiedentrymut-key-mut"></span>`fn key_mut(&mut self) -> &mut K`

  Gets a mutable reference to the key in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => {

          *o.key_mut() = key_two.clone();

      }

  }

  assert_eq!(map[&key_two], 10);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-into-key"></span>`fn into_key(self) -> &'a mut K`

  Converts the entry into a mutable reference to the key in the entry

  with a lifetime bound to the map itself.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  let inside_key: &mut Rc<&str>;

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => inside_key = o.into_key(),

  }

  *inside_key = key_two.clone();

  

  assert_eq!(map[&key_two], 10);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-get"></span>`fn get(&self) -> &V`

  Gets a reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.get(), &100),

  }

  ```

- <span id="rawoccupiedentrymut-into-mut"></span>`fn into_mut(self) -> &'a mut V`

  Converts the `OccupiedEntry` into a mutable reference to the value in the entry

  with a lifetime bound to the map itself.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  let value: &mut u32;

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => value = o.into_mut(),

  }

  *value += 900;

  

  assert_eq!(map[&"a"], 1000);

  ```

- <span id="rawoccupiedentrymut-get-mut"></span>`fn get_mut(&mut self) -> &mut V`

  Gets a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => *o.get_mut() += 900,

  }

  

  assert_eq!(map[&"a"], 1000);

  ```

- <span id="rawoccupiedentrymut-get-key-value"></span>`fn get_key_value(&self) -> (&K, &V)`

  Gets a reference to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.get_key_value(), (&"a", &100)),

  }

  ```

- <span id="rawoccupiedentrymut-get-key-value-mut"></span>`fn get_key_value_mut(&mut self) -> (&mut K, &mut V)`

  Gets a mutable reference to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => {

          let (inside_key, inside_value) = o.get_key_value_mut();

          *inside_key = key_two.clone();

          *inside_value = 100;

      }

  }

  assert_eq!(map[&key_two], 100);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-into-key-value"></span>`fn into_key_value(self) -> (&'a mut K, &'a mut V)`

  Converts the `OccupiedEntry` into a mutable reference to the key and value in the entry

  with a lifetime bound to the map itself.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  let inside_key: &mut Rc<&str>;

  let inside_value: &mut u32;

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => {

          let tuple = o.into_key_value();

          inside_key = tuple.0;

          inside_value = tuple.1;

      }

  }

  *inside_key = key_two.clone();

  *inside_value = 100;

  assert_eq!(map[&key_two], 100);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-insert"></span>`fn insert(&mut self, value: V) -> V`

  Sets the value of the entry, and returns the entry's old value.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => assert_eq!(o.insert(1000), 100),

  }

  

  assert_eq!(map[&"a"], 1000);

  ```

- <span id="rawoccupiedentrymut-insert-key"></span>`fn insert_key(&mut self, key: K) -> K`

  Sets the value of the entry, and returns the entry's old value.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => {

          let old_key = o.insert_key(key_two.clone());

          assert!(Rc::ptr_eq(&old_key, &key_one));

      }

  }

  assert_eq!(map[&key_two], 10);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-remove"></span>`fn remove(self) -> V`

  Takes the value out of the entry, and returns it.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.remove(), 100),

  }

  assert_eq!(map.get(&"a"), None);

  ```

- <span id="rawoccupiedentrymut-remove-entry"></span>`fn remove_entry(self) -> (K, V)`

  Take the ownership of the key and value from the map.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.remove_entry(), ("a", 100)),

  }

  assert_eq!(map.get(&"a"), None);

  ```

- <span id="rawoccupiedentrymut-replace-entry-with"></span>`fn replace_entry_with<F>(self, f: F) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](#rawentrymut)

  Provides shared access to the key and owned access to the value of

  the entry and allows to replace or remove it based on the

  value of the returned option.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  let raw_entry = match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {

          assert_eq!(k, &"a");

          assert_eq!(v, 100);

          Some(v + 900)

      }),

  };

  let raw_entry = match raw_entry {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {

          assert_eq!(k, &"a");

          assert_eq!(v, 1000);

          None

      }),

  };

  match raw_entry {

      RawEntryMut::Vacant(_) => { },

      RawEntryMut::Occupied(_) => panic!(),

  };

  assert_eq!(map.get(&"a"), None);

  ```

#### Trait Implementations

##### `impl Any for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for RawOccupiedEntryMut<'_, K, V, S, A>`

- <span id="rawoccupiedentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<K, V, S, A> Send for RawOccupiedEntryMut<'_, K, V, S, A>`

##### `impl<K, V, S, A> Sync for RawOccupiedEntryMut<'_, K, V, S, A>`

##### `impl<U> TryFrom for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawoccupiedentrymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawoccupiedentrymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawVacantEntryMut<'a, K, V, S, A: Allocator>`

```rust
struct RawVacantEntryMut<'a, K, V, S, A: Allocator> {
    table: &'a mut crate::raw::RawTable<(K, V), A>,
    hash_builder: &'a S,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:466-469`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L466-L469)*

A view into a vacant entry in a `HashMap`.
It is part of the [`RawEntryMut`](#rawentrymut) enum.

# Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::{HashMap, RawEntryMut, RawVacantEntryMut};

let mut map = HashMap::<&str, i32>::new();

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

let raw_v: RawVacantEntryMut<_, _, _> = match map.raw_entry_mut().from_key(&"a") {
    RawEntryMut::Vacant(view) => view,
    RawEntryMut::Occupied(_) => unreachable!(),
};
raw_v.insert("a", 10);
assert!(map[&"a"] == 10 && map.len() == 1);

// Nonexistent key (insert and update)
let hash = compute_hash(map.hasher(), &"b");
match map.raw_entry_mut().from_key_hashed_nocheck(hash, &"b") {
    RawEntryMut::Occupied(_) => unreachable!(),
    RawEntryMut::Vacant(view) => {
        let (k, value) = view.insert("b", 2);
        assert_eq!((*k, *value), ("b", 2));
        *value = 20;
    }
}
assert!(map[&"b"] == 20 && map.len() == 2);

let hash = compute_hash(map.hasher(), &"c");
match map.raw_entry_mut().from_hash(hash, |q| *q == "c") {
    RawEntryMut::Occupied(_) => unreachable!(),
    RawEntryMut::Vacant(view) => {
        assert_eq!(view.insert("c", 30), (&mut "c", &mut 30));
    }
}
assert!(map[&"c"] == 30 && map.len() == 3);
```

#### Implementations

- <span id="rawvacantentrymut-insert"></span>`fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Sets the value of the entry with the `VacantEntry`'s key,

  and returns a mutable reference to it.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"c") {

      RawEntryMut::Occupied(_) => panic!(),

      RawEntryMut::Vacant(v) => assert_eq!(v.insert("c", 300), (&mut "c", &mut 300)),

  }

  

  assert_eq!(map[&"c"], 300);

  ```

- <span id="rawvacantentrymut-insert-hashed-nocheck"></span>`fn insert_hashed_nocheck(self, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Sets the value of the entry with the `VacantEntry`'s key,

  and returns a mutable reference to it.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "c";

  let hash = compute_hash(map.hasher(), &key);

  

  match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {

      RawEntryMut::Occupied(_) => panic!(),

      RawEntryMut::Vacant(v) => assert_eq!(

          v.insert_hashed_nocheck(hash, key, 300),

          (&mut "c", &mut 300)

      ),

  }

  

  assert_eq!(map[&"c"], 300);

  ```

- <span id="rawvacantentrymut-insert-with-hasher"></span>`fn insert_with_hasher<H>(self, hash: u64, key: K, value: V, hasher: H) -> (&'a mut K, &'a mut V)`

  Set the value of an entry with a custom hasher function.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  fn make_hasher<K, S>(hash_builder: &S) -> impl Fn(&K) -> u64 + '_

  where

      K: Hash + ?Sized,

      S: BuildHasher,

  {

      move |key: &K| {

          use core::hash::Hasher;

          let mut state = hash_builder.build_hasher();

          key.hash(&mut state);

          state.finish()

      }

  }

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let key = "a";

  let hash_builder = map.hasher().clone();

  let hash = make_hasher(&hash_builder)(&key);

  

  match map.raw_entry_mut().from_hash(hash, |q| q == &key) {

      RawEntryMut::Occupied(_) => panic!(),

      RawEntryMut::Vacant(v) => assert_eq!(

          v.insert_with_hasher(hash, key, 100, make_hasher(&hash_builder)),

          (&mut "a", &mut 100)

      ),

  }

  map.extend([("b", 200), ("c", 300), ("d", 400), ("e", 500), ("f", 600)]);

  assert_eq!(map[&"a"], 100);

  ```

- <span id="rawvacantentrymut-insert-entry"></span>`fn insert_entry(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>` — [`RawOccupiedEntryMut`](#rawoccupiedentrymut)

#### Trait Implementations

##### `impl Any for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V, S, A: Allocator> Debug for RawVacantEntryMut<'_, K, V, S, A>`

- <span id="rawvacantentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawvacantentrymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawvacantentrymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawEntryBuilder<'a, K, V, S, A: Allocator>`

```rust
struct RawEntryBuilder<'a, K, V, S, A: Allocator> {
    map: &'a crate::HashMap<K, V, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:505-507`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L505-L507)*

A builder for computing where in a [`HashMap`](../hash_map/index.md) a key-value pair would be stored.

See the `HashMap::raw_entry` docs for usage examples.

# Examples

```rust
use hashbrown::hash_map::{HashMap, RawEntryBuilder};
use core::hash::{BuildHasher, Hash};

let mut map = HashMap::new();
map.extend([(1, 10), (2, 20), (3, 30)]);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

for k in 0..6 {
    let hash = compute_hash(map.hasher(), &k);
    let v = map.get(&k).cloned();
    let kv = v.as_ref().map(|v| (&k, v));

    println!("Key: {} and value: {:?}", k, v);
    let builder: RawEntryBuilder<_, _, _> = map.raw_entry();
    assert_eq!(builder.from_key(&k), kv);
    assert_eq!(map.raw_entry().from_hash(hash, |q| *q == k), kv);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &k), kv);
}
```

#### Implementations

- <span id="rawentrybuilder-from-key"></span>`fn from_key<Q>(self, k: &Q) -> Option<(&'a K, &'a V)>`

  Access an immutable entry by key.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "a";

  assert_eq!(map.raw_entry().from_key(&key), Some((&"a", &100)));

  ```

- <span id="rawentrybuilder-from-key-hashed-nocheck"></span>`fn from_key_hashed_nocheck<Q>(self, hash: u64, k: &Q) -> Option<(&'a K, &'a V)>`

  Access an immutable entry by a key and its hash.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::HashMap;

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "a";

  let hash = compute_hash(map.hasher(), &key);

  assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &key), Some((&"a", &100)));

  ```

- <span id="rawentrybuilder-search"></span>`fn search<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>`

- <span id="rawentrybuilder-from-hash"></span>`fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>`

  Access an immutable entry by hash and matching function.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::HashMap;

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "a";

  let hash = compute_hash(map.hasher(), &key);

  assert_eq!(map.raw_entry().from_hash(hash, |k| k == &key), Some((&"a", &100)));

  ```

#### Trait Implementations

##### `impl Any for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V, S, A: Allocator> Debug for RawEntryBuilder<'_, K, V, S, A>`

- <span id="rawentrybuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawentrybuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawentrybuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `RawEntryMut<'a, K, V, S, A: Allocator>`

```rust
enum RawEntryMut<'a, K, V, S, A: Allocator> {
    Occupied(RawOccupiedEntryMut<'a, K, V, S, A>),
    Vacant(RawVacantEntryMut<'a, K, V, S, A>),
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:304-333`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L304-L333)*

A view into a single entry in a map, which may either be vacant or occupied.

This is a lower-level version of [`Entry`](../hash_set/index.md).

This `enum` is constructed through the `raw_entry_mut` method on [`HashMap`](../hash_map/index.md),
then calling one of the methods of that [`RawEntryBuilderMut`](#rawentrybuildermut).




# Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::{HashMap, RawEntryMut, RawOccupiedEntryMut};

let mut map = HashMap::new();
map.extend([('a', 1), ('b', 2), ('c', 3)]);
assert_eq!(map.len(), 3);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

// Existing key (insert)
let raw: RawEntryMut<_, _, _> = map.raw_entry_mut().from_key(&'a');
let _raw_o: RawOccupiedEntryMut<_, _, _> = raw.insert('a', 10);
assert_eq!(map.len(), 3);

// Nonexistent key (insert)
map.raw_entry_mut().from_key(&'d').insert('d', 40);
assert_eq!(map.len(), 4);

// Existing key (or_insert)
let hash = compute_hash(map.hasher(), &'b');
let kv = map
    .raw_entry_mut()
    .from_key_hashed_nocheck(hash, &'b')
    .or_insert('b', 20);
assert_eq!(kv, (&mut 'b', &mut 2));
*kv.1 = 20;
assert_eq!(map.len(), 4);

// Nonexistent key (or_insert)
let hash = compute_hash(map.hasher(), &'e');
let kv = map
    .raw_entry_mut()
    .from_key_hashed_nocheck(hash, &'e')
    .or_insert('e', 50);
assert_eq!(kv, (&mut 'e', &mut 50));
assert_eq!(map.len(), 5);

// Existing key (or_insert_with)
let hash = compute_hash(map.hasher(), &'c');
let kv = map
    .raw_entry_mut()
    .from_hash(hash, |q| q == &'c')
    .or_insert_with(|| ('c', 30));
assert_eq!(kv, (&mut 'c', &mut 3));
*kv.1 = 30;
assert_eq!(map.len(), 5);

// Nonexistent key (or_insert_with)
let hash = compute_hash(map.hasher(), &'f');
let kv = map
    .raw_entry_mut()
    .from_hash(hash, |q| q == &'f')
    .or_insert_with(|| ('f', 60));
assert_eq!(kv, (&mut 'f', &mut 60));
assert_eq!(map.len(), 6);

println!("Our HashMap: {:?}", map);

let mut vec: Vec<_> = map.iter().map(|(&k, &v)| (k, v)).collect();
// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [('a', 10), ('b', 20), ('c', 30), ('d', 40), ('e', 50), ('f', 60)]);
```

#### Variants

- **`Occupied`**

  An occupied entry.
  
  # Examples
  
  ```rust
  use hashbrown::{hash_map::RawEntryMut, HashMap};
  let mut map: HashMap<_, _> = [("a", 100), ("b", 200)].into();
  
  match map.raw_entry_mut().from_key(&"a") {
      RawEntryMut::Vacant(_) => unreachable!(),
      RawEntryMut::Occupied(_) => { }
  }
  ```

- **`Vacant`**

  A vacant entry.
  
  # Examples
  
  ```rust
  use hashbrown::{hash_map::RawEntryMut, HashMap};
  let mut map: HashMap<&str, i32> = HashMap::new();
  
  match map.raw_entry_mut().from_key("a") {
      RawEntryMut::Occupied(_) => unreachable!(),
      RawEntryMut::Vacant(_) => { }
  }
  ```

#### Implementations

- <span id="rawentrymut-insert"></span>`fn insert(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>` — [`RawOccupiedEntryMut`](#rawoccupiedentrymut)

  Sets the value of the entry, and returns a `RawOccupiedEntryMut`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let entry = map.raw_entry_mut().from_key("horseyland").insert("horseyland", 37);

  

  assert_eq!(entry.remove_entry(), ("horseyland", 37));

  ```

- <span id="rawentrymut-or-insert"></span>`fn or_insert(self, default_key: K, default_val: V) -> (&'a mut K, &'a mut V)`

  Ensures a value is in the entry by inserting the default if empty, and returns

  mutable references to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 3);

  assert_eq!(map["poneyland"], 3);

  

  *map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 10).1 *= 2;

  assert_eq!(map["poneyland"], 6);

  ```

- <span id="rawentrymut-or-insert-with"></span>`fn or_insert_with<F>(self, default: F) -> (&'a mut K, &'a mut V)`

  Ensures a value is in the entry by inserting the result of the default function if empty,

  and returns mutable references to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, String> = HashMap::new();

  

  map.raw_entry_mut().from_key("poneyland").or_insert_with(|| {

      ("poneyland", "hoho".to_string())

  });

  

  assert_eq!(map["poneyland"], "hoho".to_string());

  ```

- <span id="rawentrymut-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

  Provides in-place mutable access to an occupied entry before any

  potential inserts into the map.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  map.raw_entry_mut()

     .from_key("poneyland")

     .and_modify(|_k, v| { *v += 1 })

     .or_insert("poneyland", 42);

  assert_eq!(map["poneyland"], 42);

  

  map.raw_entry_mut()

     .from_key("poneyland")

     .and_modify(|_k, v| { *v += 1 })

     .or_insert("poneyland", 0);

  assert_eq!(map["poneyland"], 43);

  ```

- <span id="rawentrymut-and-replace-entry-with"></span>`fn and_replace_entry_with<F>(self, f: F) -> Self`

  Provides shared access to the key and owned access to the value of

  an occupied entry and allows to replace or remove it based on the

  value of the returned option.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::RawEntryMut;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  let entry = map

      .raw_entry_mut()

      .from_key("poneyland")

      .and_replace_entry_with(|_k, _v| panic!());

  

  match entry {

      RawEntryMut::Vacant(_) => {},

      RawEntryMut::Occupied(_) => panic!(),

  }

  

  map.insert("poneyland", 42);

  

  let entry = map

      .raw_entry_mut()

      .from_key("poneyland")

      .and_replace_entry_with(|k, v| {

          assert_eq!(k, &"poneyland");

          assert_eq!(v, 42);

          Some(v + 1)

      });

  

  match entry {

      RawEntryMut::Occupied(e) => {

          assert_eq!(e.key(), &"poneyland");

          assert_eq!(e.get(), &43);

      },

      RawEntryMut::Vacant(_) => panic!(),

  }

  

  assert_eq!(map["poneyland"], 43);

  

  let entry = map

      .raw_entry_mut()

      .from_key("poneyland")

      .and_replace_entry_with(|_k, _v| None);

  

  match entry {

      RawEntryMut::Vacant(_) => {},

      RawEntryMut::Occupied(_) => panic!(),

  }

  

  assert!(!map.contains_key("poneyland"));

  ```

#### Trait Implementations

##### `impl Any for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for RawEntryMut<'_, K, V, S, A>`

- <span id="rawentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawentrymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawentrymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

