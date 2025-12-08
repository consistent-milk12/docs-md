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

## Functions

### `zero`

```rust
fn zero<'a, Num: ArgNumber>() -> super::reg::ArgReg<'a, Num>
```

Pass a zero, or null, argument.

### `size_of`

```rust
fn size_of<'a, T: Sized, Num: ArgNumber>() -> super::reg::ArgReg<'a, Num>
```

Pass the `mem::size_of` of a type.

### `pass_usize`

```rust
fn pass_usize<'a, Num: ArgNumber>(t: usize) -> super::reg::ArgReg<'a, Num>
```

Pass an arbitrary `usize` value.

For passing pointers, use `void_star` or other functions which take a raw
pointer instead of casting to `usize`, so that provenance is preserved.

### `raw_fd`

```rust
unsafe fn raw_fd<'a, Num: ArgNumber>(fd: super::fd::RawFd) -> super::reg::ArgReg<'a, Num>
```

Pass a raw file-descriptor argument. Most users should use `ArgReg::from`
instead, to preserve I/O safety as long as possible.

# Safety

`fd` must be a valid open file descriptor.

### `no_fd`

```rust
fn no_fd<'a, Num: ArgNumber>() -> super::reg::ArgReg<'a, Num>
```

Deliberately pass `-1` to a file-descriptor argument, for system calls
like `mmap` where this indicates the argument is omitted.

### `slice_just_addr`

```rust
fn slice_just_addr<T: Sized, Num: ArgNumber>(v: &[T]) -> super::reg::ArgReg<'_, Num>
```

### `slice_just_addr_mut`

```rust
fn slice_just_addr_mut<T: Sized, Num: ArgNumber>(v: &mut [T]) -> super::reg::ArgReg<'_, Num>
```

### `slice`

```rust
fn slice<T: Sized, Num0: ArgNumber, Num1: ArgNumber>(v: &[T]) -> (super::reg::ArgReg<'_, Num0>, super::reg::ArgReg<'_, Num1>)
```

### `slice_mut`

```rust
fn slice_mut<T: Sized, Num0: ArgNumber, Num1: ArgNumber>(v: &mut [T]) -> (super::reg::ArgReg<'_, Num0>, super::reg::ArgReg<'_, Num1>)
```

### `by_ref`

```rust
fn by_ref<T: Sized, Num: ArgNumber>(t: &T) -> super::reg::ArgReg<'_, Num>
```

### `by_mut`

```rust
fn by_mut<T: Sized, Num: ArgNumber>(t: &mut T) -> super::reg::ArgReg<'_, Num>
```

### `opt_mut`

```rust
fn opt_mut<T: Sized, Num: ArgNumber>(t: Option<&mut T>) -> super::reg::ArgReg<'_, Num>
```

Convert an optional mutable reference into a `usize` for passing to a
syscall.

### `opt_ref`

```rust
fn opt_ref<T: Sized, Num: ArgNumber>(t: Option<&T>) -> super::reg::ArgReg<'_, Num>
```

Convert an optional immutable reference into a `usize` for passing to a
syscall.

### `c_int`

```rust
fn c_int<'a, Num: ArgNumber>(i: c::c_int) -> super::reg::ArgReg<'a, Num>
```

Convert a `c_int` into an `ArgReg`.

Be sure to use `raw_fd` to pass `RawFd` values.

### `c_uint`

```rust
fn c_uint<'a, Num: ArgNumber>(i: c::c_uint) -> super::reg::ArgReg<'a, Num>
```

Convert a `c_uint` into an `ArgReg`.

### `loff_t`

```rust
fn loff_t<'a, Num: ArgNumber>(i: linux_raw_sys::general::__kernel_loff_t) -> super::reg::ArgReg<'a, Num>
```

### `loff_t_from_u64`

```rust
fn loff_t_from_u64<'a, Num: ArgNumber>(i: u64) -> super::reg::ArgReg<'a, Num>
```

### `dev_t`

```rust
fn dev_t<'a, Num: ArgNumber>(dev: u64) -> super::reg::ArgReg<'a, Num>
```

### `ret`

```rust
unsafe fn ret(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<()>
```

Convert a `usize` returned from a syscall that effectively returns `()` on
success.

# Safety

The caller must ensure that this is the return value of a syscall which
just returns 0 on success.

### `ret_infallible`

```rust
unsafe fn ret_infallible(raw: super::reg::RetReg<super::reg::R0>)
```

Convert a `usize` returned from a syscall that effectively always returns
`()`.

# Safety

The caller must ensure that this is the return value of a syscall which
always returns `()`.

### `ret_c_int`

```rust
fn ret_c_int(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<c::c_int>
```

Convert a `usize` returned from a syscall that effectively returns a
`c_int` on success.

### `ret_c_uint`

```rust
fn ret_c_uint(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<c::c_uint>
```

Convert a `usize` returned from a syscall that effectively returns a
`c_uint` on success.

### `ret_u64`

```rust
fn ret_u64(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<u64>
```

Convert a `usize` returned from a syscall that effectively returns a `u64`
on success.

### `ret_usize`

```rust
fn ret_usize(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<usize>
```

Convert a `usize` returned from a syscall that effectively returns a
`usize` on success.

### `ret_usize_infallible`

```rust
unsafe fn ret_usize_infallible(raw: super::reg::RetReg<super::reg::R0>) -> usize
```

Convert a `usize` returned from a syscall that effectively always
returns a `usize`.

# Safety

This function must only be used with return values from infallible
syscalls.

### `ret_c_int_infallible`

```rust
unsafe fn ret_c_int_infallible(raw: super::reg::RetReg<super::reg::R0>) -> c::c_int
```

Convert a `c_int` returned from a syscall that effectively always
returns a `c_int`.

# Safety

This function must only be used with return values from infallible
syscalls.

### `ret_c_uint_infallible`

```rust
unsafe fn ret_c_uint_infallible(raw: super::reg::RetReg<super::reg::R0>) -> c::c_uint
```

Convert a `c_uint` returned from a syscall that effectively always
returns a `c_uint`.

# Safety

This function must only be used with return values from infallible
syscalls.

### `ret_owned_fd`

```rust
unsafe fn ret_owned_fd(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<crate::fd::OwnedFd>
```

Convert a `usize` returned from a syscall that effectively returns an
`OwnedFd` on success.

# Safety

The caller must ensure that this is the return value of a syscall which
returns an owned file descriptor.

### `ret_discarded_fd`

```rust
unsafe fn ret_discarded_fd(raw: super::reg::RetReg<super::reg::R0>) -> io::Result<()>
```

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

Convert a `usize` returned from a syscall that effectively returns a
`*mut c_void` on success.

