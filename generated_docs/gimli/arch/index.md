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

- <span id="arm-r0"></span>`const R0: Register`

- <span id="arm-r1"></span>`const R1: Register`

- <span id="arm-r2"></span>`const R2: Register`

- <span id="arm-r3"></span>`const R3: Register`

- <span id="arm-r4"></span>`const R4: Register`

- <span id="arm-r5"></span>`const R5: Register`

- <span id="arm-r6"></span>`const R6: Register`

- <span id="arm-r7"></span>`const R7: Register`

- <span id="arm-r8"></span>`const R8: Register`

- <span id="arm-r9"></span>`const R9: Register`

- <span id="arm-r10"></span>`const R10: Register`

- <span id="arm-r11"></span>`const R11: Register`

- <span id="arm-r12"></span>`const R12: Register`

- <span id="arm-r13"></span>`const R13: Register`

- <span id="arm-r14"></span>`const R14: Register`

- <span id="arm-r15"></span>`const R15: Register`

- <span id="arm-wcgr0"></span>`const WCGR0: Register`

- <span id="arm-wcgr1"></span>`const WCGR1: Register`

- <span id="arm-wcgr2"></span>`const WCGR2: Register`

- <span id="arm-wcgr3"></span>`const WCGR3: Register`

- <span id="arm-wcgr4"></span>`const WCGR4: Register`

- <span id="arm-wcgr5"></span>`const WCGR5: Register`

- <span id="arm-wcgr6"></span>`const WCGR6: Register`

- <span id="arm-wcgr7"></span>`const WCGR7: Register`

- <span id="arm-wr0"></span>`const WR0: Register`

- <span id="arm-wr1"></span>`const WR1: Register`

- <span id="arm-wr2"></span>`const WR2: Register`

- <span id="arm-wr3"></span>`const WR3: Register`

- <span id="arm-wr4"></span>`const WR4: Register`

- <span id="arm-wr5"></span>`const WR5: Register`

- <span id="arm-wr6"></span>`const WR6: Register`

- <span id="arm-wr7"></span>`const WR7: Register`

- <span id="arm-wr8"></span>`const WR8: Register`

- <span id="arm-wr9"></span>`const WR9: Register`

- <span id="arm-wr10"></span>`const WR10: Register`

- <span id="arm-wr11"></span>`const WR11: Register`

- <span id="arm-wr12"></span>`const WR12: Register`

- <span id="arm-wr13"></span>`const WR13: Register`

- <span id="arm-wr14"></span>`const WR14: Register`

- <span id="arm-wr15"></span>`const WR15: Register`

- <span id="arm-spsr"></span>`const SPSR: Register`

- <span id="arm-spsr-fiq"></span>`const SPSR_FIQ: Register`

- <span id="arm-spsr-irq"></span>`const SPSR_IRQ: Register`

- <span id="arm-spsr-abt"></span>`const SPSR_ABT: Register`

- <span id="arm-spsr-und"></span>`const SPSR_UND: Register`

- <span id="arm-spsr-svc"></span>`const SPSR_SVC: Register`

- <span id="arm-ra-auth-code"></span>`const RA_AUTH_CODE: Register`

- <span id="arm-r8-usr"></span>`const R8_USR: Register`

- <span id="arm-r9-usr"></span>`const R9_USR: Register`

- <span id="arm-r10-usr"></span>`const R10_USR: Register`

- <span id="arm-r11-usr"></span>`const R11_USR: Register`

- <span id="arm-r12-usr"></span>`const R12_USR: Register`

- <span id="arm-r13-usr"></span>`const R13_USR: Register`

- <span id="arm-r14-usr"></span>`const R14_USR: Register`

- <span id="arm-r8-fiq"></span>`const R8_FIQ: Register`

- <span id="arm-r9-fiq"></span>`const R9_FIQ: Register`

- <span id="arm-r10-fiq"></span>`const R10_FIQ: Register`

- <span id="arm-r11-fiq"></span>`const R11_FIQ: Register`

- <span id="arm-r12-fiq"></span>`const R12_FIQ: Register`

