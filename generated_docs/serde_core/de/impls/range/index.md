*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range](index.md)*

---

# Module `range`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeVisitor`](#rangevisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeVisitor<Idx>`

```rust
struct RangeVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2524-2527`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2524-L2527)*

#### Trait Implementations

##### `impl Any for RangeVisitor<Idx>`

- <span id="rangevisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeVisitor<Idx>`

- <span id="rangevisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeVisitor<Idx>`

- <span id="rangevisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for RangeVisitor<Idx>`

- <span id="rangevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RangeVisitor<Idx>`

- <span id="rangevisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeVisitor<Idx>`

- <span id="rangevisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RangeVisitor<Idx>`

- <span id="rangevisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangevisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeVisitor<Idx>`

- <span id="rangevisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangevisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Idx> Visitor for RangeVisitor<Idx>`

- <span id="rangevisitor-visitor-type-value"></span>`type Value = (Idx, Idx)`

- <span id="rangevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangevisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangevisitor-visitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
    Start,
    End,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2475-2478`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2475-L2478)*

#### Trait Implementations

##### `impl Any for Field`

- <span id="field-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Field`

- <span id="field-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Field`

- <span id="field-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deserialize for Field`

- <span id="field-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../index.md#deserializer)

##### `impl DeserializeOwned for Field`

##### `impl<T> From for Field`

- <span id="field-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Field`

- <span id="field-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Field`

- <span id="field-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="field-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Field`

- <span id="field-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="field-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `FIELDS`
```rust
const FIELDS: &[&str];
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2469`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2469)*

