*[gimli](../index.md) / [arch](index.md)*

---

# Module `arch`

## Structs

### `Arm`

```rust
struct Arm;
```

ARM architecture specific definitions.

See [DWARF for the ARM Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf32/aadwarf32.rst).

#### Implementations

- `fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- `fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for Arm`

- `fn clone(self: &Self) -> Arm` — [`Arm`](../index.md)

##### `impl Copy for Arm`

##### `impl Debug for Arm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AArch64`

```rust
struct AArch64;
```

ARM 64-bit (AArch64) architecture specific definitions.

See [DWARF for the ARM 64-bit Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst).

#### Implementations

- `const X0: Register`

- `const X1: Register`

- `const X2: Register`

- `const X3: Register`

- `const X4: Register`

- `const X5: Register`

- `const X6: Register`

- `const X7: Register`

- `const X8: Register`

- `const X9: Register`

- `const X10: Register`

- `const X11: Register`

- `const X12: Register`

- `const X13: Register`

- `const X14: Register`

- `const X15: Register`

- `const X16: Register`

- `const X17: Register`

- `const X18: Register`

- `const X19: Register`

- `const X20: Register`

- `const X21: Register`

- `const X22: Register`

- `const X23: Register`

- `const X24: Register`

- `const X25: Register`

- `const X26: Register`

- `const X27: Register`

- `const X28: Register`

- `const X29: Register`

- `const X30: Register`

- `const SP: Register`

- `const PC: Register`

- `const ELR_MODE: Register`

- `const RA_SIGN_STATE: Register`

- `const TPIDRRO_EL0: Register`

- `const TPIDR_EL0: Register`

- `const TPIDR_EL1: Register`

- `const TPIDR_EL2: Register`

- `const TPIDR_EL3: Register`

- `const VG: Register`

- `const FFR: Register`

- `const P0: Register`

- `const P1: Register`

- `const P2: Register`

- `const P3: Register`

- `const P4: Register`

- `const P5: Register`

- `const P6: Register`

- `const P7: Register`

- `const P8: Register`

- `const P9: Register`

- `const P10: Register`

- `const P11: Register`

- `const P12: Register`

- `const P13: Register`

- `const P14: Register`

- `const P15: Register`

- `const V0: Register`

- `const V1: Register`

- `const V2: Register`

- `const V3: Register`

- `const V4: Register`

- `const V5: Register`

- `const V6: Register`

- `const V7: Register`

- `const V8: Register`

- `const V9: Register`

- `const V10: Register`

- `const V11: Register`

- `const V12: Register`

- `const V13: Register`

- `const V14: Register`

- `const V15: Register`

- `const V16: Register`

- `const V17: Register`

- `const V18: Register`

- `const V19: Register`

- `const V20: Register`

- `const V21: Register`

- `const V22: Register`

- `const V23: Register`

- `const V24: Register`

- `const V25: Register`

- `const V26: Register`

- `const V27: Register`

- `const V28: Register`

- `const V29: Register`

- `const V30: Register`

- `const V31: Register`

- `const Z0: Register`

- `const Z1: Register`

- `const Z2: Register`

- `const Z3: Register`

- `const Z4: Register`

- `const Z5: Register`

- `const Z6: Register`

- `const Z7: Register`

- `const Z8: Register`

- `const Z9: Register`

- `const Z10: Register`

- `const Z11: Register`

- `const Z12: Register`

- `const Z13: Register`

- `const Z14: Register`

- `const Z15: Register`

- `const Z16: Register`

- `const Z17: Register`

- `const Z18: Register`

- `const Z19: Register`

- `const Z20: Register`

- `const Z21: Register`

- `const Z22: Register`

- `const Z23: Register`

- `const Z24: Register`

- `const Z25: Register`

- `const Z26: Register`

- `const Z27: Register`

- `const Z28: Register`

- `const Z29: Register`

- `const Z30: Register`

- `const Z31: Register`

#### Trait Implementations

##### `impl Clone for AArch64`

- `fn clone(self: &Self) -> AArch64` — [`AArch64`](../index.md)

##### `impl Copy for AArch64`

##### `impl Debug for AArch64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LoongArch`

```rust
struct LoongArch;
```

LoongArch architecture specific definitions.

See [LoongArch ELF psABI specification](https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html).

#### Implementations

- `fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- `fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for LoongArch`

