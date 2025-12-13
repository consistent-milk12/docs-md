*[clap_builder](../../../index.md) / [parser](../../index.md) / [matches](../index.md) / [value_source](index.md)*

---

# Module `value_source`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValueSource`](#valuesource) | enum | Origin of the argument's value |

## Enums

### `ValueSource`

```rust
enum ValueSource {
    DefaultValue,
    EnvVariable,
    CommandLine,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/value_source.rs:4-11`](../../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/value_source.rs#L4-L11)*

Origin of the argument's value

#### Variants

- **`DefaultValue`**

  Value came `Arg::default_value`

- **`EnvVariable`**

  Value came `Arg::env`

- **`CommandLine`**

  Value was passed in on the command-line

#### Implementations

- <span id="valuesource-is-explicit"></span>`fn is_explicit(self) -> bool`

#### Trait Implementations

##### `impl Any for ValueSource`

- <span id="valuesource-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueSource`

- <span id="valuesource-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueSource`

- <span id="valuesource-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueSource`

- <span id="valuesource-clone"></span>`fn clone(&self) -> ValueSource` — [`ValueSource`](#valuesource)

##### `impl CloneToUninit for ValueSource`

- <span id="valuesource-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueSource`

##### `impl Debug for ValueSource`

- <span id="valuesource-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueSource`

##### `impl<T> From for ValueSource`

- <span id="valuesource-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ValueSource`

- <span id="valuesource-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ValueSource`

- <span id="valuesource-ord-cmp"></span>`fn cmp(&self, other: &ValueSource) -> cmp::Ordering` — [`ValueSource`](#valuesource)

##### `impl PartialEq for ValueSource`

- <span id="valuesource-partialeq-eq"></span>`fn eq(&self, other: &ValueSource) -> bool` — [`ValueSource`](#valuesource)

##### `impl PartialOrd for ValueSource`

- <span id="valuesource-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ValueSource) -> option::Option<cmp::Ordering>` — [`ValueSource`](#valuesource)

##### `impl StructuralPartialEq for ValueSource`

##### `impl ToOwned for ValueSource`

- <span id="valuesource-toowned-type-owned"></span>`type Owned = T`

- <span id="valuesource-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuesource-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueSource`

- <span id="valuesource-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuesource-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueSource`

- <span id="valuesource-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuesource-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

