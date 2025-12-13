*[rustix](../../index.md) / [maybe_polyfill](../index.md) / [io](index.md)*

---

# Module `io`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IoSliceMut`](#ioslicemut) | struct |  |

## Structs

### `IoSliceMut<'ctx, R>`

```rust
struct IoSliceMut<'ctx, R>
where
    R: gimli::Reader {
    unit: &'ctx crate::unit::ResUnit<R>,
    sections: &'ctx gimli::Dwarf<R>,
    function: &'ctx crate::function::Function<R>,
    inlined_functions: iter::Rev<alloc::vec::IntoIter<&'ctx crate::function::InlinedFunction<R>>>,
    next: Option<Location<'ctx>>,
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:43-52`](../../../../.source_1765633015/addr2line-0.25.1/src/frame.rs#L43-L52)*

*Re-exported from `addr2line`*

#### Trait Implementations

##### `impl Any for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frameiterframes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`Buffer`](../../buffer/index.md#buffer), [`buffer`](../../buffer/index.md#buffer)

##### `impl<U> TryInto for FrameIterFrames<'ctx, R>`

- <span id="frameiterframes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frameiterframes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`Buffer`](../../buffer/index.md#buffer), [`buffer`](../../buffer/index.md#buffer)