- `fn clone(self: &Self) -> LoongArch` — [`LoongArch`](../index.md)

##### `impl Copy for LoongArch`

##### `impl Debug for LoongArch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MIPS`

```rust
struct MIPS;
```

MIPS architecture specific definitions.

See [MIPS Details](https://en.wikibooks.org/wiki/MIPS_Assembly/MIPS_Details).

#### Implementations

- `fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- `fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for MIPS`

- `fn clone(self: &Self) -> MIPS` — [`MIPS`](../index.md)

##### `impl Copy for MIPS`

##### `impl Debug for MIPS`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RiscV`

```rust
struct RiscV;
```

RISC-V architecture specific definitions.

See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc).

#### Implementations

- `fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- `fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for RiscV`

- `fn clone(self: &Self) -> RiscV` — [`RiscV`](../index.md)

##### `impl Copy for RiscV`

##### `impl Debug for RiscV`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `X86`

```rust
struct X86;
```

Intel i386 architecture specific definitions.

See section 2.4.2 of the [i386 psABI](https://gitlab.com/x86-psABIs/i386-ABI).

#### Implementations

- `fn register_name(register: Register) -> Option<&'static str>` — [`Register`](../index.md)

- `fn name_to_register(value: &str) -> Option<Register>` — [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for X86`

- `fn clone(self: &Self) -> X86` — [`X86`](../index.md)

##### `impl Copy for X86`

##### `impl Debug for X86`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `X86_64`

```rust
struct X86_64;
```

AMD64 architecture specific definitions.

See section 3.6.2 of the [x86-64 psABI](https://gitlab.com/x86-psABIs/x86-64-ABI).

#### Implementations

- `const RAX: Register`

- `const RDX: Register`

- `const RCX: Register`

- `const RBX: Register`

- `const RSI: Register`

- `const RDI: Register`

- `const RBP: Register`

- `const RSP: Register`

- `const R8: Register`

- `const R9: Register`

- `const R10: Register`

- `const R11: Register`

- `const R12: Register`

- `const R13: Register`

- `const R14: Register`

- `const R15: Register`

- `const RA: Register`

- `const XMM0: Register`

- `const XMM1: Register`

- `const XMM2: Register`

- `const XMM3: Register`

- `const XMM4: Register`

- `const XMM5: Register`

- `const XMM6: Register`

- `const XMM7: Register`

- `const XMM8: Register`

- `const XMM9: Register`

- `const XMM10: Register`

- `const XMM11: Register`

- `const XMM12: Register`

- `const XMM13: Register`

- `const XMM14: Register`

- `const XMM15: Register`

- `const ST0: Register`

- `const ST1: Register`

- `const ST2: Register`

- `const ST3: Register`

- `const ST4: Register`

- `const ST5: Register`

- `const ST6: Register`

- `const ST7: Register`

- `const MM0: Register`

- `const MM1: Register`

- `const MM2: Register`

- `const MM3: Register`

- `const MM4: Register`

- `const MM5: Register`

- `const MM6: Register`

- `const MM7: Register`

- `const RFLAGS: Register`

- `const ES: Register`

- `const CS: Register`

- `const SS: Register`

- `const DS: Register`

- `const FS: Register`

- `const GS: Register`

- `const FS_BASE: Register`

- `const GS_BASE: Register`

- `const TR: Register`

- `const LDTR: Register`

- `const MXCSR: Register`

- `const FCW: Register`

- `const FSW: Register`

- `const XMM16: Register`

- `const XMM17: Register`

- `const XMM18: Register`

- `const XMM19: Register`

- `const XMM20: Register`

- `const XMM21: Register`

- `const XMM22: Register`

- `const XMM23: Register`

- `const XMM24: Register`

- `const XMM25: Register`

- `const XMM26: Register`

- `const XMM27: Register`

- `const XMM28: Register`

- `const XMM29: Register`

- `const XMM30: Register`

- `const XMM31: Register`

- `const K0: Register`

- `const K1: Register`

- `const K2: Register`

- `const K3: Register`

- `const K4: Register`

- `const K5: Register`

- `const K6: Register`

- `const K7: Register`

- `const R16: Register`

- `const R17: Register`

- `const R18: Register`

- `const R19: Register`

- `const R20: Register`

- `const R21: Register`

- `const R22: Register`

- `const R23: Register`

- `const R24: Register`

- `const R25: Register`

- `const R26: Register`

- `const R27: Register`

- `const R28: Register`

- `const R29: Register`

- `const R30: Register`

- `const R31: Register`

- `const TMM0: Register`

- `const TMM1: Register`

- `const TMM2: Register`

- `const TMM3: Register`

- `const TMM4: Register`

- `const TMM5: Register`

- `const TMM6: Register`

- `const TMM7: Register`

- `const TILECFG: Register`

#### Trait Implementations

##### `impl Clone for X86_64`

- `fn clone(self: &Self) -> X86_64` — [`X86_64`](../index.md)

##### `impl Copy for X86_64`

##### `impl Debug for X86_64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PowerPc64`

```rust
struct PowerPc64;
```

PowerPC 64bit

See [64-bit ELF ABI Specification for OpenPOWER Architecture](https://openpowerfoundation.org/specifications/64bitelfabi/).

#### Implementations

- `const R0: Register`

- `const R1: Register`

- `const R2: Register`

- `const R3: Register`

- `const R4: Register`

- `const R5: Register`

- `const R6: Register`

- `const R7: Register`

- `const R8: Register`

- `const R9: Register`

- `const R10: Register`

- `const R11: Register`

- `const R12: Register`

- `const R13: Register`

- `const R14: Register`

- `const R15: Register`

- `const R16: Register`

- `const R17: Register`

- `const R18: Register`

- `const R19: Register`

- `const R20: Register`

- `const R21: Register`

- `const R22: Register`

- `const R23: Register`

- `const R24: Register`

- `const R25: Register`

- `const R26: Register`

- `const R27: Register`

- `const R28: Register`

- `const R29: Register`

- `const R30: Register`

- `const R31: Register`

- `const F0: Register`

- `const F1: Register`

- `const F2: Register`

- `const F3: Register`

- `const F4: Register`

- `const F5: Register`

- `const F6: Register`

- `const F7: Register`

- `const F8: Register`

- `const F9: Register`

- `const F10: Register`

- `const F11: Register`

- `const F12: Register`

- `const F13: Register`

- `const F14: Register`

- `const F15: Register`

- `const F16: Register`

- `const F17: Register`

- `const F18: Register`

- `const F19: Register`

- `const F20: Register`

- `const F21: Register`

- `const F22: Register`

- `const F23: Register`

- `const F24: Register`

- `const F25: Register`

- `const F26: Register`

- `const F27: Register`

- `const F28: Register`

- `const F29: Register`

- `const F30: Register`

- `const F31: Register`

- `const LR: Register`

- `const CTR: Register`

- `const CR0: Register`

- `const CR1: Register`

- `const CR2: Register`

- `const CR3: Register`

- `const CR4: Register`

- `const CR5: Register`

- `const CR6: Register`

- `const CR7: Register`

- `const XER: Register`

- `const VR0: Register`

- `const VR1: Register`

- `const VR2: Register`

- `const VR3: Register`

- `const VR4: Register`

- `const VR5: Register`

- `const VR6: Register`

- `const VR7: Register`

- `const VR8: Register`

- `const VR9: Register`

- `const VR10: Register`

- `const VR11: Register`

- `const VR12: Register`

- `const VR13: Register`

- `const VR14: Register`

- `const VR15: Register`

- `const VR16: Register`

- `const VR17: Register`

- `const VR18: Register`

- `const VR19: Register`

- `const VR20: Register`

- `const VR21: Register`

- `const VR22: Register`

- `const VR23: Register`

- `const VR24: Register`

- `const VR25: Register`

- `const VR26: Register`

- `const VR27: Register`

- `const VR28: Register`

- `const VR29: Register`

- `const VR30: Register`

- `const VR31: Register`

- `const VSCR: Register`

- `const TFHAR: Register`

- `const TFIAR: Register`

- `const TEXASR: Register`

#### Trait Implementations

##### `impl Clone for PowerPc64`

- `fn clone(self: &Self) -> PowerPc64` — [`PowerPc64`](../index.md)

##### `impl Copy for PowerPc64`

##### `impl Debug for PowerPc64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Macros

### `registers!`

