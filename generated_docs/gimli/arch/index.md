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

*Defined in [`gimli-0.32.3/src/arch.rs:50`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L50)*

ARM architecture specific definitions.

See [DWARF for the ARM Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf32/aadwarf32.rst).

#### Implementations

- <span id="arm-const-r0"></span>`const R0: Register`

- <span id="arm-const-r1"></span>`const R1: Register`

- <span id="arm-const-r2"></span>`const R2: Register`

- <span id="arm-const-r3"></span>`const R3: Register`

- <span id="arm-const-r4"></span>`const R4: Register`

- <span id="arm-const-r5"></span>`const R5: Register`

- <span id="arm-const-r6"></span>`const R6: Register`

- <span id="arm-const-r7"></span>`const R7: Register`

- <span id="arm-const-r8"></span>`const R8: Register`

- <span id="arm-const-r9"></span>`const R9: Register`

- <span id="arm-const-r10"></span>`const R10: Register`

- <span id="arm-const-r11"></span>`const R11: Register`

- <span id="arm-const-r12"></span>`const R12: Register`

- <span id="arm-const-r13"></span>`const R13: Register`

- <span id="arm-const-r14"></span>`const R14: Register`

- <span id="arm-const-r15"></span>`const R15: Register`

- <span id="arm-const-wcgr0"></span>`const WCGR0: Register`

- <span id="arm-const-wcgr1"></span>`const WCGR1: Register`

- <span id="arm-const-wcgr2"></span>`const WCGR2: Register`

- <span id="arm-const-wcgr3"></span>`const WCGR3: Register`

- <span id="arm-const-wcgr4"></span>`const WCGR4: Register`

- <span id="arm-const-wcgr5"></span>`const WCGR5: Register`

- <span id="arm-const-wcgr6"></span>`const WCGR6: Register`

- <span id="arm-const-wcgr7"></span>`const WCGR7: Register`

- <span id="arm-const-wr0"></span>`const WR0: Register`

- <span id="arm-const-wr1"></span>`const WR1: Register`

- <span id="arm-const-wr2"></span>`const WR2: Register`

- <span id="arm-const-wr3"></span>`const WR3: Register`

- <span id="arm-const-wr4"></span>`const WR4: Register`

- <span id="arm-const-wr5"></span>`const WR5: Register`

- <span id="arm-const-wr6"></span>`const WR6: Register`

- <span id="arm-const-wr7"></span>`const WR7: Register`

- <span id="arm-const-wr8"></span>`const WR8: Register`

- <span id="arm-const-wr9"></span>`const WR9: Register`

- <span id="arm-const-wr10"></span>`const WR10: Register`

- <span id="arm-const-wr11"></span>`const WR11: Register`

- <span id="arm-const-wr12"></span>`const WR12: Register`

- <span id="arm-const-wr13"></span>`const WR13: Register`

- <span id="arm-const-wr14"></span>`const WR14: Register`

- <span id="arm-const-wr15"></span>`const WR15: Register`

- <span id="arm-const-spsr"></span>`const SPSR: Register`

- <span id="arm-const-spsr-fiq"></span>`const SPSR_FIQ: Register`

- <span id="arm-const-spsr-irq"></span>`const SPSR_IRQ: Register`

- <span id="arm-const-spsr-abt"></span>`const SPSR_ABT: Register`

- <span id="arm-const-spsr-und"></span>`const SPSR_UND: Register`

- <span id="arm-const-spsr-svc"></span>`const SPSR_SVC: Register`

- <span id="arm-const-ra-auth-code"></span>`const RA_AUTH_CODE: Register`

- <span id="arm-const-r8-usr"></span>`const R8_USR: Register`

- <span id="arm-const-r9-usr"></span>`const R9_USR: Register`

- <span id="arm-const-r10-usr"></span>`const R10_USR: Register`

- <span id="arm-const-r11-usr"></span>`const R11_USR: Register`

- <span id="arm-const-r12-usr"></span>`const R12_USR: Register`

- <span id="arm-const-r13-usr"></span>`const R13_USR: Register`

- <span id="arm-const-r14-usr"></span>`const R14_USR: Register`

- <span id="arm-const-r8-fiq"></span>`const R8_FIQ: Register`

- <span id="arm-const-r9-fiq"></span>`const R9_FIQ: Register`

- <span id="arm-const-r10-fiq"></span>`const R10_FIQ: Register`

- <span id="arm-const-r11-fiq"></span>`const R11_FIQ: Register`

- <span id="arm-const-r12-fiq"></span>`const R12_FIQ: Register`

- <span id="arm-const-r13-fiq"></span>`const R13_FIQ: Register`

- <span id="arm-const-r14-fiq"></span>`const R14_FIQ: Register`

- <span id="arm-const-r13-irq"></span>`const R13_IRQ: Register`

- <span id="arm-const-r14-irq"></span>`const R14_IRQ: Register`

- <span id="arm-const-r13-abt"></span>`const R13_ABT: Register`

- <span id="arm-const-r14-abt"></span>`const R14_ABT: Register`

- <span id="arm-const-r13-und"></span>`const R13_UND: Register`

- <span id="arm-const-r14-und"></span>`const R14_UND: Register`

- <span id="arm-const-r13-svc"></span>`const R13_SVC: Register`

- <span id="arm-const-r14-svc"></span>`const R14_SVC: Register`

- <span id="arm-const-wc0"></span>`const WC0: Register`

- <span id="arm-const-wc1"></span>`const WC1: Register`

- <span id="arm-const-wc2"></span>`const WC2: Register`

- <span id="arm-const-wc3"></span>`const WC3: Register`

- <span id="arm-const-wc4"></span>`const WC4: Register`

- <span id="arm-const-wc5"></span>`const WC5: Register`

- <span id="arm-const-wc6"></span>`const WC6: Register`

- <span id="arm-const-wc7"></span>`const WC7: Register`

- <span id="arm-const-d0"></span>`const D0: Register`

- <span id="arm-const-d1"></span>`const D1: Register`

- <span id="arm-const-d2"></span>`const D2: Register`

- <span id="arm-const-d3"></span>`const D3: Register`

- <span id="arm-const-d4"></span>`const D4: Register`

- <span id="arm-const-d5"></span>`const D5: Register`

- <span id="arm-const-d6"></span>`const D6: Register`

- <span id="arm-const-d7"></span>`const D7: Register`

- <span id="arm-const-d8"></span>`const D8: Register`

- <span id="arm-const-d9"></span>`const D9: Register`

- <span id="arm-const-d10"></span>`const D10: Register`

- <span id="arm-const-d11"></span>`const D11: Register`

- <span id="arm-const-d12"></span>`const D12: Register`

- <span id="arm-const-d13"></span>`const D13: Register`

- <span id="arm-const-d14"></span>`const D14: Register`

- <span id="arm-const-d15"></span>`const D15: Register`

- <span id="arm-const-d16"></span>`const D16: Register`

- <span id="arm-const-d17"></span>`const D17: Register`

- <span id="arm-const-d18"></span>`const D18: Register`

- <span id="arm-const-d19"></span>`const D19: Register`

- <span id="arm-const-d20"></span>`const D20: Register`

- <span id="arm-const-d21"></span>`const D21: Register`

- <span id="arm-const-d22"></span>`const D22: Register`

- <span id="arm-const-d23"></span>`const D23: Register`

- <span id="arm-const-d24"></span>`const D24: Register`

- <span id="arm-const-d25"></span>`const D25: Register`

- <span id="arm-const-d26"></span>`const D26: Register`

- <span id="arm-const-d27"></span>`const D27: Register`

- <span id="arm-const-d28"></span>`const D28: Register`

- <span id="arm-const-d29"></span>`const D29: Register`

- <span id="arm-const-d30"></span>`const D30: Register`

- <span id="arm-const-d31"></span>`const D31: Register`

- <span id="arm-const-tpidruro"></span>`const TPIDRURO: Register`

- <span id="arm-const-tpidrurw"></span>`const TPIDRURW: Register`

- <span id="arm-const-tpidpr"></span>`const TPIDPR: Register`

