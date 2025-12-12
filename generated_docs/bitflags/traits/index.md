*[bitflags](../index.md) / [traits](index.md)*

---

# Module `traits`

## Contents

- [Modules](#modules)
  - [`__private`](#private)
- [Structs](#structs)
  - [`Flag`](#flag)
- [Traits](#traits)
  - [`Flags`](#flags)
  - [`Bits`](#bits)
  - [`Primitive`](#primitive)
  - [`PublicFlags`](#publicflags)
- [Macros](#macros)
  - [`impl_bits!`](#impl-bits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`__private`](#private) | mod |  |
| [`Flag`](#flag) | struct | A defined flags value that may be named or unnamed. |
| [`Flags`](#flags) | trait | A set of defined flags using a bits type as storage. |
| [`Bits`](#bits) | trait | A bits type that can be used as storage for a flags type. |
| [`Primitive`](#primitive) | trait |  |
| [`PublicFlags`](#publicflags) | trait | A trait for referencing the `bitflags`-owned internal type without exposing it publicly. |
| [`impl_bits!`](#impl-bits) | macro |  |

## Modules

- [`__private`](__private/index.md)

## Structs

### `Flag<B>`

```rust
struct Flag<B> {
    name: &'static str,
    value: B,
}
```

*Defined in [`bitflags-2.10.0/src/traits.rs:15-18`](../../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L15-L18)*

A defined flags value that may be named or unnamed.

#### Implementations

- <span id="flag-new"></span>`const fn new(name: &'static str, value: B) -> Self`

- <span id="flag-name"></span>`const fn name(&self) -> &'static str`

- <span id="flag-value"></span>`const fn value(&self) -> &B`

- <span id="flag-is-named"></span>`const fn is_named(&self) -> bool`

- <span id="flag-is-unnamed"></span>`const fn is_unnamed(&self) -> bool`

#### Trait Implementations

##### `impl<B: fmt::Debug> Debug for Flag<B>`

- <span id="flag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Flags`

```rust
trait Flags: Sized + 'static { ... }
```

*Defined in [`bitflags-2.10.0/src/traits.rs:132-341`](../../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L132-L341)*

A set of defined flags using a bits type as storage.

## Implementing `Flags`

This trait is implemented by the [`bitflags`](#bitflags) macro:

```rust
use bitflags::bitflags;

bitflags! {
    struct MyFlags: u8 {
        const A = 1;
        const B = 1 << 1;
    }
}
```

It can also be implemented manually:

```rust
use bitflags::{Flag, Flags};

struct MyFlags(u8);

impl Flags for MyFlags {
    const FLAGS: &'static [Flag<Self>] = &[
        Flag::new("A", MyFlags(1)),
        Flag::new("B", MyFlags(1 << 1)),
    ];

    type Bits = u8;

    fn from_bits_retain(bits: Self::Bits) -> Self {
        MyFlags(bits)
    }

    fn bits(&self) -> Self::Bits {
        self.0
    }
}
```

## Using `Flags`

The `Flags` trait can be used generically to work with any flags types. In this example,
we can count the number of defined named flags:

```rust
use bitflags::{bitflags, Flags};
fn defined_flags<F: Flags>() -> usize {
    F::FLAGS.iter().filter(|f| f.is_named()).count()
}

bitflags! {
    struct MyFlags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;

        const _ = !0;
    }
}

assert_eq!(3, defined_flags::<MyFlags>());
```

#### Associated Types

- `type Bits: 1`

#### Associated Constants

- `const FLAGS: &'static [Flag<Self>]`

#### Required Methods

- `fn bits(&self) -> <Self as >::Bits`

  Get the underlying bits value.

- `fn from_bits_retain(bits: <Self as >::Bits) -> Self`

  Convert from a bits value exactly.

#### Provided Methods

- `fn empty() -> Self`

  Get a flags value with all bits unset.

- `fn all() -> Self`

  Get a flags value with all known bits set.

- `fn contains_unknown_bits(&self) -> bool`

  This method will return `true` if any unknown bits are set.

- `fn from_bits(bits: <Self as >::Bits) -> Option<Self>`

  Convert from a bits value.

- `fn from_bits_truncate(bits: <Self as >::Bits) -> Self`

  Convert from a bits value, unsetting any unknown bits.

- `fn from_name(name: &str) -> Option<Self>`

  Get a flags value with the bits of a flag with the given name set.

- `fn iter(&self) -> iter::Iter<Self>`

  Yield a set of contained flags values.

- `fn iter_names(&self) -> iter::IterNames<Self>`

  Yield a set of contained named flags values.

- `fn iter_defined_names() -> iter::IterDefinedNames<Self>`

  Yield a set of all named flags defined by `Self::FLAGS`.

- `fn is_empty(&self) -> bool`

  Whether all bits in this flags value are unset.

- `fn is_all(&self) -> bool`

  Whether all known bits in this flags value are set.

- `fn intersects(&self, other: Self) -> bool`

  Whether any set bits in a source flags value are also set in a target flags value.

- `fn contains(&self, other: Self) -> bool`

  Whether all set bits in a source flags value are also set in a target flags value.

- `fn truncate(&mut self)`

  Remove any unknown bits from the flags.

- `fn insert(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

- `fn remove(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

- `fn toggle(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

- `fn set(&mut self, other: Self, value: bool)`

  Call `Flags::insert` when `value` is `true` or `Flags::remove` when `value` is `false`.

- `fn clear(&mut self)`

  Unsets all bits in the flags.

- `fn intersection(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

- `fn union(self, other: Self) -> Self`

  The bitwise or (`|`) of the bits in two flags values.

- `fn difference(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

- `fn symmetric_difference(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

- `fn complement(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

### `Bits`

```rust
trait Bits: Clone + Copy + PartialEq + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Not<Output = Self> + Sized + 'static { ... }
```

*Defined in [`bitflags-2.10.0/src/traits.rs:346-362`](../../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L346-L362)*

A bits type that can be used as storage for a flags type.

#### Associated Constants

- `const EMPTY: Self`

- `const ALL: Self`

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `Primitive`

```rust
trait Primitive { ... }
```

*Defined in [`bitflags-2.10.0/src/traits.rs:366`](../../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L366)*

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `PublicFlags`

```rust
trait PublicFlags { ... }
```

*Defined in [`bitflags-2.10.0/src/traits.rs:422-428`](../../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L422-L428)*

A trait for referencing the `bitflags`-owned internal type
without exposing it publicly.

#### Associated Types

- `type Primitive: 1`

- `type Internal`

## Macros

### `impl_bits!`

*Defined in [`bitflags-2.10.0/src/traits.rs:368-409`](../../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L368-L409)*

