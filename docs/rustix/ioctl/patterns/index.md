*[rustix](../../index.md) / [ioctl](../index.md) / [patterns](index.md)*

---

# Module `patterns`

Implements typical patterns for `ioctl` usage.

## Structs

### `NoArg<const OPCODE: super::Opcode>`

```rust
struct NoArg<const OPCODE: super::Opcode> {
}
```

Implements an `ioctl` with no real arguments.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](../opcode/index.md) module.


#### Implementations

- `const unsafe fn new() -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode> Debug for NoArg<OPCODE>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode> Ioctl for NoArg<OPCODE>`

- `type Output = ()`

- `const IS_MUTATING: bool`

- `fn opcode(self: &Self) -> self::Opcode` — [`Opcode`](../index.md)

- `fn as_ptr(self: &mut Self) -> *mut c::c_void`

- `unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

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
[`opcode`](../opcode/index.md) module.


#### Fields

- **`output`**: `mem::MaybeUninit<Output>`

  The output data.

#### Implementations

- `const unsafe fn new() -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode, Output> Debug for Getter<OPCODE, Output>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode, Output> Ioctl for Getter<OPCODE, Output>`

- `type Output = Output`

- `const IS_MUTATING: bool`

- `fn opcode(self: &Self) -> self::Opcode` — [`Opcode`](../index.md)

- `fn as_ptr(self: &mut Self) -> *mut c::c_void`

- `unsafe fn output_from_ptr(_: IoctlOutput, ptr: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

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
[`opcode`](../opcode/index.md) module.


#### Fields

- **`input`**: `Input`

  The input data.

#### Implementations

- `const unsafe fn new(input: Input) -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode, Input: fmt::Debug> Debug for Setter<OPCODE, Input>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode, Input> Ioctl for Setter<OPCODE, Input>`

- `type Output = ()`

- `const IS_MUTATING: bool`

- `fn opcode(self: &Self) -> self::Opcode` — [`Opcode`](../index.md)

- `fn as_ptr(self: &mut Self) -> *mut c::c_void`

- `unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

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
[`opcode`](../opcode/index.md) module.


#### Fields

- **`value`**: `&'a mut Value`

  Reference to input/output data.

#### Implementations

- `unsafe fn new(value: &'a mut Value) -> Self`

#### Trait Implementations

##### `impl<'a, const OPCODE: super::Opcode, T> Ioctl for Updater<'a, OPCODE, T>`

- `type Output = ()`

- `const IS_MUTATING: bool`

- `fn opcode(self: &Self) -> self::Opcode` — [`Opcode`](../index.md)

- `fn as_ptr(self: &mut Self) -> *mut c::c_void`

- `unsafe fn output_from_ptr(_output: IoctlOutput, _ptr: *mut c::c_void) -> Result<()>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md)

### `IntegerSetter<const OPCODE: super::Opcode>`

```rust
struct IntegerSetter<const OPCODE: super::Opcode> {
    value: *mut c::c_void,
}
```

Implements an `ioctl` that passes an integer into the `ioctl`.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](../opcode/index.md) module.


#### Fields

- **`value`**: `*mut c::c_void`

  The value to pass in.
  
  For strict provenance preservation, this is a pointer.

#### Implementations

- `const unsafe fn new_usize(value: usize) -> Self`

- `const unsafe fn new_pointer(value: *mut c::c_void) -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode> Ioctl for IntegerSetter<OPCODE>`

- `type Output = ()`

- `const IS_MUTATING: bool`

- `fn opcode(self: &Self) -> self::Opcode` — [`Opcode`](../index.md)

- `fn as_ptr(self: &mut Self) -> *mut c::c_void`

- `unsafe fn output_from_ptr(_out: IoctlOutput, _extract_output: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

