*[syn](../index.md) / [tt](index.md)*

---

# Module `tt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenTreeHelper`](#tokentreehelper) | struct |  |
| [`TokenStreamHelper`](#tokenstreamhelper) | struct |  |

## Structs

### `TokenTreeHelper<'a>`

```rust
struct TokenTreeHelper<'a>(&'a proc_macro2::TokenTree);
```

*Defined in [`syn-2.0.111/src/tt.rs:4`](../../../.source_1765521767/syn-2.0.111/src/tt.rs#L4)*

#### Trait Implementations

##### `impl Any for TokenTreeHelper<'a>`

- <span id="tokentreehelper-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TokenTreeHelper<'a>`

- <span id="tokentreehelper-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TokenTreeHelper<'a>`

- <span id="tokentreehelper-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TokenTreeHelper<'a>`

- <span id="tokentreehelper-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for TokenTreeHelper<'a>`

- <span id="tokentreehelper-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl<U> Into for TokenTreeHelper<'a>`

- <span id="tokentreehelper-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TokenTreeHelper<'a>`

- <span id="tokentreehelper-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<U> TryFrom for TokenTreeHelper<'a>`

- <span id="tokentreehelper-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tokentreehelper-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TokenTreeHelper<'a>`

- <span id="tokentreehelper-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tokentreehelper-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TokenStreamHelper<'a>`

```rust
struct TokenStreamHelper<'a>(&'a proc_macro2::TokenStream);
```

*Defined in [`syn-2.0.111/src/tt.rs:65`](../../../.source_1765521767/syn-2.0.111/src/tt.rs#L65)*

#### Trait Implementations

##### `impl Any for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<U> TryFrom for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tokenstreamhelper-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TokenStreamHelper<'a>`

- <span id="tokenstreamhelper-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tokenstreamhelper-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

