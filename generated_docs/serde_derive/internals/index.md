*[serde_derive](../index.md) / [internals](index.md)*

---

# Module `internals`

## Contents

- [Modules](#modules)
  - [`ast`](#ast)
  - [`attr`](#attr)
  - [`name`](#name)
  - [`case`](#case)
  - [`check`](#check)
  - [`ctxt`](#ctxt)
  - [`receiver`](#receiver)
  - [`respan`](#respan)
  - [`symbol`](#symbol)
- [Structs](#structs)
  - [`Ctxt`](#ctxt)
- [Enums](#enums)
  - [`Derive`](#derive)
- [Functions](#functions)
  - [`replace_receiver`](#replace-receiver)
  - [`ungroup`](#ungroup)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ast`](#ast) | mod | A Serde ast, parsed from the Syn ast and ready to generate Rust code. |
| [`attr`](#attr) | mod |  |
| [`name`](#name) | mod |  |
| [`case`](#case) | mod | Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the case of the source (e.g. `my-field`, `MY_FIELD`). |
| [`check`](#check) | mod |  |
| [`ctxt`](#ctxt) | mod |  |
| [`receiver`](#receiver) | mod |  |
| [`respan`](#respan) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`Ctxt`](#ctxt) | struct |  |
| [`Derive`](#derive) | enum |  |
| [`replace_receiver`](#replace-receiver) | fn |  |
| [`ungroup`](#ungroup) | fn |  |

## Modules

- [`ast`](ast/index.md) — A Serde ast, parsed from the Syn ast and ready to generate Rust code.
- [`attr`](attr/index.md)
- [`name`](name/index.md)
- [`case`](case/index.md) — Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the
- [`check`](check/index.md)
- [`ctxt`](ctxt/index.md)
- [`receiver`](receiver/index.md)
- [`respan`](respan/index.md)
- [`symbol`](symbol/index.md)

## Structs

### `Ctxt`

```rust
struct Ctxt {
    errors: std::cell::RefCell<Option<Vec<syn::Error>>>,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ctxt.rs:12-16`](../../../.source_1765633015/serde_derive-1.0.228/src/internals/ctxt.rs#L12-L16)*

A type to collect errors together and format them.

Dropping this object will cause a panic. It must be consumed using `check`.

References can be shared since this type uses run-time exclusive mut checking.

#### Implementations

- <span id="ctxt-new"></span>`fn new() -> Self`

  Create a new context object.

  

  This object contains no errors, but will still trigger a panic if it is not `check`ed.

- <span id="ctxt-error-spanned-by"></span>`fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T)`

  Add an error to the context object with a tokenenizable object.

  

  The object is used for spanning in error messages.

- <span id="ctxt-syn-error"></span>`fn syn_error(&self, err: syn::Error)`

  Add one of Syn's parse errors.

- <span id="ctxt-check"></span>`fn check(self) -> syn::Result<()>`

  Consume this object, producing a formatted error string if there are errors.

#### Trait Implementations

##### `impl Any for Ctxt`

- <span id="ctxt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ctxt`

- <span id="ctxt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ctxt`

- <span id="ctxt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for Ctxt`

- <span id="ctxt-default"></span>`fn default() -> Ctxt` — [`Ctxt`](ctxt/index.md#ctxt)

##### `impl Drop for Ctxt`

- <span id="ctxt-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Ctxt`

- <span id="ctxt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ctxt`

- <span id="ctxt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Ctxt`

- <span id="ctxt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ctxt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ctxt`

- <span id="ctxt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ctxt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Derive`

```rust
enum Derive {
    Serialize,
    Deserialize,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/mod.rs:18-21`](../../../.source_1765633015/serde_derive-1.0.228/src/internals/mod.rs#L18-L21)*

#### Trait Implementations

##### `impl Any for Derive`

- <span id="derive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Derive`

- <span id="derive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Derive`

- <span id="derive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Derive`

- <span id="derive-clone"></span>`fn clone(&self) -> Derive` — [`Derive`](#derive)

##### `impl CloneToUninit for Derive`

- <span id="derive-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Derive`

##### `impl<T> From for Derive`

- <span id="derive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Derive`

- <span id="derive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Derive`

- <span id="derive-toowned-type-owned"></span>`type Owned = T`

- <span id="derive-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="derive-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Derive`

- <span id="derive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="derive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Derive`

- <span id="derive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="derive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `replace_receiver`

```rust
fn replace_receiver(input: &mut syn::DeriveInput)
```

*Defined in [`serde_derive-1.0.228/src/internals/receiver.rs:10-19`](../../../.source_1765633015/serde_derive-1.0.228/src/internals/receiver.rs#L10-L19)*

### `ungroup`

```rust
fn ungroup(ty: &syn::Type) -> &syn::Type
```

*Defined in [`serde_derive-1.0.228/src/internals/mod.rs:23-28`](../../../.source_1765633015/serde_derive-1.0.228/src/internals/mod.rs#L23-L28)*

