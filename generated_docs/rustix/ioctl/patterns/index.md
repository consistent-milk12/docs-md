*[rustix](../../index.md) / [ioctl](../index.md) / [patterns](index.md)*

---

# Module `patterns`

Implements typical patterns for `ioctl` usage.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoArg`](#noarg) | struct | Implements an `ioctl` with no real arguments. |
| [`Getter`](#getter) | struct | Implements the traditional “getter” pattern for `ioctl`s. |
| [`Setter`](#setter) | struct | Implements the pattern for `ioctl`s where a pointer argument is given to the `ioctl`. |
| [`Updater`](#updater) | struct | Implements an “updater” pattern for `ioctl`s. |
| [`IntegerSetter`](#integersetter) | struct | Implements an `ioctl` that passes an integer into the `ioctl`. |

## Structs

### `NoArg<const OPCODE: super::Opcode>`

```rust
struct NoArg<const OPCODE: super::Opcode> {
}
```

*Defined in [`rustix-1.1.2/src/ioctl/patterns.rs:17`](../../../../.source_1765633015/rustix-1.1.2/src/ioctl/patterns.rs#L17)*

Implements an `ioctl` with no real arguments.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](../opcode/index.md) module.


#### Implementations

- <span id="noarg-new"></span>`const unsafe fn new() -> Self`

  Create a new no-argument `ioctl` object.

  

  # Safety

  

   - `OPCODE` must provide a valid opcode.

#### Trait Implementations

##### `impl Any for NoArg<OPCODE>`

- <span id="noarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoArg<OPCODE>`

- <span id="noarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoArg<OPCODE>`

- <span id="noarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for NoArg<OPCODE>`

- <span id="noarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NoArg<OPCODE>`

- <span id="noarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoArg<OPCODE>`

- <span id="noarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ioctl for NoArg<OPCODE>`

- <span id="noarg-ioctl-type-output"></span>`type Output = ()`

- <span id="noarg-ioctl-const-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="noarg-ioctl-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md#opcode)

- <span id="noarg-ioctl-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="noarg-ioctl-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md#ioctloutput), [`Result`](../../io/errno/index.md#result), [`Ioctl`](../index.md#ioctl)

##### `impl<U> TryFrom for NoArg<OPCODE>`

- <span id="noarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="noarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoArg<OPCODE>`

- <span id="noarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="noarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Getter<const OPCODE: super::Opcode, Output>`

```rust
struct Getter<const OPCODE: super::Opcode, Output> {
    output: mem::MaybeUninit<Output>,
}
```

