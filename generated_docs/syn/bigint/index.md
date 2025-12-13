*[syn](../index.md) / [bigint](index.md)*

---

# Module `bigint`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BigInt`](#bigint) | struct |  |

## Structs

### `BigInt`

```rust
struct BigInt {
    digits: Vec<u8>,
}
```

*Defined in [`syn-2.0.111/src/bigint.rs:4-6`](../../../.source_1765633015/syn-2.0.111/src/bigint.rs#L4-L6)*

#### Implementations

- <span id="bigint-new"></span>`fn new() -> Self`

- <span id="bigint-to-string"></span>`fn to_string(&self) -> String`

- <span id="bigint-reserve-two-digits"></span>`fn reserve_two_digits(&mut self)`

#### Trait Implementations

##### `impl AddAssign for BigInt`

- <span id="bigint-addassign-add-assign"></span>`fn add_assign(&mut self, increment: u8)`

##### `impl Any for BigInt`

- <span id="bigint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BigInt`

- <span id="bigint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BigInt`

- <span id="bigint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for BigInt`

- <span id="bigint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BigInt`

- <span id="bigint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl MulAssign for BigInt`

- <span id="bigint-mulassign-mul-assign"></span>`fn mul_assign(&mut self, base: u8)`

##### `impl<U> TryFrom for BigInt`

- <span id="bigint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bigint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BigInt`

- <span id="bigint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bigint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

