*[console](../index.md) / [kb](index.md)*

---

# Module `kb`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Key`](#key) | enum | Key mapping |

## Enums

### `Key`

```rust
enum Key {
    Unknown,
    UnknownEscSeq(alloc::vec::Vec<char>),
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Enter,
    Escape,
    Backspace,
    Home,
    End,
    Tab,
    BackTab,
    Alt,
    Del,
    Shift,
    Insert,
    PageUp,
    PageDown,
    Char(char),
    CtrlC,
}
```

*Defined in [`console-0.16.1/src/kb.rs:9-32`](../../../.source_1765521767/console-0.16.1/src/kb.rs#L9-L32)*

Key mapping

This is an incomplete mapping of keys that are supported for reading
from the keyboard.

#### Variants

- **`UnknownEscSeq`**

  Unrecognized sequence containing Esc and a list of chars

#### Trait Implementations

##### `impl Any for Key`

- <span id="key-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Key`

- <span id="key-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Key`

- <span id="key-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

##### `impl CloneToUninit for Key`

- <span id="key-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Key`

- <span id="key-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Key`

##### `impl<T> From for Key`

- <span id="key-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Key`

- <span id="key-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Key`

- <span id="key-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Key`

- <span id="key-partialeq-eq"></span>`fn eq(&self, other: &Key) -> bool` — [`Key`](#key)

##### `impl StructuralPartialEq for Key`

##### `impl ToOwned for Key`

- <span id="key-toowned-type-owned"></span>`type Owned = T`

- <span id="key-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="key-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Key`

- <span id="key-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="key-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Key`

- <span id="key-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="key-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

