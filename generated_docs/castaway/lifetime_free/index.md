*[castaway](../index.md) / [lifetime_free](index.md)*

---

# Module `lifetime_free`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alloc_impls`](#alloc-impls) | mod |  |
| [`LifetimeFree`](#lifetimefree) | trait | Marker trait for types that do not contain any lifetime parameters. |
| [`tuple_impls!`](#tuple-impls) | macro |  |

## Modules

- [`alloc_impls`](alloc_impls/index.md)

## Traits

### `LifetimeFree`

```rust
trait LifetimeFree { ... }
```

*Defined in [`castaway-0.2.4/src/lifetime_free.rs:43`](../../../.source_1765633015/castaway-0.2.4/src/lifetime_free.rs#L43)*

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

### `tuple_impls!`

*Defined in [`castaway-0.2.4/src/lifetime_free.rs:86-92`](../../../.source_1765633015/castaway-0.2.4/src/lifetime_free.rs#L86-L92)*

