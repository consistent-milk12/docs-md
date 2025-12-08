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
start, create a type that implements [`Ioctl`](#ioctl). Then, pass it to [`ioctl`](#ioctl)
to make the `ioctl` call.

## Contents

- [Modules](#modules)
  - [`patterns`](#patterns)
  - [`linux`](#linux)
  - [`opcode`](#opcode)
- [Structs](#structs)
  - [`NoArg`](#noarg)
  - [`Getter`](#getter)
  - [`Setter`](#setter)
  - [`Updater`](#updater)
  - [`IntegerSetter`](#integersetter)
- [Enums](#enums)
  - [`Direction`](#direction)
- [Traits](#traits)
  - [`Ioctl`](#ioctl)
- [Functions](#functions)
  - [`ioctl`](#ioctl)
  - [`_ioctl`](#_ioctl)
  - [`_ioctl_readonly`](#_ioctl_readonly)
- [Type Aliases](#type-aliases)
  - [`IoctlOutput`](#ioctloutput)
  - [`Opcode`](#opcode)
  - [`_Opcode`](#_opcode)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`patterns`](#patterns) | mod | Implements typical patterns for `ioctl` usage. |
| [`linux`](#linux) | mod | `ioctl` opcode behavior for Linux platforms. |
| [`opcode`](#opcode) | mod | Const functions for computing opcode values. |
| [`NoArg`](#noarg) | struct | Implements an `ioctl` with no real arguments. |
| [`Getter`](#getter) | struct | Implements the traditional “getter” pattern for `ioctl`s. |
| [`Setter`](#setter) | struct | Implements the pattern for `ioctl`s where a pointer argument is given to |
| [`Updater`](#updater) | struct | Implements an “updater” pattern for `ioctl`s. |
| [`IntegerSetter`](#integersetter) | struct | Implements an `ioctl` that passes an integer into the `ioctl`. |
| [`Direction`](#direction) | enum | The direction that an `ioctl` is going. |
| [`Ioctl`](#ioctl) | trait | A trait defining the properties of an `ioctl` command. |
| [`ioctl`](#ioctl) | fn | Perform an `ioctl` call. |
| [`_ioctl`](#_ioctl) | fn |  |
| [`_ioctl_readonly`](#_ioctl_readonly) | fn |  |
| [`IoctlOutput`](#ioctloutput) | type | The type used by the `ioctl` to signify the output. |
| [`Opcode`](#opcode) | type | The type used by the `ioctl` to signify the command. |
| [`_Opcode`](#_opcode) | type |  |

## Modules

- [`patterns`](patterns/index.md) - Implements typical patterns for `ioctl` usage.
- [`linux`](linux/index.md) - `ioctl` opcode behavior for Linux platforms.
- [`opcode`](opcode/index.md) - Const functions for computing opcode values.

## Structs

### `NoArg<const OPCODE: super::Opcode>`

```rust
struct NoArg<const OPCODE: super::Opcode> {
}
```

Implements an `ioctl` with no real arguments.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](opcode/index.md) module.


#### Implementations

- <span id="noarg-new"></span>`const unsafe fn new() -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode> Debug for NoArg<OPCODE>`

- <span id="noarg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode> Ioctl for NoArg<OPCODE>`

- <span id="noarg-output"></span>`type Output = ()`

- <span id="noarg-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="noarg-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](#opcode)

- <span id="noarg-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="noarg-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](#ioctloutput), [`Result`](../io/index.md), [`Ioctl`](#ioctl)

### `Getter<const OPCODE: super::Opcode, Output>`

```rust
struct Getter<const OPCODE: super::Opcode, Output> {
    output: mem::MaybeUninit<Output>,
}
```

Implements the traditional “getter” pattern for `ioctl`s.

Some `ioctl`s just read data into the userspace. As this is a popular
pattern, this structure implements it.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](opcode/index.md) module.


#### Fields

- **`output`**: `mem::MaybeUninit<Output>`

  The output data.

#### Implementations

- <span id="getter-new"></span>`const unsafe fn new() -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode, Output> Debug for Getter<OPCODE, Output>`

- <span id="getter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode, Output> Ioctl for Getter<OPCODE, Output>`

- <span id="getter-output"></span>`type Output = Output`

- <span id="getter-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="getter-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](#opcode)

- <span id="getter-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="getter-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, ptr: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](#ioctloutput), [`Result`](../io/index.md), [`Ioctl`](#ioctl)

### `Setter<const OPCODE: super::Opcode, Input>`

```rust
struct Setter<const OPCODE: super::Opcode, Input> {
    input: Input,
}
```

Implements the pattern for `ioctl`s where a pointer argument is given to
the `ioctl`.

The opcode must be read-only.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](opcode/index.md) module.


#### Fields

- **`input`**: `Input`

  The input data.

#### Implementations

- <span id="setter-new"></span>`const unsafe fn new(input: Input) -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode, Input: fmt::Debug> Debug for Setter<OPCODE, Input>`

- <span id="setter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode, Input> Ioctl for Setter<OPCODE, Input>`

- <span id="setter-output"></span>`type Output = ()`

- <span id="setter-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="setter-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](#opcode)

- <span id="setter-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="setter-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](#ioctloutput), [`Result`](../io/index.md), [`Ioctl`](#ioctl)

### `Updater<'a, const OPCODE: super::Opcode, Value>`

```rust
struct Updater<'a, const OPCODE: super::Opcode, Value> {
    value: &'a mut Value,
}
```

Implements an “updater” pattern for `ioctl`s.

The ioctl takes a reference to a struct that it reads its input from,
then writes output to the same struct.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](opcode/index.md) module.


#### Fields

- **`value`**: `&'a mut Value`

  Reference to input/output data.

#### Implementations

- <span id="updater-new"></span>`unsafe fn new(value: &'a mut Value) -> Self`

#### Trait Implementations

##### `impl<'a, const OPCODE: super::Opcode, T> Ioctl for Updater<'a, OPCODE, T>`

- <span id="updater-output"></span>`type Output = ()`

- <span id="updater-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="updater-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](#opcode)

- <span id="updater-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="updater-output-from-ptr"></span>`unsafe fn output_from_ptr(_output: IoctlOutput, _ptr: *mut c::c_void) -> Result<()>` — [`IoctlOutput`](#ioctloutput), [`Result`](../io/index.md)

### `IntegerSetter<const OPCODE: super::Opcode>`

```rust
struct IntegerSetter<const OPCODE: super::Opcode> {
    value: *mut c::c_void,
}
```

Implements an `ioctl` that passes an integer into the `ioctl`.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](opcode/index.md) module.


#### Fields

- **`value`**: `*mut c::c_void`

  The value to pass in.
  
  For strict provenance preservation, this is a pointer.

#### Implementations

- <span id="integersetter-new-usize"></span>`const unsafe fn new_usize(value: usize) -> Self`

- <span id="integersetter-new-pointer"></span>`const unsafe fn new_pointer(value: *mut c::c_void) -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode> Ioctl for IntegerSetter<OPCODE>`

- <span id="integersetter-output"></span>`type Output = ()`

- <span id="integersetter-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="integersetter-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](#opcode)

- <span id="integersetter-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="integersetter-output-from-ptr"></span>`unsafe fn output_from_ptr(_out: IoctlOutput, _extract_output: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](#ioctloutput), [`Result`](../io/index.md), [`Ioctl`](#ioctl)

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

##### `impl Clone for Direction`

- <span id="direction-clone"></span>`fn clone(&self) -> Direction` — [`Direction`](#direction)

##### `impl Copy for Direction`

##### `impl Debug for Direction`

- <span id="direction-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Direction`

##### `impl Hash for Direction`

- <span id="direction-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Direction`

- <span id="direction-cmp"></span>`fn cmp(&self, other: &Direction) -> cmp::Ordering` — [`Direction`](#direction)

##### `impl PartialEq for Direction`

- <span id="direction-eq"></span>`fn eq(&self, other: &Direction) -> bool` — [`Direction`](#direction)

##### `impl PartialOrd for Direction`

- <span id="direction-partial-cmp"></span>`fn partial_cmp(&self, other: &Direction) -> option::Option<cmp::Ordering>` — [`Direction`](#direction)

##### `impl StructuralPartialEq for Direction`

## Traits

### `Ioctl`

```rust
trait Ioctl { ... }
```

A trait defining the properties of an `ioctl` command.

Objects implementing this trait can be passed to [`ioctl`](#ioctl) to make an
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

- `fn opcode(&self) -> Opcode`

  Get the opcode used by this `ioctl` command.

- `fn as_ptr(&mut self) -> *mut c::c_void`

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
wrappers, like [`ioctl_fionbio`](../io/index.md) and [`ioctl_fionread`](../io/index.md). It is recommended
to use those instead of this function, as they are safer and more
idiomatic. For other cases, implement the [`Ioctl`](#ioctl) API and pass it to this
function.

See documentation for [`Ioctl`](#ioctl) for more information.


# Safety

While [`Ioctl`](#ioctl) takes much of the unsafety out of `ioctl` calls, callers
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
 - [illumos]









### `_ioctl`

```rust
unsafe fn _ioctl(fd: crate::fd::BorrowedFd<'_>, request: Opcode, arg: *mut c::c_void) -> crate::io::Result<IoctlOutput>
```

### `_ioctl_readonly`

```rust
unsafe fn _ioctl_readonly(fd: crate::fd::BorrowedFd<'_>, request: Opcode, arg: *mut c::c_void) -> crate::io::Result<IoctlOutput>
```

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

### `_Opcode`

```rust
type _Opcode = c::c_uint;
```

