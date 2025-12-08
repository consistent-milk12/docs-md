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

## Modules

- [`cfgs`](cfgs/index.md) - 
- [`utils`](utils/index.md) - 
- [`imp`](imp/index.md) - 
- [`hint`](hint/index.md) - Re-export of the [`core::hint`] module.

## Structs

### `AtomicBool`

```rust
struct AtomicBool {
    v: core::cell::UnsafeCell<u8>,
}
```

A boolean type which can be safely shared between threads.

This type has the same in-memory representation as a `bool`.

If the compiler and the platform support atomic loads and stores of `u8`,
this type is a wrapper for the standard library's
[`AtomicBool`](core::sync::atomic::AtomicBool). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

#### Implementations

- `const fn new(v: bool) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut bool) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut bool`

- `const fn into_inner(self: Self) -> bool`

- `fn load(self: &Self, order: Ordering) -> bool`

- `fn store(self: &Self, val: bool, order: Ordering)`

- `fn swap(self: &Self, val: bool, order: Ordering) -> bool`

- `fn compare_exchange(self: &Self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool>`

- `fn compare_exchange_weak(self: &Self, current: bool, new: bool, success: Ordering, failure: Ordering) -> Result<bool, bool>`

- `fn fetch_and(self: &Self, val: bool, order: Ordering) -> bool`

- `fn and(self: &Self, val: bool, order: Ordering)`

- `fn fetch_nand(self: &Self, val: bool, order: Ordering) -> bool`

- `fn fetch_or(self: &Self, val: bool, order: Ordering) -> bool`

- `fn or(self: &Self, val: bool, order: Ordering)`

- `fn fetch_xor(self: &Self, val: bool, order: Ordering) -> bool`

- `fn xor(self: &Self, val: bool, order: Ordering)`