- <span id="arm-r13-fiq"></span>`const R13_FIQ: Register`

- <span id="arm-r14-fiq"></span>`const R14_FIQ: Register`

- <span id="arm-r13-irq"></span>`const R13_IRQ: Register`

- <span id="arm-r14-irq"></span>`const R14_IRQ: Register`

- <span id="arm-r13-abt"></span>`const R13_ABT: Register`

- <span id="arm-r14-abt"></span>`const R14_ABT: Register`

- <span id="arm-r13-und"></span>`const R13_UND: Register`

- <span id="arm-r14-und"></span>`const R14_UND: Register`

- <span id="arm-r13-svc"></span>`const R13_SVC: Register`

- <span id="arm-r14-svc"></span>`const R14_SVC: Register`

- <span id="arm-wc0"></span>`const WC0: Register`

- <span id="arm-wc1"></span>`const WC1: Register`

- <span id="arm-wc2"></span>`const WC2: Register`

- <span id="arm-wc3"></span>`const WC3: Register`

- <span id="arm-wc4"></span>`const WC4: Register`

- <span id="arm-wc5"></span>`const WC5: Register`

- <span id="arm-wc6"></span>`const WC6: Register`

- <span id="arm-wc7"></span>`const WC7: Register`

- <span id="arm-d0"></span>`const D0: Register`

- <span id="arm-d1"></span>`const D1: Register`

- <span id="arm-d2"></span>`const D2: Register`

- <span id="arm-d3"></span>`const D3: Register`

- <span id="arm-d4"></span>`const D4: Register`

- <span id="arm-d5"></span>`const D5: Register`

- <span id="arm-d6"></span>`const D6: Register`

- <span id="arm-d7"></span>`const D7: Register`

- <span id="arm-d8"></span>`const D8: Register`

- <span id="arm-d9"></span>`const D9: Register`

- <span id="arm-d10"></span>`const D10: Register`

- <span id="arm-d11"></span>`const D11: Register`

- <span id="arm-d12"></span>`const D12: Register`

- <span id="arm-d13"></span>`const D13: Register`

- <span id="arm-d14"></span>`const D14: Register`

- <span id="arm-d15"></span>`const D15: Register`

- <span id="arm-d16"></span>`const D16: Register`

- <span id="arm-d17"></span>`const D17: Register`

- <span id="arm-d18"></span>`const D18: Register`

- <span id="arm-d19"></span>`const D19: Register`

- <span id="arm-d20"></span>`const D20: Register`

- <span id="arm-d21"></span>`const D21: Register`

- <span id="arm-d22"></span>`const D22: Register`

- <span id="arm-d23"></span>`const D23: Register`

- <span id="arm-d24"></span>`const D24: Register`

- <span id="arm-d25"></span>`const D25: Register`

- <span id="arm-d26"></span>`const D26: Register`

- <span id="arm-d27"></span>`const D27: Register`

- <span id="arm-d28"></span>`const D28: Register`

- <span id="arm-d29"></span>`const D29: Register`

- <span id="arm-d30"></span>`const D30: Register`

- <span id="arm-d31"></span>`const D31: Register`

- <span id="arm-tpidruro"></span>`const TPIDRURO: Register`

- <span id="arm-tpidrurw"></span>`const TPIDRURW: Register`

- <span id="arm-tpidpr"></span>`const TPIDPR: Register`

- <span id="arm-htpidpr"></span>`const HTPIDPR: Register`

- <span id="arm-sp"></span>`const SP: Register`

- <span id="arm-lr"></span>`const LR: Register`

- <span id="arm-pc"></span>`const PC: Register`

- <span id="arm-acc0"></span>`const ACC0: Register`

- <span id="arm-acc1"></span>`const ACC1: Register`

- <span id="arm-acc2"></span>`const ACC2: Register`

- <span id="arm-acc3"></span>`const ACC3: Register`

- <span id="arm-acc4"></span>`const ACC4: Register`

- <span id="arm-acc5"></span>`const ACC5: Register`

- <span id="arm-acc6"></span>`const ACC6: Register`

- <span id="arm-acc7"></span>`const ACC7: Register`

