*[rustix](../index.md) / [ioctl](index.md)*

---

# Module `ioctl`

Unsafe `ioctl` API.

Unix systems expose a number of `ioctl`'s. `ioctl`s have been adopted as a
general purpose system call for making calls into the kernel. In addition
to the wide variety of system calls that are included by default in the
kernel, many drivers expose their own `ioctl`'s for controlling their
behavior, some of which are proprietary. Therefore it is impossible to make
a safe interface for every `ioctl` call, as they all have wildly varying
semantics.

This module provides an unsafe interface to write your own `ioctl` API. To
start, create a type that implements [`Ioctl`](rustix/ioctl/index.md). Then, pass it to [`ioctl`](rustix/ioctl/index.md)
to make the `ioctl` call.

## Modules

- [`opcode`](opcode/index.md) - Const functions for computing opcode values.

## Enums

### `Direction`

```rust
enum Direction {
    None,
    Read,
    Write,
    ReadWrite,
}
```

The direction that an `ioctl` is going.

The direction is relative to userspace: `Read` means reading data from the
kernel, and `Write` means the kernel writing data to userspace.

#### Variants

- **`None`**

  None of the above.

- **`Read`**

  Read data from the kernel.

- **`Write`**

  Write data to the kernel.

- **`ReadWrite`**

  Read and write data to the kernel.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Direction`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Direction) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Direction) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Direction) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Ioctl`

```rust
trait Ioctl { ... }
```

A trait defining the properties of an `ioctl` command.

Objects implementing this trait can be passed to [`ioctl`](rustix/ioctl/index.md) to make an
`ioctl` call. The contents of the object represent the inputs to the
`ioctl` call. The inputs must be convertible to a pointer through the
`as_ptr` method. In most cases, this involves either casting a number to a
pointer, or creating a pointer to the actual data. The latter case is
necessary for `ioctl` calls that modify userspace data.

# Safety

This trait is unsafe to implement because it is impossible to guarantee
that the `ioctl` call is safe. The `ioctl` call may be proprietary, or it
may be unsafe to call in certain circumstances.

By implementing this trait, you guarantee that:

 - The `ioctl` call expects the input provided by `as_ptr` and produces the
   output as indicated by `output`.
 - That `output_from_ptr` can safely take the pointer from `as_ptr` and
   cast it to the correct type, *only* after the `ioctl` call.
 - That the return value of `opcode` uniquely identifies the `ioctl` call.
 - That, for whatever platforms you are targeting, the `ioctl` call is safe
   to make.
 - If `IS_MUTATING` is false, that no userspace data will be modified by
   the `ioctl` call.

#### Required Methods

- `type Output`

- `const IS_MUTATING: bool`

- `fn opcode(self: &Self) -> Opcode`

  Get the opcode used by this `ioctl` command.

- `fn as_ptr(self: &mut Self) -> *mut c::c_void`

  Get a pointer to the data to be passed to the `ioctl` command.

- `fn output_from_ptr(out: IoctlOutput, extract_output: *mut c::c_void) -> Result<<Self as >::Output>`

  Cast the output data to the correct type.

## Functions

### `ioctl`

```rust
unsafe fn ioctl<F: AsFd, I: Ioctl>(fd: F, ioctl: I) -> crate::io::Result<<I as >::Output>
```

Perform an `ioctl` call.

`ioctl` was originally intended to act as a way of modifying the behavior
of files, but has since been adopted as a general purpose system call for
making calls into the kernel. In addition to the default calls exposed by
generic file descriptors, many drivers expose their own `ioctl` calls for
controlling their behavior, some of which are proprietary.

This crate exposes many other `ioctl` interfaces with safe and idiomatic
wrappers, like [`ioctl_fionbio`](#ioctl-fionbio) and [`ioctl_fionread`](#ioctl-fionread). It is recommended
to use those instead of this function, as they are safer and more
idiomatic. For other cases, implement the [`Ioctl`](rustix/ioctl/index.md) API and pass it to this
function.

See documentation for [`Ioctl`](rustix/ioctl/index.md) for more information.


# Safety

While [`Ioctl`](rustix/ioctl/index.md) takes much of the unsafety out of `ioctl` calls, callers
must still ensure that the opcode value, operand type, and data access
correctly reflect what's in the device driver servicing the call. `ioctl`
calls form a protocol between the userspace `ioctl` callers and the device
drivers in the kernel, and safety depends on both sides agreeing and
upholding the expectations of the other.

And, `ioctl` calls can read and write arbitrary memory and have arbitrary
side effects. Callers must ensure that any memory accesses and side effects
are compatible with Rust language invariants.

# References
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [Apple]
 - [Solaris]
 - [illumos](#illumos)


[Linux]: https://man7.org/linux/man-pages/man2/ioctl.2.html
[Winsock]: https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-ioctlsocket
[FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=ioctl&sektion=2
[NetBSD]: https://man.netbsd.org/ioctl.2
[OpenBSD]: https://man.openbsd.org/ioctl.2
[Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ioctl.2.html
[Solaris]: https://docs.oracle.com/cd/E23824_01/html/821-1463/ioctl-2.html
[illumos](#illumos)
: https://illumos.org/man/2/ioctl

## Type Aliases

### `IoctlOutput`

```rust
type IoctlOutput = c::c_int;
```

The type used by the `ioctl` to signify the output.

### `Opcode`

```rust
type Opcode = c::c_uint;
```

The type used by the `ioctl` to signify the command.

