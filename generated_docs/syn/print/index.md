*[syn](../index.md) / [print](index.md)*

---

# Module `print`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokensOrDefault`](#tokensordefault) | struct |  |

## Structs

### `TokensOrDefault<'a, T: 'a>`

```rust
struct TokensOrDefault<'a, T: 'a>(&'a Option<T>);
```

*Defined in [`syn-2.0.111/src/print.rs:4`](../../../.source_1765633015/syn-2.0.111/src/print.rs#L4)*

#### Trait Implementations

##### `impl<T> Any for TokensOrDefault<'a, T>`

- <span id="tokensordefault-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TokensOrDefault<'a, T>`

- <span id="tokensordefault-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TokensOrDefault<'a, T>`

- <span id="tokensordefault-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TokensOrDefault<'a, T>`

- <span id="tokensordefault-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for TokensOrDefault<'a, T>`

- <span id="tokensordefault-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Sealed for TokensOrDefault<'a, T>`

##### `impl<T> Spanned for TokensOrDefault<'a, T>`

- <span id="tokensordefault-spanned-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToTokens for TokensOrDefault<'a, T>`

- <span id="tokensordefault-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<T, U> TryFrom for TokensOrDefault<'a, T>`

- <span id="tokensordefault-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tokensordefault-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TokensOrDefault<'a, T>`

- <span id="tokensordefault-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tokensordefault-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