- <span id="arm-const-htpidpr"></span>`const HTPIDPR: Register`

- <span id="arm-const-sp"></span>`const SP: Register`

- <span id="arm-const-lr"></span>`const LR: Register`

- <span id="arm-const-pc"></span>`const PC: Register`

- <span id="arm-const-acc0"></span>`const ACC0: Register`

- <span id="arm-const-acc1"></span>`const ACC1: Register`

- <span id="arm-const-acc2"></span>`const ACC2: Register`

- <span id="arm-const-acc3"></span>`const ACC3: Register`

- <span id="arm-const-acc4"></span>`const ACC4: Register`

- <span id="arm-const-acc5"></span>`const ACC5: Register`

- <span id="arm-const-acc6"></span>`const ACC6: Register`

- <span id="arm-const-acc7"></span>`const ACC7: Register`

- <span id="arm-const-s0"></span>`const S0: Register`

- <span id="arm-const-s1"></span>`const S1: Register`

- <span id="arm-const-s2"></span>`const S2: Register`

- <span id="arm-const-s3"></span>`const S3: Register`

- <span id="arm-const-s4"></span>`const S4: Register`

- <span id="arm-const-s5"></span>`const S5: Register`

- <span id="arm-const-s6"></span>`const S6: Register`

- <span id="arm-const-s7"></span>`const S7: Register`

- <span id="arm-const-s8"></span>`const S8: Register`

- <span id="arm-const-s9"></span>`const S9: Register`

- <span id="arm-const-s10"></span>`const S10: Register`

- <span id="arm-const-s11"></span>`const S11: Register`

- <span id="arm-const-s12"></span>`const S12: Register`

- <span id="arm-const-s13"></span>`const S13: Register`

- <span id="arm-const-s14"></span>`const S14: Register`

- <span id="arm-const-s15"></span>`const S15: Register`

- <span id="arm-const-s16"></span>`const S16: Register`

- <span id="arm-const-s17"></span>`const S17: Register`

- <span id="arm-const-s18"></span>`const S18: Register`

- <span id="arm-const-s19"></span>`const S19: Register`

- <span id="arm-const-s20"></span>`const S20: Register`

- <span id="arm-const-s21"></span>`const S21: Register`

- <span id="arm-const-s22"></span>`const S22: Register`

- <span id="arm-const-s23"></span>`const S23: Register`

- <span id="arm-const-s24"></span>`const S24: Register`

- <span id="arm-const-s25"></span>`const S25: Register`

- <span id="arm-const-s26"></span>`const S26: Register`

- <span id="arm-const-s27"></span>`const S27: Register`

- <span id="arm-const-s28"></span>`const S28: Register`

- <span id="arm-const-s29"></span>`const S29: Register`

- <span id="arm-const-s30"></span>`const S30: Register`

- <span id="arm-const-s31"></span>`const S31: Register`

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

*Defined in [`gimli-0.32.3/src/arch.rs:233`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L233)*

ARM 64-bit (AArch64) architecture specific definitions.

See [DWARF for the ARM 64-bit Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst).

#### Implementations

- <span id="aarch64-const-x0"></span>`const X0: Register`

- <span id="aarch64-const-x1"></span>`const X1: Register`

- <span id="aarch64-const-x2"></span>`const X2: Register`

- <span id="aarch64-const-x3"></span>`const X3: Register`

- <span id="aarch64-const-x4"></span>`const X4: Register`

- <span id="aarch64-const-x5"></span>`const X5: Register`

- <span id="aarch64-const-x6"></span>`const X6: Register`

- <span id="aarch64-const-x7"></span>`const X7: Register`

- <span id="aarch64-const-x8"></span>`const X8: Register`

- <span id="aarch64-const-x9"></span>`const X9: Register`

- <span id="aarch64-const-x10"></span>`const X10: Register`

- <span id="aarch64-const-x11"></span>`const X11: Register`

- <span id="aarch64-const-x12"></span>`const X12: Register`

- <span id="aarch64-const-x13"></span>`const X13: Register`

- <span id="aarch64-const-x14"></span>`const X14: Register`

- <span id="aarch64-const-x15"></span>`const X15: Register`

- <span id="aarch64-const-x16"></span>`const X16: Register`

- <span id="aarch64-const-x17"></span>`const X17: Register`

- <span id="aarch64-const-x18"></span>`const X18: Register`

- <span id="aarch64-const-x19"></span>`const X19: Register`

- <span id="aarch64-const-x20"></span>`const X20: Register`

- <span id="aarch64-const-x21"></span>`const X21: Register`

- <span id="aarch64-const-x22"></span>`const X22: Register`

- <span id="aarch64-const-x23"></span>`const X23: Register`

- <span id="aarch64-const-x24"></span>`const X24: Register`

- <span id="aarch64-const-x25"></span>`const X25: Register`

- <span id="aarch64-const-x26"></span>`const X26: Register`

- <span id="aarch64-const-x27"></span>`const X27: Register`

- <span id="aarch64-const-x28"></span>`const X28: Register`

- <span id="aarch64-const-x29"></span>`const X29: Register`

- <span id="aarch64-const-x30"></span>`const X30: Register`

- <span id="aarch64-const-sp"></span>`const SP: Register`

- <span id="aarch64-const-pc"></span>`const PC: Register`

- <span id="aarch64-const-elr-mode"></span>`const ELR_MODE: Register`

- <span id="aarch64-const-ra-sign-state"></span>`const RA_SIGN_STATE: Register`

- <span id="aarch64-const-tpidrro-el0"></span>`const TPIDRRO_EL0: Register`

- <span id="aarch64-const-tpidr-el0"></span>`const TPIDR_EL0: Register`

- <span id="aarch64-const-tpidr-el1"></span>`const TPIDR_EL1: Register`

- <span id="aarch64-const-tpidr-el2"></span>`const TPIDR_EL2: Register`

- <span id="aarch64-const-tpidr-el3"></span>`const TPIDR_EL3: Register`

- <span id="aarch64-const-vg"></span>`const VG: Register`

- <span id="aarch64-const-ffr"></span>`const FFR: Register`

- <span id="aarch64-const-p0"></span>`const P0: Register`

- <span id="aarch64-const-p1"></span>`const P1: Register`

- <span id="aarch64-const-p2"></span>`const P2: Register`

- <span id="aarch64-const-p3"></span>`const P3: Register`

- <span id="aarch64-const-p4"></span>`const P4: Register`

- <span id="aarch64-const-p5"></span>`const P5: Register`

- <span id="aarch64-const-p6"></span>`const P6: Register`

- <span id="aarch64-const-p7"></span>`const P7: Register`

- <span id="aarch64-const-p8"></span>`const P8: Register`

- <span id="aarch64-const-p9"></span>`const P9: Register`

- <span id="aarch64-const-p10"></span>`const P10: Register`

- <span id="aarch64-const-p11"></span>`const P11: Register`

- <span id="aarch64-const-p12"></span>`const P12: Register`

- <span id="aarch64-const-p13"></span>`const P13: Register`

- <span id="aarch64-const-p14"></span>`const P14: Register`

- <span id="aarch64-const-p15"></span>`const P15: Register`

- <span id="aarch64-const-v0"></span>`const V0: Register`

- <span id="aarch64-const-v1"></span>`const V1: Register`

- <span id="aarch64-const-v2"></span>`const V2: Register`

- <span id="aarch64-const-v3"></span>`const V3: Register`

- <span id="aarch64-const-v4"></span>`const V4: Register`

- <span id="aarch64-const-v5"></span>`const V5: Register`

- <span id="aarch64-const-v6"></span>`const V6: Register`

- <span id="aarch64-const-v7"></span>`const V7: Register`

- <span id="aarch64-const-v8"></span>`const V8: Register`

- <span id="aarch64-const-v9"></span>`const V9: Register`

- <span id="aarch64-const-v10"></span>`const V10: Register`

- <span id="aarch64-const-v11"></span>`const V11: Register`

- <span id="aarch64-const-v12"></span>`const V12: Register`

