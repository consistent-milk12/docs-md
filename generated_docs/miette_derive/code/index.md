*[miette_derive](../index.md) / [code](index.md)*

---

# Module `code`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Code`](#code) | struct |  |

## Structs

### `Code`

```rust
struct Code(String);
```

*Defined in [`miette-derive-7.6.0/src/code.rs:16`](../../../.source_1765521767/miette-derive-7.6.0/src/code.rs#L16)*

#### Implementations

- <span id="code-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md#diagnosticdef)

- <span id="code-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Any for Code`

- <span id="code-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Code`

- <span id="code-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Code`

- <span id="code-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Code`

- <span id="code-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Code`

- <span id="code-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Code`

- <span id="code-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Code`

- <span id="code-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<U> TryFrom for Code`

- <span id="code-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="code-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Code`

- <span id="code-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="code-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