*Defined in [`rustix-1.1.2/src/ioctl/patterns.rs:64-67`](../../../../.source_1765633015/rustix-1.1.2/src/ioctl/patterns.rs#L64-L67)*

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

  Create a new getter-style `ioctl` object.

  

  # Safety

  

   - `OPCODE` must provide a valid opcode.

   - For this opcode, `Output` must be the type that the kernel expects

     to write into.

#### Trait Implementations

##### `impl Any for Getter<OPCODE, Output>`

- <span id="getter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Getter<OPCODE, Output>`

- <span id="getter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Getter<OPCODE, Output>`

- <span id="getter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Output> Debug for Getter<OPCODE, Output>`

- <span id="getter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Getter<OPCODE, Output>`

- <span id="getter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Getter<OPCODE, Output>`

- <span id="getter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Output> Ioctl for Getter<OPCODE, Output>`

- <span id="getter-ioctl-type-output"></span>`type Output = Output`

- <span id="getter-ioctl-const-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="getter-ioctl-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md#opcode)

- <span id="getter-ioctl-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="getter-ioctl-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, ptr: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md#ioctloutput), [`Result`](../../io/errno/index.md#result), [`Ioctl`](../index.md#ioctl)

##### `impl<U> TryFrom for Getter<OPCODE, Output>`

- <span id="getter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="getter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Getter<OPCODE, Output>`

- <span id="getter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="getter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Setter<const OPCODE: super::Opcode, Input>`

```rust
struct Setter<const OPCODE: super::Opcode, Input> {
    input: Input,
}
```

*Defined in [`rustix-1.1.2/src/ioctl/patterns.rs:118-121`](../../../../.source_1765633015/rustix-1.1.2/src/ioctl/patterns.rs#L118-L121)*

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

  Create a new pointer setter-style `ioctl` object.

  

  # Safety

  

   - `OPCODE` must provide a valid opcode.

   - For this opcode, `Input` must be the type that the kernel expects to

     get.

#### Trait Implementations

##### `impl Any for Setter<OPCODE, Input>`

- <span id="setter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Setter<OPCODE, Input>`

- <span id="setter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Setter<OPCODE, Input>`

- <span id="setter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Input: fmt::Debug> Debug for Setter<OPCODE, Input>`

- <span id="setter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Setter<OPCODE, Input>`

- <span id="setter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Setter<OPCODE, Input>`

- <span id="setter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Input> Ioctl for Setter<OPCODE, Input>`

- <span id="setter-ioctl-type-output"></span>`type Output = ()`

- <span id="setter-ioctl-const-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="setter-ioctl-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md#opcode)

- <span id="setter-ioctl-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="setter-ioctl-output-from-ptr"></span>`unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md#ioctloutput), [`Result`](../../io/errno/index.md#result), [`Ioctl`](../index.md#ioctl)

##### `impl<U> TryFrom for Setter<OPCODE, Input>`

- <span id="setter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="setter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Setter<OPCODE, Input>`

- <span id="setter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="setter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Updater<'a, const OPCODE: super::Opcode, Value>`

```rust
struct Updater<'a, const OPCODE: super::Opcode, Value> {
    value: &'a mut Value,
}
```

*Defined in [`rustix-1.1.2/src/ioctl/patterns.rs:173-176`](../../../../.source_1765633015/rustix-1.1.2/src/ioctl/patterns.rs#L173-L176)*

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

  Create a new pointer updater-style `ioctl` object.

  

  # Safety

  

   - `OPCODE` must provide a valid opcode.

   - For this opcode, `Value` must be the type that the kernel expects to

     get.

#### Trait Implementations

##### `impl Any for Updater<'a, OPCODE, Value>`

- <span id="updater-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Updater<'a, OPCODE, Value>`

- <span id="updater-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Updater<'a, OPCODE, Value>`

- <span id="updater-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Updater<'a, OPCODE, Value>`

- <span id="updater-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Updater<'a, OPCODE, Value>`

- <span id="updater-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Ioctl for Updater<'a, OPCODE, T>`

- <span id="updater-ioctl-type-output"></span>`type Output = ()`

- <span id="updater-ioctl-const-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="updater-ioctl-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md#opcode)

- <span id="updater-ioctl-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="updater-ioctl-output-from-ptr"></span>`unsafe fn output_from_ptr(_output: IoctlOutput, _ptr: *mut c::c_void) -> Result<()>` — [`IoctlOutput`](../index.md#ioctloutput), [`Result`](../../io/errno/index.md#result)

##### `impl<U> TryFrom for Updater<'a, OPCODE, Value>`

- <span id="updater-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="updater-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Updater<'a, OPCODE, Value>`

- <span id="updater-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="updater-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntegerSetter<const OPCODE: super::Opcode>`

```rust
struct IntegerSetter<const OPCODE: super::Opcode> {
    value: *mut c::c_void,
}
```

*Defined in [`rustix-1.1.2/src/ioctl/patterns.rs:216-221`](../../../../.source_1765633015/rustix-1.1.2/src/ioctl/patterns.rs#L216-L221)*

Implements an `ioctl` that passes an integer into the `ioctl`.

To compute a value for the `OPCODE` argument, see the functions in the
[`opcode`](../opcode/index.md) module.


#### Fields

- **`value`**: `*mut c::c_void`

  The value to pass in.
  
  For strict provenance preservation, this is a pointer.

#### Implementations

- <span id="integersetter-new-usize"></span>`const unsafe fn new_usize(value: usize) -> Self`

  Create a new integer `Ioctl` helper containing a `usize`.

  

  # Safety

  

   - `OPCODE` must provide a valid opcode.

   - For this opcode, it must expect an integer.

   - The integer is in the valid range for this opcode.

- <span id="integersetter-new-pointer"></span>`const unsafe fn new_pointer(value: *mut c::c_void) -> Self`

  Create a new integer `Ioctl` helper containing a `*mut c_void`.

  

  # Safety

  

   - `OPCODE` must provide a valid opcode.

   - For this opcode, it must expect an integer.

   - The integer is in the valid range for this opcode.

#### Trait Implementations

##### `impl Any for IntegerSetter<OPCODE>`

- <span id="integersetter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntegerSetter<OPCODE>`

- <span id="integersetter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntegerSetter<OPCODE>`

- <span id="integersetter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IntegerSetter<OPCODE>`

- <span id="integersetter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntegerSetter<OPCODE>`

- <span id="integersetter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ioctl for IntegerSetter<OPCODE>`

- <span id="integersetter-ioctl-type-output"></span>`type Output = ()`

- <span id="integersetter-ioctl-const-is-mutating"></span>`const IS_MUTATING: bool`

- <span id="integersetter-ioctl-opcode"></span>`fn opcode(&self) -> self::Opcode` — [`Opcode`](../index.md#opcode)

- <span id="integersetter-ioctl-as-ptr"></span>`fn as_ptr(&mut self) -> *mut c::c_void`

- <span id="integersetter-ioctl-output-from-ptr"></span>`unsafe fn output_from_ptr(_out: IoctlOutput, _extract_output: *mut c::c_void) -> Result<<Self as >::Output>` — [`IoctlOutput`](../index.md#ioctloutput), [`Result`](../../io/errno/index.md#result), [`Ioctl`](../index.md#ioctl)

##### `impl<U> TryFrom for IntegerSetter<OPCODE>`

- <span id="integersetter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="integersetter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntegerSetter<OPCODE>`

- <span id="integersetter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="integersetter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