- <span id="aarch64-const-v13"></span>`const V13: Register`

- <span id="aarch64-const-v14"></span>`const V14: Register`

- <span id="aarch64-const-v15"></span>`const V15: Register`

- <span id="aarch64-const-v16"></span>`const V16: Register`

- <span id="aarch64-const-v17"></span>`const V17: Register`

- <span id="aarch64-const-v18"></span>`const V18: Register`

- <span id="aarch64-const-v19"></span>`const V19: Register`

- <span id="aarch64-const-v20"></span>`const V20: Register`

- <span id="aarch64-const-v21"></span>`const V21: Register`

- <span id="aarch64-const-v22"></span>`const V22: Register`

- <span id="aarch64-const-v23"></span>`const V23: Register`

- <span id="aarch64-const-v24"></span>`const V24: Register`

- <span id="aarch64-const-v25"></span>`const V25: Register`

- <span id="aarch64-const-v26"></span>`const V26: Register`

- <span id="aarch64-const-v27"></span>`const V27: Register`

- <span id="aarch64-const-v28"></span>`const V28: Register`

- <span id="aarch64-const-v29"></span>`const V29: Register`

- <span id="aarch64-const-v30"></span>`const V30: Register`

- <span id="aarch64-const-v31"></span>`const V31: Register`

- <span id="aarch64-const-z0"></span>`const Z0: Register`

- <span id="aarch64-const-z1"></span>`const Z1: Register`

- <span id="aarch64-const-z2"></span>`const Z2: Register`

- <span id="aarch64-const-z3"></span>`const Z3: Register`

- <span id="aarch64-const-z4"></span>`const Z4: Register`

- <span id="aarch64-const-z5"></span>`const Z5: Register`

- <span id="aarch64-const-z6"></span>`const Z6: Register`

- <span id="aarch64-const-z7"></span>`const Z7: Register`

- <span id="aarch64-const-z8"></span>`const Z8: Register`

- <span id="aarch64-const-z9"></span>`const Z9: Register`

- <span id="aarch64-const-z10"></span>`const Z10: Register`

- <span id="aarch64-const-z11"></span>`const Z11: Register`

- <span id="aarch64-const-z12"></span>`const Z12: Register`

- <span id="aarch64-const-z13"></span>`const Z13: Register`

- <span id="aarch64-const-z14"></span>`const Z14: Register`

- <span id="aarch64-const-z15"></span>`const Z15: Register`

- <span id="aarch64-const-z16"></span>`const Z16: Register`

- <span id="aarch64-const-z17"></span>`const Z17: Register`

- <span id="aarch64-const-z18"></span>`const Z18: Register`

- <span id="aarch64-const-z19"></span>`const Z19: Register`

- <span id="aarch64-const-z20"></span>`const Z20: Register`

- <span id="aarch64-const-z21"></span>`const Z21: Register`

- <span id="aarch64-const-z22"></span>`const Z22: Register`

- <span id="aarch64-const-z23"></span>`const Z23: Register`

- <span id="aarch64-const-z24"></span>`const Z24: Register`

- <span id="aarch64-const-z25"></span>`const Z25: Register`

- <span id="aarch64-const-z26"></span>`const Z26: Register`

- <span id="aarch64-const-z27"></span>`const Z27: Register`

- <span id="aarch64-const-z28"></span>`const Z28: Register`

- <span id="aarch64-const-z29"></span>`const Z29: Register`

- <span id="aarch64-const-z30"></span>`const Z30: Register`

- <span id="aarch64-const-z31"></span>`const Z31: Register`

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

*Defined in [`gimli-0.32.3/src/arch.rs:368`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L368)*

LoongArch architecture specific definitions.

See [LoongArch ELF psABI specification](https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html).

#### Implementations

- <span id="loongarch-const-r0"></span>`const R0: Register`

- <span id="loongarch-const-r1"></span>`const R1: Register`

- <span id="loongarch-const-r2"></span>`const R2: Register`

- <span id="loongarch-const-r3"></span>`const R3: Register`

- <span id="loongarch-const-r4"></span>`const R4: Register`

- <span id="loongarch-const-r5"></span>`const R5: Register`

- <span id="loongarch-const-r6"></span>`const R6: Register`

- <span id="loongarch-const-r7"></span>`const R7: Register`

- <span id="loongarch-const-r8"></span>`const R8: Register`

- <span id="loongarch-const-r9"></span>`const R9: Register`

- <span id="loongarch-const-r10"></span>`const R10: Register`

- <span id="loongarch-const-r11"></span>`const R11: Register`

- <span id="loongarch-const-r12"></span>`const R12: Register`

- <span id="loongarch-const-r13"></span>`const R13: Register`

- <span id="loongarch-const-r14"></span>`const R14: Register`

- <span id="loongarch-const-r15"></span>`const R15: Register`

- <span id="loongarch-const-r16"></span>`const R16: Register`

- <span id="loongarch-const-r17"></span>`const R17: Register`

- <span id="loongarch-const-r18"></span>`const R18: Register`

- <span id="loongarch-const-r19"></span>`const R19: Register`

- <span id="loongarch-const-r20"></span>`const R20: Register`

- <span id="loongarch-const-r21"></span>`const R21: Register`

- <span id="loongarch-const-r22"></span>`const R22: Register`

- <span id="loongarch-const-r23"></span>`const R23: Register`

- <span id="loongarch-const-r24"></span>`const R24: Register`

- <span id="loongarch-const-r25"></span>`const R25: Register`

- <span id="loongarch-const-r26"></span>`const R26: Register`

- <span id="loongarch-const-r27"></span>`const R27: Register`

- <span id="loongarch-const-r28"></span>`const R28: Register`

- <span id="loongarch-const-r29"></span>`const R29: Register`

- <span id="loongarch-const-r30"></span>`const R30: Register`

- <span id="loongarch-const-r31"></span>`const R31: Register`

- <span id="loongarch-const-f0"></span>`const F0: Register`

- <span id="loongarch-const-f1"></span>`const F1: Register`

- <span id="loongarch-const-f2"></span>`const F2: Register`

- <span id="loongarch-const-f3"></span>`const F3: Register`

- <span id="loongarch-const-f4"></span>`const F4: Register`

- <span id="loongarch-const-f5"></span>`const F5: Register`

- <span id="loongarch-const-f6"></span>`const F6: Register`

- <span id="loongarch-const-f7"></span>`const F7: Register`

- <span id="loongarch-const-f8"></span>`const F8: Register`

- <span id="loongarch-const-f9"></span>`const F9: Register`

- <span id="loongarch-const-f10"></span>`const F10: Register`

- <span id="loongarch-const-f11"></span>`const F11: Register`

- <span id="loongarch-const-f12"></span>`const F12: Register`

- <span id="loongarch-const-f13"></span>`const F13: Register`

- <span id="loongarch-const-f14"></span>`const F14: Register`

- <span id="loongarch-const-f15"></span>`const F15: Register`

- <span id="loongarch-const-f16"></span>`const F16: Register`

- <span id="loongarch-const-f17"></span>`const F17: Register`

- <span id="loongarch-const-f18"></span>`const F18: Register`

- <span id="loongarch-const-f19"></span>`const F19: Register`

- <span id="loongarch-const-f20"></span>`const F20: Register`

- <span id="loongarch-const-f21"></span>`const F21: Register`

- <span id="loongarch-const-f22"></span>`const F22: Register`

- <span id="loongarch-const-f23"></span>`const F23: Register`

- <span id="loongarch-const-f24"></span>`const F24: Register`

- <span id="loongarch-const-f25"></span>`const F25: Register`

- <span id="loongarch-const-f26"></span>`const F26: Register`

- <span id="loongarch-const-f27"></span>`const F27: Register`

- <span id="loongarch-const-f28"></span>`const F28: Register`

- <span id="loongarch-const-f29"></span>`const F29: Register`

- <span id="loongarch-const-f30"></span>`const F30: Register`

- <span id="loongarch-const-f31"></span>`const F31: Register`

- <span id="loongarch-const-fcc0"></span>`const FCC0: Register`