- <span id="arm-s0"></span>`const S0: Register`

- <span id="arm-s1"></span>`const S1: Register`

- <span id="arm-s2"></span>`const S2: Register`

- <span id="arm-s3"></span>`const S3: Register`

- <span id="arm-s4"></span>`const S4: Register`

- <span id="arm-s5"></span>`const S5: Register`

- <span id="arm-s6"></span>`const S6: Register`

- <span id="arm-s7"></span>`const S7: Register`

- <span id="arm-s8"></span>`const S8: Register`

- <span id="arm-s9"></span>`const S9: Register`

- <span id="arm-s10"></span>`const S10: Register`

- <span id="arm-s11"></span>`const S11: Register`

- <span id="arm-s12"></span>`const S12: Register`

- <span id="arm-s13"></span>`const S13: Register`

- <span id="arm-s14"></span>`const S14: Register`

- <span id="arm-s15"></span>`const S15: Register`

- <span id="arm-s16"></span>`const S16: Register`

- <span id="arm-s17"></span>`const S17: Register`

- <span id="arm-s18"></span>`const S18: Register`

- <span id="arm-s19"></span>`const S19: Register`

- <span id="arm-s20"></span>`const S20: Register`

- <span id="arm-s21"></span>`const S21: Register`

- <span id="arm-s22"></span>`const S22: Register`

- <span id="arm-s23"></span>`const S23: Register`

- <span id="arm-s24"></span>`const S24: Register`

- <span id="arm-s25"></span>`const S25: Register`

- <span id="arm-s26"></span>`const S26: Register`

- <span id="arm-s27"></span>`const S27: Register`

- <span id="arm-s28"></span>`const S28: Register`

- <span id="arm-s29"></span>`const S29: Register`

- <span id="arm-s30"></span>`const S30: Register`

- <span id="arm-s31"></span>`const S31: Register`

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

- <span id="aarch64-x0"></span>`const X0: Register`

- <span id="aarch64-x1"></span>`const X1: Register`

- <span id="aarch64-x2"></span>`const X2: Register`

- <span id="aarch64-x3"></span>`const X3: Register`

- <span id="aarch64-x4"></span>`const X4: Register`

- <span id="aarch64-x5"></span>`const X5: Register`

- <span id="aarch64-x6"></span>`const X6: Register`

- <span id="aarch64-x7"></span>`const X7: Register`

- <span id="aarch64-x8"></span>`const X8: Register`

- <span id="aarch64-x9"></span>`const X9: Register`

- <span id="aarch64-x10"></span>`const X10: Register`

- <span id="aarch64-x11"></span>`const X11: Register`

- <span id="aarch64-x12"></span>`const X12: Register`

- <span id="aarch64-x13"></span>`const X13: Register`

- <span id="aarch64-x14"></span>`const X14: Register`

- <span id="aarch64-x15"></span>`const X15: Register`

- <span id="aarch64-x16"></span>`const X16: Register`

- <span id="aarch64-x17"></span>`const X17: Register`

- <span id="aarch64-x18"></span>`const X18: Register`

- <span id="aarch64-x19"></span>`const X19: Register`

- <span id="aarch64-x20"></span>`const X20: Register`

- <span id="aarch64-x21"></span>`const X21: Register`

- <span id="aarch64-x22"></span>`const X22: Register`

- <span id="aarch64-x23"></span>`const X23: Register`

- <span id="aarch64-x24"></span>`const X24: Register`

- <span id="aarch64-x25"></span>`const X25: Register`

- <span id="aarch64-x26"></span>`const X26: Register`

- <span id="aarch64-x27"></span>`const X27: Register`

- <span id="aarch64-x28"></span>`const X28: Register`

- <span id="aarch64-x29"></span>`const X29: Register`

- <span id="aarch64-x30"></span>`const X30: Register`

- <span id="aarch64-sp"></span>`const SP: Register`

- <span id="aarch64-pc"></span>`const PC: Register`

- <span id="aarch64-elr-mode"></span>`const ELR_MODE: Register`

- <span id="aarch64-ra-sign-state"></span>`const RA_SIGN_STATE: Register`

