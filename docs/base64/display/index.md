*[base64](../index.md) / [display](index.md)*

---

# Module `display`

Enables base64'd output anywhere you might use a `Display` implementation, like a format string.

```rust
use base64::{display::Base64Display, engine::general_purpose::STANDARD};

let data = vec![0x0, 0x1, 0x2, 0x3];
let wrapper = Base64Display::new(&data, &STANDARD);

assert_eq!("base64: AAECAw==", format!("base64: {}", wrapper));
```

## Structs

### `Base64Display<'a, 'e, E: Engine>`

```rust
struct Base64Display<'a, 'e, E: Engine> {
    // [REDACTED: Private Fields]
}
```

A convenience wrapper for base64'ing bytes into a format string without heap allocation.

#### Implementations

- `fn new(bytes: &'a [u8], engine: &'e E) -> Base64Display<'a, 'e, E>`
  Create a `Base64Display` with the provided engine.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, 'e, E: Engine>`

- `fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