- <span id="loongarch-const-fcc1"></span>`const FCC1: Register`

- <span id="loongarch-const-fcc2"></span>`const FCC2: Register`

- <span id="loongarch-const-fcc3"></span>`const FCC3: Register`

- <span id="loongarch-const-fcc4"></span>`const FCC4: Register`

- <span id="loongarch-const-fcc5"></span>`const FCC5: Register`

- <span id="loongarch-const-fcc6"></span>`const FCC6: Register`

- <span id="loongarch-const-fcc7"></span>`const FCC7: Register`

- <span id="loongarch-const-zero"></span>`const ZERO: Register`

- <span id="loongarch-const-ra"></span>`const RA: Register`

- <span id="loongarch-const-tp"></span>`const TP: Register`

- <span id="loongarch-const-sp"></span>`const SP: Register`

- <span id="loongarch-const-a0"></span>`const A0: Register`

- <span id="loongarch-const-a1"></span>`const A1: Register`

- <span id="loongarch-const-a2"></span>`const A2: Register`

- <span id="loongarch-const-a3"></span>`const A3: Register`

- <span id="loongarch-const-a4"></span>`const A4: Register`

- <span id="loongarch-const-a5"></span>`const A5: Register`

- <span id="loongarch-const-a6"></span>`const A6: Register`

- <span id="loongarch-const-a7"></span>`const A7: Register`

- <span id="loongarch-const-t0"></span>`const T0: Register`

- <span id="loongarch-const-t1"></span>`const T1: Register`

- <span id="loongarch-const-t2"></span>`const T2: Register`

- <span id="loongarch-const-t3"></span>`const T3: Register`

- <span id="loongarch-const-t4"></span>`const T4: Register`

- <span id="loongarch-const-t5"></span>`const T5: Register`

- <span id="loongarch-const-t6"></span>`const T6: Register`

- <span id="loongarch-const-t7"></span>`const T7: Register`

- <span id="loongarch-const-t8"></span>`const T8: Register`

- <span id="loongarch-const-fp"></span>`const FP: Register`

- <span id="loongarch-const-s0"></span>`const S0: Register`

- <span id="loongarch-const-s1"></span>`const S1: Register`

- <span id="loongarch-const-s2"></span>`const S2: Register`

- <span id="loongarch-const-s3"></span>`const S3: Register`

- <span id="loongarch-const-s4"></span>`const S4: Register`

- <span id="loongarch-const-s5"></span>`const S5: Register`

- <span id="loongarch-const-s6"></span>`const S6: Register`

- <span id="loongarch-const-s7"></span>`const S7: Register`

- <span id="loongarch-const-s8"></span>`const S8: Register`

- <span id="loongarch-const-fa0"></span>`const FA0: Register`

- <span id="loongarch-const-fa1"></span>`const FA1: Register`

- <span id="loongarch-const-fa2"></span>`const FA2: Register`

- <span id="loongarch-const-fa3"></span>`const FA3: Register`

- <span id="loongarch-const-fa4"></span>`const FA4: Register`

- <span id="loongarch-const-fa5"></span>`const FA5: Register`

- <span id="loongarch-const-fa6"></span>`const FA6: Register`

- <span id="loongarch-const-fa7"></span>`const FA7: Register`

- <span id="loongarch-const-ft0"></span>`const FT0: Register`

- <span id="loongarch-const-ft1"></span>`const FT1: Register`

- <span id="loongarch-const-ft2"></span>`const FT2: Register`

- <span id="loongarch-const-ft3"></span>`const FT3: Register`

- <span id="loongarch-const-ft4"></span>`const FT4: Register`

- <span id="loongarch-const-ft5"></span>`const FT5: Register`

- <span id="loongarch-const-ft6"></span>`const FT6: Register`

- <span id="loongarch-const-ft7"></span>`const FT7: Register`

- <span id="loongarch-const-ft8"></span>`const FT8: Register`

- <span id="loongarch-const-ft9"></span>`const FT9: Register`

- <span id="loongarch-const-ft10"></span>`const FT10: Register`

- <span id="loongarch-const-ft11"></span>`const FT11: Register`

- <span id="loongarch-const-ft12"></span>`const FT12: Register`

- <span id="loongarch-const-ft13"></span>`const FT13: Register`

- <span id="loongarch-const-ft14"></span>`const FT14: Register`

- <span id="loongarch-const-ft15"></span>`const FT15: Register`

- <span id="loongarch-const-fs0"></span>`const FS0: Register`

- <span id="loongarch-const-fs1"></span>`const FS1: Register`

- <span id="loongarch-const-fs2"></span>`const FS2: Register`

- <span id="loongarch-const-fs3"></span>`const FS3: Register`

- <span id="loongarch-const-fs4"></span>`const FS4: Register`

- <span id="loongarch-const-fs5"></span>`const FS5: Register`

- <span id="loongarch-const-fs6"></span>`const FS6: Register`

- <span id="loongarch-const-fs7"></span>`const FS7: Register`

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

*Defined in [`gimli-0.32.3/src/arch.rs:516`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L516)*

MIPS architecture specific definitions.