- <span id="aarch64-tpidrro-el0"></span>`const TPIDRRO_EL0: Register`

- <span id="aarch64-tpidr-el0"></span>`const TPIDR_EL0: Register`

- <span id="aarch64-tpidr-el1"></span>`const TPIDR_EL1: Register`

- <span id="aarch64-tpidr-el2"></span>`const TPIDR_EL2: Register`

- <span id="aarch64-tpidr-el3"></span>`const TPIDR_EL3: Register`

- <span id="aarch64-vg"></span>`const VG: Register`

- <span id="aarch64-ffr"></span>`const FFR: Register`

- <span id="aarch64-p0"></span>`const P0: Register`

- <span id="aarch64-p1"></span>`const P1: Register`

- <span id="aarch64-p2"></span>`const P2: Register`

- <span id="aarch64-p3"></span>`const P3: Register`

- <span id="aarch64-p4"></span>`const P4: Register`

- <span id="aarch64-p5"></span>`const P5: Register`

- <span id="aarch64-p6"></span>`const P6: Register`

- <span id="aarch64-p7"></span>`const P7: Register`

- <span id="aarch64-p8"></span>`const P8: Register`

- <span id="aarch64-p9"></span>`const P9: Register`

- <span id="aarch64-p10"></span>`const P10: Register`

- <span id="aarch64-p11"></span>`const P11: Register`

- <span id="aarch64-p12"></span>`const P12: Register`

- <span id="aarch64-p13"></span>`const P13: Register`

- <span id="aarch64-p14"></span>`const P14: Register`

- <span id="aarch64-p15"></span>`const P15: Register`

- <span id="aarch64-v0"></span>`const V0: Register`

- <span id="aarch64-v1"></span>`const V1: Register`

- <span id="aarch64-v2"></span>`const V2: Register`

- <span id="aarch64-v3"></span>`const V3: Register`

- <span id="aarch64-v4"></span>`const V4: Register`

- <span id="aarch64-v5"></span>`const V5: Register`

- <span id="aarch64-v6"></span>`const V6: Register`

- <span id="aarch64-v7"></span>`const V7: Register`

- <span id="aarch64-v8"></span>`const V8: Register`

- <span id="aarch64-v9"></span>`const V9: Register`

- <span id="aarch64-v10"></span>`const V10: Register`

- <span id="aarch64-v11"></span>`const V11: Register`

- <span id="aarch64-v12"></span>`const V12: Register`

- <span id="aarch64-v13"></span>`const V13: Register`

- <span id="aarch64-v14"></span>`const V14: Register`

- <span id="aarch64-v15"></span>`const V15: Register`

- <span id="aarch64-v16"></span>`const V16: Register`

- <span id="aarch64-v17"></span>`const V17: Register`

- <span id="aarch64-v18"></span>`const V18: Register`

- <span id="aarch64-v19"></span>`const V19: Register`

- <span id="aarch64-v20"></span>`const V20: Register`

- <span id="aarch64-v21"></span>`const V21: Register`

- <span id="aarch64-v22"></span>`const V22: Register`

- <span id="aarch64-v23"></span>`const V23: Register`

- <span id="aarch64-v24"></span>`const V24: Register`

- <span id="aarch64-v25"></span>`const V25: Register`

- <span id="aarch64-v26"></span>`const V26: Register`

- <span id="aarch64-v27"></span>`const V27: Register`

- <span id="aarch64-v28"></span>`const V28: Register`

- <span id="aarch64-v29"></span>`const V29: Register`

- <span id="aarch64-v30"></span>`const V30: Register`

- <span id="aarch64-v31"></span>`const V31: Register`

- <span id="aarch64-z0"></span>`const Z0: Register`

- <span id="aarch64-z1"></span>`const Z1: Register`

- <span id="aarch64-z2"></span>`const Z2: Register`

- <span id="aarch64-z3"></span>`const Z3: Register`

- <span id="aarch64-z4"></span>`const Z4: Register`

- <span id="aarch64-z5"></span>`const Z5: Register`

- <span id="aarch64-z6"></span>`const Z6: Register`

