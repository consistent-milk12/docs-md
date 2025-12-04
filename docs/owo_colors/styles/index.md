*[owo_colors](../index.md) / [styles](index.md)*

---

# Module `styles`

Different display styles (strikethrough, bold, etc.)

## Structs

### `BoldDisplay<'a, T: ?Sized>`

```rust
struct BoldDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of boldening it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::bold).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DimDisplay<'a, T: ?Sized>`

```rust
struct DimDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of dimming it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::dimmed).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ItalicDisplay<'a, T: ?Sized>`

```rust
struct ItalicDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of italics. Recommended to be constructed using
[`OwoColorize`](OwoColorize::italic).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnderlineDisplay<'a, T: ?Sized>`

```rust
struct UnderlineDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while underlining it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::underline).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkDisplay<'a, T: ?Sized>`

```rust
struct BlinkDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while blinking. Recommended to be constructed using
[`OwoColorize`](OwoColorize::blink).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkFastDisplay<'a, T: ?Sized>`

```rust
struct BlinkFastDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of making it blink fast. Use [`OwoColorize`](OwoColorize::blink_fast)

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReversedDisplay<'a, T: ?Sized>`

```rust
struct ReversedDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of swapping fg and bg colors. Use [`OwoColorize`](OwoColorize::reversed)

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HiddenDisplay<'a, T: ?Sized>`

```rust
struct HiddenDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of hiding the text. Use [`OwoColorize`](OwoColorize::hidden).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StrikeThroughDisplay<'a, T: ?Sized>`

```rust
struct StrikeThroughDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
crossed out. Recommended to be constructed using
[`OwoColorize`](OwoColorize::strikethrough).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