See [MIPS Details](https://en.wikibooks.org/wiki/MIPS_Assembly/MIPS_Details).

#### Implementations

- <span id="mips-const-r0"></span>`const R0: Register`

- <span id="mips-const-r1"></span>`const R1: Register`

- <span id="mips-const-r2"></span>`const R2: Register`

- <span id="mips-const-r3"></span>`const R3: Register`

- <span id="mips-const-r4"></span>`const R4: Register`

- <span id="mips-const-r5"></span>`const R5: Register`

- <span id="mips-const-r6"></span>`const R6: Register`

- <span id="mips-const-r7"></span>`const R7: Register`

- <span id="mips-const-r8"></span>`const R8: Register`

- <span id="mips-const-r9"></span>`const R9: Register`

- <span id="mips-const-r10"></span>`const R10: Register`

- <span id="mips-const-r11"></span>`const R11: Register`

- <span id="mips-const-r12"></span>`const R12: Register`

- <span id="mips-const-r13"></span>`const R13: Register`

- <span id="mips-const-r14"></span>`const R14: Register`

- <span id="mips-const-r15"></span>`const R15: Register`

- <span id="mips-const-r16"></span>`const R16: Register`

- <span id="mips-const-r17"></span>`const R17: Register`

- <span id="mips-const-r18"></span>`const R18: Register`

- <span id="mips-const-r19"></span>`const R19: Register`

- <span id="mips-const-r20"></span>`const R20: Register`

- <span id="mips-const-r21"></span>`const R21: Register`

- <span id="mips-const-r22"></span>`const R22: Register`

- <span id="mips-const-r23"></span>`const R23: Register`

- <span id="mips-const-r24"></span>`const R24: Register`

- <span id="mips-const-r25"></span>`const R25: Register`

- <span id="mips-const-r26"></span>`const R26: Register`

- <span id="mips-const-r27"></span>`const R27: Register`

- <span id="mips-const-r28"></span>`const R28: Register`

- <span id="mips-const-r29"></span>`const R29: Register`

- <span id="mips-const-r30"></span>`const R30: Register`

- <span id="mips-const-r31"></span>`const R31: Register`

- <span id="mips-const-f0"></span>`const F0: Register`

- <span id="mips-const-f1"></span>`const F1: Register`

- <span id="mips-const-f2"></span>`const F2: Register`

- <span id="mips-const-f3"></span>`const F3: Register`

- <span id="mips-const-f4"></span>`const F4: Register`

- <span id="mips-const-f5"></span>`const F5: Register`

- <span id="mips-const-f6"></span>`const F6: Register`

- <span id="mips-const-f7"></span>`const F7: Register`

- <span id="mips-const-f8"></span>`const F8: Register`

- <span id="mips-const-f9"></span>`const F9: Register`

- <span id="mips-const-f10"></span>`const F10: Register`

- <span id="mips-const-f11"></span>`const F11: Register`

- <span id="mips-const-f12"></span>`const F12: Register`

- <span id="mips-const-f13"></span>`const F13: Register`

- <span id="mips-const-f14"></span>`const F14: Register`

- <span id="mips-const-f15"></span>`const F15: Register`

- <span id="mips-const-f16"></span>`const F16: Register`

- <span id="mips-const-f17"></span>`const F17: Register`

- <span id="mips-const-f18"></span>`const F18: Register`

- <span id="mips-const-f19"></span>`const F19: Register`

- <span id="mips-const-f20"></span>`const F20: Register`

- <span id="mips-const-f21"></span>`const F21: Register`

- <span id="mips-const-f22"></span>`const F22: Register`

- <span id="mips-const-f23"></span>`const F23: Register`

- <span id="mips-const-f24"></span>`const F24: Register`

- <span id="mips-const-f25"></span>`const F25: Register`

- <span id="mips-const-f26"></span>`const F26: Register`

- <span id="mips-const-f27"></span>`const F27: Register`

- <span id="mips-const-f28"></span>`const F28: Register`

- <span id="mips-const-f29"></span>`const F29: Register`

- <span id="mips-const-f30"></span>`const F30: Register`

- <span id="mips-const-f31"></span>`const F31: Register`

- <span id="mips-const-hi"></span>`const HI: Register`

- <span id="mips-const-lo"></span>`const LO: Register`

- <span id="mips-const-zero"></span>`const ZERO: Register`

- <span id="mips-const-at"></span>`const AT: Register`

- <span id="mips-const-v0"></span>`const V0: Register`

- <span id="mips-const-v1"></span>`const V1: Register`

- <span id="mips-const-a0"></span>`const A0: Register`

- <span id="mips-const-a1"></span>`const A1: Register`

- <span id="mips-const-a2"></span>`const A2: Register`

- <span id="mips-const-a3"></span>`const A3: Register`

- <span id="mips-const-t0"></span>`const T0: Register`

- <span id="mips-const-t1"></span>`const T1: Register`

- <span id="mips-const-t2"></span>`const T2: Register`

- <span id="mips-const-t3"></span>`const T3: Register`

- <span id="mips-const-t4"></span>`const T4: Register`

- <span id="mips-const-t5"></span>`const T5: Register`

- <span id="mips-const-t6"></span>`const T6: Register`

- <span id="mips-const-t7"></span>`const T7: Register`

- <span id="mips-const-s0"></span>`const S0: Register`

- <span id="mips-const-s1"></span>`const S1: Register`

- <span id="mips-const-s2"></span>`const S2: Register`

- <span id="mips-const-s3"></span>`const S3: Register`

- <span id="mips-const-s4"></span>`const S4: Register`

- <span id="mips-const-s5"></span>`const S5: Register`

- <span id="mips-const-s6"></span>`const S6: Register`

- <span id="mips-const-s7"></span>`const S7: Register`

- <span id="mips-const-t8"></span>`const T8: Register`

- <span id="mips-const-t9"></span>`const T9: Register`

- <span id="mips-const-k0"></span>`const K0: Register`

- <span id="mips-const-k1"></span>`const K1: Register`

- <span id="mips-const-gp"></span>`const GP: Register`

- <span id="mips-const-sp"></span>`const SP: Register`

- <span id="mips-const-fp"></span>`const FP: Register`

- <span id="mips-const-ra"></span>`const RA: Register`

- <span id="mips-const-s8"></span>`const S8: Register`

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

*Defined in [`gimli-0.32.3/src/arch.rs:629`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L629)*

RISC-V architecture specific definitions.

See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc).

#### Implementations

- <span id="riscv-const-x0"></span>`const X0: Register`

- <span id="riscv-const-x1"></span>`const X1: Register`

- <span id="riscv-const-x2"></span>`const X2: Register`

- <span id="riscv-const-x3"></span>`const X3: Register`

- <span id="riscv-const-x4"></span>`const X4: Register`

- <span id="riscv-const-x5"></span>`const X5: Register`

- <span id="riscv-const-x6"></span>`const X6: Register`

- <span id="riscv-const-x7"></span>`const X7: Register`

- <span id="riscv-const-x8"></span>`const X8: Register`

- <span id="riscv-const-x9"></span>`const X9: Register`

- <span id="riscv-const-x10"></span>`const X10: Register`

- <span id="riscv-const-x11"></span>`const X11: Register`

- <span id="riscv-const-x12"></span>`const X12: Register`

- <span id="riscv-const-x13"></span>`const X13: Register`

- <span id="riscv-const-x14"></span>`const X14: Register`

- <span id="riscv-const-x15"></span>`const X15: Register`

- <span id="riscv-const-x16"></span>`const X16: Register`

- <span id="riscv-const-x17"></span>`const X17: Register`

- <span id="riscv-const-x18"></span>`const X18: Register`

- <span id="riscv-const-x19"></span>`const X19: Register`

- <span id="riscv-const-x20"></span>`const X20: Register`

- <span id="riscv-const-x21"></span>`const X21: Register`

- <span id="riscv-const-x22"></span>`const X22: Register`

- <span id="riscv-const-x23"></span>`const X23: Register`

- <span id="riscv-const-x24"></span>`const X24: Register`

- <span id="riscv-const-x25"></span>`const X25: Register`

- <span id="riscv-const-x26"></span>`const X26: Register`

- <span id="riscv-const-x27"></span>`const X27: Register`

- <span id="riscv-const-x28"></span>`const X28: Register`

- <span id="riscv-const-x29"></span>`const X29: Register`

- <span id="riscv-const-x30"></span>`const X30: Register`

- <span id="riscv-const-x31"></span>`const X31: Register`

- <span id="riscv-const-f0"></span>`const F0: Register`

- <span id="riscv-const-f1"></span>`const F1: Register`

- <span id="riscv-const-f2"></span>`const F2: Register`

- <span id="riscv-const-f3"></span>`const F3: Register`

- <span id="riscv-const-f4"></span>`const F4: Register`

- <span id="riscv-const-f5"></span>`const F5: Register`

- <span id="riscv-const-f6"></span>`const F6: Register`

- <span id="riscv-const-f7"></span>`const F7: Register`

- <span id="riscv-const-f8"></span>`const F8: Register`

- <span id="riscv-const-f9"></span>`const F9: Register`

- <span id="riscv-const-f10"></span>`const F10: Register`

- <span id="riscv-const-f11"></span>`const F11: Register`

- <span id="riscv-const-f12"></span>`const F12: Register`

- <span id="riscv-const-f13"></span>`const F13: Register`

- <span id="riscv-const-f14"></span>`const F14: Register`

- <span id="riscv-const-f15"></span>`const F15: Register`

- <span id="riscv-const-f16"></span>`const F16: Register`

- <span id="riscv-const-f17"></span>`const F17: Register`

- <span id="riscv-const-f18"></span>`const F18: Register`

- <span id="riscv-const-f19"></span>`const F19: Register`

- <span id="riscv-const-f20"></span>`const F20: Register`

- <span id="riscv-const-f21"></span>`const F21: Register`

- <span id="riscv-const-f22"></span>`const F22: Register`

- <span id="riscv-const-f23"></span>`const F23: Register`

- <span id="riscv-const-f24"></span>`const F24: Register`

- <span id="riscv-const-f25"></span>`const F25: Register`

- <span id="riscv-const-f26"></span>`const F26: Register`

- <span id="riscv-const-f27"></span>`const F27: Register`

- <span id="riscv-const-f28"></span>`const F28: Register`

- <span id="riscv-const-f29"></span>`const F29: Register`

- <span id="riscv-const-f30"></span>`const F30: Register`

- <span id="riscv-const-f31"></span>`const F31: Register`

- <span id="riscv-const-zero"></span>`const ZERO: Register`

- <span id="riscv-const-ra"></span>`const RA: Register`

- <span id="riscv-const-sp"></span>`const SP: Register`

- <span id="riscv-const-gp"></span>`const GP: Register`

- <span id="riscv-const-tp"></span>`const TP: Register`

- <span id="riscv-const-t0"></span>`const T0: Register`

- <span id="riscv-const-t1"></span>`const T1: Register`

