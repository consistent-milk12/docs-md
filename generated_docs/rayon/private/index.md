*[rayon](../index.md) / [private](index.md)*

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

*Defined in [`rayon-1.11.0/src/private.rs:9`](../../../.source_1765210505/rayon-1.11.0/src/private.rs#L9)*

If this type is pub but not publicly reachable, third parties
can't name it and can't implement traits using it.

#### Trait Implementations

##### `impl IntoEither for PrivateMarker`

##### `impl Pointable for PrivateMarker`

- <span id="privatemarker-pointable-const-align"></span>`const ALIGN: usize`

- <span id="privatemarker-pointable-type-init"></span>`type Init = T`

- <span id="privatemarker-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="privatemarker-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="privatemarker-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="privatemarker-drop"></span>`unsafe fn drop(ptr: usize)`

## Macros

### `private_decl!`

*Defined in [`rayon-1.11.0/src/private.rs:11-18`](../../../.source_1765210505/rayon-1.11.0/src/private.rs#L11-L18)*

### `private_impl!`

*Defined in [`rayon-1.11.0/src/private.rs:20-26`](../../../.source_1765210505/rayon-1.11.0/src/private.rs#L20-L26)*

