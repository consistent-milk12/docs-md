*[hashbrown](../index.md) / [raw_entry](index.md)*

---

# Module `raw_entry`

## Structs

### `RawEntryBuilderMut<'a, K, V, S, A: Allocator>`

```rust
struct RawEntryBuilderMut<'a, K, V, S, A: Allocator> {
    map: &'a mut crate::HashMap<K, V, S, A>,
}
```

A builder for computing where in a [`HashMap`](../index.md) a key-value pair would be stored.

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

- `fn from_key<Q>(self: Self, k: &Q) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](#rawentrymut)

- `fn from_key_hashed_nocheck<Q>(self: Self, hash: u64, k: &Q) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](#rawentrymut)

#### Trait Implementations

##### `impl<K, V, S, A: Allocator> Debug for RawEntryBuilderMut<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RawOccupiedEntryMut<'a, K, V, S, A: Allocator>`

```rust
struct RawOccupiedEntryMut<'a, K, V, S, A: Allocator> {
    elem: crate::raw::Bucket<(K, V)>,
    table: &'a mut crate::raw::RawTable<(K, V), A>,
    hash_builder: &'a S,
}
```

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

- `fn key(self: &Self) -> &K`

- `fn key_mut(self: &mut Self) -> &mut K`

- `fn into_key(self: Self) -> &'a mut K`

- `fn get(self: &Self) -> &V`

- `fn into_mut(self: Self) -> &'a mut V`

- `fn get_mut(self: &mut Self) -> &mut V`

- `fn get_key_value(self: &Self) -> (&K, &V)`

- `fn get_key_value_mut(self: &mut Self) -> (&mut K, &mut V)`

- `fn into_key_value(self: Self) -> (&'a mut K, &'a mut V)`

- `fn insert(self: &mut Self, value: V) -> V`

- `fn insert_key(self: &mut Self, key: K) -> K`

- `fn remove(self: Self) -> V`

- `fn remove_entry(self: Self) -> (K, V)`

- `fn replace_entry_with<F>(self: Self, f: F) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](#rawentrymut)

#### Trait Implementations

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for RawOccupiedEntryMut<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S, A> Send for RawOccupiedEntryMut<'_, K, V, S, A>`

##### `impl<K, V, S, A> Sync for RawOccupiedEntryMut<'_, K, V, S, A>`

### `RawVacantEntryMut<'a, K, V, S, A: Allocator>`

```rust
struct RawVacantEntryMut<'a, K, V, S, A: Allocator> {
    table: &'a mut crate::raw::RawTable<(K, V), A>,
    hash_builder: &'a S,
}
```

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

- `fn insert(self: Self, key: K, value: V) -> (&'a mut K, &'a mut V)`

- `fn insert_hashed_nocheck(self: Self, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V)`

- `fn insert_with_hasher<H>(self: Self, hash: u64, key: K, value: V, hasher: H) -> (&'a mut K, &'a mut V)`

- `fn insert_entry(self: Self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>` — [`RawOccupiedEntryMut`](#rawoccupiedentrymut)

#### Trait Implementations

##### `impl<K, V, S, A: Allocator> Debug for RawVacantEntryMut<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RawEntryBuilder<'a, K, V, S, A: Allocator>`

```rust
struct RawEntryBuilder<'a, K, V, S, A: Allocator> {
    map: &'a crate::HashMap<K, V, S, A>,
}
```

A builder for computing where in a [`HashMap`](../index.md) a key-value pair would be stored.

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

- `fn from_key<Q>(self: Self, k: &Q) -> Option<(&'a K, &'a V)>`

- `fn from_key_hashed_nocheck<Q>(self: Self, hash: u64, k: &Q) -> Option<(&'a K, &'a V)>`

- `fn search<F>(self: Self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>`

- `fn from_hash<F>(self: Self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>`

#### Trait Implementations

##### `impl<K, V, S, A: Allocator> Debug for RawEntryBuilder<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `RawEntryMut<'a, K, V, S, A: Allocator>`

```rust
enum RawEntryMut<'a, K, V, S, A: Allocator> {
    Occupied(RawOccupiedEntryMut<'a, K, V, S, A>),
    Vacant(RawVacantEntryMut<'a, K, V, S, A>),
}
```

A view into a single entry in a map, which may either be vacant or occupied.

This is a lower-level version of [`Entry`](../hash_table/index.md).

This `enum` is constructed through the `raw_entry_mut` method on [`HashMap`](../index.md),
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

- `fn insert(self: Self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>` — [`RawOccupiedEntryMut`](#rawoccupiedentrymut)

- `fn or_insert(self: Self, default_key: K, default_val: V) -> (&'a mut K, &'a mut V)`

- `fn or_insert_with<F>(self: Self, default: F) -> (&'a mut K, &'a mut V)`

- `fn and_modify<F>(self: Self, f: F) -> Self`

- `fn and_replace_entry_with<F>(self: Self, f: F) -> Self`

#### Trait Implementations

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for RawEntryMut<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

