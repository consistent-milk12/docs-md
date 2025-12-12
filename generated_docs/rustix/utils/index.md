*[rustix](../index.md) / [utils](index.md)*

---

# Module `utils`

Miscellaneous minor utilities.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`as_ptr`](#as-ptr) | fn | Convert a `&T` into a `*const T` without using an `as`. |
| [`as_mut_ptr`](#as-mut-ptr) | fn | Convert a `&mut T` into a `*mut T` without using an `as`. |
| [`option_as_ptr`](#option-as-ptr) | fn | Convert an `Option<&T>` into a possibly-null `*const T`. |
| [`option_as_mut_ptr`](#option-as-mut-ptr) | fn | Convert an `Option<&mut T>` into a possibly-null `*mut T`. |
| [`check_raw_pointer`](#check-raw-pointer) | fn | Convert a `*mut c_void` to a `*mut T`, checking that it is not null, misaligned, or pointing to a region of memory that wraps around the address space. |
| [`default_union!`](#default-union) | macro | Create a union value containing a default value in one of its arms. |

## Functions

### `as_ptr`

```rust
const fn as_ptr<T>(t: &T) -> *const T
```

*Defined in [`rustix-1.1.2/src/utils.rs:12-14`](../../../.source_1765210505/rustix-1.1.2/src/utils.rs#L12-L14)*

Convert a `&T` into a `*const T` without using an `as`.

### `as_mut_ptr`

```rust
fn as_mut_ptr<T>(t: &mut T) -> *mut T
```

*Defined in [`rustix-1.1.2/src/utils.rs:18-20`](../../../.source_1765210505/rustix-1.1.2/src/utils.rs#L18-L20)*

Convert a `&mut T` into a `*mut T` without using an `as`.

### `option_as_ptr`

```rust
const fn option_as_ptr<T>(t: Option<&T>) -> *const T
```

*Defined in [`rustix-1.1.2/src/utils.rs:24-29`](../../../.source_1765210505/rustix-1.1.2/src/utils.rs#L24-L29)*

Convert an `Option<&T>` into a possibly-null `*const T`.

### `option_as_mut_ptr`

```rust
fn option_as_mut_ptr<T>(t: Option<&mut T>) -> *mut T
```

*Defined in [`rustix-1.1.2/src/utils.rs:33-38`](../../../.source_1765210505/rustix-1.1.2/src/utils.rs#L33-L38)*

Convert an `Option<&mut T>` into a possibly-null `*mut T`.

### `check_raw_pointer`

```rust
fn check_raw_pointer<T>(value: *mut core::ffi::c_void) -> Option<core::ptr::NonNull<T>>
```

*Defined in [`rustix-1.1.2/src/utils.rs:43-51`](../../../.source_1765210505/rustix-1.1.2/src/utils.rs#L43-L51)*

Convert a `*mut c_void` to a `*mut T`, checking that it is not null,
misaligned, or pointing to a region of memory that wraps around the address
space.

## Macros

### `default_union!`

*Defined in [`rustix-1.1.2/src/utils.rs:57-76`](../../../.source_1765210505/rustix-1.1.2/src/utils.rs#L57-L76)*

Create a union value containing a default value in one of its arms.

The field names a union field which must have the same size as the union
itself.

