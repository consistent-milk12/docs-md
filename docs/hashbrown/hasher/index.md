*[hashbrown](../index.md) / [hasher](index.md)*

---

# Module `hasher`

## Structs

### `DefaultHashBuilder`

```rust
struct DefaultHashBuilder {
    inner: foldhash::fast::RandomState,
}
```

Default hash builder for the `S` type parameter of
[`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

This only implements `BuildHasher` when the "default-hasher" crate feature
is enabled; otherwise it just serves as a placeholder, and a custom `S` type
must be used to have a fully functional `HashMap` or `HashSet`.

#### Trait Implementations

##### `impl BuildHasher for DefaultHashBuilder`

- `type Hasher = DefaultHasher`

- `fn build_hasher(self: &Self) -> <Self as >::Hasher`

##### `impl Clone for DefaultHashBuilder`

- `fn clone(self: &Self) -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

##### `impl Debug for DefaultHashBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DefaultHashBuilder`

- `fn default() -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

### `DefaultHasher`

```rust
struct DefaultHasher {
    inner: <foldhash::fast::RandomState as BuildHasher>::Hasher,
}
```

Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

#### Trait Implementations

##### `impl Clone for DefaultHasher`

- `fn clone(self: &Self) -> DefaultHasher` — [`DefaultHasher`](#defaulthasher)

##### `impl Hasher for DefaultHasher`

- `fn write(self: &mut Self, arg: &[u8])`

- `fn write_u8(self: &mut Self, arg: u8)`

- `fn write_u16(self: &mut Self, arg: u16)`

- `fn write_u32(self: &mut Self, arg: u32)`

- `fn write_u64(self: &mut Self, arg: u64)`

- `fn write_u128(self: &mut Self, arg: u128)`

- `fn write_usize(self: &mut Self, arg: usize)`

- `fn write_i8(self: &mut Self, arg: i8)`

- `fn write_i16(self: &mut Self, arg: i16)`

- `fn write_i32(self: &mut Self, arg: i32)`

- `fn write_i64(self: &mut Self, arg: i64)`

- `fn write_i128(self: &mut Self, arg: i128)`

- `fn write_isize(self: &mut Self, arg: isize)`

- `fn finish(self: &Self) -> u64`

## Macros

### `forward_writes!`

