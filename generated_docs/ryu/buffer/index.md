*[ryu](../index.md) / [buffer](index.md)*

---

# Module `buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Buffer`](#buffer) | struct | Safe API for formatting floating point numbers to text. |
| [`Float`](#float) | trait | A floating point number, f32 or f64, that can be written into a [`ryu::Buffer`][Buffer]. |
| [`Sealed`](#sealed) | trait |  |
| [`NAN`](#nan) | const |  |
| [`INFINITY`](#infinity) | const |  |
| [`NEG_INFINITY`](#neg-infinity) | const |  |

## Structs

### `Buffer`

```rust
struct Buffer {
    bytes: [core::mem::MaybeUninit<u8>; 24],
}
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:20-22`](../../../.source_1765633015/ryu-1.0.20/src/buffer/mod.rs#L20-L22)*

Safe API for formatting floating point numbers to text.

## Example

```rust
let mut buffer = ryu::Buffer::new();
let printed = buffer.format_finite(1.234);
assert_eq!(printed, "1.234");
```

#### Implementations

- <span id="buffer-new"></span>`fn new() -> Self`

  This is a cheap operation; you don't need to worry about reusing buffers

  for efficiency.

- <span id="buffer-format"></span>`fn format<F: Float>(&mut self, f: F) -> &str`

  Print a floating point number into this buffer and return a reference to

  its string representation within the buffer.

  

  # Special cases

  

  This function formats NaN as the string "NaN", positive infinity as

  "inf", and negative infinity as "-inf" to match std::fmt.

  

  If your input is known to be finite, you may get better performance by

  calling the `format_finite` method instead of `format` to avoid the

  checks for special cases.

- <span id="buffer-format-finite"></span>`fn format_finite<F: Float>(&mut self, f: F) -> &str`

  Print a floating point number into this buffer and return a reference to

  its string representation within the buffer.

  

  # Special cases

  

  This function **does not** check for NaN or infinity. If the input

  number is not a finite float, the printed representation will be some

  correctly formatted but unspecified numerical value.

  

  Please check `is_finite` yourself before calling this function, or

  check `is_nan` and `is_infinite` and handle those cases yourself.

  

  

#### Trait Implementations

##### `impl Any for Buffer`

- <span id="buffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Buffer`

- <span id="buffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Buffer`

- <span id="buffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Buffer`

- <span id="buffer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Buffer`

- <span id="buffer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- <span id="buffer-default"></span>`fn default() -> Self`

##### `impl<T> From for Buffer`

- <span id="buffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Buffer`

- <span id="buffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Buffer`

- <span id="buffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Buffer`

- <span id="buffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Float`

```rust
trait Float: Sealed { ... }
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:105`](../../../.source_1765633015/ryu-1.0.20/src/buffer/mod.rs#L105)*

A floating point number, f32 or f64, that can be written into a
[`ryu::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of the
`ryu` crate.

#### Implementors

- `f32`
- `f64`

### `Sealed`

```rust
trait Sealed: Copy { ... }
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:109-113`](../../../.source_1765633015/ryu-1.0.20/src/buffer/mod.rs#L109-L113)*

#### Required Methods

- `fn is_nonfinite(self) -> bool`

- `fn format_nonfinite(self) -> &'static str`

- `fn write_to_ryu_buffer(self, result: *mut u8) -> usize`

#### Implementors

- `f32`
- `f64`

## Constants

### `NAN`
```rust
const NAN: &str;
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:7`](../../../.source_1765633015/ryu-1.0.20/src/buffer/mod.rs#L7)*

### `INFINITY`
```rust
const INFINITY: &str;
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:8`](../../../.source_1765633015/ryu-1.0.20/src/buffer/mod.rs#L8)*

### `NEG_INFINITY`
```rust
const NEG_INFINITY: &str;
```

*Defined in [`ryu-1.0.20/src/buffer/mod.rs:9`](../../../.source_1765633015/ryu-1.0.20/src/buffer/mod.rs#L9)*

