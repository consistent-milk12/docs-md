*[gimli](../index.md) / [arch](index.md)*

---

# Module `arch`

## Contents

- [Structs](#structs)
  - [`Arm`](#arm)
  - [`AArch64`](#aarch64)
  - [`LoongArch`](#loongarch)
  - [`MIPS`](#mips)
  - [`RiscV`](#riscv)
  - [`X86`](#x86)
  - [`X86_64`](#x86_64)
  - [`PowerPc64`](#powerpc64)
- [Macros](#macros)
  - [`registers!`](#registers)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Arm`](#arm) | struct | ARM architecture specific definitions. |
| [`AArch64`](#aarch64) | struct | ARM 64-bit (AArch64) architecture specific definitions. |
| [`LoongArch`](#loongarch) | struct | LoongArch architecture specific definitions. |
| [`MIPS`](#mips) | struct | MIPS architecture specific definitions. |
| [`RiscV`](#riscv) | struct | RISC-V architecture specific definitions. |
| [`X86`](#x86) | struct | Intel i386 architecture specific definitions. |
| [`X86_64`](#x86_64) | struct | AMD64 architecture specific definitions. |
| [`PowerPc64`](#powerpc64) | struct | PowerPC 64bit |
| [`registers!`](#registers) | macro |  |

## Structs

### `Arm`

```rust
struct Arm;
```

ARM architecture specific definitions.

See [DWARF for the ARM Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf32/aadwarf32.rst).

#### Implementations

- <span id="arm-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="arm-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for Arm`

- <span id="arm-clone"></span>`fn clone(&self) -> Arm` — [`Arm`](../index.md)

##### `impl Copy for Arm`

##### `impl Debug for Arm`

- <span id="arm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AArch64`

```rust
struct AArch64;
```

ARM 64-bit (AArch64) architecture specific definitions.

See [DWARF for the ARM 64-bit Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst).

#### Implementations

- <span id="aarch64-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="aarch64-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for AArch64`

- <span id="aarch64-clone"></span>`fn clone(&self) -> AArch64` — [`AArch64`](../index.md)

##### `impl Copy for AArch64`

##### `impl Debug for AArch64`

- <span id="aarch64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LoongArch`

```rust
struct LoongArch;
```

LoongArch architecture specific definitions.

See [LoongArch ELF psABI specification](https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html).

#### Implementations

- <span id="loongarch-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="loongarch-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for LoongArch`

- <span id="loongarch-clone"></span>`fn clone(&self) -> LoongArch` — [`LoongArch`](../index.md)

##### `impl Copy for LoongArch`

##### `impl Debug for LoongArch`

- <span id="loongarch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MIPS`

```rust
struct MIPS;
```

MIPS architecture specific definitions.

See [MIPS Details](https://en.wikibooks.org/wiki/MIPS_Assembly/MIPS_Details).

#### Implementations

- <span id="mips-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="mips-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for MIPS`

- <span id="mips-clone"></span>`fn clone(&self) -> MIPS` — [`MIPS`](../index.md)

##### `impl Copy for MIPS`

##### `impl Debug for MIPS`

- <span id="mips-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RiscV`

```rust
struct RiscV;
```

RISC-V architecture specific definitions.

See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc).

#### Implementations

- <span id="riscv-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="riscv-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for RiscV`

- <span id="riscv-clone"></span>`fn clone(&self) -> RiscV` — [`RiscV`](../index.md)

##### `impl Copy for RiscV`

##### `impl Debug for RiscV`

- <span id="riscv-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `X86`

```rust
struct X86;
```

Intel i386 architecture specific definitions.

See section 2.4.2 of the [i386 psABI](https://gitlab.com/x86-psABIs/i386-ABI).

#### Implementations

- <span id="x86-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="x86-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for X86`

- <span id="x86-clone"></span>`fn clone(&self) -> X86` — [`X86`](../index.md)

##### `impl Copy for X86`

##### `impl Debug for X86`

- <span id="x86-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `X86_64`

```rust
struct X86_64;
```

AMD64 architecture specific definitions.

See section 3.6.2 of the [x86-64 psABI](https://gitlab.com/x86-psABIs/x86-64-ABI).

#### Implementations

- <span id="x86-64-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="x86-64-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for X86_64`

- <span id="x86-64-clone"></span>`fn clone(&self) -> X86_64` — [`X86_64`](../index.md)

##### `impl Copy for X86_64`

##### `impl Debug for X86_64`

- <span id="x86-64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PowerPc64`

```rust
struct PowerPc64;
```

PowerPC 64bit

See [64-bit ELF ABI Specification for OpenPOWER Architecture](https://openpowerfoundation.org/specifications/64bitelfabi/).

#### Implementations

- <span id="powerpc64-register-name"></span>`fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- <span id="powerpc64-name-to-register"></span>`fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for PowerPc64`

- <span id="powerpc64-clone"></span>`fn clone(&self) -> PowerPc64` — [`PowerPc64`](../index.md)

##### `impl Copy for PowerPc64`

##### `impl Debug for PowerPc64`

- <span id="powerpc64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `registers!`

