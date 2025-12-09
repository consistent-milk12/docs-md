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

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Opaque`](#opaque)
  - [`A0`](#a0)
  - [`A1`](#a1)
  - [`A2`](#a2)
  - [`A3`](#a3)
  - [`A4`](#a4)
  - [`A5`](#a5)
  - [`R0`](#r0)
  - [`ArgReg`](#argreg)
  - [`RetReg`](#retreg)
  - [`SyscallNumber`](#syscallnumber)
- [Traits](#traits)
  - [`ToAsm`](#toasm)
  - [`FromAsm`](#fromasm)
  - [`ArgNumber`](#argnumber)
  - [`RetNumber`](#retnumber)
- [Functions](#functions)
  - [`raw_arg`](#raw_arg)
  - [`nr`](#nr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | Seal our various traits using the technique documented [here]. |
| [`Opaque`](#opaque) | struct | To preserve provenance, syscall arguments and return values are passed as pointer types. |
| [`A0`](#a0) | struct |  |
| [`A1`](#a1) | struct |  |
| [`A2`](#a2) | struct |  |
| [`A3`](#a3) | struct |  |
| [`A4`](#a4) | struct |  |
| [`A5`](#a5) | struct |  |
| [`R0`](#r0) | struct |  |
| [`ArgReg`](#argreg) | struct | Syscall arguments use register-sized types. |
| [`RetReg`](#retreg) | struct | Syscall return values use register-sized types. |
| [`SyscallNumber`](#syscallnumber) | struct |  |
| [`ToAsm`](#toasm) | trait |  |
| [`FromAsm`](#fromasm) | trait |  |
| [`ArgNumber`](#argnumber) | trait |  |
| [`RetNumber`](#retnumber) | trait |  |
| [`raw_arg`](#raw_arg) | fn | Encode a system call argument as an `ArgReg`. |
| [`nr`](#nr) | fn | Encode a system call number (a `__NR_*` constant) as a `SyscallNumber`. |

## Modules

- [`private`](private/index.md) — Seal our various traits using the technique documented [here].

## Structs

### `Opaque`

```rust
struct Opaque(c::c_void);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:49`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L49)*

To preserve provenance, syscall arguments and return values are passed as
pointer types. They need a type to point to, so we define a custom private
type, to prevent it from being used for anything else.

### `A0`

```rust
struct A0(());
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:52`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L52)*

#### Trait Implementations

##### `impl ArgNumber for A0`

##### `impl Sealed for super::A0`

### `A1`

```rust
struct A1(());
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:53`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L53)*

#### Trait Implementations

##### `impl ArgNumber for A1`

##### `impl Sealed for super::A1`

### `A2`

```rust
struct A2(());
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:54`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L54)*

#### Trait Implementations

##### `impl ArgNumber for A2`

##### `impl Sealed for super::A2`

### `A3`

```rust
struct A3(());
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:55`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L55)*

#### Trait Implementations

##### `impl ArgNumber for A3`

##### `impl Sealed for super::A3`

### `A4`

```rust
struct A4(());
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:56`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L56)*

#### Trait Implementations

##### `impl ArgNumber for A4`

##### `impl Sealed for super::A4`

### `A5`

```rust
struct A5(());
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:57`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L57)*

#### Trait Implementations

##### `impl ArgNumber for A5`

##### `impl Sealed for super::A5`

### `R0`

```rust
struct R0(());
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:76`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L76)*

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

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:89-92`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L89-L92)*

Syscall arguments use register-sized types. We use a newtype to
discourage accidental misuse of the raw integer values.

This type doesn't implement `Clone` or `Copy`; it should be used exactly
once. And it has a lifetime to ensure that it doesn't outlive any resources
it might be pointing to.

#### Trait Implementations

##### `impl<'a, Num: super::ArgNumber> Sealed for super::ArgReg<'a, Num>`

##### `impl<'a, Num: ArgNumber> ToAsm for ArgReg<'a, Num>`

