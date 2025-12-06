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

## Modules

- [`iter`](iter/index.md) - Yield the bits of a source flags value in a set of contained flags values.
- [`parser`](parser/index.md) - Parsing flags from text.

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

## Macros

### `bitflags!`

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

