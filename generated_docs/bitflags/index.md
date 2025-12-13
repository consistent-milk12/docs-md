# Crate `bitflags`

Generate types for C-style flags with ergonomic APIs.

# Getting started

Add `bitflags` to your `Cargo.toml`:

```toml
[dependencies.bitflags]
version = "2.10.0"
```

## Crate features

The `bitflags` library defines a few Cargo features that you can opt-in to:

- `std`: Implement the `Error` trait on error types used by `bitflags`.
- `serde`: Support deriving `serde` traits on generated flags types.
- `arbitrary`: Support deriving `arbitrary` traits on generated flags types.
- `bytemuck`: Support deriving `bytemuck` traits on generated flags types.

## Generating flags types

Use the [`bitflags`](#bitflags) macro to generate flags types:

```rust
use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
```

See the docs for the `bitflags` macro for the full syntax.

Also see the [`example_generated`](./example_generated/index.html) module for an example of what the `bitflags` macro generates for a flags type.

### Externally defined flags

If you're generating flags types for an external source, such as a C API, you can define
an extra unnamed flag as a mask of all bits the external source may ever set. Usually this would be all bits (`!0`):

```rust
use bitflags::bitflags;
bitflags! {
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;

        // The source may set any bits
        const _ = !0;
    }
}
```

Why should you do this? Generated methods like `all` and truncating operators like `!` only consider
bits in defined flags. Adding an unnamed flag makes those methods consider additional bits,
without generating additional constants for them. It helps compatibility when the external source
may start setting additional bits at any time. The [known and unknown bits](#known-and-unknown-bits)
section has more details on this behavior.

### Custom derives

You can derive some traits on generated flags types if you enable Cargo features. The following
libraries are currently supported:

- `serde`: Support `#[derive(Serialize, Deserialize)]`, using text for human-readable formats,
  and a raw number for binary formats.
- `arbitrary`: Support `#[derive(Arbitrary)]`, only generating flags values with known bits.
- `bytemuck`: Support `#[derive(Pod, Zeroable)]`, for casting between flags values and their
  underlying bits values.

You can also define your own flags type outside of the [`bitflags`](#bitflags) macro and then use it to generate methods.
This can be useful if you need a custom `#[derive]` attribute for a library that `bitflags` doesn't
natively support:

```rust
use std::fmt::Debug as SomeTrait;
use bitflags::bitflags;
#[derive(SomeTrait)]
pub struct Flags(u32);

bitflags! {
    impl Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
```

### Adding custom methods

The [`bitflags`](#bitflags) macro supports attributes on generated flags types within the macro itself, while
`impl` blocks can be added outside of it:

```rust
use bitflags::bitflags;
bitflags! {
    // Attributes can be applied to flags types
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}

// Impl blocks can be added to flags types
impl Flags {
    pub fn as_u64(&self) -> u64 {
        self.bits() as u64
    }
}
```

## Working with flags values

Use generated constants and standard bitwise operators to interact with flags values:

```rust
use bitflags::bitflags;
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}
// union
let ab = Flags::A | Flags::B;

// intersection
let a = ab & Flags::A;

// difference
let b = ab - Flags::A;

// complement
let c = !ab;
```

See the docs for the [`Flags`](traits/index.md) trait for more details on operators and how they behave.

# Formatting and parsing

`bitflags` defines a text format that can be used to convert any flags value to and from strings.

See the [`parser`](parser/index.md) module for more details.

# Specification

The terminology and behavior of generated flags types is
[specified in the source repository](https://github.com/bitflags/bitflags/blob/main/spec.md).
Details are repeated in these docs where appropriate, but is exhaustively listed in the spec. Some
things are worth calling out explicitly here.

## Flags types, flags values, flags

The spec and these docs use consistent terminology to refer to things in the bitflags domain:

- **Bits type**: A type that defines a fixed number of bits at specific locations.
- **Flag**: A set of bits in a bits type that may have a unique name.
- **Flags type**: A set of defined flags over a specific bits type.
- **Flags value**: An instance of a flags type using its specific bits value for storage.

```rust
use bitflags::bitflags;
bitflags! {
    struct FlagsType: u8 {
//                    -- Bits type
//         --------- Flags type
        const A = 1;
//            ----- Flag
    }
}

let flag = FlagsType::A;
//  ---- Flags value
```

## Known and unknown bits

Any bits in a flag you define are called _known bits_. Any other bits are _unknown bits_.
In the following flags type:

```rust
use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}
```

The known bits are `0b0000_0111` and the unknown bits are `0b1111_1000`.

`bitflags` doesn't guarantee that a flags value will only ever have known bits set, but some operators
will unset any unknown bits they encounter. In a future version of `bitflags`, all operators will
unset unknown bits.

If you're using `bitflags` for flags types defined externally, such as from C, you probably want all
bits to be considered known, in case that external source changes. You can do this using an unnamed
flag, as described in [externally defined flags](#externally-defined-flags).

## Zero-bit flags

Flags with no bits set should be avoided because they interact strangely with `Flags::contains`
and `Flags::intersects`. A zero-bit flag is always contained, but is never intersected. The
names of zero-bit flags can be parsed, but are never formatted.

## Multi-bit flags

Flags that set multiple bits should be avoided unless each bit is also in a single-bit flag.
Take the following flags type as an example:

```rust
use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 | 1 << 1;
    }
}
```

The result of `Flags::A ^ Flags::B` is `0b0000_0010`, which doesn't correspond to either
`Flags::A` or `Flags::B` even though it's still a known bit.

## Contents

- [Modules](#modules)
  - [`iter`](#iter)
  - [`parser`](#parser)
  - [`traits`](#traits)
  - [`public`](#public)
  - [`internal`](#internal)
  - [`external`](#external)
  - [`__private`](#private)
- [Structs](#structs)
  - [`Flag`](#flag)
- [Traits](#traits)
  - [`Bits`](#bits)
  - [`Flags`](#flags)
- [Macros](#macros)
  - [`bitflags!`](#bitflags)
  - [`bitflags_match!`](#bitflags-match)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`iter`](#iter) | mod | Yield the bits of a source flags value in a set of contained flags values. |
| [`parser`](#parser) | mod | Parsing flags from text. |
| [`traits`](#traits) | mod |  |
| [`public`](#public) | mod | Generate the user-facing flags type. |
| [`internal`](#internal) | mod | Generate the internal `bitflags`-facing flags type. |
| [`external`](#external) | mod | Conditional trait implementations for external libraries. |
| [`__private`](#private) | mod |  |
| [`Flag`](#flag) | struct |  |
| [`Bits`](#bits) | trait |  |
| [`Flags`](#flags) | trait |  |
| [`bitflags!`](#bitflags) | macro | Generate a flags type. |
| [`bitflags_match!`](#bitflags-match) | macro | A macro that matches flags values, similar to Rust's `match` statement. |

## Modules

- [`iter`](iter/index.md) — Yield the bits of a source flags value in a set of contained flags values.
- [`parser`](parser/index.md) — Parsing flags from text.
- [`traits`](traits/index.md)
- [`public`](public/index.md) — Generate the user-facing flags type.
- [`internal`](internal/index.md) — Generate the internal `bitflags`-facing flags type.
- [`external`](external/index.md) — Conditional trait implementations for external libraries.
- [`__private`](__private/index.md)

## Structs

### `Flag<B>`

```rust
struct Flag<B> {
    name: &'static str,
    value: B,
}
```

*Defined in [`bitflags-2.10.0/src/traits.rs:15-18`](../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L15-L18)*

A defined flags value that may be named or unnamed.

#### Implementations

- <span id="flag-new"></span>`const fn new(name: &'static str, value: B) -> Self`

  Define a flag.

  

  If `name` is non-empty then the flag is named, otherwise it's unnamed.

      

- <span id="flag-name"></span>`const fn name(&self) -> &'static str`

  Get the name of this flag.

  

  If the flag is unnamed then the returned string will be empty.

      

- <span id="flag-value"></span>`const fn value(&self) -> &B`

  Get the flags value of this flag.

      

- <span id="flag-is-named"></span>`const fn is_named(&self) -> bool`

  Whether the flag is named.

  

  If `Flag::name` returns a non-empty string then this method will return `true`.

      

- <span id="flag-is-unnamed"></span>`const fn is_unnamed(&self) -> bool`

  Whether the flag is unnamed.

  

  If `Flag::name` returns a non-empty string then this method will return `false`.

      

#### Trait Implementations

##### `impl Any for Flag<B>`

- <span id="flag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flag<B>`

- <span id="flag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flag<B>`

- <span id="flag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<B: fmt::Debug> Debug for Flag<B>`

- <span id="flag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Flag<B>`

- <span id="flag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flag<B>`

- <span id="flag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Flag<B>`

- <span id="flag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flag<B>`

- <span id="flag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Bits`

```rust
trait Bits: Clone + Copy + PartialEq + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Not<Output = Self> + Sized + 'static { ... }
```

*Defined in [`bitflags-2.10.0/src/traits.rs:346-362`](../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L346-L362)*

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

### `Flags`

```rust
trait Flags: Sized + 'static { ... }
```

*Defined in [`bitflags-2.10.0/src/traits.rs:132-341`](../../.source_1765521767/bitflags-2.10.0/src/traits.rs#L132-L341)*

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

## Macros

### `bitflags!`

*Defined in [`bitflags-2.10.0/src/lib.rs:456-597`](../../.source_1765521767/bitflags-2.10.0/src/lib.rs#L456-L597)*

Generate a flags type.

# `struct` mode

A declaration that begins with `$vis struct` will generate a `struct` for a flags type, along with
methods and trait implementations for it. The body of the declaration defines flags as constants,
where each constant is a flags value of the generated flags type.

## Examples

Generate a flags type using `u8` as the bits type:

```rust
use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 0b0000_0100;
    }
}
```

Flags types are private by default and accept standard visibility modifiers. Flags themselves
are always public:

```rust
use bitflags::bitflags;
bitflags! {
    pub struct Flags: u8 {
        // Constants are always `pub`
        const A = 1;
    }
}
```

Flags may refer to other flags using their `Flags::bits` value:

```rust
use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const AB = Flags::A.bits() | Flags::B.bits();
    }
}
```

A single `bitflags` invocation may include zero or more flags type declarations:

```rust
use bitflags::bitflags;
bitflags! {}

bitflags! {
    struct Flags1: u8 {
        const A = 1;
    }

    struct Flags2: u8 {
        const A = 1;
    }
}
```

# `impl` mode

A declaration that begins with `impl` will only generate methods and trait implementations for the
`struct` defined outside of the `bitflags` macro.

The struct itself must be a newtype using the bits type as its field.

The syntax for `impl` mode is identical to `struct` mode besides the starting token.

## Examples

Implement flags methods and traits for a custom flags type using `u8` as its underlying bits type:

```rust
use bitflags::bitflags;
struct Flags(u8);

bitflags! {
    impl Flags: u8 {
        const A = 1;
        const B = 1 << 1;
        const C = 0b0000_0100;
    }
}
```

# Named and unnamed flags

Constants in the body of a declaration are flags. The identifier of the constant is the name of
the flag. If the identifier is `_`, then the flag is unnamed. Unnamed flags don't appear in the
generated API, but affect how bits are truncated.

## Examples

Adding an unnamed flag that makes all bits known:

```rust
use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const A = 1;
        const B = 1 << 1;

        const _ = !0;
    }
}
```

Flags types may define multiple unnamed flags:

```rust
use bitflags::bitflags;
bitflags! {
    struct Flags: u8 {
        const _ = 1;
        const _ = 1 << 1;
    }
}
```

### `bitflags_match!`

*Defined in [`bitflags-2.10.0/src/lib.rs:817-827`](../../.source_1765521767/bitflags-2.10.0/src/lib.rs#L817-L827)*

A macro that matches flags values, similar to Rust's `match` statement.

In a regular `match` statement, the syntax `Flag::A | Flag::B` is interpreted as an or-pattern,
instead of the bitwise-or of `Flag::A` and `Flag::B`. This can be surprising when combined with flags types
because `Flag::A | Flag::B` won't match the pattern `Flag::A | Flag::B`. This macro is an alternative to
`match` for flags values that doesn't have this issue.

# Syntax

```ignore
bitflags_match!(expression, {
    pattern1 => result1,
    pattern2 => result2,
    ..
    _ => default_result,
})
```

The final `_ => default_result` arm is required, otherwise the macro will fail to compile.

# Examples

```rust
use bitflags::{bitflags, bitflags_match};

bitflags! {
    #[derive(PartialEq)]
    struct Flags: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}

let flags = Flags::A | Flags::B;

// Prints `the value is A and B`
bitflags_match!(flags, {
    Flags::A | Flags::B => println!("the value is A and B"),
    _ => println!("the value is not A and B"),
});

// Prints `the value is not A`
bitflags_match!(flags, {
    Flags::A => println!("the value is A"),
    _ => println!("the value is not A"),
});
```

# How it works

The macro expands to a series of `if` statements, **checking equality** between the input expression
and each pattern. This allows for correct matching of bitflag combinations, which is not possible
with a regular match expression due to the way bitflags are implemented.

Patterns are evaluated in the order they appear in the macro.

