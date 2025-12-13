*[regex_syntax](../index.md) / [either](index.md)*

---

# Module `either`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Either`](#either) | enum | A simple binary sum type. |

## Enums

### `Either<Left, Right>`

```rust
enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}
```

*Defined in [`regex-syntax-0.8.8/src/either.rs:5-8`](../../../.source_1765521767/regex-syntax-0.8.8/src/either.rs#L5-L8)*

A simple binary sum type.

This is occasionally useful in an ad hoc fashion.

#### Trait Implementations

##### `impl Any for Either<Left, Right>`

- <span id="either-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Either<Left, Right>`

- <span id="either-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Either<Left, Right>`

- <span id="either-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Left: clone::Clone, Right: clone::Clone> Clone for Either<Left, Right>`

- <span id="either-clone"></span>`fn clone(&self) -> Either<Left, Right>` — [`Either`](#either)

##### `impl CloneToUninit for Either<Left, Right>`

- <span id="either-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Left: fmt::Debug, Right: fmt::Debug> Debug for Either<Left, Right>`

- <span id="either-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Left: cmp::Eq, Right: cmp::Eq> Eq for Either<Left, Right>`

##### `impl<T> From for Either<Left, Right>`

- <span id="either-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Either<Left, Right>`

- <span id="either-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Left: cmp::PartialEq, Right: cmp::PartialEq> PartialEq for Either<Left, Right>`

- <span id="either-partialeq-eq"></span>`fn eq(&self, other: &Either<Left, Right>) -> bool` — [`Either`](#either)

##### `impl<Left, Right> StructuralPartialEq for Either<Left, Right>`

##### `impl ToOwned for Either<Left, Right>`

- <span id="either-toowned-type-owned"></span>`type Owned = T`

- <span id="either-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="either-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Either<Left, Right>`

- <span id="either-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="either-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Either<Left, Right>`

- <span id="either-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="either-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

