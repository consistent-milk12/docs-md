*[rustversion](../index.md) / [date](index.md)*

---

# Module `date`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Date`](#date) | struct |  |
| [`parse`](#parse) | fn |  |
| [`try_parse`](#try-parse) | fn |  |

## Structs

### `Date`

```rust
struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}
```

*Defined in [`rustversion-1.0.22/src/date.rs:8-12`](../../../.source_1765633015/rustversion-1.0.22/src/date.rs#L8-L12)*

#### Trait Implementations

##### `impl Any for Date`

- <span id="date-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Date`

- <span id="date-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Date`

- <span id="date-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Date`

- <span id="date-clone"></span>`fn clone(&self) -> Date` — [`Date`](#date)

##### `impl CloneToUninit for Date`

- <span id="date-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Date`

##### `impl Debug for Date`

- <span id="date-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Date`

- <span id="date-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Date`

##### `impl<T> From for Date`

- <span id="date-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Date`

- <span id="date-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Date`

- <span id="date-ord-cmp"></span>`fn cmp(&self, other: &Date) -> cmp::Ordering` — [`Date`](#date)

##### `impl PartialEq for Date`

- <span id="date-partialeq-eq"></span>`fn eq(&self, other: &Date) -> bool` — [`Date`](#date)

##### `impl PartialOrd for Date`

- <span id="date-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Date) -> option::Option<cmp::Ordering>` — [`Date`](#date)

##### `impl StructuralPartialEq for Date`

##### `impl ToOwned for Date`

- <span id="date-toowned-type-owned"></span>`type Owned = T`

- <span id="date-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="date-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Date`

- <span id="date-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Date`

- <span id="date-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="date-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Date`

- <span id="date-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="date-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Date, Error>
```

*Defined in [`rustversion-1.0.22/src/date.rs:24-29`](../../../.source_1765633015/rustversion-1.0.22/src/date.rs#L24-L29)*

### `try_parse`

```rust
fn try_parse(iter: &'_ mut IterImpl) -> std::result::Result<Date, ()>
```

*Defined in [`rustversion-1.0.22/src/date.rs:31-50`](../../../.source_1765633015/rustversion-1.0.22/src/date.rs#L31-L50)*

