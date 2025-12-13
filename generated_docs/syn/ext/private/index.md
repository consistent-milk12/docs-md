*[syn](../../index.md) / [ext](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PeekFn`](#peekfn) | struct |  |
| [`IdentAny`](#identany) | struct |  |
| [`Sealed`](#sealed) | trait |  |

## Structs

### `PeekFn`

```rust
struct PeekFn;
```

*Defined in [`syn-2.0.111/src/ext.rs:165`](../../../../.source_1765633015/syn-2.0.111/src/ext.rs#L165)*

#### Trait Implementations

##### `impl Any for PeekFn`

- <span id="peekfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeekFn`

- <span id="peekfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeekFn`

- <span id="peekfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PeekFn`

- <span id="peekfn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PeekFn`

- <span id="peekfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PeekFn`

##### `impl<T> From for PeekFn`

- <span id="peekfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeekFn`

- <span id="peekfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Peek for private::PeekFn`

##### `impl Sealed for private::PeekFn`

##### `impl ToOwned for PeekFn`

- <span id="peekfn-toowned-type-owned"></span>`type Owned = T`

- <span id="peekfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="peekfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PeekFn`

- <span id="peekfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="peekfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeekFn`

- <span id="peekfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="peekfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IdentAny`

```rust
struct IdentAny;
```

*Defined in [`syn-2.0.111/src/ext.rs:168`](../../../../.source_1765633015/syn-2.0.111/src/ext.rs#L168)*

#### Trait Implementations

##### `impl Any for IdentAny`

- <span id="identany-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IdentAny`

- <span id="identany-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IdentAny`

- <span id="identany-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IdentAny`

- <span id="identany-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IdentAny`

- <span id="identany-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sealed for IdentAny`

##### `impl Token for IdentAny`

- <span id="identany-token-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool` — [`Cursor`](../../buffer/index.md#cursor)

- <span id="identany-token-display"></span>`fn display() -> &'static str`

##### `impl<U> TryFrom for IdentAny`

- <span id="identany-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="identany-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IdentAny`

- <span id="identany-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="identany-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

*Defined in [`syn-2.0.111/src/ext.rs:160`](../../../../.source_1765633015/syn-2.0.111/src/ext.rs#L160)*

#### Implementors

- [`Ident`](../../ident/index.md#ident)