- <span id="aarch64-z7"></span>`const Z7: Register`

- <span id="aarch64-z8"></span>`const Z8: Register`

- <span id="aarch64-z9"></span>`const Z9: Register`

- <span id="aarch64-z10"></span>`const Z10: Register`

- <span id="aarch64-z11"></span>`const Z11: Register`

- <span id="aarch64-z12"></span>`const Z12: Register`

- <span id="aarch64-z13"></span>`const Z13: Register`

- <span id="aarch64-z14"></span>`const Z14: Register`

- <span id="aarch64-z15"></span>`const Z15: Register`

- <span id="aarch64-z16"></span>`const Z16: Register`

- <span id="aarch64-z17"></span>`const Z17: Register`

- <span id="aarch64-z18"></span>`const Z18: Register`

- <span id="aarch64-z19"></span>`const Z19: Register`

- <span id="aarch64-z20"></span>`const Z20: Register`

- <span id="aarch64-z21"></span>`const Z21: Register`

- <span id="aarch64-z22"></span>`const Z22: Register`

- <span id="aarch64-z23"></span>`const Z23: Register`

- <span id="aarch64-z24"></span>`const Z24: Register`

- <span id="aarch64-z25"></span>`const Z25: Register`

- <span id="aarch64-z26"></span>`const Z26: Register`

- <span id="aarch64-z27"></span>`const Z27: Register`

- <span id="aarch64-z28"></span>`const Z28: Register`

- <span id="aarch64-z29"></span>`const Z29: Register`

- <span id="aarch64-z30"></span>`const Z30: Register`

- <span id="aarch64-z31"></span>`const Z31: Register`

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

- <span id="x86-eax"></span>`const EAX: Register`

- <span id="x86-ecx"></span>`const ECX: Register`

- <span id="x86-edx"></span>`const EDX: Register`

- <span id="x86-ebx"></span>`const EBX: Register`

- <span id="x86-esp"></span>`const ESP: Register`

- <span id="x86-ebp"></span>`const EBP: Register`

- <span id="x86-esi"></span>`const ESI: Register`

- <span id="x86-edi"></span>`const EDI: Register`

- <span id="x86-ra"></span>`const RA: Register`

- <span id="x86-st0"></span>`const ST0: Register`

- <span id="x86-st1"></span>`const ST1: Register`

- <span id="x86-st2"></span>`const ST2: Register`

- <span id="x86-st3"></span>`const ST3: Register`

- <span id="x86-st4"></span>`const ST4: Register`

- <span id="x86-st5"></span>`const ST5: Register`

- <span id="x86-st6"></span>`const ST6: Register`

- <span id="x86-st7"></span>`const ST7: Register`

- <span id="x86-xmm0"></span>`const XMM0: Register`

- <span id="x86-xmm1"></span>`const XMM1: Register`

- <span id="x86-xmm2"></span>`const XMM2: Register`

- <span id="x86-xmm3"></span>`const XMM3: Register`

- <span id="x86-xmm4"></span>`const XMM4: Register`

- <span id="x86-xmm5"></span>`const XMM5: Register`

- <span id="x86-xmm6"></span>`const XMM6: Register`

- <span id="x86-xmm7"></span>`const XMM7: Register`

- <span id="x86-mm0"></span>`const MM0: Register`

- <span id="x86-mm1"></span>`const MM1: Register`

- <span id="x86-mm2"></span>`const MM2: Register`

- <span id="x86-mm3"></span>`const MM3: Register`

- <span id="x86-mm4"></span>`const MM4: Register`

- <span id="x86-mm5"></span>`const MM5: Register`

- <span id="x86-mm6"></span>`const MM6: Register`

- <span id="x86-mm7"></span>`const MM7: Register`

- <span id="x86-mxcsr"></span>`const MXCSR: Register`

- <span id="x86-es"></span>`const ES: Register`

- <span id="x86-cs"></span>`const CS: Register`

- <span id="x86-ss"></span>`const SS: Register`

- <span id="x86-ds"></span>`const DS: Register`

- <span id="x86-fs"></span>`const FS: Register`

