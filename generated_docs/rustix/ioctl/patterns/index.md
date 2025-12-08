*[rustix](../../index.md) / [ioctl](../index.md) / [patterns](index.md)*

---

# Module `patterns`

Implements typical patterns for `ioctl` usage.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoArg`](#noarg) | struct | Implements an `ioctl` with no real arguments. |
| [`Getter`](#getter) | struct | Implements the traditional “getter” pattern for `ioctl`s. |
| [`Setter`](#setter) | struct | Implements the pattern for `ioctl`s where a pointer argument is given to |
| [`Updater`](#updater) | struct | Implements an “updater” pattern for `ioctl`s. |
| [`IntegerSetter`](#integersetter) | struct | Implements an `ioctl` that passes an integer into the `ioctl`. |

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

- <span id="noarg-new"></span>`const unsafe fn new() -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode> Debug for NoArg<OPCODE>`

- <span id="noarg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode> Ioctl for NoArg<OPCODE>`

- <span id="noarg-output"></span>`type Output = ()`

- <span id="noarg-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="noarg-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md)

- <span id="noarg-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="noarg-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

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

- <span id="getter-new"></span>`const unsafe fn new() -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode, Output> Debug for Getter<OPCODE, Output>`

- <span id="getter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode, Output> Ioctl for Getter<OPCODE, Output>`

- <span id="getter-output"></span>`type Output = Output`

- <span id="getter-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="getter-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md)

- <span id="getter-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="getter-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, ptr: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

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

- <span id="setter-new"></span>`const unsafe fn new(input: Input) -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode, Input: fmt::Debug> Debug for Setter<OPCODE, Input>`

- <span id="setter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<const OPCODE: super::Opcode, Input> Ioctl for Setter<OPCODE, Input>`

- <span id="setter-output"></span>`type Output = ()`

- <span id="setter-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="setter-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md)

- <span id="setter-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="setter-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

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

- <span id="updater-new"></span>`unsafe fn new(value: &'a mut Value) -> Self`

#### Trait Implementations

##### `impl<'a, const OPCODE: super::Opcode, T> Ioctl for Updater<'a, OPCODE, T>`

- <span id="updater-output"></span>`type Output = ()`

- <span id="updater-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="updater-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md)

- <span id="updater-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="updater-output-from-ptr"></span>`unsafe fn output_from_ptr(_output: IoctlOutput, _ptr: *mut c::c_void) -> Result<()>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md)

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

- <span id="integersetter-new-usize"></span>`const unsafe fn new_usize(value: usize) -> Self`

- <span id="integersetter-new-pointer"></span>`const unsafe fn new_pointer(value: *mut c::c_void) -> Self`

#### Trait Implementations

##### `impl<const OPCODE: super::Opcode> Ioctl for IntegerSetter<OPCODE>`

- <span id="integersetter-output"></span>`type Output = ()`

- <span id="integersetter-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="integersetter-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md)

- <span id="integersetter-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="integersetter-output-from-ptr"></span>`unsafe fn output_from_ptr(_out: IoctlOutput, _extract_output: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md), [`Result`](../../io/index.md), [`Ioctl`](../index.md)

