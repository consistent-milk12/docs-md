*[ryu](../index.md) / [raw](index.md)*

---

# Module `raw`

Unsafe functions that mirror the API of the C implementation of Ryū.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`format32`](#format32) | fn |  |
| [`format64`](#format64) | fn |  |

## Functions

### `format32`

```rust
unsafe fn format32(f: f32, result: *mut u8) -> usize
```

*Defined in [`ryu-1.0.20/src/pretty/mod.rs:159-224`](../../../.source_1765210505/ryu-1.0.20/src/pretty/mod.rs#L159-L224)*

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

### `format64`

```rust
unsafe fn format64(f: f64, result: *mut u8) -> usize
```

*Defined in [`ryu-1.0.20/src/pretty/mod.rs:52-118`](../../../.source_1765210505/ryu-1.0.20/src/pretty/mod.rs#L52-L118)*

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

