*[rayon_core](../index.md) / [unwind](index.md)*

---

# Module `unwind`

Package up unwind recovery. Note that if you are in some sensitive
place, you can use the `AbortIfPanic` helper to protect against
accidental panics in the rayon code itself.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AbortIfPanic`](#abortifpanic) | struct |  |
| [`halt_unwinding`](#halt-unwinding) | fn | Executes `f` and captures any panic, translating that panic into a `Err` result. |
| [`resume_unwinding`](#resume-unwinding) | fn |  |

## Structs

### `AbortIfPanic`

```rust
struct AbortIfPanic;
```

*Defined in [`rayon-core-1.13.0/src/unwind.rs:24`](../../../.source_1765633015/rayon-core-1.13.0/src/unwind.rs#L24)*

#### Trait Implementations

##### `impl Any for AbortIfPanic`

- <span id="abortifpanic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AbortIfPanic`

- <span id="abortifpanic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AbortIfPanic`

- <span id="abortifpanic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for AbortIfPanic`

- <span id="abortifpanic-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for AbortIfPanic`

- <span id="abortifpanic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AbortIfPanic`

- <span id="abortifpanic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for AbortIfPanic`

- <span id="abortifpanic-pointable-const-align"></span>`const ALIGN: usize`

- <span id="abortifpanic-pointable-type-init"></span>`type Init = T`

- <span id="abortifpanic-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="abortifpanic-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="abortifpanic-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="abortifpanic-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for AbortIfPanic`

- <span id="abortifpanic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abortifpanic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AbortIfPanic`

- <span id="abortifpanic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abortifpanic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `halt_unwinding`

```rust
fn halt_unwinding<F, R>(func: F) -> thread::Result<R>
where
    F: FnOnce() -> R
```

*Defined in [`rayon-core-1.13.0/src/unwind.rs:13-18`](../../../.source_1765633015/rayon-core-1.13.0/src/unwind.rs#L13-L18)*

Executes `f` and captures any panic, translating that panic into a
`Err` result. The assumption is that any panic will be propagated
later with `resume_unwinding`, and hence `f` can be treated as
exception safe.

### `resume_unwinding`

```rust
fn resume_unwinding(payload: Box<dyn Any + Send>) -> never
```

*Defined in [`rayon-core-1.13.0/src/unwind.rs:20-22`](../../../.source_1765633015/rayon-core-1.13.0/src/unwind.rs#L20-L22)*

