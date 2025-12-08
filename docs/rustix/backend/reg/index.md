*[rustix](../../index.md) / [backend](../index.md) / [reg](index.md)*

---

# Module `reg`

Encapsulation for system call arguments and return values.

The inline-asm code paths do some amount of reordering of arguments; to
ensure that we don't accidentally misroute an argument or return value, we
use distinct types for each argument index and return value.

# Safety

The `ToAsm` and `FromAsm` traits are unsafe to use; they should only be
used by the syscall code which executes actual syscall machine
instructions.

## Modules

- [`private`](private/index.md) - Seal our various traits using the technique documented [here].

## Structs

### `Opaque`

```rust
struct Opaque(c::c_void);
```

To preserve provenance, syscall arguments and return values are passed as
pointer types. They need a type to point to, so we define a custom private
type, to prevent it from being used for anything else.

### `A0`

```rust
struct A0(());
```

#### Trait Implementations

##### `impl ArgNumber for A0`

##### `impl Sealed for super::A0`

### `A1`

```rust
struct A1(());
```

#### Trait Implementations

##### `impl ArgNumber for A1`

##### `impl Sealed for super::A1`

### `A2`

```rust
struct A2(());
```

#### Trait Implementations

##### `impl ArgNumber for A2`

##### `impl Sealed for super::A2`

### `A3`

```rust
struct A3(());
```

#### Trait Implementations

##### `impl ArgNumber for A3`

##### `impl Sealed for super::A3`

### `A4`

```rust
struct A4(());
```

#### Trait Implementations

##### `impl ArgNumber for A4`

##### `impl Sealed for super::A4`

### `A5`

```rust
struct A5(());
```

#### Trait Implementations

##### `impl ArgNumber for A5`

##### `impl Sealed for super::A5`

### `R0`

```rust
struct R0(());
```

#### Trait Implementations

##### `impl RetNumber for R0`

##### `impl Sealed for super::R0`

### `ArgReg<'a, Num: ArgNumber>`

```rust
struct ArgReg<'a, Num: ArgNumber> {
    raw: *mut Opaque,
    _phantom: core::marker::PhantomData<(&'a (), Num)>,
}
```

Syscall arguments use register-sized types. We use a newtype to
discourage accidental misuse of the raw integer values.

This type doesn't implement `Clone` or `Copy`; it should be used exactly
once. And it has a lifetime to ensure that it doesn't outlive any resources
it might be pointing to.

#### Trait Implementations

##### `impl<'a, Num: super::ArgNumber> Sealed for super::ArgReg<'a, Num>`

##### `impl<'a, Num: ArgNumber> ToAsm for ArgReg<'a, Num>`

- `unsafe fn to_asm(self: Self) -> *mut Opaque` — [`Opaque`](#opaque)

### `RetReg<Num: RetNumber>`

```rust
struct RetReg<Num: RetNumber> {
    raw: *mut Opaque,
    _phantom: core::marker::PhantomData<Num>,
}
```

Syscall return values use register-sized types. We use a newtype to
discourage accidental misuse of the raw integer values.

This type doesn't implement `Clone` or `Copy`; it should be used exactly
once.

#### Implementations

- `fn decode_usize(self: Self) -> usize`

- `fn decode_raw_fd(self: Self) -> RawFd`

- `fn decode_c_int(self: Self) -> c::c_int`

- `fn decode_c_uint(self: Self) -> c::c_uint`

- `fn decode_void_star(self: Self) -> *mut c::c_void`

- `fn decode_u64(self: Self) -> u64`

- `fn decode_void(self: Self)`

- `fn decode_error_code(self: Self) -> u16`

- `fn is_nonzero(self: &Self) -> bool`

- `fn is_negative(self: &Self) -> bool`

- `fn is_in_range(self: &Self, range: Range<isize>) -> bool`

#### Trait Implementations

##### `impl<Num: RetNumber> FromAsm for RetReg<Num>`

- `unsafe fn from_asm(raw: *mut Opaque) -> Self` — [`Opaque`](#opaque)

##### `impl<Num: super::RetNumber> Sealed for super::RetReg<Num>`

### `SyscallNumber<'a>`

```rust
struct SyscallNumber<'a> {
    nr: usize,
    _phantom: core::marker::PhantomData<&'a ()>,
}
```

#### Trait Implementations

##### `impl<'a> Sealed for super::SyscallNumber<'a>`

##### `impl<'a> ToAsm for SyscallNumber<'a>`

- `unsafe fn to_asm(self: Self) -> *mut Opaque` — [`Opaque`](#opaque)

## Traits

### `ToAsm`

```rust
trait ToAsm: private::Sealed { ... }
```

#### Required Methods

- `fn to_asm(self: Self) -> *mut Opaque`

  Convert `self` to a `usize` ready to be passed to a syscall

### `FromAsm`

```rust
trait FromAsm: private::Sealed { ... }
```

#### Required Methods

- `fn from_asm(raw: *mut Opaque) -> Self`

  Convert `raw` from a value produced by a syscall machine instruction

### `ArgNumber`

```rust
trait ArgNumber: private::Sealed { ... }
```

### `RetNumber`

```rust
trait RetNumber: private::Sealed { ... }
```

## Functions

### `raw_arg`

```rust
fn raw_arg<'a, Num: ArgNumber>(raw: *mut Opaque) -> ArgReg<'a, Num>
```

Encode a system call argument as an `ArgReg`.

### `nr`

```rust
const fn nr<'a>(nr: u32) -> SyscallNumber<'a>
```

Encode a system call number (a `__NR_*` constant) as a `SyscallNumber`.

