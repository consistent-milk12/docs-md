*[hashbrown](../index.md) / [hasher](index.md)*

---

# Module `hasher`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DefaultHashBuilder`](#defaulthashbuilder) | struct | Default hash builder for the `S` type parameter of [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet). |
| [`DefaultHasher`](#defaulthasher) | struct | Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet). |
| [`forward_writes!`](#forward-writes) | macro |  |

## Structs

### `DefaultHashBuilder`

```rust
struct DefaultHashBuilder {
    inner: foldhash::fast::RandomState,
}
```

*Defined in [`hashbrown-0.16.1/src/hasher.rs:14-17`](../../../.source_1765521767/hashbrown-0.16.1/src/hasher.rs#L14-L17)*

Default hash builder for the `S` type parameter of
[`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

This only implements `BuildHasher` when the "default-hasher" crate feature
is enabled; otherwise it just serves as a placeholder, and a custom `S` type
must be used to have a fully functional `HashMap` or `HashSet`.

#### Trait Implementations

##### `impl Any for DefaultHashBuilder`

- <span id="defaulthashbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DefaultHashBuilder`

- <span id="defaulthashbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DefaultHashBuilder`

- <span id="defaulthashbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl BuildHasher for DefaultHashBuilder`

- <span id="defaulthashbuilder-buildhasher-type-hasher"></span>`type Hasher = DefaultHasher`

- <span id="defaulthashbuilder-buildhasher-build-hasher"></span>`fn build_hasher(&self) -> <Self as >::Hasher`

##### `impl Clone for DefaultHashBuilder`

- <span id="defaulthashbuilder-clone"></span>`fn clone(&self) -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

##### `impl CloneToUninit for DefaultHashBuilder`

- <span id="defaulthashbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DefaultHashBuilder`

- <span id="defaulthashbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DefaultHashBuilder`

- <span id="defaulthashbuilder-default"></span>`fn default() -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

##### `impl<T> From for DefaultHashBuilder`

- <span id="defaulthashbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DefaultHashBuilder`

- <span id="defaulthashbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DefaultHashBuilder`

- <span id="defaulthashbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="defaulthashbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="defaulthashbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DefaultHashBuilder`

- <span id="defaulthashbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="defaulthashbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DefaultHashBuilder`

- <span id="defaulthashbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="defaulthashbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DefaultHasher`

```rust
struct DefaultHasher {
    inner: <foldhash::fast::RandomState as BuildHasher>::Hasher,
}
```

*Defined in [`hashbrown-0.16.1/src/hasher.rs:34-36`](../../../.source_1765521767/hashbrown-0.16.1/src/hasher.rs#L34-L36)*

Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

#### Trait Implementations

##### `impl Any for DefaultHasher`

- <span id="defaulthasher-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DefaultHasher`

- <span id="defaulthasher-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DefaultHasher`

- <span id="defaulthasher-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DefaultHasher`

- <span id="defaulthasher-clone"></span>`fn clone(&self) -> DefaultHasher` — [`DefaultHasher`](#defaulthasher)

##### `impl CloneToUninit for DefaultHasher`

- <span id="defaulthasher-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for DefaultHasher`

- <span id="defaulthasher-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hasher for DefaultHasher`

- <span id="defaulthasher-hasher-write"></span>`fn write(&mut self, arg: &[u8])`

- <span id="defaulthasher-hasher-write-u8"></span>`fn write_u8(&mut self, arg: u8)`

- <span id="defaulthasher-hasher-write-u16"></span>`fn write_u16(&mut self, arg: u16)`

- <span id="defaulthasher-hasher-write-u32"></span>`fn write_u32(&mut self, arg: u32)`

- <span id="defaulthasher-hasher-write-u64"></span>`fn write_u64(&mut self, arg: u64)`

- <span id="defaulthasher-hasher-write-u128"></span>`fn write_u128(&mut self, arg: u128)`

- <span id="defaulthasher-hasher-write-usize"></span>`fn write_usize(&mut self, arg: usize)`

- <span id="defaulthasher-hasher-write-i8"></span>`fn write_i8(&mut self, arg: i8)`

- <span id="defaulthasher-hasher-write-i16"></span>`fn write_i16(&mut self, arg: i16)`

- <span id="defaulthasher-hasher-write-i32"></span>`fn write_i32(&mut self, arg: i32)`

- <span id="defaulthasher-hasher-write-i64"></span>`fn write_i64(&mut self, arg: i64)`

- <span id="defaulthasher-hasher-write-i128"></span>`fn write_i128(&mut self, arg: i128)`

- <span id="defaulthasher-hasher-write-isize"></span>`fn write_isize(&mut self, arg: isize)`

- <span id="defaulthasher-hasher-finish"></span>`fn finish(&self) -> u64`

##### `impl<U> Into for DefaultHasher`

- <span id="defaulthasher-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DefaultHasher`

- <span id="defaulthasher-toowned-type-owned"></span>`type Owned = T`

- <span id="defaulthasher-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="defaulthasher-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DefaultHasher`

- <span id="defaulthasher-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="defaulthasher-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DefaultHasher`

- <span id="defaulthasher-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="defaulthasher-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `forward_writes!`

*Defined in [`hashbrown-0.16.1/src/hasher.rs:39-46`](../../../.source_1765521767/hashbrown-0.16.1/src/hasher.rs#L39-L46)*