- <span id="riscv-const-t2"></span>`const T2: Register`

- <span id="riscv-const-s0"></span>`const S0: Register`

- <span id="riscv-const-s1"></span>`const S1: Register`

- <span id="riscv-const-a0"></span>`const A0: Register`

- <span id="riscv-const-a1"></span>`const A1: Register`

- <span id="riscv-const-a2"></span>`const A2: Register`

- <span id="riscv-const-a3"></span>`const A3: Register`

- <span id="riscv-const-a4"></span>`const A4: Register`

- <span id="riscv-const-a5"></span>`const A5: Register`

- <span id="riscv-const-a6"></span>`const A6: Register`

- <span id="riscv-const-a7"></span>`const A7: Register`

- <span id="riscv-const-s2"></span>`const S2: Register`

- <span id="riscv-const-s3"></span>`const S3: Register`

- <span id="riscv-const-s4"></span>`const S4: Register`

- <span id="riscv-const-s5"></span>`const S5: Register`

- <span id="riscv-const-s6"></span>`const S6: Register`

- <span id="riscv-const-s7"></span>`const S7: Register`

- <span id="riscv-const-s8"></span>`const S8: Register`

- <span id="riscv-const-s9"></span>`const S9: Register`

- <span id="riscv-const-s10"></span>`const S10: Register`

- <span id="riscv-const-s11"></span>`const S11: Register`

- <span id="riscv-const-t3"></span>`const T3: Register`

- <span id="riscv-const-t4"></span>`const T4: Register`

- <span id="riscv-const-t5"></span>`const T5: Register`

- <span id="riscv-const-t6"></span>`const T6: Register`

- <span id="riscv-const-ft0"></span>`const FT0: Register`

- <span id="riscv-const-ft1"></span>`const FT1: Register`

- <span id="riscv-const-ft2"></span>`const FT2: Register`

- <span id="riscv-const-ft3"></span>`const FT3: Register`

- <span id="riscv-const-ft4"></span>`const FT4: Register`

- <span id="riscv-const-ft5"></span>`const FT5: Register`

- <span id="riscv-const-ft6"></span>`const FT6: Register`

- <span id="riscv-const-ft7"></span>`const FT7: Register`

- <span id="riscv-const-fs0"></span>`const FS0: Register`

- <span id="riscv-const-fs1"></span>`const FS1: Register`

- <span id="riscv-const-fa0"></span>`const FA0: Register`

- <span id="riscv-const-fa1"></span>`const FA1: Register`

- <span id="riscv-const-fa2"></span>`const FA2: Register`

- <span id="riscv-const-fa3"></span>`const FA3: Register`

- <span id="riscv-const-fa4"></span>`const FA4: Register`

- <span id="riscv-const-fa5"></span>`const FA5: Register`

- <span id="riscv-const-fa6"></span>`const FA6: Register`

- <span id="riscv-const-fa7"></span>`const FA7: Register`

- <span id="riscv-const-fs2"></span>`const FS2: Register`

- <span id="riscv-const-fs3"></span>`const FS3: Register`

- <span id="riscv-const-fs4"></span>`const FS4: Register`

- <span id="riscv-const-fs5"></span>`const FS5: Register`

- <span id="riscv-const-fs6"></span>`const FS6: Register`

- <span id="riscv-const-fs7"></span>`const FS7: Register`

- <span id="riscv-const-fs8"></span>`const FS8: Register`

- <span id="riscv-const-fs9"></span>`const FS9: Register`

- <span id="riscv-const-fs10"></span>`const FS10: Register`

- <span id="riscv-const-fs11"></span>`const FS11: Register`

- <span id="riscv-const-ft8"></span>`const FT8: Register`

- <span id="riscv-const-ft9"></span>`const FT9: Register`

- <span id="riscv-const-ft10"></span>`const FT10: Register`

- <span id="riscv-const-ft11"></span>`const FT11: Register`

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

*Defined in [`gimli-0.32.3/src/arch.rs:770`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L770)*

Intel i386 architecture specific definitions.

