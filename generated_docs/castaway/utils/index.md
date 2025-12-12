*[castaway](../index.md) / [utils](index.md)*

---

# Module `utils`

Low-level utility functions.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`type_eq`](#type-eq) | fn | Determine if two static, generic types are equal to each other. |
| [`type_eq_non_static`](#type-eq-non-static) | fn | Determine if two generic types which may not be static are equal to each other. |
| [`non_static_type_id`](#non-static-type-id) | fn | Produces type IDs that are compatible with `TypeId::of::<T>`, but without `T: 'static` bound. |
| [`transmute_unchecked`](#transmute-unchecked) | fn | Reinterprets the bits of a value of one type as another type. |

## Functions

### `type_eq`

```rust
fn type_eq<T: 'static, U: 'static>() -> bool
```

*Defined in [`castaway-0.2.4/src/utils.rs:11-21`](../../../.source_1765521767/castaway-0.2.4/src/utils.rs#L11-L21)*

Determine if two static, generic types are equal to each other.

### `type_eq_non_static`

```rust
fn type_eq_non_static<T: ?Sized, U: ?Sized>() -> bool
```

*Defined in [`castaway-0.2.4/src/utils.rs:30-32`](../../../.source_1765521767/castaway-0.2.4/src/utils.rs#L30-L32)*

Determine if two generic types which may not be static are equal to each
other.

This function must be used with extreme discretion, as no lifetime checking
is done. Meaning, this function considers `Struct<'a>` to be equal to
`Struct<'b>`, even if either `'a` or `'b` outlives the other.

### `non_static_type_id`

```rust
fn non_static_type_id<T: ?Sized>() -> core::any::TypeId
```

*Defined in [`castaway-0.2.4/src/utils.rs:36-56`](../../../.source_1765521767/castaway-0.2.4/src/utils.rs#L36-L56)*

Produces type IDs that are compatible with `TypeId::of::<T>`, but without
`T: 'static` bound.

### `transmute_unchecked`

```rust
unsafe fn transmute_unchecked<T, U>(value: T) -> U
```

*Defined in [`castaway-0.2.4/src/utils.rs:78-93`](../../../.source_1765521767/castaway-0.2.4/src/utils.rs#L78-L93)*

Reinterprets the bits of a value of one type as another type.

Similar to `std::mem::transmute`, except that it makes no compile-time
guarantees about the layout of `T` or `U`, and is therefore even **more**
dangerous than `transmute`. Extreme caution must be taken when using this
function; it is up to the caller to assert that `T` and `U` have the same
size and layout and that it is safe to do this conversion. Which it probably
isn't, unless `T` and `U` are identical.

# Panics

This function panics if `T` and `U` have different sizes.

# Safety

It is up to the caller to uphold the following invariants:

- `T` must have the same alignment as `U`
- `T` must be safe to transmute into `U`

