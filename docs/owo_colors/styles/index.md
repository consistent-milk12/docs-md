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

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for BoldDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for BoldDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DimDisplay<'a, T: ?Sized>`

```rust
struct DimDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of dimming it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::dimmed).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for DimDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for DimDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ItalicDisplay<'a, T: ?Sized>`

```rust
struct ItalicDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of italics. Recommended to be constructed using
[`OwoColorize`](OwoColorize::italic).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ItalicDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for ItalicDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnderlineDisplay<'a, T: ?Sized>`

```rust
struct UnderlineDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while underlining it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::underline).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for UnderlineDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for UnderlineDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkDisplay<'a, T: ?Sized>`

```rust
struct BlinkDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while blinking. Recommended to be constructed using
[`OwoColorize`](OwoColorize::blink).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for BlinkDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for BlinkDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkFastDisplay<'a, T: ?Sized>`

```rust
struct BlinkFastDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of making it blink fast. Use [`OwoColorize`](OwoColorize::blink_fast)

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for BlinkFastDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for BlinkFastDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReversedDisplay<'a, T: ?Sized>`

```rust
struct ReversedDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of swapping fg and bg colors. Use [`OwoColorize`](OwoColorize::reversed)

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ReversedDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for ReversedDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HiddenDisplay<'a, T: ?Sized>`

```rust
struct HiddenDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of hiding the text. Use [`OwoColorize`](OwoColorize::hidden).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for HiddenDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for HiddenDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StrikeThroughDisplay<'a, T: ?Sized>`

```rust
struct StrikeThroughDisplay<'a, T: ?Sized>(&'a T);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
crossed out. Recommended to be constructed using
[`OwoColorize`](OwoColorize::strikethrough).

#### Implementations

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

#### Trait Implementations

##### `impl<'a, T: ?Sized + fmt::Binary> Binary for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Debug> Debug for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Display> Display for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerExp> LowerExp for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::LowerHex> LowerHex for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::Octal> Octal for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StrikeThroughDisplay<'a, T>`

##### `impl<'a, T: ?Sized + fmt::Pointer> Pointer for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperExp> UpperExp for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: ?Sized + fmt::UpperHex> UpperHex for StrikeThroughDisplay<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

