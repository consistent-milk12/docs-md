*[rayon_core](../index.md) / [private](index.md)*

---

# Module `private`

The public parts of this private module are used to create traits
that cannot be implemented outside of our own crate.  This way we
can feel free to extend those traits without worrying about it
being a breaking change for other implementations.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PrivateMarker`](#privatemarker) | struct | If this type is pub but not publicly reachable, third parties can't name it and can't implement traits using it. |
| [`private_decl!`](#private-decl) | macro |  |
| [`private_impl!`](#private-impl) | macro |  |

## Structs

### `PrivateMarker`

```rust
struct PrivateMarker;
```

*Defined in [`rayon-core-1.13.0/src/private.rs:9`](../../../.source_1765633015/rayon-core-1.13.0/src/private.rs#L9)*

If this type is pub but not publicly reachable, third parties
can't name it and can't implement traits using it.

#### Trait Implementations

##### `impl Any for PrivateMarker`

- <span id="privatemarker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrivateMarker`

- <span id="privatemarker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrivateMarker`

- <span id="privatemarker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for PrivateMarker`

- <span id="privatemarker-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PrivateMarker`

- <span id="privatemarker-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for PrivateMarker`

- <span id="privatemarker-pointable-const-align"></span>`const ALIGN: usize`

- <span id="privatemarker-pointable-type-init"></span>`type Init = T`

- <span id="privatemarker-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="privatemarker-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="privatemarker-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="privatemarker-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PrivateMarker`

- <span id="privatemarker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="privatemarker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrivateMarker`

- <span id="privatemarker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="privatemarker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `private_decl!`

*Defined in [`rayon-core-1.13.0/src/private.rs:11-18`](../../../.source_1765633015/rayon-core-1.13.0/src/private.rs#L11-L18)*

### `private_impl!`

*Defined in [`rayon-core-1.13.0/src/private.rs:20-26`](../../../.source_1765633015/rayon-core-1.13.0/src/private.rs#L20-L26)*

