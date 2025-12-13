*[portable_atomic](../../../../index.md) / [imp](../../../index.md) / [atomic128](../../index.md) / [x86_64](../index.md) / [detect](index.md)*

---

# Module `detect`

## Contents

- [Structs](#structs)
  - [`CpuInfo`](#cpuinfo)
- [Enums](#enums)
  - [`CpuInfoFlag`](#cpuinfoflag)
- [Functions](#functions)
  - [`set`](#set)
  - [`test`](#test)
  - [`detect`](#detect)
  - [`__cpuid`](#cpuid)
  - [`_vender`](#vender)
  - [`_vendor_id`](#vendor-id)
  - [`_vendor_has_vmovdqa_atomic`](#vendor-has-vmovdqa-atomic)
  - [`_detect`](#detect)
- [Constants](#constants)
  - [`_VENDOR_ID_INTEL`](#vendor-id-intel)
  - [`_VENDOR_ID_INTEL2`](#vendor-id-intel2)
  - [`_VENDOR_ID_AMD`](#vendor-id-amd)
  - [`_VENDOR_ID_CENTAUR`](#vendor-id-centaur)
  - [`_VENDOR_ID_ZHAOXIN`](#vendor-id-zhaoxin)
- [Macros](#macros)
  - [`flags!`](#flags)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CpuInfo`](#cpuinfo) | struct |  |
| [`CpuInfoFlag`](#cpuinfoflag) | enum |  |
| [`set`](#set) | fn |  |
| [`test`](#test) | fn |  |
| [`detect`](#detect) | fn |  |
| [`__cpuid`](#cpuid) | fn |  |
| [`_vender`](#vender) | fn |  |
| [`_vendor_id`](#vendor-id) | fn |  |
| [`_vendor_has_vmovdqa_atomic`](#vendor-has-vmovdqa-atomic) | fn |  |
| [`_detect`](#detect) | fn |  |
| [`_VENDOR_ID_INTEL`](#vendor-id-intel) | const |  |
| [`_VENDOR_ID_INTEL2`](#vendor-id-intel2) | const |  |
| [`_VENDOR_ID_AMD`](#vendor-id-amd) | const |  |
| [`_VENDOR_ID_CENTAUR`](#vendor-id-centaur) | const |  |
| [`_VENDOR_ID_ZHAOXIN`](#vendor-id-zhaoxin) | const |  |
| [`flags!`](#flags) | macro |  |

## Structs

### `CpuInfo`

```rust
struct CpuInfo(u32);
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs:5`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs#L5)*

#### Implementations

- <span id="cpuinfo-set"></span>`fn set(&mut self, bit: CpuInfoFlag)` — [`CpuInfoFlag`](#cpuinfoflag)

- <span id="cpuinfo-test"></span>`fn test(self, bit: CpuInfoFlag) -> bool` — [`CpuInfoFlag`](#cpuinfoflag)

#### Trait Implementations

##### `impl Any for CpuInfo`

- <span id="cpuinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CpuInfo`

- <span id="cpuinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CpuInfo`

- <span id="cpuinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CpuInfo`

- <span id="cpuinfo-clone"></span>`fn clone(&self) -> CpuInfo` — [`CpuInfo`](#cpuinfo)

##### `impl CloneToUninit for CpuInfo`

- <span id="cpuinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CpuInfo`

##### `impl<T> From for CpuInfo`

- <span id="cpuinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CpuInfo`

- <span id="cpuinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for CpuInfo`

- <span id="cpuinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cpuinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CpuInfo`

- <span id="cpuinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cpuinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `CpuInfoFlag`

```rust
enum CpuInfoFlag {
    Init,
    cmpxchg16b,
    vmovdqa_atomic,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs:154-160`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs#L154-L160)*

#### Trait Implementations

##### `impl Any for CpuInfoFlag`

- <span id="cpuinfoflag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CpuInfoFlag`

- <span id="cpuinfoflag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CpuInfoFlag`

- <span id="cpuinfoflag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CpuInfoFlag`

- <span id="cpuinfoflag-clone"></span>`fn clone(&self) -> CpuInfoFlag` — [`CpuInfoFlag`](#cpuinfoflag)

##### `impl CloneToUninit for CpuInfoFlag`

- <span id="cpuinfoflag-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CpuInfoFlag`

##### `impl<T> From for CpuInfoFlag`

- <span id="cpuinfoflag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CpuInfoFlag`

- <span id="cpuinfoflag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for CpuInfoFlag`

- <span id="cpuinfoflag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cpuinfoflag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CpuInfoFlag`

- <span id="cpuinfoflag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cpuinfoflag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `set`

```rust
fn set(x: u32, bit: u32) -> u32
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs:21-23`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs#L21-L23)*

### `test`

```rust
fn test(x: u32, bit: u32) -> bool
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs:26-28`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs#L26-L28)*

### `detect`

```rust
fn detect() -> CpuInfo
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs:31-47`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs#L31-L47)*

### `__cpuid`

```rust
fn __cpuid(leaf: u32) -> core::arch::x86_64::CpuidResult
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:30-51`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L30-L51)*

### `_vender`

```rust
const fn _vender(b: &[u8; 12]) -> [u32; 3]
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:59-65`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L59-L65)*

### `_vendor_id`

```rust
fn _vendor_id() -> [u32; 3]
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:66-69`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L66-L69)*

### `_vendor_has_vmovdqa_atomic`

```rust
fn _vendor_has_vmovdqa_atomic(vendor_id: [u32; 3], family: u32) -> bool
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:70-78`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L70-L78)*

### `_detect`

```rust
fn _detect(info: &mut CpuInfo)
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:81-118`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L81-L118)*

## Constants

### `_VENDOR_ID_INTEL`
```rust
const _VENDOR_ID_INTEL: [u32; 3];
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:54`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L54)*

### `_VENDOR_ID_INTEL2`
```rust
const _VENDOR_ID_INTEL2: [u32; 3];
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:55`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L55)*

### `_VENDOR_ID_AMD`
```rust
const _VENDOR_ID_AMD: [u32; 3];
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:56`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L56)*

### `_VENDOR_ID_CENTAUR`
```rust
const _VENDOR_ID_CENTAUR: [u32; 3];
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:57`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L57)*

### `_VENDOR_ID_ZHAOXIN`
```rust
const _VENDOR_ID_ZHAOXIN: [u32; 3];
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs:58`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/x86_64.rs#L58)*

## Macros

### `flags!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs:49-92`](../../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/atomic128/../detect/common.rs#L49-L92)*

