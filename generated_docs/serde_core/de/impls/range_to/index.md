*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range_to](index.md)*

---

# Module `range_to`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeToVisitor`](#rangetovisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeToVisitor<Idx>`

```rust
struct RangeToVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2819-2822`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2819-L2822)*

#### Trait Implementations

##### `impl Any for RangeToVisitor<Idx>`

- <span id="rangetovisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeToVisitor<Idx>`

- <span id="rangetovisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeToVisitor<Idx>`

- <span id="rangetovisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for RangeToVisitor<Idx>`

- <span id="rangetovisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RangeToVisitor<Idx>`

- <span id="rangetovisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeToVisitor<Idx>`

- <span id="rangetovisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RangeToVisitor<Idx>`

- <span id="rangetovisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangetovisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeToVisitor<Idx>`

- <span id="rangetovisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangetovisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Idx> Visitor for RangeToVisitor<Idx>`

- <span id="rangetovisitor-visitor-type-value"></span>`type Value = Idx`

- <span id="rangetovisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangetovisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangetovisitor-visitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
    End,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2773-2775`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2773-L2775)*

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

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2767`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2767)*

