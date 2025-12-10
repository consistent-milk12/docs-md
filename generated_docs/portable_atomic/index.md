# Crate `portable_atomic`

<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

Portable atomic types including support for 128-bit atomics, atomic float, etc.

- Provide all atomic integer types (`Atomic{I,U}{8,16,32,64}`) for all targets that can use atomic CAS. (i.e., all targets that can use `std`, and most no-std targets)
- Provide `AtomicI128` and `AtomicU128`.
- Provide `AtomicF32` and `AtomicF64`. ([optional, requires the `float` feature](#optional-features-float))
- Provide `AtomicF16` and `AtomicF128` for [unstable `f16` and `f128`](https://github.com/rust-lang/rust/issues/116909). ([optional, requires the `float` feature and unstable cfgs](#optional-features-float))
- Provide atomic load/store for targets where atomic is not available at all in the standard library. (RISC-V without A-extension, MSP430, AVR)
- Provide atomic CAS for targets where atomic CAS is not available in the standard library. (thumbv6m, pre-v6 Arm, RISC-V without A-extension, MSP430, AVR, Xtensa, etc.) (always enabled for MSP430 and AVR, [optional](#optional-features-critical-section) otherwise)
- Provide stable equivalents of the standard library's atomic types' unstable APIs, such as [`AtomicPtr::fetch_*`](https://github.com/rust-lang/rust/issues/99108).
- Make features that require newer compilers, such as [`fetch_{max,min}`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_max), [`fetch_update`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_update), [`as_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.as_ptr), [`from_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.from_ptr), [`AtomicBool::fetch_not`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html#method.fetch_not) and [stronger CAS failure ordering](https://github.com/rust-lang/rust/pull/98383) available on Rust 1.34+.
- Provide workaround for bugs in the standard library's atomic-related APIs, such as [rust-lang/rust#100650], `fence`/`compiler_fence` on MSP430 that cause LLVM error, etc.

<!-- TODO:
- mention Atomic{I,U}*::fetch_neg, Atomic{I*,U*,Ptr}::bit_*, etc.
- mention optimizations not available in the standard library's equivalents
-->

portable-atomic version of `std::sync::Arc` is provided by the [portable-atomic-util](https://github.com/taiki-e/portable-atomic/tree/HEAD/portable-atomic-util) crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
portable-atomic = "1"
```

The default features are mainly for users who use atomics larger than the pointer width.
If you don't need them, disabling the default features may reduce code size and compile time slightly.

```toml
[dependencies]
portable-atomic = { version = "1", default-features = false }
```

If your crate supports no-std environment and requires atomic CAS, enabling the `require-cas` feature will allow the `portable-atomic` to display a [helpful error message](https://github.com/taiki-e/portable-atomic/pull/100) to users on targets requiring additional action on the user side to provide atomic CAS.

```toml
[dependencies]
portable-atomic = { version = "1.3", default-features = false, features = ["require-cas"] }
```

## 128-bit atomics support

Native 128-bit atomic operations are available on x86_64 (Rust 1.59+), AArch64 (Rust 1.59+), riscv64 (Rust 1.59+), Arm64EC (Rust 1.84+), s390x (Rust 1.84+), and powerpc64 (nightly only), otherwise the fallback implementation is used.

On x86_64, even if `cmpxchg16b` is not available at compile-time (Note: `cmpxchg16b` target feature is enabled by default only on Apple, Windows (except Windows 7), and Fuchsia targets), run-time detection checks whether `cmpxchg16b` is available. If `cmpxchg16b` is not available at either compile-time or run-time detection, the fallback implementation is used. See also [`portable_atomic_no_outline_atomics`](#optional-cfg-no-outline-atomics) cfg.

They are usually implemented using inline assembly, and when using Miri or ThreadSanitizer that do not support inline assembly, core intrinsics are used instead of inline assembly if possible.

See the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md) for details.

## Optional features

- **`fallback`** *(enabled by default)*<br>
  Enable fallback implementations.

  Disabling this allows only atomic types for which the platform natively supports atomic operations.

- <a name="optional-features-float"></a>**`float`**<br>
  Provide `AtomicF{32,64}`.

  - When unstable `--cfg portable_atomic_unstable_f16` is also enabled, `AtomicF16` for [unstable `f16`](https://github.com/rust-lang/rust/issues/116909) is also provided.
  - When unstable `--cfg portable_atomic_unstable_f128` is also enabled, `AtomicF128` for [unstable `f128`](https://github.com/rust-lang/rust/issues/116909) is also provided.

  Note:
  - Atomic float's `fetch_{add,sub,min,max}` are usually implemented using CAS loops, which can be slower than equivalent operations of atomic integers. As an exception, AArch64 with FEAT_LSFE and GPU targets have atomic float instructions and we use them on AArch64 when `lsfe` target feature is available at compile-time. We [plan to use atomic float instructions for GPU targets as well in the future.](https://github.com/taiki-e/portable-atomic/issues/34))
  - Unstable cfgs are outside of the normal semver guarantees and minor or patch versions of portable-atomic may make breaking changes to them at any time.

- **`std`**<br>
  Use `std`.

- <a name="optional-features-require-cas"></a>**`require-cas`**<br>
  Emit compile error if atomic CAS is not available. See [Usage](#usage) section and [#100](https://github.com/taiki-e/portable-atomic/pull/100) for more.

- <a name="optional-features-serde"></a>**`serde`**<br>
  Implement `serde::{Serialize,Deserialize}` for atomic types.

  Note:
  - The MSRV when this feature is enabled depends on the MSRV of [`serde`](../serde/index.md).

- <a name="optional-features-critical-section"></a>**`critical-section`**<br>
  When this feature is enabled, this crate uses [critical-section] to provide atomic CAS for targets where
  it is not natively available. When enabling it, you should provide a suitable critical section implementation
  for the current target, see the [critical-section] documentation for details on how to do so.

  `critical-section` support is useful to get atomic CAS when the [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core) can't be used,
  such as multi-core targets, unprivileged code running under some RTOS, or environments where disabling interrupts
  needs extra care due to e.g. real-time requirements.

  Note that with the `critical-section` feature, critical sections are taken for all atomic operations, while with
  [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core) some operations don't require disabling interrupts (loads and stores, but
  additionally on MSP430 `add`, `sub`, `and`, `or`, `xor`, `not`). Therefore, for better performance, if
  all the `critical-section` implementation for your target does is disable interrupts, prefer using
  `unsafe-assume-single-core` feature instead.

  Note:
  - The MSRV when this feature is enabled depends on the MSRV of [critical-section].
  - It is usually *not* recommended to always enable this feature in dependencies of the library.

    Enabling this feature will prevent the end user from having the chance to take advantage of other (potentially) efficient implementations ([Implementations provided by `unsafe-assume-single-core` feature, default implementations on MSP430 and AVR](#optional-features-unsafe-assume-single-core), implementation proposed in [#60], etc. Other systems may also be supported in the future).

    The recommended approach for libraries is to leave it up to the end user whether or not to enable this feature. (However, it may make sense to enable this feature by default for libraries specific to a platform where other implementations are known not to work.)

    As an example, the end-user's `Cargo.toml` that uses a crate that provides a critical-section implementation and a crate that depends on portable-atomic as an option would be expected to look like this:

    ```toml
    [dependencies]
    portable-atomic = { version = "1", default-features = false, features = ["critical-section"] }
    crate-provides-critical-section-impl = "..."
    crate-uses-portable-atomic-as-feature = { version = "...", features = ["portable-atomic"] }
    ```

- <a name="optional-features-unsafe-assume-single-core"></a>**`unsafe-assume-single-core`**<br>
  Assume that the target is single-core.
  When this feature is enabled, this crate provides atomic CAS for targets where atomic CAS is not available in the standard library by disabling interrupts.

  This feature is `unsafe`, and note the following safety requirements:
  - Enabling this feature for multi-core systems is always **unsound**.
  - This uses privileged instructions to disable interrupts, so it usually doesn't work on unprivileged mode.
    Enabling this feature in an environment where privileged instructions are not available, or if the instructions used are not sufficient to disable interrupts in the system, it is also usually considered **unsound**, although the details are system-dependent.

    The following are known cases:
    - On pre-v6 Arm, this disables only IRQs by default. For many systems (e.g., GBA) this is enough. If the system need to disable both IRQs and FIQs, you need to enable the `disable-fiq` feature together.
    - On RISC-V without A-extension, this generates code for machine-mode (M-mode) by default. If you enable the `s-mode` together, this generates code for supervisor-mode (S-mode). In particular, `qemu-system-riscv*` uses [OpenSBI](https://github.com/riscv-software-src/opensbi) as the default firmware.

    See also the [`interrupt` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/interrupt/README.md).

  Consider using the [`critical-section` feature](#optional-features-critical-section) for systems that cannot use this feature.

  It is **very strongly discouraged** to enable this feature in libraries that depend on `portable-atomic`. The recommended approach for libraries is to leave it up to the end user whether or not to enable this feature. (However, it may make sense to enable this feature by default for libraries specific to a platform where it is guaranteed to always be sound, for example in a hardware abstraction layer targeting a single-core chip.)

  Armv6-M (thumbv6m), pre-v6 Arm (e.g., thumbv4t, thumbv5te), RISC-V without A-extension, and Xtensa are currently supported.

  Since all MSP430 and AVR are single-core, we always provide atomic CAS for them without this feature.

  Enabling this feature for targets that have atomic CAS will result in a compile error.

  Feel free to submit an issue if your target is not supported yet.

## Optional cfg

One of the ways to enable cfg is to set [rustflags in the cargo config](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerustflags):

```toml
.cargo/config.toml
[target.<target>]
rustflags = ["--cfg", "portable_atomic_no_outline_atomics"]
```

Or set environment variable:

```sh
RUSTFLAGS="--cfg portable_atomic_no_outline_atomics" cargo ...
```

- <a name="optional-cfg-unsafe-assume-single-core"></a>**`--cfg portable_atomic_unsafe_assume_single_core`**<br>
  Since 1.4.0, this cfg is an alias of [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core).

  Originally, we were providing these as cfgs instead of features, but based on a strong request from the embedded ecosystem, we have agreed to provide them as features as well. See [#94](https://github.com/taiki-e/portable-atomic/pull/94) for more.

- <a name="optional-cfg-no-outline-atomics"></a>**`--cfg portable_atomic_no_outline_atomics`**<br>
  Disable dynamic dispatching by run-time CPU feature detection.

  If dynamic dispatching by run-time CPU feature detection is enabled, it allows maintaining support for older CPUs while using features that are not supported on older CPUs, such as CMPXCHG16B (x86_64) and FEAT_LSE/FEAT_LSE2 (AArch64).

  Note:
  - Dynamic detection is currently only supported in x86_64, AArch64, Arm, RISC-V, Arm64EC, and powerpc64, otherwise it works the same as when this cfg is set.
  - If the required target features are enabled at compile-time, the atomic operations are inlined.
  - This is compatible with no-std (as with all features except `std`).
  - On some targets, run-time detection is disabled by default mainly for compatibility with incomplete build environments or support for it is experimental, and can be enabled by `--cfg portable_atomic_outline_atomics`. (When both cfg are enabled, `*_no_*` cfg is preferred.)
  - Some AArch64 targets enable LLVM's `outline-atomics` target feature by default, so if you set this cfg, you may want to disable that as well. (portable-atomic's outline-atomics does not depend on the compiler-rt symbols, so even if you need to disable LLVM's outline-atomics, you may not need to disable portable-atomic's outline-atomics.)

  See also the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md).

## Related Projects

- [atomic-maybe-uninit]: Atomic operations on potentially uninitialized integers.
- [atomic-memcpy]: Byte-wise atomic memcpy.






<!-- tidy:sync-markdown-to-rustdoc:end -->

## Contents

- [Modules](#modules)
  - [`cfgs`](#cfgs)
  - [`utils`](#utils)
  - [`imp`](#imp)
  - [`hint`](#hint)
- [Structs](#structs)
  - [`AtomicBool`](#atomicbool)
  - [`AtomicPtr`](#atomicptr)
  - [`AtomicIsize`](#atomicisize)
  - [`AtomicUsize`](#atomicusize)
  - [`AtomicI8`](#atomici8)
  - [`AtomicU8`](#atomicu8)
  - [`AtomicI16`](#atomici16)
  - [`AtomicU16`](#atomicu16)
  - [`AtomicI32`](#atomici32)
  - [`AtomicU32`](#atomicu32)
  - [`AtomicI64`](#atomici64)
  - [`AtomicU64`](#atomicu64)
  - [`AtomicI128`](#atomici128)
  - [`AtomicU128`](#atomicu128)
- [Functions](#functions)
  - [`Ordering`](#ordering)
- [Macros](#macros)
  - [`cfg_has_atomic_ptr!`](#cfg_has_atomic_ptr)
  - [`cfg_no_atomic_ptr!`](#cfg_no_atomic_ptr)
  - [`atomic_int!`](#atomic_int)
  - [`cfg_has_atomic_8!`](#cfg_has_atomic_8)
  - [`cfg_no_atomic_8!`](#cfg_no_atomic_8)
  - [`cfg_has_atomic_16!`](#cfg_has_atomic_16)
  - [`cfg_no_atomic_16!`](#cfg_no_atomic_16)
  - [`cfg_has_atomic_32!`](#cfg_has_atomic_32)
  - [`cfg_no_atomic_32!`](#cfg_no_atomic_32)
  - [`cfg_has_atomic_64!`](#cfg_has_atomic_64)
  - [`cfg_no_atomic_64!`](#cfg_no_atomic_64)
  - [`cfg_has_atomic_128!`](#cfg_has_atomic_128)
  - [`cfg_no_atomic_128!`](#cfg_no_atomic_128)
  - [`cfg_has_atomic_cas!`](#cfg_has_atomic_cas)
  - [`cfg_no_atomic_cas!`](#cfg_no_atomic_cas)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cfgs`](#cfgs) | mod |  |
| [`utils`](#utils) | mod |  |
| [`imp`](#imp) | mod |  |
| [`hint`](#hint) | mod | Re-export of the [`core::hint`] module. |
| [`AtomicBool`](#atomicbool) | struct | A boolean type which can be safely shared between threads. |
| [`AtomicPtr`](#atomicptr) | struct | A raw pointer type which can be safely shared between threads. |
| [`AtomicIsize`](#atomicisize) | struct | An integer type which can be safely shared between threads. |
| [`AtomicUsize`](#atomicusize) | struct | An integer type which can be safely shared between threads. |
| [`AtomicI8`](#atomici8) | struct | An integer type which can be safely shared between threads. |
| [`AtomicU8`](#atomicu8) | struct | An integer type which can be safely shared between threads. |
| [`AtomicI16`](#atomici16) | struct | An integer type which can be safely shared between threads. |
| [`AtomicU16`](#atomicu16) | struct | An integer type which can be safely shared between threads. |
| [`AtomicI32`](#atomici32) | struct | An integer type which can be safely shared between threads. |
| [`AtomicU32`](#atomicu32) | struct | An integer type which can be safely shared between threads. |
| [`AtomicI64`](#atomici64) | struct | An integer type which can be safely shared between threads. |
| [`AtomicU64`](#atomicu64) | struct | An integer type which can be safely shared between threads. |
| [`AtomicI128`](#atomici128) | struct | An integer type which can be safely shared between threads. |
| [`AtomicU128`](#atomicu128) | struct | An integer type which can be safely shared between threads. |
| [`Ordering`](#ordering) | fn |  |
| [`cfg_has_atomic_ptr!`](#cfg_has_atomic_ptr) | macro |  |
| [`cfg_no_atomic_ptr!`](#cfg_no_atomic_ptr) | macro |  |
| [`atomic_int!`](#atomic_int) | macro |  |
| [`cfg_has_atomic_8!`](#cfg_has_atomic_8) | macro |  |
| [`cfg_no_atomic_8!`](#cfg_no_atomic_8) | macro |  |
| [`cfg_has_atomic_16!`](#cfg_has_atomic_16) | macro |  |
| [`cfg_no_atomic_16!`](#cfg_no_atomic_16) | macro |  |
| [`cfg_has_atomic_32!`](#cfg_has_atomic_32) | macro |  |
| [`cfg_no_atomic_32!`](#cfg_no_atomic_32) | macro |  |
| [`cfg_has_atomic_64!`](#cfg_has_atomic_64) | macro |  |
| [`cfg_no_atomic_64!`](#cfg_no_atomic_64) | macro |  |
| [`cfg_has_atomic_128!`](#cfg_has_atomic_128) | macro |  |
| [`cfg_no_atomic_128!`](#cfg_no_atomic_128) | macro |  |
| [`cfg_has_atomic_cas!`](#cfg_has_atomic_cas) | macro |  |
| [`cfg_no_atomic_cas!`](#cfg_no_atomic_cas) | macro |  |

## Modules

- [`cfgs`](cfgs/index.md)
- [`utils`](utils/index.md)
- [`imp`](imp/index.md)
- [`hint`](hint/index.md) — Re-export of the [`core::hint`] module.

## Structs

### `AtomicBool`

```rust
struct AtomicBool {
    v: core::cell::UnsafeCell<u8>,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:555-557`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L555-L557)*

A boolean type which can be safely shared between threads.

This type has the same in-memory representation as a `bool`.

If the compiler and the platform support atomic loads and stores of `u8`,
this type is a wrapper for the standard library's
[`AtomicBool`](core::sync::atomic::AtomicBool). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

#### Implementations

- <span id="atomicbool-new"></span>`const fn new(v: bool) -> Self`

- <span id="atomicbool-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut bool) -> &'a Self`

- <span id="atomicbool-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicbool-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicbool-get-mut"></span>`const fn get_mut(&mut self) -> &mut bool`

- <span id="atomicbool-into-inner"></span>`const fn into_inner(self) -> bool`

- <span id="atomicbool-load"></span>`fn load(&self, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicbool-store"></span>`fn store(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicbool-swap"></span>`fn swap(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicbool-compare-exchange"></span>`fn compare_exchange(&self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool>` — [`Ordering`](#ordering)

- <span id="atomicbool-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool>` — [`Ordering`](#ordering)

- <span id="atomicbool-fetch-and"></span>`fn fetch_and(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicbool-and"></span>`fn and(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicbool-fetch-nand"></span>`fn fetch_nand(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicbool-fetch-or"></span>`fn fetch_or(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicbool-or"></span>`fn or(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicbool-fetch-xor"></span>`fn fetch_xor(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicbool-xor"></span>`fn xor(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicbool-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicbool-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicbool-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<bool, bool>` — [`Ordering`](#ordering)

- <span id="atomicbool-as-ptr"></span>`const fn as_ptr(&self) -> *mut bool`

- <span id="atomicbool-as-atomic-u8"></span>`fn as_atomic_u8(&self) -> &self::core_atomic::AtomicU8` — [`AtomicU8`](imp/core_atomic/index.md#atomicu8)

#### Trait Implementations

##### `impl Debug for AtomicBool`

- <span id="atomicbool-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicBool`

- <span id="atomicbool-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicBool`

##### `impl Sync for AtomicBool`

### `AtomicPtr<T>`

```rust
struct AtomicPtr<T> {
    inner: self::core_atomic::AtomicPtr<T>,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:1594-1596`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L1594-L1596)*

A raw pointer type which can be safely shared between threads.

This type has the same in-memory representation as a `*mut T`.

If the compiler and the platform support atomic loads and stores of pointers,
this type is a wrapper for the standard library's
[`AtomicPtr`](core::sync::atomic::AtomicPtr). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

#### Implementations

- <span id="atomicptr-new"></span>`const fn new(p: *mut T) -> Self`

- <span id="atomicptr-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut *mut T) -> &'a Self`

- <span id="atomicptr-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicptr-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicptr-get-mut"></span>`const fn get_mut(&mut self) -> &mut *mut T`

- <span id="atomicptr-into-inner"></span>`const fn into_inner(self) -> *mut T`

- <span id="atomicptr-load"></span>`fn load(&self, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-store"></span>`fn store(&self, ptr: *mut T, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicptr-swap"></span>`fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-compare-exchange"></span>`fn compare_exchange(&self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>` — [`Ordering`](#ordering)

- <span id="atomicptr-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<*mut T, *mut T>` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-ptr-add"></span>`fn fetch_ptr_add(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-ptr-sub"></span>`fn fetch_ptr_sub(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-byte-add"></span>`fn fetch_byte_add(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-byte-sub"></span>`fn fetch_byte_sub(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-or"></span>`fn fetch_or(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-and"></span>`fn fetch_and(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-fetch-xor"></span>`fn fetch_xor(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

- <span id="atomicptr-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicptr-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicptr-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicptr-as-atomic-usize"></span>`fn as_atomic_usize(&self) -> &AtomicUsize` — [`AtomicUsize`](#atomicusize)

- <span id="atomicptr-as-ptr"></span>`const fn as_ptr(&self) -> *mut *mut T`

#### Trait Implementations

##### `impl<T> Debug for AtomicPtr<T>`

- <span id="atomicptr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for AtomicPtr<T>`

- <span id="atomicptr-default"></span>`fn default() -> Self`

##### `impl<T> Pointer for AtomicPtr<T>`

- <span id="atomicptr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> RefUnwindSafe for AtomicPtr<T>`

### `AtomicIsize`

```rust
struct AtomicIsize {
    inner: self::core_atomic::AtomicIsize,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4786`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4786)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`isize`.

If the compiler and the platform support atomic loads and stores of `isize`, this type is a wrapper for the standard library's `AtomicIsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicIsize::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicisize-new"></span>`const fn new(v: isize) -> Self`

- <span id="atomicisize-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut isize) -> &'a Self`

- <span id="atomicisize-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicisize-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicisize-get-mut"></span>`const fn get_mut(&mut self) -> &mut isize`

- <span id="atomicisize-into-inner"></span>`const fn into_inner(self) -> isize`

- <span id="atomicisize-load"></span>`fn load(&self, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-store"></span>`fn store(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-swap"></span>`fn swap(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-compare-exchange"></span>`fn compare_exchange(&self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>` — [`Ordering`](#ordering)

- <span id="atomicisize-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-add"></span>`fn fetch_add(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-add"></span>`fn add(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-sub"></span>`fn fetch_sub(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-sub"></span>`fn sub(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-and"></span>`fn fetch_and(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-and"></span>`fn and(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-nand"></span>`fn fetch_nand(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-or"></span>`fn fetch_or(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-or"></span>`fn or(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-xor"></span>`fn fetch_xor(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-xor"></span>`fn xor(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<isize, isize>` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-max"></span>`fn fetch_max(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-min"></span>`fn fetch_min(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicisize-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicisize-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> isize` — [`Ordering`](#ordering)

- <span id="atomicisize-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicisize-as-ptr"></span>`const fn as_ptr(&self) -> *mut isize`

#### Trait Implementations

##### `impl Debug for AtomicIsize`

- <span id="atomicisize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicIsize`

- <span id="atomicisize-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicIsize`

### `AtomicUsize`

```rust
struct AtomicUsize {
    inner: self::core_atomic::AtomicUsize,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4788`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4788)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`usize`.

If the compiler and the platform support atomic loads and stores of `usize`, this type is a wrapper for the standard library's `AtomicUsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicUsize::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicusize-new"></span>`const fn new(v: usize) -> Self`

- <span id="atomicusize-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut usize) -> &'a Self`

- <span id="atomicusize-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicusize-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicusize-get-mut"></span>`const fn get_mut(&mut self) -> &mut usize`

- <span id="atomicusize-into-inner"></span>`const fn into_inner(self) -> usize`

- <span id="atomicusize-load"></span>`fn load(&self, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-store"></span>`fn store(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-swap"></span>`fn swap(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-compare-exchange"></span>`fn compare_exchange(&self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>` — [`Ordering`](#ordering)

- <span id="atomicusize-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-add"></span>`fn fetch_add(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-add"></span>`fn add(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-sub"></span>`fn fetch_sub(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-sub"></span>`fn sub(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-and"></span>`fn fetch_and(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-and"></span>`fn and(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-nand"></span>`fn fetch_nand(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-or"></span>`fn fetch_or(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-or"></span>`fn or(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-xor"></span>`fn fetch_xor(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-xor"></span>`fn xor(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<usize, usize>` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-max"></span>`fn fetch_max(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-min"></span>`fn fetch_min(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicusize-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicusize-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> usize` — [`Ordering`](#ordering)

- <span id="atomicusize-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicusize-as-ptr"></span>`const fn as_ptr(&self) -> *mut usize`

#### Trait Implementations

##### `impl Debug for AtomicUsize`

- <span id="atomicusize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicUsize`

- <span id="atomicusize-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicUsize`

### `AtomicI8`

```rust
struct AtomicI8 {
    inner: self::core_atomic::AtomicI8,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4796`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4796)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i8`.

If the compiler and the platform support atomic loads and stores of `i8`, this type is a wrapper for the standard library's `AtomicI8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI8::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici8-new"></span>`const fn new(v: i8) -> Self`

- <span id="atomici8-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i8) -> &'a Self`

- <span id="atomici8-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomici8-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomici8-get-mut"></span>`const fn get_mut(&mut self) -> &mut i8`

- <span id="atomici8-into-inner"></span>`const fn into_inner(self) -> i8`

- <span id="atomici8-load"></span>`fn load(&self, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-store"></span>`fn store(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-swap"></span>`fn swap(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-compare-exchange"></span>`fn compare_exchange(&self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>` — [`Ordering`](#ordering)

- <span id="atomici8-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-add"></span>`fn fetch_add(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-add"></span>`fn add(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-sub"></span>`fn fetch_sub(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-sub"></span>`fn sub(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-and"></span>`fn fetch_and(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-and"></span>`fn and(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-nand"></span>`fn fetch_nand(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-or"></span>`fn fetch_or(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-or"></span>`fn or(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-xor"></span>`fn fetch_xor(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-xor"></span>`fn xor(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i8, i8>` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-max"></span>`fn fetch_max(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-min"></span>`fn fetch_min(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici8-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici8-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i8` — [`Ordering`](#ordering)

- <span id="atomici8-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici8-as-ptr"></span>`const fn as_ptr(&self) -> *mut i8`

#### Trait Implementations

##### `impl Debug for AtomicI8`

- <span id="atomici8-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI8`

- <span id="atomici8-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI8`

### `AtomicU8`

```rust
struct AtomicU8 {
    inner: self::core_atomic::AtomicU8,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4797`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4797)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u8`.

If the compiler and the platform support atomic loads and stores of `u8`, this type is a wrapper for the standard library's `AtomicU8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU8::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu8-new"></span>`const fn new(v: u8) -> Self`

- <span id="atomicu8-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u8) -> &'a Self`

- <span id="atomicu8-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicu8-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicu8-get-mut"></span>`const fn get_mut(&mut self) -> &mut u8`

- <span id="atomicu8-into-inner"></span>`const fn into_inner(self) -> u8`

- <span id="atomicu8-load"></span>`fn load(&self, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-store"></span>`fn store(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-swap"></span>`fn swap(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-compare-exchange"></span>`fn compare_exchange(&self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](#ordering)

- <span id="atomicu8-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-add"></span>`fn fetch_add(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-add"></span>`fn add(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-sub"></span>`fn fetch_sub(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-sub"></span>`fn sub(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-and"></span>`fn fetch_and(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-and"></span>`fn and(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-nand"></span>`fn fetch_nand(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-or"></span>`fn fetch_or(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-or"></span>`fn or(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-xor"></span>`fn fetch_xor(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-xor"></span>`fn xor(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u8, u8>` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-max"></span>`fn fetch_max(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-min"></span>`fn fetch_min(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu8-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu8-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u8` — [`Ordering`](#ordering)

- <span id="atomicu8-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu8-as-ptr"></span>`const fn as_ptr(&self) -> *mut u8`

#### Trait Implementations

##### `impl Debug for AtomicU8`

- <span id="atomicu8-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU8`

- <span id="atomicu8-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU8`

### `AtomicI16`

```rust
struct AtomicI16 {
    inner: self::core_atomic::AtomicI16,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4800`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4800)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i16`.

If the compiler and the platform support atomic loads and stores of `i16`, this type is a wrapper for the standard library's `AtomicI16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI16::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici16-new"></span>`const fn new(v: i16) -> Self`

- <span id="atomici16-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i16) -> &'a Self`

- <span id="atomici16-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomici16-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomici16-get-mut"></span>`const fn get_mut(&mut self) -> &mut i16`

- <span id="atomici16-into-inner"></span>`const fn into_inner(self) -> i16`

- <span id="atomici16-load"></span>`fn load(&self, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-store"></span>`fn store(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-swap"></span>`fn swap(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-compare-exchange"></span>`fn compare_exchange(&self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` — [`Ordering`](#ordering)

- <span id="atomici16-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-add"></span>`fn fetch_add(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-add"></span>`fn add(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-sub"></span>`fn fetch_sub(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-sub"></span>`fn sub(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-and"></span>`fn fetch_and(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-and"></span>`fn and(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-nand"></span>`fn fetch_nand(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-or"></span>`fn fetch_or(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-or"></span>`fn or(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-xor"></span>`fn fetch_xor(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-xor"></span>`fn xor(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16>` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-max"></span>`fn fetch_max(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-min"></span>`fn fetch_min(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici16-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici16-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i16` — [`Ordering`](#ordering)

- <span id="atomici16-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici16-as-ptr"></span>`const fn as_ptr(&self) -> *mut i16`

#### Trait Implementations

##### `impl Debug for AtomicI16`

- <span id="atomici16-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI16`

- <span id="atomici16-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI16`

### `AtomicU16`

```rust
struct AtomicU16 {
    inner: self::core_atomic::AtomicU16,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4801-4802`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4801-L4802)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u16`](../gimli/leb128/read/index.md).

If the compiler and the platform support atomic loads and stores of [`u16`](../gimli/leb128/read/index.md), this type is a wrapper for the standard library's `AtomicU16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU16::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu16-new"></span>`const fn new(v: u16) -> Self`

- <span id="atomicu16-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u16) -> &'a Self`

- <span id="atomicu16-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicu16-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicu16-get-mut"></span>`const fn get_mut(&mut self) -> &mut u16`

- <span id="atomicu16-into-inner"></span>`const fn into_inner(self) -> u16`

- <span id="atomicu16-load"></span>`fn load(&self, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-store"></span>`fn store(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-swap"></span>`fn swap(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-compare-exchange"></span>`fn compare_exchange(&self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` — [`Ordering`](#ordering)

- <span id="atomicu16-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-add"></span>`fn fetch_add(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-add"></span>`fn add(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-sub"></span>`fn fetch_sub(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-sub"></span>`fn sub(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-and"></span>`fn fetch_and(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-and"></span>`fn and(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-nand"></span>`fn fetch_nand(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-or"></span>`fn fetch_or(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-or"></span>`fn or(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-xor"></span>`fn fetch_xor(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-xor"></span>`fn xor(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16>` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-max"></span>`fn fetch_max(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-min"></span>`fn fetch_min(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu16-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu16-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u16` — [`Ordering`](#ordering)

- <span id="atomicu16-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu16-as-ptr"></span>`const fn as_ptr(&self) -> *mut u16`

#### Trait Implementations

##### `impl Debug for AtomicU16`

- <span id="atomicu16-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU16`

- <span id="atomicu16-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU16`

### `AtomicI32`

```rust
struct AtomicI32 {
    inner: self::core_atomic::AtomicI32,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4805`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4805)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i32`.

If the compiler and the platform support atomic loads and stores of `i32`, this type is a wrapper for the standard library's `AtomicI32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI32::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici32-new"></span>`const fn new(v: i32) -> Self`

- <span id="atomici32-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i32) -> &'a Self`

- <span id="atomici32-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomici32-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomici32-get-mut"></span>`const fn get_mut(&mut self) -> &mut i32`

- <span id="atomici32-into-inner"></span>`const fn into_inner(self) -> i32`

- <span id="atomici32-load"></span>`fn load(&self, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-store"></span>`fn store(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-swap"></span>`fn swap(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-compare-exchange"></span>`fn compare_exchange(&self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` — [`Ordering`](#ordering)

- <span id="atomici32-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-add"></span>`fn fetch_add(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-add"></span>`fn add(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-sub"></span>`fn fetch_sub(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-sub"></span>`fn sub(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-and"></span>`fn fetch_and(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-and"></span>`fn and(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-nand"></span>`fn fetch_nand(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-or"></span>`fn fetch_or(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-or"></span>`fn or(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-xor"></span>`fn fetch_xor(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-xor"></span>`fn xor(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32>` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-max"></span>`fn fetch_max(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-min"></span>`fn fetch_min(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici32-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici32-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i32` — [`Ordering`](#ordering)

- <span id="atomici32-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici32-as-ptr"></span>`const fn as_ptr(&self) -> *mut i32`

#### Trait Implementations

##### `impl Debug for AtomicI32`

- <span id="atomici32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI32`

- <span id="atomici32-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI32`

### `AtomicU32`

```rust
struct AtomicU32 {
    inner: self::core_atomic::AtomicU32,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4806-4807`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4806-L4807)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u32`.

If the compiler and the platform support atomic loads and stores of `u32`, this type is a wrapper for the standard library's `AtomicU32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU32::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu32-new"></span>`const fn new(v: u32) -> Self`

- <span id="atomicu32-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u32) -> &'a Self`

- <span id="atomicu32-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicu32-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicu32-get-mut"></span>`const fn get_mut(&mut self) -> &mut u32`

- <span id="atomicu32-into-inner"></span>`const fn into_inner(self) -> u32`

- <span id="atomicu32-load"></span>`fn load(&self, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-store"></span>`fn store(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-swap"></span>`fn swap(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-compare-exchange"></span>`fn compare_exchange(&self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` — [`Ordering`](#ordering)

- <span id="atomicu32-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-add"></span>`fn fetch_add(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-add"></span>`fn add(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-sub"></span>`fn fetch_sub(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-sub"></span>`fn sub(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-and"></span>`fn fetch_and(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-and"></span>`fn and(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-nand"></span>`fn fetch_nand(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-or"></span>`fn fetch_or(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-or"></span>`fn or(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-xor"></span>`fn fetch_xor(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-xor"></span>`fn xor(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32>` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-max"></span>`fn fetch_max(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-min"></span>`fn fetch_min(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu32-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu32-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u32` — [`Ordering`](#ordering)

- <span id="atomicu32-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu32-as-ptr"></span>`const fn as_ptr(&self) -> *mut u32`

#### Trait Implementations

##### `impl Debug for AtomicU32`

- <span id="atomicu32-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU32`

- <span id="atomicu32-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU32`

### `AtomicI64`

```rust
struct AtomicI64 {
    inner: self::core_atomic::AtomicI64,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4810`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4810)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i64`.

If the compiler and the platform support atomic loads and stores of `i64`, this type is a wrapper for the standard library's `AtomicI64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI64::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici64-new"></span>`const fn new(v: i64) -> Self`

- <span id="atomici64-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i64) -> &'a Self`

- <span id="atomici64-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomici64-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomici64-get-mut"></span>`const fn get_mut(&mut self) -> &mut i64`

- <span id="atomici64-into-inner"></span>`const fn into_inner(self) -> i64`

- <span id="atomici64-load"></span>`fn load(&self, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-store"></span>`fn store(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-swap"></span>`fn swap(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-compare-exchange"></span>`fn compare_exchange(&self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` — [`Ordering`](#ordering)

- <span id="atomici64-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-add"></span>`fn fetch_add(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-add"></span>`fn add(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-sub"></span>`fn fetch_sub(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-sub"></span>`fn sub(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-and"></span>`fn fetch_and(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-and"></span>`fn and(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-nand"></span>`fn fetch_nand(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-or"></span>`fn fetch_or(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-or"></span>`fn or(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-xor"></span>`fn fetch_xor(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-xor"></span>`fn xor(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64>` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-max"></span>`fn fetch_max(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-min"></span>`fn fetch_min(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici64-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici64-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i64` — [`Ordering`](#ordering)

- <span id="atomici64-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici64-as-ptr"></span>`const fn as_ptr(&self) -> *mut i64`

#### Trait Implementations

##### `impl Debug for AtomicI64`

- <span id="atomici64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI64`

- <span id="atomici64-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI64`

### `AtomicU64`

```rust
struct AtomicU64 {
    inner: self::core_atomic::AtomicU64,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4811-4812`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4811-L4812)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u64`.

If the compiler and the platform support atomic loads and stores of `u64`, this type is a wrapper for the standard library's `AtomicU64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU64::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu64-new"></span>`const fn new(v: u64) -> Self`

- <span id="atomicu64-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u64) -> &'a Self`

- <span id="atomicu64-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicu64-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicu64-get-mut"></span>`const fn get_mut(&mut self) -> &mut u64`

- <span id="atomicu64-into-inner"></span>`const fn into_inner(self) -> u64`

- <span id="atomicu64-load"></span>`fn load(&self, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-store"></span>`fn store(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-swap"></span>`fn swap(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-compare-exchange"></span>`fn compare_exchange(&self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` — [`Ordering`](#ordering)

- <span id="atomicu64-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-add"></span>`fn fetch_add(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-add"></span>`fn add(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-sub"></span>`fn fetch_sub(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-sub"></span>`fn sub(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-and"></span>`fn fetch_and(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-and"></span>`fn and(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-nand"></span>`fn fetch_nand(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-or"></span>`fn fetch_or(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-or"></span>`fn or(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-xor"></span>`fn fetch_xor(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-xor"></span>`fn xor(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64>` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-max"></span>`fn fetch_max(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-min"></span>`fn fetch_min(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu64-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu64-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u64` — [`Ordering`](#ordering)

- <span id="atomicu64-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu64-as-ptr"></span>`const fn as_ptr(&self) -> *mut u64`

#### Trait Implementations

##### `impl Debug for AtomicU64`

- <span id="atomicu64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU64`

- <span id="atomicu64-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU64`

### `AtomicI128`

```rust
struct AtomicI128 {
    inner: self::atomic128::x86_64::AtomicI128,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4815`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4815)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i128`.

If the compiler and the platform support atomic loads and stores of `i128`, this type is a wrapper for the standard library's `AtomicI128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI128::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici128-new"></span>`const fn new(v: i128) -> Self`

- <span id="atomici128-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i128) -> &'a Self`

- <span id="atomici128-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomici128-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomici128-get-mut"></span>`const fn get_mut(&mut self) -> &mut i128`

- <span id="atomici128-into-inner"></span>`const fn into_inner(self) -> i128`

- <span id="atomici128-load"></span>`fn load(&self, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-store"></span>`fn store(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-swap"></span>`fn swap(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-compare-exchange"></span>`fn compare_exchange(&self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](#ordering)

- <span id="atomici128-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-add"></span>`fn fetch_add(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-add"></span>`fn add(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-sub"></span>`fn fetch_sub(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-sub"></span>`fn sub(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-and"></span>`fn fetch_and(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-and"></span>`fn and(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-nand"></span>`fn fetch_nand(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-or"></span>`fn fetch_or(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-or"></span>`fn or(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-xor"></span>`fn fetch_xor(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-xor"></span>`fn xor(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i128, i128>` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-max"></span>`fn fetch_max(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-min"></span>`fn fetch_min(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici128-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici128-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i128` — [`Ordering`](#ordering)

- <span id="atomici128-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomici128-as-ptr"></span>`const fn as_ptr(&self) -> *mut i128`

#### Trait Implementations

##### `impl Debug for AtomicI128`

- <span id="atomici128-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI128`

- <span id="atomici128-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    inner: self::atomic128::x86_64::AtomicU128,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4816-4817`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L4816-L4817)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u128`.

If the compiler and the platform support atomic loads and stores of `u128`, this type is a wrapper for the standard library's `AtomicU128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU128::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu128-new"></span>`const fn new(v: u128) -> Self`

- <span id="atomicu128-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u128) -> &'a Self`

- <span id="atomicu128-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicu128-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

- <span id="atomicu128-get-mut"></span>`const fn get_mut(&mut self) -> &mut u128`

- <span id="atomicu128-into-inner"></span>`const fn into_inner(self) -> u128`

- <span id="atomicu128-load"></span>`fn load(&self, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-store"></span>`fn store(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-swap"></span>`fn swap(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-compare-exchange"></span>`fn compare_exchange(&self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](#ordering)

- <span id="atomicu128-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-add"></span>`fn fetch_add(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-add"></span>`fn add(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-sub"></span>`fn fetch_sub(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-sub"></span>`fn sub(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-and"></span>`fn fetch_and(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-and"></span>`fn and(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-nand"></span>`fn fetch_nand(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-or"></span>`fn fetch_or(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-or"></span>`fn or(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-xor"></span>`fn fetch_xor(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-xor"></span>`fn xor(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u128, u128>` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-max"></span>`fn fetch_max(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-min"></span>`fn fetch_min(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu128-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu128-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u128` — [`Ordering`](#ordering)

- <span id="atomicu128-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

- <span id="atomicu128-as-ptr"></span>`const fn as_ptr(&self) -> *mut u128`

#### Trait Implementations

##### `impl Debug for AtomicU128`

- <span id="atomicu128-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU128`

- <span id="atomicu128-default"></span>`fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU128`

## Functions

*Defined in [`portable-atomic-1.11.1/src/lib.rs:483`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L483)*

## Macros

### `unnamed!`

*Defined in [`portable-atomic-1.11.1/src/lib.rs:471`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L471)*

### `unnamed!`

*Defined in [`portable-atomic-1.11.1/src/lib.rs:471`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L471)*

### `atomic_int!`

*Defined in [`portable-atomic-1.11.1/src/lib.rs:2720-4774`](../../.source_1765210505/portable-atomic-1.11.1/src/lib.rs#L2720-L4774)*

### `cfg_has_atomic_8!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:18-22`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L18-L22)*

### `cfg_no_atomic_8!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:24-26`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L24-L26)*

### `cfg_has_atomic_16!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:28-32`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L28-L32)*

### `cfg_no_atomic_16!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:34-36`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L34-L36)*

### `cfg_has_atomic_32!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:88-92`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L88-L92)*

### `cfg_no_atomic_32!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:94-96`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L94-L96)*

### `cfg_has_atomic_64!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:174-178`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L174-L178)*

### `cfg_no_atomic_64!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:180-182`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L180-L182)*

### `cfg_has_atomic_128!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:323-327`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L323-L327)*

### `cfg_no_atomic_128!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:329-331`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L329-L331)*

### `cfg_has_atomic_cas!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:446-450`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L446-L450)*

### `cfg_no_atomic_cas!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:452-454`](../../.source_1765210505/portable-atomic-1.11.1/src/cfgs.rs#L452-L454)*

