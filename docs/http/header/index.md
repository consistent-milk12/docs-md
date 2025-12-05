*[http](../index.md) / [header](index.md)*

---

# Module `header`

HTTP header types

The module provides [`HeaderName`](../index.md), [`HeaderMap`](../index.md), and a number of types
used for interacting with `HeaderMap`. These types allow representing both
HTTP/1 and HTTP/2 headers.

# `HeaderName`

The `HeaderName` type represents both standard header names as well as
custom header names. The type handles the case insensitive nature of header
names and is used as the key portion of `HeaderMap`. Header names are
normalized to lower case. In other words, when creating a `HeaderName` with
a string, even if upper case characters are included, when getting a string
representation of the `HeaderName`, it will be all lower case. This allows
for faster `HeaderMap` comparison operations.

The internal representation is optimized to efficiently handle the cases
most commonly encountered when working with HTTP. Standard header names are
special cased and are represented internally as an enum. Short custom
headers will be stored directly in the `HeaderName` struct and will not
incur any allocation overhead, however longer strings will require an
allocation for storage.

## Limitations

`HeaderName` has a max length of 32,768 for header names. Attempting to
parse longer names will result in a panic.

# `HeaderMap`

The [`HeaderMap`](../index.md) type is a specialized
[multimap](<https://en.wikipedia.org/wiki/Multimap>) structure for storing
header names and values. It is designed specifically for efficient
manipulation of HTTP headers. It supports multiple values per header name
and provides specialized APIs for insertion, retrieval, and iteration.

[*See also the `HeaderMap` type.*](HeaderMap)

## Structs

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    // [REDACTED: Private Fields]
}
```

A drain iterator for `HeaderMap`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Drop<'a, T>`

- `fn drop(self: &mut Self)`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T>`

- `type Item = (Option<HeaderName>, T)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl Sync<'a, T: Sync>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Send<'a, T: Send>`

### `GetAll<'a, T>`

```rust
struct GetAll<'a, T> {
    // [REDACTED: Private Fields]
}
```

A view to all values stored in a single entry.

This struct is returned by `HeaderMap::get_all`.

#### Implementations

- `fn iter(self: &Self) -> ValueIter<'a, T>`
  Returns an iterator visiting all values associated with the entry.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<'a, T>`

- `type Item = &'a T`

- `type IntoIter = ValueIter<'a, T>`

- `fn into_iter(self: Self) -> ValueIter<'a, T>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl PartialEq<'a, T: PartialEq>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HeaderMap<T>`

```rust
struct HeaderMap<T> {
    // [REDACTED: Private Fields]
}
```

