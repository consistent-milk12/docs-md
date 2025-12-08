*[ryu](../index.md) / [pretty](index.md)*

---

# Module `pretty`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`exponent`](#exponent) | mod |  |
| [`mantissa`](#mantissa) | mod |  |
| [`format64`](#format64) | fn | Print f64 to the given buffer and return number of bytes written. |
| [`format32`](#format32) | fn | Print f32 to the given buffer and return number of bytes written. |

## Modules

- [`exponent`](exponent/index.md) - 
- [`mantissa`](mantissa/index.md) - 

## Functions

### `format64`

```rust
unsafe fn format64(f: f64, result: *mut u8) -> usize
```

Print f64 to the given buffer and return number of bytes written.

At most 24 bytes will be written.

## Special cases

This function **does not** check for NaN or infinity. If the input
number is not a finite float, the printed representation will be some
correctly formatted but unspecified numerical value.

Please check `is_finite` yourself before calling this function, or
check `is_nan` and `is_infinite` and handle those cases yourself.



## Safety

The `result` pointer argument must point to sufficiently many writable bytes
to hold Ryū's representation of `f`.

## Example

```rust
use std::{mem::MaybeUninit, slice, str};

let f = 1.234f64;

unsafe {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
    let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
    let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let print = str::from_utf8_unchecked(slice);
    assert_eq!(print, "1.234");
}
```

### `format32`

```rust
unsafe fn format32(f: f32, result: *mut u8) -> usize
```

Print f32 to the given buffer and return number of bytes written.

At most 16 bytes will be written.

## Special cases

This function **does not** check for NaN or infinity. If the input
number is not a finite float, the printed representation will be some
correctly formatted but unspecified numerical value.

Please check `is_finite` yourself before calling this function, or
check `is_nan` and `is_infinite` and handle those cases yourself.



## Safety

The `result` pointer argument must point to sufficiently many writable bytes
to hold Ryū's representation of `f`.

## Example

```rust
use std::{mem::MaybeUninit, slice, str};

let f = 1.234f32;

unsafe {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
    let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
    let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
    let print = str::from_utf8_unchecked(slice);
    assert_eq!(print, "1.234");
}
```

