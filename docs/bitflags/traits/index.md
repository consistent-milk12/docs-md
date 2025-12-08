*[bitflags](../index.md) / [traits](index.md)*

---

# Module `traits`

## Modules

- [`__private`](__private/index.md) - 

## Structs

### `Flag<B>`

```rust
struct Flag<B> {
    name: &'static str,
    value: B,
}
```

A defined flags value that may be named or unnamed.

#### Implementations

- `const fn new(name: &'static str, value: B) -> Self`

- `const fn name(self: &Self) -> &'static str`

- `const fn value(self: &Self) -> &B`

- `const fn is_named(self: &Self) -> bool`

- `const fn is_unnamed(self: &Self) -> bool`

#### Trait Implementations

##### `impl<B: $crate::fmt::Debug> Debug for Flag<B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Flags`

```rust
trait Flags: Sized + 'static { ... }
```

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

#### Required Methods

- `const FLAGS: &'static [Flag<Self>]`

- `type Bits: 1`

- `fn empty() -> Self`

  Get a flags value with all bits unset.

- `fn all() -> Self`

  Get a flags value with all known bits set.

- `fn contains_unknown_bits(self: &Self) -> bool`

  This method will return `true` if any unknown bits are set.

- `fn bits(self: &Self) -> <Self as >::Bits`

  Get the underlying bits value.

- `fn from_bits(bits: <Self as >::Bits) -> Option<Self>`

  Convert from a bits value.

- `fn from_bits_truncate(bits: <Self as >::Bits) -> Self`

  Convert from a bits value, unsetting any unknown bits.

- `fn from_bits_retain(bits: <Self as >::Bits) -> Self`

  Convert from a bits value exactly.

- `fn from_name(name: &str) -> Option<Self>`

  Get a flags value with the bits of a flag with the given name set.

- `fn iter(self: &Self) -> iter::Iter<Self>`

  Yield a set of contained flags values.

- `fn iter_names(self: &Self) -> iter::IterNames<Self>`

  Yield a set of contained named flags values.

- `fn iter_defined_names() -> iter::IterDefinedNames<Self>`

  Yield a set of all named flags defined by `Self::FLAGS`.

- `fn is_empty(self: &Self) -> bool`

  Whether all bits in this flags value are unset.

- `fn is_all(self: &Self) -> bool`

  Whether all known bits in this flags value are set.

- `fn intersects(self: &Self, other: Self) -> bool`

  Whether any set bits in a source flags value are also set in a target flags value.

- `fn contains(self: &Self, other: Self) -> bool`

  Whether all set bits in a source flags value are also set in a target flags value.

- `fn truncate(self: &mut Self)`

  Remove any unknown bits from the flags.

- `fn insert(self: &mut Self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

- `fn remove(self: &mut Self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

- `fn toggle(self: &mut Self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

- `fn set(self: &mut Self, other: Self, value: bool)`

  Call `Flags::insert` when `value` is `true` or `Flags::remove` when `value` is `false`.

- `fn clear(self: &mut Self)`

  Unsets all bits in the flags.

- `fn intersection(self: Self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

- `fn union(self: Self, other: Self) -> Self`

  The bitwise or (`|`) of the bits in two flags values.

- `fn difference(self: Self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

- `fn symmetric_difference(self: Self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

- `fn complement(self: Self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

### `Bits`

```rust
trait Bits: Clone + Copy + PartialEq + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Not<Output = Self> + Sized + 'static { ... }
```

A bits type that can be used as storage for a flags type.

#### Required Methods

- `const EMPTY: Self`

- `const ALL: Self`

### `Primitive`

```rust
trait Primitive { ... }
```

### `PublicFlags`

```rust
trait PublicFlags { ... }
```

A trait for referencing the `bitflags`-owned internal type
without exposing it publicly.

#### Required Methods

- `type Primitive: 1`

- `type Internal`

## Macros

### `impl_bits!`

