*[syn](../index.md) / [thread](index.md)*

---

# Module `thread`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ThreadBound`](#threadbound) | struct | ThreadBound is a Sync-maker and Send-maker that allows accessing a value of type T only from the original thread on which the ThreadBound was constructed. |

## Structs

### `ThreadBound<T>`

```rust
struct ThreadBound<T> {
    value: T,
    thread_id: std::thread::ThreadId,
}
```

*Defined in [`syn-2.0.111/src/thread.rs:7-10`](../../../.source_1765633015/syn-2.0.111/src/thread.rs#L7-L10)*

ThreadBound is a Sync-maker and Send-maker that allows accessing a value
of type T only from the original thread on which the ThreadBound was
constructed.

#### Implementations

- <span id="threadbound-new"></span>`fn new(value: T) -> Self`

- <span id="threadbound-get"></span>`fn get(&self) -> Option<&T>`

#### Trait Implementations

##### `impl<T> Any for ThreadBound<T>`

- <span id="threadbound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreadBound<T>`

- <span id="threadbound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreadBound<T>`

- <span id="threadbound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Copy> Clone for ThreadBound<T>`

- <span id="threadbound-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for ThreadBound<T>`

- <span id="threadbound-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: Copy> Copy for ThreadBound<T>`

##### `impl<T: Debug> Debug for ThreadBound<T>`

- <span id="threadbound-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ThreadBound<T>`

- <span id="threadbound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ThreadBound<T>`

- <span id="threadbound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Copy> Send for ThreadBound<T>`

##### `impl<T> Sync for ThreadBound<T>`

##### `impl<T> ToOwned for ThreadBound<T>`

- <span id="threadbound-toowned-type-owned"></span>`type Owned = T`

- <span id="threadbound-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="threadbound-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for ThreadBound<T>`

- <span id="threadbound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threadbound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ThreadBound<T>`

- <span id="threadbound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threadbound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

