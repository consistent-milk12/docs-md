*[anstyle](../index.md) / [reset](index.md)*

---

# Module `reset`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Reset`](#reset) | struct | Reset terminal formatting |
| [`RESET`](#reset) | const |  |

## Structs

### `Reset`

```rust
struct Reset;
```

*Defined in [`anstyle-1.0.13/src/reset.rs:4`](../../../.source_1765633015/anstyle-1.0.13/src/reset.rs#L4)*

Reset terminal formatting

#### Implementations

- <span id="reset-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

  

  `Reset` also implements `Display` directly, so calling this method is optional.

#### Trait Implementations

##### `impl Any for Reset`

- <span id="reset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Reset`

- <span id="reset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Reset`

- <span id="reset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Reset`

- <span id="reset-clone"></span>`fn clone(&self) -> Reset` — [`Reset`](../index.md#reset)

##### `impl CloneToUninit for Reset`

- <span id="reset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Reset`

##### `impl Debug for Reset`

- <span id="reset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Reset`

- <span id="reset-default"></span>`fn default() -> Reset` — [`Reset`](../index.md#reset)

##### `impl Display for Reset`

- <span id="reset-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Reset`

##### `impl<T> From for Reset`

- <span id="reset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Reset`

- <span id="reset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Reset`

- <span id="reset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Reset`

- <span id="reset-ord-cmp"></span>`fn cmp(&self, other: &Reset) -> cmp::Ordering` — [`Reset`](../index.md#reset)

##### `impl PartialEq for Reset`

- <span id="reset-partialeq-eq"></span>`fn eq(&self, other: &Reset) -> bool` — [`Reset`](../index.md#reset)

##### `impl PartialOrd for Reset`

- <span id="reset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Reset) -> option::Option<cmp::Ordering>` — [`Reset`](../index.md#reset)

##### `impl StructuralPartialEq for Reset`

##### `impl ToOwned for Reset`

- <span id="reset-toowned-type-owned"></span>`type Owned = T`

- <span id="reset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="reset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Reset`

- <span id="reset-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Reset`

- <span id="reset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Reset`

- <span id="reset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `RESET`
```rust
const RESET: &str;
```

*Defined in [`anstyle-1.0.13/src/reset.rs:22`](../../../.source_1765633015/anstyle-1.0.13/src/reset.rs#L22)*

