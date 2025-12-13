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
  - The MSRV when this feature is enabled depends on the MSRV of [`serde`](../semver/serde/index.md).

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
  - [`cfg_has_atomic_ptr!`](#cfg-has-atomic-ptr)
  - [`cfg_no_atomic_ptr!`](#cfg-no-atomic-ptr)
  - [`atomic_int!`](#atomic-int)
  - [`cfg_has_atomic_8!`](#cfg-has-atomic-8)
  - [`cfg_no_atomic_8!`](#cfg-no-atomic-8)
  - [`cfg_has_atomic_16!`](#cfg-has-atomic-16)
  - [`cfg_no_atomic_16!`](#cfg-no-atomic-16)
  - [`cfg_has_atomic_32!`](#cfg-has-atomic-32)
  - [`cfg_no_atomic_32!`](#cfg-no-atomic-32)
  - [`cfg_has_atomic_64!`](#cfg-has-atomic-64)
  - [`cfg_no_atomic_64!`](#cfg-no-atomic-64)
  - [`cfg_has_atomic_128!`](#cfg-has-atomic-128)
  - [`cfg_no_atomic_128!`](#cfg-no-atomic-128)
  - [`cfg_has_atomic_cas!`](#cfg-has-atomic-cas)
  - [`cfg_no_atomic_cas!`](#cfg-no-atomic-cas)

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
| [`cfg_has_atomic_ptr!`](#cfg-has-atomic-ptr) | macro |  |
| [`cfg_no_atomic_ptr!`](#cfg-no-atomic-ptr) | macro |  |
| [`atomic_int!`](#atomic-int) | macro |  |
| [`cfg_has_atomic_8!`](#cfg-has-atomic-8) | macro |  |
| [`cfg_no_atomic_8!`](#cfg-no-atomic-8) | macro |  |
| [`cfg_has_atomic_16!`](#cfg-has-atomic-16) | macro |  |
| [`cfg_no_atomic_16!`](#cfg-no-atomic-16) | macro |  |
| [`cfg_has_atomic_32!`](#cfg-has-atomic-32) | macro |  |
| [`cfg_no_atomic_32!`](#cfg-no-atomic-32) | macro |  |
| [`cfg_has_atomic_64!`](#cfg-has-atomic-64) | macro |  |
| [`cfg_no_atomic_64!`](#cfg-no-atomic-64) | macro |  |
| [`cfg_has_atomic_128!`](#cfg-has-atomic-128) | macro |  |
| [`cfg_no_atomic_128!`](#cfg-no-atomic-128) | macro |  |
| [`cfg_has_atomic_cas!`](#cfg-has-atomic-cas) | macro |  |
| [`cfg_no_atomic_cas!`](#cfg-no-atomic-cas) | macro |  |

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

*Defined in [`portable-atomic-1.11.1/src/lib.rs:555-557`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L555-L557)*

A boolean type which can be safely shared between threads.

This type has the same in-memory representation as a `bool`.

If the compiler and the platform support atomic loads and stores of `u8`,
this type is a wrapper for the standard library's
[`AtomicBool`](core::sync::atomic::AtomicBool). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

#### Implementations

- <span id="atomicbool-new"></span>`const fn new(v: bool) -> Self`

  Creates a new `AtomicBool`.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicBool;

  

  let atomic_true = AtomicBool::new(true);

  let atomic_false = AtomicBool::new(false);

  ```

- <span id="atomicbool-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut bool) -> &'a Self`

  Creates a new `AtomicBool` from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicBool>()` (note that on some platforms this can

    be bigger than `align_of::<bool>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via the returned

    value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not overlap

      with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically for the

      duration of lifetime `'a`. Most use cases should be able to follow this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are done

      from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic accesses, as

    these are not supported by the memory model.

- <span id="atomicbool-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicBool;

  

  let is_lock_free = AtomicBool::is_lock_free();

  ```

- <span id="atomicbool-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicBool;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicBool::is_always_lock_free();

  ```

- <span id="atomicbool-get-mut"></span>`const fn get_mut(&mut self) -> &mut bool`

  Returns a mutable reference to the underlying `bool`.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let mut some_bool = AtomicBool::new(true);

  assert_eq!(*some_bool.get_mut(), true);

  *some_bool.get_mut() = false;

  assert_eq!(some_bool.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-into-inner"></span>`const fn into_inner(self) -> bool`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicBool;

  

  let some_bool = AtomicBool::new(true);

  assert_eq!(some_bool.into_inner(), true);

  ```

- <span id="atomicbool-load"></span>`fn load(&self, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Loads a value from the bool.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let some_bool = AtomicBool::new(true);

  

  assert_eq!(some_bool.load(Ordering::Relaxed), true);

  ```

- <span id="atomicbool-store"></span>`fn store(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the bool.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let some_bool = AtomicBool::new(true);

  

  some_bool.store(false, Ordering::Relaxed);

  assert_eq!(some_bool.load(Ordering::Relaxed), false);

  ```

- <span id="atomicbool-swap"></span>`fn swap(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Stores a value into the bool, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let some_bool = AtomicBool::new(true);

  

  assert_eq!(some_bool.swap(false, Ordering::Relaxed), true);

  assert_eq!(some_bool.load(Ordering::Relaxed), false);

  ```

- <span id="atomicbool-compare-exchange"></span>`fn compare_exchange(&self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool>` — [`Ordering`](#ordering)

  Stores a value into the `bool` if the current value is the same as the `current` value.

  

  The return value is a result indicating whether the new value was written and containing

  the previous value. On success this value is guaranteed to be equal to `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let some_bool = AtomicBool::new(true);

  

  assert_eq!(

      some_bool.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed),

      Ok(true)

  );

  assert_eq!(some_bool.load(Ordering::Relaxed), false);

  

  assert_eq!(

      some_bool.compare_exchange(true, true, Ordering::SeqCst, Ordering::Acquire),

      Err(false)

  );

  assert_eq!(some_bool.load(Ordering::Relaxed), false);

  ```

- <span id="atomicbool-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool>` — [`Ordering`](#ordering)

  Stores a value into the `bool` if the current value is the same as the `current` value.

  

  Unlike `AtomicBool::compare_exchange`, this function is allowed to spuriously fail even when the

  comparison succeeds, which can result in more efficient code on some platforms. The

  return value is a result indicating whether the new value was written and containing the

  previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let val = AtomicBool::new(false);

  

  let new = true;

  let mut old = val.load(Ordering::Relaxed);

  loop {

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicbool-fetch-and"></span>`fn fetch_and(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Logical "and" with a boolean value.

  

  Performs a logical "and" operation on the current value and the argument `val`, and sets

  the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_and(false, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_and(true, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(false);

  assert_eq!(foo.fetch_and(false, Ordering::SeqCst), false);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-and"></span>`fn and(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

  Logical "and" with a boolean value.

  

  Performs a logical "and" operation on the current value and the argument `val`, and sets

  the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop

  - MSP430: `and` instead of disabling interrupts

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  foo.and(false, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  

  let foo = AtomicBool::new(true);

  foo.and(true, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(false);

  foo.and(false, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-fetch-nand"></span>`fn fetch_nand(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Logical "nand" with a boolean value.

  

  Performs a logical "nand" operation on the current value and the argument `val`, and sets

  the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_nand(false, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_nand(true, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst) as usize, 0);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  

  let foo = AtomicBool::new(false);

  assert_eq!(foo.fetch_nand(false, Ordering::SeqCst), false);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  ```

- <span id="atomicbool-fetch-or"></span>`fn fetch_or(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Logical "or" with a boolean value.

  

  Performs a logical "or" operation on the current value and the argument `val`, and sets the

  new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_or(false, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_or(true, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(false);

  assert_eq!(foo.fetch_or(false, Ordering::SeqCst), false);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-or"></span>`fn or(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

  Logical "or" with a boolean value.

  

  Performs a logical "or" operation on the current value and the argument `val`, and sets the

  new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop

  - MSP430: `bis` instead of disabling interrupts

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  foo.or(false, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(true);

  foo.or(true, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(false);

  foo.or(false, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-fetch-xor"></span>`fn fetch_xor(&self, val: bool, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Logical "xor" with a boolean value.

  

  Performs a logical "xor" operation on the current value and the argument `val`, and sets

  the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_xor(false, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_xor(true, Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  

  let foo = AtomicBool::new(false);

  assert_eq!(foo.fetch_xor(false, Ordering::SeqCst), false);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-xor"></span>`fn xor(&self, val: bool, order: Ordering)` — [`Ordering`](#ordering)

  Logical "xor" with a boolean value.

  

  Performs a logical "xor" operation on the current value and the argument `val`, and sets

  the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop

  - MSP430: `xor` instead of disabling interrupts

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  foo.xor(false, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  

  let foo = AtomicBool::new(true);

  foo.xor(true, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  

  let foo = AtomicBool::new(false);

  foo.xor(false, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Logical "not" with a boolean value.

  

  Performs a logical "not" operation on the current value, and sets

  the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  assert_eq!(foo.fetch_not(Ordering::SeqCst), true);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  

  let foo = AtomicBool::new(false);

  assert_eq!(foo.fetch_not(Ordering::SeqCst), false);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  ```

- <span id="atomicbool-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical "not" with a boolean value.

  

  Performs a logical "not" operation on the current value, and sets

  the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop

  - MSP430: `xor` instead of disabling interrupts

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let foo = AtomicBool::new(true);

  foo.not(Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), false);

  

  let foo = AtomicBool::new(false);

  foo.not(Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), true);

  ```

- <span id="atomicbool-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<bool, bool>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function

  returned `Some(_)`, else `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been

  changed from other threads in the meantime, as long as the function

  returns `Some(_)`, but the function will have been applied only once to

  the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. The first describes the required ordering for

  when the operation finally succeeds while the second describes the

  required ordering for loads. These correspond to the success and failure

  orderings of [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part of this

  operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful

  load `Relaxed`. The (failed) load ordering can only be `SeqCst`,

  `Acquire` or `Relaxed`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicBool, Ordering};

  

  let x = AtomicBool::new(false);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(false));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(!x)), Ok(false));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(!x)), Ok(true));

  assert_eq!(x.load(Ordering::SeqCst), false);

  ```

- <span id="atomicbool-as-ptr"></span>`const fn as_ptr(&self) -> *mut bool`

  Returns a mutable pointer to the underlying `bool`.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

- <span id="atomicbool-as-atomic-u8"></span>`fn as_atomic_u8(&self) -> &self::core_atomic::AtomicU8` — [`AtomicU8`](imp/core_atomic/index.md#atomicu8)

#### Trait Implementations

##### `impl Any for AtomicBool`

- <span id="atomicbool-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicBool`

- <span id="atomicbool-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicBool`

- <span id="atomicbool-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicBool`

- <span id="atomicbool-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicBool`

- <span id="atomicbool-default"></span>`fn default() -> Self`

  Creates an `AtomicBool` initialized to `false`.

##### `impl<T> From for AtomicBool`

- <span id="atomicbool-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicBool`

- <span id="atomicbool-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicBool`

##### `impl Sync for AtomicBool`

##### `impl<U> TryFrom for AtomicBool`

- <span id="atomicbool-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicbool-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicBool`

- <span id="atomicbool-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicbool-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicPtr<T>`

```rust
struct AtomicPtr<T> {
    inner: self::core_atomic::AtomicPtr<T>,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:1594-1596`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L1594-L1596)*

A raw pointer type which can be safely shared between threads.

This type has the same in-memory representation as a `*mut T`.

If the compiler and the platform support atomic loads and stores of pointers,
this type is a wrapper for the standard library's
[`AtomicPtr`](core::sync::atomic::AtomicPtr). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

#### Implementations

- <span id="atomicptr-new"></span>`const fn new(p: *mut T) -> Self`

  Creates a new `AtomicPtr`.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicPtr;

  

  let ptr = &mut 5;

  let atomic_ptr = AtomicPtr::new(ptr);

  ```

- <span id="atomicptr-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut *mut T) -> &'a Self`

  Creates a new `AtomicPtr` from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicPtr<T>>()` (note that on some platforms this

    can be bigger than `align_of::<*mut T>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via the returned

    value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not overlap

      with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically for the

      duration of lifetime `'a`. Most use cases should be able to follow this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are done

      from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic accesses, as

    these are not supported by the memory model.

- <span id="atomicptr-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicPtr;

  

  let is_lock_free = AtomicPtr::<()>::is_lock_free();

  ```

- <span id="atomicptr-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicPtr;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicPtr::<()>::is_always_lock_free();

  ```

- <span id="atomicptr-get-mut"></span>`const fn get_mut(&mut self) -> &mut *mut T`

  Returns a mutable reference to the underlying pointer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let mut data = 10;

  let mut atomic_ptr = AtomicPtr::new(&mut data);

  let mut other_data = 5;

  *atomic_ptr.get_mut() = &mut other_data;

  assert_eq!(unsafe { *atomic_ptr.load(Ordering::SeqCst) }, 5);

  ```

- <span id="atomicptr-into-inner"></span>`const fn into_inner(self) -> *mut T`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicPtr;

  

  let mut data = 5;

  let atomic_ptr = AtomicPtr::new(&mut data);

  assert_eq!(unsafe { *atomic_ptr.into_inner() }, 5);

  ```

- <span id="atomicptr-load"></span>`fn load(&self, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Loads a value from the pointer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let ptr = &mut 5;

  let some_ptr = AtomicPtr::new(ptr);

  

  let value = some_ptr.load(Ordering::Relaxed);

  ```

- <span id="atomicptr-store"></span>`fn store(&self, ptr: *mut T, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the pointer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let ptr = &mut 5;

  let some_ptr = AtomicPtr::new(ptr);

  

  let other_ptr = &mut 10;

  

  some_ptr.store(other_ptr, Ordering::Relaxed);

  ```

- <span id="atomicptr-swap"></span>`fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Stores a value into the pointer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let ptr = &mut 5;

  let some_ptr = AtomicPtr::new(ptr);

  

  let other_ptr = &mut 10;

  

  let value = some_ptr.swap(other_ptr, Ordering::Relaxed);

  ```

- <span id="atomicptr-compare-exchange"></span>`fn compare_exchange(&self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>` — [`Ordering`](#ordering)

  Stores a value into the pointer if the current value is the same as the `current` value.

  

  The return value is a result indicating whether the new value was written and containing

  the previous value. On success this value is guaranteed to be equal to `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let ptr = &mut 5;

  let some_ptr = AtomicPtr::new(ptr);

  

  let other_ptr = &mut 10;

  

  let value = some_ptr.compare_exchange(ptr, other_ptr, Ordering::SeqCst, Ordering::Relaxed);

  ```

- <span id="atomicptr-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>` — [`Ordering`](#ordering)

  Stores a value into the pointer if the current value is the same as the `current` value.

  

  Unlike `AtomicPtr::compare_exchange`, this function is allowed to spuriously fail even when the

  comparison succeeds, which can result in more efficient code on some platforms. The

  return value is a result indicating whether the new value was written and containing the

  previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let some_ptr = AtomicPtr::new(&mut 5);

  

  let new = &mut 10;

  let mut old = some_ptr.load(Ordering::Relaxed);

  loop {

      match some_ptr.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicptr-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<*mut T, *mut T>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function

  returned `Some(_)`, else `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been

  changed from other threads in the meantime, as long as the function

  returns `Some(_)`, but the function will have been applied only once to

  the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. The first describes the required ordering for

  when the operation finally succeeds while the second describes the

  required ordering for loads. These correspond to the success and failure

  orderings of [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part of this

  operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful

  load `Relaxed`. The (failed) load ordering can only be `SeqCst`,

  `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let ptr: *mut _ = &mut 5;

  let some_ptr = AtomicPtr::new(ptr);

  

  let new: *mut _ = &mut 10;

  assert_eq!(some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(ptr));

  let result = some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {

      if x == ptr {

          Some(new)

      } else {

          None

      }

  });

  assert_eq!(result, Ok(ptr));

  assert_eq!(some_ptr.load(Ordering::SeqCst), new);

  ```

- <span id="atomicptr-fetch-ptr-add"></span>`fn fetch_ptr_add(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Offsets the pointer's address by adding `val` (in units of `T`),

  returning the previous pointer.

  

  This is equivalent to using `wrapping_add` to atomically perform the

  equivalent of `ptr = ptr.wrapping_add(val);`.

  

  This method operates in units of `T`, which means that it cannot be used

  to offset the pointer by an amount which is not a multiple of

  `size_of::<T>()`. This can sometimes be inconvenient, as you may want to

  work with a deliberately misaligned pointer. In such cases, you may use

  the [`fetch_byte_add`](Self::fetch_byte_add) method instead.

  

  `fetch_ptr_add` takes an [`Ordering`](#ordering) argument which describes the

  memory ordering of this operation. All ordering modes are possible. Note

  that using `Acquire` makes the store part of this operation

  `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let atom = AtomicPtr::<i64>::new(core::ptr::null_mut());

  assert_eq!(atom.fetch_ptr_add(1, Ordering::Relaxed).addr(), 0);

  // Note: units of `size_of::<i64>()`.

  assert_eq!(atom.load(Ordering::Relaxed).addr(), 8);

  ```

- <span id="atomicptr-fetch-ptr-sub"></span>`fn fetch_ptr_sub(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Offsets the pointer's address by subtracting `val` (in units of `T`),

  returning the previous pointer.

  

  This is equivalent to using `wrapping_sub` to atomically perform the

  equivalent of `ptr = ptr.wrapping_sub(val);`.

  

  This method operates in units of `T`, which means that it cannot be used

  to offset the pointer by an amount which is not a multiple of

  `size_of::<T>()`. This can sometimes be inconvenient, as you may want to

  work with a deliberately misaligned pointer. In such cases, you may use

  the [`fetch_byte_sub`](Self::fetch_byte_sub) method instead.

  

  `fetch_ptr_sub` takes an [`Ordering`](#ordering) argument which describes the memory

  ordering of this operation. All ordering modes are possible. Note that

  using `Acquire` makes the store part of this operation `Relaxed`,

  and using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicPtr, Ordering};

  

  let array = [1i32, 2i32];

  let atom = AtomicPtr::new(array.as_ptr().wrapping_add(1) as *mut _);

  

  assert!(core::ptr::eq(atom.fetch_ptr_sub(1, Ordering::Relaxed), &array[1]));

  assert!(core::ptr::eq(atom.load(Ordering::Relaxed), &array[0]));

  ```

- <span id="atomicptr-fetch-byte-add"></span>`fn fetch_byte_add(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Offsets the pointer's address by adding `val` *bytes*, returning the

  previous pointer.

  

  This is equivalent to using `wrapping_add` and [`cast`](#cast) to atomically

  perform `ptr = ptr.cast::<u8>().wrapping_add(val).cast::<T>()`.

  

  `fetch_byte_add` takes an [`Ordering`](#ordering) argument which describes the

  memory ordering of this operation. All ordering modes are possible. Note

  that using `Acquire` makes the store part of this operation

  `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let atom = AtomicPtr::<i64>::new(core::ptr::null_mut());

  assert_eq!(atom.fetch_byte_add(1, Ordering::Relaxed).addr(), 0);

  // Note: in units of bytes, not `size_of::<i64>()`.

  assert_eq!(atom.load(Ordering::Relaxed).addr(), 1);

  ```

- <span id="atomicptr-fetch-byte-sub"></span>`fn fetch_byte_sub(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Offsets the pointer's address by subtracting `val` *bytes*, returning the

  previous pointer.

  

  This is equivalent to using `wrapping_sub` and [`cast`](#cast) to atomically

  perform `ptr = ptr.cast::<u8>().wrapping_sub(val).cast::<T>()`.

  

  `fetch_byte_sub` takes an [`Ordering`](#ordering) argument which describes the

  memory ordering of this operation. All ordering modes are possible. Note

  that using `Acquire` makes the store part of this operation

  `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let atom = AtomicPtr::<i64>::new(sptr::invalid_mut(1));

  assert_eq!(atom.fetch_byte_sub(1, Ordering::Relaxed).addr(), 1);

  assert_eq!(atom.load(Ordering::Relaxed).addr(), 0);

  ```

- <span id="atomicptr-fetch-or"></span>`fn fetch_or(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Performs a bitwise "or" operation on the address of the current pointer,

  and the argument `val`, and stores a pointer with provenance of the

  current pointer and the resulting address.

  

  This is equivalent to using [`map_addr`](#map-addr) to atomically perform

  `ptr = ptr.map_addr(|a| a | val)`. This can be used in tagged

  pointer schemes to atomically set tag bits.

  

  **Caveat**: This operation returns the previous value. To compute the

  stored value without losing provenance, you may use [`map_addr`](#map-addr). For

  example: `a.fetch_or(val).map_addr(|a| a | val)`.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory

  ordering of this operation. All ordering modes are possible. Note that

  using `Acquire` makes the store part of this operation `Relaxed`,

  and using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This API and its claimed semantics are part of the Strict Provenance

  experiment, see the `module documentation for `ptr`` for

  details.

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let pointer = &mut 3i64 as *mut i64;

  

  let atom = AtomicPtr::<i64>::new(pointer);

  // Tag the bottom bit of the pointer.

  assert_eq!(atom.fetch_or(1, Ordering::Relaxed).addr() & 1, 0);

  // Extract and untag.

  let tagged = atom.load(Ordering::Relaxed);

  assert_eq!(tagged.addr() & 1, 1);

  assert_eq!(tagged.map_addr(|p| p & !1), pointer);

  ```

- <span id="atomicptr-fetch-and"></span>`fn fetch_and(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Performs a bitwise "and" operation on the address of the current

  pointer, and the argument `val`, and stores a pointer with provenance of

  the current pointer and the resulting address.

  

  This is equivalent to using [`map_addr`](#map-addr) to atomically perform

  `ptr = ptr.map_addr(|a| a & val)`. This can be used in tagged

  pointer schemes to atomically unset tag bits.

  

  **Caveat**: This operation returns the previous value. To compute the

  stored value without losing provenance, you may use [`map_addr`](#map-addr). For

  example: `a.fetch_and(val).map_addr(|a| a & val)`.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory

  ordering of this operation. All ordering modes are possible. Note that

  using `Acquire` makes the store part of this operation `Relaxed`,

  and using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This API and its claimed semantics are part of the Strict Provenance

  experiment, see the `module documentation for `ptr`` for

  details.

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let pointer = &mut 3i64 as *mut i64;

  // A tagged pointer

  let atom = AtomicPtr::<i64>::new(pointer.map_addr(|a| a | 1));

  assert_eq!(atom.fetch_or(1, Ordering::Relaxed).addr() & 1, 1);

  // Untag, and extract the previously tagged pointer.

  let untagged = atom.fetch_and(!1, Ordering::Relaxed).map_addr(|a| a & !1);

  assert_eq!(untagged, pointer);

  ```

- <span id="atomicptr-fetch-xor"></span>`fn fetch_xor(&self, val: usize, order: Ordering) -> *mut T` — [`Ordering`](#ordering)

  Performs a bitwise "xor" operation on the address of the current

  pointer, and the argument `val`, and stores a pointer with provenance of

  the current pointer and the resulting address.

  

  This is equivalent to using [`map_addr`](#map-addr) to atomically perform

  `ptr = ptr.map_addr(|a| a ^ val)`. This can be used in tagged

  pointer schemes to atomically toggle tag bits.

  

  **Caveat**: This operation returns the previous value. To compute the

  stored value without losing provenance, you may use [`map_addr`](#map-addr). For

  example: `a.fetch_xor(val).map_addr(|a| a ^ val)`.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory

  ordering of this operation. All ordering modes are possible. Note that

  using `Acquire` makes the store part of this operation `Relaxed`,

  and using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This API and its claimed semantics are part of the Strict Provenance

  experiment, see the `module documentation for `ptr`` for

  details.

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let pointer = &mut 3i64 as *mut i64;

  let atom = AtomicPtr::<i64>::new(pointer);

  

  // Toggle a tag bit on the pointer.

  atom.fetch_xor(1, Ordering::Relaxed);

  assert_eq!(atom.load(Ordering::Relaxed).addr() & 1, 1);

  ```

- <span id="atomicptr-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let pointer = &mut 3i64 as *mut i64;

  

  let atom = AtomicPtr::<i64>::new(pointer);

  // Tag the bottom bit of the pointer.

  assert!(!atom.bit_set(0, Ordering::Relaxed));

  // Extract and untag.

  let tagged = atom.load(Ordering::Relaxed);

  assert_eq!(tagged.addr() & 1, 1);

  assert_eq!(tagged.map_addr(|p| p & !1), pointer);

  ```

- <span id="atomicptr-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let pointer = &mut 3i64 as *mut i64;

  // A tagged pointer

  let atom = AtomicPtr::<i64>::new(pointer.map_addr(|a| a | 1));

  assert!(atom.bit_set(0, Ordering::Relaxed));

  // Untag

  assert!(atom.bit_clear(0, Ordering::Relaxed));

  ```

- <span id="atomicptr-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  #![allow(unstable_name_collisions)]

  #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc

  use portable_atomic::{AtomicPtr, Ordering};

  

  let pointer = &mut 3i64 as *mut i64;

  let atom = AtomicPtr::<i64>::new(pointer);

  

  // Toggle a tag bit on the pointer.

  atom.bit_toggle(0, Ordering::Relaxed);

  assert_eq!(atom.load(Ordering::Relaxed).addr() & 1, 1);

  ```

- <span id="atomicptr-as-atomic-usize"></span>`fn as_atomic_usize(&self) -> &AtomicUsize` — [`AtomicUsize`](#atomicusize)

- <span id="atomicptr-as-ptr"></span>`const fn as_ptr(&self) -> *mut *mut T`

  Returns a mutable pointer to the underlying pointer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl<T> Any for AtomicPtr<T>`

- <span id="atomicptr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicPtr<T>`

- <span id="atomicptr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicPtr<T>`

- <span id="atomicptr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for AtomicPtr<T>`

- <span id="atomicptr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for AtomicPtr<T>`

- <span id="atomicptr-default"></span>`fn default() -> Self`

  Creates a null `AtomicPtr<T>`.

##### `impl<T> From for AtomicPtr<T>`

- <span id="atomicptr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AtomicPtr<T>`

- <span id="atomicptr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointer for AtomicPtr<T>`

- <span id="atomicptr-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> RefUnwindSafe for AtomicPtr<T>`

##### `impl<T, U> TryFrom for AtomicPtr<T>`

- <span id="atomicptr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicptr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AtomicPtr<T>`

- <span id="atomicptr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicptr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicIsize`

```rust
struct AtomicIsize {
    inner: self::core_atomic::AtomicIsize,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4786`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4786)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`isize`.

If the compiler and the platform support atomic loads and stores of `isize`, this type is a wrapper for the standard library's `AtomicIsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicIsize::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicisize-new"></span>`const fn new(v: isize) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicIsize;

  

  let atomic_forty_two = AtomicIsize::new(42);

  ```

- <span id="atomicisize-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut isize) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicIsize>()` (note that on some platforms this

    can be bigger than `align_of::<isize>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomicisize-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicIsize;

  

  let is_lock_free = AtomicIsize::is_lock_free();

  ```

- <span id="atomicisize-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicIsize;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicIsize::is_always_lock_free();

  ```

- <span id="atomicisize-get-mut"></span>`const fn get_mut(&mut self) -> &mut isize`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let mut some_var = AtomicIsize::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomicisize-into-inner"></span>`const fn into_inner(self) -> isize`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicIsize;

  

  let some_var = AtomicIsize::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomicisize-load"></span>`fn load(&self, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let some_var = AtomicIsize::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicisize-store"></span>`fn store(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let some_var = AtomicIsize::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicisize-swap"></span>`fn swap(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let some_var = AtomicIsize::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomicisize-compare-exchange"></span>`fn compare_exchange(&self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let some_var = AtomicIsize::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicisize-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let val = AtomicIsize::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicisize-fetch-add"></span>`fn fetch_add(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicisize-add"></span>`fn add(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicisize-fetch-sub"></span>`fn fetch_sub(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicisize-sub"></span>`fn sub(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicisize-fetch-and"></span>`fn fetch_and(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicisize-and"></span>`fn and(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicisize-fetch-nand"></span>`fn fetch_nand(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomicisize-fetch-or"></span>`fn fetch_or(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicisize-or"></span>`fn or(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicisize-fetch-xor"></span>`fn fetch_xor(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicisize-xor"></span>`fn xor(&self, val: isize, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicisize-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<isize, isize>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let x = AtomicIsize::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomicisize-fetch-max"></span>`fn fetch_max(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomicisize-fetch-min"></span>`fn fetch_min(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomicisize-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomicisize-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicisize-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicisize-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicisize-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicisize-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> isize` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_isize.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_isize.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicisize-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicIsize, Ordering};

  

  let foo = AtomicIsize::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_isize.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicisize-as-ptr"></span>`const fn as_ptr(&self) -> *mut isize`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicIsize`

- <span id="atomicisize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicIsize`

- <span id="atomicisize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicIsize`

- <span id="atomicisize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicIsize`

- <span id="atomicisize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicIsize`

- <span id="atomicisize-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicIsize`

- <span id="atomicisize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicIsize`

- <span id="atomicisize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicIsize`

##### `impl<U> TryFrom for AtomicIsize`

- <span id="atomicisize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicisize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicIsize`

- <span id="atomicisize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicisize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicUsize`

```rust
struct AtomicUsize {
    inner: self::core_atomic::AtomicUsize,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4788`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4788)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`usize`.

If the compiler and the platform support atomic loads and stores of `usize`, this type is a wrapper for the standard library's `AtomicUsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicUsize::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicusize-new"></span>`const fn new(v: usize) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicUsize;

  

  let atomic_forty_two = AtomicUsize::new(42);

  ```

- <span id="atomicusize-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut usize) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicUsize>()` (note that on some platforms this

    can be bigger than `align_of::<usize>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomicusize-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicUsize;

  

  let is_lock_free = AtomicUsize::is_lock_free();

  ```

- <span id="atomicusize-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicUsize;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicUsize::is_always_lock_free();

  ```

- <span id="atomicusize-get-mut"></span>`const fn get_mut(&mut self) -> &mut usize`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let mut some_var = AtomicUsize::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomicusize-into-inner"></span>`const fn into_inner(self) -> usize`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicUsize;

  

  let some_var = AtomicUsize::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomicusize-load"></span>`fn load(&self, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let some_var = AtomicUsize::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicusize-store"></span>`fn store(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let some_var = AtomicUsize::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicusize-swap"></span>`fn swap(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let some_var = AtomicUsize::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomicusize-compare-exchange"></span>`fn compare_exchange(&self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let some_var = AtomicUsize::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicusize-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let val = AtomicUsize::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicusize-fetch-add"></span>`fn fetch_add(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicusize-add"></span>`fn add(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicusize-fetch-sub"></span>`fn fetch_sub(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicusize-sub"></span>`fn sub(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicusize-fetch-and"></span>`fn fetch_and(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicusize-and"></span>`fn and(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicusize-fetch-nand"></span>`fn fetch_nand(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomicusize-fetch-or"></span>`fn fetch_or(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicusize-or"></span>`fn or(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicusize-fetch-xor"></span>`fn fetch_xor(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicusize-xor"></span>`fn xor(&self, val: usize, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicusize-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<usize, usize>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let x = AtomicUsize::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomicusize-fetch-max"></span>`fn fetch_max(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomicusize-fetch-min"></span>`fn fetch_min(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomicusize-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomicusize-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicusize-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicusize-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicusize-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicusize-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> usize` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_usize.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_usize.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicusize-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicUsize, Ordering};

  

  let foo = AtomicUsize::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_usize.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicusize-as-ptr"></span>`const fn as_ptr(&self) -> *mut usize`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicUsize`

- <span id="atomicusize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicUsize`

- <span id="atomicusize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicUsize`

- <span id="atomicusize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicUsize`

- <span id="atomicusize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicUsize`

- <span id="atomicusize-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicUsize`

- <span id="atomicusize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicUsize`

- <span id="atomicusize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicUsize`

##### `impl<U> TryFrom for AtomicUsize`

- <span id="atomicusize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicusize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicUsize`

- <span id="atomicusize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicusize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI8`

```rust
struct AtomicI8 {
    inner: self::core_atomic::AtomicI8,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4796`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4796)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i8`.

If the compiler and the platform support atomic loads and stores of `i8`, this type is a wrapper for the standard library's `AtomicI8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI8::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici8-new"></span>`const fn new(v: i8) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI8;

  

  let atomic_forty_two = AtomicI8::new(42);

  ```

- <span id="atomici8-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i8) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicI8>()` (note that on some platforms this

    can be bigger than `align_of::<i8>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomici8-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI8;

  

  let is_lock_free = AtomicI8::is_lock_free();

  ```

- <span id="atomici8-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI8;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicI8::is_always_lock_free();

  ```

- <span id="atomici8-get-mut"></span>`const fn get_mut(&mut self) -> &mut i8`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let mut some_var = AtomicI8::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomici8-into-inner"></span>`const fn into_inner(self) -> i8`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI8;

  

  let some_var = AtomicI8::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomici8-load"></span>`fn load(&self, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let some_var = AtomicI8::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici8-store"></span>`fn store(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let some_var = AtomicI8::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici8-swap"></span>`fn swap(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let some_var = AtomicI8::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomici8-compare-exchange"></span>`fn compare_exchange(&self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let some_var = AtomicI8::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici8-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let val = AtomicI8::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomici8-fetch-add"></span>`fn fetch_add(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici8-add"></span>`fn add(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici8-fetch-sub"></span>`fn fetch_sub(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici8-sub"></span>`fn sub(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici8-fetch-and"></span>`fn fetch_and(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici8-and"></span>`fn and(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici8-fetch-nand"></span>`fn fetch_nand(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomici8-fetch-or"></span>`fn fetch_or(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici8-or"></span>`fn or(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici8-fetch-xor"></span>`fn fetch_xor(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici8-xor"></span>`fn xor(&self, val: i8, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici8-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i8, i8>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let x = AtomicI8::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomici8-fetch-max"></span>`fn fetch_max(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomici8-fetch-min"></span>`fn fetch_min(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomici8-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomici8-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici8-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici8-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici8-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici8-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i8` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i8.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_i8.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici8-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI8, Ordering};

  

  let foo = AtomicI8::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i8.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici8-as-ptr"></span>`const fn as_ptr(&self) -> *mut i8`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicI8`

- <span id="atomici8-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI8`

- <span id="atomici8-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI8`

- <span id="atomici8-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicI8`

- <span id="atomici8-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI8`

- <span id="atomici8-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicI8`

- <span id="atomici8-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI8`

- <span id="atomici8-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicI8`

##### `impl<U> TryFrom for AtomicI8`

- <span id="atomici8-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici8-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI8`

- <span id="atomici8-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici8-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU8`

```rust
struct AtomicU8 {
    inner: self::core_atomic::AtomicU8,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4797`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4797)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u8`.

If the compiler and the platform support atomic loads and stores of `u8`, this type is a wrapper for the standard library's `AtomicU8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU8::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu8-new"></span>`const fn new(v: u8) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU8;

  

  let atomic_forty_two = AtomicU8::new(42);

  ```

- <span id="atomicu8-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u8) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicU8>()` (note that on some platforms this

    can be bigger than `align_of::<u8>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomicu8-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU8;

  

  let is_lock_free = AtomicU8::is_lock_free();

  ```

- <span id="atomicu8-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU8;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicU8::is_always_lock_free();

  ```

- <span id="atomicu8-get-mut"></span>`const fn get_mut(&mut self) -> &mut u8`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let mut some_var = AtomicU8::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomicu8-into-inner"></span>`const fn into_inner(self) -> u8`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU8;

  

  let some_var = AtomicU8::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomicu8-load"></span>`fn load(&self, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let some_var = AtomicU8::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu8-store"></span>`fn store(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let some_var = AtomicU8::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu8-swap"></span>`fn swap(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let some_var = AtomicU8::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomicu8-compare-exchange"></span>`fn compare_exchange(&self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let some_var = AtomicU8::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu8-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let val = AtomicU8::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicu8-fetch-add"></span>`fn fetch_add(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu8-add"></span>`fn add(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu8-fetch-sub"></span>`fn fetch_sub(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu8-sub"></span>`fn sub(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu8-fetch-and"></span>`fn fetch_and(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu8-and"></span>`fn and(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu8-fetch-nand"></span>`fn fetch_nand(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomicu8-fetch-or"></span>`fn fetch_or(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu8-or"></span>`fn or(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu8-fetch-xor"></span>`fn fetch_xor(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu8-xor"></span>`fn xor(&self, val: u8, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu8-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u8, u8>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let x = AtomicU8::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomicu8-fetch-max"></span>`fn fetch_max(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomicu8-fetch-min"></span>`fn fetch_min(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomicu8-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomicu8-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu8-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu8-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu8-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu8-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u8` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u8.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_u8.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu8-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU8, Ordering};

  

  let foo = AtomicU8::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u8.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu8-as-ptr"></span>`const fn as_ptr(&self) -> *mut u8`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicU8`

- <span id="atomicu8-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU8`

- <span id="atomicu8-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU8`

- <span id="atomicu8-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicU8`

- <span id="atomicu8-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU8`

- <span id="atomicu8-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicU8`

- <span id="atomicu8-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU8`

- <span id="atomicu8-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicU8`

##### `impl<U> TryFrom for AtomicU8`

- <span id="atomicu8-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu8-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU8`

- <span id="atomicu8-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu8-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI16`

```rust
struct AtomicI16 {
    inner: self::core_atomic::AtomicI16,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4800`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4800)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i16`.

If the compiler and the platform support atomic loads and stores of `i16`, this type is a wrapper for the standard library's `AtomicI16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI16::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici16-new"></span>`const fn new(v: i16) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI16;

  

  let atomic_forty_two = AtomicI16::new(42);

  ```

- <span id="atomici16-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i16) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicI16>()` (note that on some platforms this

    can be bigger than `align_of::<i16>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomici16-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI16;

  

  let is_lock_free = AtomicI16::is_lock_free();

  ```

- <span id="atomici16-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI16;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicI16::is_always_lock_free();

  ```

- <span id="atomici16-get-mut"></span>`const fn get_mut(&mut self) -> &mut i16`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let mut some_var = AtomicI16::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomici16-into-inner"></span>`const fn into_inner(self) -> i16`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI16;

  

  let some_var = AtomicI16::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomici16-load"></span>`fn load(&self, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let some_var = AtomicI16::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici16-store"></span>`fn store(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let some_var = AtomicI16::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici16-swap"></span>`fn swap(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let some_var = AtomicI16::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomici16-compare-exchange"></span>`fn compare_exchange(&self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let some_var = AtomicI16::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici16-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let val = AtomicI16::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomici16-fetch-add"></span>`fn fetch_add(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici16-add"></span>`fn add(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici16-fetch-sub"></span>`fn fetch_sub(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici16-sub"></span>`fn sub(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici16-fetch-and"></span>`fn fetch_and(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici16-and"></span>`fn and(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici16-fetch-nand"></span>`fn fetch_nand(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomici16-fetch-or"></span>`fn fetch_or(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici16-or"></span>`fn or(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici16-fetch-xor"></span>`fn fetch_xor(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici16-xor"></span>`fn xor(&self, val: i16, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici16-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let x = AtomicI16::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomici16-fetch-max"></span>`fn fetch_max(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomici16-fetch-min"></span>`fn fetch_min(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomici16-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomici16-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici16-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici16-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici16-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici16-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i16` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i16.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_i16.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici16-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI16, Ordering};

  

  let foo = AtomicI16::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i16.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici16-as-ptr"></span>`const fn as_ptr(&self) -> *mut i16`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicI16`

- <span id="atomici16-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI16`

- <span id="atomici16-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI16`

- <span id="atomici16-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicI16`

- <span id="atomici16-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI16`

- <span id="atomici16-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicI16`

- <span id="atomici16-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI16`

- <span id="atomici16-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicI16`

##### `impl<U> TryFrom for AtomicI16`

- <span id="atomici16-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici16-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI16`

- <span id="atomici16-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici16-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU16`

```rust
struct AtomicU16 {
    inner: self::core_atomic::AtomicU16,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4801-4802`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4801-L4802)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u16`](../gimli/leb128/read/index.md).

If the compiler and the platform support atomic loads and stores of [`u16`](../gimli/leb128/read/index.md), this type is a wrapper for the standard library's `AtomicU16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU16::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu16-new"></span>`const fn new(v: u16) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU16;

  

  let atomic_forty_two = AtomicU16::new(42);

  ```

- <span id="atomicu16-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u16) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicU16>()` (note that on some platforms this

    can be bigger than `align_of::<u16>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomicu16-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU16;

  

  let is_lock_free = AtomicU16::is_lock_free();

  ```

- <span id="atomicu16-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU16;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicU16::is_always_lock_free();

  ```

- <span id="atomicu16-get-mut"></span>`const fn get_mut(&mut self) -> &mut u16`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let mut some_var = AtomicU16::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomicu16-into-inner"></span>`const fn into_inner(self) -> u16`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU16;

  

  let some_var = AtomicU16::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomicu16-load"></span>`fn load(&self, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let some_var = AtomicU16::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu16-store"></span>`fn store(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let some_var = AtomicU16::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu16-swap"></span>`fn swap(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let some_var = AtomicU16::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomicu16-compare-exchange"></span>`fn compare_exchange(&self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let some_var = AtomicU16::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu16-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let val = AtomicU16::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicu16-fetch-add"></span>`fn fetch_add(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu16-add"></span>`fn add(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu16-fetch-sub"></span>`fn fetch_sub(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu16-sub"></span>`fn sub(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu16-fetch-and"></span>`fn fetch_and(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu16-and"></span>`fn and(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu16-fetch-nand"></span>`fn fetch_nand(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomicu16-fetch-or"></span>`fn fetch_or(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu16-or"></span>`fn or(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu16-fetch-xor"></span>`fn fetch_xor(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu16-xor"></span>`fn xor(&self, val: u16, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu16-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let x = AtomicU16::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomicu16-fetch-max"></span>`fn fetch_max(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomicu16-fetch-min"></span>`fn fetch_min(&self, val: u16, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomicu16-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomicu16-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu16-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu16-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu16-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu16-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u16` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u16.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_u16.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu16-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU16, Ordering};

  

  let foo = AtomicU16::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u16.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu16-as-ptr"></span>`const fn as_ptr(&self) -> *mut u16`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicU16`

- <span id="atomicu16-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU16`

- <span id="atomicu16-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU16`

- <span id="atomicu16-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicU16`

- <span id="atomicu16-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU16`

- <span id="atomicu16-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicU16`

- <span id="atomicu16-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU16`

- <span id="atomicu16-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicU16`

##### `impl<U> TryFrom for AtomicU16`

- <span id="atomicu16-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu16-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU16`

- <span id="atomicu16-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu16-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI32`

```rust
struct AtomicI32 {
    inner: self::core_atomic::AtomicI32,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4805`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4805)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i32`.

If the compiler and the platform support atomic loads and stores of `i32`, this type is a wrapper for the standard library's `AtomicI32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI32::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici32-new"></span>`const fn new(v: i32) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI32;

  

  let atomic_forty_two = AtomicI32::new(42);

  ```

- <span id="atomici32-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i32) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicI32>()` (note that on some platforms this

    can be bigger than `align_of::<i32>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomici32-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI32;

  

  let is_lock_free = AtomicI32::is_lock_free();

  ```

- <span id="atomici32-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI32;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicI32::is_always_lock_free();

  ```

- <span id="atomici32-get-mut"></span>`const fn get_mut(&mut self) -> &mut i32`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let mut some_var = AtomicI32::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomici32-into-inner"></span>`const fn into_inner(self) -> i32`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI32;

  

  let some_var = AtomicI32::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomici32-load"></span>`fn load(&self, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let some_var = AtomicI32::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici32-store"></span>`fn store(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let some_var = AtomicI32::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici32-swap"></span>`fn swap(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let some_var = AtomicI32::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomici32-compare-exchange"></span>`fn compare_exchange(&self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let some_var = AtomicI32::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici32-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let val = AtomicI32::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomici32-fetch-add"></span>`fn fetch_add(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici32-add"></span>`fn add(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici32-fetch-sub"></span>`fn fetch_sub(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici32-sub"></span>`fn sub(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici32-fetch-and"></span>`fn fetch_and(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici32-and"></span>`fn and(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici32-fetch-nand"></span>`fn fetch_nand(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomici32-fetch-or"></span>`fn fetch_or(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici32-or"></span>`fn or(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici32-fetch-xor"></span>`fn fetch_xor(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici32-xor"></span>`fn xor(&self, val: i32, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici32-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let x = AtomicI32::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomici32-fetch-max"></span>`fn fetch_max(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomici32-fetch-min"></span>`fn fetch_min(&self, val: i32, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomici32-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomici32-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici32-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici32-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici32-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici32-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i32` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i32.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_i32.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici32-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI32, Ordering};

  

  let foo = AtomicI32::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i32.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici32-as-ptr"></span>`const fn as_ptr(&self) -> *mut i32`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicI32`

- <span id="atomici32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI32`

- <span id="atomici32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI32`

- <span id="atomici32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicI32`

- <span id="atomici32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI32`

- <span id="atomici32-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicI32`

- <span id="atomici32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI32`

- <span id="atomici32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicI32`

##### `impl<U> TryFrom for AtomicI32`

- <span id="atomici32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI32`

- <span id="atomici32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU32`

```rust
struct AtomicU32 {
    inner: self::core_atomic::AtomicU32,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4806-4807`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4806-L4807)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u32`.

If the compiler and the platform support atomic loads and stores of `u32`, this type is a wrapper for the standard library's `AtomicU32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU32::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu32-new"></span>`const fn new(v: u32) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU32;

  

  let atomic_forty_two = AtomicU32::new(42);

  ```

- <span id="atomicu32-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u32) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicU32>()` (note that on some platforms this

    can be bigger than `align_of::<u32>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomicu32-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU32;

  

  let is_lock_free = AtomicU32::is_lock_free();

  ```

- <span id="atomicu32-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU32;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicU32::is_always_lock_free();

  ```

- <span id="atomicu32-get-mut"></span>`const fn get_mut(&mut self) -> &mut u32`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let mut some_var = AtomicU32::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomicu32-into-inner"></span>`const fn into_inner(self) -> u32`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU32;

  

  let some_var = AtomicU32::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomicu32-load"></span>`fn load(&self, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let some_var = AtomicU32::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu32-store"></span>`fn store(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let some_var = AtomicU32::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu32-swap"></span>`fn swap(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let some_var = AtomicU32::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomicu32-compare-exchange"></span>`fn compare_exchange(&self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let some_var = AtomicU32::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu32-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let val = AtomicU32::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicu32-fetch-add"></span>`fn fetch_add(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu32-add"></span>`fn add(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu32-fetch-sub"></span>`fn fetch_sub(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu32-sub"></span>`fn sub(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu32-fetch-and"></span>`fn fetch_and(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu32-and"></span>`fn and(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu32-fetch-nand"></span>`fn fetch_nand(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomicu32-fetch-or"></span>`fn fetch_or(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu32-or"></span>`fn or(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu32-fetch-xor"></span>`fn fetch_xor(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu32-xor"></span>`fn xor(&self, val: u32, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu32-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let x = AtomicU32::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomicu32-fetch-max"></span>`fn fetch_max(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomicu32-fetch-min"></span>`fn fetch_min(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomicu32-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomicu32-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu32-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu32-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu32-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu32-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u32` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u32.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_u32.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu32-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU32, Ordering};

  

  let foo = AtomicU32::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u32.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu32-as-ptr"></span>`const fn as_ptr(&self) -> *mut u32`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicU32`

- <span id="atomicu32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU32`

- <span id="atomicu32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU32`

- <span id="atomicu32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicU32`

- <span id="atomicu32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU32`

- <span id="atomicu32-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicU32`

- <span id="atomicu32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU32`

- <span id="atomicu32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicU32`

##### `impl<U> TryFrom for AtomicU32`

- <span id="atomicu32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU32`

- <span id="atomicu32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI64`

```rust
struct AtomicI64 {
    inner: self::core_atomic::AtomicI64,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4810`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4810)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i64`.

If the compiler and the platform support atomic loads and stores of `i64`, this type is a wrapper for the standard library's `AtomicI64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI64::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici64-new"></span>`const fn new(v: i64) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI64;

  

  let atomic_forty_two = AtomicI64::new(42);

  ```

- <span id="atomici64-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i64) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicI64>()` (note that on some platforms this

    can be bigger than `align_of::<i64>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomici64-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI64;

  

  let is_lock_free = AtomicI64::is_lock_free();

  ```

- <span id="atomici64-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI64;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicI64::is_always_lock_free();

  ```

- <span id="atomici64-get-mut"></span>`const fn get_mut(&mut self) -> &mut i64`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let mut some_var = AtomicI64::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomici64-into-inner"></span>`const fn into_inner(self) -> i64`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI64;

  

  let some_var = AtomicI64::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomici64-load"></span>`fn load(&self, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let some_var = AtomicI64::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici64-store"></span>`fn store(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let some_var = AtomicI64::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici64-swap"></span>`fn swap(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let some_var = AtomicI64::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomici64-compare-exchange"></span>`fn compare_exchange(&self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let some_var = AtomicI64::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici64-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let val = AtomicI64::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomici64-fetch-add"></span>`fn fetch_add(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici64-add"></span>`fn add(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici64-fetch-sub"></span>`fn fetch_sub(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici64-sub"></span>`fn sub(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici64-fetch-and"></span>`fn fetch_and(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici64-and"></span>`fn and(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici64-fetch-nand"></span>`fn fetch_nand(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomici64-fetch-or"></span>`fn fetch_or(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici64-or"></span>`fn or(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici64-fetch-xor"></span>`fn fetch_xor(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici64-xor"></span>`fn xor(&self, val: i64, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici64-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let x = AtomicI64::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomici64-fetch-max"></span>`fn fetch_max(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomici64-fetch-min"></span>`fn fetch_min(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomici64-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomici64-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici64-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici64-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici64-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici64-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i64` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i64.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_i64.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici64-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI64, Ordering};

  

  let foo = AtomicI64::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i64.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici64-as-ptr"></span>`const fn as_ptr(&self) -> *mut i64`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicI64`

- <span id="atomici64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI64`

- <span id="atomici64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI64`

- <span id="atomici64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicI64`

- <span id="atomici64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI64`

- <span id="atomici64-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicI64`

- <span id="atomici64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI64`

- <span id="atomici64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicI64`

##### `impl<U> TryFrom for AtomicI64`

- <span id="atomici64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI64`

- <span id="atomici64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU64`

```rust
struct AtomicU64 {
    inner: self::core_atomic::AtomicU64,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4811-4812`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4811-L4812)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u64`.

If the compiler and the platform support atomic loads and stores of `u64`, this type is a wrapper for the standard library's `AtomicU64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU64::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu64-new"></span>`const fn new(v: u64) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU64;

  

  let atomic_forty_two = AtomicU64::new(42);

  ```

- <span id="atomicu64-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u64) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicU64>()` (note that on some platforms this

    can be bigger than `align_of::<u64>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomicu64-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU64;

  

  let is_lock_free = AtomicU64::is_lock_free();

  ```

- <span id="atomicu64-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU64;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicU64::is_always_lock_free();

  ```

- <span id="atomicu64-get-mut"></span>`const fn get_mut(&mut self) -> &mut u64`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let mut some_var = AtomicU64::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomicu64-into-inner"></span>`const fn into_inner(self) -> u64`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU64;

  

  let some_var = AtomicU64::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomicu64-load"></span>`fn load(&self, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let some_var = AtomicU64::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu64-store"></span>`fn store(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let some_var = AtomicU64::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu64-swap"></span>`fn swap(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let some_var = AtomicU64::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomicu64-compare-exchange"></span>`fn compare_exchange(&self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let some_var = AtomicU64::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu64-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let val = AtomicU64::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicu64-fetch-add"></span>`fn fetch_add(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu64-add"></span>`fn add(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu64-fetch-sub"></span>`fn fetch_sub(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu64-sub"></span>`fn sub(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu64-fetch-and"></span>`fn fetch_and(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu64-and"></span>`fn and(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu64-fetch-nand"></span>`fn fetch_nand(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomicu64-fetch-or"></span>`fn fetch_or(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu64-or"></span>`fn or(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu64-fetch-xor"></span>`fn fetch_xor(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu64-xor"></span>`fn xor(&self, val: u64, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu64-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let x = AtomicU64::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomicu64-fetch-max"></span>`fn fetch_max(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomicu64-fetch-min"></span>`fn fetch_min(&self, val: u64, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomicu64-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomicu64-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu64-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu64-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu64-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu64-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u64` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u64.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_u64.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu64-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU64, Ordering};

  

  let foo = AtomicU64::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u64.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu64-as-ptr"></span>`const fn as_ptr(&self) -> *mut u64`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicU64`

- <span id="atomicu64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU64`

- <span id="atomicu64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU64`

- <span id="atomicu64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicU64`

- <span id="atomicu64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU64`

- <span id="atomicu64-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicU64`

- <span id="atomicu64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU64`

- <span id="atomicu64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicU64`

##### `impl<U> TryFrom for AtomicU64`

- <span id="atomicu64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU64`

- <span id="atomicu64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI128`

```rust
struct AtomicI128 {
    inner: self::atomic128::x86_64::AtomicI128,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4815`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4815)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i128`.

If the compiler and the platform support atomic loads and stores of `i128`, this type is a wrapper for the standard library's `AtomicI128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI128::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomici128-new"></span>`const fn new(v: i128) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI128;

  

  let atomic_forty_two = AtomicI128::new(42);

  ```

- <span id="atomici128-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut i128) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicI128>()` (note that on some platforms this

    can be bigger than `align_of::<i128>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomici128-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI128;

  

  let is_lock_free = AtomicI128::is_lock_free();

  ```

- <span id="atomici128-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI128;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicI128::is_always_lock_free();

  ```

- <span id="atomici128-get-mut"></span>`const fn get_mut(&mut self) -> &mut i128`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let mut some_var = AtomicI128::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomici128-into-inner"></span>`const fn into_inner(self) -> i128`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicI128;

  

  let some_var = AtomicI128::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomici128-load"></span>`fn load(&self, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let some_var = AtomicI128::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici128-store"></span>`fn store(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let some_var = AtomicI128::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici128-swap"></span>`fn swap(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let some_var = AtomicI128::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomici128-compare-exchange"></span>`fn compare_exchange(&self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let some_var = AtomicI128::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomici128-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let val = AtomicI128::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomici128-fetch-add"></span>`fn fetch_add(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici128-add"></span>`fn add(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici128-fetch-sub"></span>`fn fetch_sub(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici128-sub"></span>`fn sub(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomici128-fetch-and"></span>`fn fetch_and(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici128-and"></span>`fn and(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomici128-fetch-nand"></span>`fn fetch_nand(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomici128-fetch-or"></span>`fn fetch_or(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici128-or"></span>`fn or(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomici128-fetch-xor"></span>`fn fetch_xor(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici128-xor"></span>`fn xor(&self, val: i128, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomici128-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i128, i128>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let x = AtomicI128::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomici128-fetch-max"></span>`fn fetch_max(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomici128-fetch-min"></span>`fn fetch_min(&self, val: i128, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomici128-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomici128-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici128-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomici128-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici128-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomici128-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i128` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i128.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_i128.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici128-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicI128, Ordering};

  

  let foo = AtomicI128::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_i128.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomici128-as-ptr"></span>`const fn as_ptr(&self) -> *mut i128`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicI128`

- <span id="atomici128-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI128`

- <span id="atomici128-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI128`

- <span id="atomici128-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicI128`

- <span id="atomici128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI128`

- <span id="atomici128-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicI128`

- <span id="atomici128-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI128`

- <span id="atomici128-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicI128`

##### `impl<U> TryFrom for AtomicI128`

- <span id="atomici128-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici128-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI128`

- <span id="atomici128-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici128-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU128`

```rust
struct AtomicU128 {
    inner: self::atomic128::x86_64::AtomicU128,
}
```

*Defined in [`portable-atomic-1.11.1/src/lib.rs:4816-4817`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L4816-L4817)*

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u128`.

If the compiler and the platform support atomic loads and stores of `u128`, this type is a wrapper for the standard library's `AtomicU128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU128::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- <span id="atomicu128-new"></span>`const fn new(v: u128) -> Self`

  Creates a new atomic integer.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU128;

  

  let atomic_forty_two = AtomicU128::new(42);

  ```

- <span id="atomicu128-from-ptr"></span>`const unsafe fn from_ptr<'a>(ptr: *mut u128) -> &'a Self`

  Creates a new reference to an atomic integer from a pointer.

  

  This is `const fn` on Rust 1.83+.

  

  # Safety

  

  * `ptr` must be aligned to `align_of::<AtomicU128>()` (note that on some platforms this

    can be bigger than `align_of::<u128>()`).

  * `ptr` must be [`valid`](../thiserror_impl/valid/index.md) for both reads and writes for the whole lifetime `'a`.

  * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value

    behind `ptr` must have a happens-before relationship with atomic accesses via

    the returned value (or vice-versa).

    * In other words, time periods where the value is accessed atomically may not

      overlap with periods where the value is accessed non-atomically.

    * This requirement is trivially satisfied if `ptr` is never used non-atomically

      for the duration of lifetime `'a`. Most use cases should be able to follow

      this guideline.

    * This requirement is also trivially satisfied if all accesses (atomic or not) are

      done from the same thread.

  * If this atomic type is *not* lock-free:

    * Any accesses to the value behind `ptr` must have a happens-before relationship

      with accesses via the returned value (or vice-versa).

    * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must

      be compatible with operations performed by this atomic type.

  * This method must not be used to create overlapping or mixed-size atomic

    accesses, as these are not supported by the memory model.

- <span id="atomicu128-is-lock-free"></span>`fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU128;

  

  let is_lock_free = AtomicU128::is_lock_free();

  ```

- <span id="atomicu128-is-always-lock-free"></span>`const fn is_always_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary

  atomic instructions, global locks for every potentially

  concurrent atomic operation will be used.

  

  **Note:** If the atomic operation relies on dynamic CPU feature detection,

  this type may be lock-free even if the function returns false.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU128;

  

  const IS_ALWAYS_LOCK_FREE: bool = AtomicU128::is_always_lock_free();

  ```

- <span id="atomicu128-get-mut"></span>`const fn get_mut(&mut self) -> &mut u128`

  Returns a mutable reference to the underlying integer.

  

  This is safe because the mutable reference guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.83+.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let mut some_var = AtomicU128::new(10);

  assert_eq!(*some_var.get_mut(), 10);

  *some_var.get_mut() = 5;

  assert_eq!(some_var.load(Ordering::SeqCst), 5);

  ```

- <span id="atomicu128-into-inner"></span>`const fn into_inner(self) -> u128`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  This is `const fn` on Rust 1.56+.

  

  # Examples

  

  ```rust

  use portable_atomic::AtomicU128;

  

  let some_var = AtomicU128::new(5);

  assert_eq!(some_var.into_inner(), 5);

  ```

- <span id="atomicu128-load"></span>`fn load(&self, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Loads a value from the atomic integer.

  

  `load` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, `Acquire` and `Relaxed`.

  

  # Panics

  

  Panics if `order` is [`Release`](../rustversion/release/index.md) or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let some_var = AtomicU128::new(5);

  

  assert_eq!(some_var.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu128-store"></span>`fn store(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

  Stores a value into the atomic integer.

  

  `store` takes an [`Ordering`](#ordering) argument which describes the memory ordering of this operation.

  Possible values are `SeqCst`, [`Release`](../rustversion/release/index.md) and `Relaxed`.

  

  # Panics

  

  Panics if `order` is `Acquire` or `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let some_var = AtomicU128::new(5);

  

  some_var.store(10, Ordering::Relaxed);

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu128-swap"></span>`fn swap(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Stores a value into the atomic integer, returning the previous value.

  

  `swap` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let some_var = AtomicU128::new(5);

  

  assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);

  ```

- <span id="atomicu128-compare-exchange"></span>`fn compare_exchange(&self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  

  The return value is a result indicating whether the new value was written and

  containing the previous value. On success this value is guaranteed to be equal to

  `current`.

  

  `compare_exchange` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let some_var = AtomicU128::new(5);

  

  assert_eq!(

      some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),

      Ok(5),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  

  assert_eq!(

      some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),

      Err(10),

  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  ```

- <span id="atomicu128-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](#ordering)

  Stores a value into the atomic integer if the current value is the same as

  the `current` value.

  Unlike [`compare_exchange`](Self::compare_exchange)

  this function is allowed to spuriously fail even

  when the comparison succeeds, which can result in more efficient code on some

  platforms. The return value is a result indicating whether the new value was

  written and containing the previous value.

  

  `compare_exchange_weak` takes two [`Ordering`](#ordering) arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `failure` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let val = AtomicU128::new(4);

  

  let mut old = val.load(Ordering::Relaxed);

  loop {

      let new = old * 2;

      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {

          Ok(_) => break,

          Err(x) => old = x,

      }

  }

  ```

- <span id="atomicu128-fetch-add"></span>`fn fetch_add(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Adds to the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0);

  assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu128-add"></span>`fn add(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

  Adds to the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_add`, this does not return the previous value.

  

  `add` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_add` on some platforms.

  

  - MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0);

  foo.add(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu128-fetch-sub"></span>`fn fetch_sub(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Subtracts from the current value, returning the previous value.

  

  This operation wraps around on overflow.

  

  `fetch_sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(20);

  assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu128-sub"></span>`fn sub(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

  Subtracts from the current value.

  

  This operation wraps around on overflow.

  

  Unlike `fetch_sub`, this does not return the previous value.

  

  `sub` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_sub` on some platforms.

  

  - MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(20);

  foo.sub(10, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 10);

  ```

- <span id="atomicu128-fetch-and"></span>`fn fetch_and(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu128-and"></span>`fn and(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "and" with the current value.

  

  Performs a bitwise "and" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_and`, this does not return the previous value.

  

  `and` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_and` on some platforms.

  

  - x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b101101);

  assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

  ```

- <span id="atomicu128-fetch-nand"></span>`fn fetch_nand(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Bitwise "nand" with the current value.

  

  Performs a bitwise "nand" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_nand` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0x13);

  assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);

  assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

  ```

- <span id="atomicu128-fetch-or"></span>`fn fetch_or(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu128-or"></span>`fn or(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "or" with the current value.

  

  Performs a bitwise "or" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_or`, this does not return the previous value.

  

  `or` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_or` on some platforms.

  

  - x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b101101);

  assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

  ```

- <span id="atomicu128-fetch-xor"></span>`fn fetch_xor(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b101101);

  assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu128-xor"></span>`fn xor(&self, val: u128, order: Ordering)` — [`Ordering`](#ordering)

  Bitwise "xor" with the current value.

  

  Performs a bitwise "xor" operation on the current value and the argument `val`, and

  sets the new value to the result.

  

  Unlike `fetch_xor`, this does not return the previous value.

  

  `xor` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_xor` on some platforms.

  

  - x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

  

  Note: On x86/x86_64, the use of either function should not usually

  affect the generated code, because LLVM can properly optimize the case

  where the result is unused.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b101101);

  foo.xor(0b110011, Ordering::SeqCst);

  assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

  ```

- <span id="atomicu128-fetch-update"></span>`fn fetch_update<F>(&self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u128, u128>` — [`Ordering`](#ordering)

  Fetches the value, and applies a function to it that returns an optional

  new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else

  `Err(previous_value)`.

  

  Note: This may call the function multiple times if the value has been changed from other threads in

  the meantime, as long as the function returns `Some(_)`, but the function will have been applied

  only once to the stored value.

  

  `fetch_update` takes two [`Ordering`](#ordering) arguments to describe the memory ordering of this operation.

  The first describes the required ordering for when the operation finally succeeds while the second

  describes the required ordering for loads. These correspond to the success and failure orderings of

  [`compare_exchange`](Self::compare_exchange) respectively.

  

  Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using [`Release`](../rustversion/release/index.md) makes the final successful load

  `Relaxed`. The (failed) load ordering can only be `SeqCst`, `Acquire` or `Relaxed`.

  

  # Panics

  

  Panics if `fetch_order` is [`Release`](../rustversion/release/index.md), `AcqRel`.

  

  # Considerations

  

  This method is not magic; it is not provided by the hardware.

  It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),

  and suffers from the same drawbacks.

  In particular, this method will not circumvent the [ABA Problem].

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let x = AtomicU128::new(7);

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));

  assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));

  assert_eq!(x.load(Ordering::SeqCst), 9);

  ```

- <span id="atomicu128-fetch-max"></span>`fn fetch_max(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Maximum with the current value.

  

  Finds the maximum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_max` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(23);

  assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);

  assert_eq!(foo.load(Ordering::SeqCst), 42);

  ```

  

  If you want to obtain the maximum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(23);

  let bar = 42;

  let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);

  assert!(max_foo == 42);

  ```

- <span id="atomicu128-fetch-min"></span>`fn fetch_min(&self, val: u128, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Minimum with the current value.

  

  Finds the minimum of the current value and the argument `val`, and

  sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_min` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(23);

  assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 23);

  assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);

  assert_eq!(foo.load(Ordering::Relaxed), 22);

  ```

  

  If you want to obtain the minimum value in one step, you can use the following:

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(23);

  let bar = 12;

  let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);

  assert_eq!(min_foo, 12);

  ```

- <span id="atomicu128-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Sets the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_set` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b0000);

  assert!(!foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_set(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  ```

- <span id="atomicu128-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Clears the bit at the specified bit-position to 1.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_clear` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b0001);

  assert!(foo.bit_clear(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu128-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](#ordering)

  Toggles the bit at the specified bit-position.

  

  Returns `true` if the specified bit was previously set to 1.

  

  `bit_toggle` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0b0000);

  assert!(!foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0001);

  assert!(foo.bit_toggle(0, Ordering::Relaxed));

  assert_eq!(foo.load(Ordering::Relaxed), 0b0000);

  ```

- <span id="atomicu128-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0);

  assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu128-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](#ordering)

  Logical negates the current value, and sets the new value to the result.

  

  Unlike `fetch_not`, this does not return the previous value.

  

  `not` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_not` on some platforms.

  

  - x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  - MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(0);

  foo.not(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), !0);

  ```

- <span id="atomicu128-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u128` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Returns the previous value.

  

  `fetch_neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(5);

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u128.wrapping_neg());

  assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_u128.wrapping_neg());

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu128-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](#ordering)

  Negates the current value, and sets the new value to the result.

  

  Unlike `fetch_neg`, this does not return the previous value.

  

  `neg` takes an [`Ordering`](#ordering) argument which describes the memory ordering

  of this operation. All ordering modes are possible. Note that using

  `Acquire` makes the store part of this operation `Relaxed`, and

  using [`Release`](../rustversion/release/index.md) makes the load part `Relaxed`.

  

  This function may generate more efficient code than `fetch_neg` on some platforms.

  

  - x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

  

  # Examples

  

  ```rust

  use portable_atomic::{AtomicU128, Ordering};

  

  let foo = AtomicU128::new(5);

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5_u128.wrapping_neg());

  foo.neg(Ordering::Relaxed);

  assert_eq!(foo.load(Ordering::Relaxed), 5);

  ```

- <span id="atomicu128-as-ptr"></span>`const fn as_ptr(&self) -> *mut u128`

  Returns a mutable pointer to the underlying integer.

  

  Returning an `*mut` pointer from a shared reference to this atomic is

  safe because the atomic types work with interior mutability. Any use of

  the returned raw pointer requires an `unsafe` block and has to uphold

  the safety requirements. If there is concurrent access, note the following

  additional safety requirements:

  

  - If this atomic type is [lock-free](Self::is_lock_free), any concurrent

    operations on it must be atomic.

  - Otherwise, any concurrent operations on it must be compatible with

    operations performed by this atomic type.

  

  This is `const fn` on Rust 1.58+.

#### Trait Implementations

##### `impl Any for AtomicU128`

- <span id="atomicu128-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU128`

- <span id="atomicu128-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU128`

- <span id="atomicu128-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicU128`

- <span id="atomicu128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU128`

- <span id="atomicu128-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicU128`

- <span id="atomicu128-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU128`

- <span id="atomicu128-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl RefUnwindSafe for AtomicU128`

##### `impl<U> TryFrom for AtomicU128`

- <span id="atomicu128-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu128-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU128`

- <span id="atomicu128-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu128-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `Ordering`

```rust
fn Ordering(self) -> Result<U, <U as TryFrom>::Error>
```

## Macros

### `cfg_has_atomic_ptr!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:174-178`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L174-L178)*

### `cfg_no_atomic_ptr!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:180-182`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L180-L182)*

### `atomic_int!`

*Defined in [`portable-atomic-1.11.1/src/lib.rs:2720-4774`](../../.source_1765633015/portable-atomic-1.11.1/src/lib.rs#L2720-L4774)*

### `cfg_has_atomic_8!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:18-22`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L18-L22)*

### `cfg_no_atomic_8!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:24-26`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L24-L26)*

### `cfg_has_atomic_16!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:28-32`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L28-L32)*

### `cfg_no_atomic_16!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:34-36`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L34-L36)*

### `cfg_has_atomic_32!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:88-92`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L88-L92)*

### `cfg_no_atomic_32!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:94-96`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L94-L96)*

### `cfg_has_atomic_64!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:174-178`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L174-L178)*

### `cfg_no_atomic_64!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:180-182`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L180-L182)*

### `cfg_has_atomic_128!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:323-327`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L323-L327)*

### `cfg_no_atomic_128!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:329-331`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L329-L331)*

### `cfg_has_atomic_cas!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:446-450`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L446-L450)*

### `cfg_no_atomic_cas!`

*Defined in [`portable-atomic-1.11.1/src/cfgs.rs:452-454`](../../.source_1765633015/portable-atomic-1.11.1/src/cfgs.rs#L452-L454)*