- <span id="argreg-to-asm"></span>`unsafe fn to_asm(self) -> *mut Opaque` — [`Opaque`](#opaque)

### `RetReg<Num: RetNumber>`

```rust
struct RetReg<Num: RetNumber> {
    raw: *mut Opaque,
    _phantom: core::marker::PhantomData<Num>,
}
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:108-111`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L108-L111)*

Syscall return values use register-sized types. We use a newtype to
discourage accidental misuse of the raw integer values.

This type doesn't implement `Clone` or `Copy`; it should be used exactly
once.

#### Implementations

- <span id="retreg-decode-usize"></span>`fn decode_usize(self) -> usize`

- <span id="retreg-decode-raw-fd"></span>`fn decode_raw_fd(self) -> RawFd` — [`RawFd`](../../maybe_polyfill/os/fd/index.md)

- <span id="retreg-decode-c-int"></span>`fn decode_c_int(self) -> c::c_int`

- <span id="retreg-decode-c-uint"></span>`fn decode_c_uint(self) -> c::c_uint`

- <span id="retreg-decode-void-star"></span>`fn decode_void_star(self) -> *mut c::c_void`

- <span id="retreg-decode-u64"></span>`fn decode_u64(self) -> u64`

- <span id="retreg-decode-void"></span>`fn decode_void(self)`

- <span id="retreg-decode-error-code"></span>`fn decode_error_code(self) -> u16`

- <span id="retreg-is-nonzero"></span>`fn is_nonzero(&self) -> bool`

- <span id="retreg-is-negative"></span>`fn is_negative(&self) -> bool`

- <span id="retreg-is-in-range"></span>`fn is_in_range(&self, range: Range<isize>) -> bool`

#### Trait Implementations

##### `impl<Num: RetNumber> FromAsm for RetReg<Num>`

- <span id="retreg-from-asm"></span>`unsafe fn from_asm(raw: *mut Opaque) -> Self` — [`Opaque`](#opaque)

##### `impl<Num: super::RetNumber> Sealed for super::RetReg<Num>`

### `SyscallNumber<'a>`

```rust
struct SyscallNumber<'a> {
    nr: usize,
    _phantom: core::marker::PhantomData<&'a ()>,
}
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:208-211`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L208-L211)*

#### Trait Implementations

##### `impl Sealed for super::SyscallNumber<'a>`

##### `impl ToAsm for SyscallNumber<'a>`

- <span id="syscallnumber-to-asm"></span>`unsafe fn to_asm(self) -> *mut Opaque` — [`Opaque`](#opaque)

## Traits

### `ToAsm`

```rust
trait ToAsm: private::Sealed { ... }
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:20-30`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L20-L30)*

#### Required Methods

- `fn to_asm(self) -> *mut Opaque`

  Convert `self` to a `usize` ready to be passed to a syscall

#### Implementors

- [`ArgReg`](#argreg)
- [`SyscallNumber`](#syscallnumber)

### `FromAsm`

```rust
trait FromAsm: private::Sealed { ... }
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:32-42`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L32-L42)*

#### Required Methods

- `fn from_asm(raw: *mut Opaque) -> Self`

  Convert `raw` from a value produced by a syscall machine instruction

#### Implementors

- [`RetReg`](#retreg)

### `ArgNumber`

```rust
trait ArgNumber: private::Sealed { ... }
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:63`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L63)*

#### Implementors

- [`A0`](#a0)
- [`A1`](#a1)
- [`A2`](#a2)
- [`A3`](#a3)
- [`A4`](#a4)
- [`A5`](#a5)

### `RetNumber`

```rust
trait RetNumber: private::Sealed { ... }
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:78`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L78)*

#### Implementors

- [`R0`](#r0)

## Functions

### `raw_arg`

```rust
fn raw_arg<'a, Num: ArgNumber>(raw: *mut Opaque) -> ArgReg<'a, Num>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:222-227`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L222-L227)*

Encode a system call argument as an `ArgReg`.

### `nr`

```rust
const fn nr<'a>(nr: u32) -> SyscallNumber<'a>
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/reg.rs:231-236`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/reg.rs#L231-L236)*

Encode a system call number (a `__NR_*` constant) as a `SyscallNumber`.

