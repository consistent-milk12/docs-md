*[rustix](../../index.md) / [backend](../index.md) / [conv](index.md)*

---

# Module `conv`

Convert values to [`ArgReg`](../reg/index.md) and from [`RetReg`](../reg/index.md).

System call arguments and return values are all communicated with inline
asm and FFI as `*mut Opaque`. To protect these raw pointers from escaping
or being accidentally misused as they travel through the code, we wrap them
in [`ArgReg`](../reg/index.md) and [`RetReg`](../reg/index.md) structs. This file provides `From`
implementations and explicit conversion functions for converting values
into and out of these wrapper structs.

# Safety

Some of this code is `unsafe` in order to work with raw file descriptors,
and some is `unsafe` to interpret the values in a `RetReg`.

## Contents

- [Functions](#functions)
  - [`zero`](#zero)
  - [`size_of`](#size-of)
  - [`pass_usize`](#pass-usize)
  - [`raw_fd`](#raw-fd)
  - [`no_fd`](#no-fd)
  - [`slice_just_addr`](#slice-just-addr)
  - [`slice_just_addr_mut`](#slice-just-addr-mut)
  - [`slice`](#slice)
  - [`slice_mut`](#slice-mut)
  - [`by_ref`](#by-ref)
  - [`by_mut`](#by-mut)
  - [`opt_mut`](#opt-mut)
  - [`opt_ref`](#opt-ref)
  - [`c_int`](#c-int)
  - [`c_uint`](#c-uint)
  - [`loff_t`](#loff-t)
  - [`loff_t_from_u64`](#loff-t-from-u64)
  - [`dev_t`](#dev-t)
  - [`ret`](#ret)
  - [`ret_infallible`](#ret-infallible)
  - [`ret_c_int`](#ret-c-int)
  - [`ret_c_uint`](#ret-c-uint)
  - [`ret_u64`](#ret-u64)
  - [`ret_usize`](#ret-usize)
  - [`ret_usize_infallible`](#ret-usize-infallible)
  - [`ret_c_int_infallible`](#ret-c-int-infallible)
  - [`ret_c_uint_infallible`](#ret-c-uint-infallible)
  - [`ret_owned_fd`](#ret-owned-fd)
  - [`ret_discarded_fd`](#ret-discarded-fd)
  - [`ret_void_star`](#ret-void-star)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`zero`](#zero) | fn | Pass a zero, or null, argument. |
| [`size_of`](#size-of) | fn | Pass the `mem::size_of` of a type. |
| [`pass_usize`](#pass-usize) | fn | Pass an arbitrary `usize` value. |
| [`raw_fd`](#raw-fd) | fn | Pass a raw file-descriptor argument. |
| [`no_fd`](#no-fd) | fn | Deliberately pass `-1` to a file-descriptor argument, for system calls like `mmap` where this indicates the argument is omitted. |
| [`slice_just_addr`](#slice-just-addr) | fn |  |
| [`slice_just_addr_mut`](#slice-just-addr-mut) | fn |  |
| [`slice`](#slice) | fn |  |
| [`slice_mut`](#slice-mut) | fn |  |
| [`by_ref`](#by-ref) | fn |  |
| [`by_mut`](#by-mut) | fn |  |
| [`opt_mut`](#opt-mut) | fn | Convert an optional mutable reference into a `usize` for passing to a syscall. |
| [`opt_ref`](#opt-ref) | fn | Convert an optional immutable reference into a `usize` for passing to a syscall. |
| [`c_int`](#c-int) | fn | Convert a `c_int` into an `ArgReg`. |
| [`c_uint`](#c-uint) | fn | Convert a `c_uint` into an `ArgReg`. |
| [`loff_t`](#loff-t) | fn |  |
| [`loff_t_from_u64`](#loff-t-from-u64) | fn |  |
| [`dev_t`](#dev-t) | fn |  |
| [`ret`](#ret) | fn | Convert a `usize` returned from a syscall that effectively returns `()` on success. |
| [`ret_infallible`](#ret-infallible) | fn | Convert a `usize` returned from a syscall that effectively always returns `()`. |
| [`ret_c_int`](#ret-c-int) | fn | Convert a `usize` returned from a syscall that effectively returns a `c_int` on success. |
| [`ret_c_uint`](#ret-c-uint) | fn | Convert a `usize` returned from a syscall that effectively returns a `c_uint` on success. |
| [`ret_u64`](#ret-u64) | fn | Convert a `usize` returned from a syscall that effectively returns a `u64` on success. |
| [`ret_usize`](#ret-usize) | fn | Convert a `usize` returned from a syscall that effectively returns a `usize` on success. |
| [`ret_usize_infallible`](#ret-usize-infallible) | fn | Convert a `usize` returned from a syscall that effectively always returns a `usize`. |
| [`ret_c_int_infallible`](#ret-c-int-infallible) | fn | Convert a `c_int` returned from a syscall that effectively always returns a `c_int`. |
| [`ret_c_uint_infallible`](#ret-c-uint-infallible) | fn | Convert a `c_uint` returned from a syscall that effectively always returns a `c_uint`. |
| [`ret_owned_fd`](#ret-owned-fd) | fn | Convert a `usize` returned from a syscall that effectively returns an `OwnedFd` on success. |
| [`ret_discarded_fd`](#ret-discarded-fd) | fn | Convert the return value of `dup2` and `dup3`. |
| [`ret_void_star`](#ret-void-star) | fn | Convert a `usize` returned from a syscall that effectively returns a `*mut c_void` on success. |

## Functions

### `zero`

```rust
fn zero<'a, Num: ArgNumber>() -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:89-91`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L89-L91)*

Pass a zero, or null, argument.

### `size_of`

```rust
fn size_of<'a, T: Sized, Num: ArgNumber>() -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:95-97`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L95-L97)*

Pass the `mem::size_of` of a type.

### `pass_usize`

```rust
fn pass_usize<'a, Num: ArgNumber>(t: usize) -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:104-106`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L104-L106)*

Pass an arbitrary `usize` value.

For passing pointers, use `void_star` or other functions which take a raw
pointer instead of casting to `usize`, so that provenance is preserved.

### `raw_fd`

```rust
unsafe fn raw_fd<'a, Num: ArgNumber>(fd: super::fd::RawFd) -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:162-176`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L162-L176)*

Pass a raw file-descriptor argument. Most users should use `ArgReg::from`
instead, to preserve I/O safety as long as possible.

# Safety

`fd` must be a valid open file descriptor.

### `no_fd`

```rust
fn no_fd<'a, Num: ArgNumber>() -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:181-183`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L181-L183)*

Deliberately pass `-1` to a file-descriptor argument, for system calls
like `mmap` where this indicates the argument is omitted.

### `slice_just_addr`

```rust
fn slice_just_addr<T: Sized, Num: ArgNumber>(v: &[T]) -> super::reg::ArgReg<'_, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:186-189`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L186-L189)*

### `slice_just_addr_mut`

```rust
fn slice_just_addr_mut<T: Sized, Num: ArgNumber>(v: &mut [T]) -> super::reg::ArgReg<'_, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:192-194`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L192-L194)*

### `slice`

```rust
fn slice<T: Sized, Num0: ArgNumber, Num1: ArgNumber>(v: &[T]) -> (super::reg::ArgReg<'_, Num0>, super::reg::ArgReg<'_, Num1>)
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:197-201`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L197-L201)*

### `slice_mut`

```rust
fn slice_mut<T: Sized, Num0: ArgNumber, Num1: ArgNumber>(v: &mut [T]) -> (super::reg::ArgReg<'_, Num0>, super::reg::ArgReg<'_, Num1>)
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:204-208`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L204-L208)*

### `by_ref`

```rust
fn by_ref<T: Sized, Num: ArgNumber>(t: &T) -> super::reg::ArgReg<'_, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:211-214`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L211-L214)*

### `by_mut`

```rust
fn by_mut<T: Sized, Num: ArgNumber>(t: &mut T) -> super::reg::ArgReg<'_, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:217-219`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L217-L219)*

### `opt_mut`

```rust
fn opt_mut<T: Sized, Num: ArgNumber>(t: Option<&mut T>) -> super::reg::ArgReg<'_, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:224-231`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L224-L231)*

Convert an optional mutable reference into a `usize` for passing to a
syscall.

### `opt_ref`

```rust
fn opt_ref<T: Sized, Num: ArgNumber>(t: Option<&T>) -> super::reg::ArgReg<'_, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:236-243`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L236-L243)*

Convert an optional immutable reference into a `usize` for passing to a
syscall.

### `c_int`

```rust
fn c_int<'a, Num: ArgNumber>(i: c::c_int) -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:249-251`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L249-L251)*

Convert a `c_int` into an `ArgReg`.

Be sure to use `raw_fd` to pass `RawFd` values.

### `c_uint`

```rust
fn c_uint<'a, Num: ArgNumber>(i: c::c_uint) -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:255-257`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L255-L257)*

Convert a `c_uint` into an `ArgReg`.

### `loff_t`

```rust
fn loff_t<'a, Num: ArgNumber>(i: linux_raw_sys::general::__kernel_loff_t) -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:261-263`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L261-L263)*

### `loff_t_from_u64`

```rust
fn loff_t_from_u64<'a, Num: ArgNumber>(i: u64) -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:267-271`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L267-L271)*

### `dev_t`

```rust
fn dev_t<'a, Num: ArgNumber>(dev: u64) -> super::reg::ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:671-673`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L671-L673)*

### `ret`

```rust
unsafe fn ret(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:885-887`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L885-L887)*

Convert a `usize` returned from a syscall that effectively returns `()` on
success.

# Safety

The caller must ensure that this is the return value of a syscall which
just returns 0 on success.

### `ret_infallible`

```rust
unsafe fn ret_infallible(raw: super::reg::RetReg<super::reg::R0>)
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:909-916`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L909-L916)*

Convert a `usize` returned from a syscall that effectively always returns
`()`.

# Safety

The caller must ensure that this is the return value of a syscall which
always returns `()`.

### `ret_c_int`

```rust
fn ret_c_int(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<c::c_int>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:921-923`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L921-L923)*

Convert a `usize` returned from a syscall that effectively returns a
`c_int` on success.

### `ret_c_uint`

```rust
fn ret_c_uint(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<c::c_uint>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:928-930`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L928-L930)*

Convert a `usize` returned from a syscall that effectively returns a
`c_uint` on success.

### `ret_u64`

```rust
fn ret_u64(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<u64>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:936-938`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L936-L938)*

Convert a `usize` returned from a syscall that effectively returns a `u64`
on success.

### `ret_usize`

```rust
fn ret_usize(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<usize>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:943-945`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L943-L945)*

Convert a `usize` returned from a syscall that effectively returns a
`usize` on success.

### `ret_usize_infallible`

```rust
unsafe fn ret_usize_infallible(raw: super::reg::RetReg<super::reg::R0>) -> usize
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:955-964`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L955-L964)*

Convert a `usize` returned from a syscall that effectively always
returns a `usize`.

# Safety

This function must only be used with return values from infallible
syscalls.

### `ret_c_int_infallible`

```rust
unsafe fn ret_c_int_infallible(raw: super::reg::RetReg<super::reg::R0>) -> c::c_int
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:974-983`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L974-L983)*

Convert a `c_int` returned from a syscall that effectively always
returns a `c_int`.

# Safety

This function must only be used with return values from infallible
syscalls.

### `ret_c_uint_infallible`

```rust
unsafe fn ret_c_uint_infallible(raw: super::reg::RetReg<super::reg::R0>) -> c::c_uint
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:993-1002`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L993-L1002)*

Convert a `c_uint` returned from a syscall that effectively always
returns a `c_uint`.

# Safety

This function must only be used with return values from infallible
syscalls.

### `ret_owned_fd`

```rust
unsafe fn ret_owned_fd(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<crate::fd::OwnedFd>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:1012-1015`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L1012-L1015)*

Convert a `usize` returned from a syscall that effectively returns an
`OwnedFd` on success.

# Safety

The caller must ensure that this is the return value of a syscall which
returns an owned file descriptor.

### `ret_discarded_fd`

```rust
unsafe fn ret_discarded_fd(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:1027-1030`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L1027-L1030)*

Convert the return value of `dup2` and `dup3`.

When these functions succeed, they return the same value as their second
argument, so we don't construct a new `OwnedFd`.

# Safety

The caller must ensure that this is the return value of a syscall which
returns a file descriptor.

### `ret_void_star`

```rust
fn ret_void_star(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<*mut c::c_void>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/conv.rs:1035-1037`](../../../../.source_1765633015/rustix-1.1.2/src/backend/linux_raw/conv.rs#L1035-L1037)*

Convert a `usize` returned from a syscall that effectively returns a
`*mut c_void` on success.

