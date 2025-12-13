*[miette](../../index.md) / [eyreish](../index.md) / [kind](index.md)*

---

# Module `kind`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Adhoc`](#adhoc) | struct |  |
| [`Trait`](#trait) | struct |  |
| [`Boxed`](#boxed) | struct |  |
| [`AdhocKind`](#adhockind) | trait |  |
| [`TraitKind`](#traitkind) | trait |  |
| [`BoxedKind`](#boxedkind) | trait |  |

## Structs

### `Adhoc`

```rust
struct Adhoc;
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:53`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L53)*

#### Implementations

- <span id="adhoc-new"></span>`fn new<M>(self, message: M) -> Report` — [`Report`](../../index.md#report)

#### Trait Implementations

##### `impl Any for Adhoc`

- <span id="adhoc-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Adhoc`

- <span id="adhoc-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Adhoc`

- <span id="adhoc-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Adhoc`

- <span id="adhoc-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Adhoc`

- <span id="adhoc-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Adhoc`

##### `impl<U> TryFrom for Adhoc`

- <span id="adhoc-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="adhoc-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Adhoc`

- <span id="adhoc-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="adhoc-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Trait`

```rust
struct Trait;
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:75`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L75)*

#### Implementations

- <span id="trait-new"></span>`fn new<E>(self, error: E) -> Report` — [`Report`](../../index.md#report)

#### Trait Implementations

##### `impl Any for Trait`

- <span id="trait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Trait`

- <span id="trait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Trait`

- <span id="trait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Trait`

- <span id="trait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Trait`

- <span id="trait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Trait`

##### `impl<U> TryFrom for Trait`

- <span id="trait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Trait`

- <span id="trait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Boxed`

```rust
struct Boxed;
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:97`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L97)*

#### Implementations

- <span id="boxed-new"></span>`fn new(self, error: Box<dyn Diagnostic + Send + Sync>) -> Report` — [`Diagnostic`](../../index.md#diagnostic), [`Report`](../../index.md#report)

#### Trait Implementations

##### `impl Any for Boxed`

- <span id="boxed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Boxed`

- <span id="boxed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Boxed`

- <span id="boxed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Boxed`

- <span id="boxed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Boxed`

- <span id="boxed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Boxed`

##### `impl<U> TryFrom for Boxed`

- <span id="boxed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boxed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Boxed`

- <span id="boxed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boxed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `AdhocKind`

```rust
trait AdhocKind: Sized { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:55-60`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L55-L60)*

#### Provided Methods

- `fn miette_kind(&self) -> Adhoc`

#### Implementors

- `&T`

### `TraitKind`

```rust
trait TraitKind: Sized { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:77-82`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L77-L82)*

#### Provided Methods

- `fn miette_kind(&self) -> Trait`

#### Implementors

- `E`

### `BoxedKind`

```rust
trait BoxedKind: Sized { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:99-104`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L99-L104)*

#### Provided Methods

- `fn miette_kind(&self) -> Boxed`

#### Implementors

- `Box<dyn Diagnostic + Send + Sync>`

