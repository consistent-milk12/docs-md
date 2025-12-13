*[clap_builder](../../index.md) / [builder](../index.md) / [arg_predicate](index.md)*

---

# Module `arg_predicate`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArgPredicate`](#argpredicate) | enum | Operations to perform on argument values |

## Enums

### `ArgPredicate`

```rust
enum ArgPredicate {
    IsPresent,
    Equals(crate::builder::OsStr),
}
```

*Defined in [`clap_builder-4.5.53/src/builder/arg_predicate.rs:8-13`](../../../../.source_1765633015/clap_builder-4.5.53/src/builder/arg_predicate.rs#L8-L13)*

Operations to perform on argument values

These do not apply to `ValueSource::DefaultValue`

#### Variants

- **`IsPresent`**

  Is the argument present?

- **`Equals`**

  Does the argument match the specified value?

#### Trait Implementations

##### `impl Any for ArgPredicate`

- <span id="argpredicate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgPredicate`

- <span id="argpredicate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgPredicate`

- <span id="argpredicate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgPredicate`

- <span id="argpredicate-clone"></span>`fn clone(&self) -> ArgPredicate` — [`ArgPredicate`](#argpredicate)

##### `impl CloneToUninit for ArgPredicate`

- <span id="argpredicate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgPredicate`

- <span id="argpredicate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArgPredicate`

##### `impl<T> From for ArgPredicate`

- <span id="argpredicate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgPredicate`

- <span id="argpredicate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgPredicate`

- <span id="argpredicate-partialeq-eq"></span>`fn eq(&self, other: &ArgPredicate) -> bool` — [`ArgPredicate`](#argpredicate)

##### `impl StructuralPartialEq for ArgPredicate`

##### `impl ToOwned for ArgPredicate`

- <span id="argpredicate-toowned-type-owned"></span>`type Owned = T`

- <span id="argpredicate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argpredicate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgPredicate`

- <span id="argpredicate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argpredicate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgPredicate`

- <span id="argpredicate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argpredicate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

