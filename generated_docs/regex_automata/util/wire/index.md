*[regex_automata](../../index.md) / [util](../index.md) / [wire](index.md)*

---

# Module `wire`

Types and routines that support the wire format of finite automata.

Currently, this module just exports a few error types and some small helpers
for deserializing [dense DFAs](crate::dfa::dense::DFA) using correct alignment.

## Contents

- [Structs](#structs)
  - [`AlignAs`](#alignas)
  - [`SerializeError`](#serializeerror)
  - [`DeserializeError`](#deserializeerror)
- [Enums](#enums)
  - [`DeserializeErrorKind`](#deserializeerrorkind)
  - [`LE`](#le)
  - [`BE`](#be)
- [Traits](#traits)
  - [`Endian`](#endian)
- [Functions](#functions)
  - [`u32s_to_state_ids`](#u32s_to_state_ids)
  - [`u32s_to_state_ids_mut`](#u32s_to_state_ids_mut)
  - [`u32s_to_pattern_ids`](#u32s_to_pattern_ids)
  - [`check_alignment`](#check_alignment)
  - [`skip_initial_padding`](#skip_initial_padding)
  - [`alloc_aligned_buffer`](#alloc_aligned_buffer)
  - [`read_label`](#read_label)
  - [`write_label`](#write_label)
  - [`write_label_len`](#write_label_len)
  - [`read_endianness_check`](#read_endianness_check)
  - [`write_endianness_check`](#write_endianness_check)
  - [`write_endianness_check_len`](#write_endianness_check_len)
  - [`read_version`](#read_version)
  - [`write_version`](#write_version)
  - [`write_version_len`](#write_version_len)
  - [`read_pattern_id`](#read_pattern_id)
  - [`read_pattern_id_unchecked`](#read_pattern_id_unchecked)
  - [`write_pattern_id`](#write_pattern_id)
  - [`try_read_state_id`](#try_read_state_id)
  - [`read_state_id`](#read_state_id)
  - [`read_state_id_unchecked`](#read_state_id_unchecked)
  - [`write_state_id`](#write_state_id)
  - [`try_read_u16_as_usize`](#try_read_u16_as_usize)
  - [`try_read_u32_as_usize`](#try_read_u32_as_usize)
  - [`try_read_u16`](#try_read_u16)
  - [`try_read_u32`](#try_read_u32)
  - [`try_read_u128`](#try_read_u128)
  - [`read_u16`](#read_u16)
  - [`read_u32`](#read_u32)
  - [`read_u128`](#read_u128)
  - [`check_slice_len`](#check_slice_len)
  - [`mul`](#mul)
  - [`add`](#add)
  - [`shl`](#shl)
  - [`padding_len`](#padding_len)
- [Type Aliases](#type-aliases)
  - [`NE`](#ne)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AlignAs`](#alignas) | struct | A hack to align a smaller type `B` with a bigger type `T`. |
| [`SerializeError`](#serializeerror) | struct | An error that occurs when serializing an object from this crate. |
| [`DeserializeError`](#deserializeerror) | struct | An error that occurs when deserializing an object defined in this crate. |
| [`DeserializeErrorKind`](#deserializeerrorkind) | enum |  |
| [`LE`](#le) | enum | Little endian writing. |
| [`BE`](#be) | enum | Big endian writing. |
| [`Endian`](#endian) | trait | A simple trait for writing code generic over endianness. |
| [`u32s_to_state_ids`](#u32s_to_state_ids) | fn | Safely converts a `&[u32]` to `&[StateID]` with zero cost. |
| [`u32s_to_state_ids_mut`](#u32s_to_state_ids_mut) | fn | Safely converts a `&mut [u32]` to `&mut [StateID]` with zero cost. |
| [`u32s_to_pattern_ids`](#u32s_to_pattern_ids) | fn | Safely converts a `&[u32]` to `&[PatternID]` with zero cost. |
| [`check_alignment`](#check_alignment) | fn | Checks that the given slice has an alignment that matches `T`. |
| [`skip_initial_padding`](#skip_initial_padding) | fn | Reads a possibly empty amount of padding, up to 7 bytes, from the beginning of the given slice. |
| [`alloc_aligned_buffer`](#alloc_aligned_buffer) | fn | Allocate a byte buffer of the given size, along with some initial padding such that `buf[padding..]` has the same alignment as `T`, where the alignment of `T` must be at most `8`. |
| [`read_label`](#read_label) | fn | Reads a NUL terminated label starting at the beginning of the given slice. |
| [`write_label`](#write_label) | fn | Writes the given label to the buffer as a NUL terminated string. |
| [`write_label_len`](#write_label_len) | fn | Returns the total number of bytes (including padding) that would be written for the given label. |
| [`read_endianness_check`](#read_endianness_check) | fn | Reads the endianness check from the beginning of the given slice and confirms that the endianness of the serialized object matches the expected endianness. |
| [`write_endianness_check`](#write_endianness_check) | fn | Writes 0xFEFF as an integer using the given endianness. |
| [`write_endianness_check_len`](#write_endianness_check_len) | fn | Returns the number of bytes written by the endianness check. |
| [`read_version`](#read_version) | fn | Reads a version number from the beginning of the given slice and confirms that is matches the expected version number given. |
| [`write_version`](#write_version) | fn | Writes the given version number to the beginning of the given slice. |
| [`write_version_len`](#write_version_len) | fn | Returns the number of bytes written by writing the version number. |
| [`read_pattern_id`](#read_pattern_id) | fn | Reads a pattern ID from the given slice. |
| [`read_pattern_id_unchecked`](#read_pattern_id_unchecked) | fn | Reads a pattern ID from the given slice. |
| [`write_pattern_id`](#write_pattern_id) | fn | Write the given pattern ID to the beginning of the given slice of bytes using the specified endianness. |
| [`try_read_state_id`](#try_read_state_id) | fn | Attempts to read a state ID from the given slice. |
| [`read_state_id`](#read_state_id) | fn | Reads a state ID from the given slice. |
| [`read_state_id_unchecked`](#read_state_id_unchecked) | fn | Reads a state ID from the given slice. |
| [`write_state_id`](#write_state_id) | fn | Write the given state ID to the beginning of the given slice of bytes using the specified endianness. |
| [`try_read_u16_as_usize`](#try_read_u16_as_usize) | fn | Try to read a u16 as a usize from the beginning of the given slice in native endian format. |
| [`try_read_u32_as_usize`](#try_read_u32_as_usize) | fn | Try to read a u32 as a usize from the beginning of the given slice in native endian format. |
| [`try_read_u16`](#try_read_u16) | fn | Try to read a u16 from the beginning of the given slice in native endian format. |
| [`try_read_u32`](#try_read_u32) | fn | Try to read a u32 from the beginning of the given slice in native endian format. |
| [`try_read_u128`](#try_read_u128) | fn | Try to read a u128 from the beginning of the given slice in native endian format. |
| [`read_u16`](#read_u16) | fn | Read a u16 from the beginning of the given slice in native endian format. |
| [`read_u32`](#read_u32) | fn | Read a u32 from the beginning of the given slice in native endian format. |
| [`read_u128`](#read_u128) | fn | Read a u128 from the beginning of the given slice in native endian format. |
| [`check_slice_len`](#check_slice_len) | fn | Checks that the given slice has some minimal length. |
| [`mul`](#mul) | fn | Multiply the given numbers, and on overflow, return an error that includes 'what' in the error message. |
| [`add`](#add) | fn | Add the given numbers, and on overflow, return an error that includes 'what' in the error message. |
| [`shl`](#shl) | fn | Shift `a` left by `b`, and on overflow, return an error that includes 'what' in the error message. |
| [`padding_len`](#padding_len) | fn | Returns the number of additional bytes required to add to the given length in order to make the total length a multiple of 4. |
| [`NE`](#ne) | type |  |

## Structs

### `AlignAs<B: ?Sized, T>`

```rust
struct AlignAs<B: ?Sized, T> {
    pub _align: [T; 0],
    pub bytes: B,
}
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:66-71`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L66-L71)*

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

##### `impl<B: fmt::Debug + ?Sized, T: fmt::Debug> Debug for AlignAs<B, T>`

- <span id="alignas-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SerializeError`

```rust
struct SerializeError {
    what: &'static str,
}
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:88-105`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L88-L105)*

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

- <span id="serializeerror-buffer-too-small"></span>`fn buffer_too_small(what: &'static str) -> SerializeError` — [`SerializeError`](#serializeerror)

#### Trait Implementations

##### `impl Debug for SerializeError`

- <span id="serializeerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SerializeError`

- <span id="serializeerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for SerializeError`

##### `impl ToString for SerializeError`

- <span id="serializeerror-to-string"></span>`fn to_string(&self) -> String`

### `DeserializeError`

```rust
struct DeserializeError(DeserializeErrorKind);
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:138`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L138)*

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

- <span id="deserializeerror-generic"></span>`fn generic(msg: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-buffer-too-small"></span>`fn buffer_too_small(what: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-invalid-usize"></span>`fn invalid_usize(what: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-version-mismatch"></span>`fn version_mismatch(expected: u32, found: u32) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-endian-mismatch"></span>`fn endian_mismatch(expected: u32, found: u32) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-alignment-mismatch"></span>`fn alignment_mismatch(alignment: usize, address: usize) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-label-mismatch"></span>`fn label_mismatch(expected: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-arithmetic-overflow"></span>`fn arithmetic_overflow(what: &'static str) -> DeserializeError` — [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-pattern-id-error"></span>`fn pattern_id_error(err: PatternIDError, what: &'static str) -> DeserializeError` — [`PatternIDError`](../primitives/index.md#patterniderror), [`DeserializeError`](#deserializeerror)

- <span id="deserializeerror-state-id-error"></span>`fn state_id_error(err: StateIDError, what: &'static str) -> DeserializeError` — [`StateIDError`](../primitives/index.md#stateiderror), [`DeserializeError`](#deserializeerror)

#### Trait Implementations

##### `impl Debug for DeserializeError`

- <span id="deserializeerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DeserializeError`

- <span id="deserializeerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for DeserializeError`

##### `impl ToString for DeserializeError`

- <span id="deserializeerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `DeserializeErrorKind`

```rust
enum DeserializeErrorKind {
    Generic {
        msg: &'static str,
    },
    BufferTooSmall {
        what: &'static str,
    },
    InvalidUsize {
        what: &'static str,
    },
    VersionMismatch {
        expected: u32,
        found: u32,
    },
    EndianMismatch {
        expected: u32,
        found: u32,
    },
    AlignmentMismatch {
        alignment: usize,
        address: usize,
    },
    LabelMismatch {
        expected: &'static str,
    },
    ArithmeticOverflow {
        what: &'static str,
    },
    PatternID {
        err: crate::util::primitives::PatternIDError,
        what: &'static str,
    },
    StateID {
        err: crate::util::primitives::StateIDError,
        what: &'static str,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:141-152`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L141-L152)*

#### Trait Implementations

##### `impl Debug for DeserializeErrorKind`

- <span id="deserializeerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LE`

```rust
enum LE {
}
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:862`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L862)*

Little endian writing.

#### Trait Implementations

##### `impl Endian for LE`

- <span id="le-write-u16"></span>`fn write_u16(n: u16, dst: &mut [u8])`

- <span id="le-write-u32"></span>`fn write_u32(n: u32, dst: &mut [u8])`

- <span id="le-write-u128"></span>`fn write_u128(n: u128, dst: &mut [u8])`

### `BE`

```rust
enum BE {
}
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:864`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L864)*

Big endian writing.

#### Trait Implementations

##### `impl Endian for BE`

- <span id="be-write-u16"></span>`fn write_u16(n: u16, dst: &mut [u8])`

- <span id="be-write-u32"></span>`fn write_u32(n: u32, dst: &mut [u8])`

- <span id="be-write-u128"></span>`fn write_u128(n: u128, dst: &mut [u8])`

## Traits

### `Endian`

```rust
trait Endian { ... }
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:844-859`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L844-L859)*

A simple trait for writing code generic over endianness.

This is similar to what byteorder provides, but we only need a very small
subset.

#### Required Methods

- `fn write_u16(n: u16, dst: &mut [u8])`

  Writes a u16 to the given destination buffer in a particular

- `fn write_u32(n: u32, dst: &mut [u8])`

  Writes a u32 to the given destination buffer in a particular

- `fn write_u128(n: u128, dst: &mut [u8])`

  Writes a u128 to the given destination buffer in a particular

#### Implementors

- [`BE`](#be)
- [`LE`](#le)

## Functions

### `u32s_to_state_ids`

```rust
fn u32s_to_state_ids(slice: &[u32]) -> &[crate::util::primitives::StateID]
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:266-278`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L266-L278)*

Safely converts a `&[u32]` to `&[StateID]` with zero cost.

### `u32s_to_state_ids_mut`

```rust
fn u32s_to_state_ids_mut(slice: &mut [u32]) -> &mut [crate::util::primitives::StateID]
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:281-293`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L281-L293)*

Safely converts a `&mut [u32]` to `&mut [StateID]` with zero cost.

### `u32s_to_pattern_ids`

```rust
fn u32s_to_pattern_ids(slice: &[u32]) -> &[crate::util::primitives::PatternID]
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:297-309`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L297-L309)*

Safely converts a `&[u32]` to `&[PatternID]` with zero cost.

### `check_alignment`

```rust
fn check_alignment<T>(slice: &[u8]) -> Result<(), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:316-325`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L316-L325)*

Checks that the given slice has an alignment that matches `T`.

This is useful for checking that a slice has an appropriate alignment
before casting it to a &[T]. Note though that alignment is not itself
sufficient to perform the cast for any `T`.

### `skip_initial_padding`

```rust
fn skip_initial_padding(slice: &[u8]) -> usize
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:336-342`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L336-L342)*

Reads a possibly empty amount of padding, up to 7 bytes, from the beginning
of the given slice. All padding bytes must be NUL bytes.

This is useful because it can be theoretically necessary to pad the
beginning of a serialized object with NUL bytes to ensure that it starts
at a correctly aligned address. These padding bytes should come immediately
before the label.

This returns the number of bytes read from the given slice.

### `alloc_aligned_buffer`

```rust
fn alloc_aligned_buffer<T>(size: usize) -> (alloc::vec::Vec<u8>, usize)
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:361-404`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L361-L404)*

Allocate a byte buffer of the given size, along with some initial padding
such that `buf[padding..]` has the same alignment as `T`, where the
alignment of `T` must be at most `8`. In particular, callers should treat
the first N bytes (second return value) as padding bytes that must not be
overwritten. In all cases, the following identity holds:

```ignore
let (buf, padding) = alloc_aligned_buffer::<StateID>(SIZE);
assert_eq!(SIZE, buf[padding..].len());
```

In practice, padding is often zero.

The requirement for `8` as a maximum here is somewhat arbitrary. In
practice, we never need anything bigger in this crate, and so this function
does some sanity asserts under the assumption of a max alignment of `8`.

### `read_label`

```rust
fn read_label(slice: &[u8], expected_label: &'static str) -> Result<usize, DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:414-442`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L414-L442)*

Reads a NUL terminated label starting at the beginning of the given slice.

If a NUL terminated label could not be found, then an error is returned.
Similarly, if a label is found but doesn't match the expected label, then
an error is returned.

Upon success, the total number of bytes read (including padding bytes) is
returned.

### `write_label`

```rust
fn write_label(label: &str, dst: &mut [u8]) -> Result<usize, SerializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:453-467`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L453-L467)*

Writes the given label to the buffer as a NUL terminated string. The label
given must not contain NUL, otherwise this will panic. Similarly, the label
must not be longer than 255 bytes, otherwise this will panic.

Additional NUL bytes are written as necessary to ensure that the number of
bytes written is always a multiple of 4.

Upon success, the total number of bytes written (including padding) is
returned.

### `write_label_len`

```rust
fn write_label_len(label: &str) -> usize
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:473-478`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L473-L478)*

Returns the total number of bytes (including padding) that would be written
for the given label. This panics if the given label contains a NUL byte or
is longer than 255 bytes. (The size restriction exists so that searching
for a label during deserialization can be done in small bounded space.)

### `read_endianness_check`

```rust
fn read_endianness_check(slice: &[u8]) -> Result<usize, DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:486-495`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L486-L495)*

Reads the endianness check from the beginning of the given slice and
confirms that the endianness of the serialized object matches the expected
endianness. If the slice is too small or if the endianness check fails,
this returns an error.

Upon success, the total number of bytes read is returned.

### `write_endianness_check`

```rust
fn write_endianness_check<E: Endian>(dst: &mut [u8]) -> Result<usize, SerializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:504-513`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L504-L513)*

Writes 0xFEFF as an integer using the given endianness.

This is useful for writing into the header of a serialized object. It can
be read during deserialization as a sanity check to ensure the proper
endianness is used.

Upon success, the total number of bytes written is returned.

### `write_endianness_check_len`

```rust
fn write_endianness_check_len() -> usize
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:516-518`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L516-L518)*

Returns the number of bytes written by the endianness check.

### `read_version`

```rust
fn read_version(slice: &[u8], expected_version: u32) -> Result<usize, DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:529-539`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L529-L539)*

Reads a version number from the beginning of the given slice and confirms
that is matches the expected version number given. If the slice is too
small or if the version numbers aren't equivalent, this returns an error.

Upon success, the total number of bytes read is returned.

N.B. Currently, we require that the version number is exactly equivalent.
In the future, if we bump the version number without a semver bump, then
we'll need to relax this a bit and support older versions.

### `write_version`

```rust
fn write_version<E: Endian>(version: u32, dst: &mut [u8]) -> Result<usize, SerializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:548-558`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L548-L558)*

Writes the given version number to the beginning of the given slice.

This is useful for writing into the header of a serialized object. It can
be read during deserialization as a sanity check to ensure that the library
code supports the format of the serialized object.

Upon success, the total number of bytes written is returned.

### `write_version_len`

```rust
fn write_version_len() -> usize
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:561-563`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L561-L563)*

Returns the number of bytes written by writing the version number.

### `read_pattern_id`

```rust
fn read_pattern_id(slice: &[u8], what: &'static str) -> Result<(crate::util::primitives::PatternID, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:570-579`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L570-L579)*

Reads a pattern ID from the given slice. If the slice has insufficient
length, then this panics. If the deserialized integer exceeds the pattern
ID limit for the current target, then this returns an error.

Upon success, this also returns the number of bytes read.

### `read_pattern_id_unchecked`

```rust
fn read_pattern_id_unchecked(slice: &[u8]) -> (crate::util::primitives::PatternID, usize)
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:586-591`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L586-L591)*

Reads a pattern ID from the given slice. If the slice has insufficient
length, then this panics. Otherwise, the deserialized integer is assumed
to be a valid pattern ID.

This also returns the number of bytes read.

### `write_pattern_id`

```rust
fn write_pattern_id<E: Endian>(pid: crate::util::primitives::PatternID, dst: &mut [u8]) -> usize
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:597-603`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L597-L603)*

Write the given pattern ID to the beginning of the given slice of bytes
using the specified endianness. The given slice must have length at least
`PatternID::SIZE`, or else this panics. Upon success, the total number of
bytes written is returned.

### `try_read_state_id`

```rust
fn try_read_state_id(slice: &[u8], what: &'static str) -> Result<(crate::util::primitives::StateID, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:610-618`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L610-L618)*

Attempts to read a state ID from the given slice. If the slice has an
insufficient number of bytes or if the state ID exceeds the limit for
the current target, then this returns an error.

Upon success, this also returns the number of bytes read.

### `read_state_id`

```rust
fn read_state_id(slice: &[u8], what: &'static str) -> Result<(crate::util::primitives::StateID, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:625-634`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L625-L634)*

Reads a state ID from the given slice. If the slice has insufficient
length, then this panics. If the deserialized integer exceeds the state ID
limit for the current target, then this returns an error.

Upon success, this also returns the number of bytes read.

### `read_state_id_unchecked`

```rust
fn read_state_id_unchecked(slice: &[u8]) -> (crate::util::primitives::StateID, usize)
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:641-646`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L641-L646)*

Reads a state ID from the given slice. If the slice has insufficient
length, then this panics. Otherwise, the deserialized integer is assumed
to be a valid state ID.

This also returns the number of bytes read.

### `write_state_id`

```rust
fn write_state_id<E: Endian>(sid: crate::util::primitives::StateID, dst: &mut [u8]) -> usize
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:652-658`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L652-L658)*

Write the given state ID to the beginning of the given slice of bytes
using the specified endianness. The given slice must have length at least
`StateID::SIZE`, or else this panics. Upon success, the total number of
bytes written is returned.

### `try_read_u16_as_usize`

```rust
fn try_read_u16_as_usize(slice: &[u8], what: &'static str) -> Result<(usize, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:668-677`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L668-L677)*

Try to read a u16 as a usize from the beginning of the given slice in
native endian format. If the slice has fewer than 2 bytes or if the
deserialized number cannot be represented by usize, then this returns an
error. The error message will include the `what` description of what is
being deserialized, for better error messages. `what` should be a noun in
singular form.

Upon success, this also returns the number of bytes read.

### `try_read_u32_as_usize`

```rust
fn try_read_u32_as_usize(slice: &[u8], what: &'static str) -> Result<(usize, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:687-696`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L687-L696)*

Try to read a u32 as a usize from the beginning of the given slice in
native endian format. If the slice has fewer than 4 bytes or if the
deserialized number cannot be represented by usize, then this returns an
error. The error message will include the `what` description of what is
being deserialized, for better error messages. `what` should be a noun in
singular form.

Upon success, this also returns the number of bytes read.

### `try_read_u16`

```rust
fn try_read_u16(slice: &[u8], what: &'static str) -> Result<(u16, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:705-711`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L705-L711)*

Try to read a u16 from the beginning of the given slice in native endian
format. If the slice has fewer than 2 bytes, then this returns an error.
The error message will include the `what` description of what is being
deserialized, for better error messages. `what` should be a noun in
singular form.

Upon success, this also returns the number of bytes read.

### `try_read_u32`

```rust
fn try_read_u32(slice: &[u8], what: &'static str) -> Result<(u32, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:720-726`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L720-L726)*

Try to read a u32 from the beginning of the given slice in native endian
format. If the slice has fewer than 4 bytes, then this returns an error.
The error message will include the `what` description of what is being
deserialized, for better error messages. `what` should be a noun in
singular form.

Upon success, this also returns the number of bytes read.

### `try_read_u128`

```rust
fn try_read_u128(slice: &[u8], what: &'static str) -> Result<(u128, usize), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:735-741`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L735-L741)*

Try to read a u128 from the beginning of the given slice in native endian
format. If the slice has fewer than 16 bytes, then this returns an error.
The error message will include the `what` description of what is being
deserialized, for better error messages. `what` should be a noun in
singular form.

Upon success, this also returns the number of bytes read.

### `read_u16`

```rust
fn read_u16(slice: &[u8]) -> u16
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:749-752`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L749-L752)*

Read a u16 from the beginning of the given slice in native endian format.
If the slice has fewer than 2 bytes, then this panics.

Marked as inline to speed up sparse searching which decodes integers from
its automaton at search time.

### `read_u32`

```rust
fn read_u32(slice: &[u8]) -> u32
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:760-763`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L760-L763)*

Read a u32 from the beginning of the given slice in native endian format.
If the slice has fewer than 4 bytes, then this panics.

Marked as inline to speed up sparse searching which decodes integers from
its automaton at search time.

### `read_u128`

```rust
fn read_u128(slice: &[u8]) -> u128
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:767-770`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L767-L770)*

Read a u128 from the beginning of the given slice in native endian format.
If the slice has fewer than 16 bytes, then this panics.

### `check_slice_len`

```rust
fn check_slice_len<T>(slice: &[T], at_least_len: usize, what: &'static str) -> Result<(), DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:775-784`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L775-L784)*

Checks that the given slice has some minimal length. If it's smaller than
the bound given, then a "buffer too small" error is returned with `what`
describing what the buffer represents.

### `mul`

```rust
fn mul(a: usize, b: usize, what: &'static str) -> Result<usize, DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:790-799`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L790-L799)*

Multiply the given numbers, and on overflow, return an error that includes
'what' in the error message.

This is useful when doing arithmetic with untrusted data.

### `add`

```rust
fn add(a: usize, b: usize, what: &'static str) -> Result<usize, DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:805-814`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L805-L814)*

Add the given numbers, and on overflow, return an error that includes
'what' in the error message.

This is useful when doing arithmetic with untrusted data.

### `shl`

```rust
fn shl(a: usize, b: usize, what: &'static str) -> Result<usize, DeserializeError>
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:820-831`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L820-L831)*

Shift `a` left by `b`, and on overflow, return an error that includes
'what' in the error message.

This is useful when doing arithmetic with untrusted data.

### `padding_len`

```rust
fn padding_len(non_padding_len: usize) -> usize
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:836-838`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L836-L838)*

Returns the number of additional bytes required to add to the given length
in order to make the total length a multiple of 4. The return value is
always less than 4.

## Type Aliases

### `NE`

```rust
type NE = LE;
```

*Defined in [`regex-automata-0.4.13/src/util/wire.rs:867`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/wire.rs#L867)*

