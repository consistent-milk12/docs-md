*[regex_automata](../../index.md) / [util](../index.md) / [wire](index.md)*

---

# Module `wire`

Types and routines that support the wire format of finite automata.

Currently, this module just exports a few error types and some small helpers
for deserializing [dense DFAs](crate::dfa::dense::DFA) using correct alignment.

## Structs

### `AlignAs<B: ?Sized, T>`

```rust
struct AlignAs<B: ?Sized, T> {
    pub _align: [T; 0],
    pub bytes: B,
}
```

A hack to align a smaller type `B` with a bigger type `T`.

The usual use of this is with `B = [u8](#u8)
` and `T = u32`. That is,
it permits aligning a sequence of bytes on a 4-byte boundary. This
is useful in contexts where one wants to embed a serialized [dense
DFA](crate::dfa::dense::DFA) into a Rust a program while guaranteeing the
alignment required for the DFA.

See [`dense::DFA::from_bytes`](crate::dfa::dense::DFA::from_bytes) for an
example of how to use this type.

#### Fields

- **`_align`**: `[T; 0]`

  A zero-sized field indicating the alignment we want.

- **`bytes`**: `B`

  A possibly non-sized field containing a sequence of bytes.

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

##### `impl Debug<B: $crate::fmt::Debug + ?Sized, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SerializeError`

```rust
struct SerializeError {
    // [REDACTED: Private Fields]
}
```

An error that occurs when serializing an object from this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8](#u8)
`. To this end, serialization is generally infallible.
However, it can fail when caller provided buffer sizes are too small. When
that occurs, a serialization error is reported.

A `SerializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

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

### `DeserializeError`

```rust
struct DeserializeError();
```

An error that occurs when deserializing an object defined in this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8](#u8)
`. Deserialization, then, refers to the process of
cheaply converting this binary format back to the object's in-memory
representation as defined in this crate. To the extent possible,
deserialization will report this error whenever this process fails.

A `DeserializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

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

