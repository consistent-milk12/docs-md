*[proc_macro2](../index.md) / [marker](index.md)*

---

# Module `marker`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProcMacroAutoTraits`](#procmacroautotraits) | struct |  |
| [`MARKER`](#marker) | const |  |

## Structs

### `ProcMacroAutoTraits`

```rust
struct ProcMacroAutoTraits(core::marker::PhantomData<alloc::rc::Rc<()>>);
```

*Defined in [`proc-macro2-1.0.103/src/marker.rs:12`](../../../.source_1765633015/proc-macro2-1.0.103/src/marker.rs#L12)*

#### Trait Implementations

##### `impl Any for ProcMacroAutoTraits`

- <span id="procmacroautotraits-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProcMacroAutoTraits`

- <span id="procmacroautotraits-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProcMacroAutoTraits`

- <span id="procmacroautotraits-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ProcMacroAutoTraits`

- <span id="procmacroautotraits-clone"></span>`fn clone(&self) -> ProcMacroAutoTraits` — [`ProcMacroAutoTraits`](#procmacroautotraits)

##### `impl CloneToUninit for ProcMacroAutoTraits`

- <span id="procmacroautotraits-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ProcMacroAutoTraits`

##### `impl<T> From for ProcMacroAutoTraits`

- <span id="procmacroautotraits-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProcMacroAutoTraits`

- <span id="procmacroautotraits-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for ProcMacroAutoTraits`

##### `impl ToOwned for ProcMacroAutoTraits`

- <span id="procmacroautotraits-toowned-type-owned"></span>`type Owned = T`

- <span id="procmacroautotraits-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="procmacroautotraits-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ProcMacroAutoTraits`

- <span id="procmacroautotraits-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="procmacroautotraits-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProcMacroAutoTraits`

- <span id="procmacroautotraits-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="procmacroautotraits-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnwindSafe for ProcMacroAutoTraits`

## Constants

### `MARKER`
```rust
const MARKER: ProcMacroAutoTraits;
```

*Defined in [`proc-macro2-1.0.103/src/marker.rs:14`](../../../.source_1765633015/proc-macro2-1.0.103/src/marker.rs#L14)*

