*[console](../index.md) / [kb](index.md)*

---

# Module `kb`

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

Key mapping

This is an incomplete mapping of keys that are supported for reading
from the keyboard.

#### Variants

- **`UnknownEscSeq`**

  Unrecognized sequence containing Esc and a list of chars

#### Trait Implementations

##### `impl Clone for Key`

- `fn clone(self: &Self) -> Key` — [`Key`](#key)

##### `impl Debug for Key`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Key`

##### `impl Hash for Key`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Key`

- `fn eq(self: &Self, other: &Key) -> bool` — [`Key`](#key)

##### `impl StructuralPartialEq for Key`