- <span id="x86-gs"></span>`const GS: Register`

- <span id="x86-tr"></span>`const TR: Register`

- <span id="x86-ldtr"></span>`const LDTR: Register`

- <span id="x86-fs-base"></span>`const FS_BASE: Register`

- <span id="x86-gs-base"></span>`const GS_BASE: Register`

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

- <span id="powerpc64-r0"></span>`const R0: Register`

- <span id="powerpc64-r1"></span>`const R1: Register`

- <span id="powerpc64-r2"></span>`const R2: Register`

- <span id="powerpc64-r3"></span>`const R3: Register`

- <span id="powerpc64-r4"></span>`const R4: Register`

- <span id="powerpc64-r5"></span>`const R5: Register`

- <span id="powerpc64-r6"></span>`const R6: Register`

- <span id="powerpc64-r7"></span>`const R7: Register`

- <span id="powerpc64-r8"></span>`const R8: Register`

- <span id="powerpc64-r9"></span>`const R9: Register`

- <span id="powerpc64-r10"></span>`const R10: Register`

- <span id="powerpc64-r11"></span>`const R11: Register`

- <span id="powerpc64-r12"></span>`const R12: Register`

- <span id="powerpc64-r13"></span>`const R13: Register`

- <span id="powerpc64-r14"></span>`const R14: Register`

- <span id="powerpc64-r15"></span>`const R15: Register`

- <span id="powerpc64-r16"></span>`const R16: Register`

- <span id="powerpc64-r17"></span>`const R17: Register`

- <span id="powerpc64-r18"></span>`const R18: Register`

- <span id="powerpc64-r19"></span>`const R19: Register`

- <span id="powerpc64-r20"></span>`const R20: Register`

- <span id="powerpc64-r21"></span>`const R21: Register`

- <span id="powerpc64-r22"></span>`const R22: Register`

- <span id="powerpc64-r23"></span>`const R23: Register`

- <span id="powerpc64-r24"></span>`const R24: Register`

- <span id="powerpc64-r25"></span>`const R25: Register`

- <span id="powerpc64-r26"></span>`const R26: Register`

- <span id="powerpc64-r27"></span>`const R27: Register`

- <span id="powerpc64-r28"></span>`const R28: Register`

- <span id="powerpc64-r29"></span>`const R29: Register`

- <span id="powerpc64-r30"></span>`const R30: Register`

- <span id="powerpc64-r31"></span>`const R31: Register`

- <span id="powerpc64-f0"></span>`const F0: Register`

- <span id="powerpc64-f1"></span>`const F1: Register`

- <span id="powerpc64-f2"></span>`const F2: Register`

- <span id="powerpc64-f3"></span>`const F3: Register`

- <span id="powerpc64-f4"></span>`const F4: Register`

- <span id="powerpc64-f5"></span>`const F5: Register`

- <span id="powerpc64-f6"></span>`const F6: Register`

- <span id="powerpc64-f7"></span>`const F7: Register`

- <span id="powerpc64-f8"></span>`const F8: Register`

- <span id="powerpc64-f9"></span>`const F9: Register`

- <span id="powerpc64-f10"></span>`const F10: Register`

- <span id="powerpc64-f11"></span>`const F11: Register`

- <span id="powerpc64-f12"></span>`const F12: Register`

- <span id="powerpc64-f13"></span>`const F13: Register`

- <span id="powerpc64-f14"></span>`const F14: Register`

- <span id="powerpc64-f15"></span>`const F15: Register`

- <span id="powerpc64-f16"></span>`const F16: Register`

- <span id="powerpc64-f17"></span>`const F17: Register`

- <span id="powerpc64-f18"></span>`const F18: Register`

- <span id="powerpc64-f19"></span>`const F19: Register`

- <span id="powerpc64-f20"></span>`const F20: Register`

- <span id="powerpc64-f21"></span>`const F21: Register`

- <span id="powerpc64-f22"></span>`const F22: Register`

- <span id="powerpc64-f23"></span>`const F23: Register`

- <span id="powerpc64-f24"></span>`const F24: Register`

