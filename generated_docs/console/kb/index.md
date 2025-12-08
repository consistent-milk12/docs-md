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

Key mapping

This is an incomplete mapping of keys that are supported for reading
from the keyboard.

#### Variants

- **`UnknownEscSeq`**

  Unrecognized sequence containing Esc and a list of chars

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](../index.md)

##### `impl Debug for Key`

- <span id="key-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Key`

##### `impl Hash for Key`

- <span id="key-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Key`

- <span id="key-eq"></span>`fn eq(&self, other: &Key) -> bool` — [`Key`](../index.md)

##### `impl StructuralPartialEq for Key`

