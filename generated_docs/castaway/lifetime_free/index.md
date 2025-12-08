*[castaway](../index.md) / [lifetime_free](index.md)*

---

# Module `lifetime_free`

## Modules

- [`alloc_impls`](alloc_impls/index.md) - 

## Traits

### `LifetimeFree`

```rust
trait LifetimeFree { ... }
```

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

## Macros

### `tuple_impls!`

