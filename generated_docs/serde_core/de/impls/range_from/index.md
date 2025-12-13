*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range_from](index.md)*

---

# Module `range_from`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeFromVisitor`](#rangefromvisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeFromVisitor<Idx>`

```rust
struct RangeFromVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2680-2683`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2680-L2683)*

#### Trait Implementations

##### `impl Any for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangefromvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangefromvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Idx> Visitor for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-visitor-type-value"></span>`type Value = Idx`

- <span id="rangefromvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangefromvisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangefromvisitor-visitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
    Start,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2634-2636`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2634-L2636)*

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

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2628`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/impls.rs#L2628)*

