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

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:107-122`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L107-L122)*

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

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:139-148`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L139-L148)*

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

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:163-170`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L163-L170)*

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

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:173-181`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L173-L181)*

### `Elf_Verdaux`

```rust
struct Elf_Verdaux {
    pub vda_name: u32,
    pub _vda_next: u32,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:184-187`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L184-L187)*

### `Elf_Dyn`

```rust
struct Elf_Dyn {
    pub d_tag: usize,
    pub d_un: Elf_Dyn_Union,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:208-211`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L208-L211)*

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

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:231-235`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L231-L235)*

#### Implementations

- <span id="elf-rela-type"></span>`fn type_(&self) -> u32`

### `Elf_Rel`

```rust
struct Elf_Rel {
    pub r_offset: usize,
    pub r_info: u64,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:260-263`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L260-L263)*

#### Implementations

- <span id="elf-rel-type"></span>`fn type_(&self) -> u32`

### `Elf_auxv_t`

```rust
struct Elf_auxv_t {
    pub a_type: usize,
    pub a_val: *mut crate::ctypes::c_void,
}
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:292-299`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L292-L299)*

#### Trait Implementations

##### `impl Clone for Elf_auxv_t`

- <span id="elf-auxv-t-clone"></span>`fn clone(&self) -> Elf_auxv_t` — [`Elf_auxv_t`](#elf-auxv-t)

## Functions

### `ELF_ST_VISIBILITY`

```rust
const fn ELF_ST_VISIBILITY(o: u8) -> u8
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:92-94`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L92-L94)*

### `ELF_ST_BIND`

```rust
const fn ELF_ST_BIND(val: u8) -> u8
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:97-99`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L97-L99)*

### `ELF_ST_TYPE`

```rust
const fn ELF_ST_TYPE(val: u8) -> u8
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:102-104`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L102-L104)*

## Constants

### `SELFMAG`
```rust
const SELFMAG: usize = 4usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:10`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L10)*

### `ELFMAG`
```rust
const ELFMAG: [u8; 4];
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:11`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L11)*

### `EI_CLASS`
```rust
const EI_CLASS: usize = 4usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:12`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L12)*

### `EI_DATA`
```rust
const EI_DATA: usize = 5usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:13`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L13)*

### `EI_VERSION`
```rust
const EI_VERSION: usize = 6usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:14`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L14)*

### `EI_OSABI`
```rust
const EI_OSABI: usize = 7usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:15`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L15)*

### `EI_ABIVERSION`
```rust
const EI_ABIVERSION: usize = 8usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:16`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L16)*

### `EV_CURRENT`
```rust
const EV_CURRENT: u8 = 1u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:17`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L17)*

### `ELFCLASS`
```rust
const ELFCLASS: u8 = 2u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:21`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L21)*

### `ELFDATA`
```rust
const ELFDATA: u8 = 1u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:23`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L23)*

### `ELFOSABI_SYSV`
```rust
const ELFOSABI_SYSV: u8 = 0u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:26`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L26)*

### `ELFOSABI_LINUX`
```rust
const ELFOSABI_LINUX: u8 = 3u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:27`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L27)*

### `ELFABIVERSION`
```rust
const ELFABIVERSION: u8 = 0u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:29`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L29)*

### `ET_DYN`
```rust
const ET_DYN: u16 = 3u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:30`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L30)*

### `EI_NIDENT`
```rust
const EI_NIDENT: usize = 16usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:31`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L31)*

### `SHN_UNDEF`
```rust
const SHN_UNDEF: u16 = 0u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:32`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L32)*

### `SHN_ABS`
```rust
const SHN_ABS: u16 = 65_521u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:33`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L33)*

### `PN_XNUM`
```rust
const PN_XNUM: u16 = 65_535u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:34`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L34)*

### `PT_LOAD`
```rust
const PT_LOAD: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:35`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L35)*

### `PT_DYNAMIC`
```rust
const PT_DYNAMIC: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:36`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L36)*

### `PT_INTERP`
```rust
const PT_INTERP: u32 = 3u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:37`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L37)*

### `PT_PHDR`
```rust
const PT_PHDR: u32 = 6u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:38`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L38)*

### `PT_TLS`
```rust
const PT_TLS: u32 = 7u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:39`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L39)*

### `PT_GNU_STACK`
```rust
const PT_GNU_STACK: u32 = 1_685_382_481u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:40`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L40)*

### `PT_GNU_RELRO`
```rust
const PT_GNU_RELRO: u32 = 1_685_382_482u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:41`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L41)*

### `PF_X`
```rust
const PF_X: u32 = 1u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:42`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L42)*

### `PF_W`
```rust
const PF_W: u32 = 2u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:43`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L43)*

### `PF_R`
```rust
const PF_R: u32 = 4u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:44`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L44)*

### `DT_NULL`
```rust
const DT_NULL: usize = 0usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:45`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L45)*

### `DT_HASH`
```rust
const DT_HASH: usize = 4usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:46`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L46)*

### `DT_STRTAB`
```rust
const DT_STRTAB: usize = 5usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:47`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L47)*

### `DT_SYMTAB`
```rust
const DT_SYMTAB: usize = 6usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:48`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L48)*

### `DT_RELA`
```rust
const DT_RELA: usize = 7usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:49`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L49)*

### `DT_RELASZ`
```rust
const DT_RELASZ: usize = 8usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:50`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L50)*

### `DT_RELAENT`
```rust
const DT_RELAENT: usize = 9usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:51`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L51)*

### `DT_REL`
```rust
const DT_REL: usize = 17usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:52`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L52)*

### `DT_RELSZ`
```rust
const DT_RELSZ: usize = 18usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:53`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L53)*

### `DT_RELENT`
```rust
const DT_RELENT: usize = 19usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:54`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L54)*

### `DT_SYMENT`
```rust
const DT_SYMENT: usize = 11usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:55`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L55)*

### `DT_GNU_HASH`
```rust
const DT_GNU_HASH: usize = 1_879_047_925usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:56`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L56)*

### `DT_VERSYM`
```rust
const DT_VERSYM: usize = 1_879_048_176usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:57`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L57)*

### `DT_VERDEF`
```rust
const DT_VERDEF: usize = 1_879_048_188usize;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:58`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L58)*

### `STB_WEAK`
```rust
const STB_WEAK: u8 = 2u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:59`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L59)*

### `STB_GLOBAL`
```rust
const STB_GLOBAL: u8 = 1u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:60`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L60)*

### `STT_NOTYPE`
```rust
const STT_NOTYPE: u8 = 0u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:61`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L61)*

### `STT_FUNC`
```rust
const STT_FUNC: u8 = 2u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:62`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L62)*

### `STN_UNDEF`
```rust
const STN_UNDEF: u32 = 0u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:63`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L63)*

### `VER_FLG_BASE`
```rust
const VER_FLG_BASE: u16 = 1u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:64`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L64)*

### `VER_DEF_CURRENT`
```rust
const VER_DEF_CURRENT: u16 = 1u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:65`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L65)*

### `STV_DEFAULT`
```rust
const STV_DEFAULT: u8 = 0u8;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:66`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L66)*

### `EM_CURRENT`
```rust
const EM_CURRENT: u16 = 62u16;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:85`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L85)*

### `R_RELATIVE`
```rust
const R_RELATIVE: u32 = 8u32;
```

*Defined in [`linux-raw-sys-0.11.0/src/elf.rs:280`](../../../.source_1765210505/linux-raw-sys-0.11.0/src/elf.rs#L280)*

