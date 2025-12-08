*[libc](../index.md) / [types](index.md)*

---

# Module `types`

Platform-agnostic support types.

## Structs

### `Padding<T: Copy>`

```rust
struct Padding<T: Copy>(core::mem::MaybeUninit<T>);
```

A transparent wrapper over `MaybeUninit<T>` to represent uninitialized padding
while providing `Default`.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + Copy> Clone for Padding<T>`

- `fn clone(self: &Self) -> Padding<T>` — [`Padding`](#padding)

##### `impl<T: $crate::marker::Copy + Copy> Copy for Padding<T>`

##### `impl<T: Copy> Debug for Padding<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Copy> Default for Padding<T>`

- `fn default() -> Self`

## Type Aliases

### `CEnumRepr`

```rust
type CEnumRepr = crate::c_uint;
```

