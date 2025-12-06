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

The usual use of this is with `B = [u8]` and `T = u32`. That is,
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

##### `impl<B: $crate::fmt::Debug + ?Sized, T: $crate::fmt::Debug> Debug for AlignAs<B, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SerializeError`

```rust
struct SerializeError {
    what: &'static str,
}
```

An error that occurs when serializing an object from this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8]`. To this end, serialization is generally infallible.
However, it can fail when caller provided buffer sizes are too small. When
that occurs, a serialization error is reported.

A `SerializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

#### Fields

- **`what`**: `&'static str`

  The name of the thing that a buffer is too small for.
  
  Currently, the only kind of serialization error is one that is
  committed by a caller: providing a destination buffer that is too
  small to fit the serialized object. This makes sense conceptually,
  since every valid inhabitant of a type should be serializable.
  
  This is somewhat exposed in the public API of this crate. For example,
  the `to_bytes_{big,little}_endian` APIs return a `Vec<u8>` and are
  guaranteed to never panic or error. This is only possible because the
  implementation guarantees that it will allocate a `Vec<u8>` that is
  big enough.
  
  In summary, if a new serialization error kind needs to be added, then
  it will need careful consideration.

#### Implementations

- `fn buffer_too_small(what: &'static str) -> SerializeError` — [`SerializeError`](#serializeerror)

#### Trait Implementations

##### `impl Debug for SerializeError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SerializeError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for SerializeError`

##### `impl<T> ToString for SerializeError`

- `fn to_string(self: &Self) -> String`

### `DeserializeError`

```rust
struct DeserializeError(DeserializeErrorKind);
```

An error that occurs when deserializing an object defined in this crate.

Serialization, as used in this crate, universally refers to the process
of transforming a structure (like a DFA) into a custom binary format
represented by `&[u8]`. Deserialization, then, refers to the process of
cheaply converting this binary format back to the object's in-memory
representation as defined in this crate. To the extent possible,
deserialization will report this error whenever this process fails.

A `DeserializeError` provides no introspection capabilities. Its only
supported operation is conversion to a human readable error message.

This error type implements the `std::error::Error` trait only when the
`std` feature is enabled. Otherwise, this type is defined in all
configurations.

#### Implementations

- `fn generic(msg: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn buffer_too_small(what: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn invalid_usize(what: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn version_mismatch(expected: u32, found: u32) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn endian_mismatch(expected: u32, found: u32) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn alignment_mismatch(alignment: usize, address: usize) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn label_mismatch(expected: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn arithmetic_overflow(what: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- `fn pattern_id_error(err: PatternIDError, what: &'static str) -> DeserializeError` — [`PatternIDError`](../primitives/index.md), [`DeserializeError`](#deserializeerror)

- `fn state_id_error(err: StateIDError, what: &'static str) -> DeserializeError` — [`StateIDError`](../primitives/index.md), [`DeserializeError`](#deserializeerror)

#### Trait Implementations

##### `impl Debug for DeserializeError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DeserializeError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DeserializeError`

##### `impl<T> ToString for DeserializeError`

- `fn to_string(self: &Self) -> String`

