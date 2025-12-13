*[unicode_ident](../index.md) / [tables](index.md)*

---

# Module `tables`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Align8`](#align8) | struct |  |
| [`Align64`](#align64) | struct |  |
| [`UNICODE_VERSION`](#unicode-version) | const |  |
| [`ASCII_START`](#ascii-start) | const |  |
| [`ASCII_CONTINUE`](#ascii-continue) | const |  |
| [`CHUNK`](#chunk) | const |  |

## Structs

### `Align8<T>`

```rust
struct Align8<T>(T);
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:8`](../../../.source_1765521767/unicode-ident-1.0.22/src/tables.rs#L8)*

#### Trait Implementations

##### `impl<T> Any for Align8<T>`

- <span id="align8-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Align8<T>`

- <span id="align8-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Align8<T>`

- <span id="align8-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Align8<T>`

- <span id="align8-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Align8<T>`

- <span id="align8-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Align8<T>`

- <span id="align8-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="align8-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Align8<T>`

- <span id="align8-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="align8-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Align64<T>`

```rust
struct Align64<T>(T);
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:10`](../../../.source_1765521767/unicode-ident-1.0.22/src/tables.rs#L10)*

#### Trait Implementations

##### `impl<T> Any for Align64<T>`

- <span id="align64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Align64<T>`

- <span id="align64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Align64<T>`

- <span id="align64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Align64<T>`

- <span id="align64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Align64<T>`

- <span id="align64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Align64<T>`

- <span id="align64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="align64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Align64<T>`

- <span id="align64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="align64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `UNICODE_VERSION`
```rust
const UNICODE_VERSION: (u8, u8, u8);
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:12`](../../../.source_1765521767/unicode-ident-1.0.22/src/tables.rs#L12)*

### `ASCII_START`
```rust
const ASCII_START: u128 = 10_633_823_810_298_881_996_379_053_697_534_001_152u128;
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:14`](../../../.source_1765521767/unicode-ident-1.0.22/src/tables.rs#L14)*

### `ASCII_CONTINUE`
```rust
const ASCII_CONTINUE: u128 = 10_633_823_849_912_963_253_799_171_395_480_977_408u128;
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:15`](../../../.source_1765521767/unicode-ident-1.0.22/src/tables.rs#L15)*

### `CHUNK`
```rust
const CHUNK: usize = 64usize;
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:17`](../../../.source_1765521767/unicode-ident-1.0.22/src/tables.rs#L17)*