See section 2.4.2 of the [i386 psABI](https://gitlab.com/x86-psABIs/i386-ABI).

#### Implementations

- <span id="x86-const-eax"></span>`const EAX: Register`

- <span id="x86-const-ecx"></span>`const ECX: Register`

- <span id="x86-const-edx"></span>`const EDX: Register`

- <span id="x86-const-ebx"></span>`const EBX: Register`

- <span id="x86-const-esp"></span>`const ESP: Register`

- <span id="x86-const-ebp"></span>`const EBP: Register`

- <span id="x86-const-esi"></span>`const ESI: Register`

- <span id="x86-const-edi"></span>`const EDI: Register`

- <span id="x86-const-ra"></span>`const RA: Register`

- <span id="x86-const-st0"></span>`const ST0: Register`

- <span id="x86-const-st1"></span>`const ST1: Register`

- <span id="x86-const-st2"></span>`const ST2: Register`

- <span id="x86-const-st3"></span>`const ST3: Register`

- <span id="x86-const-st4"></span>`const ST4: Register`

- <span id="x86-const-st5"></span>`const ST5: Register`

- <span id="x86-const-st6"></span>`const ST6: Register`

- <span id="x86-const-st7"></span>`const ST7: Register`

- <span id="x86-const-xmm0"></span>`const XMM0: Register`

- <span id="x86-const-xmm1"></span>`const XMM1: Register`

- <span id="x86-const-xmm2"></span>`const XMM2: Register`

- <span id="x86-const-xmm3"></span>`const XMM3: Register`

- <span id="x86-const-xmm4"></span>`const XMM4: Register`

- <span id="x86-const-xmm5"></span>`const XMM5: Register`

- <span id="x86-const-xmm6"></span>`const XMM6: Register`

- <span id="x86-const-xmm7"></span>`const XMM7: Register`

- <span id="x86-const-mm0"></span>`const MM0: Register`

- <span id="x86-const-mm1"></span>`const MM1: Register`

- <span id="x86-const-mm2"></span>`const MM2: Register`

- <span id="x86-const-mm3"></span>`const MM3: Register`

- <span id="x86-const-mm4"></span>`const MM4: Register`

- <span id="x86-const-mm5"></span>`const MM5: Register`

- <span id="x86-const-mm6"></span>`const MM6: Register`

- <span id="x86-const-mm7"></span>`const MM7: Register`

- <span id="x86-const-mxcsr"></span>`const MXCSR: Register`

- <span id="x86-const-es"></span>`const ES: Register`

- <span id="x86-const-cs"></span>`const CS: Register`

- <span id="x86-const-ss"></span>`const SS: Register`

- <span id="x86-const-ds"></span>`const DS: Register`

- <span id="x86-const-fs"></span>`const FS: Register`

- <span id="x86-const-gs"></span>`const GS: Register`

- <span id="x86-const-tr"></span>`const TR: Register`

- <span id="x86-const-ldtr"></span>`const LDTR: Register`

- <span id="x86-const-fs-base"></span>`const FS_BASE: Register`

- <span id="x86-const-gs-base"></span>`const GS_BASE: Register`

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

*Defined in [`gimli-0.32.3/src/arch.rs:832`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L832)*

AMD64 architecture specific definitions.

See section 3.6.2 of the [x86-64 psABI](https://gitlab.com/x86-psABIs/x86-64-ABI).

#### Implementations

- <span id="x86-64-const-rax"></span>`const RAX: Register`

- <span id="x86-64-const-rdx"></span>`const RDX: Register`

- <span id="x86-64-const-rcx"></span>`const RCX: Register`

- <span id="x86-64-const-rbx"></span>`const RBX: Register`

- <span id="x86-64-const-rsi"></span>`const RSI: Register`

- <span id="x86-64-const-rdi"></span>`const RDI: Register`

- <span id="x86-64-const-rbp"></span>`const RBP: Register`

- <span id="x86-64-const-rsp"></span>`const RSP: Register`

- <span id="x86-64-const-r8"></span>`const R8: Register`

- <span id="x86-64-const-r9"></span>`const R9: Register`

- <span id="x86-64-const-r10"></span>`const R10: Register`

- <span id="x86-64-const-r11"></span>`const R11: Register`

- <span id="x86-64-const-r12"></span>`const R12: Register`

- <span id="x86-64-const-r13"></span>`const R13: Register`

- <span id="x86-64-const-r14"></span>`const R14: Register`

- <span id="x86-64-const-r15"></span>`const R15: Register`

- <span id="x86-64-const-ra"></span>`const RA: Register`

- <span id="x86-64-const-xmm0"></span>`const XMM0: Register`

- <span id="x86-64-const-xmm1"></span>`const XMM1: Register`

- <span id="x86-64-const-xmm2"></span>`const XMM2: Register`

- <span id="x86-64-const-xmm3"></span>`const XMM3: Register`

- <span id="x86-64-const-xmm4"></span>`const XMM4: Register`

- <span id="x86-64-const-xmm5"></span>`const XMM5: Register`

- <span id="x86-64-const-xmm6"></span>`const XMM6: Register`

- <span id="x86-64-const-xmm7"></span>`const XMM7: Register`

- <span id="x86-64-const-xmm8"></span>`const XMM8: Register`

- <span id="x86-64-const-xmm9"></span>`const XMM9: Register`

- <span id="x86-64-const-xmm10"></span>`const XMM10: Register`

- <span id="x86-64-const-xmm11"></span>`const XMM11: Register`

- <span id="x86-64-const-xmm12"></span>`const XMM12: Register`

- <span id="x86-64-const-xmm13"></span>`const XMM13: Register`

- <span id="x86-64-const-xmm14"></span>`const XMM14: Register`

- <span id="x86-64-const-xmm15"></span>`const XMM15: Register`

- <span id="x86-64-const-st0"></span>`const ST0: Register`

- <span id="x86-64-const-st1"></span>`const ST1: Register`

- <span id="x86-64-const-st2"></span>`const ST2: Register`

- <span id="x86-64-const-st3"></span>`const ST3: Register`

- <span id="x86-64-const-st4"></span>`const ST4: Register`

- <span id="x86-64-const-st5"></span>`const ST5: Register`

- <span id="x86-64-const-st6"></span>`const ST6: Register`

- <span id="x86-64-const-st7"></span>`const ST7: Register`

- <span id="x86-64-const-mm0"></span>`const MM0: Register`

- <span id="x86-64-const-mm1"></span>`const MM1: Register`

- <span id="x86-64-const-mm2"></span>`const MM2: Register`

- <span id="x86-64-const-mm3"></span>`const MM3: Register`

- <span id="x86-64-const-mm4"></span>`const MM4: Register`

- <span id="x86-64-const-mm5"></span>`const MM5: Register`

- <span id="x86-64-const-mm6"></span>`const MM6: Register`

- <span id="x86-64-const-mm7"></span>`const MM7: Register`

- <span id="x86-64-const-rflags"></span>`const RFLAGS: Register`

- <span id="x86-64-const-es"></span>`const ES: Register`

- <span id="x86-64-const-cs"></span>`const CS: Register`

- <span id="x86-64-const-ss"></span>`const SS: Register`

- <span id="x86-64-const-ds"></span>`const DS: Register`

- <span id="x86-64-const-fs"></span>`const FS: Register`

- <span id="x86-64-const-gs"></span>`const GS: Register`

- <span id="x86-64-const-fs-base"></span>`const FS_BASE: Register`

- <span id="x86-64-const-gs-base"></span>`const GS_BASE: Register`

- <span id="x86-64-const-tr"></span>`const TR: Register`

- <span id="x86-64-const-ldtr"></span>`const LDTR: Register`

- <span id="x86-64-const-mxcsr"></span>`const MXCSR: Register`

- <span id="x86-64-const-fcw"></span>`const FCW: Register`

- <span id="x86-64-const-fsw"></span>`const FSW: Register`

- <span id="x86-64-const-xmm16"></span>`const XMM16: Register`

- <span id="x86-64-const-xmm17"></span>`const XMM17: Register`

- <span id="x86-64-const-xmm18"></span>`const XMM18: Register`

- <span id="x86-64-const-xmm19"></span>`const XMM19: Register`

- <span id="x86-64-const-xmm20"></span>`const XMM20: Register`

- <span id="x86-64-const-xmm21"></span>`const XMM21: Register`

- <span id="x86-64-const-xmm22"></span>`const XMM22: Register`

- <span id="x86-64-const-xmm23"></span>`const XMM23: Register`

- <span id="x86-64-const-xmm24"></span>`const XMM24: Register`

- <span id="x86-64-const-xmm25"></span>`const XMM25: Register`

- <span id="x86-64-const-xmm26"></span>`const XMM26: Register`

- <span id="x86-64-const-xmm27"></span>`const XMM27: Register`

- <span id="x86-64-const-xmm28"></span>`const XMM28: Register`

- <span id="x86-64-const-xmm29"></span>`const XMM29: Register`

- <span id="x86-64-const-xmm30"></span>`const XMM30: Register`

- <span id="x86-64-const-xmm31"></span>`const XMM31: Register`

- <span id="x86-64-const-k0"></span>`const K0: Register`

- <span id="x86-64-const-k1"></span>`const K1: Register`

- <span id="x86-64-const-k2"></span>`const K2: Register`

- <span id="x86-64-const-k3"></span>`const K3: Register`

- <span id="x86-64-const-k4"></span>`const K4: Register`

- <span id="x86-64-const-k5"></span>`const K5: Register`

- <span id="x86-64-const-k6"></span>`const K6: Register`

- <span id="x86-64-const-k7"></span>`const K7: Register`

- <span id="x86-64-const-r16"></span>`const R16: Register`

- <span id="x86-64-const-r17"></span>`const R17: Register`

- <span id="x86-64-const-r18"></span>`const R18: Register`

- <span id="x86-64-const-r19"></span>`const R19: Register`

- <span id="x86-64-const-r20"></span>`const R20: Register`

- <span id="x86-64-const-r21"></span>`const R21: Register`

- <span id="x86-64-const-r22"></span>`const R22: Register`

- <span id="x86-64-const-r23"></span>`const R23: Register`

- <span id="x86-64-const-r24"></span>`const R24: Register`

- <span id="x86-64-const-r25"></span>`const R25: Register`

- <span id="x86-64-const-r26"></span>`const R26: Register`

- <span id="x86-64-const-r27"></span>`const R27: Register`

- <span id="x86-64-const-r28"></span>`const R28: Register`

- <span id="x86-64-const-r29"></span>`const R29: Register`

- <span id="x86-64-const-r30"></span>`const R30: Register`

- <span id="x86-64-const-r31"></span>`const R31: Register`

- <span id="x86-64-const-tmm0"></span>`const TMM0: Register`

- <span id="x86-64-const-tmm1"></span>`const TMM1: Register`

- <span id="x86-64-const-tmm2"></span>`const TMM2: Register`

- <span id="x86-64-const-tmm3"></span>`const TMM3: Register`

- <span id="x86-64-const-tmm4"></span>`const TMM4: Register`

- <span id="x86-64-const-tmm5"></span>`const TMM5: Register`

- <span id="x86-64-const-tmm6"></span>`const TMM6: Register`

- <span id="x86-64-const-tmm7"></span>`const TMM7: Register`

- <span id="x86-64-const-tilecfg"></span>`const TILECFG: Register`

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

*Defined in [`gimli-0.32.3/src/arch.rs:967`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L967)*

PowerPC 64bit

See [64-bit ELF ABI Specification for OpenPOWER Architecture](https://openpowerfoundation.org/specifications/64bitelfabi/).

#### Implementations

- <span id="powerpc64-const-r0"></span>`const R0: Register`

- <span id="powerpc64-const-r1"></span>`const R1: Register`

- <span id="powerpc64-const-r2"></span>`const R2: Register`

- <span id="powerpc64-const-r3"></span>`const R3: Register`

- <span id="powerpc64-const-r4"></span>`const R4: Register`

- <span id="powerpc64-const-r5"></span>`const R5: Register`

- <span id="powerpc64-const-r6"></span>`const R6: Register`

- <span id="powerpc64-const-r7"></span>`const R7: Register`

- <span id="powerpc64-const-r8"></span>`const R8: Register`

- <span id="powerpc64-const-r9"></span>`const R9: Register`

- <span id="powerpc64-const-r10"></span>`const R10: Register`

- <span id="powerpc64-const-r11"></span>`const R11: Register`

- <span id="powerpc64-const-r12"></span>`const R12: Register`

- <span id="powerpc64-const-r13"></span>`const R13: Register`

- <span id="powerpc64-const-r14"></span>`const R14: Register`

- <span id="powerpc64-const-r15"></span>`const R15: Register`

- <span id="powerpc64-const-r16"></span>`const R16: Register`

- <span id="powerpc64-const-r17"></span>`const R17: Register`

- <span id="powerpc64-const-r18"></span>`const R18: Register`

- <span id="powerpc64-const-r19"></span>`const R19: Register`

- <span id="powerpc64-const-r20"></span>`const R20: Register`

- <span id="powerpc64-const-r21"></span>`const R21: Register`

- <span id="powerpc64-const-r22"></span>`const R22: Register`

- <span id="powerpc64-const-r23"></span>`const R23: Register`

- <span id="powerpc64-const-r24"></span>`const R24: Register`

- <span id="powerpc64-const-r25"></span>`const R25: Register`

- <span id="powerpc64-const-r26"></span>`const R26: Register`

- <span id="powerpc64-const-r27"></span>`const R27: Register`

- <span id="powerpc64-const-r28"></span>`const R28: Register`

- <span id="powerpc64-const-r29"></span>`const R29: Register`

- <span id="powerpc64-const-r30"></span>`const R30: Register`

- <span id="powerpc64-const-r31"></span>`const R31: Register`

- <span id="powerpc64-const-f0"></span>`const F0: Register`

- <span id="powerpc64-const-f1"></span>`const F1: Register`

- <span id="powerpc64-const-f2"></span>`const F2: Register`

- <span id="powerpc64-const-f3"></span>`const F3: Register`

- <span id="powerpc64-const-f4"></span>`const F4: Register`

- <span id="powerpc64-const-f5"></span>`const F5: Register`

- <span id="powerpc64-const-f6"></span>`const F6: Register`

- <span id="powerpc64-const-f7"></span>`const F7: Register`

- <span id="powerpc64-const-f8"></span>`const F8: Register`

- <span id="powerpc64-const-f9"></span>`const F9: Register`

- <span id="powerpc64-const-f10"></span>`const F10: Register`

- <span id="powerpc64-const-f11"></span>`const F11: Register`

- <span id="powerpc64-const-f12"></span>`const F12: Register`

- <span id="powerpc64-const-f13"></span>`const F13: Register`

- <span id="powerpc64-const-f14"></span>`const F14: Register`

- <span id="powerpc64-const-f15"></span>`const F15: Register`

- <span id="powerpc64-const-f16"></span>`const F16: Register`

- <span id="powerpc64-const-f17"></span>`const F17: Register`

- <span id="powerpc64-const-f18"></span>`const F18: Register`

- <span id="powerpc64-const-f19"></span>`const F19: Register`

- <span id="powerpc64-const-f20"></span>`const F20: Register`

- <span id="powerpc64-const-f21"></span>`const F21: Register`

- <span id="powerpc64-const-f22"></span>`const F22: Register`

- <span id="powerpc64-const-f23"></span>`const F23: Register`

- <span id="powerpc64-const-f24"></span>`const F24: Register`

- <span id="powerpc64-const-f25"></span>`const F25: Register`

- <span id="powerpc64-const-f26"></span>`const F26: Register`

- <span id="powerpc64-const-f27"></span>`const F27: Register`

- <span id="powerpc64-const-f28"></span>`const F28: Register`

- <span id="powerpc64-const-f29"></span>`const F29: Register`

- <span id="powerpc64-const-f30"></span>`const F30: Register`

- <span id="powerpc64-const-f31"></span>`const F31: Register`

- <span id="powerpc64-const-lr"></span>`const LR: Register`

- <span id="powerpc64-const-ctr"></span>`const CTR: Register`

- <span id="powerpc64-const-cr0"></span>`const CR0: Register`

- <span id="powerpc64-const-cr1"></span>`const CR1: Register`

- <span id="powerpc64-const-cr2"></span>`const CR2: Register`

- <span id="powerpc64-const-cr3"></span>`const CR3: Register`

- <span id="powerpc64-const-cr4"></span>`const CR4: Register`

- <span id="powerpc64-const-cr5"></span>`const CR5: Register`

- <span id="powerpc64-const-cr6"></span>`const CR6: Register`

- <span id="powerpc64-const-cr7"></span>`const CR7: Register`

- <span id="powerpc64-const-xer"></span>`const XER: Register`

- <span id="powerpc64-const-vr0"></span>`const VR0: Register`

- <span id="powerpc64-const-vr1"></span>`const VR1: Register`

- <span id="powerpc64-const-vr2"></span>`const VR2: Register`

- <span id="powerpc64-const-vr3"></span>`const VR3: Register`

- <span id="powerpc64-const-vr4"></span>`const VR4: Register`

- <span id="powerpc64-const-vr5"></span>`const VR5: Register`

- <span id="powerpc64-const-vr6"></span>`const VR6: Register`

- <span id="powerpc64-const-vr7"></span>`const VR7: Register`

- <span id="powerpc64-const-vr8"></span>`const VR8: Register`

- <span id="powerpc64-const-vr9"></span>`const VR9: Register`

- <span id="powerpc64-const-vr10"></span>`const VR10: Register`

- <span id="powerpc64-const-vr11"></span>`const VR11: Register`

- <span id="powerpc64-const-vr12"></span>`const VR12: Register`

- <span id="powerpc64-const-vr13"></span>`const VR13: Register`

- <span id="powerpc64-const-vr14"></span>`const VR14: Register`

- <span id="powerpc64-const-vr15"></span>`const VR15: Register`

- <span id="powerpc64-const-vr16"></span>`const VR16: Register`

- <span id="powerpc64-const-vr17"></span>`const VR17: Register`

- <span id="powerpc64-const-vr18"></span>`const VR18: Register`

- <span id="powerpc64-const-vr19"></span>`const VR19: Register`

- <span id="powerpc64-const-vr20"></span>`const VR20: Register`

- <span id="powerpc64-const-vr21"></span>`const VR21: Register`

- <span id="powerpc64-const-vr22"></span>`const VR22: Register`

- <span id="powerpc64-const-vr23"></span>`const VR23: Register`

- <span id="powerpc64-const-vr24"></span>`const VR24: Register`

- <span id="powerpc64-const-vr25"></span>`const VR25: Register`

- <span id="powerpc64-const-vr26"></span>`const VR26: Register`

- <span id="powerpc64-const-vr27"></span>`const VR27: Register`

- <span id="powerpc64-const-vr28"></span>`const VR28: Register`

- <span id="powerpc64-const-vr29"></span>`const VR29: Register`

- <span id="powerpc64-const-vr30"></span>`const VR30: Register`

- <span id="powerpc64-const-vr31"></span>`const VR31: Register`

- <span id="powerpc64-const-vscr"></span>`const VSCR: Register`

- <span id="powerpc64-const-tfhar"></span>`const TFHAR: Register`

- <span id="powerpc64-const-tfiar"></span>`const TFIAR: Register`

- <span id="powerpc64-const-texasr"></span>`const TEXASR: Register`

#### Trait Implementations

##### `impl Clone for PowerPc64`

- <span id="powerpc64-clone"></span>`fn clone(&self) -> PowerPc64` — [`PowerPc64`](../index.md)

##### `impl Copy for PowerPc64`

##### `impl Debug for PowerPc64`

- <span id="powerpc64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `registers!`

*Defined in [`gimli-0.32.3/src/arch.rs:3-43`](../../../.source_1765210505/gimli-0.32.3/src/arch.rs#L3-L43)*

