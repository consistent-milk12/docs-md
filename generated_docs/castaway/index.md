# Crate `castaway`

Safe, zero-cost downcasting for limited compile-time specialization.

This crate works fully on stable Rust, and also does not require the
standard library. To disable references to the standard library, you must
opt-out of the `std` feature using `default-features = false` in your
`Cargo.toml` file. When in no-std mode, a separate `alloc` feature flag
is available to support casting to several [`alloc`](../allocator_api2/index.md) types not included
in [`core`](../clap_builder/output/textwrap/core/index.md).

Castaway provides the following key macros:

- [`cast`](#cast): Attempt to cast the result of an expression into a given
  concrete type.
- [`match_type`](#match-type): Match the result of an expression against multiple
  concrete types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`lifetime_free`](#lifetime-free) | mod |  |
| [`utils`](#utils) | mod | Low-level utility functions. |
| [`LifetimeFree`](#lifetimefree) | trait |  |
| [`cast!`](#cast) | macro | Attempt to cast the result of an expression into a given concrete type. |
| [`match_type!`](#match-type) | macro | Match the result of an expression against multiple concrete types. |

## Modules

- [`lifetime_free`](lifetime_free/index.md)
- [`utils`](utils/index.md) — Low-level utility functions.

## Traits

### `LifetimeFree`

```rust
trait LifetimeFree { ... }
```

*Defined in [`castaway-0.2.4/src/lifetime_free.rs:43`](../../.source_1765633015/castaway-0.2.4/src/lifetime_free.rs#L43)*

Marker trait for types that do not contain any lifetime parameters. Such
types are safe to cast from non-static type parameters if their types are
equal.

This trait is used by [`cast!`](crate::cast) to determine what casts are legal on values
without a `'static` type constraint.

# Safety

When implementing this trait for a type, you must ensure that the type is
free of any lifetime parameters. Failure to meet **all** of the requirements
below may result in undefined behavior.

- The type must be `'static`.
- The type must be free of lifetime parameters. In other words, the type
  must be an "owned" type and not contain *any* lifetime parameters.
- All contained fields must also be `LifetimeFree`.

# Examples

```rust
use castaway::LifetimeFree;

struct Container<T>(T);

// UNDEFINED BEHAVIOR!!
// unsafe impl LifetimeFree for Container<&'static str> {}

// UNDEFINED BEHAVIOR!!
// unsafe impl<T> LifetimeFree for Container<T> {}

// This is safe.
unsafe impl<T: LifetimeFree> LifetimeFree for Container<T> {}

struct PlainOldData {
    foo: u8,
    bar: bool,
}

// This is also safe, since all fields are known to be `LifetimeFree`.
unsafe impl LifetimeFree for PlainOldData {}
```

#### Implementors

- `()`
- `(T0)`
- `(T0, T1)`
- `(T0, T1, T2)`
- `(T0, T1, T2, T3)`
- `(T0, T1, T2, T3, T4)`
- `(T0, T1, T2, T3, T4, T5)`
- `(T0, T1, T2, T3, T4, T5, T6)`
- `(T0, T1, T2, T3, T4, T5, T6, T7)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)`
- `Option<T>`
- `Result<T, E>`
- `[T; SIZE]`
- `[T]`
- `alloc::boxed::Box<T>`
- `alloc::string::String`
- `alloc::sync::Arc<T>`
- `alloc::vec::Vec<T>`
- `bool`
- `char`
- `core::cell::Cell<T>`
- `core::cell::RefCell<T>`
- `core::num::NonZeroI128`
- `core::num::NonZeroI16`
- `core::num::NonZeroI32`
- `core::num::NonZeroI64`
- `core::num::NonZeroI8`
- `core::num::NonZeroIsize`
- `core::num::NonZeroU128`
- `core::num::NonZeroU16`
- `core::num::NonZeroU32`
- `core::num::NonZeroU64`
- `core::num::NonZeroU8`
- `core::num::NonZeroUsize`
- `core::num::Wrapping<T>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Macros

### `cast!`

*Defined in [`castaway-0.2.4/src/lib.rs:177-207`](../../.source_1765633015/castaway-0.2.4/src/lib.rs#L177-L207)*

Attempt to cast the result of an expression into a given concrete type.

If the expression is in fact of the given type, an [`Ok`](#ok) is returned
containing the result of the expression as that type. If the types do not
match, the value is returned in an `Err` unchanged.

This macro is designed to work inside a generic context, and allows you to
downcast generic types to their concrete types or to another generic type at
compile time. If you are looking for the ability to downcast values at
runtime, you should use [`Any`](core::any::Any) instead.

This macro does not perform any sort of type _conversion_ (such as
re-interpreting `i32` as `u32` and so on), it only resolves generic types to
concrete types if the instantiated generic type is exactly the same as the
type you specify. If you are looking to reinterpret the bits of a value as a
type other than the one it actually is, then you should look for a different
library.

Invoking this macro is zero-cost, meaning after normal compiler optimization
steps there will be no code generated in the final binary for performing a
cast. In debug builds some glue code may be present with a small runtime
cost.

# Restrictions

Attempting to perform an illegal or unsupported cast that can never be
successful, such as casting to a value with a longer lifetime than the
expression, will produce a compile-time error.

Due to language limitations with lifetime bounds, this macro is more
restrictive than what is theoretically possible and rejects some legal
casts. This is to ensure safety and correctness around lifetime handling.
Examples include the following:

- Casting an expression by value with a non-`'static` lifetime is not
  allowed. For example, you cannot attempt to cast a `T: 'a` to `Foo<'a>`.
- Casting to a reference with a non-`'static` lifetime is not allowed if the
  expression type is not required to be a reference. For example, you can
  attempt to cast a `&T` to `&String`, but you can't attempt to cast a `T`
  to `&String` because `T` may or may not be a reference. You can, however,
  attempt to cast a `T: 'static` to `&'static String`.
- You cannot cast references whose target itself may contain non-`'static`
  references. For example, you can attempt to cast a `&'a T: 'static` to
  `&'a Foo<'static>`, but you can't attempt to cast a `&'a T: 'b` to `&'a
  Foo<'b>`.
- You can cast generic slices as long as the item type is `'static` and
  `Sized`, but you cannot cast a generic reference to a slice or vice versa.

Some exceptions are made to the above restrictions for certain types which
are known to be _lifetime-free_. You can cast a generic type to any
lifetime-free type by value or by reference, even if the generic type is not
`'static`.

A type is considered lifetime-free if it contains no generic lifetime
bounds, ensuring that all possible instantiations of the type are always
`'static`. To mark a type as being lifetime-free and enable it to be casted
to in this manner by this macro it must implement the [`LifetimeFree`](lifetime_free/index.md)
trait. This is implemented automatically for all primitive types and for
several [`core`](../clap_builder/output/textwrap/core/index.md) types. If you enable the `std` crate feature, then it will
also be implemented for several `std` types as well. If you enable the
`alloc` crate feature, then it will be implemented for several [`alloc`](../allocator_api2/index.md)
types without linking to the standard library as the `std` feature would.

# Examples

The above restrictions are admittedly complex and can be tricky to reason
about, so it is recommended to read the following examples to get a feel for
what is, and what is not, supported.

Performing trivial casts:

```rust
use castaway::cast;

let value: u8 = 0;
assert_eq!(cast!(value, u8), Ok(0));

let slice: &[u8] = &[value];
assert_eq!(cast!(slice, &[u8]), Ok(slice));
```

Performing a cast in a generic context:

```rust
use castaway::cast;

fn is_this_a_u8<T: 'static>(value: T) -> bool {
    cast!(value, u8).is_ok()
}

assert!(is_this_a_u8(0u8));
assert!(!is_this_a_u8(0u16));

// Note that we can also implement this without the `'static` type bound
// because the only type(s) we care about casting to all implement
// `LifetimeFree`:

fn is_this_a_u8_non_static<T>(value: T) -> bool {
    cast!(value, u8).is_ok()
}

assert!(is_this_a_u8_non_static(0u8));
assert!(!is_this_a_u8_non_static(0u16));
```

Specialization in a blanket trait implementation:

```rust
#[cfg(feature = "std")] {
use std::fmt::Display;
use castaway::cast;

/// Like `std::string::ToString`, but with an optimization when `Self` is
/// already a `String`.
///
/// Since the standard library is allowed to use unstable features,
/// `ToString` already has this optimization using the `specialization`
/// feature, but this isn't something normal crates can do.
pub trait FastToString {
    fn fast_to_string(&self) -> String;
}

impl<T: Display> FastToString for T {
    fn fast_to_string<'local>(&'local self) -> String {
        // If `T` is already a string, then take a different code path.
        // After monomorphization, this check will be completely optimized
        // away.
        //
        // Note we can cast a `&'local self` to a `&'local String` as `String`
        // implements `LifetimeFree`.
        if let Ok(string) = cast!(self, &String) {
            // Don't invoke the std::fmt machinery, just clone the string.
            string.to_owned()
        } else {
            // Make use of `Display` for any other `T`.
            format!("{}", self)
        }
    }
}

println!("specialized: {}", String::from("hello").fast_to_string());
println!("default: {}", "hello".fast_to_string());
}
```

### `match_type!`

*Defined in [`castaway-0.2.4/src/lib.rs:263-285`](../../.source_1765633015/castaway-0.2.4/src/lib.rs#L263-L285)*

Match the result of an expression against multiple concrete types.

You can write multiple match arms in the following syntax:

```no_compile
TYPE as name => { /* expression */ }
```

If the concrete type matches the given type, then the value will be cast to
that type and bound to the given variable name. The expression on the
right-hand side of the match is then executed and returned as the result of
the entire match expression.

The name following the `as` keyword can be any [irrefutable
pattern](https://doc.rust-lang.org/stable/reference/patterns.html#refutability).
Like `match` or `let` expressions, you can use an underscore to prevent
warnings if you don't use the casted value, such as `_value` or just `_`.

Since it would be impossible to exhaustively list all possible types of an
expression, you **must** include a final default match arm. The default
match arm does not specify a type:

```no_compile
name => { /* expression */ }
```

The original expression will be bound to the given variable name without
being casted. If you don't care about the original value, the default arm
can be:

```no_compile
_ => { /* expression */ }
```

This macro has all the same rules and restrictions around type casting as
[`cast`](#cast).

# Examples

```rust
use std::fmt::Display;
use castaway::match_type;

fn to_string<T: Display + 'static>(value: T) -> String {
    match_type!(value, {
        String as s => s,
        &str as s => s.to_string(),
        s => s.to_string(),
    })
}

println!("{}", to_string("foo"));
```

