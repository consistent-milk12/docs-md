*[rustls](../../index.md) / [crypto](../index.md) / [hmac](index.md)*

---

# Module `hmac`

HMAC interfaces.

## Structs

### `Tag`

```rust
struct Tag {
    // [REDACTED: Private Fields]
}
```

A HMAC tag, stored as a value.

#### Implementations

- `fn new(bytes: &[u8]) -> Self`
  Build a tag by copying a byte slice.

- `const MAX_LEN: usize`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

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

- `fn clone(self: &Self) -> Tag`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Hmac`

```rust
trait Hmac: Send + Sync { ... }
```

A concrete HMAC implementation, for a single cryptographic hash function.

You should have one object that implements this trait for HMAC-SHA256, another
for HMAC-SHA384, etc.

#### Required Methods

- `fn with_key(self: &Self, key: &[u8]) -> Box<dyn Key>`

  Prepare to use `key` as a HMAC key.

- `fn hash_output_len(self: &Self) -> usize`

  Give the length of the underlying hash function.  In RFC2104 terminology this is `L`.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

### `Key`

```rust
trait Key: Send + Sync { ... }
```

A HMAC key that is ready for use.

The algorithm used is implicit in the `Hmac` object that produced the key.

#### Required Methods

- `fn sign(self: &Self, data: &[&[u8]]) -> Tag`

  Calculates a tag over `data` -- a slice of byte slices.

- `fn sign_concat(self: &Self, first: &[u8], middle: &[&[u8]], last: &[u8]) -> Tag`

  Calculates a tag over the concatenation of `first`, the items in `middle`, and `last`.

- `fn tag_len(self: &Self) -> usize`

  Returns the length of the tag returned by a computation using

