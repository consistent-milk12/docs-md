*[miniz_oxide](../../index.md) / [inflate](../index.md) / [stream](index.md)*

---

# Module `stream`

Extra streaming decompression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.

## Structs

### `MinReset`

```rust
struct MinReset;
```

Resets state, without performing expensive ops (e.g. zeroing buffer)

Note that not zeroing buffer can lead to security issues when dealing with untrusted input.

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

##### `impl ResetPolicy`

- `fn reset(self: &Self, state: &mut InflateState)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ZeroReset`

```rust
struct ZeroReset;
```

Resets state and zero memory, continuing to use the same data format.

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

##### `impl ResetPolicy`

- `fn reset(self: &Self, state: &mut InflateState)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `FullReset`

```rust
struct FullReset(crate::DataFormat);
```

Full reset of the state, including zeroing memory.

Requires to provide new data format.

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

##### `impl ResetPolicy`

- `fn reset(self: &Self, state: &mut InflateState)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `InflateState`

```rust
struct InflateState {
    // [REDACTED: Private Fields]
}
```

A struct that compbines a decompressor with extra data for streaming decompression.


#### Implementations

- `fn new(data_format: DataFormat) -> InflateState`
  Create a new state.

- `fn new_boxed(data_format: DataFormat) -> Box<InflateState>`
  Create a new state on the heap.

- `fn decompressor(self: &mut Self) -> &mut DecompressorOxide`
  Access the innner decompressor.

- `const fn last_status(self: &Self) -> TINFLStatus`
  Return the status of the last call to `inflate` with this `InflateState`.

- `fn new_boxed_with_window_bits(window_bits: i32) -> Box<InflateState>`
  Create a new state using miniz/zlib style window bits parameter.

- `fn reset(self: &mut Self, data_format: DataFormat)`
  Reset the decompressor without re-allocating memory, using the given

- `fn reset_as<T: ResetPolicy>(self: &mut Self, policy: T)`
  Resets the state according to specified policy.

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

##### `impl Clone`

- `fn clone(self: &Self) -> InflateState`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Default`

- `fn default() -> Self`

## Traits

### `ResetPolicy`

```rust
trait ResetPolicy { ... }
```

Tag that determines reset policy of [InflateState](#inflatestate)

#### Required Methods

- `fn reset(self: &Self, state: &mut InflateState)`

  Performs reset

## Functions

### `inflate`

```rust
fn inflate(state: &mut InflateState, input: &[u8], output: &mut [u8], flush: crate::MZFlush) -> crate::StreamResult
```

Try to decompress from `input` to `output` with the given [`InflateState`](#inflatestate)

# `flush`

Generally, the various [`MZFlush`](../../index.md) flags have meaning only on the compression side.  They can be
supplied here, but the only one that has any semantic meaning is `MZFlush::Finish`, which is a
signal that the stream is expected to finish, and failing to do so is an error.  It isn't
necessary to specify it when the stream ends; you'll still get returned a
`MZStatus::StreamEnd` anyway.  Other values either have no effect or cause errors.  It's
likely that you'll almost always just want to use `MZFlush::None`.

# Errors

Returns `MZError::Buf` if the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called with `MZFlush::Finish` and input wasn't all
consumed.

Returns `MZError::Data` if this or a a previous call failed with an error return from
[`TINFLStatus`](../index.md); probably indicates corrupted data.

Returns `MZError::Stream` when called with `MZFlush::Full` (meaningless on
decompression), or when called without `MZFlush::Finish` after an earlier call with
`MZFlush::Finish` has been made.

