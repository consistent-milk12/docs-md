*[rustix](../index.md) / [utils](index.md)*

---

# Module `utils`

Miscellaneous minor utilities.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`as_ptr`](#as_ptr) | fn | Convert a `&T` into a `*const T` without using an `as`. |
| [`as_mut_ptr`](#as_mut_ptr) | fn | Convert a `&mut T` into a `*mut T` without using an `as`. |
| [`option_as_ptr`](#option_as_ptr) | fn | Convert an `Option<&T>` into a possibly-null `*const T`. |
| [`option_as_mut_ptr`](#option_as_mut_ptr) | fn | Convert an `Option<&mut T>` into a possibly-null `*mut T`. |
| [`check_raw_pointer`](#check_raw_pointer) | fn | Convert a `*mut c_void` to a `*mut T`, checking that it is not null |
| [`default_union!`](#default_union) | macro | Create a union value containing a default value in one of its arms. |

## Functions

### `as_ptr`

```rust
const fn as_ptr<T>(t: &T) -> *const T
```

Convert a `&T` into a `*const T` without using an `as`.

### `as_mut_ptr`

```rust
fn as_mut_ptr<T>(t: &mut T) -> *mut T
```

Convert a `&mut T` into a `*mut T` without using an `as`.

### `option_as_ptr`

```rust
const fn option_as_ptr<T>(t: Option<&T>) -> *const T
```

Convert an `Option<&T>` into a possibly-null `*const T`.

### `option_as_mut_ptr`

```rust
fn option_as_mut_ptr<T>(t: Option<&mut T>) -> *mut T
```

Convert an `Option<&mut T>` into a possibly-null `*mut T`.

### `check_raw_pointer`

```rust
fn check_raw_pointer<T>(value: *mut core::ffi::c_void) -> Option<core::ptr::NonNull<T>>
```

Convert a `*mut c_void` to a `*mut T`, checking that it is not null,
misaligned, or pointing to a region of memory that wraps around the address
space.

## Macros

### `default_union!`

Create a union value containing a default value in one of its arms.

The field names a union field which must have the same size as the union
itself.

