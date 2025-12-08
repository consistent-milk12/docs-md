*[portable_atomic](../../../../index.md) / [imp](../../../index.md) / [atomic128](../../index.md) / [x86_64](../index.md) / [detect](index.md)*

---

# Module `detect`

## Structs

### `CpuInfo`

```rust
struct CpuInfo(u32);
```

#### Implementations

- `fn cmpxchg16b(self: Self) -> bool`

- `fn vmovdqa_atomic(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for CpuInfo`

- `fn clone(self: &Self) -> CpuInfo` — [`CpuInfo`](#cpuinfo)

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

- `fn clone(self: &Self) -> CpuInfoFlag` — [`CpuInfoFlag`](#cpuinfoflag)

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