- `fn fetch_not(self: &Self, order: Ordering) -> bool`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<bool, bool>`

- `const fn as_ptr(self: &Self) -> *mut bool`

- `fn as_atomic_u8(self: &Self) -> &self::core_atomic::AtomicU8` — [`AtomicU8`](imp/core_atomic/index.md)

#### Trait Implementations

##### `impl Debug for AtomicBool`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicBool`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicBool`

##### `impl Sync for AtomicBool`

### `AtomicPtr<T>`

```rust
struct AtomicPtr<T> {
    inner: self::core_atomic::AtomicPtr<T>,
}
```

A raw pointer type which can be safely shared between threads.

This type has the same in-memory representation as a `*mut T`.

If the compiler and the platform support atomic loads and stores of pointers,
this type is a wrapper for the standard library's
[`AtomicPtr`](core::sync::atomic::AtomicPtr). If the platform supports it
but the compiler does not, atomic operations are implemented using inline
assembly.

#### Implementations

- `const fn new(p: *mut T) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut *mut T) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut *mut T`

- `const fn into_inner(self: Self) -> *mut T`

- `fn load(self: &Self, order: Ordering) -> *mut T`

- `fn store(self: &Self, ptr: *mut T, order: Ordering)`

- `fn swap(self: &Self, ptr: *mut T, order: Ordering) -> *mut T`

- `fn compare_exchange(self: &Self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>`

- `fn compare_exchange_weak(self: &Self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<*mut T, *mut T>`

- `fn fetch_ptr_add(self: &Self, val: usize, order: Ordering) -> *mut T`

- `fn fetch_ptr_sub(self: &Self, val: usize, order: Ordering) -> *mut T`

- `fn fetch_byte_add(self: &Self, val: usize, order: Ordering) -> *mut T`

- `fn fetch_byte_sub(self: &Self, val: usize, order: Ordering) -> *mut T`

- `fn fetch_or(self: &Self, val: usize, order: Ordering) -> *mut T`

- `fn fetch_and(self: &Self, val: usize, order: Ordering) -> *mut T`

- `fn fetch_xor(self: &Self, val: usize, order: Ordering) -> *mut T`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn as_atomic_usize(self: &Self) -> &AtomicUsize` — [`AtomicUsize`](#atomicusize)

- `const fn as_ptr(self: &Self) -> *mut *mut T`

#### Trait Implementations

##### `impl<T> Debug for AtomicPtr<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for AtomicPtr<T>`

- `fn default() -> Self`

##### `impl<T> Pointer for AtomicPtr<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> RefUnwindSafe for AtomicPtr<T>`

### `AtomicIsize`

```rust
struct AtomicIsize {
    inner: self::core_atomic::AtomicIsize,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`isize`.

If the compiler and the platform support atomic loads and stores of `isize`, this type is a wrapper for the standard library's `AtomicIsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicIsize::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: isize) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut isize) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut isize`

- `const fn into_inner(self: Self) -> isize`

- `fn load(self: &Self, order: Ordering) -> isize`

- `fn store(self: &Self, val: isize, order: Ordering)`

- `fn swap(self: &Self, val: isize, order: Ordering) -> isize`

- `fn compare_exchange(self: &Self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>`

- `fn compare_exchange_weak(self: &Self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>`

- `fn fetch_add(self: &Self, val: isize, order: Ordering) -> isize`

- `fn add(self: &Self, val: isize, order: Ordering)`

- `fn fetch_sub(self: &Self, val: isize, order: Ordering) -> isize`

- `fn sub(self: &Self, val: isize, order: Ordering)`

- `fn fetch_and(self: &Self, val: isize, order: Ordering) -> isize`

- `fn and(self: &Self, val: isize, order: Ordering)`

- `fn fetch_nand(self: &Self, val: isize, order: Ordering) -> isize`

- `fn fetch_or(self: &Self, val: isize, order: Ordering) -> isize`

- `fn or(self: &Self, val: isize, order: Ordering)`

- `fn fetch_xor(self: &Self, val: isize, order: Ordering) -> isize`

- `fn xor(self: &Self, val: isize, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<isize, isize>`

- `fn fetch_max(self: &Self, val: isize, order: Ordering) -> isize`

- `fn fetch_min(self: &Self, val: isize, order: Ordering) -> isize`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> isize`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> isize`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut isize`

#### Trait Implementations

##### `impl Debug for AtomicIsize`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicIsize`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicIsize`

### `AtomicUsize`

```rust
struct AtomicUsize {
    inner: self::core_atomic::AtomicUsize,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`usize`.

If the compiler and the platform support atomic loads and stores of `usize`, this type is a wrapper for the standard library's `AtomicUsize`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicUsize::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: usize) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut usize) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut usize`

- `const fn into_inner(self: Self) -> usize`

- `fn load(self: &Self, order: Ordering) -> usize`

- `fn store(self: &Self, val: usize, order: Ordering)`

- `fn swap(self: &Self, val: usize, order: Ordering) -> usize`

- `fn compare_exchange(self: &Self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>`

- `fn compare_exchange_weak(self: &Self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>`

- `fn fetch_add(self: &Self, val: usize, order: Ordering) -> usize`

- `fn add(self: &Self, val: usize, order: Ordering)`

- `fn fetch_sub(self: &Self, val: usize, order: Ordering) -> usize`

- `fn sub(self: &Self, val: usize, order: Ordering)`

- `fn fetch_and(self: &Self, val: usize, order: Ordering) -> usize`

- `fn and(self: &Self, val: usize, order: Ordering)`

- `fn fetch_nand(self: &Self, val: usize, order: Ordering) -> usize`

- `fn fetch_or(self: &Self, val: usize, order: Ordering) -> usize`

- `fn or(self: &Self, val: usize, order: Ordering)`

- `fn fetch_xor(self: &Self, val: usize, order: Ordering) -> usize`

- `fn xor(self: &Self, val: usize, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<usize, usize>`

- `fn fetch_max(self: &Self, val: usize, order: Ordering) -> usize`

- `fn fetch_min(self: &Self, val: usize, order: Ordering) -> usize`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> usize`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> usize`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut usize`

#### Trait Implementations

##### `impl Debug for AtomicUsize`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicUsize`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicUsize`

### `AtomicI8`

```rust
struct AtomicI8 {
    inner: self::core_atomic::AtomicI8,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i8`.

If the compiler and the platform support atomic loads and stores of `i8`, this type is a wrapper for the standard library's `AtomicI8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI8::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: i8) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut i8) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut i8`

- `const fn into_inner(self: Self) -> i8`

- `fn load(self: &Self, order: Ordering) -> i8`

- `fn store(self: &Self, val: i8, order: Ordering)`

- `fn swap(self: &Self, val: i8, order: Ordering) -> i8`

- `fn compare_exchange(self: &Self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>`

- `fn compare_exchange_weak(self: &Self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>`

- `fn fetch_add(self: &Self, val: i8, order: Ordering) -> i8`

- `fn add(self: &Self, val: i8, order: Ordering)`

- `fn fetch_sub(self: &Self, val: i8, order: Ordering) -> i8`

- `fn sub(self: &Self, val: i8, order: Ordering)`

- `fn fetch_and(self: &Self, val: i8, order: Ordering) -> i8`

- `fn and(self: &Self, val: i8, order: Ordering)`

- `fn fetch_nand(self: &Self, val: i8, order: Ordering) -> i8`

- `fn fetch_or(self: &Self, val: i8, order: Ordering) -> i8`

- `fn or(self: &Self, val: i8, order: Ordering)`

- `fn fetch_xor(self: &Self, val: i8, order: Ordering) -> i8`

- `fn xor(self: &Self, val: i8, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i8, i8>`

- `fn fetch_max(self: &Self, val: i8, order: Ordering) -> i8`

- `fn fetch_min(self: &Self, val: i8, order: Ordering) -> i8`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> i8`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> i8`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut i8`

#### Trait Implementations

##### `impl Debug for AtomicI8`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI8`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI8`

### `AtomicU8`

```rust
struct AtomicU8 {
    inner: self::core_atomic::AtomicU8,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u8`.

If the compiler and the platform support atomic loads and stores of `u8`, this type is a wrapper for the standard library's `AtomicU8`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU8::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: u8) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut u8) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut u8`

- `const fn into_inner(self: Self) -> u8`

- `fn load(self: &Self, order: Ordering) -> u8`

- `fn store(self: &Self, val: u8, order: Ordering)`

- `fn swap(self: &Self, val: u8, order: Ordering) -> u8`

- `fn compare_exchange(self: &Self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>`

- `fn compare_exchange_weak(self: &Self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>`

- `fn fetch_add(self: &Self, val: u8, order: Ordering) -> u8`

- `fn add(self: &Self, val: u8, order: Ordering)`

- `fn fetch_sub(self: &Self, val: u8, order: Ordering) -> u8`

- `fn sub(self: &Self, val: u8, order: Ordering)`

- `fn fetch_and(self: &Self, val: u8, order: Ordering) -> u8`

- `fn and(self: &Self, val: u8, order: Ordering)`

- `fn fetch_nand(self: &Self, val: u8, order: Ordering) -> u8`

- `fn fetch_or(self: &Self, val: u8, order: Ordering) -> u8`

- `fn or(self: &Self, val: u8, order: Ordering)`

- `fn fetch_xor(self: &Self, val: u8, order: Ordering) -> u8`

- `fn xor(self: &Self, val: u8, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u8, u8>`

- `fn fetch_max(self: &Self, val: u8, order: Ordering) -> u8`

- `fn fetch_min(self: &Self, val: u8, order: Ordering) -> u8`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> u8`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> u8`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut u8`

#### Trait Implementations

##### `impl Debug for AtomicU8`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU8`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU8`

### `AtomicI16`

```rust
struct AtomicI16 {
    inner: self::core_atomic::AtomicI16,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i16`.

If the compiler and the platform support atomic loads and stores of `i16`, this type is a wrapper for the standard library's `AtomicI16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI16::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: i16) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut i16) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut i16`

- `const fn into_inner(self: Self) -> i16`

- `fn load(self: &Self, order: Ordering) -> i16`

- `fn store(self: &Self, val: i16, order: Ordering)`

- `fn swap(self: &Self, val: i16, order: Ordering) -> i16`

- `fn compare_exchange(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>`

- `fn compare_exchange_weak(self: &Self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>`

- `fn fetch_add(self: &Self, val: i16, order: Ordering) -> i16`

- `fn add(self: &Self, val: i16, order: Ordering)`

- `fn fetch_sub(self: &Self, val: i16, order: Ordering) -> i16`

- `fn sub(self: &Self, val: i16, order: Ordering)`

- `fn fetch_and(self: &Self, val: i16, order: Ordering) -> i16`

- `fn and(self: &Self, val: i16, order: Ordering)`

- `fn fetch_nand(self: &Self, val: i16, order: Ordering) -> i16`

- `fn fetch_or(self: &Self, val: i16, order: Ordering) -> i16`

- `fn or(self: &Self, val: i16, order: Ordering)`

- `fn fetch_xor(self: &Self, val: i16, order: Ordering) -> i16`

- `fn xor(self: &Self, val: i16, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i16, i16>`

- `fn fetch_max(self: &Self, val: i16, order: Ordering) -> i16`

- `fn fetch_min(self: &Self, val: i16, order: Ordering) -> i16`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> i16`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> i16`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut i16`

#### Trait Implementations

##### `impl Debug for AtomicI16`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI16`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI16`

### `AtomicU16`

```rust
struct AtomicU16 {
    inner: self::core_atomic::AtomicU16,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`u16`](../gimli/leb128/read/index.md).

If the compiler and the platform support atomic loads and stores of [`u16`](../gimli/leb128/read/index.md), this type is a wrapper for the standard library's `AtomicU16`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU16::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: u16) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut u16) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut u16`

- `const fn into_inner(self: Self) -> u16`

- `fn load(self: &Self, order: Ordering) -> u16`

- `fn store(self: &Self, val: u16, order: Ordering)`

- `fn swap(self: &Self, val: u16, order: Ordering) -> u16`

- `fn compare_exchange(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>`

- `fn compare_exchange_weak(self: &Self, current: u16, new: u16, success: Ordering, failure: Ordering) -> Result<u16, u16>`

- `fn fetch_add(self: &Self, val: u16, order: Ordering) -> u16`

- `fn add(self: &Self, val: u16, order: Ordering)`

- `fn fetch_sub(self: &Self, val: u16, order: Ordering) -> u16`

- `fn sub(self: &Self, val: u16, order: Ordering)`

- `fn fetch_and(self: &Self, val: u16, order: Ordering) -> u16`

- `fn and(self: &Self, val: u16, order: Ordering)`

- `fn fetch_nand(self: &Self, val: u16, order: Ordering) -> u16`

- `fn fetch_or(self: &Self, val: u16, order: Ordering) -> u16`

- `fn or(self: &Self, val: u16, order: Ordering)`

- `fn fetch_xor(self: &Self, val: u16, order: Ordering) -> u16`

- `fn xor(self: &Self, val: u16, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u16, u16>`

- `fn fetch_max(self: &Self, val: u16, order: Ordering) -> u16`

- `fn fetch_min(self: &Self, val: u16, order: Ordering) -> u16`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> u16`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> u16`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut u16`

#### Trait Implementations

##### `impl Debug for AtomicU16`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU16`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU16`

### `AtomicI32`

```rust
struct AtomicI32 {
    inner: self::core_atomic::AtomicI32,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i32`.

If the compiler and the platform support atomic loads and stores of `i32`, this type is a wrapper for the standard library's `AtomicI32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI32::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: i32) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut i32) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut i32`

- `const fn into_inner(self: Self) -> i32`

- `fn load(self: &Self, order: Ordering) -> i32`

- `fn store(self: &Self, val: i32, order: Ordering)`

- `fn swap(self: &Self, val: i32, order: Ordering) -> i32`

- `fn compare_exchange(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>`

- `fn compare_exchange_weak(self: &Self, current: i32, new: i32, success: Ordering, failure: Ordering) -> Result<i32, i32>`

- `fn fetch_add(self: &Self, val: i32, order: Ordering) -> i32`

- `fn add(self: &Self, val: i32, order: Ordering)`

- `fn fetch_sub(self: &Self, val: i32, order: Ordering) -> i32`

- `fn sub(self: &Self, val: i32, order: Ordering)`

- `fn fetch_and(self: &Self, val: i32, order: Ordering) -> i32`

- `fn and(self: &Self, val: i32, order: Ordering)`

- `fn fetch_nand(self: &Self, val: i32, order: Ordering) -> i32`

- `fn fetch_or(self: &Self, val: i32, order: Ordering) -> i32`

- `fn or(self: &Self, val: i32, order: Ordering)`

- `fn fetch_xor(self: &Self, val: i32, order: Ordering) -> i32`

- `fn xor(self: &Self, val: i32, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i32, i32>`

- `fn fetch_max(self: &Self, val: i32, order: Ordering) -> i32`

- `fn fetch_min(self: &Self, val: i32, order: Ordering) -> i32`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> i32`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> i32`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut i32`

#### Trait Implementations

##### `impl Debug for AtomicI32`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI32`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI32`

### `AtomicU32`

```rust
struct AtomicU32 {
    inner: self::core_atomic::AtomicU32,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u32`.

If the compiler and the platform support atomic loads and stores of `u32`, this type is a wrapper for the standard library's `AtomicU32`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU32::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: u32) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut u32) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut u32`

- `const fn into_inner(self: Self) -> u32`

- `fn load(self: &Self, order: Ordering) -> u32`

- `fn store(self: &Self, val: u32, order: Ordering)`

- `fn swap(self: &Self, val: u32, order: Ordering) -> u32`

- `fn compare_exchange(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>`

- `fn compare_exchange_weak(self: &Self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>`

- `fn fetch_add(self: &Self, val: u32, order: Ordering) -> u32`

- `fn add(self: &Self, val: u32, order: Ordering)`

- `fn fetch_sub(self: &Self, val: u32, order: Ordering) -> u32`

- `fn sub(self: &Self, val: u32, order: Ordering)`

- `fn fetch_and(self: &Self, val: u32, order: Ordering) -> u32`

- `fn and(self: &Self, val: u32, order: Ordering)`

- `fn fetch_nand(self: &Self, val: u32, order: Ordering) -> u32`

- `fn fetch_or(self: &Self, val: u32, order: Ordering) -> u32`

- `fn or(self: &Self, val: u32, order: Ordering)`

- `fn fetch_xor(self: &Self, val: u32, order: Ordering) -> u32`

- `fn xor(self: &Self, val: u32, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u32, u32>`

- `fn fetch_max(self: &Self, val: u32, order: Ordering) -> u32`

- `fn fetch_min(self: &Self, val: u32, order: Ordering) -> u32`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> u32`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> u32`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut u32`

#### Trait Implementations

##### `impl Debug for AtomicU32`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU32`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU32`

### `AtomicI64`

```rust
struct AtomicI64 {
    inner: self::core_atomic::AtomicI64,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i64`.

If the compiler and the platform support atomic loads and stores of `i64`, this type is a wrapper for the standard library's `AtomicI64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI64::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: i64) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut i64) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut i64`

- `const fn into_inner(self: Self) -> i64`

- `fn load(self: &Self, order: Ordering) -> i64`

- `fn store(self: &Self, val: i64, order: Ordering)`

- `fn swap(self: &Self, val: i64, order: Ordering) -> i64`

- `fn compare_exchange(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>`

- `fn compare_exchange_weak(self: &Self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>`

- `fn fetch_add(self: &Self, val: i64, order: Ordering) -> i64`

- `fn add(self: &Self, val: i64, order: Ordering)`

- `fn fetch_sub(self: &Self, val: i64, order: Ordering) -> i64`

- `fn sub(self: &Self, val: i64, order: Ordering)`

- `fn fetch_and(self: &Self, val: i64, order: Ordering) -> i64`

- `fn and(self: &Self, val: i64, order: Ordering)`

- `fn fetch_nand(self: &Self, val: i64, order: Ordering) -> i64`

- `fn fetch_or(self: &Self, val: i64, order: Ordering) -> i64`

- `fn or(self: &Self, val: i64, order: Ordering)`

- `fn fetch_xor(self: &Self, val: i64, order: Ordering) -> i64`

- `fn xor(self: &Self, val: i64, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i64, i64>`

- `fn fetch_max(self: &Self, val: i64, order: Ordering) -> i64`

- `fn fetch_min(self: &Self, val: i64, order: Ordering) -> i64`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> i64`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> i64`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut i64`

#### Trait Implementations

##### `impl Debug for AtomicI64`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI64`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI64`

### `AtomicU64`

```rust
struct AtomicU64 {
    inner: self::core_atomic::AtomicU64,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u64`.

If the compiler and the platform support atomic loads and stores of `u64`, this type is a wrapper for the standard library's `AtomicU64`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU64::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: u64) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut u64) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut u64`

- `const fn into_inner(self: Self) -> u64`

- `fn load(self: &Self, order: Ordering) -> u64`

- `fn store(self: &Self, val: u64, order: Ordering)`

- `fn swap(self: &Self, val: u64, order: Ordering) -> u64`

- `fn compare_exchange(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>`

- `fn compare_exchange_weak(self: &Self, current: u64, new: u64, success: Ordering, failure: Ordering) -> Result<u64, u64>`

- `fn fetch_add(self: &Self, val: u64, order: Ordering) -> u64`

- `fn add(self: &Self, val: u64, order: Ordering)`

- `fn fetch_sub(self: &Self, val: u64, order: Ordering) -> u64`

- `fn sub(self: &Self, val: u64, order: Ordering)`

- `fn fetch_and(self: &Self, val: u64, order: Ordering) -> u64`

- `fn and(self: &Self, val: u64, order: Ordering)`

- `fn fetch_nand(self: &Self, val: u64, order: Ordering) -> u64`

- `fn fetch_or(self: &Self, val: u64, order: Ordering) -> u64`

- `fn or(self: &Self, val: u64, order: Ordering)`

- `fn fetch_xor(self: &Self, val: u64, order: Ordering) -> u64`

- `fn xor(self: &Self, val: u64, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u64, u64>`

- `fn fetch_max(self: &Self, val: u64, order: Ordering) -> u64`

- `fn fetch_min(self: &Self, val: u64, order: Ordering) -> u64`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> u64`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> u64`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut u64`

#### Trait Implementations

##### `impl Debug for AtomicU64`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU64`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU64`

### `AtomicI128`

```rust
struct AtomicI128 {
    inner: self::atomic128::x86_64::AtomicI128,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`i128`.

If the compiler and the platform support atomic loads and stores of `i128`, this type is a wrapper for the standard library's `AtomicI128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicI128::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: i128) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut i128) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut i128`

- `const fn into_inner(self: Self) -> i128`

- `fn load(self: &Self, order: Ordering) -> i128`

- `fn store(self: &Self, val: i128, order: Ordering)`

- `fn swap(self: &Self, val: i128, order: Ordering) -> i128`

- `fn compare_exchange(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>`

- `fn compare_exchange_weak(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>`

- `fn fetch_add(self: &Self, val: i128, order: Ordering) -> i128`

- `fn add(self: &Self, val: i128, order: Ordering)`

- `fn fetch_sub(self: &Self, val: i128, order: Ordering) -> i128`

- `fn sub(self: &Self, val: i128, order: Ordering)`

- `fn fetch_and(self: &Self, val: i128, order: Ordering) -> i128`

- `fn and(self: &Self, val: i128, order: Ordering)`

- `fn fetch_nand(self: &Self, val: i128, order: Ordering) -> i128`

- `fn fetch_or(self: &Self, val: i128, order: Ordering) -> i128`

- `fn or(self: &Self, val: i128, order: Ordering)`

- `fn fetch_xor(self: &Self, val: i128, order: Ordering) -> i128`

- `fn xor(self: &Self, val: i128, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<i128, i128>`

- `fn fetch_max(self: &Self, val: i128, order: Ordering) -> i128`

- `fn fetch_min(self: &Self, val: i128, order: Ordering) -> i128`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> i128`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> i128`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut i128`

#### Trait Implementations

##### `impl Debug for AtomicI128`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicI128`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    inner: self::atomic128::x86_64::AtomicU128,
}
```

An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
`u128`.

If the compiler and the platform support atomic loads and stores of `u128`, this type is a wrapper for the standard library's `AtomicU128`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call `AtomicU128::is_lock_free()` to check whether
atomic instructions or locks will be used.

#### Implementations

- `const fn new(v: u128) -> Self`

- `const unsafe fn from_ptr<'a>(ptr: *mut u128) -> &'a Self`

- `fn is_lock_free() -> bool`

- `const fn is_always_lock_free() -> bool`

- `const fn get_mut(self: &mut Self) -> &mut u128`

- `const fn into_inner(self: Self) -> u128`

- `fn load(self: &Self, order: Ordering) -> u128`

- `fn store(self: &Self, val: u128, order: Ordering)`

- `fn swap(self: &Self, val: u128, order: Ordering) -> u128`

- `fn compare_exchange(self: &Self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>`

- `fn compare_exchange_weak(self: &Self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>`

- `fn fetch_add(self: &Self, val: u128, order: Ordering) -> u128`

- `fn add(self: &Self, val: u128, order: Ordering)`

- `fn fetch_sub(self: &Self, val: u128, order: Ordering) -> u128`

- `fn sub(self: &Self, val: u128, order: Ordering)`

- `fn fetch_and(self: &Self, val: u128, order: Ordering) -> u128`

- `fn and(self: &Self, val: u128, order: Ordering)`

- `fn fetch_nand(self: &Self, val: u128, order: Ordering) -> u128`

- `fn fetch_or(self: &Self, val: u128, order: Ordering) -> u128`

- `fn or(self: &Self, val: u128, order: Ordering)`

- `fn fetch_xor(self: &Self, val: u128, order: Ordering) -> u128`

- `fn xor(self: &Self, val: u128, order: Ordering)`

- `fn fetch_update<F>(self: &Self, set_order: Ordering, fetch_order: Ordering, f: F) -> Result<u128, u128>`

- `fn fetch_max(self: &Self, val: u128, order: Ordering) -> u128`

- `fn fetch_min(self: &Self, val: u128, order: Ordering) -> u128`

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool`

- `fn fetch_not(self: &Self, order: Ordering) -> u128`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, order: Ordering) -> u128`

- `fn neg(self: &Self, order: Ordering)`

- `const fn as_ptr(self: &Self) -> *mut u128`

#### Trait Implementations

##### `impl Debug for AtomicU128`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicU128`

- `fn default() -> Self`

##### `impl RefUnwindSafe for AtomicU128`

## Functions

## Macros

### `unnamed!`

### `unnamed!`

### `atomic_int!`

### `cfg_has_atomic_8!`

### `cfg_no_atomic_8!`

### `cfg_has_atomic_16!`

### `cfg_no_atomic_16!`

### `cfg_has_atomic_32!`

### `cfg_no_atomic_32!`

### `cfg_has_atomic_64!`

### `cfg_no_atomic_64!`

### `cfg_has_atomic_128!`

### `cfg_no_atomic_128!`

### `cfg_has_atomic_cas!`

### `cfg_no_atomic_cas!`

