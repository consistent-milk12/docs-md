*[linux_raw_sys](../index.md) / [auxvec](index.md)*

---

# Module `auxvec`

## Contents

- [Constants](#constants)
  - [`AT_SYSINFO_EHDR`](#at_sysinfo_ehdr)
  - [`AT_VECTOR_SIZE_ARCH`](#at_vector_size_arch)
  - [`AT_NULL`](#at_null)
  - [`AT_IGNORE`](#at_ignore)
  - [`AT_EXECFD`](#at_execfd)
  - [`AT_PHDR`](#at_phdr)
  - [`AT_PHENT`](#at_phent)
  - [`AT_PHNUM`](#at_phnum)
  - [`AT_PAGESZ`](#at_pagesz)
  - [`AT_BASE`](#at_base)
  - [`AT_FLAGS`](#at_flags)
  - [`AT_ENTRY`](#at_entry)
  - [`AT_NOTELF`](#at_notelf)
  - [`AT_UID`](#at_uid)
  - [`AT_EUID`](#at_euid)
  - [`AT_GID`](#at_gid)
  - [`AT_EGID`](#at_egid)
  - [`AT_PLATFORM`](#at_platform)
  - [`AT_HWCAP`](#at_hwcap)
  - [`AT_CLKTCK`](#at_clktck)
  - [`AT_SECURE`](#at_secure)
  - [`AT_BASE_PLATFORM`](#at_base_platform)
  - [`AT_RANDOM`](#at_random)
  - [`AT_HWCAP2`](#at_hwcap2)
  - [`AT_RSEQ_FEATURE_SIZE`](#at_rseq_feature_size)
  - [`AT_RSEQ_ALIGN`](#at_rseq_align)
  - [`AT_HWCAP3`](#at_hwcap3)
  - [`AT_HWCAP4`](#at_hwcap4)
  - [`AT_EXECFN`](#at_execfn)
  - [`AT_MINSIGSTKSZ`](#at_minsigstksz)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AT_SYSINFO_EHDR`](#at_sysinfo_ehdr) | const |  |
| [`AT_VECTOR_SIZE_ARCH`](#at_vector_size_arch) | const |  |
| [`AT_NULL`](#at_null) | const |  |
| [`AT_IGNORE`](#at_ignore) | const |  |
| [`AT_EXECFD`](#at_execfd) | const |  |
| [`AT_PHDR`](#at_phdr) | const |  |
| [`AT_PHENT`](#at_phent) | const |  |
| [`AT_PHNUM`](#at_phnum) | const |  |
| [`AT_PAGESZ`](#at_pagesz) | const |  |
| [`AT_BASE`](#at_base) | const |  |
| [`AT_FLAGS`](#at_flags) | const |  |
| [`AT_ENTRY`](#at_entry) | const |  |
| [`AT_NOTELF`](#at_notelf) | const |  |
| [`AT_UID`](#at_uid) | const |  |
| [`AT_EUID`](#at_euid) | const |  |
| [`AT_GID`](#at_gid) | const |  |
| [`AT_EGID`](#at_egid) | const |  |
| [`AT_PLATFORM`](#at_platform) | const |  |
| [`AT_HWCAP`](#at_hwcap) | const |  |
| [`AT_CLKTCK`](#at_clktck) | const |  |
| [`AT_SECURE`](#at_secure) | const |  |
| [`AT_BASE_PLATFORM`](#at_base_platform) | const |  |
| [`AT_RANDOM`](#at_random) | const |  |
| [`AT_HWCAP2`](#at_hwcap2) | const |  |
| [`AT_RSEQ_FEATURE_SIZE`](#at_rseq_feature_size) | const |  |
| [`AT_RSEQ_ALIGN`](#at_rseq_align) | const |  |
| [`AT_HWCAP3`](#at_hwcap3) | const |  |
| [`AT_HWCAP4`](#at_hwcap4) | const |  |
| [`AT_EXECFN`](#at_execfn) | const |  |
| [`AT_MINSIGSTKSZ`](#at_minsigstksz) | const |  |

## Constants

### `AT_SYSINFO_EHDR`

```rust
const AT_SYSINFO_EHDR: u32 = 33u32;
```

### `AT_VECTOR_SIZE_ARCH`

```rust
const AT_VECTOR_SIZE_ARCH: u32 = 3u32;
```

### `AT_NULL`

```rust
const AT_NULL: u32 = 0u32;
```

### `AT_IGNORE`

```rust
const AT_IGNORE: u32 = 1u32;
```

### `AT_EXECFD`

```rust
const AT_EXECFD: u32 = 2u32;
```

### `AT_PHDR`

```rust
const AT_PHDR: u32 = 3u32;
```

### `AT_PHENT`

```rust
const AT_PHENT: u32 = 4u32;
```

### `AT_PHNUM`

```rust
const AT_PHNUM: u32 = 5u32;
```

### `AT_PAGESZ`

```rust
const AT_PAGESZ: u32 = 6u32;
```

### `AT_BASE`

```rust
const AT_BASE: u32 = 7u32;
```

### `AT_FLAGS`

```rust
const AT_FLAGS: u32 = 8u32;
```

### `AT_ENTRY`

```rust
const AT_ENTRY: u32 = 9u32;
```

### `AT_NOTELF`

```rust
const AT_NOTELF: u32 = 10u32;
```

### `AT_UID`

```rust
const AT_UID: u32 = 11u32;
```

### `AT_EUID`

```rust
const AT_EUID: u32 = 12u32;
```

### `AT_GID`

```rust
const AT_GID: u32 = 13u32;
```

### `AT_EGID`

```rust
const AT_EGID: u32 = 14u32;
```

### `AT_PLATFORM`

```rust
const AT_PLATFORM: u32 = 15u32;
```

### `AT_HWCAP`

```rust
const AT_HWCAP: u32 = 16u32;
```

### `AT_CLKTCK`

```rust
const AT_CLKTCK: u32 = 17u32;
```

### `AT_SECURE`

```rust
const AT_SECURE: u32 = 23u32;
```

### `AT_BASE_PLATFORM`

```rust
const AT_BASE_PLATFORM: u32 = 24u32;
```

### `AT_RANDOM`

```rust
const AT_RANDOM: u32 = 25u32;
```

### `AT_HWCAP2`

```rust
const AT_HWCAP2: u32 = 26u32;
```

### `AT_RSEQ_FEATURE_SIZE`

```rust
const AT_RSEQ_FEATURE_SIZE: u32 = 27u32;
```

### `AT_RSEQ_ALIGN`

```rust
const AT_RSEQ_ALIGN: u32 = 28u32;
```

### `AT_HWCAP3`

```rust
const AT_HWCAP3: u32 = 29u32;
```

### `AT_HWCAP4`

```rust
const AT_HWCAP4: u32 = 30u32;
```

### `AT_EXECFN`

```rust
const AT_EXECFN: u32 = 31u32;
```

### `AT_MINSIGSTKSZ`

```rust
const AT_MINSIGSTKSZ: u32 = 51u32;
```