- <span id="powerpc64-f25"></span>`const F25: Register`

- <span id="powerpc64-f26"></span>`const F26: Register`

- <span id="powerpc64-f27"></span>`const F27: Register`

- <span id="powerpc64-f28"></span>`const F28: Register`

- <span id="powerpc64-f29"></span>`const F29: Register`

- <span id="powerpc64-f30"></span>`const F30: Register`

- <span id="powerpc64-f31"></span>`const F31: Register`

- <span id="powerpc64-lr"></span>`const LR: Register`

- <span id="powerpc64-ctr"></span>`const CTR: Register`

- <span id="powerpc64-cr0"></span>`const CR0: Register`

- <span id="powerpc64-cr1"></span>`const CR1: Register`

- <span id="powerpc64-cr2"></span>`const CR2: Register`

- <span id="powerpc64-cr3"></span>`const CR3: Register`

- <span id="powerpc64-cr4"></span>`const CR4: Register`

- <span id="powerpc64-cr5"></span>`const CR5: Register`

- <span id="powerpc64-cr6"></span>`const CR6: Register`

- <span id="powerpc64-cr7"></span>`const CR7: Register`

- <span id="powerpc64-xer"></span>`const XER: Register`

- <span id="powerpc64-vr0"></span>`const VR0: Register`

- <span id="powerpc64-vr1"></span>`const VR1: Register`

- <span id="powerpc64-vr2"></span>`const VR2: Register`

- <span id="powerpc64-vr3"></span>`const VR3: Register`

- <span id="powerpc64-vr4"></span>`const VR4: Register`

- <span id="powerpc64-vr5"></span>`const VR5: Register`

- <span id="powerpc64-vr6"></span>`const VR6: Register`

- <span id="powerpc64-vr7"></span>`const VR7: Register`

- <span id="powerpc64-vr8"></span>`const VR8: Register`

- <span id="powerpc64-vr9"></span>`const VR9: Register`

- <span id="powerpc64-vr10"></span>`const VR10: Register`

- <span id="powerpc64-vr11"></span>`const VR11: Register`

- <span id="powerpc64-vr12"></span>`const VR12: Register`

- <span id="powerpc64-vr13"></span>`const VR13: Register`

- <span id="powerpc64-vr14"></span>`const VR14: Register`

- <span id="powerpc64-vr15"></span>`const VR15: Register`

- <span id="powerpc64-vr16"></span>`const VR16: Register`

- <span id="powerpc64-vr17"></span>`const VR17: Register`

- <span id="powerpc64-vr18"></span>`const VR18: Register`

- <span id="powerpc64-vr19"></span>`const VR19: Register`

- <span id="powerpc64-vr20"></span>`const VR20: Register`

- <span id="powerpc64-vr21"></span>`const VR21: Register`

- <span id="powerpc64-vr22"></span>`const VR22: Register`

- <span id="powerpc64-vr23"></span>`const VR23: Register`

- <span id="powerpc64-vr24"></span>`const VR24: Register`

- <span id="powerpc64-vr25"></span>`const VR25: Register`

- <span id="powerpc64-vr26"></span>`const VR26: Register`

- <span id="powerpc64-vr27"></span>`const VR27: Register`

- <span id="powerpc64-vr28"></span>`const VR28: Register`

- <span id="powerpc64-vr29"></span>`const VR29: Register`

- <span id="powerpc64-vr30"></span>`const VR30: Register`

- <span id="powerpc64-vr31"></span>`const VR31: Register`

- <span id="powerpc64-vscr"></span>`const VSCR: Register`

- <span id="powerpc64-tfhar"></span>`const TFHAR: Register`

- <span id="powerpc64-tfiar"></span>`const TFIAR: Register`

- <span id="powerpc64-texasr"></span>`const TEXASR: Register`

#### Trait Implementations

##### `impl Clone for PowerPc64`

- <span id="powerpc64-clone"></span>`fn clone(&self) -> PowerPc64` — [`PowerPc64`](../index.md)

##### `impl Copy for PowerPc64`

##### `impl Debug for PowerPc64`

- <span id="powerpc64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `registers!`

