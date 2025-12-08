*[serde_core](../../index.md) / [de](../index.md) / [impls](index.md)*

---

# Module `impls`

## Modules

- [`range`](range/index.md) - 
- [`range_from`](range_from/index.md) - 
- [`range_to`](range_to/index.md) - 

## Structs

### `UnitVisitor`

```rust
struct UnitVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for UnitVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for UnitVisitor`

- `type Value = ()`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_unit<E>(self: Self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `BoolVisitor`

```rust
struct BoolVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for BoolVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for BoolVisitor`

- `type Value = bool`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_bool<E>(self: Self, v: bool) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `CharVisitor`

```rust
struct CharVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for CharVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for CharVisitor`

- `type Value = char`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_char<E>(self: Self, v: char) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_str<E>(self: Self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `StringVisitor`

```rust
struct StringVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for StringVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for StringVisitor`

- `type Value = String`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_str<E>(self: Self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_string<E>(self: Self, v: String) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_bytes<E>(self: Self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_byte_buf<E>(self: Self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `StringInPlaceVisitor<'a>`

```rust
struct StringInPlaceVisitor<'a>(&'a mut String);
```

#### Trait Implementations

##### `impl<'de, T> Expected for StringInPlaceVisitor<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, 'de> Visitor for StringInPlaceVisitor<'a>`

- `type Value = ()`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_str<E>(self: Self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_string<E>(self: Self, v: String) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_bytes<E>(self: Self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_byte_buf<E>(self: Self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `StrVisitor`

```rust
struct StrVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for StrVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Visitor for StrVisitor`

- `type Value = &'a str`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_borrowed_str<E>(self: Self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_borrowed_bytes<E>(self: Self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `BytesVisitor`

```rust
struct BytesVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for BytesVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Visitor for BytesVisitor`

- `type Value = &'a [u8]`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_borrowed_bytes<E>(self: Self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_borrowed_str<E>(self: Self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `CStringVisitor`

```rust
struct CStringVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for CStringVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for CStringVisitor`

- `type Value = CString`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

- `fn visit_bytes<E>(self: Self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_byte_buf<E>(self: Self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_str<E>(self: Self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_string<E>(self: Self, v: String) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `OptionVisitor<T>`

```rust
struct OptionVisitor<T> {
    marker: PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<'de, T> Expected for OptionVisitor<T>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for OptionVisitor<T>`

- `type Value = Option<T>`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_unit<E>(self: Self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_none<E>(self: Self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_some<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`Visitor`](../index.md)

### `PhantomDataVisitor<T: ?Sized>`

```rust
struct PhantomDataVisitor<T: ?Sized> {
    marker: PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<'de, T> Expected for PhantomDataVisitor<T>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for PhantomDataVisitor<T>`

- `type Value = PhantomData<T>`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_unit<E>(self: Self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `ArrayVisitor<A>`

```rust
struct ArrayVisitor<A> {
    marker: PhantomData<A>,
}
```

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl<'de, T> Expected for ArrayVisitor<A>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for ArrayVisitor<[T; 25]>`

- `type Value = [T; 25]`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

### `ArrayInPlaceVisitor<'a, A: 'a>`

```rust
struct ArrayInPlaceVisitor<'a, A: 'a>(&'a mut A);
```

#### Trait Implementations

##### `impl<'de, T> Expected for ArrayInPlaceVisitor<'a, A>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, 'de, T> Visitor for ArrayInPlaceVisitor<'a, [T; 29]>`

- `type Value = ()`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

### `PathVisitor`

```rust
struct PathVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for PathVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Visitor for PathVisitor`

- `type Value = &'a Path`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_borrowed_str<E>(self: Self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_borrowed_bytes<E>(self: Self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `PathBufVisitor`

```rust
struct PathBufVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for PathBufVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for PathBufVisitor`

- `type Value = PathBuf`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_str<E>(self: Self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_string<E>(self: Self, v: String) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_bytes<E>(self: Self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- `fn visit_byte_buf<E>(self: Self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `OsStringVisitor`

```rust
struct OsStringVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for OsStringVisitor`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for OsStringVisitor`

- `type Value = OsString`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_enum<A>(self: Self, data: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

### `FromStrVisitor<T>`

```rust
struct FromStrVisitor<T> {
    expecting: &'static str,
    ty: PhantomData<T>,
}
```

#### Implementations

- `fn new(expecting: &'static str) -> Self`

#### Trait Implementations

##### `impl<'de, T> Expected for FromStrVisitor<T>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for FromStrVisitor<T>`

- `type Value = T`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_str<E>(self: Self, s: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

## Enums

### `OsStringKind`

```rust
enum OsStringKind {
    Unix,
    Windows,
}
```

#### Trait Implementations

##### `impl<'de> Deserialize for OsStringKind`

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../index.md)

##### `impl<T> DeserializeOwned for OsStringKind`

## Functions

### `nop_reserve`

```rust
fn nop_reserve<T>(_seq: T, _n: usize)
```

## Macros

### `impl_deserialize_num!`

### `num_self!`

### `num_as_self!`

### `num_as_copysign_self!`

### `int_to_int!`

### `int_to_uint!`

### `uint_to_self!`

### `num_128!`

### `forwarded_impl!`

### `seq_impl!`

### `array_impls!`

### `tuple_impls!`

### `tuple_impl_body!`

### `map_impl!`

### `parse_ip_impl!`

### `variant_identifier!`

### `deserialize_enum!`

### `parse_socket_impl!`

### `box_forwarded_impl!`

### `atomic_impl!`

