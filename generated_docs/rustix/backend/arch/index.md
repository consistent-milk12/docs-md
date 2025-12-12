*[rustix](../../index.md) / [backend](../index.md) / [arch](index.md)*

---

# Module `arch`

Architecture-specific syscall code.

This module also has a `choose` submodule which chooses a scheme and is
what most of the `rustix` syscalls use.

Compilers should really have intrinsics for making system calls. They're
much like regular calls, with custom calling conventions, and calling
conventions are otherwise the compiler's job. But for now, use inline asm.

The calling conventions for Linux syscalls are [documented here].

# Safety

This contains the inline `asm` statements performing the syscall
instructions.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`asm`](#asm) | mod | x86-64 Linux system calls. |
| [`syscall!`](#syscall) | macro |  |
| [`syscall_always_asm!`](#syscall-always-asm) | macro |  |
| [`syscall_readonly!`](#syscall-readonly) | macro | Like `syscall`, but adds the `readonly` attribute to the inline asm, which indicates that the syscall does not mutate any memory. |

## Modules

- [`asm`](asm/index.md) — x86-64 Linux system calls.

## Macros

### `syscall!`

*Defined in [`rustix-1.1.2/src/backend/linux_raw/arch/mod.rs:75-151`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/arch/mod.rs#L75-L151)*

### `syscall_always_asm!`

*Defined in [`rustix-1.1.2/src/backend/linux_raw/arch/mod.rs:156-230`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/arch/mod.rs#L156-L230)*

### `syscall_readonly!`

*Defined in [`rustix-1.1.2/src/backend/linux_raw/arch/mod.rs:234-310`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/arch/mod.rs#L234-L310)*

Like `syscall`, but adds the `readonly` attribute to the inline asm, which
indicates that the syscall does not mutate any memory.

