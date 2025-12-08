*[linux_raw_sys](../index.md) / [elf](index.md)*

---

# Module `elf`

The ELF ABI. 🧝

This module is not as comprehensive as bindgened `elf_uapi` and provides only types for target
pointer width: instead of `elf32_phdr` and `elf64_phdr` there's only [`Elf_Phdr`](#elf-phdr).




## Contents

- [Structs](#structs)
  - [`Elf_Ehdr`](#elf_ehdr)
  - [`Elf_Phdr`](#elf_phdr)
  - [`Elf_Sym`](#elf_sym)
  - [`Elf_Verdef`](#elf_verdef)
  - [`Elf_Verdaux`](#elf_verdaux)
  - [`Elf_Dyn`](#elf_dyn)
  - [`Elf_Rela`](#elf_rela)
  - [`Elf_Rel`](#elf_rel)
  - [`Elf_auxv_t`](#elf_auxv_t)
- [Functions](#functions)
  - [`ELF_ST_VISIBILITY`](#elf_st_visibility)
  - [`ELF_ST_BIND`](#elf_st_bind)
  - [`ELF_ST_TYPE`](#elf_st_type)
- [Constants](#constants)
  - [`SELFMAG`](#selfmag)
  - [`ELFMAG`](#elfmag)
  - [`EI_CLASS`](#ei_class)
  - [`EI_DATA`](#ei_data)
  - [`EI_VERSION`](#ei_version)
  - [`EI_OSABI`](#ei_osabi)
  - [`EI_ABIVERSION`](#ei_abiversion)
  - [`EV_CURRENT`](#ev_current)
  - [`ELFCLASS`](#elfclass)
  - [`ELFDATA`](#elfdata)
  - [`ELFOSABI_SYSV`](#elfosabi_sysv)
  - [`ELFOSABI_LINUX`](#elfosabi_linux)
  - [`ELFABIVERSION`](#elfabiversion)
  - [`ET_DYN`](#et_dyn)
  - [`EI_NIDENT`](#ei_nident)
  - [`SHN_UNDEF`](#shn_undef)
  - [`SHN_ABS`](#shn_abs)
  - [`PN_XNUM`](#pn_xnum)
  - [`PT_LOAD`](#pt_load)
  - [`PT_DYNAMIC`](#pt_dynamic)
  - [`PT_INTERP`](#pt_interp)
  - [`PT_PHDR`](#pt_phdr)
  - [`PT_TLS`](#pt_tls)
  - [`PT_GNU_STACK`](#pt_gnu_stack)
  - [`PT_GNU_RELRO`](#pt_gnu_relro)
  - [`PF_X`](#pf_x)
  - [`PF_W`](#pf_w)
  - [`PF_R`](#pf_r)
  - [`DT_NULL`](#dt_null)
  - [`DT_HASH`](#dt_hash)
  - [`DT_STRTAB`](#dt_strtab)
  - [`DT_SYMTAB`](#dt_symtab)
  - [`DT_RELA`](#dt_rela)
  - [`DT_RELASZ`](#dt_relasz)
  - [`DT_RELAENT`](#dt_relaent)
  - [`DT_REL`](#dt_rel)
  - [`DT_RELSZ`](#dt_relsz)
  - [`DT_RELENT`](#dt_relent)
  - [`DT_SYMENT`](#dt_syment)
  - [`DT_GNU_HASH`](#dt_gnu_hash)
  - [`DT_VERSYM`](#dt_versym)
  - [`DT_VERDEF`](#dt_verdef)
  - [`STB_WEAK`](#stb_weak)
  - [`STB_GLOBAL`](#stb_global)
  - [`STT_NOTYPE`](#stt_notype)
  - [`STT_FUNC`](#stt_func)
  - [`STN_UNDEF`](#stn_undef)
  - [`VER_FLG_BASE`](#ver_flg_base)
  - [`VER_DEF_CURRENT`](#ver_def_current)
  - [`STV_DEFAULT`](#stv_default)
  - [`EM_CURRENT`](#em_current)
  - [`R_RELATIVE`](#r_relative)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Elf_Ehdr`](#elf_ehdr) | struct |  |
| [`Elf_Phdr`](#elf_phdr) | struct |  |
| [`Elf_Sym`](#elf_sym) | struct |  |
| [`Elf_Verdef`](#elf_verdef) | struct |  |
| [`Elf_Verdaux`](#elf_verdaux) | struct |  |
| [`Elf_Dyn`](#elf_dyn) | struct |  |
| [`Elf_Rela`](#elf_rela) | struct |  |
| [`Elf_Rel`](#elf_rel) | struct |  |
| [`Elf_auxv_t`](#elf_auxv_t) | struct |  |
| [`ELF_ST_VISIBILITY`](#elf_st_visibility) | fn |  |
| [`ELF_ST_BIND`](#elf_st_bind) | fn |  |
| [`ELF_ST_TYPE`](#elf_st_type) | fn |  |
| [`SELFMAG`](#selfmag) | const |  |
| [`ELFMAG`](#elfmag) | const |  |
| [`EI_CLASS`](#ei_class) | const |  |
| [`EI_DATA`](#ei_data) | const |  |
| [`EI_VERSION`](#ei_version) | const |  |
| [`EI_OSABI`](#ei_osabi) | const |  |
| [`EI_ABIVERSION`](#ei_abiversion) | const |  |
| [`EV_CURRENT`](#ev_current) | const |  |
| [`ELFCLASS`](#elfclass) | const |  |
| [`ELFDATA`](#elfdata) | const |  |
| [`ELFOSABI_SYSV`](#elfosabi_sysv) | const |  |
| [`ELFOSABI_LINUX`](#elfosabi_linux) | const |  |
| [`ELFABIVERSION`](#elfabiversion) | const |  |
| [`ET_DYN`](#et_dyn) | const |  |
| [`EI_NIDENT`](#ei_nident) | const |  |
| [`SHN_UNDEF`](#shn_undef) | const |  |
| [`SHN_ABS`](#shn_abs) | const |  |
| [`PN_XNUM`](#pn_xnum) | const |  |
| [`PT_LOAD`](#pt_load) | const |  |
| [`PT_DYNAMIC`](#pt_dynamic) | const |  |
| [`PT_INTERP`](#pt_interp) | const |  |
| [`PT_PHDR`](#pt_phdr) | const |  |
| [`PT_TLS`](#pt_tls) | const |  |
| [`PT_GNU_STACK`](#pt_gnu_stack) | const |  |
| [`PT_GNU_RELRO`](#pt_gnu_relro) | const |  |
| [`PF_X`](#pf_x) | const |  |
| [`PF_W`](#pf_w) | const |  |
| [`PF_R`](#pf_r) | const |  |
| [`DT_NULL`](#dt_null) | const |  |
| [`DT_HASH`](#dt_hash) | const |  |
| [`DT_STRTAB`](#dt_strtab) | const |  |
| [`DT_SYMTAB`](#dt_symtab) | const |  |
| [`DT_RELA`](#dt_rela) | const |  |
| [`DT_RELASZ`](#dt_relasz) | const |  |
| [`DT_RELAENT`](#dt_relaent) | const |  |
| [`DT_REL`](#dt_rel) | const |  |
| [`DT_RELSZ`](#dt_relsz) | const |  |
| [`DT_RELENT`](#dt_relent) | const |  |
| [`DT_SYMENT`](#dt_syment) | const |  |
| [`DT_GNU_HASH`](#dt_gnu_hash) | const |  |
| [`DT_VERSYM`](#dt_versym) | const |  |
| [`DT_VERDEF`](#dt_verdef) | const |  |
| [`STB_WEAK`](#stb_weak) | const |  |
| [`STB_GLOBAL`](#stb_global) | const |  |
| [`STT_NOTYPE`](#stt_notype) | const |  |
| [`STT_FUNC`](#stt_func) | const |  |
| [`STN_UNDEF`](#stn_undef) | const |  |
| [`VER_FLG_BASE`](#ver_flg_base) | const |  |
| [`VER_DEF_CURRENT`](#ver_def_current) | const |  |
| [`STV_DEFAULT`](#stv_default) | const |  |
| [`EM_CURRENT`](#em_current) | const |  |
| [`R_RELATIVE`](#r_relative) | const |  |

## Structs

### `Elf_Ehdr`

```rust
struct Elf_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: usize,
    pub e_phoff: usize,
    pub e_shoff: usize,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}
```

### `Elf_Phdr`

```rust
struct Elf_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: usize,
    pub p_vaddr: usize,
    pub p_paddr: usize,
    pub p_filesz: usize,
    pub p_memsz: usize,
    pub p_align: usize,
}
```

### `Elf_Sym`

```rust
struct Elf_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: usize,
    pub st_size: usize,
}
```

### `Elf_Verdef`

```rust
struct Elf_Verdef {
    pub vd_version: u16,
    pub vd_flags: u16,
    pub vd_ndx: u16,
    pub vd_cnt: u16,
    pub vd_hash: u32,
    pub vd_aux: u32,
    pub vd_next: u32,
}
```

### `Elf_Verdaux`

```rust
struct Elf_Verdaux {
    pub vda_name: u32,
    pub _vda_next: u32,
}
```

### `Elf_Dyn`

```rust
struct Elf_Dyn {
    pub d_tag: usize,
    pub d_un: Elf_Dyn_Union,
}
```

#### Trait Implementations

##### `impl Clone for Elf_Dyn`

- <span id="elf-dyn-clone"></span>`fn clone(&self) -> Elf_Dyn` — [`Elf_Dyn`](#elf-dyn)

##### `impl Copy for Elf_Dyn`

### `Elf_Rela`

```rust
struct Elf_Rela {
    pub r_offset: usize,
    pub r_info: u64,
    pub r_addend: usize,
}
```

#### Implementations

- <span id="elf-rela-type"></span>`fn type_(&self) -> u32`

### `Elf_Rel`

```rust
struct Elf_Rel {
    pub r_offset: usize,
    pub r_info: u64,
}
```

#### Implementations

- <span id="elf-rel-type"></span>`fn type_(&self) -> u32`

### `Elf_auxv_t`

```rust
struct Elf_auxv_t {
    pub a_type: usize,
    pub a_val: *mut crate::ctypes::c_void,
}
```

#### Trait Implementations

##### `impl Clone for Elf_auxv_t`

- <span id="elf-auxv-t-clone"></span>`fn clone(&self) -> Elf_auxv_t` — [`Elf_auxv_t`](#elf-auxv-t)

## Functions

### `ELF_ST_VISIBILITY`

```rust
const fn ELF_ST_VISIBILITY(o: u8) -> u8
```

### `ELF_ST_BIND`

```rust
const fn ELF_ST_BIND(val: u8) -> u8
```

### `ELF_ST_TYPE`

```rust
const fn ELF_ST_TYPE(val: u8) -> u8
```

## Constants

### `SELFMAG`

```rust
const SELFMAG: usize = 4usize;
```

### `ELFMAG`

```rust
const ELFMAG: [u8; 4];
```

### `EI_CLASS`

```rust
const EI_CLASS: usize = 4usize;
```

### `EI_DATA`

```rust
const EI_DATA: usize = 5usize;
```

### `EI_VERSION`

```rust
const EI_VERSION: usize = 6usize;
```

### `EI_OSABI`

```rust
const EI_OSABI: usize = 7usize;
```

### `EI_ABIVERSION`

```rust
const EI_ABIVERSION: usize = 8usize;
```

### `EV_CURRENT`

```rust
const EV_CURRENT: u8 = 1u8;
```

### `ELFCLASS`

```rust
const ELFCLASS: u8 = 2u8;
```

### `ELFDATA`

```rust
const ELFDATA: u8 = 1u8;
```

### `ELFOSABI_SYSV`

```rust
const ELFOSABI_SYSV: u8 = 0u8;
```

### `ELFOSABI_LINUX`

```rust
const ELFOSABI_LINUX: u8 = 3u8;
```

### `ELFABIVERSION`

```rust
const ELFABIVERSION: u8 = 0u8;
```

### `ET_DYN`

```rust
const ET_DYN: u16 = 3u16;
```

### `EI_NIDENT`

```rust
const EI_NIDENT: usize = 16usize;
```

### `SHN_UNDEF`

```rust
const SHN_UNDEF: u16 = 0u16;
```

### `SHN_ABS`

```rust
const SHN_ABS: u16 = 65_521u16;
```

### `PN_XNUM`

```rust
const PN_XNUM: u16 = 65_535u16;
```

### `PT_LOAD`

```rust
const PT_LOAD: u32 = 1u32;
```

### `PT_DYNAMIC`

```rust
const PT_DYNAMIC: u32 = 2u32;
```

### `PT_INTERP`

```rust
const PT_INTERP: u32 = 3u32;
```

### `PT_PHDR`

```rust
const PT_PHDR: u32 = 6u32;
```

### `PT_TLS`

```rust
const PT_TLS: u32 = 7u32;
```

### `PT_GNU_STACK`

```rust
const PT_GNU_STACK: u32 = 1_685_382_481u32;
```

### `PT_GNU_RELRO`

```rust
const PT_GNU_RELRO: u32 = 1_685_382_482u32;
```

### `PF_X`

```rust
const PF_X: u32 = 1u32;
```

### `PF_W`

```rust
const PF_W: u32 = 2u32;
```

### `PF_R`

```rust
const PF_R: u32 = 4u32;
```

### `DT_NULL`

```rust
const DT_NULL: usize = 0usize;
```

### `DT_HASH`

```rust
const DT_HASH: usize = 4usize;
```

### `DT_STRTAB`

```rust
const DT_STRTAB: usize = 5usize;
```

### `DT_SYMTAB`

```rust
const DT_SYMTAB: usize = 6usize;
```

### `DT_RELA`

```rust
const DT_RELA: usize = 7usize;
```

### `DT_RELASZ`

```rust
const DT_RELASZ: usize = 8usize;
```

### `DT_RELAENT`

```rust
const DT_RELAENT: usize = 9usize;
```

### `DT_REL`

```rust
const DT_REL: usize = 17usize;
```

### `DT_RELSZ`

```rust
const DT_RELSZ: usize = 18usize;
```

### `DT_RELENT`

```rust
const DT_RELENT: usize = 19usize;
```

### `DT_SYMENT`

```rust
const DT_SYMENT: usize = 11usize;
```

### `DT_GNU_HASH`

```rust
const DT_GNU_HASH: usize = 1_879_047_925usize;
```

### `DT_VERSYM`

```rust
const DT_VERSYM: usize = 1_879_048_176usize;
```

### `DT_VERDEF`

```rust
const DT_VERDEF: usize = 1_879_048_188usize;
```

### `STB_WEAK`

```rust
const STB_WEAK: u8 = 2u8;
```

### `STB_GLOBAL`

```rust
const STB_GLOBAL: u8 = 1u8;
```

### `STT_NOTYPE`

```rust
const STT_NOTYPE: u8 = 0u8;
```

### `STT_FUNC`

```rust
const STT_FUNC: u8 = 2u8;
```

### `STN_UNDEF`

```rust
const STN_UNDEF: u32 = 0u32;
```

### `VER_FLG_BASE`

```rust
const VER_FLG_BASE: u16 = 1u16;
```

### `VER_DEF_CURRENT`

```rust
const VER_DEF_CURRENT: u16 = 1u16;
```

### `STV_DEFAULT`

```rust
const STV_DEFAULT: u8 = 0u8;
```

### `EM_CURRENT`

```rust
const EM_CURRENT: u16 = 62u16;
```

### `R_RELATIVE`

```rust
const R_RELATIVE: u32 = 8u32;
```

