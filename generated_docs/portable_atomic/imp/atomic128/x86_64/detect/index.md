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
  - [`__cpuid`](#__cpuid)
  - [`_vender`](#_vender)
  - [`_vendor_id`](#_vendor_id)
  - [`_vendor_has_vmovdqa_atomic`](#_vendor_has_vmovdqa_atomic)
  - [`_detect`](#_detect)
- [Constants](#constants)
  - [`_VENDOR_ID_INTEL`](#_vendor_id_intel)
  - [`_VENDOR_ID_INTEL2`](#_vendor_id_intel2)
  - [`_VENDOR_ID_AMD`](#_vendor_id_amd)
  - [`_VENDOR_ID_CENTAUR`](#_vendor_id_centaur)
  - [`_VENDOR_ID_ZHAOXIN`](#_vendor_id_zhaoxin)
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
| [`__cpuid`](#__cpuid) | fn |  |
| [`_vender`](#_vender) | fn |  |
| [`_vendor_id`](#_vendor_id) | fn |  |
| [`_vendor_has_vmovdqa_atomic`](#_vendor_has_vmovdqa_atomic) | fn |  |
| [`_detect`](#_detect) | fn |  |
| [`_VENDOR_ID_INTEL`](#_vendor_id_intel) | const |  |
| [`_VENDOR_ID_INTEL2`](#_vendor_id_intel2) | const |  |
| [`_VENDOR_ID_AMD`](#_vendor_id_amd) | const |  |
| [`_VENDOR_ID_CENTAUR`](#_vendor_id_centaur) | const |  |
| [`_VENDOR_ID_ZHAOXIN`](#_vendor_id_zhaoxin) | const |  |
| [`flags!`](#flags) | macro |  |

## Structs

### `CpuInfo`

```rust
struct CpuInfo(u32);
```

#### Implementations

- <span id="cpuinfo-set"></span>`fn set(&mut self, bit: CpuInfoFlag)` — [`CpuInfoFlag`](#cpuinfoflag)

- <span id="cpuinfo-test"></span>`fn test(self, bit: CpuInfoFlag) -> bool` — [`CpuInfoFlag`](#cpuinfoflag)

#### Trait Implementations

##### `impl Clone for CpuInfo`

- <span id="cpuinfo-clone"></span>`fn clone(&self) -> CpuInfo` — [`CpuInfo`](#cpuinfo)

##### `impl Copy for CpuInfo`

## Enums

### `CpuInfoFlag`

```rust
enum CpuInfoFlag {
    Init,
    cmpxchg16b,
    vmovdqa_atomic,
}
```

#### Trait Implementations

##### `impl Clone for CpuInfoFlag`

- <span id="cpuinfoflag-clone"></span>`fn clone(&self) -> CpuInfoFlag` — [`CpuInfoFlag`](#cpuinfoflag)

##### `impl Copy for CpuInfoFlag`

## Functions

### `set`

```rust
fn set(x: u32, bit: u32) -> u32
```

### `test`

```rust
fn test(x: u32, bit: u32) -> bool
```

### `detect`

```rust
fn detect() -> CpuInfo
```

### `__cpuid`

```rust
fn __cpuid(leaf: u32) -> core::arch::x86_64::CpuidResult
```

### `_vender`

```rust
const fn _vender(b: &[u8; 12]) -> [u32; 3]
```

### `_vendor_id`

```rust
fn _vendor_id() -> [u32; 3]
```

### `_vendor_has_vmovdqa_atomic`

```rust
fn _vendor_has_vmovdqa_atomic(vendor_id: [u32; 3], family: u32) -> bool
```

### `_detect`

```rust
fn _detect(info: &mut CpuInfo)
```

## Constants

### `_VENDOR_ID_INTEL`

```rust
const _VENDOR_ID_INTEL: [u32; 3];
```

### `_VENDOR_ID_INTEL2`

```rust
const _VENDOR_ID_INTEL2: [u32; 3];
```

### `_VENDOR_ID_AMD`

```rust
const _VENDOR_ID_AMD: [u32; 3];
```

### `_VENDOR_ID_CENTAUR`

```rust
const _VENDOR_ID_CENTAUR: [u32; 3];
```

### `_VENDOR_ID_ZHAOXIN`

```rust
const _VENDOR_ID_ZHAOXIN: [u32; 3];
```

## Macros

### `flags!`