A specialized [multimap](<https://en.wikipedia.org/wiki/Multimap>) for
header names and values.

# Overview

`HeaderMap` is designed specifically for efficient manipulation of HTTP
headers. It supports multiple values per header name and provides
specialized APIs for insertion, retrieval, and iteration.

The internal implementation is optimized for common usage patterns in HTTP,
and may change across versions. For example, the current implementation uses
[Robin Hood
hashing](<https://en.wikipedia.org/wiki/Hash_table#Robin_Hood_hashing>) to
store entries compactly and enable high load factors with good performance.
However, the collision resolution strategy and storage mechanism are not
part of the public API and may be altered in future releases.

# Iteration order

Unless otherwise specified, the order in which items are returned by
iterators from `HeaderMap` methods is arbitrary; there is no guaranteed
ordering among the elements yielded by such an iterator. Changes to the
iteration order are not considered breaking changes, so users must not rely
on any incidental order produced by such an iterator. However, for a given
crate version, the iteration order will be consistent across all platforms.

# Adaptive hashing

`HeaderMap` uses an adaptive strategy for hashing to maintain fast lookups
while resisting hash collision attacks. The default hash function
prioritizes performance. In scenarios where high collision rates are
detected—typically indicative of denial-of-service attacks—the
implementation switches to a more secure, collision-resistant hash function.

# Limitations

A `HeaderMap` can store at most 32,768 entries \(header name/value pairs\).
Attempting to exceed this limit will result in a panic.


# Examples

Basic usage

```rust
use http::HeaderMap;
use http::header::{CONTENT_LENGTH, HOST, LOCATION};
let mut headers = HeaderMap::new();

headers.insert(HOST, "example.com".parse().unwrap());
headers.insert(CONTENT_LENGTH, "123".parse().unwrap());

assert!(headers.contains_key(HOST));
assert!(!headers.contains_key(LOCATION));

assert_eq!(headers[HOST], "example.com");

headers.remove(HOST);

assert!(!headers.contains_key(HOST));
```

#### Implementations

- `fn with_capacity(capacity: usize) -> HeaderMap<T>`
  Create an empty `HeaderMap` with the specified capacity.

- `fn try_with_capacity(capacity: usize) -> Result<HeaderMap<T>, MaxSizeReached>`
  Create an empty `HeaderMap` with the specified capacity.

- `fn len(self: &Self) -> usize`
  Returns the number of headers stored in the map.

- `fn keys_len(self: &Self) -> usize`
  Returns the number of keys stored in the map.

- `fn is_empty(self: &Self) -> bool`
  Returns true if the map contains no elements.

- `fn clear(self: &mut Self)`
  Clears the map, removing all key-value pairs. Keeps the allocated memory

- `fn capacity(self: &Self) -> usize`
  Returns the number of headers the map can hold without reallocating.

- `fn reserve(self: &mut Self, additional: usize)`
  Reserves capacity for at least `additional` more headers to be inserted

- `fn try_reserve(self: &mut Self, additional: usize) -> Result<(), MaxSizeReached>`
  Reserves capacity for at least `additional` more headers to be inserted

- `fn get<K>(self: &Self, key: K) -> Option<&T>`
  Returns a reference to the value associated with the key.

- `fn get_mut<K>(self: &mut Self, key: K) -> Option<&mut T>`
  Returns a mutable reference to the value associated with the key.

- `fn get_all<K>(self: &Self, key: K) -> GetAll<'_, T>`
  Returns a view of all values associated with a key.

- `fn contains_key<K>(self: &Self, key: K) -> bool`
  Returns true if the map contains a value for the specified key.

- `fn iter(self: &Self) -> Iter<'_, T>`
  An iterator visiting all key-value pairs.

- `fn iter_mut(self: &mut Self) -> IterMut<'_, T>`
  An iterator visiting all key-value pairs, with mutable value references.

- `fn keys(self: &Self) -> Keys<'_, T>`
  An iterator visiting all keys.

- `fn values(self: &Self) -> Values<'_, T>`
  An iterator visiting all values.

- `fn values_mut(self: &mut Self) -> ValuesMut<'_, T>`
  An iterator visiting all values mutably.

- `fn drain(self: &mut Self) -> Drain<'_, T>`
  Clears the map, returning all entries as an iterator.

- `fn entry<K>(self: &mut Self, key: K) -> Entry<'_, T>`
  Gets the given key's corresponding entry in the map for in-place

- `fn try_entry<K>(self: &mut Self, key: K) -> Result<Entry<'_, T>, InvalidHeaderName>`
  Gets the given key's corresponding entry in the map for in-place

- `fn insert<K>(self: &mut Self, key: K, val: T) -> Option<T>`
  Inserts a key-value pair into the map.

- `fn try_insert<K>(self: &mut Self, key: K, val: T) -> Result<Option<T>, MaxSizeReached>`
  Inserts a key-value pair into the map.

- `fn append<K>(self: &mut Self, key: K, value: T) -> bool`
  Inserts a key-value pair into the map.

- `fn try_append<K>(self: &mut Self, key: K, value: T) -> Result<bool, MaxSizeReached>`
  Inserts a key-value pair into the map.

- `fn remove<K>(self: &mut Self, key: K) -> Option<T>`
  Removes a key from the map, returning the value associated with the key.

- `fn new() -> Self`
  Create an empty `HeaderMap`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator<T>`

- `fn from_iter<I>(iter: I) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<T>`

- `type Item = (Option<HeaderName>, T)`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> IntoIter<T>`
  Creates a consuming iterator, that is, one that moves keys and values

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> HeaderMap<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<T: Eq>`

##### `impl Extend<T>`

- `fn extend<I: IntoIterator<Item = (Option<HeaderName>, T)>>(self: &mut Self, iter: I)`
  Extend a `HeaderMap` with the contents of another `HeaderMap`.

##### `impl Extend<T>`

- `fn extend<I: IntoIterator<Item = (HeaderName, T)>>(self: &mut Self, iter: I)`

##### `impl Index<K, T>`

- `type Output = T`

- `fn index(self: &Self, index: K) -> &T`
  # Panics

##### `impl PartialEq<T: PartialEq>`

- `fn eq(self: &Self, other: &HeaderMap<T>) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<'a, K, V, S, T>`

- `type Error = Error`

- `fn try_from(c: &'a HashMap<K, V, S>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T>`

- `fn default() -> Self`

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    // [REDACTED: Private Fields]
}
```

An owning iterator over the entries of a `HeaderMap`.

This struct is created by the `into_iter` method on `HeaderMap`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Drop<T>`

- `fn drop(self: &mut Self)`

##### `impl FusedIterator<T>`

##### `impl Iterator<T>`

- `type Item = (Option<HeaderName>, T)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    // [REDACTED: Private Fields]
}
```

`HeaderMap` entry iterator.

Yields `(&HeaderName, &value)` tuples. The same header name may be yielded
more than once if it has more than one associated value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T>`

- `type Item = (&'a HeaderName, &'a T)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl Sync<'a, T: Sync>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Send<'a, T: Sync>`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    // [REDACTED: Private Fields]
}
```

`HeaderMap` mutable entry iterator

Yields `(&HeaderName, &mut value)` tuples. The same header name may be
yielded more than once if it has more than one associated value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T>`

- `type Item = (&'a HeaderName, &'a mut T)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl Sync<'a, T: Sync>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Send<'a, T: Send>`

### `Keys<'a, T>`

```rust
struct Keys<'a, T> {
    // [REDACTED: Private Fields]
}
```

An iterator over `HeaderMap` keys.

Each header name is yielded only once, even if it has more than one
associated value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl ExactSizeIterator<'a, T>`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T>`

- `type Item = &'a HeaderName`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

- `fn count(self: Self) -> usize`

- `fn last(self: Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MaxSizeReached`

```rust
struct MaxSizeReached {
    // [REDACTED: Private Fields]
}
```

Error returned when max capacity of `HeaderMap` is exceeded

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `OccupiedEntry<'a, T>`

```rust
struct OccupiedEntry<'a, T> {
    // [REDACTED: Private Fields]
}
```

A view into a single occupied location in a `HeaderMap`.

This struct is returned as part of the `Entry` enum.

#### Implementations

- `fn key(self: &Self) -> &HeaderName`
  Returns a reference to the entry's key.

- `fn get(self: &Self) -> &T`
  Get a reference to the first value in the entry.

- `fn get_mut(self: &mut Self) -> &mut T`
  Get a mutable reference to the first value in the entry.

- `fn into_mut(self: Self) -> &'a mut T`
  Converts the `OccupiedEntry` into a mutable reference to the **first**

- `fn insert(self: &mut Self, value: T) -> T`
  Sets the value of the entry.

- `fn insert_mult(self: &mut Self, value: T) -> ValueDrain<'_, T>`
  Sets the value of the entry.

- `fn append(self: &mut Self, value: T)`
  Insert the value into the entry.

- `fn remove(self: Self) -> T`
  Remove the entry from the map.

- `fn remove_entry(self: Self) -> (HeaderName, T)`
  Remove the entry from the map.

- `fn remove_entry_mult(self: Self) -> (HeaderName, ValueDrain<'a, T>)`
  Remove the entry from the map.

- `fn iter(self: &Self) -> ValueIter<'_, T>`
  Returns an iterator visiting all values associated with the entry.

- `fn iter_mut(self: &mut Self) -> ValueIterMut<'_, T>`
  Returns an iterator mutably visiting all values associated with the

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<'a, T>`

- `type Item = &'a mut T`

- `type IntoIter = ValueIterMut<'a, T>`

- `fn into_iter(self: Self) -> ValueIterMut<'a, T>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `VacantEntry<'a, T>`

```rust
struct VacantEntry<'a, T> {
    // [REDACTED: Private Fields]
}
```

A view into a single empty location in a `HeaderMap`.

This struct is returned as part of the `Entry` enum.

#### Implementations

- `fn key(self: &Self) -> &HeaderName`
  Returns a reference to the entry's key

- `fn into_key(self: Self) -> HeaderName`
  Take ownership of the key

- `fn insert(self: Self, value: T) -> &'a mut T`
  Insert the value into the entry.

- `fn try_insert(self: Self, value: T) -> Result<&'a mut T, MaxSizeReached>`
  Insert the value into the entry.

- `fn insert_entry(self: Self, value: T) -> OccupiedEntry<'a, T>`
  Insert the value into the entry.

- `fn try_insert_entry(self: Self, value: T) -> Result<OccupiedEntry<'a, T>, MaxSizeReached>`
  Insert the value into the entry.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ValueDrain<'a, T>`

```rust
struct ValueDrain<'a, T> {
    // [REDACTED: Private Fields]
}
```

An drain iterator of all values associated with a single header name.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Drop<'a, T>`

- `fn drop(self: &mut Self)`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl Sync<'a, T: Sync>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Send<'a, T: Send>`

### `ValueIter<'a, T>`

```rust
struct ValueIter<'a, T> {
    // [REDACTED: Private Fields]
}
```

An iterator of all values associated with a single header name.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl DoubleEndedIterator<'a, T: 'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T: 'a>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ValueIterMut<'a, T>`

```rust
struct ValueIterMut<'a, T> {
    // [REDACTED: Private Fields]
}
```

A mutable iterator of all values associated with a single header name.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl DoubleEndedIterator<'a, T: 'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T: 'a>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl Sync<'a, T: Sync>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Send<'a, T: Send>`

