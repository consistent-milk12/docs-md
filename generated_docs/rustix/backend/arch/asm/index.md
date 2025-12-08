*[rustix](../../../index.md) / [backend](../../index.md) / [arch](../index.md) / [asm](index.md)*

---

# Module `asm`

x86-64 Linux system calls.

## Contents

- [Functions](#functions)
  - [`syscall0_readonly`](#syscall0_readonly)
  - [`syscall1`](#syscall1)
  - [`syscall1_readonly`](#syscall1_readonly)
  - [`syscall1_noreturn`](#syscall1_noreturn)
  - [`syscall2`](#syscall2)
  - [`syscall2_readonly`](#syscall2_readonly)
  - [`syscall3`](#syscall3)
  - [`syscall3_readonly`](#syscall3_readonly)
  - [`syscall4`](#syscall4)
  - [`syscall4_readonly`](#syscall4_readonly)
  - [`syscall5`](#syscall5)
  - [`syscall5_readonly`](#syscall5_readonly)
  - [`syscall6`](#syscall6)
  - [`syscall6_readonly`](#syscall6_readonly)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`syscall0_readonly`](#syscall0_readonly) | fn |  |
| [`syscall1`](#syscall1) | fn |  |
| [`syscall1_readonly`](#syscall1_readonly) | fn |  |
| [`syscall1_noreturn`](#syscall1_noreturn) | fn |  |
| [`syscall2`](#syscall2) | fn |  |
| [`syscall2_readonly`](#syscall2_readonly) | fn |  |
| [`syscall3`](#syscall3) | fn |  |
| [`syscall3_readonly`](#syscall3_readonly) | fn |  |
| [`syscall4`](#syscall4) | fn |  |
| [`syscall4_readonly`](#syscall4_readonly) | fn |  |
| [`syscall5`](#syscall5) | fn |  |
| [`syscall5_readonly`](#syscall5_readonly) | fn |  |
| [`syscall6`](#syscall6) | fn |  |
| [`syscall6_readonly`](#syscall6_readonly) | fn |  |

## Functions

### `syscall0_readonly`

```rust
unsafe fn syscall0_readonly(nr: crate::backend::reg::SyscallNumber<'_>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall1`

```rust
unsafe fn syscall1(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall1_readonly`

```rust
unsafe fn syscall1_readonly(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall1_noreturn`

```rust
unsafe fn syscall1_noreturn(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>) -> never
```

### `syscall2`

```rust
unsafe fn syscall2(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall2_readonly`

```rust
unsafe fn syscall2_readonly(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall3`

```rust
unsafe fn syscall3(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall3_readonly`

```rust
unsafe fn syscall3_readonly(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall4`

```rust
unsafe fn syscall4(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>, a3: crate::backend::reg::ArgReg<'_, crate::backend::reg::A3>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall4_readonly`

```rust
unsafe fn syscall4_readonly(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>, a3: crate::backend::reg::ArgReg<'_, crate::backend::reg::A3>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall5`

```rust
unsafe fn syscall5(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>, a3: crate::backend::reg::ArgReg<'_, crate::backend::reg::A3>, a4: crate::backend::reg::ArgReg<'_, crate::backend::reg::A4>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall5_readonly`

```rust
unsafe fn syscall5_readonly(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>, a3: crate::backend::reg::ArgReg<'_, crate::backend::reg::A3>, a4: crate::backend::reg::ArgReg<'_, crate::backend::reg::A4>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall6`

```rust
unsafe fn syscall6(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>, a3: crate::backend::reg::ArgReg<'_, crate::backend::reg::A3>, a4: crate::backend::reg::ArgReg<'_, crate::backend::reg::A4>, a5: crate::backend::reg::ArgReg<'_, crate::backend::reg::A5>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

### `syscall6_readonly`

```rust
unsafe fn syscall6_readonly(nr: crate::backend::reg::SyscallNumber<'_>, a0: crate::backend::reg::ArgReg<'_, crate::backend::reg::A0>, a1: crate::backend::reg::ArgReg<'_, crate::backend::reg::A1>, a2: crate::backend::reg::ArgReg<'_, crate::backend::reg::A2>, a3: crate::backend::reg::ArgReg<'_, crate::backend::reg::A3>, a4: crate::backend::reg::ArgReg<'_, crate::backend::reg::A4>, a5: crate::backend::reg::ArgReg<'_, crate::backend::reg::A5>) -> crate::backend::reg::RetReg<crate::backend::reg::R0>
```

