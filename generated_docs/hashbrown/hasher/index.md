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

##### `impl BuildHasher for DefaultHashBuilder`

- <span id="defaulthashbuilder-buildhasher-type-hasher"></span>`type Hasher = DefaultHasher`

- <span id="defaulthashbuilder-build-hasher"></span>`fn build_hasher(&self) -> <Self as >::Hasher`

##### `impl Clone for DefaultHashBuilder`

- <span id="defaulthashbuilder-clone"></span>`fn clone(&self) -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

##### `impl Debug for DefaultHashBuilder`

- <span id="defaulthashbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DefaultHashBuilder`

- <span id="defaulthashbuilder-default"></span>`fn default() -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

### `DefaultHasher`

```rust
struct DefaultHasher {
    inner: <foldhash::fast::RandomState as BuildHasher>::Hasher,
}
```

*Defined in [`hashbrown-0.16.1/src/hasher.rs:34-36`](../../../.source_1765521767/hashbrown-0.16.1/src/hasher.rs#L34-L36)*

Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

#### Trait Implementations

##### `impl Clone for DefaultHasher`

- <span id="defaulthasher-clone"></span>`fn clone(&self) -> DefaultHasher` — [`DefaultHasher`](#defaulthasher)

##### `impl Hasher for DefaultHasher`

- <span id="defaulthasher-write"></span>`fn write(&mut self, arg: &[u8])`

- <span id="defaulthasher-write-u8"></span>`fn write_u8(&mut self, arg: u8)`

- <span id="defaulthasher-write-u16"></span>`fn write_u16(&mut self, arg: u16)`

- <span id="defaulthasher-write-u32"></span>`fn write_u32(&mut self, arg: u32)`

- <span id="defaulthasher-write-u64"></span>`fn write_u64(&mut self, arg: u64)`

- <span id="defaulthasher-write-u128"></span>`fn write_u128(&mut self, arg: u128)`

- <span id="defaulthasher-write-usize"></span>`fn write_usize(&mut self, arg: usize)`

- <span id="defaulthasher-write-i8"></span>`fn write_i8(&mut self, arg: i8)`

- <span id="defaulthasher-write-i16"></span>`fn write_i16(&mut self, arg: i16)`

- <span id="defaulthasher-write-i32"></span>`fn write_i32(&mut self, arg: i32)`

- <span id="defaulthasher-write-i64"></span>`fn write_i64(&mut self, arg: i64)`

- <span id="defaulthasher-write-i128"></span>`fn write_i128(&mut self, arg: i128)`

- <span id="defaulthasher-write-isize"></span>`fn write_isize(&mut self, arg: isize)`

- <span id="defaulthasher-finish"></span>`fn finish(&self) -> u64`

## Macros

### `forward_writes!`

*Defined in [`hashbrown-0.16.1/src/hasher.rs:39-46`](../../../.source_1765521767/hashbrown-0.16.1/src/hasher.rs#L39-L46)*