### `Values<'a, T>`

```rust
struct Values<'a, T> {
    // [REDACTED: Private Fields]
}
```

`HeaderMap` value iterator.

Each value contained in the `HeaderMap` will be yielded.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ValuesMut<'a, T>`

```rust
struct ValuesMut<'a, T> {
    // [REDACTED: Private Fields]
}
```

`HeaderMap` mutable value iterator

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator<'a, T>`

##### `impl Iterator<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HeaderName`

```rust
struct HeaderName {
    // [REDACTED: Private Fields]
}
```

Represents an HTTP header field name

Header field names identify the header. Header sets may include multiple
headers with the same name. The HTTP specification defines a number of
standard headers, but HTTP messages may include non-standard header names as
well as long as they adhere to the specification.

`HeaderName` is used as the [`HeaderMap`](../index.md) key. Constants are available for
all standard header names in the [`header`](#header) module.

# Representation

`HeaderName` represents standard header names using an `enum`, as such they
will not require an allocation for storage. All custom header names are
lower cased upon conversion to a `HeaderName` value. This avoids the
overhead of dynamically doing lower case conversion during the hash code
computation and the comparison operation.



#### Implementations

- `fn from_bytes(src: &[u8]) -> Result<HeaderName, InvalidHeaderName>`
  Converts a slice of bytes to an HTTP header name.

- `fn from_lowercase(src: &[u8]) -> Result<HeaderName, InvalidHeaderName>`
  Converts a slice of bytes to an HTTP header name.

- `const fn from_static(src: &'static str) -> HeaderName`
  Converts a static string to a HTTP header name.

- `fn as_str(self: &Self) -> &str`
  Returns a `str` representation of the header.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(src: &'a HeaderName) -> HeaderName`

##### `impl FromStr`

- `type Err = InvalidHeaderName`

- `fn from_str(s: &str) -> Result<HeaderName, InvalidHeaderName>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoHeaderName`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsHeaderName`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow`

- `fn borrow(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> HeaderName`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a str) -> bool`
  Performs a case-insensitive comparison of the string against the header

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a HeaderName) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HeaderName) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`
  Performs a case-insensitive comparison of the string against the header

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom`

- `type Error = InvalidHeaderName`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderName`

- `fn try_from(s: &'a String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidHeaderName`

- `fn try_from(s: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderName`

- `fn try_from(s: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderName`

- `fn try_from(s: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `InvalidHeaderName`

```rust
struct InvalidHeaderName {
    // [REDACTED: Private Fields]
}
```

A possible error when converting a `HeaderName` from another type.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HeaderValue`

```rust
struct HeaderValue {
    // [REDACTED: Private Fields]
}
```

Represents an HTTP header field value.

In practice, HTTP header field values are usually valid ASCII. However, the
HTTP spec allows for a header value to contain opaque bytes as well. In this
case, the header field value is not able to be represented as a string.

To handle this, the `HeaderValue` is usable as a type and can be compared
with strings and implements `Debug`. A `to_str` fn is provided that returns
an `Err` if the header value contains non visible ascii characters.

#### Implementations

- `const fn from_static(src: &'static str) -> HeaderValue`
  Convert a static string to a `HeaderValue`.

- `fn from_str(src: &str) -> Result<HeaderValue, InvalidHeaderValue>`
  Attempt to convert a string to a `HeaderValue`.

- `fn from_name(name: HeaderName) -> HeaderValue`
  Converts a HeaderName into a HeaderValue

- `fn from_bytes(src: &[u8]) -> Result<HeaderValue, InvalidHeaderValue>`
  Attempt to convert a byte slice to a `HeaderValue`.

- `fn from_maybe_shared<T>(src: T) -> Result<HeaderValue, InvalidHeaderValue>`
  Attempt to convert a `Bytes` buffer to a `HeaderValue`.

- `unsafe fn from_maybe_shared_unchecked<T>(src: T) -> HeaderValue`
  Convert a `Bytes` directly into a `HeaderValue` without validating.

- `fn to_str(self: &Self) -> Result<&str, ToStrError>`
  Yields a `&str` slice if the `HeaderValue` only contains visible ASCII

- `fn len(self: &Self) -> usize`
  Returns the length of `self`.

- `fn is_empty(self: &Self) -> bool`
  Returns true if the `HeaderValue` has a length of zero bytes.

- `fn as_bytes(self: &Self) -> &[u8]`
  Converts a `HeaderValue` to a byte slice.

- `fn set_sensitive(self: &mut Self, val: bool)`
  Mark that the header value represents sensitive information.

- `fn is_sensitive(self: &Self) -> bool`
  Returns `true` if the value represents sensitive data.

#### Trait Implementations

##### `impl From`

- `fn from(num: i32) -> HeaderValue`

##### `impl From`

- `fn from(num: usize) -> HeaderValue`

##### `impl From`

- `fn from(num: u32) -> HeaderValue`

##### `impl From`

- `fn from(num: i64) -> HeaderValue`

##### `impl From`

- `fn from(num: u16) -> HeaderValue`

##### `impl From`

- `fn from(h: HeaderName) -> HeaderValue`

##### `impl From<'a>`

- `fn from(t: &'a HeaderValue) -> Self`

##### `impl From`

- `fn from(num: u64) -> HeaderValue`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(num: i16) -> HeaderValue`

##### `impl From`

- `fn from(num: isize) -> HeaderValue`

##### `impl FromStr`

- `type Err = InvalidHeaderValue`

- `fn from_str(s: &str) -> Result<HeaderValue, <Self as >::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> HeaderValue`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HeaderValue) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &[u8]) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq<'a, T: ?Sized>`

- `fn eq(self: &Self, other: &&'a T) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &[u8]) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`

##### `impl PartialOrd<'a, T: ?Sized>`

- `fn partial_cmp(self: &Self, other: &&'a T) -> Option<cmp::Ordering>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderValue`

- `fn try_from(t: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderValue`

- `fn try_from(t: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidHeaderValue`

- `fn try_from(t: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidHeaderValue`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderValue`

- `fn try_from(s: &'a String) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `InvalidHeaderValue`

```rust
struct InvalidHeaderValue {
    // [REDACTED: Private Fields]
}
```

A possible error when converting a `HeaderValue` from a string or byte
slice.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ToStrError`

```rust
struct ToStrError {
    // [REDACTED: Private Fields]
}
```

A possible error when converting a `HeaderValue` to a string representation.

Header field values may contain opaque bytes, in which case it is not
possible to represent the value as a string.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `Entry<'a, T: 'a>`

```rust
enum Entry<'a, T: 'a> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}
```

A view into a single location in a `HeaderMap`, which may be vacant or occupied.

#### Variants

- **`Occupied`**

  An occupied entry

- **`Vacant`**

  A vacant entry

#### Implementations

- `fn or_insert(self: Self, default: T) -> &'a mut T`
  Ensures a value is in the entry by inserting the default if empty.

- `fn or_try_insert(self: Self, default: T) -> Result<&'a mut T, MaxSizeReached>`
  Ensures a value is in the entry by inserting the default if empty.

- `fn or_insert_with<F: FnOnce() -> T>(self: Self, default: F) -> &'a mut T`
  Ensures a value is in the entry by inserting the result of the default

- `fn or_try_insert_with<F: FnOnce() -> T>(self: Self, default: F) -> Result<&'a mut T, MaxSizeReached>`
  Ensures a value is in the entry by inserting the result of the default

- `fn key(self: &Self) -> &HeaderName`
  Returns a reference to the entry's key

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug + 'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

## Constants

